use std::fmt::{Display, Formatter, Result as FmtResult};

//copy and clone is done bcoz we are coping u8 to u16 in line 25: *self as u16. To do deep copy we are derving copy and clone trait.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "Ok",
            Self::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            _ => "",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
