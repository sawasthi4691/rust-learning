#![allow(dead_code)]
use http::request;
use http::Method;
use server::Server;
use website_handler::Websitehandler;
use std::env;

mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    dbg!(&default_path);
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080".to_string()); // this is associate function as it start with colon.
    server.run(Websitehandler::new(public_path));
}

fn extra_string_func() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..];
    let string_borrow: &str = &string;
    let string_literal = "1234";
    //println!("String slice {}", string_slice);

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);
}
