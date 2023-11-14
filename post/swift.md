+++
category = "Learning"
tags = ["Swift"]
+++
# Swift 学习笔记
熟悉我的朋友都知道我是 XR 技术的忠实拥趸，为了能及时学习 Vision Pro 的开发（好像也不及时，毕竟微信已经支持 Vision Pro 平台了），准备进行一个 Swift 和 SwiftUI 的速成，在这里记录我认为有趣的 feature 或者我遇到的难点。

## 数据类型和运算
### 字符串
Swift 原生支持 Unicode 编码，并且可以通过 `utf8` 和 `utf16` 属性访问字符串的对应编码值，代价则是与 Rust 类似，想要按照“字符”索引访问的时间复杂度是 `O(n)`。

Swift 支持多行字符串和类似于 Rust 的扩展字符串分隔符，这是我非常喜欢的地方，尤其是多行字符串不包含引号所在行、允许续行符、支持缩进，使我这个源码强迫症极为舒适：
```swift
let multilineString = """
    Write here and no extra white space or line break \
    will be stored.
    """
```

这一点让我这个 TypeScript 程序员馋哭了，我本来想通过修改 String 的原型链为 TypeScript 提供这一 feature，但是在我翻阅[模板字符串](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#tagged_templates)的文档时发现了 **Tagged Templates** 的神秘语法，因此我就可以：
```typescript
function swiftLikeString(template: TemplateStringsArray, ...args: any[]) {
  const string = template.reduce((str, part, i) => str + part + (args[i] ?? '').toString(), '');
  const tailingSpace = string.length - string.lastIndexOf('\n') - 1;
  return string.split('\n').map(line => line.slice(tailingSpace)).slice(1, -1).join('\n');
}

const str = swiftLikeString`
  Now you can write swift-like string in typescript.
  `
```

~~实不相瞒就是因为搞了这个 hack 才想起来写篇 blog 记录下。~~
