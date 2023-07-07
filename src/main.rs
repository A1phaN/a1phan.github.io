mod components;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use router::{switch, Route};

#[function_component(Blog)]
fn blog() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={switch} />
    </BrowserRouter>
  }
}

fn main() {
  yew::Renderer::<Blog>::new().render();
}
