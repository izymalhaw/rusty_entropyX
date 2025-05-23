use super::{
    handler_trait::Handler,
    interface::{DynEntropyXMethod, Interface},
};
use crate::{
    connection::{Connection, IncomingRoute},
    connection_handler::ServerContext,
    error::GrpcServerResult,
};
use entropyx_core::debug;
use entropyx_grpc_core::{
    ops::EntropyXPayloadOps,
    protowire::{EntropyXRequest, EntropyXResponse},
};

pub struct RequestHandler {
    rpc_op: EntropyXPayloadOps,
    incoming_route: IncomingRoute,
    server_ctx: ServerContext,
    method: DynEntropyXMethod,
    connection: Connection,
}

impl RequestHandler {
    pub fn new(
        rpc_op: EntropyXPayloadOps,
        incoming_route: IncomingRoute,
        server_context: ServerContext,
        interface: &Interface,
        connection: Connection,
    ) -> Self {
        let method = interface.get_method(&rpc_op);
        Self { rpc_op, incoming_route, server_ctx: server_context, method, connection }
    }

    pub async fn handle_request(&self, request: EntropyXRequest) -> GrpcServerResult<EntropyXResponse> {
        let id = request.id;
        let mut response = self.method.call(self.server_ctx.clone(), self.connection.clone(), request).await?;
        response.id = id;
        Ok(response)
    }
}

#[async_trait::async_trait]
impl Handler for RequestHandler {
    async fn start(&mut self) {
        debug!("GRPC, Starting request handler {:?} for client {}", self.rpc_op, self.connection);
        while let Ok(request) = self.incoming_route.recv().await {
            let response = self.handle_request(request).await;
            match response {
                Ok(response) => {
                    if self.connection.enqueue(response).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    debug!("GRPC, Request handling error {} for client {}", e, self.connection);
                }
            }
        }
        debug!("GRPC, Exiting request handler {:?} for client {}", self.rpc_op, self.connection);
    }
}
