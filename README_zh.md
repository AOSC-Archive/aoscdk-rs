# DeployKit

DeployKit 是 AOSC OS 的系统安装工具。

## 如何使用

AOSC OS LiveKit 中预装 DeployKit。进入 AOSC OS LiveKit 输入 `deploykit` 命令
即可开始使用。若您只希望使用 DeployKit（而不打算开发与调试），您应该不需要
自行安装。

## 编译与使用

**注意，安装 AOSC OS 只需使用 LiveKit 中自带的 DeployKit！**

若要自行编译使用 DeployKit（比如您想在其他发行版尝试 DeployKit），您需要在
编译前自行安装以下依赖：

- OpenSSL 1.1
- Glibc
- C Compile (like gcc)
- ncurses
- pkg-config
- libparted
- Rust + Cargo
- unsquashfs (from squash-tools)

若您使用的是 AOSC OS，请使用如下命令安装其依赖：

```
# apt install llvm pkg-config gcc parted ncurses openssl squashfs-tools
```

而后运行如下命令编译：

```
$ cargo build --release
```

## Retro

若要编译 DeployKit 的 AOSC OS/Retro 版本，请打开 `is_retro` 特性：

```
$ cargo build --release --features is_retro
```

## 调试

欲调试 DeployKit，请先完成如下步骤：

1. 创建一个用于安装系统的硬盘镜像：

```
// 确保您已加载 loop 模块，若如下命令出错，您可能需要重新编译内核
# modprobe loop

// 创建一个 35GiB 的硬盘镜像文件（以正常安装 AOSC OS KDE），起名为 test.img
$ dd if=/dev/zero of=/path/to/aoscdk-rs-src/test.img bs=1M count=35840 status=progress

// 使用 losetup 挂载镜像至 /dev/loop10
# losetup /dev/loop0 /path/to/aoscdk-rs-src/test.img

// 分区：分区一为主分区，若你的机器使用 UEFI 启动，则分区二为 EFI 系统分区 (ESP)
# cfdisk /dev/loop10

// 刷新分区表
# partprobe /dev/loop10
```

2. 编译 DeployKit 的调试版本：

```
// 编译调试版 DeployKit，注意不要加 --release
$ cargo build

// 启动调试版 DeployKit
# /path/to/aoscdk-rs-src/target/debug/aoscdk-rs
```

**在此建议使用性能较好的机器调试 DeployKit，因为使用性能较低的机器调试
DeployKit 是一种折磨！**

若需要输出日志到终端，但 TUI 显示消息比较困难，您可以借助 test 函数，
如 `frontend/mod` 中一例：

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

// 观察输出，您应该会看到类似这样的信息
 Running unittests (target/debug/deps/aoscdk_rs-3b358921c017024b)

// 而后执行如下命令进行调试
# target/debug/deps/aoscdk_rs-3b358921c017024b --nocapture
```
