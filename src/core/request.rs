

pub struct Request<'a> {
  endpoint: &'a str,
  request: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(endpoint: &'a str, request: &'a str) -> Request<'a> {
      Request {
        endpoint: &endpoint,
        request: &request
      }
    }

    // pub fn get_param(variable: &str) -> &str {

    // }
}