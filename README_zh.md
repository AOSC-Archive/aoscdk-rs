# AOSC OS DeployKit
DeployKit 是 AOSC OS 系统安装工具。

## 如何使用
Deploykit 已经在 AOSC OS Livekit 中附带了，进入 AOSC OS Livekit 输入 `deploykit` 即可开始使用。若你只想使用 DeployKit（而不打算开发与调试），你应该不需要自行安装。

## 编译与使用
**注意，在大多时候你只需要使用 AOSC OS Livekit 自带的 DeployKit 就好！**

若要自行编译使用 DeployKit（例如，你在其他发行版中想尝试 deploykit），你需要自行安装其依赖再编译，依赖如下：

- OpenSSL 1.1
- Glibc
- C Compile (like gcc)
- ncurses
- pkg-config
- libparted
- Rust w/ Cargo

若你在 AOSC OS 下，请使用以下命令安装其依赖：

```
# apt install llvm pkg-config gcc parted ncurses openssl
```

之后再编译：

```
$ cargo build --release
```

## Retro
若要在一些较老的架构下运行，你可能需要打开 `is_retro` 特性：

```
$ cargo build --release --features is_retro
```

## 调试
若要调试 DeployKit，你可能需要做：

1. 创建一个用于安装系统的硬盘镜像（img）：

```
// 确保你的环境已经加载了 loop 模块，若失败，你可能需要重新编译内核。
# modprobe loop 
// 创建一个 35GB 的硬盘镜像文件（为了能够正常安装 AOSC OS KDE tarball），起名为 test.img:
$ dd if=/dev/zero of=/path/to/aoscdk-rs-src/test.img bs=1M count=35840 status=progress
// 使用 losetup 挂载镜像至 /dev/loop0
# losetup /dev/loop0 /path/to/aoscdk-rs-src/test.img
// 分区，分区一为主分区，若你的机器使用 UEFI 启动，则分区二为 EFI 分区
# cfdisk /dev/loop0
// 刷新分区表
# partprobe /dev/loop0
```

2. 编译 debug 版本的 deploykit：

```
// 编译 debug 版 deploykit，注意不要加 --release
$ cargo build
// 运行
# /path/to/aoscdk-rs-src/target/target/debug/aoscdk-rs
```

**在此建议使用 I/O 较快的机器调试 DeployKit，因为 I/O 较慢的机器调试 DeployKit 是一种折磨！**

若需要输出日志到终端，但 TUI 显示消息比较困难，你可以借助 test 函数，像 `frontend/mod` 的一个例子：

```Rust
#[test]
fn test_download_amd64() {
    use tempfile::TempDir;
    let json = r#"{"variant":{"name":"Base","size":821730832,"install_size":4157483520,"date":"20210602","sha256sum":"b5a5b9d889888a0e4f16b9f299b8a820ae2c8595aa363eb1e797d32ed0e957ed","url":"os-amd64/base/aosc-os_base_20210602_amd64.tar.xz"},"partition":{"path":"/dev/loop0p1","parent_path":"/dev/loop0","fs_type":"ext4","size":3145728},"mirror":{"name":"Beijing Foreign Studies University","name-tr":"bfsu-name","loc":"China","loc-tr":"bfsu-loc","url":"https://mirrors.bfsu.edu.cn/anthon/aosc-os/"},"user":"test","password":"test","hostname":"test","locale":"","continent":"Asia","city":"Shanghai","tc":"UTC"}"#;
    let config = serde_json::from_str(json).unwrap();
    let (tx, _rx) = std::sync::mpsc::channel();
    let tempdir = TempDir::new().unwrap().into_path();
    begin_install(tx, config, tempdir).unwrap();
}
```

运行：

```
$ cargo test --nocapture
// 若你的测试需要管理员权限，观察输出，看到类似这样的信息：
 Running unittests (target/debug/deps/aoscdk_rs-3b358921c017024b)
// 再执行
# sudo target/debug/deps/aoscdk_rs-3b358921c017024b --nocapture
```