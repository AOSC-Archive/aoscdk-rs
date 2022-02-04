# DeployKit

DeployKit is the AOSC OS system installer.

## Usage

DeployKit comes pre-installed with LiveKit. Simply run the `deploykit` command
to launch the installer. You shouldn't need to compile or install LiveKit
manually unless you would like to help with development or debugging.

## Building DeployKit

**Please note that you'd only need to use the DeployKit pre-installed with
LiveKit to install AOSC OS.**

If you would like to compile your own DeployKit (or trying it out on a
different distribution, please install the following dependencies:

- OpenSSL 1.1
- Glibc
- C Compile (like gcc)
- ncurses
- pkg-config
- libparted
- Rust + Cargo
- unsquashfs (from squash-tools)

If you are using AOSC OS, please install the dependencies with the following
command:

```
# apt install llvm pkg-config gcc parted ncurses openssl squashfs-tools

```

And run the following command to build DeployKit:

```
$ cargo build --release
```

## Retro

If you would like to build DeployKit for AOSC OS/Retro, please enable the
`is_retro` feature:

```
$ cargo build --release --features is_retro
```

## Debugging

If you would like to debug DeployKit, please follow the following steps.

1. Create a hard disk image on which to install AOSC OS.

```
// Ensuring that you have loaded the loop kernel module, if the following
// command returns an error, you may need to recompile your kernel.
# modprobe loop

// Create a 35GiB hard disk image (to install AOSC OS KDE), naming it test.img.
$ dd if=/dev/zero of=/path/to/aoscdk-rs-src/test.img bs=1M count=35840 status=progress

// Mount the image as /dev/loop10 with losetup.
# losetup /dev/loop10 /path/to/aoscdk-rs-src/test.img

// Partition the image. The first partition should be the system partition.
// If you are using EFI, please create an ESP (EFI System Partition) as the
// second partition.
# cfdisk /dev/loop10

// Inform the kernel about partition changes.
# partprobe /dev/loop10
```

2. Building DeployKit for debugging.

```
// Build a debug version of DeployKit, leave out the --release parameter.
$ cargo build

// Launch debug DeployKit.
# /path/to/aoscdk-rs-src/target/debug/aoscdk-rs
```

**You may want to use a beefy device to debug DeployKit to spare yourself some
grey hair.**

If you would like DeployKit to print log to the terminal - and since that it
would be difficult to check real-time log with a TUI application - you may want
to use the `test` function.

Take the `frontend/mod` for example:

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

Run the following to begin real-time logging:

```
$ cargo test --nocapture

// You should see something like this.
 Running unittests (target/debug/deps/aoscdk_rs-3b358921c017024b)

// Run the following command to begin debugging with logging.
# target/debug/deps/aoscdk_rs-3b358921c017024b --nocapture
```
