use winapi::{GUID, UINT};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Format {
    Unknown = 0,
    R32G32B32A32Typeless = 1,
    R32G32B32A32Float = 2,
    R32G32B32A32Uint = 3,
    R32G32B32A32Sint = 4,
    R32G32B32Typeless = 5,
    R32G32B32Float = 6,
    R32G32B32Uint = 7,
    R32G32B32Sint = 8,
    R16G16B16A16Typeless = 9,
    R16G16B16A16Float = 10,
    R16G16B16A16Unorm = 11,
    R16G16B16A16Uint = 12,
    R16G16B16A16Snorm = 13,
    R16G16B16A16Sint = 14,
    R32G32Typeless = 15,
    R32G32Float = 16,
    R32G32Uint = 17,
    R32G32Sint = 18,
    R32G8x24Typeless = 19,
    D32FloatS8x24Uint = 20,
    R32FloatX8x24Typeless = 21,
    X32TypelessG8x24Uint = 22,
    R10G10B10A2Typeless = 23,
    R10G10B10A2Unorm = 24,
    R10G10B10A2Uint = 25,
    R11G11B10Float = 26,
    R8G8B8A8Typeless = 27,
    R8G8B8A8Unorm = 28,
    R8G8B8A8UnormSRGB = 29,
    R8G8B8A8Uint = 30,
    R8G8B8A8Snorm = 31,
    R8G8B8A8Sint = 32,
    R16G16Typeless = 33,
    R16G16Float = 34,
    R16G16Unorm = 35,
    R16G16Uint = 36,
    R16G16Snorm = 37,
    R16G16Sint = 38,
    R32Typeless = 39,
    D32Float = 40,
    R32Float = 41,
    R32Uint = 42,
    R32Sint = 43,
    R24G8Typeless = 44,
    D24UnormS8Uint = 45,
    R24UnormX8Typeless = 46,
    X24TypelessG8Uint = 47,
    R8G8Typeless = 48,
    R8G8Unorm = 49,
    R8G8Uint = 50,
    R8G8Snorm = 51,
    R8G8Sint = 52,
    R16Typeless = 53,
    R16Float = 54,
    D16Unorm = 55,
    R16Unorm = 56,
    R16Uint = 57,
    R16Snorm = 58,
    R16Sint = 59,
    R8Typeless = 60,
    R8Unorm = 61,
    R8Uint = 62,
    R8Snorm = 63,
    R8Sint = 64,
    A8Unorm = 65,
    R1Unorm = 66,
    R9G9B9E5SharedExp = 67,
    R8G8B8G8Unorm = 68,
    G8R8G8B8Unorm = 69,
    BC1Typeless = 70,
    BC1Unorm = 71,
    BC1UnormSRGB = 72,
    BC2Typeless = 73,
    BC2Unorm = 74,
    BC2UnormSRGB = 75,
    BC3Typeless = 76,
    BC3Unorm = 77,
    BC3UnormSRGB = 78,
    BC4Typeless = 79,
    BC4Unorm = 80,
    BC4Snorm = 81,
    BC5Typeless = 82,
    BC5Unorm = 83,
    BC5Snorm = 84,
    B5G6R5Unorm = 85,
    B5G5R5A1Unorm = 86,
    B8G8R8A8Unorm = 87,
    B8G8R8X8Unorm = 88,
    R10G10B10XRBiasA2Unorm = 89,
    B8G8R8A8Typeless = 90,
    B8G8R8A8UnormSRGB = 91,
    B8G8R8X8Typeless = 92,
    B8G8R8X8UnormSRGB = 93,
    BC6HTypeless = 94,
    BC6HUf16 = 95,
    BC6HSf16 = 96,
    BC7Typeless = 97,
    BC7Unorm = 98,
    BC7UnormSRGB = 99,
    AYUV = 100,
    Y410 = 101,
    Y416 = 102,
    NV12 = 103,
    P010 = 104,
    P016 = 105,
    YUV420Opaque = 106,
    YUY2 = 107,
    Y210 = 108,
    Y216 = 109,
    NV11 = 110,
    AI44 = 111,
    IA44 = 112,
    P8 = 113,
    A8P8 = 114,
    B4G4R4A4Unorm = 115,
    #[cfg(feature = "dxgi1_4")] P208 = 130,
    #[cfg(feature = "dxgi1_4")] V208 = 131,
    #[cfg(feature = "dxgi1_4")] V408 = 132,
}

