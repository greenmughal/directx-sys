use winapi::{BOOL, FALSE, FLOAT, INT, TRUE, UINT};

use super::super::d3d11_0::*;
use super::enums::*;

use dxgi;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Texture2DDesc1 {
    pub width: UINT,
    pub height: UINT,
    pub mip_levels: UINT,
    pub array_size: UINT,
    pub format: dxgi::Format,
    pub sample_desc: dxgi::SampleDesc,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag,
    pub texture_layout: TextureLayout,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Texture3DDesc1 {
    pub width: UINT,
    pub height: UINT,
    pub depth: UINT,
    pub mip_levels: UINT,
    pub format: dxgi::Format,
    pub usage: Usage,
    pub bind_flags: BindFlag,
    pub cpu_access_flags: CPUAccessFlag,
    pub misc_flags: ResourceMiscFlag,
    pub texture_layout: TextureLayout,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RasterizerDesc2 {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: BOOL,
    pub depth_bias: INT,
    pub depth_bias_clamp: FLOAT,
    pub slope_scaled_depth_bias: FLOAT,
    pub depth_clip_enable: BOOL,
    pub scissor_enable: BOOL,
    pub multisample_enable: BOOL,
    pub antialiased_line_enable: BOOL,
    pub forced_sample_count: UINT,
    pub conservative_raster: ConservativeRasterizationMode,
}

impl Default for RasterizerDesc2 {
    fn default() -> RasterizerDesc2 {
        RasterizerDesc2 {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            front_counter_clockwise: FALSE,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: TRUE,
            scissor_enable: FALSE,
            multisample_enable: FALSE,
            antialiased_line_enable: FALSE,
            forced_sample_count: 0,
            conservative_raster: ConservativeRasterizationMode::Off,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DSRV1 {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArraySRV1 {
    pub most_detailed_mip: UINT,
    pub mip_levels: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ShaderResourceViewDesc1 {
    pub format: dxgi::Format,
    pub view_dimension: SRVDimension,
    union: Tex2DArraySRV1,
}

union! {
    ShaderResourceViewDesc1.union {
        fn buffer() -> BufferSRV,
        fn set_buffer(value: BufferSRV),
        fn texture_1d() -> Tex1DSRV,
        fn set_texture_1d(value: Tex1DSRV),
        fn texture_1d_array() -> Tex1DArraySRV,
        fn set_texture_1d_array(value: Tex1DArraySRV),
        fn texture_2d() -> Tex2DSRV1,
        fn set_texture_2d(value: Tex2DSRV1),
        fn texture_2d_array() -> Tex2DArraySRV1,
        fn set_texture_2d_array(value: Tex2DArraySRV1),
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

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DRTV1 {
    pub mip_slice: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArrayRTV1 {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderTargetViewDesc1 {
    pub format: dxgi::Format,
    pub view_dimension: RTVDimension,
    union: Tex2DArrayRTV1,
}

union! {
    RenderTargetViewDesc1.union {
        fn buffer() -> BufferRTV,
        fn set_buffer(value: BufferRTV),
        fn texture_1d() -> Tex1DRTV,
        fn set_texture_1d(value: Tex1DRTV),
        fn texture_1d_array() -> Tex1DArrayRTV,
        fn set_texture_1d_array(value: Tex1DArrayRTV),
        fn texture_2d() -> Tex2DRTV1,
        fn set_texture_2d(value: Tex2DRTV1),
        fn texture_2d_array() -> Tex2DArrayRTV1,
        fn set_texture_2d_array(value: Tex2DArrayRTV1),
        fn texture_2d_ms() -> Tex2DMSRTV,
        fn set_texture_2d_ms(value: Tex2DMSRTV),
        fn texture_2d_ms_array() -> Tex2DMSArrayRTV,
        fn set_texture_2d_ms_array(value: Tex2DMSArrayRTV),
        fn texture_3d() -> Tex3DRTV,
        fn set_texture_3d(value: Tex3DRTV)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DUAV1 {
    pub mip_slice: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tex2DArrayUAV1 {
    pub mip_slice: UINT,
    pub first_array_slice: UINT,
    pub array_size: UINT,
    pub plane_slice: UINT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UnorderedAccessViewDesc1 {
    pub format: dxgi::Format,
    pub view_dimension: UAVDimension,
    union: Tex2DArrayUAV1,
}

union! {
    UnorderedAccessViewDesc1.union {
        fn buffer() -> BufferUAV,
        fn set_buffer(value: BufferUAV),
        fn texture_1d() -> Tex1DUAV,
        fn set_texture_1d(value: Tex1DUAV),
        fn texture_1d_array() -> Tex1DArrayUAV,
        fn set_texture_1d_array(value: Tex1DArrayUAV),
        fn texture_2d() -> Tex2DUAV1,
        fn set_texture_2d(value: Tex2DUAV1),
        fn texture_2d_array() -> Tex2DArrayUAV1,
        fn set_texture_2d_array(value: Tex2DArrayUAV1),
        fn texture_3d() -> Tex3DUAV,
        fn set_texture_3d(value: Tex3DUAV)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QueryDesc1 {
    pub query: Query,
    pub misc_flags: QueryMiscFlag,
    pub context_type: ContextType,
}

#[test]
fn check_d3d11_3_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<Texture2DDesc1>(), 48);
    assert_eq!(size_of::<Texture3DDesc1>(), 40);
    assert_eq!(size_of::<RasterizerDesc2>(), 48);
    assert_eq!(size_of::<Tex2DSRV1>(), 12);
    assert_eq!(size_of::<Tex2DArraySRV1>(), 20);
    assert_eq!(size_of::<ShaderResourceViewDesc1>(), 28);
    assert_eq!(size_of::<Tex2DRTV1>(), 8);
    assert_eq!(size_of::<Tex2DArrayRTV1>(), 16);
    assert_eq!(size_of::<RenderTargetViewDesc1>(), 24);
    assert_eq!(size_of::<Tex2DUAV1>(), 8);
    assert_eq!(size_of::<Tex2DArrayUAV1>(), 16);
    assert_eq!(size_of::<UnorderedAccessViewDesc1>(), 24);
    assert_eq!(size_of::<QueryDesc1>(), 12);
}
