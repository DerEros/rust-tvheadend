use anyhow::Result;
use crate::htps::field::{field, serialize as serialize_field};
use std::io::Write;

#[derive(Debug)]
pub enum Request {
    Hello {
        htsp_version: u32,
        client_name: &'static str,
        client_version: &'static str,
    },
}

use Request::*;
pub fn serialize(req: Request, out: &mut dyn Write) -> Result<()> {
    let mut body: Vec<u8> = vec![];
    serialize_body(req, &mut body)?;

    out.write_all(&(body.len() as u32).to_le_bytes())?;
    out.write_all(body.as_slice())?;

    Ok(())
}

fn serialize_body(req: Request, out: &mut dyn Write) -> Result<()> {
    match req {
        Hello {
            htsp_version,
            client_name,
            client_version,
        } => {
            serialize_field(&field("method", "hello"), out)?;
            serialize_field(&field("htsp_version", htsp_version), out)?;
            serialize_field(&field("client_name", client_name), out)?;
            serialize_field(&field("client_version", client_version), out)?;
        }
    }

    Ok(())
}
