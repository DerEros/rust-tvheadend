pub mod htsp;

use crate::htsp::HtspSerializer;
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

    let test = htsp::Request::Hello {
        htsp_version: 25,
        client_name: "FooName",
        client_version: "1.2.3",
    };
    let mut ser = HtspSerializer::create();
    let res = test.serialize(&mut ser)?;
    warn!("{:?}", res);

    Ok(())
}
