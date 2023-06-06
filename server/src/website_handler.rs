use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct Websitehandler {
    public_string: String,
}

impl Websitehandler {
    pub fn new(public_string: String) -> Self {
        Self { public_string }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_string, file_path);
        //dbg!(&path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_string) {
                    fs::read_to_string(path).ok()
                }else {
                    println!("Directory tracersal Attack Attempted {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for Websitehandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        //dbg!(self.read_file("index.html"));
        //dbg!(self.read_file("hello.html"));
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::OK, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) =>  Response::new(StatusCode::OK,Some(contents)),
                    None =>  Response::new(StatusCode::NotFound,None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
        //Response::new(StatusCode::OK, Some("<h1>TEST</h2>".to_string()))
    }
}
