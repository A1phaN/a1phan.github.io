mod about;
mod home;
mod not_found;
mod posts;

use crate::components::{Layout, MetaProvider};
use yew::prelude::*;
use yew_router::prelude::*;

use about::About;
use home::Home;
use not_found::NotFound;
use posts::{Post, Posts};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/about")]
  About,
  #[at("/posts")]
  Posts,
  #[at("/post/*path")]
  Post { path: String },
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: Route) -> Html {
  html! {
    <MetaProvider>
      <Layout active={route.clone()}>
        {
          match route {
            Route::Home => html! { <Home /> },
            Route::About => html! { <About /> },
            Route::Posts => html! { <Posts /> },
            Route::Post { path } => html! { <Post {path} /> },
            Route::NotFound => html! { <NotFound /> },
          }
        }
      </Layout>
    </MetaProvider>
  }
}
