use std::os::raw::c_void;

use winapi::{HRESULT, REFIID};

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
    pub fn CreateDXGIFactory(
        iid: REFIID,
        factory: *mut *mut c_void) -> HRESULT;
    pub fn CreateDXGIFactory1(
        iid: REFIID,
        factory: *mut *mut c_void) -> HRESULT;
}

// #[link(name = "dxgidebug")]
// extern "stdcall" {
//     pub fn DXGIGetDebugInterface(
//         iid: REFIID,
//         debug: *mut *mut c_void) -> HRESULT;
// }
