use super::StatusCode;
//use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::{io::{Write, Result as IoResult}};

#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status: StatusCode, body: Option<String>) -> Self {
        Response { status, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n {} ", 
                self.status, 
                self.status.reason_phrase(), 
                body)
    }
}

/*impl Display for Response {
    
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(f, "HTTP/1.1 {} {}\r\n\r\n {} ", 
                self.status, 
                self.status.reason_phrase(), 
                body)
    }
}*/
