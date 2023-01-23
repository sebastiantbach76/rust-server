// Crate inclusion.
use std::io::{Result as IoResult, Write};

// Module inclusion.
use super::StatusCode;

// Response structure definition.
#[derive(Debug)]
pub struct Response
{
    status_code: StatusCode,
    body: Option<String>,
}

// Response structure implementation.
impl Response
{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self
    {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>
    {
        let body = match &self.body
        {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
