use winapi::{FLOAT, LARGE_INTEGER, UINT};

use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix3X2F {
    pub _11: FLOAT,
    pub _12: FLOAT,
    pub _21: FLOAT,
    pub _22: FLOAT,
    pub _31: FLOAT,
    pub _32: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DecodeSwapChainDesc {
    pub flags: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameStatisticsMedia {
    pub present_count: UINT,
    pub present_refresh_count: UINT,
    pub sync_refresh_count: UINT,
    pub sync_qpc_time: LARGE_INTEGER,
    pub sync_gpu_time: LARGE_INTEGER,
    pub composition_mode: FramePresentationMode,
    pub approved_present_duration: UINT
}

#[test]
fn check_dxgi1_3_struct_sizes() {
    use std::mem::size_of;
    assert_eq!(size_of::<Matrix3X2F>(), 24);
    assert_eq!(size_of::<DecodeSwapChainDesc>(), 4);
    assert_eq!(size_of::<FrameStatisticsMedia>(), 40);
}
