use core_foundation::base::TCFType;
use core_media::{
    format_description::{CMFormatDescription, CMFormatDescriptionRef},
    time::CMTime,
};
use objc2::{extern_class, msg_send, msg_send_id, mutability::InteriorMutable, rc::Id, ClassType};
use objc2_foundation::{NSArray, NSError, NSInteger, NSObject, NSObjectProtocol, NSString};

use crate::{capture_session_preset::AVCaptureSessionPreset, media_format::AVMediaType};

extern "C" {
    pub static AVCaptureDeviceWasConnectedNotification: &'static NSString;
    pub static AVCaptureDeviceWasDisconnectedNotification: &'static NSString;
    pub static AVCaptureDeviceSubjectAreaDidChangeNotification: &'static NSString;
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDevice;

    unsafe impl ClassType for AVCaptureDevice {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDevice {}

impl AVCaptureDevice {
    pub fn devices() -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), devices] }
    }

    pub fn devices_with_media_type(media_type: &AVMediaType) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), devicesWithMediaType: media_type] }
    }

    pub fn default_device_with_media_type(media_type: &AVMediaType) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), defaultDeviceWithMediaType: media_type] }
    }

    pub fn device_with_unique_id(unique_id: &NSString) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), deviceWithUniqueID: unique_id] }
    }

    pub fn unique_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, uniqueID] }
    }

    pub fn model_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, modelID] }
    }

    pub fn localized_name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, localizedName] }
    }

    pub fn manufacturer(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, manufacturer] }
    }

    #[cfg(target_os = "macos")]
    pub fn transport_type(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, transportType] }
    }

    pub fn has_media_type(&self, media_type: &AVMediaType) -> bool {
        unsafe { msg_send![self, hasMediaType: media_type] }
    }

    pub fn lock_for_configuration(&self) -> Result<bool, Id<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: bool = unsafe { msg_send![self, lockForConfiguration: &mut error] };
        if result {
            Ok(result)
        } else {
            Err(unsafe { Id::retain(error).unwrap() })
        }
    }

    pub fn unlock_for_configuration(&self) {
        unsafe { msg_send![self, unlockForConfiguration] }
    }

    pub fn supports_av_capture_session_preset(&self, preset: &AVCaptureSessionPreset) -> bool {
        unsafe { msg_send![self, supportsAVCaptureSessionPreset: preset] }
    }

    pub fn is_connected(&self) -> bool {
        unsafe { msg_send![self, isConnected] }
    }

    #[cfg(target_os = "macos")]
    pub fn is_in_use_by_another_application(&self) -> bool {
        unsafe { msg_send![self, isInUseByAnotherApplication] }
    }

    pub fn is_suspended(&self) -> bool {
        unsafe { msg_send![self, isSuspended] }
    }

    #[cfg(target_os = "macos")]
    pub fn linked_devices(&self) -> Id<NSArray<AVCaptureDevice>> {
        unsafe { msg_send_id![self, linkedDevices] }
    }

    pub fn formats(&self) -> Id<NSArray<AVCaptureDeviceFormat>> {
        unsafe { msg_send_id![self, formats] }
    }

    pub fn get_active_format(&self) -> Id<AVCaptureDeviceFormat> {
        unsafe { msg_send_id![self, activeFormat] }
    }

    pub fn set_active_format(&self, format: &AVCaptureDeviceFormat) {
        unsafe { msg_send![self, setActiveFormat: format] }
    }

    pub fn get_active_video_min_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeVideoMinFrameDuration] }
    }

    pub fn set_active_video_min_frame_duration(&self, duration: CMTime) {
        unsafe { msg_send![self, setActiveVideoMinFrameDuration: duration] }
    }

    pub fn get_active_video_max_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeVideoMaxFrameDuration] }
    }

    pub fn set_active_video_max_frame_duration(&self, duration: CMTime) {
        unsafe { msg_send![self, setActiveVideoMaxFrameDuration: duration] }
    }

    pub fn input_sources(&self) -> Id<NSArray<AVCaptureDeviceInputSource>> {
        unsafe { msg_send_id![self, inputSources] }
    }

    pub fn get_active_input_source(&self) -> Id<AVCaptureDeviceInputSource> {
        unsafe { msg_send_id![self, activeInputSource] }
    }

    pub fn set_active_input_source(&self, input_source: &AVCaptureDeviceInputSource) {
        unsafe { msg_send![self, setActiveInputSource: input_source] }
    }
}

pub type AVCaptureDevicePosition = NSInteger;

pub const AVCaptureDevicePositionUnspecified: AVCaptureDevicePosition = 0;
pub const AVCaptureDevicePositionBack: AVCaptureDevicePosition = 1;
pub const AVCaptureDevicePositionFront: AVCaptureDevicePosition = 2;

