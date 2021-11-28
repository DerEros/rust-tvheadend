pub mod protocol;

use crate::protocol::messages::Request;
use crate::protocol::request_serializer::RequestSerializer;
use crate::protocol::server::Server;
use crate::protocol::wire_format::ToBytes;
use anyhow::Result;
use log::*;
use std::time::Duration;

fn setup_logging() {
    env_logger::init();
    trace!("Logging initialized {}", 1);
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    info!("Hello, world!");

    let mut server = Server::new("herman:9982");
    let _ = server.run().await?;
    let _ = server.hello(25, "client_name", "client_version").await?;

    server.run().await?;
    std::thread::sleep(Duration::from_secs(10));
    Ok(())
}
