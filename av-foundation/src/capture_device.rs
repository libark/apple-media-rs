use core_foundation::base::TCFType;
use core_media::{
    format_description::{CMFormatDescription, CMFormatDescriptionRef},
    time::CMTime,
};
use objc2::{extern_class, msg_send, msg_send_id, mutability::InteriorMutable, rc::Id, ClassType, Encode, Encoding, RefEncode};
use objc2_foundation::{CGFloat, CGPoint, NSArray, NSError, NSInteger, NSObject, NSObjectProtocol, NSString};

use crate::{capture_session_preset::AVCaptureSessionPreset, media_format::AVMediaType};

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub static AVCaptureLensPositionCurrent: &'static f32;
    pub static AVCaptureExposureTargetBiasCurrent: &'static f32;
    pub static AVCaptureExposureDurationCurrent: &'static CMTime;
    pub static AVCaptureISOCurrent: &'static f32;
    pub static AVCaptureMaxAvailableTorchLevel: &'static f32;
}

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

pub type AVCaptureExposureMode = NSInteger;
pub const AVCaptureExposureModeLocked: AVCaptureExposureMode = 0;
pub const AVCaptureExposureModeAutoExpose: AVCaptureExposureMode = 1;
pub const AVCaptureExposureModeContinuousAutoExposure: AVCaptureExposureMode = 2;
pub const AVCaptureExposureModeCustom: AVCaptureExposureMode = 3;

impl AVCaptureDevice {
    pub fn is_exposure_mode_supported(&self, exposure_mode: AVCaptureExposureMode) -> bool {
        unsafe { msg_send![self, isExposureModeSupported: exposure_mode] }
    }

    pub fn exposure_mode(&self) -> AVCaptureExposureMode {
        unsafe { msg_send![self, exposureMode] }
    }

    pub fn set_exposure_mode(&self, mode: AVCaptureExposureMode) -> AVCaptureExposureMode {
        unsafe { msg_send![self, exposureMode: mode] }
    }

    pub fn is_exposure_point_of_interest_supported(&self) -> bool {
        unsafe { msg_send![self, isExposurePointOfInterestSupported] }
    }

    pub fn exposure_point_of_interest(&self) -> CGPoint {
        unsafe { msg_send![self, exposurePointOfInterest] }
    }

    pub fn set_exposure_point_of_interest(&self, point: CGPoint) -> CGPoint {
        unsafe { msg_send![self, exposurePointOfInterest: point] }
    }
}

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, exposureDuration] }
    }

    pub fn active_max_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, activeMaxExposureDuration] }
    }

    pub fn ISO(&self) -> f32 {
        unsafe { msg_send![self, ISO] }
    }

    pub fn lens_aperture(&self) -> f32 {
        unsafe { msg_send![self, lensAperture] }
    }

    pub fn is_face_drive_auto_exposure_enabled(&self) -> bool {
        unsafe { msg_send![self, isFaceDrivenAutoExposureEnabled] }
    }

    pub fn automatically_adjusts_face_driven_auto_exposure_enabled(&self) -> bool {
        unsafe { msg_send![self, automaticallyAdjustsFaceDrivenAutoExposureEnabled] }
    }

    pub fn set_automatically_adjusts_face_driven_auto_exposure_enabled(&self, value: bool) -> bool {
        unsafe { msg_send![self, automaticallyAdjustsFaceDrivenAutoExposureEnabled: value] }
    }

    pub fn exposure_target_offset(&self) -> f32 {
        unsafe { msg_send![self, exposureTargetOffset] }
    }

    pub fn exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, exposureTargetBias] }
    }

    pub fn min_exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, minExposureTargetBias] }
    }

    pub fn max_exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, maxExposureTargetBias] }
    }
}

pub type AVCaptureFocusMode = NSInteger;
pub const AVCaptureFocusModeLocked: AVCaptureFocusMode = 0;
pub const AVCaptureFocusModeAutoFocus: AVCaptureFocusMode = 1;
pub const AVCaptureFocusModeContinuousAutoFocus: AVCaptureFocusMode = 2;

impl AVCaptureDevice {
    pub fn is_focus_mode_supported(&self, focus_mode: AVCaptureFocusMode) -> bool {
        unsafe { msg_send![self, isFocusModeSupported: focus_mode] }
    }

    pub fn focus_mode(&self) -> AVCaptureFocusMode {
        unsafe { msg_send![self, focusMode] }
    }

    pub fn set_focus_mode(&self, focus_mode: AVCaptureFocusMode) -> AVCaptureFocusMode {
        unsafe { msg_send![self, focusMode: focus_mode] }
    }

    pub fn is_focus_point_of_interest_supported(&self) -> bool {
        unsafe { msg_send![self, isFocusPointOfInterestSupported] }
    }

    pub fn focus_point_of_interest(&self) -> CGPoint {
        unsafe { msg_send![self, focusPointOfInterest] }
    }

    pub fn set_focus_point_of_interest(&self, point: CGPoint) -> CGPoint {
        unsafe { msg_send![self, focusPointOfInterest: point] }
    }

