pub mod protocol;

use crate::protocol::messages::Request;
use crate::protocol::request_serializer::RequestSerializer;
use anyhow::Result;
use log::*;
use serde::Serialize;

fn setup_logging() {
    env_logger::init();
    trace!("Logging initialized {}", 1);
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    info!("Hello, world!");

    let req = Request::Hello {
        htsp_version: 25,
        client_version: "1.0.0",
        client_name: "rust-tvheadend",
    };

    let mut serializer = RequestSerializer {};
    let res = req.serialize(serializer)?;

    warn!("Result: {:?}", res);
    Ok(())
}
