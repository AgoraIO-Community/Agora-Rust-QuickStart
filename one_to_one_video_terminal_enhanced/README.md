# Agora Basic One-to-One Video Call Demo (Terminal Version with Dynamic Library Copy Program)

*[中文](Readme.zh.md) | English*

This is a Rust demo for Agora 1-to-1 video call in Terminal including a dynamic library copy program which can automatically find the required Agora dynamic library in your system and add it to the right path.

### Requirements

- rustc
- cargo

If you are not familiar with Rust, please visit [Rust Programming Language](https://www.rust-lang.org/) for more infomation.

### Dependencies

- agorartc-sys

## Run Demo

1. Add your appid to `demo/src/main.rs`.

   Find the following code in `main.rs` and add your appid. *(If you do not have an App ID, see Appendix.)*

   ```rust
   static YOUR_APP_ID: &str = "";
   ```

2. Build demo.

   Make sure that you run the following instruction under `.../one_to_one_video_terminal_enhanced` path instead of `.../one_to_one_video_terminal_enhanced/demo` so that both programs can be compiled together.

   ```bash
   $ cargo build
   ```

3. Run copy program.

   ```bash
   $ cargo run --bin copy_dlib
   ```

4. Run demo.

   ```bash
   $ cargo run --bin demo
   ```

## Appendix

### Create an Account and Obtain an App ID

To use our SDK, you must obtain an app ID: 

1. Create a developer account at [agora.io](https://dashboard.agora.io/signin/). Once you finish the sign-up process, you are redirected to the dashboard.
2. Navigate in the dashboard tree on the left to **Projects** > **Project List**.
3. Copy the app ID that you obtained from the dashboard into a text file. You will use it when you run demo (there is an input box in our GUI demo).