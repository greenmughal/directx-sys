use std::os::raw::c_void;

use com_rs::{HResult, IID};

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

#[cfg(test)]
mod tests;

extern "stdcall" {
    pub fn CreateDXGIFactory(
        iid: &IID,
        factory: *mut *mut c_void) -> HResult;
    pub fn CreateDXGIFactory1(
        iid: &IID,
        factory: *mut *mut c_void) -> HResult;
}
