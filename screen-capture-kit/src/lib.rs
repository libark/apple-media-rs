#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

extern crate block2;
#[cfg(feature = "audio")]
extern crate core_audio_types;
extern crate core_foundation;
extern crate core_graphics2 as core_graphics;
extern crate core_media;
#[cfg(feature = "video")]
extern crate core_video;
extern crate dispatch2;
extern crate libc;
#[macro_use]
extern crate objc2;
extern crate objc2_foundation;

#[cfg(target_os = "macos")]
#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

pub mod encode;
pub mod error;
pub mod shareable_content;
pub mod stream;
