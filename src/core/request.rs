

pub struct Request<'a> {
  endpoint: &'a str,
  // request: &'a str,
}

impl<'a> Request<'a> {
    pub fn new(endpoint: &str) -> Request {
      Request {
        endpoint
      }
    }

    // pub fn get_param(variable: &str) -> &str {

    // }
}