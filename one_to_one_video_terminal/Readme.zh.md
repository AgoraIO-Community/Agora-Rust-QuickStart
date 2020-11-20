# Agora基础一对一视频通话示例（命令行版）

*[English](README.md) | 中文*



### 运行环境

- rustc
- cargo

若您第一次接触Rust语言，请访问[Rust官网](https://www.rust-lang.org/zh-CN/)以获取更多信息。

### 依赖包

- agorartc-sys

## 运行示例

1. 将您的appid添加至`src/main.rs`。

   在`main.rs`中找到如下代码，并将您的appid添加至其中。*（如您还未获取App ID，您可以查看附录。）*

   ```rust
   static YOUR_APP_ID: &str = "";
   ```

2. 编译示例

   ```bash
   $ cargo build
   ```

3. 下载所需的SDK。

   - （macOS）在 [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs` 目录下的 `AograRtcEngineKit.framework` 复制到`target/debug`中。
   - （Windows）在 [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs/x86_64` 目录下的 `agora_rtc_sdk.dll` 和 `agora_rtc_sdk.lib` 复制到`target/debug`中。

4. 运行示例。

   ```bash
   $ cargo run
   ```

## 附录

### 创建Agora账户并获取App ID

如果想要使用我们的SDK，您需要先获得一个App ID：

1. 在[agora.io](https://dashboard.agora.io/signin/)中注册一个账号。当您完成注册后，您将被链接至控制台。
2. 在控制台左侧点击**Projects** > **Project List**。
3. 请将您从控制台中获取的App ID保存，您将会在运行示例时使用（示例图形化界面中有输入框）。