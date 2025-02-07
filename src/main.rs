use tracing::error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = oss_security_bot::run().await {
        error!("{:?}", e);
    }

    Ok(())
}
