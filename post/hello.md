+++
category = "Developing"
tags = ["Blog", "Rust", "Yew"]
+++
# Hello World v3
上一次写博客还是上一次（实际上上一次写博客的时候第一句话也是这个，但是压根没发出来，xs），旧的博客的内容实际上已经过时，考虑到你协[技能引导文档](https://docs.net9.org/)的存在，那些旧的博客大概也不必再找回来。实际上直到现在，我还是没有完全搞清楚 hexo 如何正确处理博客的时间，看 CI 配置应该已经正确修改文件的修改时间为 Git 提交时间了，但实际上所有文章仍显示为最后一次提交时间，本着用轮子不如造轮子的精神，早就有重新写一套博客的想法。

前段时间已经尝试用 React 写过一版，但最终没有部署，还是想用 Rust 重写图一乐，于是就有了现在这个版本。
## Yew.rs
作为一个 React 忠实用户，换用 Rust 写前端毫无疑问地使用了 [Yew](https://yew.rs/) 作为开发框架，这两个框架相似度很高，但实际使用中还是会遇到一些不习惯之处，参见 [Yew 踩坑笔记](/post/yew)。
## 静态博客渲染
这一版博客使用的思路和用 React 的想法一致，博客主体实现为一个单页面应用，将若干博文放在单独的文件夹中，部署时生成包含博客信息的 json 文件，前端动态获取 json 文件和博客内容，并用 markdown 语法进行渲染。

因此使用 GitHub Actions 构建和部署静态网站的配置如下：
```yaml
name: Blog Deploy

on:
  push:
    branches: ["master"]

jobs:
  deploy:
    permissions:
      pages: write
      id-token: write
    environment:
      name: github-pages
      url: ${{ steps.deployment.outputs.page_url }}
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v3
      - name: Install Rust Toolchain
        uses: dtolnay/rust-toolchain@stable
      - name: Install Trunk
        run: |
          rustup target add wasm32-unknown-unknown
          cargo install --locked trunk
      - name: Build
        run: |
          trunk build --release
          cp dist/index.html dist/404.html
      - name: Setup Pages
        uses: actions/configure-pages@v3
      - name: Upload artifact
        uses: actions/upload-pages-artifact@v1
        with:
          path: './dist'
      - name: Deploy to GitHub Pages
        id: deployment
        uses: actions/deploy-pages@v2
```

比 Node.js 好的一点是，我可以写一个单独的可执行程序（`src/bin/generate_index.rs`）来完成必要的预处理工作，这比上一个版本中执行 `node generate_index.js` 看起来要优雅一些，而且便于与博客的其他部分共用类型定义等内容。

不同的是 Yew 目前的生态与 React 相比还有欠缺，我没有找到可以与 [antd](https://ant.design/) 相比的适用于 Yew 的组件库，而且 [Yew 本身在 CSS 支持方面仍有许多不足](https://yew.rs/docs/more/css)，~~考虑到我目前还没有自己写一套组件库的动力，整个博客的 CSS（暂时）应当还没有很多，因此我决定直接把这些内容写在 `index.html` 中，虽然这并不优雅，但大约也可以使用相当长一段时间~~。应该说我还是错怪了 trunk，虽然它提供的打包能力相比与 webpack 肯定还有很大的差距，但至少提供了编译 SASS / SCSS 的能力，因此我实际上不必将 CSS 都写在 `index.html` 中，而是只需要添加一句：
```html
<link data-trunk href="./style.scss" rel="scss" />
```
另外通过配置 trunk 的拷贝资源文件夹和构建 hook，我将构建过程精简到只需要一句 `trunk build --release`（为了使 GitHub 能够正确跳转，还复制了 `404.html`），这样在开发过程中就不必每次修改代码之后都手动运行 generate_index 脚本并复制内容了，这极大改善了我的开发舒适度（但目前观察到修改一个文件之后似乎 trunk 会重复编译多次，原因未知，也有可能和 CLion 的文件保存机制有关，不得不说 CLion 还是会做一些多余的写入操作）。
## markdown 渲染
作为一个简单的博客框架，毫无疑问我选择了 markdown 作为实际撰写内容使用的标记语言，但在 Yew 当中并没有类似于我之前使用的 [markdown 渲染工具](https://remarkjs.github.io/react-markdown/)，我能找到的最贴近我用途的库是 [markdown-rs](https://github.com/wooorm/markdown-rs)，这个库提供了 HTML 渲染功能，但渲染的结果是字符串，因此如果我不想写 `innerHTML={markdown::to_html(content)}` 这样的东西，就只能自己实现渲染组件。

好在 markdown-rs 提供了 `to_mdast` 方法以获得 markdown 代码的抽象语法树，这大大降低了实现渲染组件的难度，只需要根据抽象语法树对应渲染为 HTML 节点即可，因此这部分的核心实现如下：
```rust
#[function_component(MarkdownElement)]
fn markdown_element(props: &MarkdownElementProps) -> Html {
  html! {
    { for props.children.iter().map(|child| match child {
      Node::Root(root) => html! { <MarkdownElement children={root.children.clone()} /> },
      Node::BlockQuote(block_quote) => html! { <blockquote><MarkdownElement children={block_quote.children.clone()} /></blockquote>},
      /* other kinds... */
    }) }
  }
}
```

这带来的额外好处是可以完全自定义各种节点的实现方式，而不必像 react-markdown 那样注册替代品。
## 代码块高亮
代码块高亮算是一个相对比较重要的 feature，因为我写的博客应该每一篇都会用到，但是在 Rust 当中没有找到适当的工具实现，最终找到一个比较简单的库是 [Prism](https://prismjs.com/)，这个库对框架的耦合很小，只要引入对应的 JS 和 CSS 文件，给代码块添加对应语言的 class 即可，例如：
```html
<pre>
  <code class="language-rust">
    /* some rust code here */
  </code>
</pre>
```
~~反正 `index.html` 里已经有很多东西，再加几行也可以容忍吧~~，作为参考，使用方法如下：
```html
<html>
  <head>
    <link href="https://cdn.jsdelivr.net/npm/prismjs@v1.x/themes/prism.css" rel="stylesheet" />
  </head>
  <body>
    <script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/components/prism-core.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/prismjs@v1.x/plugins/autoloader/prism-autoloader.min.js"></script>
  </body>
</html>
```
引入这些文件后就可以自动将上文所示的代码块高亮，但这一行为是通过 `DOMContentLoaded` 事件触发的，也就是说在做 CSR 时当 DOM 修改时这一事件并不会触发，因此需要在每次渲染代码块后重新触发 Prism 的高亮，简单来说可以实现为：
```jsx
function Post(props) {
  useEffect(() => {
    window.Prism.highlightAll();
  });
  
  return <Markdown content={props.content} />;
}
```
为了实现这一目标（尽管并不太优雅，但是已经是尽量减少对 Rust 代码的侵入了），我使用了如下方法：
```rust
use js_sys::eval;

#[function_component(Markdown)]
pub fn markdown_component(props: &MarkdownProps) -> Html {
  let mdast = markdown::to_mdast(&props.content, &parse_options()).unwrap();
  use_effect(|| {
    let _ = eval("window.Prism.highlightAll();");
  });

  html! {
    if let Node::Root(root) = mdast {
      <MarkdownElement children={root.children} />
    }
  }
}
```
使用 `eval` 在任何情况下都是邪恶的，但是如果 `eval` 的内容是固定的，则可以被原谅（大嘘）。社区中我看到了 Prism 的 wasm 绑定，但是这些库几乎没有人用，也基本没有在活跃，使用这些库大约也不比直接 `eval` 更优雅罢。
### 我在 Rust 项目中学习 React
我原本一直以为 `useEffect` 是在加载 DOM 之前执行的，虽然现在仔细想想好像这样实现起来不是很合理，但直到我这里遇到需要确保副作用在 DOM 加载之后才运行的时候，我才去认真看了文档，发现 React 文档如此描述：
> The function with your Effect’s logic. Your setup function may also optionally return a cleanup function. **When your component is added to the DOM, React will run your setup function.**
>## 简单的分页
虽然现在的博客数量还远远没有必要实现分页，但是为了美观起见还是要加一个。在第一个版本中就添加了分页的预处理，将所有的博文信息每 10 篇作为一页存储在一个 JSON 文件中，博文列表页根据页码获取对应的信息做展示。因此只需要增加一个接受 `page` 和 `set_page` 的分页器组件，修改父组件的页码即可，从 antd 里抄了一点 CSS，不算太难看就行了。

这部分实现好之后就可以将博文列表组件进行微调，使得这个组件可以用于所有博文、根据分类或标签筛选博文这两种场景。
## 分类和标签
常见的博客功能，模仿 Hexo 的设计，通过在博文的 markdown 文件中加入一些元数据使预处理过程能够将博文进行分类和标记，同时我导出了每个分类和标签的博文的清单，这样就可以实现分类查看，但是前端还没有实现。

这里首先学到的是 markdown 扩展语法，在实现 markdown 渲染器时就已经注意到有很多我没有使用过也不太清楚是什么的元素，例如 Toml 和 Yaml 等（此前我以为 Hexo 实现的 Yaml 元数据是自定义语法），但查找相关资料时我才知道有更多的 markdown 扩展语法。因此无需做多余的定义即可使用 Toml 作为元数据的声明语法。对于 markdown-rs 来说，使用如下的解析选项可以得到我需要的信息：
```rust
pub fn parse_options() -> ParseOptions {
  ParseOptions {
    constructs: Constructs {
      frontmatter: true,
      gfm_table: true,
      ..Default::default()
    },
    ..Default::default()
  }
}
```

其中 `frontmatter` 字段就控制了文件开头的 Toml 和 Yaml 信息的解析。开启这一选项后，只需要在预处理数据时读取这些数据，而在渲染时忽略（`Node::Toml(_) => html! {}`），就可以丝滑实现这一目标。
## Flags
另外还有更多的 feature 等待实现，关于这一部分 flag 可以参考 README，每当完成新的 flag 时我会在这里更新我的实现思路。