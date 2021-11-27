use crate::{Request, RequestSerializer, ToBytes};
use anyhow::{bail, Result};
use bytes::Bytes;
use log::*;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

pub struct Server {
    stream: Option<TcpStream>,
}

impl Server {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub async fn connect<T: ToSocketAddrs>(&mut self, address: T) -> Result<()> {
        debug!("Connecting to server");
        let stream = TcpStream::connect(address).await?;
        trace!("Connection successful");
        self.stream = Some(stream);
        Ok(())
    }

    pub fn hello<T: AsRef<str>>(
        &self,
        htsp_version: u32,
        client_name: T,
        client_version: T,
    ) -> Result<()> {
        let msg = Request::Hello {
            client_name: client_name.as_ref(),
            client_version: client_version.as_ref(),
            htsp_version,
        };
        self.send_request(msg)?;

        Ok(())
    }

    fn send_request(&self, request: Request) -> Result<()> {
        if let Some(stream) = self.stream.as_ref() {
            let intermediate = request.serialize(RequestSerializer::new())?;
            let data: Bytes = intermediate.to_bytes();
            trace!("Data: {:?}", data);
            stream.try_write(&data)?;

            Ok(())
        } else {
            bail!("Trying to send message without connection")
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}
