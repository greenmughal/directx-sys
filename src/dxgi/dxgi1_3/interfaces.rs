use com_rs::{IUnknown, Unknown};
use winapi::{BOOL, HANDLE, HRESULT, RECT, UINT};

use super::super::dxgi1_0::*;
use super::super::dxgi1_2::*;

use super::enums::*;
use super::structs::*;

com_interface! {
    struct IDXGIDevice3: IDXGIDevice2, IDXGIDevice1, IDXGIDevice, IDXGIObject,
                         IUnknown {
        iid: IID_IDXGIDEVICE3 {
            0x6007896C, 0x3244, 0x4AFD,
            0xBF, 0x18, 0xA6, 0xD3, 0xBE, 0xDA, 0x50, 0x23
        },
        vtable: IDXGIDevice3Vtbl
    }

    trait DXGIDevice3: DXGIDevice2, DXGIDevice1, DXGIDevice, DXGIObject,
                       Unknown {
        fn trim() -> ()
    }
}

com_interface! {
    struct IDXGISwapChain2: IDXGISwapChain1, IDXGISwapChain,
                            IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISWAPCHAIN2 {
            0xA8BE2AC4, 0x199F, 0x4946,
            0xB3, 0x31, 0x79, 0x59, 0x9F, 0xB9, 0x8D, 0xE7
        },
        vtable: IDXGISwapChain2Vtbl
    }

    trait DXGISwapChain2: DXGISwapChain1, DXGISwapChain, DXGIDeviceSubObject,
                          DXGIObject, Unknown {
        fn set_source_size(
            width: UINT,
            height: UINT) -> HRESULT,
        fn get_source_size(
            width: *mut UINT,
            height: *mut UINT) -> HRESULT,
        fn set_maximum_frame_latency(max_latency: UINT) -> HRESULT,
        fn get_maximum_frame_latency(max_latency: *mut UINT) -> HRESULT,
        fn get_frame_latency_waitable_object() -> HANDLE,
        fn set_matrix_transform(matrix: *const Matrix3X2F) -> HRESULT,
        fn get_matrix_transform(matrix: *mut Matrix3X2F) -> HRESULT
    }
}

com_interface! {
    struct IDXGIOutput2: IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUT2 {
            0x595E39D1, 0x2724, 0x4663,
            0x99, 0xB1, 0xDA, 0x96, 0x9D, 0xE2, 0x83, 0x64
        },
        vtable: IDXGIOutput2Vtbl
    }

    trait DXGIOutput2: DXGIOutput1, DXGIOutput, DXGIObject, Unknown {
        fn supports_overlays() -> BOOL
    }
}

com_interface! {
    struct IDXGIFactory3: IDXGIFactory2, IDXGIFactory1, IDXGIFactory,
                          IDXGIObject, IUnknown {
        iid: IID_IDXGIFACTORY3 {
            0x25483823, 0xCD46, 0x4C7D,
            0x86, 0xCA, 0x47, 0xAA, 0x95, 0xB8, 0x37, 0xBD
        },
        vtable: IDXGIFactory3Vtbl
    }

    trait DXGIFactory3: DXGIFactory2, DXGIFactory1, DXGIFactory, DXGIObject,
                        Unknown {
        fn get_creation_flags() -> CreateFactoryFlags
    }
}

com_interface! {
    struct IDXGIDecodeSwapChain: IUnknown {
        iid: IID_IDXGIDECODESWAPCHAIN {
            0x2633066B, 0x4514, 0x4C7A,
            0x8F, 0xD8, 0x12, 0xEA, 0x98, 0x05, 0x9D, 0x18
        },
        vtable: IDXGIDecodeSwapChainVtbl
    }

    trait DXGIDecodeSwapChain: Unknown {
        fn present_buffer(
            buffer_to_present: UINT,
            sync_interval: UINT,
            flags: PresentFlags) -> HRESULT,
        fn set_source_rect(rect: *const RECT) -> HRESULT,
        fn set_target_rect(rect: *const RECT) -> HRESULT,
        fn set_dest_size(
            width: UINT,
            height: UINT) -> HRESULT,
        fn get_source_rect(rect: *mut RECT) -> HRESULT,
        fn get_target_rect(rect: *mut RECT) -> HRESULT,
        fn get_dest_size(
            width: *mut UINT,
            height: *mut UINT) -> HRESULT,
        fn set_color_space(color_space: MultiPlaneOverlayYCbCrFlags) -> HRESULT,
        fn get_color_space() -> MultiPlaneOverlayYCbCrFlags
    }
}

com_interface! {
    struct IDXGIFactoryMedia: IUnknown {
        iid: IID_IDXGIFACTORYMEDIA {
            0x41E7D1F2, 0xA591, 0x4F7B,
            0xA2, 0xE5, 0xFA, 0x9C, 0x84, 0x3E, 0x1C, 0x12
        },
        vtable: IDXGIFactoryMediaVtbl
    }

    trait DXGIFactoryMedia: Unknown {
        fn create_swap_chain_for_composition_surface_handle(
            device: *const IUnknown,
            surface: HANDLE,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
        fn create_decode_swap_chain_for_composition_surface_handle(
            device: *const IUnknown,
            surface: HANDLE,
            desc: *const DecodeSwapChainDesc,
            yuv_decode_buffers: *const IDXGIResource,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGIDecodeSwapChain) -> HRESULT
    }
}

com_interface! {
    struct IDXGISwapChainMedia: IUnknown {
        iid: IID_IDXGISWAPCHAINMEDIA {
            0xDD95B90B, 0xF05F, 0x4F6A,
            0xBD, 0x65, 0x25, 0xBF, 0xB2, 0x64, 0xBD, 0x84
        },
        vtable: IDXGISwapChainMediaVtbl
    }

    trait DXGISwapChainMedia: Unknown {
        fn get_frame_statistics_media(
            stats: *mut FrameStatisticsMedia) -> HRESULT,
        fn set_present_duration(duration: UINT) -> HRESULT,
        fn check_present_duration_support(
            desired_present_duration: UINT,
            closest_smaller_present_duration: *mut UINT,
            closest_larger_present_duration: *mut UINT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIOutput3: IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject,
                         IUnknown {
        iid: IID_IDXGIOUTPUT3 {
            0x8A6BB301, 0x7E7E, 0x41F4,
            0xA8, 0xE0, 0x5B, 0x32, 0xF7, 0xF9, 0x9B, 0x18
        },
        vtable: IDXGIOutput3Vtbl
    }

    trait DXGIOutput3: DXGIOutput2, DXGIOutput1, DXGIOutput, DXGIObject,
                       Unknown {
        fn check_overlay_support(
            format: Format,
            concerned_device: *const IUnknown,
            flags: *mut OverlaySupportFlag) -> HRESULT
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
