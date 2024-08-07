use av_foundation::{
    capture_device::{
        AVCaptureDevice, AVCaptureExposureModeCustom, AVCaptureFocusModeAutoFocus, AVCaptureWhiteBalanceModeAutoWhiteBalance
    },
    media_format::AVMediaTypeVideo,
};
use objc2::rc::Id;
use objc2_foundation::CGPoint;

fn main() {
    let device: Id<AVCaptureDevice> =
        unsafe { AVCaptureDevice::default_device_with_media_type(AVMediaTypeVideo).unwrap() };
    println!("device: {:?}", device);
    println!("device type: {:?}", device.device_type());
    println!("device position: {:?}", device.position());

    let exposure_supported = device.is_exposure_mode_supported(AVCaptureExposureModeCustom);
    println!("device exposure mode supported: {:?}", exposure_supported);

    println!(
        "device exposure point of interest supported: {:?}",
        device.is_exposure_point_of_interest_supported()
    );
    if device.is_exposure_point_of_interest_supported() {
        println!(
            "device exposure point of interest: {:?}",
            device.exposure_point_of_interest()
        );

        device.set_exposure_point_of_interest(CGPoint::new(0.5, 0.5));
    }

    let focus_supported = device.is_focus_mode_supported(AVCaptureFocusModeAutoFocus);
    println!("device focus mode supported: {:?}", focus_supported);
    if focus_supported {
        println!("device focus mode: {:?}", device.focus_mode());
        device.set_focus_mode(AVCaptureFocusModeAutoFocus);
    }

    let white_balance_supported =
        device.is_white_balance_mode_supported(AVCaptureWhiteBalanceModeAutoWhiteBalance);
    println!(
        "device white balance mode supported: {:?}",
        white_balance_supported
    );
    if white_balance_supported {
        println!(
            "device white balance mode: {:?}",
            device.white_balance_mode()
        );
        device.set_white_balance_mode(AVCaptureWhiteBalanceModeAutoWhiteBalance);
    }
}