use super::list::PostList;
use blog::types::{BuildMeta, Category, Tag};
use yew::prelude::*;

#[function_component(Posts)]
pub fn posts() -> Html {
  let meta = use_context::<BuildMeta>().expect("BuildMeta not found");

  html! {
    <PostList page={(meta.post + 9) / 10} path="/" />
  }
}

#[derive(Properties, PartialEq)]
pub struct CategoryPostsProps {
  pub category: Category,
}

#[function_component(CategoryPosts)]
pub fn category_posts(props: &CategoryPostsProps) -> Html {
  let meta = use_context::<BuildMeta>().expect("BuildMeta not found");

  html! {
    if let Some(cnt) = meta.category.get(&props.category) {
      <PostList page={(cnt + 9) / 10} path={format!("/category/{}/", props.category)} />
    }
  }
}

#[derive(Properties, PartialEq)]
pub struct TagPostsProps {
  pub tag: Tag,
}

#[function_component(TagPosts)]
pub fn tag_posts(props: &TagPostsProps) -> Html {
  let meta = use_context::<BuildMeta>().expect("BuildMeta not found");

  html! {
    if let Some(cnt) = meta.tag.get(&props.tag) {
      <PostList page={(cnt + 9) / 10} path={format!("/tag/{}/", props.tag)} />
    }
  }
}
