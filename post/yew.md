+++
category = "Learning"
tags = ["Rust", "Yew"]
+++
# Yew 踩坑笔记
作为一个 React 忠实用户和 Rust 爱好者，使用 [Yew](https://yew.rs/) 看起来是一件理所当然的事情，在我尝试实现这个前端之前我也认为这个学习过程应当非常简单，但实际上还是会遇到许多奇妙的问题，在这里记录我已经和即将遇到的问题。
## 开发环境
### trunk
Yew 官方推荐的方式是使用 [trunk](https://trunkrs.dev/) 作为打包工具，但通过 cargo 安装的速度相当慢，这导致每次更新博客都需要大约五分钟时间跑 CI。尽管有预编译的二进制文件，但是出于一种“买新不买旧”的奇妙心理我还是更喜欢通过 cargo 安装。

在本地运行时，无论使用 `trunk serve` 还是 `trunk build`，都有很大的概率无法正常在浏览器中打开，我怀疑这是我某些地方实现有错误导致白屏，但是我并没有成功排查到这一问题，好在部署的版本是稳定的。如果有幸遇到知道原因的读者还请不吝赐教。
### IDE
我习惯使用的 Rust IDE 是 CLion，通常来说 CLion 可以分析一些 Rust 宏的语法，但 Yew 的 `html!` 宏显然还是过于复杂了一些，IDE 只会在有语法错误的时候关掉高亮显示大段语法错误，但并不会在需要的时候给出任何提示，这导致开发体验相比 React 有很大的差距（我曾经说过 Rust 的好处在于无需像 React 一样自定义 JSX 格式就可以支持这种风格的内嵌 HTML 代码，但如今看来这是否是一种好处还不一定）。

另外，定义函数组件时使用的 `#[function_component]` 宏几乎没有得到任何提示，在尝试引用函数组件时没有任何提示。
## Yew 函数组件
Yew 的函数组件写好之后的代码应该说还是很好看的，得益于 Rust 语法优秀的表现力，一些用到 match 等语法的组件可以有很优雅的表示。但 Rust 对生命周期的管理也造成了一些问题，对于 React 程序员来说，我们都会默认所有的状态在 React 全局状态管理中被保存，因此几乎可以任意（只读）引用这些数据，而这种假设在 Rust 中并不能被实现，往往需要大量的复制来解决问题，尽管 Yew 做了一些封装来减少复制的数据，但这一行为的发生仍然十分频繁，以最简单的获取博客内容的代码为例：
```rust
let content = use_state_eq(String::new);
{
  let path = props.path.clone();
  let content = content.clone();
  use_effect_with_deps(
    move |_| {
      let content = content.clone();
      wasm_bindgen_futures::spawn_local(async move {
        let res: String = Request::get(&format!("/post/{}.md", &path))
          .send()
          .await
          .unwrap()
          .text()
          .await
          .unwrap();
        content.set(res);
      });
    },
    props.path.clone(),
  );
}
```
这里需要多次复制 `content`（应该只是复制了 `UseStateHandle<String>` 的壳，但如果 `content` 来自于 Props 则无法避免）来确保 `content` 的所有权不会移动，而在 React 当中则不必如此。

另一个比较棘手的问题是函数组件的参数，由于参数必须实现 `Properties` trait，而它又依赖于 `ImplicitClone` trait，这导致如果外部类型没有实现 `ImplicitClone`，将会导致无法将这种类型用在组件参数中，这应该是由于语言特性导致的，JavaScript 当中所有的对象都相当于计数引用，在传递和比较更新时都只需要对比引用，而在 Rust 当中由于生命周期的存在，一个数据的引用不能任意传递，导致要实现相同的功能需要构造更复杂的数据结构，例如使用 `std::rc::Rc` 等方式实现引用计数。

此外吐槽的一个小点是 Yew 官方文档对于 hook 的介绍十分语焉不详，比如对于 `state` 的使用，官方文档中使用的例子是 `usize` 类型的数据，但 `usize` 实现了 `Copy` trait，这导致很多用法对于其他类型是不通用的。而这个即使数据没有改变仍然会导致重新渲染的 `use_state` hook 我并没有理解，虽然有可能仍然与语言特性有关，但是如果官方文档能给出一个例子就更好了。