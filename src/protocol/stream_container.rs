use log::*;

use bytes::{Bytes, BytesMut};
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::stream::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::select;

#[derive(Debug)]
pub enum StreamCommand {
    Send(Bytes),
    Stop,
}

pub struct StreamContainer {
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
