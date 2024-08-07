use av_foundation::{
    capture_device::{
        AVCaptureDevicePositionUnspecified, AVCaptureDeviceTypeBuiltInWideAngleCamera,
    },
    capture_device_discovery_session::AVCaptureDeviceDiscoverySession,
    media_format::AVMediaTypeVideo,
};
use objc2_foundation::NSArray;

fn main() {
    let empty_array = NSArray::new();
    let device_types = unsafe { empty_array.arrayByAddingObject(AVCaptureDeviceTypeBuiltInWideAngleCamera) };
    let discovery_session = unsafe {
        AVCaptureDeviceDiscoverySession::discovery_session_with_device_types(
            &device_types,
            AVMediaTypeVideo,
            AVCaptureDevicePositionUnspecified,
        )
    };
    let devices = discovery_session.devices();
    devices.iter().for_each(|device| {
        println!("device: {:?}", device);
    });
}