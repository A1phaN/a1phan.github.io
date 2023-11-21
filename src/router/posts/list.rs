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
    let page = *page;
    let path = props.path.clone();
    let post_list = post_list.clone();
    use_effect_with(
      (page, path.clone()),
      move |_| {
        let post_list = post_list.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let list: Vec<blog::types::Post> = Request::get(&format!("/meta{}{}.json", path, page))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
          post_list.set(list);
        });
      },
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
      <div class="post-list">
        <ul class="post-list-items">
          { for post_list.iter().map(|post| {
            html! {
              <li
                class="post-list-item"
                onclick={navigate_to(Route::Post { path: post.path.clone() })}
              >
                <h4>{ post.title.clone() }</h4>
                <div style="display: flex">
                  <span style="width: 160px">
                    { format!("发表于: {}", Local.timestamp_opt(post.create as i64, 0).unwrap().format("%Y-%m-%d").to_string()) }
                  </span>
                  <span style="width: 160px">
                    { format!("更新于: {}", Local.timestamp_opt(post.modify as i64, 0).unwrap().format("%Y-%m-%d").to_string()) }
                  </span>
                  <span style="width: 90px">
                    { format!("字数: {}", post.length) }
                  </span>
                  <span style="width: 135px">
                    { "分类: " }
                    <a onclick={navigate_to(Route::Category { category: post.meta.category })}>
                      { format!("{}", post.meta.category) }
                    </a>
                  </span>
                  if post.meta.tags.len() > 0 {
                    <span>
                      { "标签: " }
                      { for post.meta.tags.iter().map(|tag| {
                        html! {
                          <a onclick={navigate_to(Route::Tag { tag: tag.clone() })}>
                            { format!(" {} ", tag) }
                          </a>
                        }
                      })}
                    </span>
                  }
                </div>
                // TODO: Thumb
              </li>
            }
          })}
        </ul>
      </div>
      <div class="post-list-pagination">
        <Paginator page={*page} total={props.page} {set_page} />
      </div>
    </div>
  }
}
