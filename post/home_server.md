+++
category = "Hacking"
tags = ["Linux"]
+++
# 我的家庭服务器配置
~~长远来看我是希望搞一台 ESXi 服务器来承载各种需求的~~，但是这个计划有一些障碍，一方面是 ESXi 官方只支持服务器 CPU 和主板，而我个人又很不喜欢淘电子垃圾，真要搞服务器成本有些太高，需求相对又没那么强烈。另一方面我希望我的 All in One 服务器可以做游戏机，用服务器的话单核无论如何都上不去。

UPDATE：坏消息是博通收购 VMWare 后决定不提供免费的 ESXi 了，好消息是我不必再纠结这个问题，本身家庭服务器的性能需求是没有那么大的，直接用一个 Windows 宿主机挂一些别的虚拟机也未尝不可以接受。

大部分的对环境要求不高的简单需求，可以挂在[我的路由器](/post/router.md)上，而一些自定义的服务则可以挂在虚拟的 Kubernetes 集群中。

## Database
DB 9 的同款需求，对于我个人来说很多时候没必要为每件事搞一个数据库，而且此前看到一篇反对数据库容器化的文章（可惜没保存链接）感觉颇有道理，所以直接在宿主机部署一个数据库。

### PostgreSQL
虽然我认为把数据库放在宿主机是一个好主意，但 PostgreSQL 的一系列 `createdb` `createuser` 的命令很难不让人觉得他们默认 PostgreSQL 独占一个环境。大约反对容器化的人的意思也是数据库占据整个裸机吧。
```bash
sudo pacman -S postgresql
sudo su - postgres -c "initdb --locale en_US.UTF-8 -D '/var/lib/postgres/data'"
sudo systemctl enable --now postgresql.service
sudo su - postgres -c "createuser --interactive"
```

### MongoDB
包 `mongodb` 是从头编译的，真的很慢，可以换成 `mongodb-bin`。
```bash
yay -S mongodb mongosh-bin
sudo systemctl enable --now mongodb.service
```

### Redis
```bash
sudo pacman -S redis
sudo systemctl enable --now redis.service
```

## Home Assistant OS
一个我觉得很值得仔细研究的东西，但是现在还没研究，占个位

## Kubernetes
虽然用多个虚拟机节点做 Kubernetes 集群颇有一些脱裤子放屁的意思，但是因为工作会用到，拿来玩玩倒也无不可。