use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::collections::HashMap;

use super::response::Response;
use super::request::Request;
use super::route::Route;

type EndpointHandler = Box<Fn(&mut Response)>;

pub struct Server {
  pub address: String,
  pub endpoints: Vec<Route>,
}

impl Server {
  pub fn new() -> Server {
    Server {
      address: "127.0.0.1:3000".to_string(),
      endpoints: Vec::new(),
    }
  }

  pub fn listen(&self) {
    let listener = TcpListener::bind(self.address.clone())
      .expect("error occured at TcpListener.bind");

    println!("server started at {}", self.address);

    for stream in listener.incoming() {
      let stream = stream.unwrap();

      self.incoming_handler(stream);
    }
  }

  fn incoming_handler(&self, mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let full_request = String::from_utf8_lossy(&buffer);
    let end_endpoint_infos = full_request.find(" HTTP/1.1");

    if end_endpoint_infos.is_some() {
      let (request_route, _more) = full_request.split_at(end_endpoint_infos.unwrap());

      let matching_route = self.endpoints
        .iter()
        .find(|&route| route.does_match(request_route));

      if matching_route.is_some() {
        let endpoint_function = &matching_route.unwrap().action;
        
        let mut response = Response::new(&mut stream);
        endpoint_function(&mut response)
      }

      else {
        let response = format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 not found");
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
      }
      
      /*
      let endpoint_function = self.endpoints.get(endpoint);

      if endpoint_function.is_some() {
        let mut response = Response::new(&mut stream);
        endpoint_function.unwrap()(&mut response)
      }

      else {
        let response = format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 not found");
        
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
      }
      */
    }

    else {
      println!("incorrect request: {}", full_request);
    }
  }

  pub fn get(&mut self, route: &str, route_action: EndpointHandler) {
    self.endpoints.push(Route::new(&["GET", route].join(" "), route_action));
  }
}
