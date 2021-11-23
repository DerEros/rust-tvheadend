use anyhow::Result;
use log::*;

use htps::htsp::*;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

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
        htsp_version: 34,
        client_name: "FooName",
        client_version: "1.0.0",
    };

    let mut stream = TcpStream::connect("herman.fritz.box:9982").await?;
    stream.writable().await?;

    let mut buffer: Vec<u8> = vec![];
    serialize(test, &mut buffer)?;
    warn!("Request: {:?}", buffer);
    stream.write_all(buffer.as_slice()).await?;

    let mut read_buffer = [0_u8; 65536];
    stream.read(&mut read_buffer[..]).await?;

    warn!("Reply: {:?}", read_buffer);
    Ok(())
}
