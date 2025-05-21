use smoke_test::server::run;
use smoke_test::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    run().await?;
    Ok(())
}
