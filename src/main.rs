use anyhow::Result;
use log::*;

use crate::htps::htsp::deserialize;
use htps::htsp::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

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

    let mut read_buffer = [0_u8; 1024];
    stream.read(&mut read_buffer[..]).await?;

    let reply = deserialize(&mut read_buffer.to_vec().as_slice(), "hello")?;
    warn!("Reply: {:?}", reply);


    let test2 = Request::GetDiskSpace {};
    let mut buffer2: Vec<u8> = vec![];
    serialize(test2, &mut buffer2)?;
    stream.write_all(buffer2.as_slice()).await?;
    stream.read(&mut read_buffer[..]).await?;
    let reply2 = deserialize(&mut read_buffer.to_vec().as_slice(), "getDiskSpace")?;
    warn!("Reply: {:?}", reply2);


    Ok(())
}
