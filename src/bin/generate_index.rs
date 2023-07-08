use blog::utils::{find_first_element, get_text, BuildMeta, Post};
use chrono::prelude::*;
use markdown::mdast::Node;
use std::io::Write;
use std::{
  fs::File,
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
      let length = File::open(path)
        .expect(&format!("failed to open file {}", path.to_str().unwrap()))
        .read_to_string(&mut content)
        .expect(&format!("failed to read file {}", path.to_str().unwrap()));
      let mdast = markdown::to_mdast(&content, &markdown::ParseOptions::default())
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
      }
    })
    .collect::<Vec<_>>();
  posts.sort_by_key(|post| post.create);
  let meta = BuildMeta {
    timestamp: Local::now().timestamp(),
    post: posts.len(),
  };

  File::create(&Path::new("dist/meta.json"))?
    .write_all(serde_json::to_string(&meta)?.as_bytes())?;
  for (idx, page) in posts.chunks(10).enumerate() {
    File::create(&Path::new(&format!("dist/{}.json", idx + 1)))?
      .write_all(serde_json::to_string(page)?.as_bytes())?;
  }

  Ok(())
}
