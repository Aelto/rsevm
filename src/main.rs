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

  server.get("/home", Box::new(|_request, response| {
    answer_file(response, 200, "index.html")
  }));


  server.get("/", Box::new(|_request, response| {
    let r = Node::new("div", "", attr!{ class = "parent" }, vec! {
      Node::new("p", "hello", attr!{}, vec![])
    });

    answer_text(response, 200, &r.render())
  }));

  server.get("/users/:id", Box::new(|req, res| {
    let user_id = match req.get_param(":id") {
      Some(expr) => expr,
      None => "0",
    };

    answer_text(res, 200, user_id)
  }));

  server.get("/books/:title/:language", Box::new(|req, res| {
    let title = match req.get_param(":title") {
      Some(expr) => expr,
      None => return answer_text(res, 400, "book title required"),
    };

    let language = match req.get_param(":language") {
      Some(lang) => lang,
      None => "english"
    };

    answer_text(res, 200, &format!("looking for book named {} in {}", title, language))
  }));

  server.post("/new-user", Box::new(|req, res| {
    let body = req.get_body();

    println!("{:?}", body);
    // answer_text(res, 200, body["message"])
  }));

  server.listen();
}
