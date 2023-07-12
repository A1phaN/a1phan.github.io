use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
};

#[derive(Clone, Copy, Deserialize, Eq, Hash, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Category {
  Learning,
  Developing,
  Debugging,
  Hacking,
  Unclassified,
}

impl Default for Category {
  fn default() -> Self {
    Self::Unclassified
  }
}

impl Display for Category {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Category::Learning => "Learning",
        Category::Developing => "Developing",
        Category::Debugging => "Debugging",
        Category::Hacking => "Hacking",
        Category::Unclassified => "Unclassified",
      }
    )
  }
}

impl Category {
  pub fn values() -> Vec<Self> {
    vec![
      Self::Learning,
      Self::Developing,
      Self::Debugging,
      Self::Hacking,
      Self::Unclassified,
    ]
  }
}

#[derive(Clone, Copy, Deserialize, Eq, Hash, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum Tag {
  Blog,
  Rust,
  Yew,
}

impl Display for Tag {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Tag::Blog => "Blog",
        Tag::Rust => "Rust",
        Tag::Yew => "Yew",
      }
    )
  }
}

impl Tag {
  pub fn values() -> Vec<Self> {
    vec![Self::Blog, Self::Rust, Self::Yew]
  }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct PostMeta {
  #[serde(default)]
  pub category: Category,
  #[serde(default)]
  pub tags: Vec<Tag>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Post {
  pub path: String,
  pub title: String,
  pub image: Option<String>,
  pub length: usize,
  pub create: usize,
  pub modify: usize,
  pub meta: PostMeta,
}

#[derive(Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct BuildMeta {
  pub timestamp: i64,
  pub post: usize,
  pub category: HashMap<Category, usize>,
  pub tag: HashMap<Tag, usize>,
}
