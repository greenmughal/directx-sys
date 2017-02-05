use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, DWORD, FLOAT, HANDLE, HRESULT, INT, LPCWSTR, UINT};
#[cfg(feature = "dxgi1_4")]
use winapi::{GUID, UINT64};

use super::super::d3d11_0::*;

use super::enums::*;
use super::structs::*;
#[cfg(feature = "dxgi1_4")]
use dxgi::{ColorSpaceType, Format, Rational};

extern {
    static IID_ID3D11BlendState1: IID;
    static IID_ID3D11RasterizerState1: IID;
    static IID_ID3DDeviceContextState: IID;
    static IID_ID3D11DeviceContext1: IID;
    #[cfg(feature = "dxgi1_4")] static IID_ID3D11VideoContext1: IID;
    #[cfg(feature = "dxgi1_4")] static IID_ID3D11VideoDevice1: IID;
    #[cfg(feature = "dxgi1_4")] static IID_ID3D11VideoProcessorEnumerator1: IID;
    static IID_ID3D11Device1: IID;
    static IID_ID3DUserDefinedAnnotation: IID;
}

com_interface! {
    interface ID3D11BlendState1: ID3D11BlendState, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11BlendState1,
        vtable: ID3D11BlendState1Vtbl,
        fn get_desc1(desc: *mut BlendDesc1) -> ();
    }
}

com_interface! {
    interface ID3D11RasterizerState1: ID3D11RasterizerState, ID3D11DeviceChild,
                                      IUnknown {
        iid: IID_ID3D11RasterizerState1,
        vtable: ID3D11RasterizerState1Vtbl,
        fn get_desc1(desc: *mut RasterizerDesc1) -> ();
    }
}

com_interface! {
    interface ID3DDeviceContextState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3DDeviceContextState,
        vtable: ID3DDeviceContextStateVtbl,
    }
}

com_interface! {
    interface ID3D11DeviceContext1: ID3D11DeviceContext, ID3D11DeviceChild,
                                    IUnknown {
        iid: IID_ID3D11DeviceContext1,
        vtable: ID3D11DeviceContext1Vtbl,
        fn copy_subresource_region1(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_x: UINT,
            dst_y: UINT,
            dst_z: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            src_box: *const BoxRegion,
            copy_flags: CopyFlags) -> ();
        fn update_subresource1(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_box: *const BoxRegion,
            src_data: *const c_void,
            src_row_pitch: UINT,
            src_depth_pitch: UINT,
            copy_flags: CopyFlags) -> ();
        fn discard_resource(resource: *const ID3D11Resource) -> ();
        fn discard_view(resource_view: *const ID3D11View) -> ();
        fn vs_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn hs_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn ds_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn gs_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn ps_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn cs_set_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            first_constant: *const UINT,
            num_constants: *const UINT) -> ();
        fn vs_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn hs_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn ds_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn gs_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn ps_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn cs_get_constant_buffers1(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer,
            first_constant: *mut UINT,
            num_constants: *mut UINT) -> ();
        fn swap_device_context_state(
            state: *const ID3DDeviceContextState,
            previous_state: *mut *mut ID3DDeviceContextState) -> ();
        fn clear_view(
            view: *const ID3D11View,
            color: &[FLOAT; 4],
            rect: *const Rect,
            num_rects: UINT) -> ();
        fn discard_view1(
            resource: *const ID3D11View,
            rects: *const Rect,
            num_rects: UINT) -> ();
    }
}

