# Agora Rust 快速开始

*[English](README.md) | 中文*

本仓库中包含一些代码示例。您可以选择一些示例进行运行体验。

## 索引

- [Agora基础一对一视频通话示例（命令行版）](https://github.com/AgoraIO-Community/Agora-Rust-QuickStart/tree/main/one_to_one_video_terminal)
- [Agora基础一对一视频通话示例（命令行版附加拷贝动态库脚本）](https://github.com/AgoraIO-Community/Agora-Rust-QuickStart/tree/main/one_to_one_video_terminal_enhanced)
- [Agora基础一对一视频通话示例（图形化界面版）](https://github.com/AgoraIO-Community/Agora-Rust-QuickStart/tree/main/one_to_one_video_gui)

## 运行环境

- rustc
- cargo

若您第一次接触Rust语言，请访问[Rust官网](https://www.rust-lang.org/zh-CN/)以获取更多信息。

## 快速开始

您也可以根据如下教程完成一个简单的示例。

1. 打开终端（macOS）或PowerShell（Windows）并创建一个cargo项目。

   ```bash
   $ cargo new first_agorartc_proj
   $ cd first_agorartc_proj
   ```

2. 在您项目的`Cargo.toml`中加入依赖项。

   ```rust
   [dependencies]
   agorartc-sys = "*"
   ```

3. （macOS）添加一个配置让编译器在编译时将loader路径设置为`@rpath`。

   ```bash
   $ mkdir .cargo
   $ touch .cargo/config.toml
   $ echo "[build]\nrustflags = [\"-C\", \"link-args=-Wl,-rpath,@loader_path\"]" > .cargo/config.toml
   ```

4. 在`src/main.rs`中写下一个简单的示例。

   ```rust
   unsafe extern "C" fn on_error(error: ::std::os::raw::c_int, msg: *const ::std::os::raw::c_char) {
       let msg = CStr::from_ptr(msg);
       println!("On Error: code: {:?}, msg: {:?}", error, msg);
   }
   
   unsafe extern "C" fn on_warning(warn: ::std::os::raw::c_int, msg: *const ::std::os::raw::c_char) {
       if msg != std::ptr::null() {
           let msg = CStr::from_ptr(msg);
           println!("On Warn: code: {:?}, msg: {:?}", warn, msg);
       }
       println!("On Warning: code: {:?}", warn);
   }
   
   unsafe extern "C" fn on_join_channel(
       arg1: *const ::std::os::raw::c_char,
       uid: agorartc_sys::agorartc::agorartcnative::uid_t,
       elapsed: ::std::os::raw::c_int,
   ) {
       println!("onJoinChannel");
   }
   
   unsafe extern "C" fn on_user_joined(uid: agorartc_sys::agorartc::agorartcnative::uid_t, elapsed: ::std::os::raw::c_int) {
       println!("onUserJoined");
   }
   
   fn main() {
       let rtc_engine = &agorartc_sys::agorartc::Agora_Rtc_Engine;
       rtc_engine.add_event_handler(&mut agorartc_sys::agorartc::agorartcnative::RtcEventHandler {
           onJoinChannelSuccess: Some(on_join_channel),
           onReJoinChannelSuccess: None,
           onLeaveChannel: None,
           onConnectionLost: None,
           onConnectionInterrupted: None,
           onRequestToken: None,
           onUserJoined: Some(on_user_joined),
           onUserOffline: None,
           onAudioVolumeIndication: None,
           onUserMuteAudio: None,
           onWarning: Some(on_warning),
           onError: Some(on_error),
           onRtcStats: None,
           onAudioMixingFinished: None,
           onAudioRouteChanged: None,
           onFirstRemoteVideoDecoded: None,
           onVideoSizeChanged: None,
           onClientRoleChanged: None,
           onUserMuteVideo: None,
           onMicrophoneEnabled: None,
           onApiCallExecuted: None,
           onFirstLocalAudioFrame: None,
           onFirstRemoteAudioFrame: None,
           onLastmileQuality: None,
           onAudioQuality: None,
           onStreamInjectedStatus: None,
           onStreamUnpublished: None,
           onStreamPublished: None,
           onStreamMessageError: None,
           onStreamMessage: None,
           onConnectionBanned: None,
           onRemoteVideoTransportStats: None,
           onRemoteAudioTransportStats: None,
           onTranscodingUpdated: None,
           onAudioDeviceVolumeChanged: None,
           onActiveSpeaker: None,
           onMediaEngineStartCallSuccess: None,
           onMediaEngineLoadSuccess: None,
           onConnectionStateChanged: None,
           onRemoteSubscribeFallbackToAudioOnly: None,
           onLocalPublishFallbackToAudioOnly: None,
           onUserEnableLocalVideo: None,
           onRemoteVideoStateChanged: None,
           onVideoDeviceStateChanged: None,
           onAudioEffectFinished: None,
           onRemoteAudioMixingEnd: None,
           onRemoteAudioMixingBegin: None,
           onCameraExposureAreaChanged: None,
           onCameraFocusAreaChanged: None,
           onCameraReady: None,
           onAudioDeviceStateChanged: None,
           onUserEnableVideo: None,
           onFirstRemoteVideoFrame: None,
           onFirstLocalVideoFrame: None,
           onRemoteAudioStats: None,
           onRemoteVideoStats: None,
           onLocalVideoStats: None,
           onNetworkQuality: None,
           onTokenPrivilegeWillExpire: None,
           onVideoStopped: None,
           onAudioMixingStateChanged: None,
           onFirstRemoteAudioDecoded: None,
           onLocalVideoStateChanged: None,
           onNetworkTypeChanged: None,
           onRtmpStreamingStateChanged: None,
           onLastmileProbeResult: None,
           onLocalUserRegistered: None,
           onUserInfoUpdated: None,
           onLocalAudioStateChanged: None,
           onRemoteAudioStateChanged: None,
           onLocalAudioStats: None,
           onChannelMediaRelayStateChanged: None,
           onChannelMediaRelayEvent: None,
           onFacePositionChanged: None,
           onTestEnd: None,
       });
       // If you do not have an App ID, see Appendix.
       rtc_engine.initialize("YOUR-APPID", agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);
       rtc_engine.enable_video();
       rtc_engine.join_channel("", "channel-name", "", 0u32);
       let mut input = String::new();
       std::io::stdin().read_line(&mut input).expect("error: unable to read user input");
       rtc_engine.leave_channel();
       rtc_engine.release(true);
   }
   ```

5. 添加App ID。

   如果您还未获取App ID，请参见附录。请获取一个没有token的App ID。将`rtc_engine.initialize("YOUR-APPID", agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);`中`YOUR-APPID`替换为您的App ID。
   
6. 编译您的项目。

   ```bash
   $ cargo build
   ```

7. 下载所需的SDK。

   - （macOS）在 [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs` 目录下的 `AograRtcEngineKit.framework` 复制到`target/debug`中。
   - （Windows）在 [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip) 下载 SDK。解压缩之后，将 `libs/x86_64` 目录下的 `agora_rtc_sdk.dll` 和 `agora_rtc_sdk.lib` 复制到`target/debug`中。

8. 运行示例。

   ```bash
   $ cargo run
   ```

## 附录

### 创建Agora账户并获取App ID

如果想要使用我们的SDK，您需要先获得一个App ID：

1. 在[agora.io](https://dashboard.agora.io/signin/)中注册一个账号。当您完成注册后，您将被链接至控制台。
2. 在控制台左侧点击**Projects** > **Project List**。
3. 请将您从控制台中获取的App ID保存，您将会在调用SDK时使用。
