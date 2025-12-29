# rust-cli-playground

这是一个用于 **Rust 学习热身的 CLI 小项目**。

本项目的目的不是实现复杂功能，而是帮助我建立：

- 使用 Cargo 创建 Rust 项目的基本手感
- Rust 项目结构与模块拆分的直觉
- 跑通「创建 → 编写 → 构建 → 运行」的完整流程

## 🎯 项目目标

本项目只做三件事：

1. 启动一个 Rust CLI 程序
2. 接收命令行参数
3. 打印接收到的参数

**不追求功能完整性，只关注基础结构。**

## ▶️ 如何运行

```bash
cargo run -- hello rust
```

示例输出：
```text
Args: ["target/debug/rust-cli-playground", "hello", "rust"]
```

## 🧠 学习关注点
> 所有应注意的问题均已在代码内采用注释的形式进行了说明