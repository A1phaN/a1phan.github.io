+++
category = "Learning"
tags = ["Linux", "Network"]
+++
# nftables 学习笔记
多年前系里一位老师对夏老板如是说：
> 现在计算机系的同学已经不会配 iptables 了！

虽然确实配 iptables 的实际需求非常小，但是为了配好 [我的新路由器](/post/router.md)，还是决定学习一波。但是刚打开 ArchWiki 就看到：
> Note: iptables is a legacy framework, nftables aims to provide a modern replacement including a compatibility layer.

经过简单的搜索后我确认了 nftables 确实将替代 iptables，因此我~~直接将这篇博客改为 nftables 学习笔记~~选择直接学习 nftables。

## 安装
nftables 可以使用 firewalld 和 ufw 做前端，同时也有 iptables-nft 提供与 iptables 的兼容层，为了避免 iptables（已经默认存在）的干扰，直接安装：
```bash
sudo pacman -S iptables-nft
```
将自动安装 nftables 和 iptables 兼容层，并卸载原有的 iptables。

启动 nftables：
```bash
sudo systemctl enable --now nftables.service
```

## 参考资料
- [iptables - ArchWiki](https://wiki.archlinux.org/title/iptables)
- [金枪鱼之夜：坏人的 iptables 小讲堂](https://www.youtube.com/watch?v=w_vGD-96O54)