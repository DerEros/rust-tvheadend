use crate::protocol::intermediate::Field;
use crate::protocol::message_receiver::Receiver;
use crate::{Request, RequestSerializer, ToBytes};
use anyhow::{bail, Result};
use bytes::Bytes;
use log::*;
use serde::Serialize;
use std::sync::RwLock;
use futures::channel::mpsc::UnboundedReceiver;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;

pub struct Server {
    stream: Option<TcpStream>,
    next_sequence_number: RwLock<usize>,
    receiver_stop_channel: Option<UnboundedReceiver<bool>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            stream: None,
            next_sequence_number: RwLock::new(0),
            receiver_stop_channel: None,
        }
    }

    pub async fn connect<T: ToSocketAddrs>(&mut self, address: T) -> Result<()> {
        debug!("Connecting to server");
        let stream = TcpStream::connect(address).await?;
        trace!("Connection successful");
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn hello<T: AsRef<str>>(
        &mut self,
        htsp_version: u32,
        client_name: T,
        client_version: T,
    ) -> Result<()> {
        ServerRequest::from_message(
            self,
            Request::Hello {
                client_name: client_name.as_ref(),
                client_version: client_version.as_ref(),
                htsp_version,
            },
        )
        .with_seq()
        .send()
        .await?;

        Ok(())
    }

    pub(self) fn get_next_sequence_number(&self) -> usize {
        let lock = self.next_sequence_number.write();
        let mut number = lock.unwrap(); // panic if lock cannot be acquired
        *number += 1;
        *number
    }

    pub(self) async fn send(&mut self, data: &Bytes) -> Result<()> {
        if let Some(ref mut stream) = self.stream {
            stream.writable().await?;
            stream.write_all(data).await?;
            Ok(())
        } else {
            bail!("Trying to send message without connection")
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        if let Some(ref mut stream) = self.stream {
            let mut receiver = Receiver::new(stream);
            receiver.event_loop().await?;

            Ok(())
        } else {
            bail!("Trying to run server without connection")
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}

pub struct ServerRequest<'a, 'b> {
    message: Request<'a>,
    server: &'b mut Server,
    sequence_number: Option<usize>,
}

impl<'a, 'b> ServerRequest<'a, 'b> {
    pub fn from_message(server: &'b mut Server, message: Request<'a>) -> Self {
        Self {
            message,
            server,
            sequence_number: None,
        }
    }

    pub fn with_seq(&mut self) -> &mut Self {
        self.sequence_number = Some(self.server.get_next_sequence_number());
        self
    }

    pub async fn send(&mut self) -> Result<()> {
        let mut intermediate = self.message.serialize(RequestSerializer::new())?;
        if let Some(sequence_number) = self.sequence_number {
            intermediate.push(Field::from_u32("seq".to_string(), sequence_number as u32));
        }

        let data: Bytes = intermediate.to_bytes();
        trace!("Data: {:?}", data);
        self.server.send(&data).await?;

        Ok(())
    }
}
