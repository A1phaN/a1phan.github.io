use crate::router::Route;
use blog::utils::parse_options;
use js_sys::eval;
use markdown::mdast::Node;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct MarkdownElementProps {
  // FIXME: This cause too many vector clones and should be fixed by updating markdown crate.
  pub children: Vec<Node>,
}

#[function_component(MarkdownElement)]
fn markdown_element(props: &MarkdownElementProps) -> Html {
  html! {
    { for props.children.iter().map(|child| match child {
      Node::Root(root) => html! { <MarkdownElement children={root.children.clone()} /> },
      Node::BlockQuote(block_quote) => html! { <blockquote><MarkdownElement children={block_quote.children.clone()} /></blockquote>},
      Node::FootnoteDefinition(_) => html! {},
      Node::MdxJsxFlowElement(_) => html! {},
      Node::List(list) => if list.ordered {
        html! {
          <ol>
            <MarkdownElement children={list.children.clone()} />
          </ol>
        }
      } else {
        html! {
          <ul>
            <MarkdownElement children={list.children.clone()} />
          </ul>
        }
      },
      Node::MdxjsEsm(_) => html! {},
      Node::Toml(_) => html! { /* meta data of blog */ },
      Node::Yaml(_) => html! {},
      Node::Break(_) => html! { <br /> },
      Node::InlineCode(inline_code) => html! { <code>{ inline_code.value.clone() }</code> },
      Node::InlineMath(_) => html! {},
      Node::Delete(delete) => html! { <del><MarkdownElement children={delete.children.clone()} /></del> },
      Node::Emphasis(emphasis) => html! { <em><MarkdownElement children={emphasis.children.clone()} /></em> },
      Node::MdxTextExpression(_) => html! {},
      Node::FootnoteReference(_) => html! {},
      Node::Html(htm) => html! { <div innerHTML={htm.value.clone()} /> },
      Node::Image(image) => html! { <img src={image.url.clone()} alt={image.alt.clone()} /> },
      Node::ImageReference(_) => html! {},
      Node::MdxJsxTextElement(_) => html! {},
      Node::Link(link) => match Route::recognize(&link.url).unwrap() {
        Route::NotFound => html! { <a href={link.url.clone()}><MarkdownElement children={link.children.clone()} /></a> },
        route => html! { <Link<Route> to={route}><MarkdownElement children={link.children.clone()} /></Link<Route>> },
      },
      Node::LinkReference(_) => html! {},
      Node::Strong(strong) => html! { <strong><MarkdownElement children={strong.children.clone()} /></strong> },
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
        let children = html! { <MarkdownElement children={heading.children.clone()} /> };
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
                <th>
                  if let Node::TableCell(th) = th {
                    <MarkdownElement children={th.children.clone()} />
                  }
                </th>
              }) }
            </tr>
            <MarkdownElement children={Vec::from(table.children.get(1..table.children.len()).unwrap())} />
          </table>
        }
      },
      Node::ThematicBreak(_) => html! { <hr /> },
      Node::TableRow(table_row) => html! { <tr><MarkdownElement children={table_row.children.clone()} /></tr> },
      Node::TableCell(table_cell) => html! { <td><MarkdownElement children={table_cell.children.clone()} /></td> },
      Node::ListItem(list_item) => html! { <li><MarkdownElement children={list_item.children.clone()} /></li> },
      Node::Definition(_) => html! {},
      Node::Paragraph(paragraph) => html! { <p><MarkdownElement children={paragraph.children.clone()} /></p> },
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
    use_effect_with_deps(
      move |_| {
        let ast = markdown::to_mdast(&content, &parse_options()).unwrap();
        mdast.set(ast);
      },
      props.content.clone(),
    );
  }
  use_effect(|| {
    let _ = eval("window.Prism.highlightAll();");
  });

  html! {
    if let Node::Root(root) = (*mdast).clone() {
      <MarkdownElement children={root.children} />
    }
  }
}
