use crate::protocol::intermediate::Field;
use crate::{Request, RequestSerializer, ToBytes};
use anyhow::{bail, Result};
use bytes::{Bytes, BytesMut};
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::StreamExt;
use log::*;
use serde::Serialize;
use std::sync::RwLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

pub struct Server {
    address: String,
    next_sequence_number: RwLock<usize>,
    stream_channel: Option<UnboundedSender<StreamCommand>>,
}

impl Server {
    pub fn new<T: AsRef<str>>(address: T) -> Self {
        Self {
            address: address.as_ref().to_string(),
            next_sequence_number: RwLock::new(0),
            stream_channel: None,
        }
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
        if let Some(channel) = self.stream_channel.as_ref() {
            let _ = channel.unbounded_send(StreamCommand::Send(data.clone()));
            Ok(())
        } else {
            bail!("Trying to send message without connection")
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let tx = StreamContainer::create(self.address.clone()).run().await;
        self.stream_channel = Some(tx);
        // let mut receiver = Receiver::new(stream);
        // receiver.event_loop().await?;
        Ok(())
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

#[derive(Debug)]
pub enum StreamCommand {
    Send(Bytes),
    Stop,
}

struct StreamContainer {
    address: String,
}

impl StreamContainer {
    pub fn create(address: String) -> Self {
        Self { address }
    }

    pub async fn run(&self) -> UnboundedSender<StreamCommand> {
        let (tx, rx) = unbounded();
        let addr = self.address.clone();
        tokio::spawn(async move {
            let stream = TcpStream::connect(addr).await.unwrap();
            Self::event_loop(stream, rx).await;
        });

        tx
    }

    async fn event_loop(mut stream: TcpStream, mut rx: UnboundedReceiver<StreamCommand>) {
        loop {
            trace!("Running...");
            select! {
                Some(data) = rx.next() => {
                    trace!("Received: {:?}", data);
                    Self::handle_stream_command(&mut stream, data).await;
                },
                Ok(message_size) = stream.read_u32() => {
                    trace!("Announced message of {} bytes", message_size);
                    Self::handle_message_incoming(&mut stream, message_size).await;
                }
                else => break
            }
        }
    }

    async fn handle_stream_command(stream: &mut TcpStream, command: StreamCommand) {
        match command {
            StreamCommand::Send(data) => {
                trace!("Received send");
                let _ = stream.writable().await;
                let _ = stream.write_all(&data).await;
            }
            StreamCommand::Stop => trace!("Received stop"),
        }
    }

    async fn handle_message_incoming(stream: &mut TcpStream, expected_size: u32) {
        let mut buffer = BytesMut::with_capacity(expected_size as usize);
        let _ = stream.read_buf(&mut buffer).await;

        trace!("Received data: {:?}", buffer);
    }
}
