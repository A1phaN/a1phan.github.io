use crate::{components::Paginator, router::Route};
use chrono::prelude::*;
use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostListProps {
  pub page: usize,
  pub path: String,
}

#[function_component(PostList)]
pub fn post_list(props: &PostListProps) -> Html {
  let page = use_state_eq(|| 1);
  // TODO: Add post list cache
  let post_list = use_state_eq(|| vec![]);
  {
    let p = page.clone();
    let path = props.path.to_string();
    let post_list = post_list.clone();
    use_effect_with_deps(
      move |_| {
        let post_list = post_list.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let list: Vec<blog::types::Post> = Request::get(&format!("/meta{}{}.json", path, *p))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
          post_list.set(list);
        });
      },
      (*page, props.path.clone()),
    );
  }
  let navigator = use_navigator().unwrap();
  let navigate_to = |route: Route| {
    let navigator = navigator.clone();
    Callback::from(move |e: MouseEvent| {
      navigator.push(&route);
      e.stop_propagation();
    })
  };
  let set_page = {
    let page = page.clone();
    Callback::from(move |p: usize| page.set(p))
  };

  html! {
    <div>
      <div class={classes!("post-list")}>
        <ul class={classes!("post-list-items")}>
          { for post_list.iter().map(|post| {
            html! {
              <li
                class={classes!("post-list-item")}
                onclick={navigate_to(Route::Post { path: post.path.clone() })}
              >
                <h4>{ post.title.clone() }</h4>
                <div>
                  {
                    format!(
                      "发表于: {} | 更新于: {} | 字数: {} | 分类: ",
                      Local.timestamp_opt(post.create as i64, 0).unwrap().format("%Y-%m-%d").to_string(),
                      Local.timestamp_opt(post.modify as i64, 0).unwrap().format("%Y-%m-%d").to_string(),
                      post.length,
                    )
                  }
                  <a onclick={navigate_to(Route::Category { category: post.meta.category })}>
                    { format!("{}", post.meta.category) }
                  </a>
                  if post.meta.tags.len() > 0 {
                    { " | 标签:" }
                    { for post.meta.tags.iter().map(|tag| {
                      html! {
                        <a onclick={navigate_to(Route::Tag { tag: tag.clone() })}>
                          { format!(" {} ", tag) }
                        </a>
                      }
                    })}
                  }
                </div>
                // TODO: Thumb
              </li>
            }
          })}
        </ul>
      </div>
      <div class={classes!("post-list-pagination")}>
        <Paginator page={*page} total={props.page} {set_page} />
      </div>
    </div>
  }
}
