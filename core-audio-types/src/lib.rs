#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]

#[macro_use]
extern crate cfg_if;
extern crate core_foundation_sys;
extern crate libc;

pub type OSType = u32;

pub mod base_types;
pub mod session_types;
