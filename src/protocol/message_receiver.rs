use anyhow::Result;
use bytes::BytesMut;
use futures::channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use log::*;
use std::sync::RwLock;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Receiver<'a> {
    stream: &'a mut TcpStream,
    stop: RwLock<bool>,
    cancel_sender: UnboundedSender<bool>,
    cancel_receiver: UnboundedReceiver<bool>,
}

impl<'a> Receiver<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        let (tx, rx) = unbounded();
        Self {
            stream,
            stop: RwLock::new(false),
            cancel_sender: tx,
            cancel_receiver: rx,
        }
    }

    pub async fn event_loop(&mut self) -> Result<()> {
        trace!("Entering event loop");
        while !self.must_stop() {
            trace!("Waiting for server message");
            self.stream.readable().await?;
            let msg_len = self.stream.read_u32().await?;
            trace!("Reading {} bytes of data", msg_len);

            let mut buffer = BytesMut::with_capacity(msg_len as usize);
            self.stream.read_buf(&mut buffer).await?;
            trace!("Received bytes: {}", buffer.len());

            trace!("Received data: {:?}", buffer);
        }
        Ok(())
    }

    fn must_stop(&self) -> bool {
        *self.stop.read().unwrap()
    }
}
