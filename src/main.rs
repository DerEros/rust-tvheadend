use anyhow::Result;
use log::*;

fn setup_logging() {
    env_logger::init();
    trace!("Logging initialized {}", 1);
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    info!("Hello, world!");

    Ok(())
}
