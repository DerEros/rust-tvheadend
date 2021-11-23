use crate::htps::field::{field, serialize as serialize_field, Convertible, ParsableField};
use anyhow::{anyhow, bail, Result};
use std::io::{Read, Write};

#[derive(Debug)]
pub enum Request {
    Hello {
        htsp_version: u32,
        client_name: &'static str,
        client_version: &'static str,
    },
}

#[derive(Debug)]
pub enum Reply {
    Hello {
        htsp_version: u32,
        server_name: String,
        server_version: String,
        challenge: Vec<u8>,
        server_capabilities: Vec<String>,
        webroot: Option<String>,
    },
}

use std::collections::HashMap;
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

pub fn deserialize(input: &mut dyn Read, method: &str) -> Result<Reply> {
    let mut consumed: usize = 0;
    let mut length_bytes = [0_u8; 4];
    input.read_exact(&mut length_bytes)?;

    let length = u32::from_be_bytes(length_bytes) as usize;
    let mut fields: HashMap<String, ParsableField> = HashMap::new();

    while consumed < length {
        let (bytes, field) = ParsableField::from_read(input)?;
        fields.insert(field.name(), field);
        consumed += bytes;
    }

    map2reply(fields, method)
}

fn map2reply(map: HashMap<String, ParsableField>, method: &str) -> Result<Reply> {
    match method {
        "hello" => Ok(Reply::Hello {
            htsp_version: get_field(&map, "htspversion")?,
            server_name: get_field(&map, "servername")?,
            server_version: get_field(&map, "serverversion")?,
            challenge: get_field(&map, "challenge")?,
            server_capabilities: get_list_field(&map, "servercapability")?,
            webroot: get_opt_field(&map, "webroot")?,
        }),
        _ => bail!("Unknown method returned: {}", method),
    }
}

fn get_field<T>(map: &HashMap<String, ParsableField>, field_name: &str) -> Result<T>
where
    ParsableField: Convertible<T>,
{
    map.get(field_name)
        .ok_or_else(|| anyhow!("Field missing in reply: {}", field_name))?
        .convert()
}

fn get_opt_field<T>(map: &HashMap<String, ParsableField>, field_name: &str) -> Result<Option<T>>
where
    ParsableField: Convertible<T>,
{
    let field_opt = map.get(field_name);
    if let Some(field) = field_opt {
        let converted = field.convert()?;
        Ok(Some(converted))
    } else {
        Ok(None)
    }
}

fn get_list_field<T>(map: &HashMap<String, ParsableField>, field_name: &str) -> Result<Vec<T>>
where
    ParsableField: Convertible<Vec<T>>,
{
    let list: Vec<T> = map
        .get(field_name)
        .ok_or_else(|| anyhow!("Field missing in reply: {}", field_name))?
        .convert()?;

    Ok(list)
}
