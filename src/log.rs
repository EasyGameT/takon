use tracing_subscriber::{filter::LevelFilter, EnvFilter};

pub struct Log;

impl Log {
    pub fn new() -> Self {
        let filter = EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env()
            .unwrap()
            .add_directive("debug".parse().unwrap());

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .compact()
            .init();

        Self {}
    }
}
