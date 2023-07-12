use markdown::{mdast::Node, Constructs, ParseOptions};

pub fn find_first_element<T>(node: &Node, predict: fn(node: &Node) -> Option<T>) -> Option<T> {
  if let Some(elem) = predict(node) {
    Some(elem)
  } else {
    if let Some(children) = node.children() {
      for child in children {
        if let Some(elem) = find_first_element(child, predict) {
          return Some(elem);
        }
      }
    }
    None
  }
}

pub fn get_text(node: &Node) -> String {
  if let Node::Text(text) = node {
    text.value.clone()
  } else {
    let mut text = String::new();
    if let Some(children) = node.children() {
      for child in children {
        text += &get_text(child);
      }
    }
    text
  }
}

pub fn parse_options() -> ParseOptions {
  ParseOptions {
    constructs: Constructs {
      frontmatter: true,
      gfm_table: true,
      ..Default::default()
    },
    ..Default::default()
  }
}
