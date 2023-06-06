use crate::http::{ParseError, Request, Response, StatusCode};
use std::{
    convert::TryFrom,
    io::{Read, Write},
    net::TcpListener,
};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn request_bad_handler(&mut self, e: &ParseError) -> Response {
        println!("Recied Error {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

//Struct is like a class in java , consists of similar datatype.
pub struct Server {
    addr: String,
}

// impl consists of simliar function of a Struct.
impl Server {
    //This is an associte function , or we can say construct for struct Server.
    pub fn new(addr: String) -> Self {
        //Self = Server
        //Server can be replaces Self word as it's represent the struct name.
        Self { addr: addr }
    }

    //This is a method of Struct Server.
    pub fn run(self, mut handler: impl Handler) {
        //self is like "this" from java, method first variable is always self keyword.
        println!("Server is listening {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // here _ represent it can be any value.
                    let mut buffer = [0; 1024]; // IN rust we need to initailize array always and need to give the size. 1024 = size. 0 represent the value
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request : {} ", String::from_utf8_lossy(&buffer));
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    //Response::new(StatusCode::OK, Some("<h1>It worked.</h1>".to_string()))
                                    //write!(stream, "{}" , response);
                                    //response.send(&mut stream);
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    dbg!(&e);
                                    //Response::new(StatusCode::BadRequest, Some("<h1>bad Reqeust</h1>".to_string()))
                                    handler.request_bad_handler(&e)
                                }
                            };
                            //let result: &Result<Request, _> = &buffer[..].try_into();
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response : {} ", e);
                            }
                        }
                        Err(e) => println!("Failed to read connection {} ", e),
                    }
                }
                Err(e) => println!("Failed to establish connection {} ", e),
                // _ => println!("Default"),   // default case like in switch of java.
            }
        }
    }
}
