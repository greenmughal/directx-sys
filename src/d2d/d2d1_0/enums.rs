use winapi::UINT;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlphaMode {
    Unknown = 0,
    Premultiplied = 1,
    Straight = 2,
    Ignore = 3
}

impl Default for AlphaMode {
    fn default() -> AlphaMode { AlphaMode::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterpolationModeDefinition {
    NearestNeighbor = 0,
    Linear = 1,
    Cubic = 2,
    MultiSampleLinear = 3,
    Anisotropic = 4,
    HighQualityCubic = 5,
    Fant = 6,
    MipmapLinear = 7
}

impl Default for InterpolationModeDefinition {
    fn default() -> InterpolationModeDefinition {
        InterpolationModeDefinition::NearestNeighbor
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Gamma {
    Gamma22 = 0,
    Gamma10 = 1,
}

impl Default for Gamma {
    fn default() -> Gamma { Gamma::Gamma22 }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpacityMaskContent {
    Graphics = 0,
    TextNatural = 1,
    TextGDICompatible = 2
}

impl Default for OpacityMaskContent {
    fn default() -> OpacityMaskContent { OpacityMaskContent::Graphics }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExtendMode {
    Clamp = 0,
    Wrap = 1,
    Mirror = 2
}

impl Default for ExtendMode {
    fn default() -> ExtendMode { ExtendMode::Clamp }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AntialiasMode {
    PerPrimitive = 0,
    Aliased = 1
}

impl Default for AntialiasMode {
    fn default() -> AntialiasMode { AntialiasMode::PerPrimitive }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAntialiasMode {
    Default = 0,
    Cleartype = 1,
    Grayscale = 2,
    Aliased = 3
}

impl Default for TextAntialiasMode {
    fn default() -> TextAntialiasMode { TextAntialiasMode::Default }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BitmapInterpolationMode {
    NearestNeighbor = 0,
    Linear = 1
}

impl Default for BitmapInterpolationMode {
    fn default() -> BitmapInterpolationMode {
        BitmapInterpolationMode::NearestNeighbor
    }
}

bitflags! {
    #[repr(C)]
    flags DrawTextOptions: UINT {
        const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP = 1,
        const D2D1_DRAW_TEXT_OPTIONS_CLIP = 2
    }
}

impl Default for DrawTextOptions {
    fn default() -> DrawTextOptions { DrawTextOptions::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArcSize {
    Small = 0,
    Large = 1
}

impl Default for ArcSize {
    fn default() -> ArcSize { ArcSize::Small }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapStyle {
    Flat = 0,
    Square = 1,
    Round = 2,
    Triangle = 3
}

impl Default for CapStyle {
    fn default() -> CapStyle { CapStyle::Flat }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DashStyle {
    Solid = 0,
    Dash = 1,
    Dot = 2,
    DashDot = 3,
    DashDotDot = 4,
    Custom = 5
}

impl Default for DashStyle {
    fn default() -> DashStyle { DashStyle::Solid }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineJoin {
    Miter = 0,
    Bevel = 1,
    Round = 2,
    MiterOrBevel = 3
}

impl Default for LineJoin {
    fn default() -> LineJoin { LineJoin::Miter }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CombineMode {
    Union = 0,
    Intersect = 1,
    XOR = 2,
    Exclude = 3
}

impl Default for CombineMode {
    fn default() -> CombineMode { CombineMode::Union }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryRelation {
    Unknown = 0,
    Disjoint = 1,
    IsContained = 2,
    Contains = 3,
    Overlap = 4
}

impl Default for GeometryRelation {
    fn default() -> GeometryRelation { GeometryRelation::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometrySimplificationOption {
    CubicsAndLines = 0,
    Lines = 1
}

impl Default for GeometrySimplificationOption {
    fn default() -> GeometrySimplificationOption {
        GeometrySimplificationOption::CubicsAndLines
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FigureBegin {
    Filled = 0,
    Hollow = 1
}

impl Default for FigureBegin {
    fn default() -> FigureBegin { FigureBegin::Filled }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FigureEnd {
    Open = 0,
    Closed = 1
}

impl Default for FigureEnd {
    fn default() -> FigureEnd { FigureEnd::Open }
}

bitflags! {
    #[repr(C)]
    flags PathSegment: UINT {
        const D2D1_PATH_SEGMENT_FORCE_UNSTROKED = 1,
        const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN = 2
    }
}

impl Default for PathSegment {
    fn default() -> PathSegment { PathSegment::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SweepDirection {
    CounterClockwise = 0,
    Clockwise = 1
}

impl Default for SweepDirection {
    fn default() -> SweepDirection { SweepDirection::CounterClockwise }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FillMode {
    Alternate = 0,
    Winding = 1
}

impl Default for FillMode {
    fn default() -> FillMode { FillMode::Alternate }
}

bitflags! {
    #[repr(C)]
    flags LayerOptions: UINT {
        const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE = 1
    }
}

impl Default for LayerOptions {
    fn default() -> LayerOptions { LayerOptions::empty() }
}

bitflags! {
    #[repr(C)]
    flags WindowState: UINT {
        const D2D1_WINDOW_STATE_OCCLUDED = 1
    }
}

impl Default for WindowState {
    fn default() -> WindowState { WindowState::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderTargetType {
    Default = 0,
    Software = 1,
    Hardware = 2
}

impl Default for RenderTargetType {
    fn default() -> RenderTargetType { RenderTargetType::Default }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureLevel {
    Default = 0,
    D3D9 = 0x9100,
    D3D10 = 0xa000
}

impl Default for FeatureLevel {
    fn default() -> FeatureLevel { FeatureLevel::Default }
}

bitflags! {
    #[repr(C)]
    flags RenderTargetUsage: UINT {
        const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING = 1,
        const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE = 2
    }
}

impl Default for RenderTargetUsage {
    fn default() -> RenderTargetUsage { RenderTargetUsage::empty() }
}

bitflags! {
    #[repr(C)]
    flags PresentOptions: UINT {
        const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS = 1,
        const D2D1_PRESENT_OPTIONS_IMMEDIATELY = 2
    }
}

impl Default for PresentOptions {
    fn default() -> PresentOptions { PresentOptions::empty() }
}

bitflags! {
    #[repr(C)]
    flags CompatibleRenderTargetOptions: UINT {
        const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE = 1
    }
}

impl Default for CompatibleRenderTargetOptions {
    fn default() -> CompatibleRenderTargetOptions {
        CompatibleRenderTargetOptions::empty()
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DCInitializeMode {
    Copy = 0,
    Clear = 1
}

impl Default for DCInitializeMode {
    fn default() -> DCInitializeMode { DCInitializeMode::Copy }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DebugLevel {
    None = 0,
    Error = 1,
    Warning = 2,
    Information = 3
}

impl Default for DebugLevel {
    fn default() -> DebugLevel { DebugLevel::None }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactoryType {
    SingleThreaded = 0,
    MultiThreaded = 1
}

impl Default for FactoryType {
    fn default() -> FactoryType { FactoryType::SingleThreaded }
}
