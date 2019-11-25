extern crate rsevm;
extern crate serde_json;
extern crate serde;
extern crate flame;

use rsevm::answers::{answer_file};
use rsevm::html::{Node};
use std::sync::atomic::{AtomicBool, Ordering};

macro_rules! attr {
  ( $($key: ident = $value:expr), *) => {
    [$( (stringify!($key).to_owned(), $value.to_owned()), )*]
      .iter().cloned().collect()
  };
}

#[derive(serde::Deserialize, Clone)]
struct User {
  name: String
}

#[derive(Clone)]
struct State {
  users: Vec<User>,
  counter: u8,
}

impl rsevm::ServerState for State {
  fn new() -> State {
    State {
      users: Vec::new(),
      counter: 0
    }
  }
}

fn main() {
  let mut server = rsevm::Server::<State>::new();

  server.get("/home", Box::new(|_request, state| {
    Ok((answer_file("index.html"), state))
  }));

  server.get("/", Box::new(|_request, state| {
    let r = Node::new("div", "", attr!{ class = "parent" }, vec! {
      Node::new("p", "hello", attr!{}, vec![])
    });

    Ok((r.render(), state))
  }));

  server.get("/counter", Box::new(|_request, mut state| {
    state.counter += 1;

    Ok((format!("counter: {}", state.counter), state))
  }));

  server.get("/users/:id", Box::new(|req, state| {
    match req.get_param(":id") {
      Some(id) => {
        let id = id.parse::<usize>().unwrap();

        if id >= state.users.len() {
          Err((404, "no such user".to_string()))
        }
        else {
          Ok((state.users[id].name.clone(), state))
        }
      },
      None => Err((403, "no id provided".to_string()))
    }
  }));

  server.post("/new-user", Box::new(|req, mut state| {
    let body = req.get_body()
      .ok_or((500, "no body data received".to_string()))?;

    let data: User = serde_json::from_value(body)
      .map_err(|_e| (500, "could not parse json body".to_string()))?;

    let username = data.name.clone();
    state.users.push(data);

    Ok((format!("user {} created", username), state))
  }));

  use std::fs::File;

  
  server.listen();

  flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
