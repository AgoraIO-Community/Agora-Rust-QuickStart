# Agora基础一对一视频通话示例（图形化界面版）

*[English](README.md) | 中文*



### 运行环境

- Xcode (macOS)
- Visual Studio 2017+ with C++ (Windows)
- rustc
- cargo

若您第一次接触Rust语言，请访问[Rust官网](https://www.rust-lang.org/zh-CN/)以获取更多信息。

### 依赖包

- agorartc-sys
- qt_widgets
- qt_gui
- qt_core
- qt_widget
- qt_ui_tools
- cpp_core

## 运行示例

1. 下载并安装Qt。

   详情请参考[Qt官网](https://www.qt.io/cn)。推荐您同时阅读[Setting up for Rust Qt](https://rust-qt.github.io/qt/setting_up/)以获得一些帮助。

2. 编译示例

   ```bash
   $ cargo build
   ```

3. 下载所需的SDK。

   - （macOS）在 [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs` 目录下的 `AograRtcEngineKit.framework` 复制到`target/debug`中。
   - （Windows）在 [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs/x86_64` 目录下的 `agora_rtc_sdk.dll` 和 `agora_rtc_sdk.lib` 复制到`target/debug`中。

4. （macOS）将链接到Qt的`@rpath`替换成正确的绝对路径。

   找到QtGui、QtWidgets和QtCore的正确位置和路径。输入以下命令：

   ```bash
   $ cd target/debug
   $ install_name_tool -change @rpath/libQt5Gui.5.dylib [your-path-to]/lib/QtGui.framework/QtGui demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Widgets.5.dylib [your-path-to]/lib/QtWidgets.framework/QtWidgets demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Core.5.dylib [your-path-to]/lib/QtCore.framework/QtCore demo_gui_qt
   ```

   例如：

   ```bash
   $ cd target/debug
   $ install_name_tool -change @rpath/libQt5Gui.5.dylib ~/Qt/5.15.1/clang_64/lib/QtGui.framework/QtGui demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Widgets.5.dylib ~/Qt/5.15.1/clang_64/lib/QtWidgets.framework/QtWidgets demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Core.5.dylib ~/Qt/5.15.1/clang_64/lib/QtCore.framework/QtCore demo_gui_qt
   ```

5. 运行示例。

   Windows用户需要在**VS command prompt**中运行以下命令。

   ```bash
   $ cargo run
   ```

## 附录

### 创建Agora账户并获取App ID

如果想要使用我们的SDK，您需要先获得一个App ID：

1. 在[agora.io](https://dashboard.agora.io/signin/)中注册一个账号。当您完成注册后，您将被链接至控制台。
2. 在控制台左侧点击**Projects** > **Project List**。
3. 请将您从控制台中获取的App ID保存，您将会在运行示例时使用（示例图形化界面中有输入框）。