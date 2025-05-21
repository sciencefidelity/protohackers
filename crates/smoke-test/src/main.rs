use smoke_test::server::run;
use smoke_test::telemetry::init_subscriber;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber();
    run().await?;
    Ok(())
}
