use prime_time::server::run;
use prime_time::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    run().await?;
    Ok(())
}
