

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
}