impl Default for Format {
    fn default() -> Format { Format::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScanlineOrder {
    Unspecified = 0,
    Progressive = 1,
    UpperFieldFirst = 2,
    LowerFieldFirst = 3
}

impl Default for ScanlineOrder {
    fn default() -> ScanlineOrder { ScanlineOrder::Unspecified }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalingMode {
    Unspecified = 0,
    Centered = 1,
    Stretched = 2
}

impl Default for ScalingMode {
    fn default() -> ScalingMode { ScalingMode::Unspecified }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RotationMode {
    Unspecified = 0,
    Identity = 1,
    Rotate90 = 2,
    Rotate180 = 3,
    Rotate270 = 4
}

impl Default for RotationMode {
    fn default() -> RotationMode { RotationMode::Unspecified }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CPUAccessFlags {
    None = 0,
    Dynamic = 1,
    ReadWrite = 2,
    Scratch = 3
}

impl Default for CPUAccessFlags {
    fn default() -> CPUAccessFlags { CPUAccessFlags::None }
}

bitflags! {
    #[repr(C)]
    flags UsageFlags: UINT {
        const USAGE_SHADER_INPUT = 1,
        const USAGE_RENDER_TARGET_OUTPUT = 2,
        const USAGE_BACK_BUFFER = 4,
        const USAGE_SHARED = 8,
        const USAGE_READ_ONLY = 16,
        const USAGE_DISCARD_ON_PRESENT = 32,
        const USAGE_UNORDERED_ACCESS = 64
    }
}

impl Default for UsageFlags {
    fn default() -> UsageFlags { UsageFlags::empty() }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Usage {
    bits: u32
}

bitfields! {
    Usage.bits: u32 {
        (0, 4 => struct CPUAccessFlags, cpu_access, set_cpu_access),
        (4, 15 => struct UsageFlags, dxgi_usage, set_dxgi_usage)
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourcePriority {
    Minimum = 0x28000000,
    Low = 0x50000000,
    Normal = 0x78000000,
    High = 0xa0000000,
    Maximum = 0xc8000000
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Residency {
    FullyResident = 1,
    ResidentInSharedMemory = 2,
    EvictedToDisk = 3
}

impl Default for Residency {
    fn default() -> Residency { Residency::FullyResident }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SwapEffect {
    Discard = 0,
    Sequential = 1,
    FlipSequential = 3,
    #[cfg(feature = "dxgi1_4")] FlipDiscard = 4,
}

impl Default for SwapEffect {
    fn default() -> SwapEffect { SwapEffect::Discard }
}

bitflags! {
    #[repr(C)]
    flags SwapChainFlag: UINT {
        const SWAP_CHAIN_FLAG_NONPREROTATED = 1,
        const SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH = 2,
        const SWAP_CHAIN_FLAG_GDI_COMPATIBLE = 4,
        const SWAP_CHAIN_FLAG_RESTRICTED_CONTENT = 8,
        const SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER = 16,
        const SWAP_CHAIN_FLAG_DISPLAY_ONLY = 32,
        const SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT = 64,
        const SWAP_CHAIN_FLAG_FOREGROUND_LAYER = 128,
        const SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO = 256,
        const SWAP_CHAIN_FLAG_YUV_VIDEO = 512,
        #[cfg(feature = "dxgi1_4")]
        const SWAP_CHAIN_FLAG_HW_PROTECTED = 1024,
    }
}

impl Default for SwapChainFlag {
    fn default() -> SwapChainFlag { SwapChainFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags MapFlags: UINT {
        const MAP_READ = 1,
        const MAP_WRITE = 2,
        const MAP_DISCARD = 4
    }
}

impl Default for MapFlags {
    fn default() -> MapFlags { MapFlags::empty() }
}

bitflags! {
    #[repr(C)]
    flags DisplayModeFlag: UINT {
        const ENUM_MODES_INTERLACED = 1,
        const ENUM_MODES_SCALING = 2,
        #[cfg(feature = "dxgi1_2")] const ENUM_MODES_STEREO = 4,
        #[cfg(feature = "dxgi1_2")] const ENUM_MODES_DISABLED_STEREO = 4,
    }
}

impl Default for DisplayModeFlag {
    fn default() -> DisplayModeFlag { DisplayModeFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags PresentFlags: UINT {
        const PRESENT_TEST = 0x01,
        const PRESENT_DO_NOT_SEQUENCE = 0x02,
        const PRESENT_RESTART = 0x04,
        const PRESENT_DO_NOT_WAIT = 0x08,
        const PRESENT_STEREO_PREFER_RIGHT = 0x10,
        const PRESENT_STEREO_TEMPORARY_MONO = 0x20,
        const PRESENT_RESTRICT_TO_OUTPUT = 0x40,
        const PRESENT_USE_DURATION = 0x100
    }
}

impl Default for PresentFlags {
    fn default() -> PresentFlags { PresentFlags::empty() }
}

bitflags! {
    #[repr(C)]
    flags WindowAssociationFlags: UINT {
        const MWA_NO_WINDOW_CHANGES = (1 << 0),
        const MWA_NO_ALT_ENTER = (1 << 1),
        const MWA_NO_PRINT_SCREEN = (1 << 2),
    }
}

impl Default for WindowAssociationFlags {
    fn default() -> WindowAssociationFlags { WindowAssociationFlags::empty() }
}

bitflags! {
    #[repr(C)]
    flags AdapterFlag: UINT {
        const ADAPTER_FLAG_NONE = 0,
        const ADAPTER_FLAG_REMOTE = 1,
        const ADAPTER_FLAG_SOFTWARE = 2
    }
}

impl Default for AdapterFlag {
    fn default() -> AdapterFlag { AdapterFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags DebugRLOFlags: UINT {
        const DEBUG_RLO_SUMMARY = 0x1,
        const DEBUG_RLO_DETAIL = 0x2,
        const DEBUG_RLO_ALL = 0x3
    }
}

guid!(DEBUG_ALL =
    0xe48ae283, 0xda80, 0x490b, 0x87, 0xe6, 0x43, 0xe9, 0xa9, 0xcf, 0xda, 0x08);
guid!(DEBUG_DX =
    0x35cdd7fc, 0x13b2, 0x421d, 0xa5, 0xd7, 0x7e, 0x44, 0x51, 0x28, 0x7d, 0x64);
guid!(DEBUG_DXGI =
    0x25cddaa4, 0xb1c6, 0x47e1, 0xac, 0x3e, 0x98, 0x87, 0x5b, 0x5a, 0x2e, 0x2a);
guid!(DEBUG_APP =
    0x06cd6e01, 0x4219, 0x4ebd, 0x87, 0x09, 0x27, 0xed, 0x23, 0x36, 0x0c, 0x62);

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfoQueueMessageCategory {
    Unknown,
    Miscellaneous,
    Initialization,
    Cleanup,
    Compilation,
    StateCreation,
    StateSetting,
    StateGetting,
    ResourceManipulation,
    Execution,
    Shader
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InfoQueueMessageSeverity {
    Corruption,
    Error,
    Warning,
    Info,
    Message
}
