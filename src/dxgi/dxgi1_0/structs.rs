use std::fmt;

use libc::c_char;
use winapi::{BOOL, BYTE, FLOAT, GUID, HANDLE, HMONITOR, HWND, INT,
             LARGE_INTEGER, LUID, RECT, SIZE_T, UINT, WCHAR};

use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGB {
    pub red: FLOAT,
    pub green: FLOAT,
    pub blue: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RGBA {
    pub r: FLOAT,
    pub g: FLOAT,
    pub b: FLOAT,
    pub a: FLOAT
}

#[repr(C)]
#[derive(Copy)]
pub struct GammaControl {
    pub scale: RGB,
    pub offset: RGB,
    pub gamma_curve: [RGB; 1025]
}

impl Clone for GammaControl {
    fn clone(&self) -> GammaControl { *self }
}

impl fmt::Debug for GammaControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "GammaControl {{ "));
        try!(write!(f, "scale: {:?}, offset: {:?}, ", self.scale, self.offset));
        try!(write!(f, "gamma_curve: [{:?}", self.gamma_curve[0]));
        for i in 1..1025 {
            try!(write!(f, ", {:?}", self.gamma_curve[i]));
        }
        write!(f, "]}}")
    }
}

impl Default for GammaControl {
    fn default() -> GammaControl { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct GammaControlCapabilities {
    pub scale_and_offset_supported: BOOL,
    pub max_converted_value: FLOAT,
    pub min_converted_value: FLOAT,
    pub num_gamma_control_points: UINT,
    pub control_point_positions: [FLOAT; 1025]
}

impl Clone for GammaControlCapabilities {
    fn clone(&self) -> GammaControlCapabilities { *self }
}

impl fmt::Debug for GammaControlCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "GammaControlCapabilities {{ "));
        try!(write!(f, "scale_and_offset_supported: {:?}, ",
                    self.scale_and_offset_supported));
        try!(write!(f, "max_converted_value: {:?}, ",
                    self.max_converted_value));
        try!(write!(f, "min_converted_value: {:?}, ",
                    self.min_converted_value));
        try!(write!(f, "num_gamma_control_points: {:?}, ",
                    self.num_gamma_control_points));
        try!(write!(f, "control_point_positions: [{:?}",
                    self.control_point_positions[0]));
        for i in 1..1025 {
            try!(write!(f, ", {:?}", self.control_point_positions[i]));
        }
        write!(f, "] }}")
    }
}

