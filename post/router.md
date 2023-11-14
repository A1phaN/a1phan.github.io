+++
category = "Hacking"
tags = ["Router"]
+++
# 从零开始配置路由器
这是一个鸽了非常非常久的项目，长期以来我一直想要一套完全自主可控的网络环境，我希望能实现以下目标：
- 让所有设备在同一个 WireGuard 网段中，使所有的设备无论在什么环境下都可以自由互相访问
- 自动分流（私有内网、校园网、国内、国外）
- 基于 MITM 的广告拦截和用户脚本

这些想法如果只是在个别设备上实现其实相对而言比较容易，首先我已经有一台阿里云服务器用作 WireGuard 的公网节点，我的电脑只需分别配置 WireGuard、V2Ray（X-Ray） 和 AdGuard 即可实现上述三个目标，甚至如果操作得当只需要配置 ShadowRocket 或 Surge 这样的软件就可以实现所有的目标。

但这种方式的缺点就在于必须每个设备单独配置，而且（在不同的平台上）使用较多的软件组合也不利于配置的同步，此外一些设备配置 WireGuard 和用户脚本时更加复杂，这样的配置显然不能算理想。因此我的计划是使用一台 ArchLinux 的主机作为路由器，为接入该路由器的所有设备分配私有内网 IP，并且通过正确配置路由表内外部设备能够通过 WireGuard 互相访问，使所有的设备（在外部的通过 WireGuard）的流量都经过该路由器的过滤，这样所有的设备至多只需配置 WireGuard 即可实现上述的所有功能，而在该路由器下接入的设备则无需任何配置。

