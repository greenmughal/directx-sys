use com_rs::{IID, IUnknown};
use winapi::{BOOL, HANDLE, HRESULT, RECT, UINT};

use super::super::dxgi1_0::*;
use super::super::dxgi1_2::*;

use super::enums::*;
use super::structs::*;

#[link(name = "dxguid")]
extern {
    static IID_IDXGIDecodeSwapChain: IID;
    static IID_IDXGIDevice3: IID;
    static IID_IDXGIFactory3: IID;
    static IID_IDXGIFactoryMedia: IID;
    static IID_IDXGIOutput2: IID;
    static IID_IDXGIOutput3: IID;
    static IID_IDXGISwapChain2: IID;
    static IID_IDXGISwapChainMedia: IID;
}

com_interface! {
    interface IDXGIDevice3: IDXGIDevice2, IDXGIDevice1, IDXGIDevice,
                            IDXGIObject, IUnknown {
        iid: IID_IDXGIDevice3,
        vtable: IDXGIDevice3Vtbl,
        fn trim() -> ();
    }
}

com_interface! {
    interface IDXGISwapChain2: IDXGISwapChain1, IDXGISwapChain,
                               IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISwapChain2,
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

com_interface! {
    interface IDXGIOutput2: IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOutput2,
        vtable: IDXGIOutput2Vtbl,
        fn supports_overlays() -> BOOL;
    }
}

com_interface! {
    interface IDXGIFactory3: IDXGIFactory2, IDXGIFactory1, IDXGIFactory,
                             IDXGIObject, IUnknown {
        iid: IID_IDXGIFactory3,
        vtable: IDXGIFactory3Vtbl,
        fn get_creation_flags() -> CreateFactoryFlags;
    }
}

com_interface! {
    interface IDXGIDecodeSwapChain: IUnknown {
        iid: IID_IDXGIDecodeSwapChain,
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

com_interface! {
    interface IDXGIFactoryMedia: IUnknown {
        iid: IID_IDXGIFactoryMedia,
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

com_interface! {
    interface IDXGISwapChainMedia: IUnknown {
        iid: IID_IDXGISwapChainMedia,
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

com_interface! {
    interface IDXGIOutput3: IDXGIOutput2, IDXGIOutput1, IDXGIOutput,
                            IDXGIObject, IUnknown {
        iid: IID_IDXGIOutput3,
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
