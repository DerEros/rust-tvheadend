use log::*;
use anyhow::Result;
use crate::htps::field::{field, serialize as serialize_field, ParsableField, Convertible};
use std::io::{Write, Read};

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

    out.write_all(&(body.len() as u32).to_be_bytes())?;
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
            serialize_field(&field("htspversion", htsp_version), out)?;
            serialize_field(&field("clientname", client_name), out)?;
            serialize_field(&field("clientversion", client_version), out)?;
        }
    }

    Ok(())
}

pub fn deserialize(input: &mut dyn Read) -> Result<()> {
    let mut consumed: usize = 0;
    let mut length_bytes = [0_u8; 4];
    input.read_exact(&mut length_bytes)?;

    let length = u32::from_be_bytes(length_bytes) as usize;

    while consumed < length {
        let (bytes, field) = ParsableField::from_read(input)?;
        consumed += bytes;

        warn!("Field: {:?}", field);
        if field.field_type() == 3 {
            let s: String = field.convert()?;
            warn!("  Str: {}", s);
        }
        if field.field_type() == 2 {
            let u: u32 = field.convert()?;
            warn!("  S64: {}", u);
        }
    }

    Ok(())
}