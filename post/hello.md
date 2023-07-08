# Hello World v3
上一次写博客还是上一次（实际上上一次写博客的时候第一句话也是这个，但是压根没发出来，xs），旧的博客的内容实际上已经过时，考虑到你协[技能引导文档](https://docs.net9.org/)的存在，那些旧的博客大概也不必再找回来。 实际上直到现在，我还是没有完全搞清楚 hexo 如何正确处理博客的时间，看 CI 配置应该已经正确修改文件的修改时间为 Git 提交时间了，但实际上所有文章仍显示为最后一次提交时间，本着用轮子不如造轮子的精神，早就有重新写一套博客的想法。

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
          cargo run --bin generate_index
          cp -r post dist/
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

不同的是 Yew 目前的生态与 React 相比还有欠缺，我没有找到可以与 [antd](https://ant.design/) 相比的适用于 Yew 的组件库，而且 [Yew 本身在 CSS 支持方面仍有许多不足](https://yew.rs/docs/more/css)，考虑到我目前还没有自己写一套组件库的动力，整个博客的 CSS （暂时）应当还没有很多，因此我决定直接把这些内容写在 `index.html` 中，虽然这并不优雅，但大约也可以使用相当长一段时间。

另外还有更多的 Feature 等待实现，关于这一部分 flag 可以参考 README，每当完成新的 flag 时我会在这里更新我的实现思路。