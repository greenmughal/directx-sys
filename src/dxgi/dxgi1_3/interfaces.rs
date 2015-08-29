use com_rs::IUnknown;
use winapi::{BOOL, HANDLE, HRESULT, RECT, UINT};

use super::super::dxgi1_0::*;
use super::super::dxgi1_2::*;

use super::enums::*;
use super::structs::*;

iid!(IID_IDXGIDEVICE3 =
    0x6007896c, 0x3244, 0x4afd, 0xbf, 0x18, 0xa6, 0xd3, 0xbe, 0xda, 0x50, 0x23);
com_interface! {
    interface IDXGIDevice3: IDXGIDevice2, IDXGIDevice1, IDXGIDevice,
                            IDXGIObject, IUnknown {
        iid: IID_IDXGIDEVICE3,
        vtable: IDXGIDevice3Vtbl,
        fn trim() -> ();
    }
}

iid!(IID_IDXGISWAPCHAIN2 =
    0xa8be2ac4, 0x199f, 0x4946, 0xb3, 0x31, 0x79, 0x59, 0x9f, 0xb9, 0x8d, 0xe7);
com_interface! {
    interface IDXGISwapChain2: IDXGISwapChain1, IDXGISwapChain,
                               IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISWAPCHAIN2,
        vtable: IDXGISwapChain2Vtbl,
        fn set_source_size(
            width: UINT,
            height: UINT) -> HRESULT;
        fn get_source_size(
            width: *mut UINT,
            height: *mut UINT) -> HRESULT;
        fn set_maximum_frame_latency(max_latency: UINT) -> HRESULT;
        fn get_maximum_frame_latency(max_latency: *mut UINT) -> HRESULT;
        fn get_frame_latency_waitable_object() -> HANDLE;
        fn set_matrix_transform(matrix: *const Matrix3X2F) -> HRESULT;
        fn get_matrix_transform(matrix: *mut Matrix3X2F) -> HRESULT;
    }
}

iid!(IID_IDXGIOUTPUT2 =
    0x595e39d1, 0x2724, 0x4663, 0x99, 0xb1, 0xda, 0x96, 0x9d, 0xe2, 0x83, 0x64);
com_interface! {
    interface IDXGIOutput2: IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUT2,
        vtable: IDXGIOutput2Vtbl,
        fn supports_overlays() -> BOOL;
    }
}

iid!(IID_IDXGIFACTORY3 =
    0x25483823, 0xcd46, 0x4c7d, 0x86, 0xca, 0x47, 0xaa, 0x95, 0xb8, 0x37, 0xbd);
com_interface! {
    interface IDXGIFactory3: IDXGIFactory2, IDXGIFactory1, IDXGIFactory,
                             IDXGIObject, IUnknown {
        iid: IID_IDXGIFACTORY3,
        vtable: IDXGIFactory3Vtbl,
        fn get_creation_flags() -> CreateFactoryFlags;
    }
}

iid!(IID_IDXGIDECODESWAPCHAIN =
    0x2633066b, 0x4514, 0x4c7a, 0x8f, 0xd8, 0x12, 0xea, 0x98, 0x05, 0x9d, 0x18);
com_interface! {
    interface IDXGIDecodeSwapChain: IUnknown {
        iid: IID_IDXGIDECODESWAPCHAIN,
        vtable: IDXGIDecodeSwapChainVtbl,
        fn present_buffer(
            buffer_to_present: UINT,
            sync_interval: UINT,
            flags: PresentFlags) -> HRESULT;
        fn set_source_rect(rect: *const RECT) -> HRESULT;
        fn set_target_rect(rect: *const RECT) -> HRESULT;
        fn set_dest_size(width: UINT, height: UINT) -> HRESULT;
        fn get_source_rect(rect: *mut RECT) -> HRESULT;
        fn get_target_rect(rect: *mut RECT) -> HRESULT;
        fn get_dest_size(width: *mut UINT, height: *mut UINT) -> HRESULT;
        fn set_color_space(color_space: MultiPlaneOverlayYCbCrFlags) -> HRESULT;
        fn get_color_space() -> MultiPlaneOverlayYCbCrFlags;
    }
}