[//]: # (在发表于 Weekly 9 时应当添加 WireGuard 的背景介绍，但是此处先略去)
## 总体方案

## 使用设备
这个想法虽然很好，但所需的设备却十分刁钻：这个设备应当性能足以运行 ArchLinux 并基于 KVM 运行一个 Windows 系统，但是性能应当尽量不要太高，需要将能耗控制在适当的范围内；同时这个设备应当具有多网口和较好的无线天线，此外如果具有少量的扩展性则可以更好地充当一个影音终端。

许多天前我在摸鱼的时候发现了这样的一个精准命中需求的设备：[Maxtang 大唐TRI系列台式NUC迷你组装电脑英特尔12代四核双网口商务高速固态无风扇主机 【2.5G四网版】J6412 准系统](https://item.jd.com/10072905428786.html)，搭载 Intel J6412 处理器、四网口和 Wi-Fi 6 天线，甚至还支持通过 Sim 卡上网，基本满足了我的所有设想，因此我立即决定先买下来以备日后摸鱼。

我另外购买了一条光威的 8GB DDR4 内存，以及一条金储星的 256GB M.2 SATA 硬盘（要不是因为必须用 M.2 SATA 硬盘我还真没听说过这个品牌）。至此一切准备就绪，开始进入第一步。

## 安装系统
这里一半是为了做教程，一半是为了防止我忘记了自己读过的文档，尽量做一个详细的记录。

首先通过 [TUNA 源](https://mirrors.tuna.tsinghua.edu.cn/archlinux/iso) 下载最新的 ArchLinux 镜像：
```bash
wget https://mirrors.tuna.tsinghua.edu.cn/archlinux/iso/2023.09.01/archlinux-2023.09.01-x86_64.iso
```

这里我选择通过 USB 安装系统，通过 diskutil 找到磁盘名称：
```bash
$ diskutil list
/dev/disk8 (external, physical):
   #:                       TYPE NAME                    SIZE       IDENTIFIER
   0:     FDisk_partition_scheme                        *65.0 GB    disk8
   1:               Windows_NTFS 未命名                  65.0 GB    disk8s1
```

然后直接写入：
```bash
diskutil unmountDisk /dev/disk8
sudo dd bs=4M if=archlinux-2023.09.01-x86_64.iso of=/dev/disk8 conv=fsync oflag=direct status=progress
```

从 U 盘启动系统，进入安装程序，ArchLinux 的安装程序是预装了 Zsh 和一些其他常用工具的环境，需要手动完成系统安装。
1. 通过 [iwctl](https://wiki.archlinux.org/title/Iwctl) 连接网络
2. 更新系统时间
   ```bash
   timedatectl
   ```
3. 硬盘分区：
   1. 找到目标磁盘
      ```bash
      fdisk -l
      ```
      这里看到我的硬盘对应的路径是 `/dev/sda`，由于这是一块新硬盘，且没有安装多系统的打算，需要创建分区表和 EFI 分区。
   2. 创建分区表和 EFI 分区：
      ```bash
      fdisk /dev/sda
      ```
      在 fdisk 工具中依次执行：
      - `g`: 创建 GPT 分区表
      - `n`: 创建新分区，前两项默认，第三项 `+512M` 来创建一个 512M 的 EFI 分区
      - `t`: 将刚刚创建的分区修改为 EFI System 类型
      - `w`: 写入修改
      
      格式化该分区：
      ```bash
      mkfs.fat -F32 /dev/sda1
      ```
   3. 创建数据分区
      在 fdisk 工具中依次执行：
      - `n`: 全部默认，将剩余空间都用于数据分区
      - `w`: 写入修改

      格式化该分区：
      ```bash
      mkfs.ext4 /dev/sda2
      ```
4. 挂载硬盘
   ```bash
   mount /dev/sda2 /mnt
   mount --mkdir /dev/sda1 /mnt/boot
   ```
5. 安装系统

   首先添加 TUNA 源：
   ```bash
   vim /etc/pacman.d/mirrorlist
   ```
   在最前面添加
   ```text
   Server = http://mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os/$arch
   ```
   安装基本包：
   ```bash
   pacstrap -K /mnt base linux linux-firmware # 这里可能需要 dhcpcd
   ```
6. 配置系统
   1. 配置 fstab
      ```bash
      genfstab -L /mnt >> /mnt/etc/fstab
      ```
   2. chroot 到刚安装的系统中
      ```bash
      arch-chroot /mnt
      ```
   3. 设置时区
      ```bash
      ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
      hwclock --systohc
      ```
   4. 设置地区
      ```bash
      # 新的系统中没有安装 vim，可以 pacman -S vim 安装或使用其他方式编辑
      vim /etc/locale.gen
      ```
      在文件中找到 `zh_CN.UTF-8 UTF-8` `en_US.UTF-8 UTF-8` 这两行，去掉行首的#号，保存并退出。然后运行
      ```bash
      locale-gen
      ```
      在 `/etc/locale.conf` 的第一行写入 `LANG=en_US.UTF-8`
   5. 设置主机名，这里我使用 `router`：
      ```bash
      echo 'router' > /etc/hostname
      echo -e '127.0.0.1\trouter' > /etc/hosts
      echo -e '::1\trouter' > /etc/hosts
      echo -e '127.0.1.1\trouter.localdomain router' > /etc/hosts
      ```
   6. 设置 root 密码
      ```bash
      passwd
      ```
   7. 安装 intel-ucode
      ```bash
      pacman -S intel-ucode
      ```
   8. 安装 BootLoader
      ```bash
      pacman -S grub efibootmgr
      grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=grub
      grub-mkconfig -o /boot/grub/grub.cfg
      ```
   9. 重新启动
      ```bash
      # 退出 chroot
      exit
      # 卸载文件系统
      umount /mnt/boot
      umount /mnt
      # 重新启动
      reboot
      ```

至此完成新系统的安装

### 连接网络
我所选用的设备有多种方式可以连接网络，但实际需求中应该主要是通过有线或者 Sim 卡上网，然后通过 LAN 口和 Wi-Fi 组建局域网。

#### 在校园网中
通过网线连接校园网，启动 DHCP 客户端：
```bash
systemctl enable --now dhcpcd
```
通过 U 盘拷贝一份最新版的 [auth-thu](https://github.com/z4yx/GoAuthing) 和 [systemd 配置](https://github.com/z4yx/GoAuthing/blob/master/docs/goauthing%40.service)，完成校园网登录：
```bash
mv ./auth-thu /usr/local/bin
cp goauthing@.service /usr/lib/systemd/system
echo '{"username":"username","password":"password"}' > ~/.auth-thu
systemctl enable --now goauthing@root.service
```
#### 使用 Sim 卡

### 基本配置
联网后安装一些必要的软件：
```bash
pacman -S iwd openssh sudo tmux vim wireguard-tools zsh
```
#### 添加日常使用的用户
```bash
# 取消注释 %whell ALL=(ALL:ALL) ALL
EDITOR=/usr/bin/vim visudo
# 添加新用户
useradd -m -G wheel -s /usr/bin/zsh user_name
# 设置新用户的密码
passwd user_name
```
随后重新启动并以新用户登录

#### 配置 WireGuard 和 SSH 方便远程访问
```bash
sudo systemctl enable --now sshd
sudo vim /etc/wireguard/wg0.conf
sudo systemctl enable --now wg-quick@wg0.service
```

## 配置子网
我的路由器上有四个 2.5G RJ45 网口，
按照 ArchWiki 