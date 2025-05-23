use crate::imports::*;

#[derive(Default, Handler)]
#[help("Disconnect from the entropyx network")]
pub struct Disconnect;

impl Disconnect {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, _argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<EntropyXCli>()?;
        if let Some(wrpc_client) = ctx.wallet().try_wrpc_client().as_ref() {
            wrpc_client.disconnect().await?;
        } else {
            terrorln!(ctx, "Unable to disconnect from non-wRPC client");
        }
        Ok(())
    }
}