    pub fn minimum_focus_distance(&self) -> f32 {
        unsafe { msg_send![self, minimumFocusDistance] }
    }
}

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn is_smooth_auto_focus_supported(&self) -> bool {
        unsafe { msg_send![self, isSmoothAutoFocusSupported] }
    }

    pub fn is_smooth_auto_focus_enabled(&self) -> bool {
        unsafe { msg_send![self, isSmoothAutoFocusEnabled] }
    }

    pub fn is_face_driven_auto_focus_enabled(&self) -> bool {
        unsafe { msg_send![self, isFaceDrivenAutoFocusEnabled] }
    }

    pub fn automatically_adjust_face_drive_auto_focus_enabled(&self) -> bool {
        unsafe { msg_send![self, automaticallyAdjustsFaceDrivenAutoFocusEnabled] }
    }

    pub fn is_locking_focus_with_custom_lens_position_supported(&self) -> bool {
        unsafe { msg_send![self, isLockingFocusWithCustomLensPositionSupported] }
    }

    pub fn lens_position(&self) -> f32 {
        unsafe { msg_send![self, lensPosition] }
    }

    pub fn set_lens_position(&self, lens_position: f32) -> f32 {
        unsafe { msg_send![self, lensPosition: lens_position] }
    }
}

pub type AVCaptureAutoFocusRangeRestriction = NSInteger;
pub const AVCaptureAutoFocusRangeRestrictionNone: AVCaptureAutoFocusRangeRestriction = 0;
pub const AVCaptureAutoFocusRangeRestrictionNear: AVCaptureAutoFocusRangeRestriction = 1;
pub const AVCaptureAutoFocusRangeRestrictionFar: AVCaptureAutoFocusRangeRestriction = 2;

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn is_auto_focus_range_restriction_supported(&self) -> bool {
        unsafe { msg_send![self, isAutoFocusRangeRestrictionSupported] }
    }

    pub fn auto_focus_range_restriction(&self) -> AVCaptureAutoFocusRangeRestriction {
        unsafe { msg_send![self, autoFocusRangeRestriction] }
    }

    pub fn set_auto_focus_range_restriction(
        &self,
        range_restriction: AVCaptureAutoFocusRangeRestriction,
    ) -> AVCaptureAutoFocusRangeRestriction {
        unsafe { msg_send![self, autoFocusRangeRestriction: range_restriction] }
    }
}

pub type AVCaptureWhiteBalanceMode = NSInteger;
pub const AVCaptureWhiteBalanceModeLocked: AVCaptureWhiteBalanceMode = 0;
pub const AVCaptureWhiteBalanceModeAutoWhiteBalance: AVCaptureWhiteBalanceMode = 1;
pub const AVCaptureWhiteBalanceModeContinuousAutoWhiteBalance: AVCaptureWhiteBalanceMode = 2;

#[allow(non_snake_case)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
#[repr(C)]
pub struct AVCaptureWhiteBalanceGains {
    pub blueGain: f32,
    pub greenGain: f32,
    pub redGain: f32,
}

unsafe impl Encode for AVCaptureWhiteBalanceGains {
    const ENCODING: Encoding =
        Encoding::Struct("AVCaptureWhiteBalanceGains", &[CGFloat::ENCODING, CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for AVCaptureWhiteBalanceGains {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl AVCaptureDevice {
    pub fn is_white_balance_mode_supported(
        &self,
        white_balance_mode: AVCaptureWhiteBalanceMode,
    ) -> bool {
        unsafe { msg_send![self, isWhiteBalanceModeSupported: white_balance_mode] }
    }

    pub fn white_balance_mode(&self) -> AVCaptureWhiteBalanceMode {
        unsafe { msg_send![self, whiteBalanceMode] }
    }

    pub fn is_adjusting_white_balance(&self) -> bool {
        unsafe { msg_send![self, isAdjustingWhiteBalance] }
    }

    pub fn set_white_balance_mode(
        &self,
        white_balance_mode: AVCaptureWhiteBalanceMode,
    ) -> AVCaptureWhiteBalanceMode {
        unsafe { msg_send![self, whiteBalanceMode: white_balance_mode] }
    }
}

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn device_white_balance_gains(&self) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, deviceWhiteBalanceGains] }
    }

    pub fn gray_world_device_white_balance_gains(&self) -> AVCaptureWhiteBalanceGains {
        unsafe { msg_send![self, grayWorldDeviceWhiteBalanceGains] }
    }

    pub fn max_white_balance_gains(&self) -> f32 {
        unsafe { msg_send![self, maxWhiteBalanceGain] }
    }
}

pub type AVCaptureFlashMode = NSInteger;
pub const AVCaptureFlashModeOff: AVCaptureFlashMode = 0;
pub const AVCaptureFlashModeOn: AVCaptureFlashMode = 1;
pub const AVCaptureFlashModeAuto: AVCaptureFlashMode = 2;

impl AVCaptureDevice {
    pub fn has_flash(&self) -> bool {
        unsafe { msg_send![self, hasFlash] }
    }

    pub fn is_flash_available(&self) -> bool {
        unsafe { msg_send![self, isFlashAvailable] }
    }
}

