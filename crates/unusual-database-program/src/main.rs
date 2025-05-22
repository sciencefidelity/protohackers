use unusual_database_program::server::run;
use unusual_database_program::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    run().await?;
    Ok(())
}
