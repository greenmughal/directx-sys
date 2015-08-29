#[repr(C)]
#[derive(Debug)]
pub enum ColorSpaceType {
    RGBFullG22NoneP709,
    RGBFullG10NoneP709,
    RGBStudioG22NoneP709,
    RGBStudioG22NoneP2020,
    Reserved,
    YCbCrFullG22NoneP709X601,
    YCbCrStudioG22LeftP601,
    YCbCrFullG22LeftP601,
    YCbCrStudioG22LeftP709,
    YCbCrFullG22LeftP709,
    YCbCrStudioG22LeftP2020,
    YCbCrFullG22LeftP2020,
    Custom = 0xFFFFFFFF,
}

bitflags! {
    #[repr(C)]
    flags SwapChainColorSpaceSupportFlags: u32 {
        const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT
            = 0x00000001,
        const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT
            = 0x00000002,
    }
}

bitflags! {
    #[repr(C)]
    flags OverlayColorSpaceSupportFlags: u32 {
        const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT = 0x00000001,
    }
}

#[repr(C)]
#[derive(Debug)]
pub enum MemorySegmentGroup {
    Local,
    NonLocal,
}