pub type AVCaptureTorchMode = NSInteger;
pub const AVCaptureTorchModeOff: AVCaptureTorchMode = 0;
pub const AVCaptureTorchModeOn: AVCaptureTorchMode = 1;
pub const AVCaptureTorchModeAuto: AVCaptureTorchMode = 2;

impl AVCaptureDevice {
    pub fn has_torch(&self) -> bool {
        unsafe { msg_send![self, hasTorch] }
    }

    pub fn is_torch_available(&self) -> bool {
        unsafe { msg_send![self, isTorchAvailable] }
    }

    pub fn is_torch_active(&self) -> bool {
        unsafe { msg_send![self, isTorchActive] }
    }

    pub fn is_torch_mode_supported(&self, torch_mode: AVCaptureTorchMode) -> bool {
        unsafe { msg_send![self, isTorchModeSupported: torch_mode] }
    }

    pub fn torch_mode(&self) -> AVCaptureTorchMode {
        unsafe { msg_send![self, torchMode] }
    }

    pub fn set_torch_mode_on_with_level(&self, level: f32) -> f32 {
        unsafe { msg_send![self, setTorchModeOnWithLevel: level] }
    }

    pub fn set_torch_mode(&self, mode: AVCaptureTorchMode) -> AVCaptureTorchMode {
        unsafe { msg_send![self, setTorchMode: mode] }
    }
}

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn is_low_light_boost_supported(&self) -> bool {
        unsafe { msg_send![self, isLowLightBoostSupported] }
    }

    pub fn is_low_light_boost_enabled(&self) -> bool {
        unsafe { msg_send![self, isLowLightBoostEnabled] }
    }

    pub fn automatically_enables_low_light_boost_when_available(&self) -> bool {
        unsafe { msg_send![self, automaticallyEnablesLowLightBoostWhenAvailable] }
    }

    pub fn set_automatically_enables_low_light_boost_when_available(&self, value: bool) -> bool {
        unsafe { msg_send![self, automaticallyEnablesLowLightBoostWhenAvailable: value] }
    }
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

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn automatically_adjusts_video_hdr_enabled(&self) -> bool {
        unsafe { msg_send![self, automaticallyAdjustsVideoHDREnabled] }
    }

    pub fn is_video_hdr_enabled(&self) -> bool {
        unsafe { msg_send![self, isVideoHDREnabled] }
    }

    pub fn is_global_tone_mapping_enabled(&self) -> bool {
        unsafe { msg_send![self, isGlobalToneMappingEnabled] }
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

impl AVCaptureDevice {
    pub fn display_zoom_factor_multiplier(&self) -> CGFloat {
        unsafe { msg_send![self, displayZoomFactorMultiplier] }
    }
}

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn video_zoom_factor(&self) -> f64 {
        unsafe { msg_send![self, videoZoomFactor] }
    }

    pub fn set_video_zoom_factor(&self, factor: CGFloat) -> CGFloat {
        unsafe { msg_send![self, videoZoomFactor: factor] }
    }

    pub fn is_ramping_video_zoom(&self) -> bool {
        unsafe { msg_send![self, isRampingVideoZoom] }
    }

    pub fn min_available_video_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, minAvailableVideoZoomFactor] }
    }

    pub fn max_available_video_zoom_factor(&self) -> CGFloat {
        unsafe { msg_send![self, maxAvailableVideoZoomFactor] }
    }

    pub fn is_geometric_distortion_correction_supported(&self) -> bool {
        unsafe { msg_send![self, isGeometricDistortionCorrectionSupported] }
    }

    pub fn geometric_distortion_correction_enabled(&self) -> bool {
        unsafe { msg_send![self, geometricDistortionCorrectionEnabled] }
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

#[cfg(target_os = "ios")]
impl AVCaptureDevice {
    pub fn min_exposure_target_bias(&self) -> f32 {
        unsafe { msg_send![self, minExposureTargetBias] }
    }

    pub fn min_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, minExposureDuration] }
    }

    pub fn max_exposure_duration(&self) -> CMTime {
        unsafe { msg_send![self, maxExposureDuration] }
    }

    pub fn is_video_stabilization_supported(&self) -> bool {
        unsafe { msg_send![self, isVideoStabilizationSupported] }
    }

    pub fn is_video_stabilization_enabled(&self) -> bool {
        unsafe { msg_send![self, isVideoStabilizationEnabled] }
    }

    pub fn set_video_stabilization_enabled(&self, value: bool) -> bool {
        unsafe { msg_send![self, setVideoStabilizationEnabled: value] }
    }
}

pub type AVCaptureAutoFocusSystem = NSInteger;
pub const AVCaptureAutoFocusSystemNone: AVCaptureAutoFocusSystem = 0;
pub const AVCaptureAutoFocusSystemPhaseDetection: AVCaptureAutoFocusSystem = 1;
pub const AVCaptureAutoFocusSystemContrastDetection: AVCaptureAutoFocusSystem = 2;

#[cfg(target_os = "ios")]
impl AVCaptureDeviceFormat {
    pub fn auto_focus_system(&self) -> AVCaptureAutoFocusSystem {
        unsafe { msg_send![self, autoFocusSystem] }
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
