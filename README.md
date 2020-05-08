# naive-kv

本项目为一个键值服务器演示。

## 子项目

### Server

基于 actix-web 框架。查询结果（若有）由 bincode 库编码，以确保返回大量数据时的带宽消耗处于与直接使用 socket 实现整个服务器时相近的水平。使用 ``u64`` 而非 ``[u8; 8]`` 作为键，因为 ``u64`` 之间的比较会快一些。借助 RAII 实现 graceful shutdown（POST "/stop" 或 SIGTERM）时自动保存键值数据到文件。

实现的接口：
* GET "/"：类似于 TiKV 的 Get；
* GET "/scan"：类似于 TiKV 的 Scan；
* PUT "/"：类似于 TiKV 的 Put，旧值（若有）会被覆盖；
* DELETE "/"：类似于 TiKV 的 Delete，忽略本不存在的键；
* POST "/stop"：关闭服务器（graceful shutdown）。

从安全角度来说，上述最后一个接口存在隐患，但考虑到本项目只是一个原理演示，我们暂时不去解决这个问题。

运行服务器：

```powershell
$env:RUST_LOG="info"
cargo run --bin server
```

### Stopper

一个用来发送 POST "/stop" 的简易程序。

关闭服务器：
```powershell
cargo run --bin stopper
```

### Client

与服务器相应的一个客户端。

查看运行提示：
```powershell
cargo run --bin client
```

## 其他

本项目已运行过 ``cargo fmt``、``cargo clippy``、``cargo fix``。

在服务端准备了部分文档，可 ``cargo doc --open`` 查看。

在客户端准备了少量的单元测试，见 ``client/src/lib.rs``。
