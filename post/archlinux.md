+++
category = "Hacking"
tags = ["Linux"]
+++
# 安装配置 ArchLinux
这里一半是为了做教程，一半是为了防止我忘记了自己读过的文档，尽量做一个详细的记录。

## 安装系统
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
1. 通过 [iwctl](https://wiki.archlinux.org/title/Iwctl) 或网线连接网络。
2. 更新系统时间：
   ```bash
   timedatectl
   ```
3. 硬盘分区：
   1. 找到目标磁盘：
      ```bash
      fdisk -l
      ```
      这里看到我的硬盘对应的路径是 `/dev/sda`，由于这是一块新硬盘，且没有安装多系统的打算，需要创建分区表和 EFI 分区。
   2. 创建分区表和 EFI 分区：
      ```bash
      fdisk /dev/sda
      ```
      在 fdisk 工具中依次执行：
      - `g`: 创建 GPT 分区表；
      - `n`: 创建新分区，前两项默认，第三项 `+512M` 来创建一个 512M 的 EFI 分区；
      - `t`: 将刚刚创建的分区修改为 EFI System 类型；
      - `w`: 写入修改。

      格式化该分区：
      ```bash
      mkfs.fat -F32 /dev/sda1
      ```
   3. 创建数据分区：
      在 fdisk 工具中依次执行：
      - `n`: 全部默认，将剩余空间都用于数据分区；
      - `w`: 写入修改。

      格式化该分区：
      ```bash
      mkfs.ext4 /dev/sda2
      ```
4. 挂载硬盘：
   ```bash
   mount /dev/sda2 /mnt
   mount --mkdir /dev/sda1 /mnt/boot
   ```
5. 安装系统：

   首先添加 TUNA 源：
   ```bash
   vim /etc/pacman.d/mirrorlist
   ```
   在最前面添加：
   ```text
   Server = http://mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os/$arch
   ```
   安装基本包：
   ```bash
   pacstrap -K /mnt base linux linux-firmware # 这里可能需要 dhcpcd
   # 如果需要连接校园网可以提前下载 auth-thu
   curl https://github.com/z4yx/GoAuthing/releases/download/v2.2.1/auth-thu.linux.x86_64 -o /usr/local/bin/auth-thu
   curl https://raw.githubusercontent.com/z4yx/GoAuthing/master/docs/systemd/user/goauthing.service -o /usr/lib/systemd/system/goauthing@.service
   ```
6. 配置系统：
   1. 配置 fstab：
      ```bash
      genfstab -L /mnt >> /mnt/etc/fstab
      ```
   2. chroot 到刚安装的系统中：
      ```bash
      arch-chroot /mnt
      ```
   3. 设置时区：
      ```bash
      ln -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
      hwclock --systohc
      ```
   4. 设置地区：
      ```bash
      # 新的系统中没有安装 vim，可以 pacman -S vim 安装或使用其他方式编辑
      vim /etc/locale.gen
      ```
      在文件中找到 `zh_CN.UTF-8 UTF-8` `en_US.UTF-8 UTF-8` 这两行，去掉行首的#号，保存并退出。然后运行：
      ```bash
      locale-gen
      ```
      在 `/etc/locale.conf` 的第一行写入 `LANG=en_US.UTF-8`。
   5. 设置主机名，这里我使用 `router`：
      ```bash
      echo 'router' > /etc/hostname
      echo -e '127.0.0.1\tlocalhost' >> /etc/hosts
      echo -e '::1\tlocalhost' >> /etc/hosts
      echo -e '127.0.1.1\trouter.localdomain router' >> /etc/hosts
      ```
   6. 设置 root 密码：
      ```bash
      passwd
      ```
   7. （对于 Intel CPU 的设备）安装 intel-ucode：
      ```bash
      pacman -S intel-ucode
      ```
   8. 安装 BootLoader：
      ```bash
      pacman -S grub efibootmgr
      grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=grub
      grub-mkconfig -o /boot/grub/grub.cfg
      ```
   9. 重新启动：
      ```bash
      # 退出 chroot
      exit
      # 卸载文件系统
      umount /mnt/boot
      umount /mnt
      # 重新启动
      reboot
      ```

至此完成新系统的安装。

## 连接网络
我所使用的设备在 [从零开始配置路由器](/post/router.md) 中有介绍，这里写连接网络的方式。

尽管这台机器可以通过有线、无线和 Sim 卡上网，但作为路由器无线网卡自然是被用作 LAN，因此只有两种连接外部网络的方式：

### 在校园网中
通过网线连接校园网，启动 DHCP 客户端：
```bash
systemctl enable --now dhcpcd
echo '{"username":"username","password":"password"}' > ~/.auth-thu
systemctl enable --now goauthing@root.service
```

### 使用 Sim 卡

## 配置系统
联网后安装一些必要的软件：
```bash
pacman -S htop git iwd less man openssh sudo tmux unzip vim wget wireguard-tools zsh
```

### 添加日常使用的用户
```bash
# 取消注释 %wheel ALL=(ALL:ALL) ALL
EDITOR=/usr/bin/vim visudo
# 添加新用户
useradd -m -G wheel -s /usr/bin/zsh user_name
# 设置新用户的密码
passwd user_name
```
随后重新启动并以新用户登录

### 启动 SSH 服务并禁止密码登录
修改 SSHD 配置：
```bash
sudo vim /etc/ssh/sshd_config
```
其中找到 `PasswordAuthentication` 并将其修改为 `no`（如果没有则在文件末添加 `PasswordAuthentication no`）。

启动 SSH 服务：
```bash
sudo systemctl enable --now sshd
```

### 配置 Zsh
在上面创建用户时我指定了 `/usr/bin/zsh` 作为默认 shell，但并未进行配置，简单起见使用 oh-my-zsh 配置：
```bash
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
```

### 安装 KDE
本来对于路由器其实可以没有图形界面，但是后面操作 Qemu 的时候没有图形界面就有些复杂，因此还是准备装一个 KDE：
```bash
sudo pacman -S plasma-meta plasma-wayland-session sddm
sudo systemctl enable --now sddm.service
```

### 安装 KVM
根据路由器的总体方案，需要运行一个 AdGuard 作为 MITM 代理，但 AdGuard 并不支持 Linux 平台，因此需要通过 Windows 虚拟机完成网页代理。

首先根据 [ArchWiki](https://wiki.archlinux.org/title/KVM) 检查硬件兼容性并加载对应的模块，然后安装 Qemu 和相关工具：
```bash
sudo pacman -S qemu libvirt virt-manager ebtables dnsmasq bridge-utils openbsd-netcat swtpm
sudo systemctl enable --now libvirtd
```