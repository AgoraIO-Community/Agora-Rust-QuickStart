#![allow(non_upper_case_globals)]

use std::io;
use std::ffi::CStr;

static YOUR_APP_ID: &str = "";

static RTC_ENGINE: &agorartc_sys::agorartc::Agora_Rtc_Engine = &agorartc_sys::agorartc::Agora_Rtc_Engine;

unsafe extern "C" fn on_api_call_executed(
    err: ::std::os::raw::c_int,
    api: *const ::std::os::raw::c_char,
    result: *const ::std::os::raw::c_char,
) {
    let api_ = CStr::from_ptr(api);
    let result_ = CStr::from_ptr(result);
    println!("API Excuted: err {:?}, api {:?}, result {:?}", err, api_, result_);
}


fn main() {
    println!("rtcEngine: {:?}", RTC_ENGINE);
    println!("begin");
    let mut handler = agorartc_sys::agorartc::agorartcnative::RtcEventHandler {
        onJoinChannelSuccess: None,
        onReJoinChannelSuccess: None,
        onLeaveChannel: None,
        onConnectionLost: None,
        onConnectionInterrupted: None,
        onRequestToken: None,
        onUserJoined: None,
        onUserOffline: None,
        onAudioVolumeIndication: None,
        onUserMuteAudio: None,
        onWarning: None,
        onError: None,
        onRtcStats: None,
        onAudioMixingFinished: None,
        onAudioRouteChanged: None,
        onFirstRemoteVideoDecoded: None,
        onVideoSizeChanged: None,
        onClientRoleChanged: None,
        onUserMuteVideo: None,
        onMicrophoneEnabled: None,
        onApiCallExecuted: Some(on_api_call_executed),
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
    };
    println!("begin 2");
    RTC_ENGINE.add_event_handler(&mut handler as *mut _);
    RTC_ENGINE.initialize(YOUR_APP_ID, agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);
    RTC_ENGINE.enable_video();
    {
        let video_device_manager = RTC_ENGINE.create_video_device_manager();
        let count = video_device_manager.get_device_count();
        println!("video device count: {}", count);

        let (name, id, ret) = video_device_manager.get_device(0);
        println!("device: {}, {}, {}", name, id, ret);
    }
    RTC_ENGINE.join_channel("", "123", "", 0u32);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    println!("{}", input);
    RTC_ENGINE.leave_channel();
    RTC_ENGINE.release(true);
}