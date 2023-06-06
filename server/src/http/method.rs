use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    DELETE,
    PATCH,
    HEAD,
    TRACE,
    CONNECT,
    OPTIONS,
}

//overridden as per our enum Method.
impl FromStr for Method {
    type Err = MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
