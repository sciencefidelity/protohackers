use means_to_an_end::server::run;
use means_to_an_end::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_subscriber();
    run().await?;
    Ok(())
}
