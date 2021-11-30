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

#[derive(Debug)]
pub enum Data {
    Bin(Bytes),
}

pub struct StreamContainer {
    address: String,
}

impl StreamContainer {
    pub fn create(address: String) -> Self {
        Self { address }
    }

    pub async fn run(&self) -> (UnboundedSender<StreamCommand>, UnboundedReceiver<Data>) {
        let (cmd_tx, cmd_rx) = unbounded();
        let (data_tx, data_rx) = unbounded();
        let addr = self.address.clone();
        tokio::spawn(async move {
            let stream = TcpStream::connect(addr).await.unwrap();
            Self::event_loop(stream, cmd_rx, data_tx).await;
        });

        (cmd_tx, data_rx)
    }

    async fn event_loop(
        mut stream: TcpStream,
        mut command_in: UnboundedReceiver<StreamCommand>,
        mut data_out: UnboundedSender<Data>,
    ) {
        loop {
            trace!("Running...");
            select! {
                Some(data) = command_in.next() => {
                    trace!("Received: {:?}", data);
                    Self::handle_stream_command(&mut stream, data).await;
                },
                Ok(message_size) = stream.read_u32() => {
                    trace!("Announced message of {} bytes", message_size);
                    Self::handle_message_incoming(&mut stream, message_size, &mut data_out).await;
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

    async fn handle_message_incoming(
        stream: &mut TcpStream,
        expected_size: u32,
        out: &mut UnboundedSender<Data>,
    ) {
        let mut buffer = BytesMut::with_capacity(expected_size as usize);
        let _ = stream.read_buf(&mut buffer).await;

        trace!("Received data: {:?}", buffer);
        let _ = out.unbounded_send(Data::Bin(buffer.freeze()));
    }
}
