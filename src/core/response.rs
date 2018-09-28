use std::net::TcpStream;
use std::io::prelude::*;

pub struct Response<'a> {
  pub stream: &'a mut TcpStream,
}

impl<'a> Response<'a> {
  pub fn new(stream: &'a mut TcpStream) -> Response {
    Response {
      stream: stream
    }
  }

  pub fn set_response(&mut self, content: &String) {
    self.stream.write(content.as_bytes()).unwrap();
    self.stream.flush().unwrap();
  }
}