extern crate rsevm;
extern crate serde_json;
extern crate serde;

use rsevm::answers::{answer_file};
use rsevm::html::{Node};

use std::sync::{Arc, Mutex, MutexGuard};

macro_rules! attr {
  ( $($key: ident = $value:expr), *) => {
    [$( (stringify!($key).to_owned(), $value.to_owned()), )*]
      .iter().cloned().collect()
  };
}

#[derive(serde::Deserialize)]
struct User {
  name: String
}

fn main() {
  let user_list = Arc::new(Mutex::new(Vec::new()));
  let mut server = rsevm::Server::new();

  server.get("/home", Box::new(|_request| {
    Ok(answer_file("index.html"))
  }));

  server.get("/", Box::new(|_request| {
    let r = Node::new("div", "", attr!{ class = "parent" }, vec! {
      Node::new("p", "hello", attr!{}, vec![])
    });

    Ok(r.render())
  }));

  let users = user_list.clone();
  server.get("/users/:id", Box::new(move |req| {
    match req.get_param(":id") {
      Some(id) => {
        let users: MutexGuard<Vec<User>> = users.lock().unwrap();
        let id = id.parse::<usize>().unwrap();

        if id >= users.len() {
          Err((404, "no such user".to_string()))
        }
        else {
          Ok(users[id].name.clone())
        }
      },
      None => Err((403, "no id provided".to_string()))
    }
  }));

  let users = user_list.clone();
  server.post("/new-user", Box::new(move |req| {
    let body = req.get_body()
      .ok_or((500, "no body data received".to_string()))?;

    let data: User = serde_json::from_value(body)
      .map_err(|_e| (500, "could not parse json body".to_string()))?;

    let username = data.name.clone();
    users.lock().unwrap().push(data);

    Ok(format!("user {} created", username))
  }));

  server.listen();
}
