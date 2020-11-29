# rust 智能合约样板

## 准备工作

1. 安装 rust 环境，包括 rustup, cargo, rustc https://www.rust-lang.org/tools/install
2. 安装 wasm-pack https://rustwasm.github.io/wasm-pack/installer/
3. ```npm install```


## 编译运行

1. 编译

```shell script
wasm-pack build --target  web --no-typescript 
```

2. 运行

```shell script
# 手动构造 abi
echo '[{"name":"init","type":"function","inputs":[{"name":"a","type":"string"},{"name":"b","type":"string"}],"outputs":[{"type":"string"}]},{"name":"concat_world","type":"function","inputs":[{"name":"s","type":"string"}],"outputs":[{"type":"string"}]}]' >> ./pkg/hello_wasm_bg.abi.json
light-server -s .
```

打开 localhost:4000/tests/index.html