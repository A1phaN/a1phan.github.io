use super::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
  html! {
    <>
      <h1>{ "Welcome!" }</h1>
      <p>
        { "欢迎来到 ayf 的 blog V3，虽然过去花在搭建博客上的内容比写内容的还多，但从现在开始会尝试记录一些做新的项目的笔记（flag）。" }
        { "不妨从" }
        <Link<Route> to={Route::Post { path: "hello".to_string() } }>{ "这一版 blog 的搭建" }</Link<Route>>
        { "开始吧。" }
      </p>
    </>
  }
}
