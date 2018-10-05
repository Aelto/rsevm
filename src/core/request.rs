extern crate serde_json;

pub struct Request<'a> {
  endpoint: &'a str,
  request: &'a str,
  full_request: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(endpoint: &'a str, request: &'a str, full_request: &'a str) -> Request<'a> {
      Request {
        endpoint: &endpoint,
        request: &request,
        full_request: &full_request,
      }
    }

    pub fn get_param(&self, variable: &str) -> Option<&str> {
      let endpoint_split = self.endpoint.split("/");
      let request_split = self.request.split("/");

      let param = endpoint_split
        .zip(request_split)
        .find(|&(param, _value)| param == variable);

      return match param {
         Some((_param, value)) => Some(value),
         None => None,
       };
    }

    pub fn get_body(&self) -> Option<serde_json::Value> {
      let mut lines = self.full_request
        .lines();

      println!("{:?}", lines.nth(3));

      let content_length = match lines.nth(3) {
        Some(line) => line
          .chars()
          .skip_while(|&c| c != ' ')
          .skip(1)
          .collect() // convert this in a string to allow ::parse()
          .parse::<u32>().unwrap(),
        None => return None,
      };

      let body_line = lines
        .nth(10);

      println!("{:?}", body_line);

      match body_line {
        Some(line) => {
          serde_json::from_str(&String::from(line)).ok()
        },
        None => None,
      }
    }  
}