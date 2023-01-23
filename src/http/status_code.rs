// Crate inclusions.
use std::fmt::{Display, Formatter, Result as FmtResult};

// StatusCode enumeration definition.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode
{
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

// StatusCode enumeration implementation block.
impl StatusCode
{
    pub fn reason_phrase(&self) -> &str
    {
        match self
        {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

// Display trait implementation.
impl Display for StatusCode
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult
    {
        write!(f, "{}", *self as u16)
    }
}
