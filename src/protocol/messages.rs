use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub enum Request<'a> {
    Hello {
        htsp_version: u32,
        client_name: &'a str,
        client_version: &'a str,
    },
}

#[derive(Debug, Deserialize)]
pub enum Reply {
    Nop
}