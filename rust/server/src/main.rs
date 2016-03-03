extern crate hyper;

use hyper::Server;
use hyper::server::{Request, Response};
use std::io::Read;

fn server(mut req: Request, res: Response) {
    println!("Plain data received from {}", req.remote_addr);
    let mut input = String::new();
    req.read_to_string(&mut input);
    res.send(format!
        ("Gotcha! Here's your data back: {}", input).as_bytes())
        .unwrap();
}

fn main() {
    println!("Server going to listen on port 80");
    Server::http("127.0.0.1:80").unwrap()
        .handle(server).unwrap();
}