iid!(IID_IDXGIFACTORYMEDIA =
    0x41e7d1f2, 0xa591, 0x4f7b, 0xa2, 0xe5, 0xfa, 0x9c, 0x84, 0x3e, 0x1c, 0x12);
com_interface! {
    interface IDXGIFactoryMedia: IUnknown {
        iid: IID_IDXGIFACTORYMEDIA,
        vtable: IDXGIFactoryMediaVtbl,
        fn create_swap_chain_for_composition_surface_handle(
            device: *const IUnknown,
            surface: HANDLE,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT;
        fn create_decode_swap_chain_for_composition_surface_handle(
            device: *const IUnknown,
            surface: HANDLE,
            desc: *const DecodeSwapChainDesc,
            yuv_decode_buffers: *const IDXGIResource,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGIDecodeSwapChain) -> HRESULT;
    }
}

iid!(IID_IDXGISWAPCHAINMEDIA =
    0xdd95b90b, 0xf05f, 0x4f6a, 0xbd, 0x65, 0x25, 0xbf, 0xb2, 0x64, 0xbd, 0x84);
com_interface! {
    interface IDXGISwapChainMedia: IUnknown {
        iid: IID_IDXGISWAPCHAINMEDIA,
        vtable: IDXGISwapChainMediaVtbl,
        fn get_frame_statistics_media(
            stats: *mut FrameStatisticsMedia) -> HRESULT;
        fn set_present_duration(duration: UINT) -> HRESULT;
        fn check_present_duration_support(
            desired_present_duration: UINT,
            closest_smaller_present_duration: *mut UINT,
            closest_larger_present_duration: *mut UINT) -> HRESULT;
    }
}

iid!(IID_IDXGIOUTPUT3 =
    0x8a6bb301, 0x7e7e, 0x41F4, 0xa8, 0xe0, 0x5b, 0x32, 0xf7, 0xf9, 0x9b, 0x18);
com_interface! {
    interface IDXGIOutput3: IDXGIOutput2, IDXGIOutput1, IDXGIOutput,
                            IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUT3,
        vtable: IDXGIOutput3Vtbl,
        fn check_overlay_support(
            format: Format,
            concerned_device: *const IUnknown,
            flags: *mut OverlaySupportFlag) -> HRESULT;
    }
}

#[test]
fn check_dxgi1_3_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<IDXGIDecodeSwapChainVtbl>(), 96);
        assert_eq!(size_of::<IDXGIDevice3Vtbl>(), 144);
        assert_eq!(size_of::<IDXGIFactory3Vtbl>(), 208);
        assert_eq!(size_of::<IDXGIFactoryMediaVtbl>(), 40);
        assert_eq!(size_of::<IDXGIOutput2Vtbl>(), 192);
        assert_eq!(size_of::<IDXGIOutput3Vtbl>(), 200);
        assert_eq!(size_of::<IDXGISwapChain2Vtbl>(), 288);
        assert_eq!(size_of::<IDXGISwapChainMediaVtbl>(), 48);
    } else {
        assert_eq!(size_of::<IDXGIDecodeSwapChainVtbl>(), 48);
        assert_eq!(size_of::<IDXGIDevice3Vtbl>(), 72);
        assert_eq!(size_of::<IDXGIFactory3Vtbl>(), 104);
        assert_eq!(size_of::<IDXGIFactoryMediaVtbl>(), 20);
        assert_eq!(size_of::<IDXGIOutput2Vtbl>(), 96);
        assert_eq!(size_of::<IDXGIOutput3Vtbl>(), 100);
        assert_eq!(size_of::<IDXGISwapChain2Vtbl>(), 144);
        assert_eq!(size_of::<IDXGISwapChainMediaVtbl>(), 24);
    }
}
