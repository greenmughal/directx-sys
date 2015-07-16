use std::fmt;
use std::os::raw::c_void;

use winapi::{BOOL, BYTE, FALSE, FLOAT, INT, LPCSTR, TRUE, UINT, UINT8, UINT64};

use dxgi;
use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InputElementDesc {
    pub semantic_name: LPCSTR,
    pub semantic_index: UINT,
    pub format: dxgi::Format,
    pub input_slot: UINT,
    pub aligned_byte_offset: UINT,
    pub input_slot_class: InputClassification,
    pub instance_data_step_rate: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SODeclarationEntry {
    pub stream: UINT,
    pub semantic_name: LPCSTR,
    pub semantic_index: UINT,
    pub start_component: BYTE,
    pub component_count: BYTE,
    pub output_slot: BYTE
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    pub top_left_x: FLOAT,
    pub top_left_y: FLOAT,
    pub width: FLOAT,
    pub height: FLOAT,
    pub min_depth: FLOAT,
    pub max_depth: FLOAT
}

impl Default for Viewport {
    fn default() -> Viewport {
        Viewport {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 1.0
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BoxRegion {
    pub left: UINT,
    pub top: UINT,
    pub front: UINT,
    pub right: UINT,
    pub bottom: UINT,
    pub back: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DepthStencilOpDesc {
    pub stencil_fail_op: StencilOp,
    pub stencil_depth_fail_op: StencilOp,
    pub stencil_pass_op: StencilOp,
    pub stencil_func: ComparisonFunc
}

impl Default for DepthStencilOpDesc {
    fn default() -> DepthStencilOpDesc {
        DepthStencilOpDesc {
            stencil_fail_op: StencilOp::Keep,
            stencil_depth_fail_op: StencilOp::Keep,
            stencil_pass_op: StencilOp::Keep,
            stencil_func: ComparisonFunc::Always
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DepthStencilDesc {
    pub depth_enable: BOOL,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil_enable: BOOL,
    pub stencil_read_mask: UINT8,
    pub stencil_write_mask: UINT8,
    pub front_face: DepthStencilOpDesc,
    pub back_face: DepthStencilOpDesc
}

impl Default for DepthStencilDesc {
    fn default() -> DepthStencilDesc {
        DepthStencilDesc {
            depth_enable: TRUE,
            depth_write_mask: D3D11_DEPTH_WRITE_MASK_ALL,
            depth_func: ComparisonFunc::Less,
            stencil_enable: FALSE,
            stencil_read_mask: 0xFF,
            stencil_write_mask: 0xFF,
            front_face: DepthStencilOpDesc::default(),
            back_face: DepthStencilOpDesc::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: BOOL,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub render_target_write_mask: ColorWriteEnable
}

impl Default for RenderTargetBlendDesc {
    fn default() -> RenderTargetBlendDesc {
        RenderTargetBlendDesc {
            blend_enable: FALSE,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: BlendOp::Add,
            src_blend_alpha: Blend::One,
            dest_blend_alpha: Blend::Zero,
            blend_op_alpha: BlendOp::Add,
            render_target_write_mask: ColorWriteEnable::all()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlendDesc {
    pub alpha_to_coverage_enable: BOOL,
    pub independent_blend_enable: BOOL,
    pub render_target: [RenderTargetBlendDesc; 8]
}

impl Default for BlendDesc {
    fn default() -> BlendDesc {
        BlendDesc {
            alpha_to_coverage_enable: FALSE,
            independent_blend_enable: FALSE,
            render_target: [RenderTargetBlendDesc::default(); 8]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: BOOL,
    pub depth_bias: INT,
    pub depth_bias_clamp: FLOAT,
    pub slope_scaled_depth_bias: FLOAT,
    pub depth_clip_enable: BOOL,
    pub scissor_enable: BOOL,
    pub multisample_enable: BOOL,
    pub antialiased_line_enable: BOOL
}

impl Default for RasterizerDesc {
    fn default() -> RasterizerDesc {
        RasterizerDesc {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            front_counter_clockwise: FALSE,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: TRUE,
            scissor_enable: FALSE,
            multisample_enable: FALSE,
            antialiased_line_enable: FALSE
        }
    }
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct SubresourceData {
    pub sys_mem: *const c_void,
    pub sys_mem_pitch: UINT,
    pub sys_mem_slice_pitch: UINT
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct MappedSubresource {
    pub data: *mut c_void,
    pub row_pitch: UINT,
    pub depth_pitch: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferDesc {
    pub byte_width: UINT,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag,
    pub structure_byte_stride: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Texture1DDesc {
    pub width: UINT,
    pub mip_levels: UINT,
    pub array_size: UINT,
    pub format: dxgi::Format,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Texture2DDesc {
    pub width: UINT,
    pub height: UINT,
    pub mip_levels: UINT,
    pub array_size: UINT,
    pub format: dxgi::Format,
    pub sample_desc: dxgi::SampleDesc,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Texture3DDesc {
    pub width: UINT,
    pub height: UINT,
    pub depth: UINT,
    pub mip_levels: UINT,
    pub format: dxgi::Format,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferSRV {
    union1: UINT,
    union2: UINT
}

union! {
    BufferSRV.union1 {
        fn first_element() -> UINT,
        fn set_first_element(value: UINT),
        fn element_offset() -> UINT,
        fn set_element_offset(value: UINT)
    }
}

union! {
    BufferSRV.union2 {
        fn num_elements() -> UINT,
        fn set_num_elements(value: UINT),
        fn element_width() -> UINT,
        fn set_element_width(value: UINT)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferExSRV {
    pub first_element: UINT,
    pub num_elements: UINT,
    pub flags: BufferExSRVFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DSRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DArraySRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DSRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArraySRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex3DSRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TexCubeSRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TexCubeArraySRV {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT,
    pub first_2d_array_face: UINT,
    pub num_cubes: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSSRV {
    pub unused_field_nothing_to_define: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSArraySRV {
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShaderResourceViewDesc  {
    pub format: dxgi::Format,
    pub view_dimension: SRVDimension,
    union: [u8; 16]
}

union! {
    ShaderResourceViewDesc.union {
        fn buffer() -> BufferSRV,
        fn set_buffer(value: BufferSRV),
        fn texture_1d() -> Tex1DSRV,
        fn set_texture_1d(value: Tex1DSRV),
        fn texture_1d_array() -> Tex1DArraySRV,
        fn set_texture_1d_array(value: Tex1DArraySRV),
        fn texture_2d() -> Tex2DSRV,
        fn set_texture_2d(value: Tex2DSRV),
        fn texture_2d_array() -> Tex2DArraySRV,
        fn set_texture_2d_array(value: Tex2DArraySRV),
        fn texture_2d_ms() -> Tex2DMSSRV,
        fn set_texture_2d_ms(value: Tex2DMSSRV),
        fn texture_2d_ms_array() -> Tex2DMSArraySRV,
        fn set_texture_2d_ms_array(value: Tex2DMSArraySRV),
        fn texture_3d() -> Tex3DSRV,
        fn set_texture_3d(value: Tex3DSRV),
        fn texture_cube() -> TexCubeSRV,
        fn set_texture_cube(value: TexCubeSRV),
        fn texture_cube_array() -> TexCubeArraySRV,
        fn set_texture_cube_array(value: TexCubeArraySRV),
        fn buffer_ex() -> BufferExSRV,
        fn set_buffer_ex(value: BufferExSRV)
    }
}

impl fmt::Debug for ShaderResourceViewDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "ShaderResourceViewDesc {{ "));
        try!(write!(f, "format: {:?}, ", self.format));
        try!(write!(f, "view_dimension: {:?}", self.view_dimension));
        match self.view_dimension {
            SRVDimension::Unknown => { },
            SRVDimension::Buffer => {
                try!(write!(f, ", buffer: {:?}", self.buffer()));
            },
            SRVDimension::Texture1D => {
                try!(write!(f, ", texture_1d: {:?}", self.texture_1d()));
            },
            SRVDimension::Texture1DArray => {
                try!(write!(f, ", texture_1d_array: {:?}",
                            self.texture_1d_array()));
            },
            SRVDimension::Texture2D => {
                try!(write!(f, ", texture_2d: {:?}", self.texture_2d()));
            },
            SRVDimension::Texture2DArray => {
                try!(write!(f, ", texture_2d_array: {:?}",
                            self.texture_2d_array()));
            },
            SRVDimension::Texture2DMS => {
                try!(write!(f, ", texture_2d_ms: {:?}", self.texture_2d_ms()));
            },
            SRVDimension::Texture2DMSArray => {
                try!(write!(f, ", texture_2d_ms_array: {:?}",
                            self.texture_2d_ms_array()));
            },
            SRVDimension::Texture3D => {
                try!(write!(f, ", texture_3d: {:?}", self.texture_3d()));
            },
            SRVDimension::TextureCube => {
                try!(write!(f, ", texture_cube: {:?}", self.texture_cube()));
            },
            SRVDimension::TextureCubeArray => {
                try!(write!(f, ", texture_cube_array: {:?}",
                            self.texture_cube_array()));
            },
            SRVDimension::BufferEx => {
                try!(write!(f, ", buffer_ex: {:?}", self.buffer_ex()));
            }
        }
        write!(f, " }}")
    }
}

impl Default for ShaderResourceViewDesc {
    fn default() -> ShaderResourceViewDesc {
        ShaderResourceViewDesc {
            format: dxgi::Format::Unknown,
            view_dimension: SRVDimension::Unknown,
            union: [0; 16]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferRTV {
    union1: UINT,
    union2: UINT
}

union! {
    BufferRTV.union1 {
        fn first_element() -> UINT,
        fn set_first_element(value: UINT),
        fn element_offset() -> UINT,
        fn set_element_offset(value: UINT)
    }
}

union! {
    BufferRTV.union2 {
        fn num_elements() -> UINT,
        fn set_num_elements(value: UINT),
        fn element_width() -> UINT,
        fn set_element_width(value: UINT)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DRTV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DArrayRTV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DRTV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSRTV {
    pub unused_field_nothing_to_define: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArrayRTV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSArrayRTV {
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex3DRTV {
    pub mip_slice: UINT,
    pub first_w_slice: UINT,
    pub w_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderTargetViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: RTVDimension,
    union: [u8; 12]
}

union! {
    RenderTargetViewDesc.union {
        fn buffer() -> BufferRTV,
        fn set_buffer(value: BufferRTV),
        fn texture_1d() -> Tex1DRTV,
        fn set_texture_1d(value: Tex1DRTV),
        fn texture_1d_array() -> Tex1DArrayRTV,
        fn set_texture_1d_array(value: Tex1DArrayRTV),
        fn texture_2d() -> Tex2DRTV,
        fn set_texture_2d(value: Tex2DRTV),
        fn texture_2d_array() -> Tex2DArrayRTV,
        fn set_texture_2d_array(value: Tex2DArrayRTV),
        fn texture_2d_ms() -> Tex2DMSRTV,
        fn set_texture_2d_ms(value: Tex2DMSRTV),
        fn texture_2d_ms_array() -> Tex2DMSArrayRTV,
        fn set_texture_2d_ms_array(value: Tex2DMSArrayRTV),
        fn texture_3d() -> Tex3DRTV,
        fn set_texture_3d(value: Tex3DRTV)
    }
}

impl fmt::Debug for RenderTargetViewDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "RenderTargetViewDesc {{ "));
        try!(write!(f, "format: {:?}, ", self.format));
        try!(write!(f, "view_dimension: {:?}", self.view_dimension));
        match self.view_dimension {
            RTVDimension::Unknown => { },
            RTVDimension::Buffer => {
                try!(write!(f, ", buffer: {:?}", self.buffer()));
            },
            RTVDimension::Texture1D => {
                try!(write!(f, ", texture_1d: {:?}", self.texture_1d()));
            },
            RTVDimension::Texture1DArray => {
                try!(write!(f, ", texture_1d_array: {:?}",
                            self.texture_1d_array()));
            },
            RTVDimension::Texture2D => {
                try!(write!(f, ", texture_2d: {:?}", self.texture_2d()));
            },
            RTVDimension::Texture2DArray => {
                try!(write!(f, ", texture_2d_array: {:?}",
                            self.texture_2d_array()));
            },
            RTVDimension::Texture2DMS => {
                try!(write!(f, ", texture_2d_ms: {:?}", self.texture_2d_ms()));
            },
            RTVDimension::Texture2DMSArray => {
                try!(write!(f, ", texture_2d_ms_array: {:?}",
                            self.texture_2d_ms_array()));
            },
            RTVDimension::Texture3D => {
                try!(write!(f, ", texture_3d: {:?}", self.texture_3d()));
            }
        }
        write!(f, " }}")
    }
}

impl Default for RenderTargetViewDesc {
    fn default() -> RenderTargetViewDesc {
        RenderTargetViewDesc {
            format: dxgi::Format::Unknown,
            view_dimension: RTVDimension::Unknown,
            union: [0; 12]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DDSV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DArrayDSV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DDSV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArrayDSV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSDSV {
    pub unused_field_nothing_to_define: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DMSArrayDSV {
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DepthStencilViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: DSVDimension,
    pub flags: DSVFlag,
    union: [u8; 12]
}

union! {
    DepthStencilViewDesc.union {
        fn texture_1d() -> Tex1DDSV,
        fn set_texture_1d(value: Tex1DDSV),
        fn texture_1d_array() -> Tex1DArrayDSV,
        fn set_texture_1d_array(value: Tex1DArrayDSV),
        fn texture_2d() -> Tex2DDSV,
        fn set_texture_2d(value: Tex2DDSV),
        fn texture_2d_array() -> Tex2DArrayDSV,
        fn set_texture_2d_array(value: Tex2DArrayDSV),
        fn texture_2d_ms() -> Tex2DMSDSV,
        fn set_texture_2d_ms(value: Tex2DMSDSV),
        fn texture_2d_ms_array() -> Tex2DMSArrayDSV,
        fn set_texture_2d_ms_array(value: Tex2DMSArrayDSV)
    }
}

impl fmt::Debug for DepthStencilViewDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "DepthStencilViewDesc {{ "));
        try!(write!(f, "format: {:?}, ", self.format));
        try!(write!(f, "view_dimension: {:?}, ", self.view_dimension));
        try!(write!(f, "flags: {:?}", self.flags));
        match self.view_dimension {
            DSVDimension::Unknown => { },
            DSVDimension::Texture1D => {
                try!(write!(f, ", texture_1d: {:?}", self.texture_1d()));
            },
            DSVDimension::Texture1DArray => {
                try!(write!(f, ", texture_1d_array: {:?}",
                            self.texture_1d_array()));
            },
            DSVDimension::Texture2D => {
                try!(write!(f, ", texture_2d: {:?}", self.texture_2d()));
            },
            DSVDimension::Texture2DArray => {
                try!(write!(f, ", texture_2d_array: {:?}",
                            self.texture_2d_array()));
            },
            DSVDimension::Texture2DMS => {
                try!(write!(f, ", texture_2d_ms: {:?}", self.texture_2d_ms()));
            },
            DSVDimension::Texture2DMSArray => {
                try!(write!(f, ", texture_2d_ms_array: {:?}",
                            self.texture_2d_ms_array()));
            }
        }
        write!(f, " }}")
    }
}

impl Default for DepthStencilViewDesc {
    fn default() -> DepthStencilViewDesc {
        DepthStencilViewDesc {
            format: dxgi::Format::Unknown,
            view_dimension: DSVDimension::Unknown,
            flags: DSVFlag::empty(),
            union: [0; 12]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BufferUAV {
    pub first_element: UINT,
    pub num_elements: UINT,
    pub flags: BufferUAVFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DUAV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex1DArrayUAV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DUAV {
    pub mip_slice: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArrayUAV {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex3DUAV {
    pub mip_slice: UINT,
    pub first_w_slice: UINT,
    pub w_size: UINT
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnorderedAccessViewDesc {
    pub format: dxgi::Format,
    pub view_dimension: UAVDimension,
    union: [u8; 12]
}

union! {
    UnorderedAccessViewDesc.union {
        fn buffer() -> BufferUAV,
        fn set_buffer(value: BufferUAV),
        fn texture_1d() -> Tex1DUAV,
        fn set_texture_1d(value: Tex1DUAV),
        fn texture_1d_array() -> Tex1DArrayUAV,
        fn set_texture_1d_array(value: Tex1DArrayUAV),
        fn texture_2d() -> Tex2DUAV,
        fn set_texture_2d(value: Tex2DUAV),
        fn texture_2d_array() -> Tex2DArrayUAV,
        fn set_texture_2d_array(value: Tex2DArrayUAV),
        fn texture_3d() -> Tex3DUAV,
        fn set_texture_3d(value: Tex3DUAV)
    }
}

impl fmt::Debug for UnorderedAccessViewDesc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "UnorderedAccessViewDesc {{ "));
        try!(write!(f, "format: {:?}, ", self.format));
        try!(write!(f, "view_dimension: {:?}", self.view_dimension));
        match self.view_dimension {
            UAVDimension::Unknown => { },
            UAVDimension::Buffer => {
                try!(write!(f, ", buffer: {:?}", self.buffer()));
            },
            UAVDimension::Texture1D => {
                try!(write!(f, ", texture_1d: {:?}", self.texture_1d()));
            },
            UAVDimension::Texture1DArray => {
                try!(write!(f, ", texture_1d_array: {:?}",
                            self.texture_1d_array()));
            },
            UAVDimension::Texture2D => {
                try!(write!(f, ", texture_2d: {:?}", self.texture_2d()));
            },
            UAVDimension::Texture2DArray => {
                try!(write!(f, ", texture_2d_array: {:?}",
                            self.texture_2d_array()));
            },
            UAVDimension::Texture3D => {
                try!(write!(f, ", texture_3d: {:?}", self.texture_3d()));
            }
        }
        write!(f, " }}")
    }
}

impl Default for UnorderedAccessViewDesc {
    fn default() -> UnorderedAccessViewDesc {
        UnorderedAccessViewDesc {
            format: dxgi::Format::Unknown,
            view_dimension: UAVDimension::Unknown,
            union: [0; 12]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SamplerDesc {
    pub filter: Filter,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub mip_lod_bias: FLOAT,
    pub max_anisotropy: UINT,
    pub comparison_func: ComparisonFunc,
    pub border_color: [FLOAT; 4],
    pub min_lod: FLOAT,
    pub max_lod: FLOAT
}

impl Default for SamplerDesc {
    fn default() -> SamplerDesc {
        SamplerDesc {
            filter: Filter::MinMagMipLinear,
            address_u: TextureAddressMode::Clamp,
            address_v: TextureAddressMode::Clamp,
            address_w: TextureAddressMode::Clamp,
            mip_lod_bias: 0.0,
            max_anisotropy: 1,
            comparison_func: ComparisonFunc::Never,
            border_color: [1.0; 4],
            min_lod: ::std::f32::MIN,
            max_lod: ::std::f32::MAX
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueryDesc {
    pub query: Query,
    pub misc_flags: QueryMiscFlag
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueryDataTimestampDisjoint {
    pub frequency: UINT64,
    pub disjoint: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueryDataPipelineStatistics {
    pub ia_vertices: UINT64,
    pub ia_primitives: UINT64,
    pub vs_invocations: UINT64,
    pub gs_invocations: UINT64,
    pub gs_primitives: UINT64,
    pub c_invocations: UINT64,
    pub c_primitives: UINT64,
    pub ps_invocations: UINT64,
    pub hs_invocations: UINT64,
    pub ds_invocations: UINT64,
    pub cs_invocations: UINT64
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueryDataSOStatistics {
    pub num_primitives_written: UINT64,
    pub primitives_storage_needed: UINT64
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CounterDesc {
    pub counter: Counter,
    pub misc_flags: UINT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CounterInfo {
    pub last_device_dependent_counter: Counter,
    pub num_simultaneous_counters: UINT,
    pub num_detectable_parallel_units: UINT8
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ClassInstanceDesc {
    pub instance_id: UINT,
    pub instance_index: UINT,
    pub type_id: UINT,
    pub constant_buffer: UINT,
    pub base_constant_buffer_offset: UINT,
    pub base_texture: UINT,
    pub base_sampler: UINT,
    pub created: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataThreading {
    pub driver_concurrent_creates: BOOL,
    pub driver_command_lists: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataDoubles {
    pub double_precision_float_shader_ops: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataFormatSupport {
    pub in_format: dxgi::Format,
    pub out_format_support: FormatSupport
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataFormatSupport2 {
    pub in_format: dxgi::Format,
    pub out_format_support_2: FormatSupport2
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D10XHardwareOptions {
    pub compute_shaders_plus_raw_and_structured_buffers_via_shader_4x: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D11Options {
    pub output_merger_logic_op: BOOL,
    pub uav_only_rendering_forced_sample_count: BOOL,
    pub discard_apis_seen_by_driver: BOOL,
    pub flags_for_update_and_copy_seen_by_driver: BOOL,
    pub clear_view: BOOL,
    pub copy_with_overlap: BOOL,
    pub constant_buffer_partial_update: BOOL,
    pub constant_buffer_offsetting: BOOL,
    pub map_no_overwrite_on_dynamic_constant_buffer: BOOL,
    pub map_no_overwrite_on_dynamic_buffer_srv: BOOL,
    pub multisample_rtv_with_forced_sample_count_one: BOOL,
    pub sad4_shader_instructions: BOOL,
    pub extended_doubles_shader_instructions: BOOL,
    pub extended_resource_sharing: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataArchitectureInfo {
    pub tile_based_deferred_renderer: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D9Options {
    pub full_non_pow2_texture_support: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D9ShadowSupport {
    pub supports_depth_as_texture_with_less_equal_comparison_filter: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataShaderMinPrecisionSupport {
    pub pixel_shader_min_precision: ShaderMinPrecisionSupport,
    pub all_other_shader_stages_min_precision: ShaderMinPrecisionSupport
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D11Options1 {
    pub tiled_resources_tier: TiledResourcesTier,
    pub min_max_filtering: BOOL,
    pub clear_view_also_supports_depth_only_formats: BOOL,
    pub map_on_default_buffers: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D9SimpleInstancingSupport {
    pub simple_instancing_supported: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataMarkerSupport {
    pub profile: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FeatureDataD3D9Options1 {
    pub full_non_pow2_texture_supported: BOOL,
    pub depth_as_texture_with_less_equal_comparison_filter_supported: BOOL,
    pub simple_instancing_supported: BOOL,
    pub texture_cube_face_render_target_with_non_cube_depth_stencil_supported:
        BOOL
}

#[test]
fn check_d3d11_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<AsyncGetDataFlag>(), 4);
    assert_eq!(size_of::<BindFlag>(), 4);
    assert_eq!(size_of::<Blend>(), 4);
    assert_eq!(size_of::<BlendDesc>(), 264);
    assert_eq!(size_of::<BlendOp>(), 4);
    assert_eq!(size_of::<BoxRegion>(), 24);
    assert_eq!(size_of::<BufferDesc>(), 24);
    assert_eq!(size_of::<BufferExSRV>(), 12);
    assert_eq!(size_of::<BufferExSRVFlag>(), 4);
    assert_eq!(size_of::<BufferRTV>(), 8);
    assert_eq!(size_of::<BufferSRV>(), 8);
    assert_eq!(size_of::<BufferUAV>(), 12);
    assert_eq!(size_of::<BufferUAVFlag>(), 4);
    assert_eq!(size_of::<ClassInstanceDesc>(), 32);
    assert_eq!(size_of::<ClearFlag>(), 4);
    assert_eq!(size_of::<ColorWriteEnable>(), 4);
    assert_eq!(size_of::<ComparisonFunc>(), 4);
    assert_eq!(size_of::<Counter>(), 4);
    assert_eq!(size_of::<CounterDesc>(), 8);
    assert_eq!(size_of::<CounterInfo>(), 12);
    assert_eq!(size_of::<CounterType>(), 4);
    assert_eq!(size_of::<CPUAccessFlag>(), 4);
    assert_eq!(size_of::<CreateDeviceFlag>(), 4);
    assert_eq!(size_of::<CullMode>(), 4);
    assert_eq!(size_of::<DepthStencilDesc>(), 52);
    assert_eq!(size_of::<DepthStencilOpDesc>(), 16);
    assert_eq!(size_of::<DepthStencilViewDesc>(), 24);
    assert_eq!(size_of::<DepthWriteMask>(), 4);
    assert_eq!(size_of::<DeviceContextType>(), 4);
    assert_eq!(size_of::<DriverType>(), 4);
    assert_eq!(size_of::<DSVDimension>(), 4);
    assert_eq!(size_of::<DSVFlag>(), 4);
    assert_eq!(size_of::<Feature>(), 4);
    assert_eq!(size_of::<FeatureDataD3D10XHardwareOptions>(), 4);
    assert_eq!(size_of::<FeatureDataDoubles>(), 4);
    assert_eq!(size_of::<FeatureDataFormatSupport2>(), 8);
    assert_eq!(size_of::<FeatureDataFormatSupport>(), 8);
    assert_eq!(size_of::<FeatureDataThreading>(), 8);
    assert_eq!(size_of::<FeatureLevel>(), 4);
    assert_eq!(size_of::<FillMode>(), 4);
    assert_eq!(size_of::<Filter>(), 4);
    assert_eq!(size_of::<FilterType>(), 4);
    assert_eq!(size_of::<FormatSupport2>(), 4);
    assert_eq!(size_of::<FormatSupport>(), 4);
    assert_eq!(size_of::<InputClassification>(), 4);
    assert_eq!(size_of::<Map>(), 4);
    assert_eq!(size_of::<MapFlag>(), 4);
    assert_eq!(size_of::<Primitive>(), 4);
    assert_eq!(size_of::<PrimitiveTopology>(), 4);
    assert_eq!(size_of::<Query>(), 4);
    assert_eq!(size_of::<QueryDataPipelineStatistics>(), 88);
    assert_eq!(size_of::<QueryDataSOStatistics>(), 16);
    assert_eq!(size_of::<QueryDataTimestampDisjoint>(), 16);
    assert_eq!(size_of::<QueryDesc>(), 8);
    assert_eq!(size_of::<QueryMiscFlag>(), 4);
    assert_eq!(size_of::<RaiseFlag>(), 4);
    assert_eq!(size_of::<RasterizerDesc>(), 40);
    assert_eq!(size_of::<Rect>(), 16);
    assert_eq!(size_of::<RenderTargetBlendDesc>(), 32);
    assert_eq!(size_of::<RenderTargetViewDesc>(), 20);
    assert_eq!(size_of::<ResourceDimension>(), 4);
    assert_eq!(size_of::<ResourceMiscFlag>(), 4);
    assert_eq!(size_of::<RTVDimension>(), 4);
    assert_eq!(size_of::<SamplerDesc>(), 52);
    assert_eq!(size_of::<ShaderResourceViewDesc>(), 24);
    assert_eq!(size_of::<SRVDimension>(), 4);
    assert_eq!(size_of::<StandardMultisampleQualityLevels>(), 4);
    assert_eq!(size_of::<StencilOp>(), 4);
    assert_eq!(size_of::<Tex1DArrayDSV>(), 12);
    assert_eq!(size_of::<Tex1DArrayRTV>(), 12);
    assert_eq!(size_of::<Tex1DArraySRV>(), 16);
    assert_eq!(size_of::<Tex1DArrayUAV>(), 12);
    assert_eq!(size_of::<Tex1DDSV>(), 4);
    assert_eq!(size_of::<Tex1DRTV>(), 4);
    assert_eq!(size_of::<Tex1DSRV>(), 8);
    assert_eq!(size_of::<Tex1DUAV>(), 4);
    assert_eq!(size_of::<Tex2DArrayDSV>(), 12);
    assert_eq!(size_of::<Tex2DArrayRTV>(), 12);
    assert_eq!(size_of::<Tex2DArraySRV>(), 16);
    assert_eq!(size_of::<Tex2DArrayUAV>(), 12);
    assert_eq!(size_of::<Tex2DDSV>(), 4);
    assert_eq!(size_of::<Tex2DMSArrayDSV>(), 8);
    assert_eq!(size_of::<Tex2DMSArrayRTV>(), 8);
    assert_eq!(size_of::<Tex2DMSArraySRV>(), 8);
    assert_eq!(size_of::<Tex2DMSDSV>(), 4);
    assert_eq!(size_of::<Tex2DMSRTV>(), 4);
    assert_eq!(size_of::<Tex2DMSSRV>(), 4);
    assert_eq!(size_of::<Tex2DRTV>(), 4);
    assert_eq!(size_of::<Tex2DSRV>(), 8);
    assert_eq!(size_of::<Tex2DUAV>(), 4);
    assert_eq!(size_of::<Tex3DRTV>(), 12);
    assert_eq!(size_of::<Tex3DSRV>(), 8);
    assert_eq!(size_of::<Tex3DUAV>(), 12);
    assert_eq!(size_of::<TexCubeArraySRV>(), 16);
    assert_eq!(size_of::<TexCubeSRV>(), 8);
    assert_eq!(size_of::<Texture1DDesc>(), 32);
    assert_eq!(size_of::<Texture2DDesc>(), 44);
    assert_eq!(size_of::<Texture3DDesc>(), 36);
    assert_eq!(size_of::<TextureAddressMode>(), 4);
    assert_eq!(size_of::<TextureCubeFace>(), 4);
    assert_eq!(size_of::<UAVDimension>(), 4);
    assert_eq!(size_of::<UnorderedAccessViewDesc>(), 20);
    assert_eq!(size_of::<Usage>(), 4);
    assert_eq!(size_of::<Viewport>(), 24);

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<InputElementDesc>(), 32);
        assert_eq!(size_of::<MappedSubresource>(), 16);
        assert_eq!(size_of::<SODeclarationEntry>(), 24);
        assert_eq!(size_of::<SubresourceData>(), 16);
    } else {
        assert_eq!(size_of::<InputElementDesc>(), 28);
        assert_eq!(size_of::<MappedSubresource>(), 12);
        assert_eq!(size_of::<SODeclarationEntry>(), 16);
        assert_eq!(size_of::<SubresourceData>(), 12);
    }
}