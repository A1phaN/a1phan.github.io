use blog::utils::BuildMeta;
use gloo_net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MetaProviderProps {
  pub children: Children,
}

#[function_component(MetaProvider)]
pub fn meta_provider(props: &MetaProviderProps) -> Html {
  let meta = use_state_eq(|| BuildMeta {
    timestamp: 0,
    post: 0,
  });
  {
    let meta = meta.clone();
    use_effect_with_deps(
      move |_| {
        let meta = meta.clone();
        wasm_bindgen_futures::spawn_local(async move {
          let fetched_meta: BuildMeta = Request::get("/meta.json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
          meta.set(fetched_meta);
        })
      },
      (),
    );
  }

  html! {
    <ContextProvider<BuildMeta> context={*meta}>
      { for props.children.iter() }
    </ContextProvider<BuildMeta>>
  }
}
