use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginatorProps {
  pub page: usize,
  pub total: usize,
  pub set_page: Callback<usize>,
}

#[function_component(Paginator)]
pub fn paginator(props: &PaginatorProps) -> Html {
  let min = if props.page > 2 { props.page - 2 } else { 1 };
  let max = if props.page + 2 < props.total { props.page + 2 } else { props.total };
  let set_page = |page: usize| {
    let set_page = props.set_page.clone();
    move |_| set_page.emit(page)
  };

  html! {
    <ul
      style="
        display: flex;
        list-style: none;
      "
    >
      <li>
        <button
          disabled={props.page <= 1}
          onclick={set_page(props.page - 1)}
        >
          { "<" }
        </button>
      </li>
      { for (min..=max).map(|page| html! {
        <li>
          <button
            class={classes!(if props.page == page { Some("active-page") } else { None })}
            disabled={props.page == page}
            onclick={set_page(page)}
          >
            { page }
          </button>
        </li>
      }) }
      <li>
        <button
          disabled={props.page >= props.total}
          onclick={set_page(props.page + 1)}
        >
          { ">" }
        </button>
      </li>
    </ul>
  }
}