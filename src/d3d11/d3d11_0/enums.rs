use winapi::{RECT, UINT};

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SRVDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture1DArray = 3,
    Texture2D = 4,
    Texture2DArray = 5,
    Texture2DMS = 6,
    Texture2DMSArray = 7,
    Texture3D = 8,
    TextureCube = 9,
    TextureCubeArray = 10,
    BufferEx = 11
}

impl Default for SRVDimension {
    fn default() -> SRVDimension { SRVDimension::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveTopology {
    Undefined = 0,
    PointList = 1,
    LineList = 2,
    LineStrip = 3,
    TriangleList = 4,
    TriangleStrip = 5,
    LineListAdj = 10,
    LineStripAdj = 11,
    TriangleListAdj = 12,
    TriangleStripAdj = 13,
    PatchList1ControlPoint = 33,
    PatchList2ControlPoints = 34,
    PatchList3ControlPoints = 35,
    PatchList4ControlPoints = 36,
    PatchList5ControlPoints = 37,
    PatchList6ControlPoints = 38,
    PatchList7ControlPoints = 39,
    PatchList8ControlPoints = 40,
    PatchList9ControlPoints = 41,
    PatchList10ControlPoints = 42,
    PatchList11ControlPoints = 43,
    PatchList12ControlPoints = 44,
    PatchList13ControlPoints = 45,
    PatchList14ControlPoints = 46,
    PatchList15ControlPoints = 47,
    PatchList16ControlPoints = 48,
    PatchList17ControlPoints = 49,
    PatchList18ControlPoints = 50,
    PatchList19ControlPoints = 51,
    PatchList20ControlPoints = 52,
    PatchList21ControlPoints = 53,
    PatchList22ControlPoints = 54,
    PatchList23ControlPoints = 55,
    PatchList24ControlPoints = 56,
    PatchList25ControlPoints = 57,
    PatchList26ControlPoints = 58,
    PatchList27ControlPoints = 59,
    PatchList28ControlPoints = 60,
    PatchList29ControlPoints = 61,
    PatchList30ControlPoints = 62,
    PatchList31ControlPoints = 63,
    PatchList32ControlPoints = 64
}

impl Default for PrimitiveTopology {
    fn default() -> PrimitiveTopology { PrimitiveTopology::Undefined }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primitive {
    Undefined = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
    LineAdj = 6,
    TriangleAdj = 7,
    Patch1ControlPoint = 8,
    Patch2ControlPoints = 9,
    Patch3ControlPoints = 10,
    Patch4ControlPoints = 11,
    Patch5ControlPoints = 12,
    Patch6ControlPoints = 13,
    Patch7ControlPoints = 14,
    Patch8ControlPoints = 15,
    Patch9ControlPoints = 16,
    Patch10ControlPoints = 17,
    Patch11ControlPoints = 18,
    Patch12ControlPoints = 19,
    Patch13ControlPoints = 20,
    Patch14ControlPoints = 21,
    Patch15ControlPoints = 22,
    Patch16ControlPoints = 23,
    Patch17ControlPoints = 24,
    Patch18ControlPoints = 25,
    Patch19ControlPoints = 26,
    Patch20ControlPoints = 28,
    Patch21ControlPoints = 29,
    Patch22ControlPoints = 30,
    Patch23ControlPoints = 31,
    Patch24ControlPoints = 32,
    Patch25ControlPoints = 33,
    Patch26ControlPoints = 34,
    Patch27ControlPoints = 35,
    Patch28ControlPoints = 36,
    Patch29ControlPoints = 37,
    Patch30ControlPoints = 38,
    Patch31ControlPoints = 39,
    Patch32ControlPoints = 40,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DriverType {
    Unknown,
    Hardware,
    Reference,
    Null,
    Software,
    WARP
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureLevel {
    Level_9_1 = 0x9100,
    Level_9_2 = 0x9200,
    Level_9_3 = 0x9300,
    Level_10_0 = 0xa000,
    Level_10_1 = 0xa100,
    Level_11_0 = 0xb000,
    Level_11_1 = 0xb100
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputClassification {
    PerVertexData = 0,
    PerInstanceData = 1
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FillMode {
    Wireframe = 2,
    Solid = 3
}

impl Default for FillMode {
    fn default() -> FillMode { FillMode::Solid }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CullMode {
    None = 1,
    Front = 2,
    Back = 3
}

impl Default for CullMode {
    fn default() -> CullMode { CullMode::Back }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture2D = 3,
    Texture3D = 4
}

impl Default for ResourceDimension {
    fn default() -> ResourceDimension { ResourceDimension::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DSVDimension {
    Unknown = 0,
    Texture1D = 1,
    Texture1DArray = 2,
    Texture2D = 3,
    Texture2DArray = 4,
    Texture2DMS = 5,
    Texture2DMSArray = 6,
}

impl Default for DSVDimension {
    fn default() -> DSVDimension { DSVDimension::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RTVDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture1DArray = 3,
    Texture2D = 4,
    Texture2DArray = 5,
    Texture2DMS = 6,
    Texture2DMSArray = 7,
    Texture3D = 8
}

impl Default for RTVDimension {
    fn default() -> RTVDimension { RTVDimension::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UAVDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture1DArray = 3,
    Texture2D = 4,
    Texture2DArray = 5,
    Texture3D = 8
}

impl Default for UAVDimension {
    fn default() -> UAVDimension { UAVDimension::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Usage {
    Default = 0,
    Immutable = 1,
    Dynamic = 2,
    Staging = 3
}

impl Default for Usage {
    fn default() -> Usage { Usage::Default }
}

bitflags! {
    #[repr(C)]
    flags BindFlag: UINT {
        const BIND_VERTEX_BUFFER = 0x00000001,
        const BIND_INDEX_BUFFER = 0x00000002,
        const BIND_CONSTANT_BUFFER = 0x00000004,
        const BIND_SHADER_RESOURCE = 0x00000008,
        const BIND_STREAM_OUTPUT = 0x00000010,
        const BIND_RENDER_TARGET = 0x00000020,
        const BIND_DEPTH_STENCIL = 0x00000040,
        const BIND_UNORDERED_ACCESS = 0x00000080,
        const BIND_DECODER = 0x00000200,
        const BIND_VIDEO_ENCODER = 0x00000400
    }
}

impl Default for BindFlag {
    fn default() -> BindFlag { BindFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags CPUAccessFlag: UINT {
        const CPU_ACCESS_WRITE = 0x00010000,
        const CPU_ACCESS_READ = 0x00020000
    }
}

impl Default for CPUAccessFlag {
    fn default() -> CPUAccessFlag { CPUAccessFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags ResourceMiscFlag: UINT {
        const RESOURCE_MISC_GENERATE_MIPS = 0x00000001,
        const RESOURCE_MISC_SHARED = 0x00000002,
        const RESOURCE_MISC_TEXTURECUBE = 0x00000004,
        const RESOURCE_MISC_DRAWINDIRECT_ARGS = 0x00000010,
        const RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS = 0x00000020,
        const RESOURCE_MISC_BUFFER_STRUCTURED = 0x00000040,
        const RESOURCE_MISC_RESOURCE_CLAMP = 0x00000080,
        const RESOURCE_MISC_SHARED_KEYEDMUTEX = 0x00000100,
        const RESOURCE_MISC_GDI_COMPATIBLE = 0x00000200,
        const RESOURCE_MISC_SHARED_NTHANDLE = 0x00000800,
        const RESOURCE_MISC_RESTRICTED_CONTENT = 0x00001000,
        const RESOURCE_MISC_RESTRICT_SHARED_RESOURCE = 0x00002000,
        const RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER = 0x00004000,
        const RESOURCE_MISC_GUARDED = 0x00008000,
        const RESOURCE_MISC_TILE_POOL = 0x00020000,
        const RESOURCE_MISC_TILED = 0x00040000,
        #[cfg(feature = "d3d11_3")]
        const RESOURCE_MISC_HW_PROTECTED = 0x00080000,
    }
}

impl Default for ResourceMiscFlag {
    fn default() -> ResourceMiscFlag { ResourceMiscFlag::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Map {
    Read = 1,
    Write = 2,
    ReadWrite = 3,
    WriteDiscard = 4,
    WriteNoOverwrite = 5
}

bitflags! {
    #[repr(C)]
    flags MapFlag: UINT {
        const MAP_FLAG_DO_NOT_WAIT = 0x00100000
    }
}

impl Default for MapFlag {
    fn default() -> MapFlag { MapFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags RaiseFlag: UINT {
        const RAISE_FLAG_DRIVER_INTERNAL_ERROR = 0x1
    }
}

impl Default for RaiseFlag {
    fn default() -> RaiseFlag { RaiseFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags ClearFlag: UINT {
        const CLEAR_DEPTH = 0x01,
        const CLEAR_STENCIL = 0x02
    }
}

impl Default for ClearFlag {
    fn default() -> ClearFlag { ClearFlag::empty() }
}

pub type Rect = RECT;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparisonFunc {
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8
}

impl Default for ComparisonFunc {
    fn default() -> ComparisonFunc { ComparisonFunc::Less }
}

bitflags! {
    #[repr(C)]
    flags DepthWriteMask: UINT {
        const DEPTH_WRITE_MASK_ZERO = 0,
        const DEPTH_WRITE_MASK_ALL = 1
    }
}

impl Default for DepthWriteMask {
    fn default() -> DepthWriteMask { DepthWriteMask::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StencilOp {
    Keep = 1,
    Zero = 2,
    Replace = 3,
    IncrSat = 4,
    DecrSat = 5,
    Invert = 6,
    Incr = 7,
    Decr = 8
}

impl Default for StencilOp {
    fn default() -> StencilOp { StencilOp::Keep }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Blend {
    Zero = 1,
    One = 2,
    SrcColor = 3,
    InvSrcColor = 4,
    SrcAlpha = 5,
    InvSrcAlpha = 6,
    DestAlpha = 7,
    InvDestAlpha = 8,
    DestColor = 9,
    InvDestColor = 10,
    SrcAlphaSat = 11,
    BlendFactor = 14,
    InvBlendFactor = 15,
    Src1Color = 16,
    InvSrc1Color = 17,
    Src1Alpha = 18,
    InvSrc1Alpha = 19
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlendOp {
    Add = 1,
    Subtract = 2,
    RevSubtract = 3,
    Min = 4,
    Max = 5
}

bitflags! {
    #[repr(C)]
    flags ColorWriteEnable: UINT {
        const COLOR_WRITE_ENABLE_RED = 1,
        const COLOR_WRITE_ENABLE_GREEN = 2,
        const COLOR_WRITE_ENABLE_BLUE = 4,
        const COLOR_WRITE_ENABLE_ALPHA = 8
    }
}

impl Default for ColorWriteEnable {
    fn default() -> ColorWriteEnable { ColorWriteEnable::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureCubeFace {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5
}

bitflags! {
    #[repr(C)]
    flags BufferExSRVFlag: UINT {
        const BUFFEREX_SRV_FLAG_RAW = 0x00000001
    }
}

impl Default for BufferExSRVFlag {
    fn default() -> BufferExSRVFlag { BufferExSRVFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags DSVFlag: UINT {
        const DSV_READ_ONLY_DEPTH = 0x1,
        const DSV_READ_ONLY_STENCIL = 0x2
    }
}

impl Default for DSVFlag {
    fn default() -> DSVFlag { DSVFlag::empty() }
}

bitflags! {
    #[repr(C)]
    flags BufferUAVFlag: UINT {
        const BUFFER_UAV_FLAG_RAW = 0x00000001,
        const BUFFER_UAV_FLAG_APPEND = 0x00000002,
        const BUFFER_UAV_FLAG_COUNTER = 0x00000004
    }
}

impl Default for BufferUAVFlag {
    fn default() -> BufferUAVFlag { BufferUAVFlag::empty() }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Filter {
    MinMagMipPoint = 0x00000000,
    MinMagPointMipLinear = 0x00000001,
    MinPointMagLinearMipPoint = 0x00000004,
    MinPointMagMipLinear = 0x00000005,
    MinLinearMagMipPoint = 0x00000010,
    MinLinearMagPointMipLinear = 0x00000011,
    MinMagLinearMipPoint = 0x00000014,
    MinMagMipLinear = 0x00000015,
    Anisotropic = 0x00000055,
    ComparisonMinMagMipPoint = 0x00000080,
    ComparisonMinMagPointMipLinear = 0x00000081,
    ComparisonMinPointMagLinearMipPoint = 0x00000084,
    ComparisonMinPointMagMipLinear = 0x00000085,
    ComparisonMinLinearMagMipPoint = 0x00000090,
    ComparisonMinLinearMagPointMipLinear = 0x00000091,
    ComparisonMinMagLinearMipPoint = 0x00000094,
    ComparisonMinMagMipLinear = 0x00000095,
    ComparisonAnisotropic = 0x000000D5,
    MinimumMinMagMipPoint = 0x00000100,
    MinimumMinMagPointMipLinear = 0x00000101,
    MinimumMinPointMagLinearMipPoint = 0x00000104,
    MinimumMinPointMagMipLinear = 0x00000105,
    MinimumMinLinearMagMipPoint = 0x00000110,
    MinimumMinLinearMagPointMipLinear = 0x00000111,
    MinimumMinMagLinearMipPoint = 0x00000114,
    MinimumMinMagMipLinear = 0x00000115,
    MinimumAnisotropic = 0x00000155,
    MaximumMinMagMipPoint = 0x00000180,
    MaximumMinMagPointMipLinear = 0x00000181,
    MaximumMinPointMagLinearMipPoint = 0x00000184,
    MaximumMinPointMagMipLinear = 0x00000185,
    MaximumMinLinearMagMipPoint = 0x00000190,
    MaximumMinLinearMagPointMipLinear = 0x00000191,
    MaximumMinMagLinearMipPoint = 0x00000194,
    MaximumMinMagMipLinear = 0x00000195,
    MaximumAnisotropic = 0x000001D5
}

impl Filter {
    pub fn min(&self) -> FilterType {
        let min = (((*self) as u32) >> 4) & 0x3;
        unsafe { ::std::mem::transmute(min) }
    }

    pub fn mag(&self) -> FilterType {
        let mag = (((*self) as u32) >> 2) & 0x3;
        unsafe { ::std::mem::transmute(mag) }
    }

    pub fn mip(&self) -> FilterType {
        let mip = ((*self) as u32) & 0x3;
        unsafe { ::std::mem::transmute(mip) }
    }

    pub fn anisotropic(&self) -> bool {
        let aniso = ((*self) as u32) & 0x17F;
        aniso & 0x55 == 0x55
    }

    pub fn reduction(&self) -> FilterReductionType {
        let reduction = (((*self) as u32) >> 7) & 0x3;
        unsafe { ::std::mem::transmute(reduction) }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterType {
    Point = 0,
    Linear = 1
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FilterReductionType {
    Standard = 0,
    Comparison = 1,
    Minimum = 2,
    Maximum = 3
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureAddressMode {
    Wrap = 1,
    Mirror = 2,
    Clamp = 3,
    Border = 4,
    MirrorOnce = 5
}

bitflags! {
    #[repr(C)]
    flags FormatSupport: UINT {
        const FORMAT_SUPPORT_BUFFER = 0x00000001,
        const FORMAT_SUPPORT_IA_VERTEX_BUFFER = 0x00000002,
        const FORMAT_SUPPORT_IA_INDEX_BUFFER = 0x00000004,
        const FORMAT_SUPPORT_SO_BUFFER = 0x00000008,
        const FORMAT_SUPPORT_TEXTURE1D = 0x00000010,
        const FORMAT_SUPPORT_TEXTURE2D = 0x00000020,
        const FORMAT_SUPPORT_TEXTURE3D = 0x00000040,
        const FORMAT_SUPPORT_TEXTURECUBE = 0x00000080,
        const FORMAT_SUPPORT_SHADER_LOAD = 0x00000100,
        const FORMAT_SUPPORT_SHADER_SAMPLE = 0x00000200,
        const FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON = 0x00000400,
        const FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT = 0x00000800,
        const FORMAT_SUPPORT_MIP = 0x00001000,
        const FORMAT_SUPPORT_MIP_AUTOGEN = 0x00002000,
        const FORMAT_SUPPORT_RENDER_TARGET = 0x00004000,
        const FORMAT_SUPPORT_BLENDABLE = 0x00008000,
        const FORMAT_SUPPORT_DEPTH_STENCIL = 0x00010000,
        const FORMAT_SUPPORT_CPU_LOCKABLE = 0x00020000,
        const FORMAT_SUPPORT_MULTISAMPLE_RESOLVE = 0x00040000,
        const FORMAT_SUPPORT_DISPLAY = 0x00080000,
        const FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT = 0x00100000,
        const FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET = 0x00200000,
        const FORMAT_SUPPORT_MULTISAMPLE_LOAD = 0x00400000,
        const FORMAT_SUPPORT_SHADER_GATHER = 0x00800000,
        const FORMAT_SUPPORT_BACK_BUFFER_CAST = 0x01000000,
        const FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW = 0x02000000,
        const FORMAT_SUPPORT_SHADER_GATHER_COMPARISON = 0x04000000,
        const FORMAT_SUPPORT_DECODER_OUTPUT = 0x08000000,
        const FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT = 0x10000000,
        const FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT = 0x20000000,
        const FORMAT_SUPPORT_VIDEO_ENCODER = 0x40000000
    }
}

impl Default for FormatSupport {
    fn default() -> FormatSupport { FormatSupport::empty() }
}

bitflags! {
    #[repr(C)]
    flags FormatSupport2: UINT {
        const FORMAT_SUPPORT2_UAV_ATOMIC_ADD = 0x00000001,
        const FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS = 0x00000002,
        const FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE
            = 0x00000004,
        const FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE = 0x00000008,
        const FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX = 0x00000010,
        const FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX = 0x00000020,
        const FORMAT_SUPPORT2_UAV_TYPED_LOAD = 0x00000040,
        const FORMAT_SUPPORT2_UAV_TYPED_STORE = 0x00000080,
        const FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP = 0x00000100,
        const FORMAT_SUPPORT2_TILED = 0x00000200,
        const FORMAT_SUPPORT2_SHAREABLE = 0x00000400,
        #[cfg(feature = "d3d11_3")]
        const FORMAT_SUPPORT2_MULTIPLANE_OVERLAY = 0x00004000,
    }
}

impl Default for FormatSupport2 {
    fn default() -> FormatSupport2 { FormatSupport2::empty() }
}

bitflags! {
    #[repr(C)]
    flags AsyncGetDataFlag: UINT {
        const ASYNC_GETDATA_DONOTFLUSH = 0x1
    }
}

impl Default for AsyncGetDataFlag {
    fn default() -> AsyncGetDataFlag { AsyncGetDataFlag::empty() }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Query {
    Event = 0,
    Occlusion,
    Timestamp,
    TimestampDisjoint,
    PipelineStatistics,
    OcclusionPredicate,
    SOStatistics,
    SOOverflowPredicate,
    SOStatisticsStream0,
    SOOverflowPredicateStream0,
    SOStatisticsStream1,
    SOOverflowPredicateStream1,
    SOStatisticsStream2,
    SOOverflowPredicateStream2,
    SOStatisticsStream3,
    SOOverflowPredicateStream3
}

impl Default for Query {
    fn default() -> Query { Query::Event }
}

bitflags! {
    #[repr(C)]
    flags QueryMiscFlag: UINT {
        const QUERY_MISC_PREDICATEHINT = 0x1
    }
}

impl Default for QueryMiscFlag {
    fn default() -> QueryMiscFlag { QueryMiscFlag::empty() }
}


#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Counter {
    __Dummy,
    DeviceDependent0 = 0x40000000
}

impl Default for Counter {
    fn default() -> Counter { Counter::DeviceDependent0 }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CounterType {
    Float32,
    Uint16,
    Uint32,
    Uint64
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StandardMultisampleQualityLevels {
    Standard = 0xffffffff,
    Center = 0xfffffffe
}

impl Default for StandardMultisampleQualityLevels {
    fn default() -> StandardMultisampleQualityLevels {
        StandardMultisampleQualityLevels::Standard
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceContextType {
    Immediate,
    Deferred
}

impl Default for DeviceContextType {
    fn default() -> DeviceContextType { DeviceContextType::Immediate }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Feature {
    Threading,
    Doubles,
    FormatSupport,
    FormatSupport2,
    D3D10XHardwareOptions,
    D3D11Options,
    ArchitectureInfo,
    D3D9Options,
    ShaderMinPrecisionSupport,
    D3D9ShadowSupport,
    D3D11Options1,
    D3D9SimpleInstancingSupport,
    MarkerSupport,
    D3D9Options1,
    #[cfg(feature = "d3d11_3")] D3D11Options2,
    #[cfg(feature = "d3d11_3")] D3D11Options3,
    #[cfg(feature = "d3d11_3")] GPUVirtualAddressSupport,
}

bitflags! {
    #[repr(C)]
    flags ShaderMinPrecisionSupport: UINT {
        const SHADER_MIN_PRECISION_10_BIT = 0x1,
        const SHADER_MIN_PRECISION_16_BIT = 0x2
    }
}

impl Default for ShaderMinPrecisionSupport {
    fn default() -> ShaderMinPrecisionSupport {
        ShaderMinPrecisionSupport::empty()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TiledResourcesTier {
    NotSupported = 0,
    Tier1 = 1,
    Tier2 = 2,
    #[cfg(feature = "d3d11_3")] Tier3 = 3,
}

impl Default for TiledResourcesTier {
    fn default() -> TiledResourcesTier { TiledResourcesTier::NotSupported }
}

#[cfg(feature = "d3d11_3")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConservativeRasterizationTier {
    NotSupported = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

#[cfg(feature = "d3d11_3")]
impl Default for ConservativeRasterizationTier {
    fn default() -> ConservativeRasterizationTier {
        ConservativeRasterizationTier::NotSupported
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoDecoderBufferType {
    PictureParameters,
    MacroBlockControl,
    ResidualDifference,
    DeblockingControl,
    InverseQuantizationMatrix,
    SliceControl,
    Bitstream,
    MotionVector,
    FilmGrain,
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorFormatSupport: u32 {
        const VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT = 0x00000001,
        const VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT = 0x00000002,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorDeviceCaps: u32 {
        const VIDEO_PROCESSOR_DEVICE_CAPS_LINEAR_SPACE            = 0x1,
        const VIDEO_PROCESSOR_DEVICE_CAPS_XVYCC                   = 0x2,
        const VIDEO_PROCESSOR_DEVICE_CAPS_RGB_RANGE_CONVERSION    = 0x4,
        const VIDEO_PROCESSOR_DEVICE_CAPS_YCBCR_MATRIX_CONVERSION = 0x8,
        const VIDEO_PROCESSOR_DEVICE_CAPS_NOMINAL_RANGE           = 0x10,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorFeatureCaps: u32 {
        const VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_FILL             = 0x1,
        const VIDEO_PROCESSOR_FEATURE_CAPS_CONSTRICTION           = 0x2,
        const VIDEO_PROCESSOR_FEATURE_CAPS_LUMA_KEY               = 0x4,
        const VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_PALETTE          = 0x8,
        const VIDEO_PROCESSOR_FEATURE_CAPS_LEGACY                 = 0x10,
        const VIDEO_PROCESSOR_FEATURE_CAPS_STEREO                 = 0x20,
        const VIDEO_PROCESSOR_FEATURE_CAPS_ROTATION               = 0x40,
        const VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_STREAM           = 0x80,
        const VIDEO_PROCESSOR_FEATURE_CAPS_PIXEL_ASPECT_RATIO     = 0x100,
        #[cfg(feature = "d3d11_3")]
        const VIDEO_PROCESSOR_FEATURE_CAPS_MIRROR                 = 0x200,
        #[cfg(feature = "d3d11_3")]
        const VIDEO_PROCESSOR_FEATURE_CAPS_SHADER_USAGE           = 0x400,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorFilterCaps: u32 {
        const VIDEO_PROCESSOR_FILTER_CAPS_BRIGHTNESS         = 0x1,
        const VIDEO_PROCESSOR_FILTER_CAPS_CONTRAST           = 0x2,
        const VIDEO_PROCESSOR_FILTER_CAPS_HUE                = 0x4,
        const VIDEO_PROCESSOR_FILTER_CAPS_SATURATION         = 0x8,
        const VIDEO_PROCESSOR_FILTER_CAPS_NOISE_REDUCTION    = 0x10,
        const VIDEO_PROCESSOR_FILTER_CAPS_EDGE_ENHANCEMENT   = 0x20,
        const VIDEO_PROCESSOR_FILTER_CAPS_ANAMORPHIC_SCALING = 0x40,
        const VIDEO_PROCESSOR_FILTER_CAPS_STEREO_ADJUSTMENT  = 0x80,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorFormatCaps: u32 {
        const VIDEO_PROCESSOR_FORMAT_CAPS_RGB_INTERLACED     = 0x1,
        const VIDEO_PROCESSOR_FORMAT_CAPS_RGB_PROCAMP        = 0x2,
        const VIDEO_PROCESSOR_FORMAT_CAPS_RGB_LUMA_KEY       = 0x4,
        const VIDEO_PROCESSOR_FORMAT_CAPS_PALETTE_INTERLACED = 0x8,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorAutoStreamCaps: u32 {
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DENOISE              = 0x01,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DERINGING            = 0x02,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_EDGE_ENHANCEMENT     = 0x04,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_COLOR_CORRECTION     = 0x08,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_FLESH_TONE_MAPPING   = 0x10,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_IMAGE_STABILIZATION  = 0x20,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_SUPER_RESOLUTION     = 0x40,
        const VIDEO_PROCESSOR_AUTO_STREAM_CAPS_ANAMORPHIC_SCALING   = 0x80,
    }
}

bitflags! {
    #[repr(C)]
    flags VideoProcessorStereoCaps: u32 {
        const VIDEO_PROCESSOR_STEREO_CAPS_MONO_OFFSET         = 0x01,
        const VIDEO_PROCESSOR_STEREO_CAPS_ROW_INTERLEAVED     = 0x02,
        const VIDEO_PROCESSOR_STEREO_CAPS_COLUMN_INTERLEAVED  = 0x04,
        const VIDEO_PROCESSOR_STEREO_CAPS_CHECKERBOARD        = 0x08,
        const VIDEO_PROCESSOR_STEREO_CAPS_FLIP_MODE           = 0x10,
    }
}

bitflags! {
    #[repr(C)]
    flags ContentProtectionCaps: u32 {
        const CONTENT_PROTECTION_CAPS_SOFTWARE = 0x00000001,
        const CONTENT_PROTECTION_CAPS_HARDWARE = 0x00000002,
        const CONTENT_PROTECTION_CAPS_PROTECTION_ALWAYS_ON = 0x00000004,
        const CONTENT_PROTECTION_CAPS_PARTIAL_DECRYPTION = 0x00000008,
        const CONTENT_PROTECTION_CAPS_CONTENT_KEY = 0x00000010,
        const CONTENT_PROTECTION_CAPS_FRESHEN_SESSION_KEY = 0x00000020,
        const CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK = 0x00000040,
        const CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK_KEY = 0x00000080,
        const CONTENT_PROTECTION_CAPS_SEQUENTIAL_CTR_IV = 0x00000100,
        const CONTENT_PROTECTION_CAPS_ENCRYPT_SLICEDATA_ONLY = 0x00000200,
        const CONTENT_PROTECTION_CAPS_DECRYPTION_BLT = 0x00000400,
        #[cfg(feature = "d3d11_3")]
        const CONTENT_PROTECTION_CAPS_HARDWARE_PROTECT_UNCOMPRESSED = 0x00000800,
        #[cfg(feature = "d3d11_3")]
        const CONTENT_PROTECTION_CAPS_HARDWARE_PROTECTED_MEMORY_PAGEABLE = 0x00001000,
        #[cfg(feature = "d3d11_3")]
        const CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN = 0x00002000,
        #[cfg(feature = "d3d11_3")]
        const CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION = 0x00004000,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorFilter {
    Brightness,
    Contrast,
    Hue,
    Saturation,
    NoiseReduction,
    EdgeEnhancement,
    AnamorphicScaling,
    StereoAdjustment,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoFrameFormat {
    Progressive,
    InterlacedTopFieldFirst,
    InterlacedBottomFieldFirst,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoUsage {
    PlaybackNormal,
    OptimalSpeed,
    OptimalQuality,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorNominalRange {
    Undefined,
    Range16to235,
    Range0to255,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorAlphaFillMode {
    Opaque,
    Background,
    Destination,
    SourceStream,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorOutputRate {
    Normal,
    Half,
    Custom,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorStereoFormat {
    Mono,
    Horizontal,
    Vertical,
    Separate,
    MonoOffset,
    RowInterleaved,
    ColumnInterleaved,
    Checkerboard,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorStereoFlipMode {
    None,
    Frame0,
    Frame1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VideoProcessorRotation {
    Identity,
    Rot90,
    Rot180,
    Rot270,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticatedChannelType {
    D3D11 = 1,
    DriverSoftware,
    DriverHardware,
}

bitflags! {
    #[repr(C)]
    flags AuthenticatedProtectionFlags: u32 {
        const AUTHENTICATED_PROTECTION_FLAG_ENABLED                        = 0x1,
        const AUTHENTICATED_PROTECTION_FLAG_OVERLAY_OR_FULLSCREEN_REQUIRED = 0x2,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticatedProcessIdentifierType {
    Unknown,
    DWM,
    Handle,
}

bitflags! {
    #[repr(C)]
    flags BusType: u32 {
        const BUS_TYPE_OTHER                                            = 0x00000000,
        const BUS_TYPE_PCI                                              = 0x00000001,
        const BUS_TYPE_PCIX                                             = 0x00000002,
        const BUS_TYPE_PCIEXPRESS                                       = 0x00000003,
        const BUS_TYPE_AGP                                              = 0x00000004,
        const BUS_IMPL_MODIFIER_INSIDE_OF_CHIPSET                       = 0x00010000,
        const BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP          = 0x00020000,
        const BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET        = 0x00030000,
        const BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR                = 0x00040000,
        const BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE = 0x00050000,
        const BUS_IMPL_MODIFIER_NON_STANDARD                            = 0x80000000,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VDOVDimension {
    Unknown,
    Texture2D,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VPIVDimension {
    Unknown,
    Texture2D,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VPOVDimension {
    Unknown,
    Texture2D,
    Texture2DArray,
}

bitflags! {
    #[repr(C)]
    flags CreateDeviceFlag: UINT {
        const CREATE_DEVICE_SINGLETHREADED = 0x1,
        const CREATE_DEVICE_DEBUG = 0x2,
        const CREATE_DEVICE_SWITCH_TO_REF = 0x4,
        const CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS
            = 0x8,
        const CREATE_DEVICE_BGRA_SUPPORT = 0x20,
        const CREATE_DEVICE_DEBUGGABLE = 0x40,
        const CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY
            = 0x0080,
        const CREATE_DEVICE_DISABLE_GPU_TIMEOUT = 0x0100,
        const CREATE_DEVICE_VIDEO_SUPPORT = 0x0800
    }
}
