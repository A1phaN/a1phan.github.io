+++
category = "Hacking"
tags = ["Router"]
+++
# 从零开始配置路由器
这是一个鸽了非常非常久的项目，长期以来我一直想要一套完全自主可控的网络环境，我希望能实现以下目标：
- 让所有设备在同一个 WireGuard 网段中，使所有的设备无论在什么环境下都可以自由互相访问
- 自动分流（私有内网、校园网、国内、国外）
- 基于 MITM 的广告拦截和用户脚本

这些想法如果只是在个别设备上实现其实相对而言比较容易，首先我已经有一台阿里云服务器用作 WireGuard 的公网节点，我的电脑只需分别配置 WireGuard、V2Ray（Xray） 和 AdGuard 即可实现上述三个目标，甚至如果操作得当只需要配置 ShadowRocket 或 Surge 这样的软件就可以实现所有的目标。

但这种方式的缺点就在于必须每个设备单独配置，而且（在不同的平台上）使用较多的软件组合也不利于配置的同步，此外一些设备配置 WireGuard 和用户脚本时更加复杂，这样的配置显然不能算理想。因此我的计划是使用一台 ArchLinux 的主机作为路由器，为接入该路由器的所有设备分配私有内网 IP，并且通过正确配置路由表内外部设备能够通过 WireGuard 互相访问，使所有的设备（在外部的通过 WireGuard）的流量都经过该路由器的过滤，这样所有的设备至多只需配置 WireGuard 即可实现上述的所有功能，而在该路由器下接入的设备则无需任何配置。

