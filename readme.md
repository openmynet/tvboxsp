# Readme
tvbox 视频源检测与合并工具

## 构建
```bash
cargo build --release
```
## 运行
```bash

tvboxsp -h
# 合并指定文件中的所有视频源
tvboxsp -m merge -i ./data/input.txt -o ./data/output.json

# 检测视频源文件
tvboxsp -i ./data/output.json
# 检查视频源地址，并导出新的视频源文件
tvboxsp -i https://raw.liucn.cc/box/m.json

```

## 使用
```
# 源地址
https://github.com/openmynet/tvboxsp/raw/master/data/output.json

# 镜像1
https://cdn.staticaly.com/gh/openmynet/tvboxsp/master/data/output.json

# 镜像2
https://ghproxy.com/https://raw.githubusercontent.com/openmynet/tvboxsp/raw/master/data/output.json


```


## 跨平台编译
> windows powershell

```bash
docker run --rm -it -v "${pwd}:/home/rust/src" -v "${env:userprofile}/.cargo/registry:/usr/local/cargo/registry" -v "${env:userprofile}/.cargo/config:/usr/local/cargo/config" rust:1.66-bullseye


# 修改软件源镜像
sed -i 's#http://deb.debian.org#https://mirrors.ustc.edu.cn#g' /etc/apt/sources.list
sed -i 's#http://security.debian.org#http://mirrors.ustc.edu.cn#g' /etc/apt/sources.list

# 安装必要的软件
apt update -y && apt-get install libssl-dev-y 

# aarch64
# # 添加架构支持
# dpkg --add-architecture arm64
# # 安装必要的软件
# apt update -y && apt upgrade -y && apt-get install gcc-aarch64-linux-gnu -y 

# 进入工作区
cd /home/rust/src/
cp ./cargo.config /usr/local/cargo/config

# 打包编译
cargo build --release


```