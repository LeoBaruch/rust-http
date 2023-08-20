use super::router::Router;
use http::httprequest::HttpRequest;
use std::{
    io::{BufRead, BufReader},
    net::TcpListener,
};
pub struct Server<'a> {
    socked_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Server { socked_addr: addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socked_addr).unwrap();
        println!("Running on {}", self.socked_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            let mut reader = BufReader::new(&mut stream);
            let mut request = String::new();
            let _ = reader.read_line(&mut request);
            println!("request: {:?}", request);
            let req: HttpRequest = request.into();
            println!("req: {:?}", req);
            Router::route(req, &mut stream);
        }
    }
}
