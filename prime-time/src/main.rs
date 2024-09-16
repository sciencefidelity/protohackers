use prime_time::server::run;
use prime_time::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    init_subscriber();
    run().await?;
    Ok(())
}
