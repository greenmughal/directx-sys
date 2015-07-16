use winapi::UINT;

bitflags! {
    #[repr(C)]
    flags CreateFactoryFlags: UINT {
        const DXGI_CREATE_FACTORY_DEBUG = 0x1
    }
}

impl Default for CreateFactoryFlags {
    fn default() -> CreateFactoryFlags { CreateFactoryFlags::empty() }
}

bitflags! {
    flags MultiPlaneOverlayYCbCrFlags: UINT {
        const DXGI_MULTIPLANE_OVERLAY_YCBCR_FLAG_NOMINAL_RANGE = 0x1,
        const DXGI_MULTIPLANE_OVERLAY_YCBCR_FLAG_BT709 = 0x2,
        const DXGI_MULTIPLANE_OVERLAY_YCBCR_FLAG_XVYCC = 0x4
    }
}

impl Default for MultiPlaneOverlayYCbCrFlags {
    fn default() -> MultiPlaneOverlayYCbCrFlags {
        MultiPlaneOverlayYCbCrFlags::empty()
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FramePresentationMode {
    Composed = 0,
    Overlay = 1,
    None = 2
}

impl Default for FramePresentationMode {
    fn default() -> FramePresentationMode { FramePresentationMode::Composed }
}

bitflags! {
    flags OverlaySupportFlag: UINT {
        const DXGI_OVERLAY_SUPPORT_FLAG_DIRECT = 0x1,
        const DXGI_OVERLAY_SUPPORT_FLAG_SCALING = 0x2
    }
}

impl Default for OverlaySupportFlag {
    fn default() -> OverlaySupportFlag { OverlaySupportFlag::empty() }
}
