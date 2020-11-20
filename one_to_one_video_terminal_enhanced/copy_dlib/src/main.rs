//
//  one_to_one_video_terminal
//  Agora-Rust-Video-Tutorial
//
//  Created by Yiqing Huang on 2020/11/20.
//  Copyright © 2020 Agora. All rights reserved.
//

use std::ops::Add;
use std::env;
use walkdir::WalkDir;

fn main() {
    WalkDir::new(env::var("CARGO_HOME").unwrap().add("/registry/src/"))
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|x| if x.depth() == 2 && x.file_name().to_str().unwrap().contains("agora") {
            if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .args(&["/C", "cp ".to_string().add(x.path().to_str().unwrap()).add("/vender/agora_rtc_sdk.dll target/debug").as_str()])
                    .output()
                    .expect("failed");
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg("cp -r ".to_string().add(x.path().to_str().unwrap()).add("/vender/AgoraRtcKit.framework target/debug"))
                    .output()
                    .expect("failed");
            }
        });
}