#[cfg(feature = "dxgi1_4")]
com_interface! {
    interface ID3D11VideoContext1: ID3D11VideoContext, ID3D11DeviceChild,
                                   IUnknown {
        iid: IID_ID3D11VideoContext1,
        vtable: ID3D11VideoContext1Vtbl,
        fn submit_decoder_buffers1(
            decoder: *const ID3D11VideoDecoder,
            num_buffers: UINT,
            buffer_desc: *const VideoDecoderBufferDesc1) -> HRESULT;
        fn get_data_for_new_hardware_key(
            crypto_session: *const ID3D11CryptoSession,
            private_input_size: UINT,
            private_input_data: *const c_void,
            private_output_data: *mut UINT64) -> HRESULT;
        fn check_crypto_session_status(
            crypto_session: *const ID3D11CryptoSession,
            status: *mut CryptoSessionStatus) -> HRESULT;
        fn decoder_enable_downsampling(
            decoder: *const ID3D11VideoDecoder,
            input_color_space: ColorSpaceType,
            output_desc: *const VideoSampleDesc,
            reference_frame_count: UINT) -> HRESULT;
        fn decoder_update_downsampling(
            decoder: *const ID3D11VideoDecoder,
            output_desc: *const VideoSampleDesc) -> HRESULT;
        fn video_processor_set_output_color_space1(
            video_processor: *const ID3D11VideoProcessor,
            color_space: ColorSpaceType) -> ();
        fn video_processor_set_output_shader_usage(
            video_processor: *const ID3D11VideoProcessor,
            shader_usage: BOOL) -> ();
        fn video_processor_get_output_color_space1(
            video_processor: *const ID3D11VideoProcessor,
            color_space: *mut ColorSpaceType) -> ();
        fn video_processor_get_output_shader_usage(
            video_processor: *const ID3D11VideoProcessor,
            shader_usage: *mut BOOL) -> ();
        fn video_processor_set_stream_color_space1(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            color_space: ColorSpaceType) -> ();
        fn video_processor_set_stream_mirror(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            flip_horizontal: BOOL,
            flip_vertical: BOOL) -> ();
        fn video_processor_get_stream_color_space1(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            color_space: *mut ColorSpaceType) -> ();
        fn video_processor_get_stream_mirror(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: *mut BOOL,
            flip_horizontal: *mut BOOL,
            flip_vertical: *mut BOOL) -> ();
        fn video_processor_get_behavior_hints(
            video_processor: *const ID3D11VideoProcessor,
            output_width: UINT,
            output_height: UINT,
            output_format: Format,
            stream_count: UINT,
            streams: *const VideoProcessorBehaviorHints,
            behavior_hints: *mut UINT) -> HRESULT;
    }
}

#[cfg(feature = "dxgi1_4")]
com_interface! {
    interface ID3D11VideoDevice1: ID3D11VideoDevice, IUnknown {
        iid: IID_ID3D11VideoDevice1,
        vtable: ID3D11VideoDevice1Vtbl,
        fn get_crypto_session_private_data_size(
            crypto_type: *const GUID,
            decoder_profile: *const GUID,
            key_exchange_type: *const GUID,
            private_input_size: *mut UINT,
            private_output_size: *mut UINT) -> HRESULT;
        fn get_video_decoder_caps(
            decoder_profile: *const GUID,
            sample_width: UINT,
            sample_height: UINT,
            frame_rate: *const Rational,
            bit_rate: UINT,
            crypto_type: *const GUID,
            decoder_caps: *mut VideoDecoderCaps) -> HRESULT;
        fn check_video_decoder_downsampling(
            input_desc: *const VideoDecoderDesc,
            input_color_space: ColorSpaceType,
            input_config: *const VideoDecoderConfig,
            frame_rate: *const Rational,
            output_desc: *const VideoSampleDesc,
            supported: *mut BOOL,
            real_time_hint: *mut BOOL) -> HRESULT;
        fn recommend_video_decoder_downsample_parameters(
            input_desc: *const VideoDecoderDesc,
            input_color_space: ColorSpaceType,
            input_config: *const VideoDecoderConfig,
            frame_rate: *const Rational,
            recommended_output_desc: *mut VideoSampleDesc) -> HRESULT;
    }
}

#[cfg(feature = "dxgi1_4")]
com_interface! {
    interface ID3D11VideoProcessorEnumerator1: ID3D11VideoProcessorEnumerator,
                                               ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VideoProcessorEnumerator1,
        vtable: ID3D11VideoProcessorEnumerator1Vtbl,
        fn check_video_processor_format_conversion(
            input_format: Format,
            input_color_space: ColorSpaceType,
            output_format: Format,
            output_color_space: ColorSpaceType,
            supported: *mut BOOL) -> HRESULT;
    }
}

