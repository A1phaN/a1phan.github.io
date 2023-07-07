use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct NavbarItemProps {
  pub active: bool,
  pub label: &'static str,
  pub to: Route,
}

#[function_component(NavbarItem)]
fn navbar_item(props: &NavbarItemProps) -> Html {
  html! {
    <li
      class={classes!(
        "navbar-item",
        if props.active {
          Some("active-item")
        } else {
          None
        }
      )}
    >
      <Link<Route> to={props.to.clone()}>
        { props.label }
      </Link<Route>>
    </li>
  }
}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
  pub children: Children,
  pub active: Route,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
  html! {
    <section class={classes!("layout")}>
      <header class={classes!("header")}>
        <ul class={classes!("navbar")}>
          <NavbarItem
            active={matches!(props.active, Route::Home)}
            label="Home"
            to={Route::Home.clone()}
          />
          <NavbarItem
            active={matches!(props.active, Route::About)}
            label="About"
            to={Route::About.clone()}
          />
          <NavbarItem
            active={matches!(props.active, Route::Posts | Route::Post { .. })}
            label="Posts"
            to={Route::Posts.clone()}
          />
        </ul>
      </header>
      <main
        class={classes!(
          "content",
          match props.active {
            Route::Posts => None,
            _ => Some("ordered-headings")
          }
        )}
      >
        { for props.children.iter() }
      </main>
    </section>
  }
}
