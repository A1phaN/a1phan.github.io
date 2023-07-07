use chrono::prelude::*;
use crate::components::Markdown;
use gloo_net::http::Request;
use super::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Posts)]
pub fn posts() -> Html {
  let page = use_state_eq(|| 1);
  // TODO: Add post list cache
  let post_list = use_state_eq(|| vec![]);
  {
    let p = page.clone();
    let post_list = post_list.clone();
    use_effect_with_deps(move |_| {
      let post_list = post_list.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let list: Vec<blog::utils::Post> = Request::get(&format!("/{}.json", *p))
          .send()
          .await
          .unwrap()
          .json()
          .await
          .unwrap();
        post_list.set(list);
      });
    }, page);
  }
  let navigator = use_navigator().unwrap();

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
                      "Posted: {} | Updated: {}",
                      Local.timestamp_opt(post.create as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
                      Local.timestamp_opt(post.modify as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string(),
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
        // TODO: Pagination
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
    use_effect_with_deps(move |_| {
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
    }, props.path.clone());
  }

  html! {
    if !content.is_empty()  {
      <Markdown content={(*content).clone()} />
    }
  }
}
