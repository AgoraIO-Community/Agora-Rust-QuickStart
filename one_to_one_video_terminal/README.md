# Agora Basic One-to-One Video Call Demo (Terminal Version)

*[中文](Readme.zh.md) | English*

This is a Rust demo for Agora 1-to-1 video call in Terminal.

### Requirements

- rustc
- cargo

If you are not familiar with Rust, please visit [Rust Programming Language](https://www.rust-lang.org/) for more infomation.

### Dependencies

- agorartc-sys

## Run Demo

1. Add your appid to `src/main.rs`.

   Find the following code in `main.rs` and add your appid. *(If you do not have an App ID, see Appendix.)*

   ```rust
   static YOUR_APP_ID: &str = "";
   ```

2. Build demo.

   ```bash
   $ cargo build
   ```

3. Download the required SDK.

   - (macOS) Download SDK [Agora Video SDK for macOS](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Mac_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `AograRtcEngineKit.framework` from `libs` folder into `target/debug` folder.
   - (Windows) Download SDK [Agora Video SDK for Windows](https://download.agora.io/sdk/release/Agora_Native_SDK_for_Windows_v3_1_2_FULL.zip). Unzip the downloaded SDK package and copy the `agora_rtc_sdk.dll` and `agora_rtc_sdk.lib` files from `libs/x86_64` into `target/debug` folder.

4. Run demo.

   ```bash
   $ cargo run
   ```

## Appendix

### Create an Account and Obtain an App ID

To use our SDK, you must obtain an app ID: 

1. Create a developer account at [agora.io](https://dashboard.agora.io/signin/). Once you finish the sign-up process, you are redirected to the dashboard.
2. Navigate in the dashboard tree on the left to **Projects** > **Project List**.
3. Copy the app ID that you obtained from the dashboard into a text file. You will use it when you run demo (there is an input box in our GUI demo).