#[cfg(feature = "dxgi1_4")]
use std::os::raw::c_void;

use winapi::{BOOL, FALSE, FLOAT, INT, TRUE, UINT};
#[cfg(feature = "dxgi1_4")]
use winapi::{GUID, HRESULT, UINT64};

use super::super::d3d11_0::*;
use super::enums::*;

#[cfg(feature = "dxgi1_4")]
use dxgi::{ColorSpaceType, Format};

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderTargetBlendDesc1 {
    pub blend_enable: BOOL,
    pub logic_op_enable: BOOL,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub render_target_write_mask: ColorWriteEnable,
}

impl Default for RenderTargetBlendDesc1 {
    fn default() -> RenderTargetBlendDesc1 {
        RenderTargetBlendDesc1 {
            blend_enable: FALSE,
            logic_op_enable: FALSE,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: BlendOp::Add,
            src_blend_alpha: Blend::One,
            dest_blend_alpha: Blend::Zero,
            blend_op_alpha: BlendOp::Add,
            logic_op: LogicOp::NoOp,
            render_target_write_mask: ColorWriteEnable::all()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlendDesc1 {
    pub alpha_to_coverage_enable: BOOL,
    pub independent_blend_enable: BOOL,
    pub render_target: [RenderTargetBlendDesc1; 8],
}

impl Default for BlendDesc1 {
    fn default() -> BlendDesc1 {
        BlendDesc1 {
            alpha_to_coverage_enable: FALSE,
            independent_blend_enable: FALSE,
            render_target: [RenderTargetBlendDesc1::default(); 8]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RasterizerDesc1 {
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
}

impl Default for RasterizerDesc1 {
    fn default() -> RasterizerDesc1 {
        RasterizerDesc1 {
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
        }
    }
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoDecoderSubSampleMappingBlock {
    pub clear_size: UINT,
    pub encrypted_size: UINT,
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoDecoderBufferDesc1 {
    pub buffer_type: VideoDecoderBufferType,
    pub data_offset: UINT,
    pub data_size: UINT,
    pub iv: *mut c_void,
    pub iv_size: UINT,
    pub sub_sample_mapping: *mut VideoDecoderSubSampleMappingBlock,
    pub sub_sample_mapping_count: UINT,
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoDecoderBeginFrameCryptoSession {
    pub crypto_session: *mut ID3D11CryptoSession,
    pub blob_size: UINT,
    pub blob: *mut c_void,
    pub key_info_id: *mut GUID,
    pub private_data_size: UINT,
    pub private_data: *mut c_void,
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoProcessorStreamBehaviorHint {
    pub enable: BOOL,
    pub width: UINT,
    pub height: UINT,
    pub format: Format,
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyExchangeHWProtectionInputData {
    pub private_data_size: UINT,
    pub hw_protection_data_size: UINT,
    pub input: [u8; 4],
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyExchangeHWProtectionOutputData {
    pub private_data_size: UINT,
    pub max_hw_protection_data_size: UINT,
    pub hw_protection_data_size: UINT,
    pub transport_time: UINT64,
    pub execution_time: UINT64,
    pub output: [u8; 4],
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyExchangeHWProtectionData {
    pub hw_protection_function_id: UINT,
    pub input_data: *mut KeyExchangeHWProtectionInputData,
    pub output_data: *mut KeyExchangeHWProtectionOutputData,
    pub status: HRESULT,
}

#[cfg(feature = "dxgi1_4")]
#[repr(C)]
#[derive(Debug)]
pub struct VideoSampleDesc {
    pub width: UINT,
    pub height: UINT,
    pub format: Format,
    pub color_space: ColorSpaceType,
}

#[test]
fn check_d3d11_1_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<BlendDesc1>(), 328);
    assert_eq!(size_of::<RasterizerDesc1>(), 44);
    assert_eq!(size_of::<RenderTargetBlendDesc1>(), 40);
}

#[test]
#[cfg(feature = "dxgi1_4")]
fn check_d3d11_1_struct_sizes_extra() {
    use std::mem::size_of;

    assert_eq!(size_of::<VideoDecoderSubSampleMappingBlock>(), 8);
    assert_eq!(size_of::<KeyExchangeHWProtectionInputData>(), 12);
    assert_eq!(size_of::<KeyExchangeHWProtectionOutputData>(), 40);
    assert_eq!(size_of::<VideoSampleDesc>(), 16);

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<VideoDecoderBufferDesc1>(), 48);
        assert_eq!(size_of::<VideoProcessorStreamBehaviorHint>(), 16);
        assert_eq!(size_of::<VideoDecoderBeginFrameCryptoSession>(), 48);
        assert_eq!(size_of::<KeyExchangeHWProtectionData>(), 32);
    } else {
        assert_eq!(size_of::<VideoDecoderBufferDesc1>(), 28);
        assert_eq!(size_of::<VideoDecoderBeginFrameCryptoSession>(), 24);
        assert_eq!(size_of::<VideoProcessorStreamBehaviorHint>(), 16);
        assert_eq!(size_of::<KeyExchangeHWProtectionData>(), 16);
    }
}
