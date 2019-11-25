extern crate ctrlc;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use super::response::Response;
use super::request::Request;
use super::route::{
  Route,
  EndpointHandler,
};

pub trait ServerState {
  fn new() -> Self;
}

pub struct Server<T: ServerState>
  where T: Clone {
  pub address: String,
  pub endpoints: Vec<Route<T>>,
  pub state: T,
  pub running: Arc<AtomicBool>
}

impl<T: ServerState> Server<T>
  where T: Clone {

  pub fn new() -> Server<T> {
    Server {
      address: "127.0.0.1:3000".to_string(),
      endpoints: Vec::new(),
      state: T::new(),
      running: Arc::new(AtomicBool::new(false)),
    }
  }

  pub fn listen(&mut self) {
    let listener = TcpListener::bind(self.address.clone())
      .expect("error occured at TcpListener.bind");

    println!("server started at {}", self.address);
    println!("use ctrl+c to exit.");

    self.running.store(true, Ordering::SeqCst);

    let running = self.running.clone();
    ctrlc::set_handler(move || {
      println!("server closing");

      running.store(false, Ordering::SeqCst);

      std::process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    for stream in listener.incoming() {
      if !self.running.load(Ordering::SeqCst) {
        break;
      }
      
      let stream = stream.unwrap();

      self.incoming_handler(stream);
    }

    println!("server closed.")
  }

  fn incoming_handler(&mut self, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let full_request = String::from_utf8_lossy(&buffer);
    let end_endpoint_infos = full_request.find(" HTTP/1.1");

    if end_endpoint_infos.is_some() {
      let (request_route, _more) = full_request.split_at(end_endpoint_infos.unwrap());

      let matching_route = self.endpoints
        .iter()
        .find(|&route| route.does_match(request_route));

      if matching_route.is_some() {
        let endpoint_route = matching_route.unwrap();
        let request = Request::new(&endpoint_route.route, &request_route, &full_request);
        let endpoint_function = &endpoint_route.action;
        let mut response = Response::new(&mut stream);
        
        match endpoint_function(request, self.state.clone()) {
          Ok((text, state)) => self.handle_endpoint_ok(&mut response, &text, &state),
          Err((code, text)) => self.handle_endpoint_err(&mut response, code, &text)
        };
      }

      else {
        let response = format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", "404 not found");
        
        stream
          .write(response.as_bytes())
          .unwrap();

        stream
          .flush()
          .unwrap();
      }
    }

    else {
      println!("incorrect request: {}", full_request);
    }
  }

  fn handle_endpoint_ok(&mut self, response: &mut Response, text: &String, state: &T) {
    response.set_response(&format!("HTTP/1.1 200 OK\r\n\r\n{}", text));

    self.state = state.clone();
  }

  fn handle_endpoint_err(&mut self, response: &mut Response, code: u16, text: &String) {
    response.set_response(&format!("HTTP/1.1 {} OK\r\n\r\n{}", code, text));
  }

  pub fn get(&mut self, route: &str, route_action: EndpointHandler<T>) {
    self
      .endpoints
      .push(Route::new(&["GET", route].join(" "), route_action));
  }

  pub fn post(&mut self, route: &str, route_action: EndpointHandler<T>) {
    self
      .endpoints
      .push(Route::new(&["POST", route].join(" "), route_action));
  }
}
