use crate::{
    common::ProtocolError,
    core::adaptor::ConnectionInitializer,
    handshake::EntropyXHandshake,
    pb::{self, VersionMessage},
    IncomingRoute, EntropyXMessagePayloadType, Router,
};
use entropyx_core::{debug, time::unix_now, trace, warn};
use std::sync::Arc;
use tonic::async_trait;
use uuid::Uuid;

/// An example flow, echoing all messages back to the network
pub struct EchoFlow {
    receiver: IncomingRoute,
    router: Arc<Router>,
}

impl EchoFlow {
    pub async fn register(router: Arc<Router>) {
        // Subscribe to messages
        trace!("EchoFlow, subscribe to all p2p messages");
        let receiver = router.subscribe(vec![
            EntropyXMessagePayloadType::Addresses,
            EntropyXMessagePayloadType::Block,
            EntropyXMessagePayloadType::Transaction,
            EntropyXMessagePayloadType::BlockLocator,
            EntropyXMessagePayloadType::RequestAddresses,
            EntropyXMessagePayloadType::RequestRelayBlocks,
            EntropyXMessagePayloadType::RequestTransactions,
            EntropyXMessagePayloadType::IbdBlock,
            EntropyXMessagePayloadType::InvRelayBlock,
            EntropyXMessagePayloadType::InvTransactions,
            EntropyXMessagePayloadType::Ping,
            EntropyXMessagePayloadType::Pong,
            // EntropyXMessagePayloadType::Verack,
            // EntropyXMessagePayloadType::Version,
            // EntropyXMessagePayloadType::Ready,
            EntropyXMessagePayloadType::TransactionNotFound,
            EntropyXMessagePayloadType::Reject,
            EntropyXMessagePayloadType::PruningPointUtxoSetChunk,
            EntropyXMessagePayloadType::RequestIbdBlocks,
            EntropyXMessagePayloadType::UnexpectedPruningPoint,
            EntropyXMessagePayloadType::IbdBlockLocator,
            EntropyXMessagePayloadType::IbdBlockLocatorHighestHash,
            EntropyXMessagePayloadType::RequestNextPruningPointUtxoSetChunk,
            EntropyXMessagePayloadType::DonePruningPointUtxoSetChunks,
            EntropyXMessagePayloadType::IbdBlockLocatorHighestHashNotFound,
            EntropyXMessagePayloadType::BlockWithTrustedData,
            EntropyXMessagePayloadType::DoneBlocksWithTrustedData,
            EntropyXMessagePayloadType::RequestPruningPointAndItsAnticone,
            EntropyXMessagePayloadType::BlockHeaders,
            EntropyXMessagePayloadType::RequestNextHeaders,
            EntropyXMessagePayloadType::DoneHeaders,
            EntropyXMessagePayloadType::RequestPruningPointUtxoSet,
            EntropyXMessagePayloadType::RequestHeaders,
            EntropyXMessagePayloadType::RequestBlockLocator,
            EntropyXMessagePayloadType::PruningPoints,
            EntropyXMessagePayloadType::RequestPruningPointProof,
            EntropyXMessagePayloadType::PruningPointProof,
            EntropyXMessagePayloadType::BlockWithTrustedDataV4,
            EntropyXMessagePayloadType::TrustedData,
            EntropyXMessagePayloadType::RequestIbdChainBlockLocator,
            EntropyXMessagePayloadType::IbdChainBlockLocator,
            EntropyXMessagePayloadType::RequestAntipast,
            EntropyXMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks,
        ]);
        let mut echo_flow = EchoFlow { router, receiver };
        debug!("EchoFlow, start app-layer receiving loop");
        tokio::spawn(async move {
            debug!("EchoFlow, start message dispatching loop");
            while let Some(msg) = echo_flow.receiver.recv().await {
                if !(echo_flow.call(msg).await) {
                    warn!("EchoFlow, receive loop - call failed");
                    break;
                }
            }
            debug!("EchoFlow, exiting message dispatch loop");
        });
    }

    async fn call(&self, msg: pb::EntropyXMessage) -> bool {
        // echo
        trace!("EchoFlow, got message:{:?}", msg);
        self.router.enqueue(msg).await.is_ok()
    }
}

