use objc2_foundation::NSString;

pub type AVCaptureSessionPreset = NSString;

extern "C" {
    pub static AVCaptureSessionPresetPhoto: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPresetHigh: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPresetMedium: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPresetLow: &'static AVCaptureSessionPreset;
    #[cfg(target_os = "macos")]
    pub static AVCaptureSessionPreset320x240: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPreset352x288: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPreset640x480: &'static AVCaptureSessionPreset;
    #[cfg(target_os = "macos")]
    pub static AVCaptureSessionPreset960x540: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPreset1280x720: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPreset1920x1080: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPreset3840x2160: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPresetiFrame960x540: &'static AVCaptureSessionPreset;
    pub static AVCaptureSessionPresetiFrame1280x720: &'static AVCaptureSessionPreset;
    #[cfg(target_os = "ios")]
    pub static AVCaptureSessionPresetInputPriority: &'static AVCaptureSessionPreset;
}