com_interface! {
    interface ID3D11Device1: ID3D11Device, IUnknown {
        iid: IID_ID3D11Device1,
        vtable: ID3D11Device1Vtbl,
        fn get_immediate_context1(
            immediate_context: *mut ID3D11DeviceContext1) -> ();
        fn create_deferred_context1(
            context_flags: UINT,
            deferred_context: *mut *mut ID3D11DeviceContext1) -> HRESULT;
        fn create_blend_state1(
            blend_state_desc: *const BlendDesc1,
            blend_state: *mut *mut ID3D11BlendState1) -> HRESULT;
        fn create_rasterizer_state1(
            rasterizer_desc: *const RasterizerDesc1,
            rasterizer_state: *mut *mut ID3D11RasterizerState1) -> HRESULT;
        fn create_device_context_state(
            flags: CreateDeviceContextStateFlags,
            feature_levels: *const FeatureLevel,
            num_feature_levels: UINT,
            sdk_version: UINT,
            emulated_interface: &IID,
            chosen_feature_level: *mut FeatureLevel,
            context_state: *mut *mut ID3DDeviceContextState) -> HRESULT;
        fn open_shared_resource1(
            resource_handle: HANDLE,
            returned_interface: &IID,
            resource: *mut *mut c_void) -> HRESULT;
        fn open_shared_resource_by_name(
            name: LPCWSTR,
            desired_access: DWORD, // TODO type?
            returned_interface: &IID,
            resource: *mut *mut c_void) -> HRESULT;
    }
}

com_interface! {
    interface ID3DUserDefinedAnnotation: IUnknown {
        iid: IID_ID3DUserDefinedAnnotation,
        vtable: ID3DUserDefinedAnnotationVtbl,
        fn begin_event(name: LPCWSTR) -> INT;
        fn end_event() -> INT;
        fn set_marker(name: LPCWSTR) -> ();
        fn get_status() -> BOOL;
    }
}

#[test]
fn check_d3d11_1_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID3D11BlendState1Vtbl>(), 72);
        assert_eq!(size_of::<ID3D11RasterizerState1Vtbl>(), 72);
        assert_eq!(size_of::<ID3DDeviceContextStateVtbl>(), 56);
        assert_eq!(size_of::<ID3D11DeviceContext1Vtbl>(), 1072);
        assert_eq!(size_of::<ID3D11Device1Vtbl>(), 400);
        assert_eq!(size_of::<ID3DUserDefinedAnnotationVtbl>(), 56);
    } else {
        assert_eq!(size_of::<ID3D11BlendState1Vtbl>(), 36);
        assert_eq!(size_of::<ID3D11RasterizerState1Vtbl>(), 36);
        assert_eq!(size_of::<ID3DDeviceContextStateVtbl>(), 28);
        assert_eq!(size_of::<ID3D11DeviceContext1Vtbl>(), 536);
        assert_eq!(size_of::<ID3D11Device1Vtbl>(), 200);
        assert_eq!(size_of::<ID3DUserDefinedAnnotationVtbl>(), 28);
    }
}

#[test]
#[cfg(feature = "dxgi1_4")]
fn check_d3d11_1_vtable_sizes_extra() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID3D11VideoContext1Vtbl>(), 632);
        assert_eq!(size_of::<ID3D11VideoDevice1Vtbl>(), 192);
        assert_eq!(size_of::<ID3D11VideoProcessorEnumerator1Vtbl>(), 112);
    } else {
        assert_eq!(size_of::<ID3D11VideoContext1Vtbl>(), 316);
        assert_eq!(size_of::<ID3D11VideoDevice1Vtbl>(), 96);
        assert_eq!(size_of::<ID3D11VideoProcessorEnumerator1Vtbl>(), 56);
    }
}
