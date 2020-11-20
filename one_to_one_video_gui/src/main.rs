#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ffi::CStr;

use qt_widgets::*;
use qt_core::*;
use cpp_core::{Ptr, StaticUpcast};
use std::rc::Rc;
use qt_ui_tools::ui_form;

static mut local_win_id: std::os::raw::c_ulonglong = 0;
static mut remote_win_id: std::os::raw::c_ulonglong = 0;
static rtc_engine: &agorartc_sys::agorartc::Agora_Rtc_Engine = &agorartc_sys::agorartc::Agora_Rtc_Engine;

unsafe extern "C" fn on_api_call_executed(
    err: ::std::os::raw::c_int,
    api: *const ::std::os::raw::c_char,
    result: *const ::std::os::raw::c_char,
) {
    let api_ = CStr::from_ptr(api);
    let result_ = CStr::from_ptr(result);
    println!("API Excuted: err {:?}, api {:?}, result {:?}", err, api_, result_);
}

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
    println!("on_join_channel >>>>>>>>>>> {:?}", local_win_id);
    let channle_id: [::std::os::raw::c_char; 65usize] = [0; 65usize];
    rtc_engine.setup_local_video(agorartc_sys::agorartc::agorartcnative::VideoCanvas {
        view: local_win_id as *mut std::ffi::c_void,
        renderMode: 1,
        channelId: channle_id,
        uid: 0,
        priv_: std::ptr::null_mut(),
        mirrorMode: 0,
    });

    rtc_engine.start_preview();
}

unsafe extern "C" fn on_user_joined(uid: agorartc_sys::agorartc::agorartcnative::uid_t, elapsed: ::std::os::raw::c_int) {
    let channle_id: [::std::os::raw::c_char; 65usize] = [0; 65usize];
    rtc_engine.setup_remote_video(agorartc_sys::agorartc::agorartcnative::VideoCanvas {
        view: remote_win_id as *mut std::ffi::c_void,
        renderMode: 1,
        channelId: channle_id,
        uid: uid,
        priv_: std::ptr::null_mut(),
        mirrorMode: 0,
    });
}

#[ui_form("../ui/form.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    joinButton: QPtr<QPushButton>,
    leaveButton: QPtr<QPushButton>,
    gridLayout_2: QPtr<QGridLayout>,
    remote_window: QPtr<QOpenGLWidget>,
    local_window: QPtr<QOpenGLWidget>,
    appIdEdit: QPtr<QLineEdit>,
    channelEdit: QPtr<QLineEdit>,
}

#[derive(Debug)]
struct main_window {
    form: Form,
}

impl StaticUpcast<QObject> for main_window {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl main_window {
    fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(main_window {
                form: Form::load(),
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.form.remote_window.set_attribute_2a(WidgetAttribute::WANativeWindow, true);
        self.form.joinButton.clicked().connect(&self.slot_on_join_button_clicked());
        self.form.leaveButton.clicked().connect(&self.slot_on_leave_button_clicked());
    }

    fn check_app_id(self: &Rc<Self>, app_id: String) -> bool {
        if app_id.len() == 0 {
            return false;
        }
        true
    }

    fn check_channel_name(self: &Rc<Self>, channel_name: String) -> bool {
        let channel_name_slice = channel_name.as_bytes();
        for channel_name_char in channel_name_slice.iter() {
            let n = *channel_name_char;
            if n >= 97 && n <= 122 {} else if n >= 65 && n <= 90 {} else if n >= 48 && n <= 57 {} else if n == 32 || n == 33 || n == 36 || n == 37 || n == 38 || n == 40 || n == 43 ||
                n == 45 || n == 58 || n == 59 || n == 60 || n == 61 || n == 46 || n == 62 ||
                n == 63 || n == 64 || n == 91 || n == 93 || n == 94 || n == 95 || n == 123 ||
                n == 125 || n == 124 || n == 126 {} else {
                return false;
            }
        }
        true
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_join_button_clicked(self: &Rc<Self>) {
        local_win_id = self.form.local_window.effective_win_id();
        remote_win_id = self.form.remote_window.effective_win_id();
        let app_id = CStr::from_ptr((*(*self.form.appIdEdit.text().as_ptr()).to_local8_bit().as_ptr()).data()).to_string_lossy().into_owned();
        let channel_name = CStr::from_ptr((*(*self.form.channelEdit.text().as_ptr()).to_local8_bit().as_ptr()).data()).to_string_lossy().into_owned();
        let msg = QMessageBox::new();
        msg.set_text(&qs("Message"));
        if self.check_app_id(app_id.clone()) == false {
            msg.set_informative_text(&qs("Please input your App ID of your project."));
            msg.exec();
            return;
        }
        if channel_name.len() == 0 {
            msg.set_informative_text(&qs("Please input the channel name."));
            msg.exec();
            return;
        }
        if channel_name.len() > 64 {
            msg.set_informative_text(&qs("The length of the channel name must be less than 64."));
            msg.exec();
            return;
        }
        if self.check_channel_name(channel_name.clone()) == false {
            return;
        }

        let mut handler = agorartc_sys::agorartc::agorartcnative::RtcEventHandler {
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
        rtc_engine.add_event_handler(&mut handler as *mut _);
        rtc_engine.initialize(&app_id, agorartc_sys::agorartc::AREA_CODE::AREA_CODE_GLOBAL);
        rtc_engine.enable_video();
        rtc_engine.join_channel("", &channel_name, "", 0u32);
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_leave_button_clicked(self: &Rc<Self>) {
        rtc_engine.leave_channel();
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_list_selection_changed(self: &Rc<Self>) {}

    #[slot(SlotNoArgs)]
    unsafe fn on_filter_changed(self: &Rc<Self>) {}

    #[slot(SlotNoArgs)]
    unsafe fn on_remove_selected_clicked(self: &Rc<Self>) {}

    #[slot(SlotNoArgs)]
    unsafe fn on_remove_completed_clicked(self: &Rc<Self>) {}

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }
}

fn main() {
    QApplication::init(|_| {
        let todo_widget = main_window::new();
        todo_widget.show();
        unsafe { QApplication::exec() }
    })
}