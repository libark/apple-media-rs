use std::ptr::null_mut;

use core_foundation::base::{kCFAllocatorDefault, CFAllocatorRef, OSStatus, TCFType};

use crate::sync::{CMClock, CMClockRef};

extern "C" {
    pub fn CMAudioClockCreate(allocator: CFAllocatorRef, clockOut: *mut CMClockRef) -> OSStatus;
}

impl CMClock {
    #[inline]
    pub fn new_audio_clock() -> Result<CMClock, OSStatus> {
        unsafe {
            let mut clock = null_mut();
            let status = CMAudioClockCreate(kCFAllocatorDefault, &mut clock);
            if status == 0 {
                Ok(TCFType::wrap_under_create_rule(clock))
            } else {
                Err(status)
            }
        }
    }
}
