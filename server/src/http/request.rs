use super::method::{Method, MethodError};
use super::{QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method, //super keyword goes to parent module and than search in parent module.
}


//getter method.
impl<'buf> Request<'buf> {
    pub fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

//TryForm : it help in conversion.
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        /*  match str::from_utf8(buf) {
            Ok(result) => {}
            Err(_) =>  return Err(ParseError::InvalidEncoding),
        }
        match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
            Ok(request) => {}
            Err(e) => return Err(e),
        }
        str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;*/

        //to direct add ? we need to implement from fn for parseError. above code is converted to below code. ? behave as match.
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        //find return usize variable if value exists or None in case of no find is found.
        // various ways to extract path value.
        /* match path.find('?') {
            Some(i) => {
                 query_string = Some(&path[i+1..]);
                 path = &path[..i];
            },
            None => {}
        }

        let size =  path.find('?');
        if size.is_some() {
            let s = size.unwrap();
            query_string = Some(&path[s+1..]);
            path = &path[..s];
        } */

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    //this iterator over string.
    /* let mut iter = request.chars();
    loop {
        let item = iter.next();
        match item {
            Some(c) => {}
            None => break,
        }
    } */

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

impl From<MethodError> for ParseError {
    fn from(value: MethodError) -> Self {
        Self::InvalidMethod
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", &self.message())
    }
}

impl ParseError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Error for ParseError {}
