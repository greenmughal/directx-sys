use winapi::UINT;

bitflags! {
    flags OutDuplPointerShapeType: UINT {
        const OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME = 0x1,
        const OUTDUPL_POINTER_SHAPE_TYPE_COLOR = 0x2,
        const OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR = 0x4
    }
}

impl Default for OutDuplPointerShapeType {
    fn default() -> OutDuplPointerShapeType { OutDuplPointerShapeType::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphaMode {
    Unspecified = 0,
    Premultiplied = 1,
    Straight = 2,
    Ignore = 3
}

impl Default for AlphaMode {
    fn default() -> AlphaMode { AlphaMode::Unspecified }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OfferResourcePriority {
    Low = 1,
    Normal = 2,
    High = 3
}

impl Default for OfferResourcePriority {
    fn default() -> OfferResourcePriority { OfferResourcePriority::Low }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scaling {
    Stretch = 0,
    None = 1,
    AspectRatioStretch = 2
}

impl Default for Scaling {
    fn default() -> Scaling { Scaling::Stretch }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphicsPreemptionGranularity {
    DmaBufferBoundary = 0,
    PrimitiveBoundary = 1,
    TriangleBoundary = 2,
    PixelBoundary = 3,
    InstructionBoundary = 4
}

impl Default for GraphicsPreemptionGranularity {
    fn default() -> GraphicsPreemptionGranularity {
        GraphicsPreemptionGranularity::DmaBufferBoundary
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputePreemptionGranularity {
    DmaBufferBoundary = 0,
    DispatchBoundary = 1,
    ThreadGroupBoundary = 2,
    ThreadBoundary = 3,
    InstructionBoundary = 4
}

impl Default for ComputePreemptionGranularity {
    fn default() -> ComputePreemptionGranularity {
        ComputePreemptionGranularity::DmaBufferBoundary
    }
}
