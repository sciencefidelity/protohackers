use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};

pub fn init_subscriber() {
    let subscriber = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
