use std::sync::mpsc::channel;

use core_foundation::base::TCFType;
use core_media::sample_buffer::{CMSampleBuffer, CMSampleBufferRef};
use core_video::pixel_buffer::CVPixelBuffer;
use dispatch2::{Queue, QueueAttribute};
use libc::size_t;
use objc2::{
    declare_class, extern_methods, msg_send_id, mutability,
    rc::{Allocated, Id},
    runtime::ProtocolObject,
    ClassType, DeclaredClass,
};
use objc2_foundation::{NSArray, NSError, NSObject, NSObjectProtocol};
use screen_capture_kit::{
    shareable_content::SCShareableContent,
    stream::{SCContentFilter, SCStream, SCStreamConfiguration, SCStreamDelegate, SCStreamOutput, SCStreamOutputType},
};

pub struct DelegateIvars {}

declare_class!(
    struct Delegate;

    unsafe impl ClassType for Delegate {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "StreamOutputSampleBufferDelegate";
    }

    impl DeclaredClass for Delegate {
        type Ivars = DelegateIvars;
    }

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl SCStreamOutput for Delegate {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        unsafe fn stream_did_output_sample_buffer(&self, _stream: &SCStream, sample_buffer: CMSampleBufferRef, of_type: SCStreamOutputType) {
            if of_type != SCStreamOutputType::Screen {
                return;
            }
            let sample_buffer = CMSampleBuffer::wrap_under_get_rule(sample_buffer);
            if let Some(image_buffer) = sample_buffer.get_image_buffer() {
                if let Some(pixel_buffer) = image_buffer.downcast::<CVPixelBuffer>() {
                    println!("pixel buffer: {:?}", pixel_buffer);
                }
            }
        }
    }

    unsafe impl SCStreamDelegate for Delegate {
        #[method(stream:didStopWithError:)]
        unsafe fn stream_did_stop_with_error(&self, _stream: &SCStream, error: &NSError) {
            println!("error: {:?}", error);
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
    let (tx, rx) = channel();
    SCShareableContent::get_shareable_content_with_completion_closure(move |shareable_content, error| {
        let ret = shareable_content.ok_or_else(|| error.unwrap());
        tx.send(ret).unwrap();
    });
    let shareable_content = rx.recv().unwrap();
    if let Err(error) = shareable_content {
        println!("error: {:?}", error);
        return;
    }
    let shareable_content = shareable_content.unwrap();
    let displays = shareable_content.displays();
    let display = match displays.first() {
        Some(display) => display,
        None => {
            println!("no display found");
            return;
        }
    };
    let filter = SCContentFilter::init_with_display_exclude_windows(SCContentFilter::alloc(), display, &NSArray::new());
    let configuration: Id<SCStreamConfiguration> = SCStreamConfiguration::new();
    configuration.set_width(display.width() as size_t);
    configuration.set_height(display.height() as size_t);
    let delegate = Delegate::new();
    let stream_error = ProtocolObject::from_ref(&*delegate);
    let stream = SCStream::init_with_filter(SCStream::alloc(), &filter, &configuration, stream_error);
    let queue = Queue::new("com.screen_capture.queue", QueueAttribute::Serial);
    let output = ProtocolObject::from_ref(&*delegate);
    if let Err(ret) = stream.add_stream_output(output, SCStreamOutputType::Screen, &queue) {
        println!("error: {:?}", ret);
        return;
    }
    stream.start_capture(move |result| {
        if let Some(error) = result {
            println!("error: {:?}", error);
        }
    });
    std::thread::sleep(std::time::Duration::from_secs(10));
    stream.stop_capture(move |result| {
        if let Some(error) = result {
            println!("error: {:?}", error);
        }
    });
}
