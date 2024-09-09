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
整个方案的核心部分是一台路由器和一台公网服务器（如果路由器可以取得公网 IP 则可以省去，但为了验证这个方案我还是会同时使用），路由器的 DHCP 池设置为 `10.1.1.128/25`，而 WireGuard IP 池则为 `10.1.1.0/25`，这样一来就可以把所有的设备都放在 `10.1.1.0/24` 的子网中，只要路由器和 WireGuard 的路由规则正确，理论上连接到路由器或 WireGuard 的设备可以任意互相访问。

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
我的路由器上有四个 2.5G RJ45 网口，我选择使用 LAN 1 作为 WAN 口，这四个接口默认的名称依次是 `enp1s0` `enp2s0` `enp3s0` `enp4s0`。为了方便起见，使用 systemd networkd 对这几个接口重命名，分别命名为 `wan` `lan2` `lan3` `lan4`。以 `wan` 为例：
```ini
# /etc/systemd/network/10-wan.link
[Match]
MACAddress=00:00:00:00:00:00

[Link]
Description=WAN
Name=wan
```

#### 子网配置
通过 systemd networkd 分别配置 `wan` 和 `lan*`。`wan` 口使用 DHCP 获取 IP：
```ini
# /etc/systemd/network/20-wan.network
[Match]
Name=wan

[Network]
DHCP=yes
# 这里我不确定是 DHCP 配置问题导致没有拿到合适的 DNS，还是我的 DHCP 服务器问题
DNS=1.1.1.1
```

`lan*` 口通过一个网桥连接，网桥配置为静态 IP 地址，并将通过 `br0` 口传来的数据视为从本地发出的请求：
```ini
# /etc/systemd/network/25-br0.netdev
[NetDev]
Name=br0
Kind=bridge

# /etc/systemd/network/25-br0.network
[Match]
Name=br0

[Network]
Address=10.1.1.129/25
DNS=1.1.1.1
IPMasquerade=both

# /etc/systemd/network/20-lan2.network
[Match]
Name=lan2

[Network]
Bridge=br0
```

开启内核转发、启动 systemd networkd 并重启应用变更：
```bash
echo 'net.ipv4.ip_forward=1' | sudo tee /etc/sysctl.conf
sudo systemctl enable --now systemd-sysctl
sudo systemctl enable --now systemd-networkd
sudo reboot
```
（对于 `.link` 配置需要重启，如果只修改了 `.network` 配置可以通过 `systemctl daemon-reload` 和 `networkctl reload` 重新加载）。

#### 无线接入点配置
通过 `hostapd` 配置无线接入点：
```bash
sudo pacman -S hostapd
```

编辑配置文件 `/etc/hostapd/hostapd.conf`
```ini
interface=wlan

ssid=A1phaN
driver=nl80211
country_code=CN

# a 是 802.11a (5 GHz)，但是似乎模式和信道、国家代码相关，等测试出可用的 5 GHz 配置再改
hw_mode=g

wpa=2
auth_algs=1

# 加密协议；禁用不安全的 TKIP
wpa_pairwise=CCMP
# 加密算法
wpa_key_mgmt=WPA-PSK
# 密码
wpa_passphrase=some_password

# 启用802.11n支持
ieee80211n=1
# 启用802.11ac支持
ieee80211ac=1
# 启用802.11ax支持
ieee80211ax=1

# TODO: Use "iw list" to show device capabilities and modify ht_capab and vht_capab accordingly
#ht_capab=[HT40+][SHORT-GI-40][TX-STBC][RX-STBC1][DSSS_CCK-40]
#vht_capab=[RXLDPC][SHORT-GI-80][TX-STBC-2BY1][RX-STBC-1]
```

启动 `hostapd`
```bash
sudo systemctl enable --now hostapd.service
```

#### DNS 和 DHCP 配置
关于不同 DHCP 服务器工具的对比可以参考 [Router ArchWiki](https://wiki.archlinux.org/title/Router#DNS_and_DHCP) 中这一部分的内容。这里我选择使用 `dnsmasq`。修改配置文件 `/etc/dnsmasq.conf`：
- 添加 `interface=lan2` `interface=lan3` `interface=lan4` `interface=wlan`
- 添加 `dhcp-range=10.1.1.129,10.1.1.254,255.255.255.128,12h`

至此可以通过网线连接 LAN 口或通过 WLAN 拿到子网 IP 并顺利上网。

> 后来因为换了地方，实际配置的时候通过 `wlan` 口联网，将四个网口都作为 lan 口，只需通过 `iwd` 连接无线网，禁用 `hostapd` 并将上述 `wan` 口的配置用于 `wlan` 口即可。

#### 插曲：配置 802.1x 认证的网络
通过 wpa_supplicant 实现 802.1x 认证，配置文件如下：
```
# /etc/wpa_supplicant/wpa_supplicant-wired-wan.conf
ctrl_interface=/run/wpa_supplicant

network={
    key_mgmt=IEEE8021X
    eap=PEAP
    identity="your_username"
    password="your_password"
    phase2="auth=MSCHAPV2"
}
```

其中 `eap` 和 `phase2` 的内容可能需要根据网络设置，我这里通过我的 Mac 联网后得到了这两个值。

### WireGuard 子网
按照总体设计，WireGuard 子网的 IP 段是 `10.1.1.0/25`，因此有如下配置
```text
# /etc/wireguard/wg0.conf
[Interface]
PrivateKey = PrivateKey
ListenPort = 45555
Address = 10.1.1.0/25
```
对于每个需要连接到 WireGuard 的设备，在配置中添加：
```text
[Peer]
PublicKey = PublicKey
AllowedIPs = 10.1.1.[1-127]/32
```

配置 iptables 允许 WireGuard 网络和路由器的子网互通，并
```bash
sudo iptables -A FORWARD -i wg0 -o br0 -s 10.1.1.0/25 -d 10.1.1.128/25 -j ACCEPT
sudo iptables -A FORWARD -i br0 -o wg0 -s 10.1.1.128/25 -d 10.1.1.0/25 -j ACCEPT
sudo iptables -t nat -A POSTROUTING -o eth0 -s 10.1.1.0/25 -d 10.1.1.128/25 -j MASQUERADE
sudo iptables-save | sudo tee /etc/iptables/router.rules
```
完成配置后启动：
```bash
sudo systemctl enable --now wg-quick@wg0.service
```