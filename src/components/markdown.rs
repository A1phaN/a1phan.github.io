use crate::router::Route;
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
      Node::Toml(_) => html! {},
      Node::Yaml(_) => html! {},
      Node::Break(_) => html! {},
      Node::InlineCode(inline_code) => html! { <code>{ inline_code.value.clone() }</code> },
      Node::InlineMath(_) => html! {},
      Node::Delete(delete) => html! { <del><MarkdownElement children={delete.children.clone()} /></del> },
      Node::Emphasis(emphasis) => html! { <em><MarkdownElement children={emphasis.children.clone()} /></em> },
      Node::MdxTextExpression(_) => html! {},
      Node::FootnoteReference(_) => html! {},
      Node::Html(_) => html! {},
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
      Node::Table(_) => html! {},
      Node::ThematicBreak(_) => html! {},
      Node::TableRow(_) => html! {},
      Node::TableCell(_) => html! {},
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
  let mdast = markdown::to_mdast(&props.content, &markdown::ParseOptions::default()).expect("");
  use_effect(|| {
    let _ = eval("window.Prism.highlightAll();");
  });

  html! {
    if let Node::Root(root) = mdast {
      <MarkdownElement children={root.children} />
    }
  }
}
