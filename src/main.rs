use anyhow::Result;
use log::*;

use htps::htsp::*;

pub mod htps;

fn setup_logging() {
    env_logger::init();
    trace!("Logging initialized {}", 1);
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();
    info!("Hello, world!");

    let test = Request::Hello {
        htsp_version: 25,
        client_name: "FooName",
        client_version: "1.2.3",
    };

    let mut out: Vec<u8> = vec![];
    serialize(test, &mut out)?;
    warn!("Result: {:?}", out);
    Ok(())
}