impl AVCaptureDevice {
    pub fn position(&self) -> AVCaptureDevicePosition {
        unsafe { msg_send![self, position] }
    }
}

pub type AVCaptureDeviceType = NSString;

extern "C" {
    pub static AVCaptureDeviceTypeExternal: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeMicrophone: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDualCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTripleCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeContinuityCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "macos")]
    pub static AVCaptureDeviceTypeDeskViewCamera: &'static AVCaptureDeviceType;
    #[cfg(target_os = "macos")]
    pub static AVCaptureDeviceTypeExternalUnknown: &'static AVCaptureDeviceType;
    #[cfg(target_os = "ios")]
    pub static AVCaptureDeviceTypeBuiltInDuoCamera: &'static AVCaptureDeviceType;
    pub static AVCaptureDeviceTypeBuiltInMicrophone: &'static AVCaptureDeviceType;
}

impl AVCaptureDevice {
    pub fn device_type(&self) -> Id<AVCaptureDeviceType> {
        unsafe { msg_send_id![self, deviceType] }
    }

    pub fn default_device_with_device_type(
        device_type: &AVCaptureDeviceType,
        media_type: &AVMediaType,
        position: AVCaptureDevicePosition,
    ) -> Option<Id<AVCaptureDevice>> {
        unsafe { msg_send_id![AVCaptureDevice::class(), defaultDeviceWithDeviceType: device_type mediaType: media_type position: position] }
    }
}

impl AVCaptureDevice {
    #[cfg(target_os = "ios")]
    pub fn automatically_adjusts_video_hdr_enabled(&self) -> bool {
        unsafe { msg_send![self, automaticallyAdjustsVideoHDREnabled] }
    }

    #[cfg(target_os = "ios")]
    pub fn is_video_hdr_enabled(&self) -> bool {
        unsafe { msg_send![self, isVideoHDREnabled] }
    }
}

pub type AVCaptureColorSpace = NSInteger;

pub const AVCaptureColorSpace_sRGB: AVCaptureColorSpace = 0;
pub const AVCaptureColorSpace_P3_D65: AVCaptureColorSpace = 1;
pub const AVCaptureColorSpace_HLG_BT2020: AVCaptureColorSpace = 2;
pub const AVCaptureColorSpace_AppleLog: AVCaptureColorSpace = 3;

impl AVCaptureDevice {
    pub fn active_color_space(&self) -> AVCaptureColorSpace {
        unsafe { msg_send![self, activeColorSpace] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVFrameRateRange;

    unsafe impl ClassType for AVFrameRateRange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVFrameRateRange {}

impl AVFrameRateRange {
    pub fn min_frame_rate(&self) -> f64 {
        unsafe { msg_send![self, minFrameRate] }
    }

    pub fn max_frame_rate(&self) -> f64 {
        unsafe { msg_send![self, maxFrameRate] }
    }

    pub fn min_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, minFrameDuration] }
    }

    pub fn max_frame_duration(&self) -> CMTime {
        unsafe { msg_send![self, maxFrameDuration] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceFormat;

    unsafe impl ClassType for AVCaptureDeviceFormat {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceFormat {}

impl AVCaptureDeviceFormat {
    pub fn media_type(&self) -> Id<AVMediaType> {
        unsafe { msg_send_id![self, mediaType] }
    }

    pub fn format_description(&self) -> CMFormatDescription {
        unsafe {
            let format_description: CMFormatDescriptionRef = msg_send![self, formatDescription];
            CMFormatDescription::wrap_under_get_rule(format_description)
        }
    }

    pub fn video_supported_frame_rate_ranges(&self) -> Id<NSArray<AVFrameRateRange>> {
        unsafe { msg_send_id![self, videoSupportedFrameRateRanges] }
    }

    pub fn video_field_of_view(&self) -> f64 {
        unsafe { msg_send![self, videoFieldOfView] }
    }

    pub fn is_video_binned(&self) -> bool {
        unsafe { msg_send![self, isVideoBinned] }
    }

    pub fn is_video_hdr_supported(&self) -> bool {
        unsafe { msg_send![self, isVideoHDRSupported] }
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct AVCaptureDeviceInputSource;

    unsafe impl ClassType for AVCaptureDeviceInputSource {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

unsafe impl NSObjectProtocol for AVCaptureDeviceInputSource {}

impl AVCaptureDeviceInputSource {
    pub fn input_source_id(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, inputSourceID] }
    }

    pub fn localized_name(&self) -> Id<NSString> {
        unsafe { msg_send_id![self, localizedName] }
    }
}
