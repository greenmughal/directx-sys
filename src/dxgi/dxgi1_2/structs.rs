use std::fmt;

use winapi::{BOOL, LARGE_INTEGER, LUID, POINT, RECT, SIZE_T, UINT, WCHAR};

use super::super::dxgi1_0::*;

use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OutDuplMoveRect {
    pub source_point: POINT,
    pub destination_rect: RECT
}

impl Default for OutDuplMoveRect {
    fn default() -> OutDuplMoveRect { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OutDuplDesc {
    pub mode_desc: ModeDesc,
    pub rotation: RotationMode,
    pub desktop_image_in_system_memory: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OutDuplPointerPosition {
    pub position: POINT,
    pub visible: BOOL
}

impl Default for OutDuplPointerPosition {
    fn default() -> OutDuplPointerPosition { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OutDuplPointerShapeInfo {
    pub shape_type: OutDuplPointerShapeType,
    pub width: UINT,
    pub height: UINT,
    pub pitch: UINT,
    pub hotspot: POINT
}

impl Default for OutDuplPointerShapeInfo {
    fn default() -> OutDuplPointerShapeInfo { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct OutDuplFrameInfo {
    pub last_present_time: LARGE_INTEGER,
    pub last_mouse_update_time: LARGE_INTEGER,
    pub accumulated_frames: UINT,
    pub rects_coalesced: BOOL,
    pub protected_content_masked_out: BOOL,
    pub pointer_position: OutDuplPointerPosition,
    pub total_metadata_buffer_size: UINT,
    pub pointer_shape_buffer_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModeDesc1 {
    pub width: UINT,
    pub height: UINT,
    pub refresh_rate: Rational,
    pub format: Format,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: ScalingMode,
    pub stereo: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SwapChainDesc1 {
    pub width: UINT,
    pub height: UINT,
    pub format: Format,
    pub stereo: BOOL,
    pub sample_desc: SampleDesc,
    pub buffer_usage: Usage,
    pub buffer_count: UINT,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: SwapChainFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SwapChainFullscreenDesc {
    pub refresh_rate: Rational,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: Scaling,
    pub windowed: BOOL
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PresentParameters {
    pub dirty_rects_count: UINT,
    pub dirty_rects: *mut RECT,
    pub scroll_rect: *mut RECT,
    pub scroll_offset: *mut POINT
}

impl Default for PresentParameters {
    fn default() -> PresentParameters { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct AdapterDesc2 {
    pub description: [WCHAR; 128],
    pub vendor_id: UINT,
    pub device_id: UINT,
    pub subsys_id: UINT,
    pub revision: UINT,
    pub dedicated_video_memory: SIZE_T,
    pub dedicated_system_memory: SIZE_T,
    pub shared_system_memory: SIZE_T,
    pub adapter_luid: LUID,
    pub flags: AdapterFlag,
    pub graphics_preemption_granularity: GraphicsPreemptionGranularity,
    pub compute_preemption_granularity: ComputePreemptionGranularity
}

impl Clone for AdapterDesc2 {
    fn clone(&self) -> AdapterDesc2 { *self }
}

impl fmt::Debug for AdapterDesc2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "AdapterDesc2 {{ "));
        try!(write!(f, "description: [{:?}", self.description[0]));
        for i in 1..128 {
            try!(write!(f, ", {:?}", self.description[i]));
        }
        try!(write!(f, "], vendor_id: {:?}, ", self.vendor_id));
        try!(write!(f, "device_id: {:?}, ", self.device_id));
        try!(write!(f, "subsys_id: {:?}, ", self.subsys_id));
        try!(write!(f, "revision: {:?}, ", self.revision));
        try!(write!(f, "dedicated_video_memory: {:?}, ",
                    self.dedicated_video_memory));
        try!(write!(f, "dedicated_system_memory: {:?}, ",
                    self.dedicated_system_memory));
        try!(write!(f, "shared_system_memory: {:?}, ",
                    self.shared_system_memory));
        try!(write!(f, "adapter_luid: {:?}, ", self.adapter_luid));
        try!(write!(f, "flags: {:?}, ", self.flags));
        try!(write!(f, "graphics_preemption_granularity: {:?}, ",
                    self.graphics_preemption_granularity));
        write!(f, "compute_preemption_granularity: {:?} }}",
               self.compute_preemption_granularity)
    }
}

impl Default for AdapterDesc2 {
    fn default() -> AdapterDesc2 { unsafe { ::std::mem::zeroed() } }
}

#[test]
fn check_dxgi1_2_struct_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<AdapterDesc2>(), 320);
        assert_eq!(size_of::<ModeDesc1>(), 32);
        assert_eq!(size_of::<PresentParameters>(), 32);
    } else {
        assert_eq!(size_of::<AdapterDesc2>(), 304);
        assert_eq!(size_of::<ModeDesc1>(), 28);
        assert_eq!(size_of::<PresentParameters>(), 16);
    }

    assert_eq!(size_of::<OutDuplDesc>(), 36);
    assert_eq!(size_of::<OutDuplFrameInfo>(), 48);
    assert_eq!(size_of::<OutDuplMoveRect>(), 24);
    assert_eq!(size_of::<OutDuplPointerPosition>(), 12);
    assert_eq!(size_of::<OutDuplPointerShapeInfo>(), 24);
    assert_eq!(size_of::<SwapChainDesc1>(), 48);
    assert_eq!(size_of::<SwapChainFullscreenDesc>(), 20);
}
