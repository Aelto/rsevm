 use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

extern crate rsevm;

macro_rules! answer {
  ($stream:ident, $status:expr, $response:expr) => {
    $stream.set_response(
      &format!("HTTP/1.1 {} OK\r\n\r\n{}", stringify!($expression), $response)
    )
  }
}

macro_rules! map {
  ( $(key:ident : $value:expr),*) => {
    [$(
      (stringify!($key), $value),
    )*]
    .iter().cloned().collect()
  };
}


// macro_rules! html {
//   ($open:ident $(op:tt)*) => {
//     format!("{}</{}>", stringify!{op})
//   };

//   // (< $tag:ident > </ $closing_tag:ident >) => {
//   //   format!("<{}></{}>", stringify!{tag}, stringify!{closing_tag})
//   // };

//   // ($($token:tt)*) => {
//   //   stringify!{token}
//   // };

// }

struct Node {
  pub children: Vec<Node>,
  pub text: String,
  pub tag: String,
  pub attributes: HashMap<String, String>,
}

impl Node {
  pub fn new_empty() -> Node {
    Node {
      children: vec!{},
      tag: String::new(),
      text: String::new(),
      attributes: HashMap::new(),
    }
  }

  pub fn new(tag: &str, text: &str, attributes: HashMap<String, String>, children: Vec<Node>) -> Node {
    Node {
      children: children,
      tag: tag.to_owned(),
      text: text.to_owned(),
      attributes: attributes
    }
  }

  pub fn render(self) -> String {
    let len = self.children.len();

    let attributes_string = self.attributes
        .iter()
        .fold(String::new(), |acc, (key, value)| acc + &format!("{}='{}'", key, value));

    if len == 0 {
      format!("<{} {}>{}</{}>", self.tag, attributes_string, self.text, self.tag)
    }

    else {
      let children_result = self.children
        .into_iter()
        .fold(String::new(), |acc, child| acc + &child.render());

      format!("<{} {}>{}</{}>", self.tag, attributes_string, children_result, self.tag)
    }
  }
}

macro_rules! html {
  ($($body:tt)*) => {{
    let mut dom_nodes: Vec<Node> = vec![];

    {
      let mut current_node: Option<Node> = None;
      html_recurse! { dom_nodes current_node $($body)* };
    }

    let mut attributes = HashMap::new();
    attributes.insert("class".to_owned(), "parent".to_owned());
    
    Node::new("div", "", attributes, dom_nodes);
  }};
}

macro_rules! html_recurse {
  // tag with props
  // < div class = "..." >
  ( $dom_nodes:ident $current_node:ident < $opening_tag:ident $(remaining:tt)+ ) => {
    // open scope
    let node = Node::new_empty();
    node.tag = stringify!(ident);

    // continue parsing
    html_recurse! { dom_nodes current_node $($body)* };
  };

  // tag without props
  // < div >
  ( $dom_nodes:ident $current_node:ident < $opening_tag:ident > $(remaining:tt)*) => {

  };

  ( $dom_nodes:ident $current_node:ident > $(remaining:tt)* ) => {
    // close scope
    
    
  };

  () => {
    ()
  };
}

fn main() {
  let mut server = rsevm::Server::new();

  // server.endpoints.insert("GET /".to_owned(), Box::new(|response| {
  //   let mut file = File::open("index.html")
  //     .expect("could not open index.html");

  //   let mut file_content = String::new();
  //   file.read_to_string(&mut file_content).unwrap();
    
  //   answer!(response, 200, file_content)
  // }));

  server.endpoints.insert("GET /".to_owned(), Box::new(|response| {
    let mut attributes = HashMap::new();
    attributes.insert("class".to_owned(), "parent".to_owned());
    let r = Node::new("div", "", attributes, vec! {
      Node::new("p", "hello", map!{}, vec![])
    });

    answer!(response, 200, r.render())
  }));

  

  server.endpoints.insert("GET /home".to_owned(), Box::new(|response| {
    let text = String::from("<body>home</body>");
    
    answer!(response, 200, text)
  }));

  html! {
    < div > 
      < p > hello < / p >
    < / div >
  }

  server.listen();
}
