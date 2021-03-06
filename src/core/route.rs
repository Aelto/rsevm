
use super::request::Request;

type EndpointHandler = Box<Fn(Request) -> Result<String, (u16, String)>>;

pub struct Route {
  pub route: String,
  pub action: EndpointHandler,
}

impl Route {
  pub fn new(route: &str, action: EndpointHandler) -> Route {
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