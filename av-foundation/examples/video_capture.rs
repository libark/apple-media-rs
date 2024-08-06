use av_foundation::{
    capture_device::AVCaptureDevice,
    capture_input::AVCaptureDeviceInput,
    capture_output_base::AVCaptureOutput,
    capture_session::{AVCaptureConnection, AVCaptureSession},
    capture_video_data_output::{AVCaptureVideoDataOutput, AVCaptureVideoDataOutputSampleBufferDelegate},
    media_format::AVMediaTypeVideo,
};
use core_foundation::base::TCFType;
use core_media::sample_buffer::{CMSampleBuffer, CMSampleBufferRef};
use core_video::pixel_buffer::CVPixelBuffer;
use dispatch2::Queue;
use objc2::{
    declare_class, extern_methods, msg_send_id, mutability,
    rc::{Allocated, Id},
    runtime::ProtocolObject,
    ClassType, DeclaredClass,
};
use objc2_foundation::{NSObject, NSObjectProtocol};

pub struct DelegateIvars {}

declare_class!(
    struct Delegate;

    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "OutputSampleBufferDelegate";
    }

    impl DeclaredClass for Delegate {
        type Ivars = DelegateIvars;
    }

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl AVCaptureVideoDataOutputSampleBufferDelegate for Delegate {
        #[method(captureOutput:didOutputSampleBuffer:fromConnection:)]
        unsafe fn capture_output_did_output_sample_buffer(
            &self,
            _capture_output: &AVCaptureOutput,
            sample_buffer: CMSampleBufferRef,
            _connection: &AVCaptureConnection,
        ) {
            let sample_buffer = CMSampleBuffer::wrap_under_get_rule(sample_buffer);
            if let Some(image_buffer) = sample_buffer.get_image_buffer() {
                if let Some(pixel_buffer) = image_buffer.downcast::<CVPixelBuffer>() {
                    println!("pixel buffer: {:?}", pixel_buffer);
                }
            }
        }
    }

    unsafe impl Delegate {
        #[method_id(init)]
        fn init(this: Allocated<Self>) -> Option<Id<Self>> {
            let this = this.set_ivars(DelegateIvars {});
            unsafe { msg_send_id![super(this), init] }
        }
    }
);

extern_methods!(
    unsafe impl Delegate {
        #[method_id(new)]
        pub fn new() -> Id<Self>;
    }
);

fn main() {
    let session = AVCaptureSession::new();
    let device: Id<AVCaptureDevice> = unsafe { AVCaptureDevice::default_device_with_media_type(AVMediaTypeVideo).unwrap() };
    let input = AVCaptureDeviceInput::from_device(&device).unwrap();
    let output = AVCaptureVideoDataOutput::new();
    let delegate = Delegate::new();
    let delegate = ProtocolObject::from_ref(&*delegate);
    let queue = Queue::new("com.video_capture.queue", dispatch2::QueueAttribute::Serial);
    output.set_sample_buffer_delegate(delegate, &queue);
    output.set_always_discards_late_video_frames(true);

    session.begin_configuration();
    session.add_input(&input);
    session.add_output(&output);
    session.commit_configuration();

    session.start_running();
    std::thread::sleep(std::time::Duration::from_secs(10));
    session.stop_running();
}
