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
