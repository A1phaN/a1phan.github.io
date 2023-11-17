use blog::{
  types::{BuildMeta, Category, Post, PostMeta, Tag},
  utils::{find_first_element, get_text, parse_options},
};
use chrono::prelude::*;
use markdown::mdast::Node;
use std::io::Write;
use std::{
  collections::HashMap,
  fs::{self, File},
  io::{Read, Result},
  path::Path,
  process::Command,
};

fn main() -> Result<()> {
  let posts = Command::new("git").args(["ls-files", "post"]).output()?;
  let mut posts = unsafe { String::from_utf8_unchecked(posts.stdout) }
    .trim()
    .split("\n")
    .filter(|path| path.ends_with(".md"))
    .map(Path::new)
    .map(|path| {
      let mut content = String::new();
      File::open(path)
        .expect(&format!("failed to open file {}", path.to_str().unwrap()))
        .read_to_string(&mut content)
        .expect(&format!("failed to read file {}", path.to_str().unwrap()));
      let length = content.chars().count();
      let mdast = markdown::to_mdast(&content, &parse_options())
        .expect(&format!("failed to parse {}", path.to_str().unwrap()));
      let path = path.to_str().unwrap();
      Post {
        path: path
          .strip_prefix("post/")
          .unwrap()
          .strip_suffix(".md")
          .unwrap()
          .to_string(),
        title: find_first_element(&mdast, |node| match node {
          Node::Heading(heading) => {
            if heading.depth == 1 {
              Some(get_text(node))
            } else {
              None
            }
          }
          _ => None,
        })
        .unwrap_or("Untitled".to_string()),
        image: find_first_element(&mdast, |node| match node {
          Node::Image(image) => Some(image.url.clone()),
          _ => None,
        }),
        length,
        create: unsafe {
          String::from_utf8_unchecked(
            Command::new("git")
              .args([
                "log",
                "-1",
                "--diff-filter=A",
                "--pretty=format:%ct",
                "--",
                path,
              ])
              .output()
              .expect(&format!("failed to get create time of {path}"))
              .stdout,
          )
        }
        .parse()
        .expect("invalid create time"),
        modify: unsafe {
          String::from_utf8_unchecked(
            Command::new("git")
              .args(["log", "-1", "--pretty=format:%ct", "--", path])
              .output()
              .expect(&format!("failed to get modify time of {path}"))
              .stdout,
          )
        }
        .parse()
        .expect("invalid modify time"),
        meta: find_first_element(&mdast, |node| match node {
          Node::Toml(toml) => Some(toml.value.clone()),
          _ => None,
        })
        .map(|meta| toml::from_str::<PostMeta>(&meta).unwrap())
        .unwrap(),
      }
    })
    .collect::<Vec<_>>();
  posts.sort_by_key(|post| usize::MAX - post.create);
  let mut meta = BuildMeta {
    timestamp: Local::now().timestamp(),
    post: posts.len(),
    category: HashMap::new(),
    tag: HashMap::new(),
  };

  fs::create_dir_all(&Path::new("meta"))?;
  for (idx, page) in posts.chunks(10).enumerate() {
    File::create(&Path::new(&format!("meta/{}.json", idx + 1)))?
      .write_all(serde_json::to_string(page)?.as_bytes())?;
  }
  for category in Category::values() {
    let posts = posts
      .iter()
      .filter(|post| post.meta.category == category)
      .collect::<Vec<_>>();
    meta.category.insert(category, posts.len());
    for (idx, page) in posts.chunks(10).enumerate() {
      fs::create_dir_all(&Path::new(&format!("meta/category/{}", category)))?;
      File::create(&Path::new(&format!(
        "meta/category/{}/{}.json",
        category,
        idx + 1,
      )))?
      .write_all(serde_json::to_string(page)?.as_bytes())?;
    }
  }
  for tag in Tag::values() {
    let posts = posts
      .iter()
      .filter(|post| post.meta.tags.contains(&tag))
      .collect::<Vec<_>>();
    meta.tag.insert(tag, posts.len());
    for (idx, page) in posts.chunks(10).enumerate() {
      fs::create_dir_all(&Path::new(&format!("meta/tag/{}", tag)))?;
      File::create(&Path::new(&format!("meta/tag/{}/{}.json", tag, idx + 1)))?
        .write_all(serde_json::to_string(page)?.as_bytes())?;
    }
  }
  File::create(&Path::new("meta/meta.json"))?
    .write_all(serde_json::to_string(&meta)?.as_bytes())?;

  Ok(())
}
