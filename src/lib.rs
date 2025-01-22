use tracing_subscriber::EnvFilter;

pub mod cli;
pub mod dir;
pub mod info;

pub fn setup_tracing_subscriber() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt().with_env_filter(filter).init();

}
