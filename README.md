# spl-stake

SPL 质押合约

功能：

1、管理员（admin）允许设置支持的币种 SPL Token

2、用户（user）允许存入和提取SPL Token

3、合约跟踪和记录每个用户的 SPL Token 余额

4、测试币水龙头，用户可以领取 SPL 测试币。


```shell
$ cargo version
cargo 1.80.0 (376290515 2024-07-16)
$ rustc --version
rustc 1.80.0 (051478957 2024-07-21)
```

```shell
$ solana --version
solana-cli 1.18.15 (src:767d24e5; feat:4215500110, client:SolanaLabs)
```

```shell
$ solana-test-validator --version
solana-test-validator 1.18.15 (src:767d24e5; feat:4215500110, client:SolanaLabs)
```

```shell
$ anchor --version   
anchor-cli 0.30.1
```

```shell
$ node --version
v20.16.0
```

```shell
$ npm --version
10.8.1
```

```shell
$ yarn --version
1.22.22
```

* 编译

```shell
$ anchor build --arch sbf
```

* 运行单元测试

```shell
$ yarn install
$ anchor test --arch sbf
$ cargo test-sbf
```

* 启动 solana 本地测试节点

```shell
$ solana-test-validator
```

* 部署

```shell
$ anchor deploy
```

* 验证 IDL

```shell
anchor idl init --filepath target/idl/spl_stake.json 4gzQgfWr98rvu5WqJZophHUS42rMWAz6yntxyEdqcQNY
```
