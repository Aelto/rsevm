
use std::collections::HashMap;

pub struct Node {
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
      children,
      attributes,
      tag: tag.to_owned(),
      text: text.to_owned(),
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
