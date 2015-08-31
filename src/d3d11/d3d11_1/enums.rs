bitflags! {
    #[repr(C)]
    flags CopyFlags: u32 {
        const COPY_NO_OVERWRITE = 0x00000001,
        const COPY_DISCARD      = 0x00000002,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicOp {
    Clear,
    Set,
    Copy,
    CopyInverted,
    NoOp,
    Invert,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Equiv,
    AndReverse,
    AndInverted,
    OrReverse,
    OrInverted,
}

bitflags! {
    #[repr(C)]
    flags CreateDeviceContextStateFlags: u32 {
        const CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED = 0x1,
    }
}

#[cfg(feature = "dxgi1_4")]
bitflags! {
    #[repr(C)]
    flags VideoDecoderCaps: u32 {
        const VIDEO_DECODER_CAPS_DOWNSAMPLE = 0x1,
        const VIDEO_DECODER_CAPS_NON_REAL_TIME = 0x02,
        const VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC = 0x04,
        const VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED = 0x08,
        const VIDEO_DECODER_CAPS_UNSUPPORTED = 0x10,
    }
}

#[cfg(feature = "dxgi1_4")]
bitflags! {
    #[repr(C)]
    flags VideoProcessorBehaviorHints: u32 {
        const VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION = 0x01,
        const VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE = 0x02,
        const VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION = 0x04,
        const VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT = 0x08,
    }
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CryptoSessionStatus {
    Ok,
    KeyLost,
    KeyAndContentLost,
}
