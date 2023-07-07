use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
  html! {
    <>
      <h1>{ "About" }</h1>
      <h1>{ "安一帆" }</h1>
      <h2>{ "简介" }</h2>
      <p>
        { "清华大学计算机科学与技术系本科在读，坚定的技术乐观主义者，热衷于有趣但不一定有用的东西。" }
      </p>
    </>
  }
}
