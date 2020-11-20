# Agora Basic One-to-One Video Call Demo (GUI Version)

*[中文](Readme.zh.md) | English*

This is a Rust demo for Agora 1-to-1 video call with GUI.

### Requirements

- Xcode (macOS)
- Visual Studio 2017+ with C++ (Windows)
- rustc
- cargo

If you are not familiar with Rust, please visit [Rust Programming Language](https://www.rust-lang.org/) for more infomation.

### Dependencies

- agorartc-sys
- qt_widgets
- qt_gui
- qt_core
- qt_widget
- qt_ui_tools
- cpp_core

## Run Demo

1. Download and install Qt.

   For more information, please visit [Qt.io](https://www.qt.io/) and we also recommend you to take a glance at [Setting up for Rust Qt](https://rust-qt.github.io/qt/setting_up/).

2. Build demo.

   ```bash
   $ cargo build
   ```

3. Download the required SDK.

   - (macOS) Download SDK [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `AograRtcEngineKit.framework` from `libs` folder into `target/debug` folder.
   - (Windows) Download SDK [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `agora_rtc_sdk.dll` and `agora_rtc_sdk.lib` files from `libs/x86_64` into `target/debug` folder.

4. (macOS) Change the `@rpath` link to Qt related dynamic lib to the correct path.

   Find your absolute path to QtGui, QtWidgets and QtCore. Run the following instructions:

   ```bash
   $ cd target/debug
   $ install_name_tool -change @rpath/libQt5Gui.5.dylib [your-path-to]/lib/QtGui.framework/QtGui demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Widgets.5.dylib [your-path-to]/lib/QtWidgets.framework/QtWidgets demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Core.5.dylib [your-path-to]/lib/QtCore.framework/QtCore demo_gui_qt
   ```

   For example: 

   ```bash
   $ cd target/debug
   $ install_name_tool -change @rpath/libQt5Gui.5.dylib ~/Qt/5.15.1/clang_64/lib/QtGui.framework/QtGui demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Widgets.5.dylib ~/Qt/5.15.1/clang_64/lib/QtWidgets.framework/QtWidgets demo_gui_qt
   $ install_name_tool -change @rpath/libQt5Core.5.dylib ~/Qt/5.15.1/clang_64/lib/QtCore.framework/QtCore demo_gui_qt
   ```

5. Run demo.

   For Windows users, please run the following instruction in the **VS command prompt**.

   ```rust
   $ cargo run
   ```

## Appendix

### Create an Account and Obtain an App ID

To use our SDK, you must obtain an app ID: 

1. Create a developer account at [agora.io](https://dashboard.agora.io/signin/). Once you finish the sign-up process, you are redirected to the dashboard.
2. Navigate in the dashboard tree on the left to **Projects** > **Project List**.
3. Copy the app ID that you obtained from the dashboard into a text file. You will use it when you run demo (there is an input box in our GUI demo).