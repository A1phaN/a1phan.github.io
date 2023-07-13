use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fmt::{Display, Formatter},
  str::FromStr,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq)]
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

impl FromStr for Category {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Learning" => Ok(Self::Learning),
      "Developing" => Ok(Self::Developing),
      "Debugging" => Ok(Self::Debugging),
      "Hacking" => Ok(Self::Hacking),
      "Unclassified" => Ok(Self::Unclassified),
      _ => Err(()),
    }
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq)]
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

impl FromStr for Tag {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Blog" => Ok(Self::Blog),
      "Rust" => Ok(Self::Rust),
      "Yew" => Ok(Self::Yew),
      _ => Err(()),
    }
  }
}

impl Tag {
  pub fn values() -> Vec<Self> {
    vec![Self::Blog, Self::Rust, Self::Yew]
  }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostMeta {
  #[serde(default)]
  pub category: Category,
  #[serde(default)]
  pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Post {
  pub path: String,
  pub title: String,
  pub image: Option<String>,
  pub length: usize,
  pub create: usize,
  pub modify: usize,
  pub meta: PostMeta,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq)]
pub struct BuildMeta {
  pub timestamp: i64,
  pub post: usize,
  pub category: HashMap<Category, usize>,
  pub tag: HashMap<Tag, usize>,
}
