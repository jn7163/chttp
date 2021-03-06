extern crate chttp;
extern crate http;

use http::Request;
use std::io::Read;


fn main() {
    let request = Request::get("http://example.org").body(chttp::Body::Empty).unwrap();

    let mut transport = chttp::transport::Transport::new();
    transport.send(request).unwrap();

    let mut body = String::new();
    transport.read_to_string(&mut body).unwrap();

    println!("{}", body);
}
