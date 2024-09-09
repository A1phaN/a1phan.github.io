+++
category = "Learning"
tags = ["K8S"]
+++
# kubernetes 学习笔记
全栈必备，无须多言。

虽然可以使用 [minikube](https://minikube.sigs.k8s.io/) 作为学习环境，但是想要学的更细节一些，另外有一个集群也很方便（大概？）我挂一些自己的东西，因此准备在我的家用服务器上搞一个集群，简单来说就是搞若干的资源很少的虚拟机来当作集群的节点。

## 环境配置
使用搭载 ESXi 7.0 的服务器，安装一个 debian 12 的虚拟机（这里我选择了每个虚拟机 2C2G + 32G），先进行一些基本环境配置再复制节点，主要是安装容器运行时（Container Runtime）等环境。需要注意的是 debian 默认的安装程序会设置 swap 分区（也可能是因为给的内存小），需要手动删除该分区，否则 kubeadm 会报一个 Warning。

这里选择使用 CRI-O 作为容器运行时（实际上第一次尝试使用了 containerd，但是遇到了神秘的兼容性问题，这里没有深入调查）
```bash
apt-get update
apt-get install -y software-properties-common curl

KUBERNETES_VERSION=v1.29
PROJECT_PATH=prerelease:/main

curl -fsSL https://pkgs.k8s.io/core:/stable:/$KUBERNETES_VERSION/deb/Release.key |
    gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg

echo "deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/$KUBERNETES_VERSION/deb/ /" |
    tee /etc/apt/sources.list.d/kubernetes.list

curl -fsSL https://pkgs.k8s.io/addons:/cri-o:/$PROJECT_PATH/deb/Release.key |
    gpg --dearmor -o /etc/apt/keyrings/cri-o-apt-keyring.gpg

echo "deb [signed-by=/etc/apt/keyrings/cri-o-apt-keyring.gpg] https://pkgs.k8s.io/addons:/cri-o:/$PROJECT_PATH/deb/ /" |
    tee /etc/apt/sources.list.d/cri-o.list

apt-get update
apt-get install -y cri-o kubelet kubeadm kubectl

systemctl start crio.service

echo '[crio.image]
pause_image="registry.k8s.io/pause:3.6"' >> /etc/crio/crio.conf
systemctl reload crio
```

安装 kubeadm、kubelet 和 kubectl
```bash
apt-get update
apt-get install -y apt-transport-https ca-certificates curl gpg
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.29/deb/Release.key | sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.29/deb/ /' | sudo tee /etc/apt/sources.list.d/kubernetes.list
apt-get update
apt-get install -y kubelet kubeadm kubectl
apt-mark hold kubelet kubeadm kubectl

# 这段似乎重启需要重新执行一下
modprobe br_netfilter
echo "net.bridge.bridge-nf-call-iptables = 1" >> /etc/sysctl.conf
echo "net.ipv4.ip_forward = 1" >> /etc/sysctl.conf
sysctl -p
```

至此配置好基础镜像，将这个镜像复制若干分别作为 master 节点和 worker 节点。
> ### 在 ESXi 中复制镜像的方式：
> ESXi 的数据目录在 `/vmfs/volumes/{volume_name}` 下，如果一个虚拟机的文件在 `vm` 文件下，默认创建的虚拟机文件名形如 `vm.*`，只需复制以下文件：
> ```bash
> mkdir vm-copy
> cp vm/vm.vmx vm-copy
> cp vm/vm.vmdk vm-copy
> cp vm/vm-flat.vmdk vm-copy
> ```
> 在 ESXi 中导入新的虚拟机，然后开机时选择 `我已复制` 即可配置一台新的机器，启动后使用 hostnamectl 修改 hostname：
> ```bash
> sudo hostnamectl set-hostname new-hostname
> sudo sed -i 's/old-hostname/new-hostname/g' /etc/hosts
> ```

### master 节点设置
使用 `kubeadm` 初始化控制平面：
```bash
# 注意这里的 pod-network-cidr 必须和后面的网络配置相同
# 例如 kube-flannel 的默认设置为 10.244.0.0/16
# 初始化完成后会输出一条 kubeadm join 命令，需要保留下来
sudo kubeadm init --pod-network-cidr=10.244.0.0/16

mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config

# 如果需要修改默认的网络配置，可以先下载下来修改其中的 net-conf.json 内容
kubectl apply -f https://github.com/flannel-io/flannel/releases/latest/download/kube-flannel.yml

# 等待一点时间应该可以看到所有 pods 状态变为 Running
kubectl get pods --all-namespaces
```

### Worker 节点配置
使用前面的 `kubeadm join` 命令即可：
```bash
sudo kubeadm join master-host:6443 --token {TOKEN} --discovery-token-ca-cert-hash sha256:{HASH}
```

如果没有保存之前生成的 token 或 token 已过期（默认 24 小时），可以使用如下方式创建：
```bash
# 创建 token
> kubeadm token create
5didvk.d09sbcov8ph2amjw

# 获取 discovery-token-ca-cert-hash
> openssl x509 -pubkey -in /etc/kubernetes/pki/ca.crt | \
  openssl rsa -pubin -outform der 2>/dev/null | \
  openssl dgst -sha256 -hex | sed 's/^.* //'
8cb2de97839780a412b93877f8507ad6c94f73add17d5d7058e91741c9d5ec78
```

## kubernetes 入门
