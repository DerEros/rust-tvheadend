use crate::protocol::intermediate::Field;
use crate::protocol::stream_container::{Data, StreamCommand, StreamContainer};
use crate::{Request, RequestSerializer, ToBytes};
use anyhow::{bail, Result};
use bytes::Bytes;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use log::*;
use serde::Serialize;
use std::sync::RwLock;

pub struct Server {
    address: String,
    next_sequence_number: RwLock<usize>,
    send_channel: Option<UnboundedSender<StreamCommand>>,
    receive_channel: Option<UnboundedReceiver<Data>>,
}

impl Server {
    pub fn new<T: AsRef<str>>(address: T) -> Self {
        Self {
            address: address.as_ref().to_string(),
            next_sequence_number: RwLock::new(0),
            send_channel: None,
            receive_channel: None,
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
        if let Some(channel) = self.send_channel.as_ref() {
            let _ = channel.unbounded_send(StreamCommand::Send(data.clone()));
            Ok(())
        } else {
            bail!("Trying to send message without connection")
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let (tx, rx) = StreamContainer::create(self.address.clone()).run().await;
        self.send_channel = Some(tx);
        self.receive_channel = Some(rx);
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
