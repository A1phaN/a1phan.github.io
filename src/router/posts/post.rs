use crate::components::Markdown;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
  pub path: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
  let content = use_state_eq(String::new);
  {
    let path = if props.path.ends_with(".md") {
      format!("/post/{}", props.path)
    } else {
      format!("/post/{}.md", props.path)
    };
    let content = content.clone();
    use_effect_with(
      path.clone(),
      move |_| {
        let content = content.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let res: String = Request::get(&path)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
          content.set(res);
        });
      },
    );
  }

  html! {
    if !content.is_empty()  {
      <Markdown content={(*content).clone()} />
    }
  }
}
