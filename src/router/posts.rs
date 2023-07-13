use super::Route;
use crate::components::{Markdown, Paginator};
use blog::types::BuildMeta;
use chrono::prelude::*;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Posts)]
pub fn posts() -> Html {
  let meta = use_context::<BuildMeta>().expect("BuildMeta not found");
  let page = use_state_eq(|| 1);
  // TODO: Add post list cache
  let post_list = use_state_eq(|| vec![]);
  {
    let p = page.clone();
    let post_list = post_list.clone();
    use_effect_with_deps(
      move |_| {
        let post_list = post_list.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let list: Vec<blog::types::Post> = Request::get(&format!("/{}.json", *p))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
          post_list.set(list);
        });
      },
      *page,
    );
  }
  let navigator = use_navigator().unwrap();
  let set_page = {
    let page = page.clone();
    Callback::from(move |p: usize| page.set(p))
  };

  html! {
    <div>
      <div class={classes!("post-list")}>
        <ul class={classes!("post-list-items")}>
          { for post_list.iter().map(|post| {
            let navigator = navigator.clone();
            let route = Route::Post { path: post.path.clone() };
            html! {
              <li
                class={classes!("post-list-item")}
                onclick={Callback::from(move |_| { navigator.push(&route); })}
              >
                <h4>{ post.title.clone() }</h4>
                <div>
                  {
                    format!(
                      "发表于: {} | 更新于: {} | 字数: {} | 分类: {}{}",
                      Local.timestamp_opt(post.create as i64, 0).unwrap().format("%Y-%m-%d").to_string(),
                      Local.timestamp_opt(post.modify as i64, 0).unwrap().format("%Y-%m-%d").to_string(),
                      post.length,
                      post.meta.category,
                      if post.meta.tags.len() > 0 {
                        format!(
                          " | 标签: {}",
                          post
                            .meta
                            .tags
                            .iter()
                            .map(|tag| format!("{tag}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                        )
                      } else {
                        "".to_string()
                      }
                    )
                  }
                </div>
                // TODO: Thumb
              </li>
            }
          })}
        </ul>
      </div>
      <div class={classes!("post-list-pagination")}>
        <Paginator page={*page} total={(meta.post + 9) / 10} {set_page} />
      </div>
    </div>
  }
}

#[derive(Properties, PartialEq)]
pub struct PostProps {
  pub path: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
  let content = use_state_eq(String::new);
  {
    let path = props.path.clone();
    let content = content.clone();
    use_effect_with_deps(
      move |_| {
        let content = content.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let res: String = Request::get(&format!("/post/{}.md", &path))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
          content.set(res);
        });
      },
      props.path.clone(),
    );
  }

  html! {
    if !content.is_empty()  {
      <Markdown content={(*content).clone()} />
    }
  }
}
