use std::os::raw::c_void;

use com_rs::IID;
use winapi::{HRESULT, UINT};

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

#[cfg(test)]
mod tests;

#[link(name = "dxgi")]
extern "stdcall" {
    pub fn CreateDXGIFactory2(
        flags: CreateFactoryFlags,
        iid: &IID,
        factory: *mut *mut c_void) -> HRESULT;
    pub fn DXGIGetDebugInterface1(
        flags: UINT,
        iid: &IID,
        factory: *mut *mut c_void) -> HRESULT;
}
