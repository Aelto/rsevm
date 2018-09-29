extern crate rsevm;

use rsevm::answers::{answer_text, answer_file};
use rsevm::html::{Node};

macro_rules! attr {
  ( $($key: ident = $value:expr), *) => {
    [$( (stringify!($key).to_owned(), $value.to_owned()), )*]
      .iter().cloned().collect()
  };
}

fn main() {
  
  let mut server = rsevm::Server::new();

  server.get("/home", Box::new(|response| {
    answer_file(response, 200, "index.html")
  }));


  server.get("/", Box::new(|response| {
    let r = Node::new("div", "", attr!{ class = "parent" }, vec! {
      Node::new("p", "hello", attr!{}, vec![])
    });

    answer_text(response, 200, &r.render())
  }));

  server.listen();
}
