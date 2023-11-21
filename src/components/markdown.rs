use crate::router::Route;
use blog::utils::parse_options;
use js_sys::eval;
use markdown::mdast::Node;
use yew::prelude::*;
use yew_router::prelude::*;

fn markdown_element(props: &Vec<Node>) -> Html {
  html! {
    { for props.iter().map(|child| match child {
      Node::Root(root) => markdown_element(&root.children),
      Node::BlockQuote(block_quote) => html! {
        <blockquote>{markdown_element(&block_quote.children)}</blockquote>
      },
      Node::FootnoteDefinition(_) => html! {},
      Node::MdxJsxFlowElement(_) => html! {},
      Node::List(list) => if list.ordered {
        html! { <ol>{markdown_element(&list.children)}</ol> }
      } else {
        html! { <ul>{markdown_element(&list.children)}</ul> }
      },
      Node::MdxjsEsm(_) => html! {},
      Node::Toml(_) => html! { /* meta data of blog */ },
      Node::Yaml(_) => html! {},
      Node::Break(_) => html! { <br /> },
      Node::InlineCode(inline_code) => html! { <code>{ inline_code.value.clone() }</code> },
      Node::InlineMath(_) => html! {},
      Node::Delete(delete) => html! { <del>{markdown_element(&delete.children)}</del> },
      Node::Emphasis(emphasis) => html! { <em>{markdown_element(&emphasis.children)}</em> },
      Node::MdxTextExpression(_) => html! {},
      Node::FootnoteReference(_) => html! {},
      Node::Html(htm) => html! { <div innerHTML={htm.value.clone()} /> },
      Node::Image(image) => html! { <img src={image.url.clone()} alt={image.alt.clone()} style="max-width: 100%" /> },
      Node::ImageReference(_) => html! {},
      Node::MdxJsxTextElement(_) => html! {},
      Node::Link(link) => match Route::recognize(&link.url).unwrap() {
        Route::NotFound => html! { <a href={link.url.clone()}>{markdown_element(&link.children)}</a> },
        route => html! { <Link<Route> to={route}>{markdown_element(&link.children)}</Link<Route>> },
      },
      Node::LinkReference(_) => html! {},
      Node::Strong(strong) => html! { <strong>{markdown_element(&strong.children)}</strong> },
      Node::Text(text) => html! { text.value.clone() },
      Node::Code(code) => html! {
        <pre>
          <code class={classes!(code.lang.as_ref().map(|lang| format!("language-{}", lang)))}>
            { code.value.clone() }
          </code>
        </pre>
      },
      Node::Math(_) => html! {},
      Node::MdxFlowExpression(_) => html! {},
      Node::Heading(heading) => {
        let children = markdown_element(&heading.children);
        match heading.depth {
          1 => html! { <h1>{children}</h1> },
          2 => html! { <h2>{children}</h2> },
          3 => html! { <h3>{children}</h3> },
          4 => html! { <h4>{children}</h4> },
          5 => html! { <h5>{children}</h5> },
          6 => html! { <h6>{children}</h6> },
          _ => html! {},
        }
      },
      Node::Table(table) => html! {
        if let Node::TableRow(tr) = &table.children[0] {
          <table>
            <tr>
              { for tr.children.iter().map(|th| html! {
                if let Node::TableCell(th) = th {
                  <th>
                    {markdown_element(&th.children)}
                  </th>
                }
              }) }
            </tr>
            {markdown_element(&Vec::from(table.children.get(1..table.children.len()).unwrap()))}
          </table>
        }
      },
      Node::ThematicBreak(_) => html! { <hr /> },
      Node::TableRow(table_row) => html! { <tr>{markdown_element(&table_row.children)}</tr> },
      Node::TableCell(table_cell) => html! { <td>{markdown_element(&table_cell.children)}</td> },
      Node::ListItem(list_item) => html! { <li>{markdown_element(&list_item.children)}</li> },
      Node::Definition(_) => html! {},
      Node::Paragraph(paragraph) => html! { <p>{markdown_element(&paragraph.children)}</p> },
    }) }
  }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
  pub content: String,
}

#[function_component(Markdown)]
pub fn markdown_component(props: &MarkdownProps) -> Html {
  // TODO: avoid parsing markdown every time
  let mdast = use_state_eq(|| {
    Node::Root(markdown::mdast::Root {
      position: None,
      children: vec![],
    })
  });
  {
    let mdast = mdast.clone();
    let content = props.content.clone();
    use_effect_with(
      content.clone(),
      move |_| {
        let ast = markdown::to_mdast(&content, &parse_options()).unwrap();
        mdast.set(ast);
      },
    );
  }
  use_effect(|| {
    let _ = eval("window.Prism.highlightAll();");
  });

  if let Node::Root(root) = (*mdast).clone() {
    markdown_element(&root.children)
  } else {
    html! {}
  }
}