impl Default for GammaControlCapabilities {
    fn default() -> GammaControlCapabilities { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rational {
    pub numerator: UINT,
    pub denominator: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModeDesc {
    pub width: UINT,
    pub height: UINT,
    pub refresh_rate: Rational,
    pub format: Format,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: ScalingMode
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SampleDesc {
    pub count: UINT,
    pub quality: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameStatistics {
    pub present_count: UINT,
    pub present_refresh_count: UINT,
    pub sync_refresh_count: UINT,
    pub sync_qpc_time: LARGE_INTEGER,
    pub sync_gpu_time: LARGE_INTEGER
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MappedRect {
    pub pitch: INT,
    pub bits: *mut BYTE
}

impl Default for MappedRect {
    fn default() -> MappedRect { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct AdapterDesc {
    pub description: [WCHAR; 128],
    pub vendor_id: UINT,
    pub device_id: UINT,
    pub subsys_id: UINT,
    pub revision: UINT,
    pub dedicated_video_memory: SIZE_T,
    pub dedicated_system_memory: SIZE_T,
    pub shared_system_memory: SIZE_T,
    pub adapter_luid: LUID
}

impl Clone for AdapterDesc {
    fn clone(&self) -> AdapterDesc { *self }
}

impl fmt::Debug for AdapterDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "AdapterDesc {{ "));
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
        write!(f, "adapter_luid: {:?} }}", self.adapter_luid)
    }
}

impl Default for AdapterDesc {
    fn default() -> AdapterDesc { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OutputDesc {
    pub device_name: [WCHAR; 32],
    pub desktop_coordinates: RECT,
    pub attached_to_desktop: BOOL,
    pub rotation: RotationMode,
    pub monitor: HMONITOR
}

impl Default for OutputDesc {
    fn default() -> OutputDesc { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedResource {
    pub handle: HANDLE
}

impl Default for SharedResource {
    fn default() -> SharedResource { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SurfaceDesc {
    pub width: UINT,
    pub height: UINT,
    pub format: Format,
    pub sample_desc: SampleDesc
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SwapChainDesc {
    pub buffer_desc: ModeDesc,
    pub sample_desc: SampleDesc,
    pub buffer_usage: Usage,
    pub buffer_count: UINT,
    pub output_window: HWND,
    pub windowed: BOOL,
    pub swap_effect: SwapEffect,
    pub flags: SwapChainFlag
}

impl Default for SwapChainDesc {
    fn default() -> SwapChainDesc { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct AdapterDesc1 {
    pub description: [WCHAR; 128],
    pub vendor_id: UINT,
    pub device_id: UINT,
    pub subsys_id: UINT,
    pub revision: UINT,
    pub dedicated_video_memory: SIZE_T,
    pub dedicated_system_memory: SIZE_T,
    pub shared_system_memory: SIZE_T,
    pub adapter_luid: LUID,
    pub flags: AdapterFlag
}

impl Clone for AdapterDesc1 {
    fn clone(&self) -> AdapterDesc1 { *self }
}

impl fmt::Debug for AdapterDesc1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "AdapterDesc1 {{ "));
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
        write!(f, "flags: {:?} }}", self.flags)
    }
}

impl Default for AdapterDesc1 {
    fn default() -> AdapterDesc1 { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayColorSpace {
    pub primary_coordinates: [[FLOAT; 2]; 8],
    pub white_points: [[FLOAT; 2]; 16]
}

impl Default for DisplayColorSpace {
    fn default() -> DisplayColorSpace {
        DisplayColorSpace {
            primary_coordinates: [[0.0; 2]; 8],
            white_points: [[0.0; 2]; 16]
        }
    }
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct InfoQueueMessage {
    pub producer: GUID,
    pub category: InfoQueueMessageCategory,
    pub severity: InfoQueueMessageSeverity,
    pub id: INT,
    pub description: *const c_char,
    pub description_byte_length: SIZE_T
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct InfoQueueFilterDesc {
    pub num_categories: UINT,
    pub category_list: *mut InfoQueueMessageCategory,
    pub num_severities: UINT,
    pub severity_list: *mut InfoQueueMessageSeverity,
    pub num_ids: UINT,
    pub id_list: *mut INT
}

#[repr(C)]
#[derive(Debug)]
pub struct InfoQueueFilter {
    pub allow_list: InfoQueueFilterDesc,
    pub deny_list: InfoQueueFilterDesc
}


#[test]
fn check_dxgi_struct_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<AdapterDesc>(), 304);
        assert_eq!(size_of::<AdapterDesc1>(), 312);
        assert_eq!(size_of::<InfoQueueMessage>(), 48);
        assert_eq!(size_of::<InfoQueueFilterDesc>(), 48);
        assert_eq!(size_of::<InfoQueueFilter>(), 96);
        assert_eq!(size_of::<MappedRect>(), 16);
        assert_eq!(size_of::<OutputDesc>(), 96);
        assert_eq!(size_of::<SharedResource>(), 8);
        assert_eq!(size_of::<SwapChainDesc>(), 72);
    } else {
        assert_eq!(size_of::<AdapterDesc>(), 292);
        assert_eq!(size_of::<AdapterDesc1>(), 296);
        assert_eq!(size_of::<InfoQueueMessage>(), 36);
        assert_eq!(size_of::<InfoQueueFilterDesc>(), 24);
        assert_eq!(size_of::<InfoQueueFilter>(), 48);
        assert_eq!(size_of::<MappedRect>(), 8);
        assert_eq!(size_of::<OutputDesc>(), 92);
        assert_eq!(size_of::<SharedResource>(), 4);
        assert_eq!(size_of::<SwapChainDesc>(), 60);
    }

    assert_eq!(size_of::<DisplayColorSpace>(), 192);
    assert_eq!(size_of::<FrameStatistics>(), 32);
    assert_eq!(size_of::<GammaControl>(), 12324);
    assert_eq!(size_of::<GammaControlCapabilities>(), 4116);
    assert_eq!(size_of::<ModeDesc>(), 28);
    assert_eq!(size_of::<RGB>(), 12);
    assert_eq!(size_of::<RGBA>(), 16);
    assert_eq!(size_of::<Rational>(), 8);
    assert_eq!(size_of::<SampleDesc>(), 8);
    assert_eq!(size_of::<SurfaceDesc>(), 20);
}