/// An example initializer, performing handshake and registering a simple echo flow
#[derive(Default)]
pub struct EchoFlowInitializer {}

fn build_dummy_version_message() -> VersionMessage {
    pb::VersionMessage {
        protocol_version: 5,
        services: 0,
        timestamp: unix_now() as i64,
        address: None,
        id: Vec::from(Uuid::new_v4().as_bytes()),
        user_agent: String::new(),
        disable_relay_tx: false,
        subnetwork_id: None,
        network: "entropyx-mainnet".to_string(),
    }
}

impl EchoFlowInitializer {
    pub fn new() -> Self {
        EchoFlowInitializer {}
    }
}

#[async_trait]
impl ConnectionInitializer for EchoFlowInitializer {
    async fn initialize_connection(&self, router: Arc<Router>) -> Result<(), ProtocolError> {
        //
        // Example code to illustrate entropyx P2P handshaking
        //

        // Build the handshake object and subscribe to handshake messages
        let mut handshake = EntropyXHandshake::new(&router);

        // We start the router receive loop only after we registered to handshake routes
        router.start();

        // Build the local version message
        let self_version_message = build_dummy_version_message();

        // Perform the handshake
        let peer_version_message = handshake.handshake(self_version_message).await?;
        debug!("protocol versions - self: {}, peer: {}", 5, peer_version_message.protocol_version);

        // Subscribe to remaining messages. In this example we simply subscribe to all messages with a single echo flow
        EchoFlow::register(router.clone()).await;

        // Send a ready signal
        handshake.exchange_ready_messages().await?;

        // Note: at this point receivers for handshake subscriptions
        // are dropped, thus effectively unsubscribing

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use super::*;
    use crate::{Adaptor, Hub};
    use entropyx_core::debug;
    use entropyx_utils::networking::NetAddress;

    #[tokio::test]
    async fn test_handshake() {
        entropyx_core::log::try_init_logger("debug");

        let address1 = NetAddress::from_str("[::1]:50053").unwrap();
        let adaptor1 = Adaptor::bidirectional(address1, Hub::new(), Arc::new(EchoFlowInitializer::new()), Default::default()).unwrap();

        let address2 = NetAddress::from_str("[::1]:50054").unwrap();
        let adaptor2 = Adaptor::bidirectional(address2, Hub::new(), Arc::new(EchoFlowInitializer::new()), Default::default()).unwrap();

        // Initiate the connection from `adaptor1` (outbound) to `adaptor2` (inbound)
        let peer2_id = adaptor1
            .connect_peer_with_retries(String::from("[::1]:50054"), 16, Duration::from_secs(1))
            .await
            .expect("peer connection failed");

        // Wait for handshake completion
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let adaptor1_initial_peers = adaptor1.active_peers();
        let adaptor2_initial_peers = adaptor2.active_peers();

        // For now assert the handshake by checking the peer exists (since peer is removed on handshake error)
        assert_eq!(adaptor1_initial_peers.len(), 1, "handshake failed -- outbound peer is missing");
        assert_eq!(adaptor2_initial_peers.len(), 1, "handshake failed -- inbound peer is missing");

        assert!(adaptor1_initial_peers[0].is_outbound());
        assert!(!adaptor2_initial_peers[0].is_outbound());

        adaptor1.terminate(peer2_id).await;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Make sure the peers are cleaned-up on both sides
        assert_eq!(adaptor1.active_peers().len(), 0, "peer termination failed -- outbound peer was not removed");
        assert_eq!(adaptor2.active_peers().len(), 0, "peer termination failed -- inbound peer was not removed");

        adaptor1.close().await;
        adaptor2.close().await;
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Make sure that all internal loops exit and adaptors are ready to be dropped
        debug!("{} {}", Arc::strong_count(&adaptor1), Arc::strong_count(&adaptor2));
        assert_eq!(Arc::strong_count(&adaptor1), 1, "some adaptor resources did not cleanup");
        assert_eq!(Arc::strong_count(&adaptor2), 1, "some adaptor resources did not cleanup");

        drop(adaptor1);
        drop(adaptor2);
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
