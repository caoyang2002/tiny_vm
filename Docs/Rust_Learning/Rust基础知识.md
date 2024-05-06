---
title: "Rust 的基础知识"
author: caoyang2002
---

[toc]

- 官网：https://www.rust-lang.org/zh-CN/

# 一、安装与更新

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 1. 更新 rust

- `rustup update`  更新
- `rustup self uninstall`  卸载
- `rustup component add rustfmt`  添加组件
- `rustup --version`  查看版本



`rustup install nightly` 安装夜版（开发版）

`rustup default nightly` 更改为夜版

`rustup default stable`  更改为稳定版





## 2. Rust 编译器

- `rustc --version`  查看版本
- `rustc -o output_filename filename.rs`  编译生成二进制文件
- `rustc --create-type lib filename.rs`  编译生成库文件





## 3. 开发环境

- vscode
    - rust-analyzer
    - Error Lens



## 4. Rust 的包管理工具 Cargo

- 隐式地使用 rustc 进行编译
- 命令：
    - 创建 `cargo new project_name`
    - 创建一个新的 Rust 库项目 `cargo new --lib project_name`
    - 构建项目（生成二进制可执行文件或库文件）`cargo build`
    - 生成优化的可执行文件，常用于生产环境 `cargo build --release`
    - 检测  `cargo check`
    - 运行  `cargo run` 
    - 运行  `cargo run` 
    - 测试 `cargo test`





# cargo.toml

```toml
[package] 
name = "rysigy"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- [package] 设置项目名、版本等
- [dependencies] 设置依赖
    - [build-dependencies]  列出了在构建项目时需要的依赖项
    - [dev-dependencoes] 列出了只在开发时需要的依赖项













# 错误

```bash
caoyang@cccy Rust_Learning % rustup update
info: syncing channel updates for 'stable-aarch64-apple-darwin'
error: could not download file from 'https://static.rust-lang.org/dist/channel-rust-stable.toml.sha256' to '/Users/caoyang/.rustup/tmp/a8e9zeo21_hs07fn_file'
info: checking for self-update
error: could not download file from 'https://static.rust-lang.org/rustup/release-stable.toml' to '/var/folders/lk/1mc2q1150j9btq1mbn_s32v40000gn/T/rustup-updateohYPD2/release-stable.toml': failed to make network request: error sending request for url (https://static.rust-lang.org/rustup/release-stable.toml): error trying to connect: connection closed via error: error trying to connect: connection closed via error: connection closed via error
```

> 网络故障
