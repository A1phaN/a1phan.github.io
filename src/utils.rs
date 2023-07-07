use markdown::mdast::Node;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct BuildMeta {
  pub timestamp: i64,
  pub post: usize,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Post {
  pub path: String,
  pub title: String,
  pub image: Option<String>,
  pub length: usize,
  pub create: usize,
  pub modify: usize,
}

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
