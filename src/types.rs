use serde::{Deserialize, Serialize};
use std::collections::HashMap;

macro_rules! create_enum {
  ($name:ident, $($variant:ident),*) => {
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Serialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub enum $name {
      $($variant),*
    }

    impl std::fmt::Display for $name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
          f,
          "{}",
          match self {
            $($name::$variant => stringify!($variant)),*
          }
        )
      }
    }

    impl std::str::FromStr for $name {
      type Err = ();

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          $(stringify!($variant) => Ok(Self::$variant),)*
          _ => Err(()),
        }
      }
    }

    impl $name {
      pub fn values() -> Vec<Self> {
        vec![$(Self::$variant),*]
      }
    }
  };
}

create_enum!(
  Category,
  Learning,
  Developing,
  Debugging,
  Hacking,
  Unclassified
);

impl Default for Category {
  fn default() -> Self {
    Self::Unclassified
  }
}

create_enum!(Tag, Blog, Linux, Network, Router, Rust, Swift, Yew);

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
