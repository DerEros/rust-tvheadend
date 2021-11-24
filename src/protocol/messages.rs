use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum Request<'a> {
    Hello {
        htsp_version: u32,
        client_name: &'a str,
        client_version: &'a str,
    },
}
