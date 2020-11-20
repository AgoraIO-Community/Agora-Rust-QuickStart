# Agora基础一对一视频通话示例（命令行版附加拷贝动态库脚本）

*[English](README.md) | 中文*



### 运行环境

- rustc
- cargo

若您第一次接触Rust语言，请访问[Rust官网](https://www.rust-lang.org/zh-CN/)以获取更多信息。

### 依赖包

- agorartc-sys

## 运行示例

1. 将您的appid添加至`demo/src/main.rs`。

   在`main.rs`中找到如下代码，并将您的appid添加至其中。*（如您还未获取App ID，您可以查看附录。）*

   ```rust
   static YOUR_APP_ID: &str = "";
   ```

2. 编译示例

   确保您在`.../one_to_one_video_terminal_enhanced`路经下运行以下命令而不是`.../one_to_one_video_terminal_enhanced/demo`，这样两个程序可以被一起编译。

   ```bash
   $ cargo build
   ```

3. 运行复制脚本

   ```bash
   $ cargo run --bin copy_dlib
   ```

4. 运行示例。

   ```bash
   $ cargo run --bin demo
   ```

## 附录

### 创建Agora账户并获取App ID

如果想要使用我们的SDK，您需要先获得一个App ID：

1. 在[agora.io](https://dashboard.agora.io/signin/)中注册一个账号。当您完成注册后，您将被链接至控制台。
2. 在控制台左侧点击**Projects** > **Project List**。
3. 请将您从控制台中获取的App ID保存，您将会在运行示例时使用（示例图形化界面中有输入框）。