[//]: # (在发表于 Weekly 9 时应当添加 WireGuard 的背景介绍，但是此处先略去)
## 总体方案
整个方案的核心部分是一台路由器和一台公网服务器（如果路由器可以取得公网 IP 则可以省去，但为了验证这个方案我还是会同时使用），路由器的 DHCP 池设置为 `10.0.0.1/25`，而 WireGuard IP 池则为 `10.0.0.128/25`，这样一来就可以把所有的设备都放在 `10.0.0.1/24` 的子网中，只要路由器和 WireGuard 的路由规则正确，理论上连接到路由器或 WireGuard 的设备可以任意互相访问。

由于分流规则并非全部通过 IP 确定，所以不能简单地使用路由规则完成，必然会用到一个代理工具，这里选择使用 Xray，而广告拦截和用户脚本则选择 AdGuard 作为 MITM 代理。

## 使用设备
这个想法虽然很好，但所需的设备却十分刁钻：这个设备应当性能足以运行 ArchLinux 并基于 KVM 运行一个 Windows 系统，但是性能应当尽量不要太高，需要将能耗控制在适当的范围内；同时这个设备应当具有多网口和较好的无线天线，此外如果具有少量的扩展性则可以更好地充当一个影音终端。

许多天前我在摸鱼的时候发现了这样的一个精准命中需求的设备：[Maxtang 大唐TRI系列台式NUC迷你组装电脑英特尔12代四核双网口商务高速固态无风扇主机 【2.5G四网版】J6412 准系统](https://item.jd.com/10072905428786.html)，搭载 Intel J6412 处理器、四网口和 Wi-Fi 6 天线，甚至还支持通过 Sim 卡上网，基本满足了我的所有设想，因此我立即决定先买下来以备日后摸鱼。

我另外购买了一条光威的 8GB DDR4 内存，以及一条金储星的 256GB M.2 SATA 硬盘（要不是因为必须用 M.2 SATA 硬盘我还真没听说过这个品牌）。至此一切准备就绪，开始进入第一步。

## 安装系统
过程比较繁琐，单独记录在 [安装配置 ArchLinux](/post/archlinux.md)。

## 配置分流
使用 Xray 作为分流代理，所有设备无论是通过局域网还是 WireGuard 接入后都应该将代理地址设置为 Xray 的端口。

首先安装 Xray：
```bash
# 自动加载 systemd
sudo bash -c "$(curl -L https://github.com/XTLS/Xray-install/raw/main/install-release.sh)" @ install
```
编辑配置文件 `/usr/local/etc/xray/config.json`：
```json
{
  "log": {
    "access": "/var/log/xray/access.log",
    "error": "/var/log/xray/error.log",
    "loglevel": "warning"
  },
  "inbounds": [
    {
      "protocol": "socks",
      "settings": {
        "ip": "127.0.0.1",
        "auth": "noauth",
        "udp": true
      },
      "tag": "socksinbound",
      "port": 1081
    },
    {
      "protocol": "socks",
      "settings": {
        "ip": "127.0.0.1",
        "auth": "noauth",
        "udp": true
      },
      "tag": "socksinbound_all",
      "port": 1082
    },
    {
      "protocol": "http",
      "settings": {
        "timeout": 0
      },
      "tag": "httpinbound",
      "port": 8001
    },
    {
      "protocol": "http",
      "settings": {
        "timeout": 0
      },
      "tag": "httpinbound_all",
      "port": 8002
    }
  ],
  "outbounds": [
    {
      "protocol": "freedom",
      "settings": {
        "domainStrategy": "UseIP"
      },
      "tag": "direct"
    },
    {
      "tag": "Foreign",
      "sendThrough": "0.0.0.0",
      "mux": {
        "enabled": false,
        "concurrency": 8
      },
      "protocol": "vmess",
      "settings": {
        "vnext": [
          {
            "address": "foreign host",
            "users": [
              {
                "id": "uuidv4",
                "alterId": 0
              }
            ],
            "port": 65535
          }
        ]
      },
      "streamSettings": {
        "network": "ws",
        "security": "tls",
        "wsSettings": {
          "path": "/websocket/path",
          "headers": {
            "Host": "foreign host"
          }
        }
      }
    }
  ],
  "routing": {
    "domainStrategy": "AsIs",
    "rules": [
      {
        "type": "field",
        "inboundTag": ["httpinbound_all", "socksinbound_all"],
        "outboundTag": "Foreign"
      },
      {
        "type": "field",
        "ip": ["10.0.0.0/8", "geoip:cn"],
        "outboundTag": "direct"
      },
      {
        "type": "field",
        "ip": ["0.0.0.0/0"],
        "outboundTag": "Foreign"
      }
    ]
  }
}
```
重新启动 Xray 加载配置：
```bash
sudo systemctl restart xray
```

## 配置子网
### 有线和无线子网
我的路由器上有四个 2.5G RJ45 网口，我选择使用 LAN 4 作为 WAN 口，这四个接口默认的名称依次是 `enp1s0` `enp2s0` `enp3s0` `enp4s0`。为了方便起见，

按照 [ArchWiki](https://wiki.archlinux.org/title/Router#IP_configuration) 的说明，可以通过 netctl 或 systemd-networkd 完成下一步配置，关于这两个工具的对比，GPT-4 给出的评价是：
```text
If you're familiar with systemd and prefer tools that are part of the systemd ecosystem, or if you're setting up a system that you want to be lightweight and efficient (e.g., a server, router, or embedded system), then systemd-networkd might be the better choice.

If you like the idea of easily switchable profiles, especially for a mobile device or if you're more comfortable with Arch-specific tools, then netctl could be the way to go.
```
考虑到我最近应该不会配其他的非 ArchLinux 的路由器，我决定首先尝试使用 netctl 解决问题：
```bash
sudo pacman -S netctl
```

### WireGuard 子网
按照总体设计，WireGuard 子网的 IP 段是 `10.0.0.128/25`，因此有如下配置
```text
# /etc/wireguard/wg0.conf
[Interface]
PrivateKey = PrivateKey
ListenPort = 45555
Address = 10.0.0.128/25
```
对于每个需要连接到 WireGuard 的设备，在配置中添加：
```text
[Peer]
PublicKey = PublicKey
AllowedIPs = 10.0.0.[129-254]/32
```
完成配置后启动：
```bash
sudo systemctl enable --now wg-quick@wg0.service
```