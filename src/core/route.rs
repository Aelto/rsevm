
use super::request::Request;

pub type EndpointHandlerOkResponse<T> = (String, T);
pub type EndpointHandlerErrResponse = (u16, String);
pub type EndpointHandlerFn<T> = dyn Fn(Request, T) -> Result<EndpointHandlerOkResponse<T>, EndpointHandlerErrResponse>;
pub type EndpointHandler<T> = Box<EndpointHandlerFn<T>>;

pub struct Route<T> {
  pub route: String,
  pub action: EndpointHandler<T>,
}

impl<T> Route<T> {
  pub fn new(route: &str, action: EndpointHandler<T>) -> Route<T> {
    Route {
      route: route.to_string(),
      action
    }
  }

  pub fn does_match(&self, request: &str) -> bool {
    let mut self_route_split = self.route.split("/");
    let mut request_route_split = request.split("/");

    return self_route_split
      .all(|s| {
        match request_route_split.next() {
          Some(expr) => s.starts_with(":") || expr == s,
          None => s.starts_with(":") || false,
        }
      })
  }
}