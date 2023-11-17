use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
  html! {
    <>
      <h1>{ "About" }</h1>
      <h2>{ "安一帆" }</h2>
      <h3>{ "简介" }</h3>
      <p>
        { "清华大学计算机科学与技术系本科在读，坚定的技术乐观主义者，热衷于有趣但不一定有用的东西。" }
      </p>
      <p>
        { "2019 年进入清华大学物理系，在兴趣驱使下选择在 2020 年转系进入计算机系，仍然在兴趣驱使下选择放弃推研，从事开发工作，对代码格式化和强类型检查有强烈的偏好。" }
        { "目前就职于北京智谱华章科技有限公司。" }
      </p>
      <p>
        { "感谢天才群友的灵感，现在你可以叫我 A1phaN。" }
      </p>
      <h3>{ "主要经历" }</h3>
      <ul>
        <li>{ "2020.6 - 今: 计算机系学生科协网络部，曾在 2022.6 - 2023.6 担任副主席，致力于推进内部信息系统建设，提高同学们的开发热情。" }</li>
        <li>{ "2021.8 - 2022.1: 在飞书商业化部门做前端开发，同一时期推进一项基于飞书开放平台的创业项目。" }</li>
        <li>{ "2022.8 - 2022.9: 参与 TriUOJ 的重构，推进计算机系课程评测系统的统一，并支持了此后连续两年的推免研究生上机考试。" }</li>
        <li>{ "2023.1 - 2023.3: 参与一个创业项目的后端开发。" }</li>
        <li>{ "2023.6 - 2023.7: 在龙芯中科实习，推进开源操作系统 Redox OS 的 LoongArch 平台支持。" }</li>
        <li>{ "2023.7 - 2023.8: 在阿里云游戏做全栈实习生，主要参与了云游戏 SDK 和内部工具的开发。" }</li>
        <li>{ "2023.8 - 今: 在智谱 AI 做前端实习生，主要负责内部平台的开发。" }</li>
        <li>{ "其他：在校内参与过来自院系、学校和其他团队的众多外包，也提供过许多无偿服务。" }</li>
      </ul>
      <h3>{ "技术栈和个人兴趣" }</h3>
      <p>{ "做过各类开发工作，最大的兴趣在于 VR/AR 相关技术，坚定相信 AR 即将取代手机成为下一代个人计算平台，对于 AI 一窍不通也无兴趣。我用过的技术栈主要有：" }</p>
      <h4>{ "后端" }</h4>
      <ul>
        <li>{ "express.js: 我最熟悉的后端框架，可以完成各类需求。" }</li>
        <li>{ "Actix: 写过完整项目，阅读过大型项目的源码，但由于生态差异（尤其指 ORM），开发速度和信心无法与 express 相比。" }</li>
      </ul>
      <h4>{ "前端" }</h4>
      <ul>
        <li>{ "React: 几乎所有实际项目都用，可以完成各类需求。" }</li>
        <li>{ "Vue: 只用过一点点，而且版本也比较旧，不喜欢。" }</li>
        <li>{ "Yew: 只有在当前博客中用过，喜欢但确实太难用了（包括但不限于 IDE 支持、打包工具、CSS 支持等问题）。" }</li>
      </ul>
    </>
  }
}
