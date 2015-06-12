use com_rs::{IUnknown, Unknown};
use libc::c_void;
use winapi::{BOOL, DWORD, HANDLE, HRESULT, HWND, LPCWSTR, LUID, RECT, REFIID,
             SECURITY_ATTRIBUTES, UINT};

use super::super::dxgi1_0::*;

use super::enums::*;
use super::structs::*;

com_interface! {
    struct IDXGIDisplayControl: IUnknown {
        iid: IID_IDXGIDISPLAYCONTROL {
            0xEA9DBF1A, 0xC88E, 0x4486,
            0x85, 0x4A, 0x98, 0xAA, 0x01, 0x38, 0xF3, 0x0C
        },
        vtable: IDXGIDisplayControlVtbl
    }

    trait DXGIDisplayControl: Unknown {
        fn is_stereo_enabled() -> BOOL,
        fn set_stereo_enabled(enabled: BOOL) -> ()
    }
}

com_interface! {
    struct IDXGIOutputDuplication: IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUTDUPLICATION {
            0x191CFAC3, 0xA341, 0x470D,
            0xB2, 0x6E, 0xA8, 0x64, 0xF4, 0x28, 0x31, 0x9C
        },
        vtable: IDXGIOutputDuplicationVtbl
    }

    trait DXGIOutputDuplication: DXGIObject, Unknown {
        fn get_desc(desc: *mut OutDuplDesc) -> (),
        fn acquire_next_frame(
            timeout_in_milliseconds: UINT,
            frame_info: *mut OutDuplFrameInfo,
            desktop_resource: *mut *mut IDXGIResource) -> HRESULT,
        fn get_frame_dirty_rects(
            dirty_rects_buffer_size: UINT,
            dirty_rects_buffer: *mut RECT,
            dirty_rects_buffer_size_required: *mut UINT) -> HRESULT,
        fn get_frame_move_rects(
            move_rects_buffer_size: UINT,
            move_rects_buffer: *mut OutDuplMoveRect,
            move_rects_buffer_size_required: *mut UINT) -> HRESULT,
        fn get_frame_pointer_shape(
            pointer_shape_buffer_size: UINT,
            pointer_shape_buffer: *mut c_void,
            pointer_shape_buffer_size_required: *mut UINT,
            pointer_shape_info: *mut OutDuplPointerShapeInfo) -> HRESULT,
        fn map_desktop_surface(locked_rect: *mut MappedRect) -> HRESULT,
        fn un_map_desktop_surface() -> HRESULT,
        fn release_frame() -> HRESULT
    }
}

com_interface! {
    struct IDXGISurface2: IDXGISurface1, IDXGISurface, IDXGIDeviceSubObject,
                          IDXGIObject, IUnknown {
        iid: IID_IDXGISURFACE2 {
            0xABA496DD, 0xB617, 0x4CB8,
            0xA8, 0x66, 0xBC, 0x44, 0xD7, 0xEB, 0x1F, 0xA2
        },
        vtable: IDXGISurface2Vtbl
    }

    trait DXGISurface2: DXGISurface1, DXGISurface, DXGIDeviceSubObject,
                        DXGIObject, Unknown {
        fn get_resource(
            iid: REFIID,
            parent_resource: *mut *mut c_void,
            subresource_index: UINT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIResource1: IDXGIResource, IDXGIDeviceSubObject, IDXGIObject,
                           IUnknown {
        iid: IID_IDXGIRESOURCE1 {
            0x30961379, 0x4609, 0x4A41,
            0x99, 0x8E, 0x54, 0xFE, 0x56, 0x7E, 0xE0, 0xC1
        },
        vtable: IDXGIResource1Vtbl
    }

    trait DXGIResource1: DXGIResource, DXGIDeviceSubObject, DXGIObject,
                         Unknown {
        fn create_subresource_surface(
            index: UINT,
            surface: *mut *mut IDXGISurface2) -> HRESULT,
        fn create_shared_handle(
            attributes: *const SECURITY_ATTRIBUTES,
            access: DWORD,
            name: LPCWSTR,
            handle: *mut HANDLE) -> HRESULT
    }
}

com_interface! {
    struct IDXGIDevice2: IDXGIDevice1, IDXGIDevice, IDXGIObject, IUnknown {
        iid: IID_IDXGIDEVICE2 {
            0x05008617, 0xFBFD, 0x4051,
            0xA7, 0x90, 0x14, 0x48, 0x84, 0xB4, 0xF6, 0xA9
        },
        vtable: IDXGIDevice2Vtbl
    }

    trait DXGIDevice2: DXGIDevice1, DXGIDevice, DXGIObject, Unknown {
        fn offer_resources(
            num_resources: UINT,
            resources: *const *const IDXGIResource,
            priority: OfferResourcePriority) -> HRESULT,
        fn reclaim_resources(
            num_resources: UINT,
            resources: *const *const IDXGIResource,
            discarded: *mut BOOL) -> HRESULT,
        fn enqueue_set_event(event: HANDLE) -> HRESULT
    }
}

com_interface! {
    struct IDXGISwapChain1: IDXGISwapChain, IDXGIDeviceSubObject, IDXGIObject,
                            IUnknown {
        iid: IID_IDXGISWAPCHAIN1 {
            0x790A45F7, 0x0D42, 0x4876,
            0x98, 0x3A, 0x0A, 0x55, 0xCF, 0xE6, 0xF4, 0xAA
        },
        vtable: IDXGISwapChain1Vtbl
    }

    trait DXGISwapChain1: DXGISwapChain, DXGIDeviceSubObject, DXGIObject,
                          Unknown {
        fn get_desc1(desc: *mut SwapChainDesc1) -> HRESULT,
        fn get_fullscreen_desc(desc: *mut SwapChainFullscreenDesc) -> HRESULT,
        fn get_hwnd(hwnd: *mut HWND) -> HRESULT,
        fn get_core_window(iid: REFIID, unknown: *mut *mut c_void) -> HRESULT,
        fn present1(
            sync_interval: UINT,
            present_flags: PresentFlags,
            present_parameters: *const PresentParameters) -> HRESULT,
        fn is_temporary_mono_supported() -> BOOL,
        fn get_restrict_to_output(
            restrict_to_output: *mut *mut IDXGIOutput) -> HRESULT,
        fn set_background_color(color: *const RGBA) -> HRESULT,
        fn get_background_color(color: *mut RGBA) -> HRESULT,
        fn set_rotation(rotation: RotationMode) -> HRESULT,
        fn get_rotation(rotation: *mut RotationMode) -> HRESULT
    }
}

com_interface! {
    struct IDXGIFactory2: IDXGIFactory1, IDXGIFactory, IDXGIObject, IUnknown {
        iid: IID_IDXGIFACTORY2 {
            0x50C83A1C, 0xE072, 0x4C48,
            0x87, 0xB0, 0x36, 0x30, 0xFA, 0x36, 0xA6, 0xD0
        },
        vtable: IDXGIFactory2Vtbl
    }

    trait DXGIFactory2: DXGIFactory1, DXGIFactory, DXGIObject, Unknown {
        fn is_windowed_stereo_enabled() -> BOOL,
        fn create_swap_chain_for_hwnd(
            device: *const IUnknown,
            hwnd: HWND,
            desc: *const SwapChainDesc1,
            fullscreen_desc: *const SwapChainFullscreenDesc,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
        fn create_swap_chain_for_core_window(
            device: *const IUnknown,
            window: *const IUnknown,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT,
        fn get_shared_resource_adapter_luid(
            resource: HANDLE,
            luid: *mut LUID) -> HRESULT,
        fn register_stereo_status_window(
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD) -> HRESULT,
        fn register_stereo_status_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT,
        fn unregister_stereo_status(cookie: DWORD) -> (),
        fn register_occlusion_status_window(
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD) -> HRESULT,
        fn register_occlusion_status_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT,
        fn unregister_occlusion_status(cookie: DWORD) -> (),
        fn create_swap_chain_for_composition(
            device: *const IUnknown,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT
    }
}

com_interface! {
    struct IDXGIAdapter2: IDXGIAdapter1, IDXGIAdapter, IDXGIObject, IUnknown {
        iid: IID_IDXGIADAPTER2 {
            0x0AA1AE0A, 0xFA0E, 0x4B84,
            0x86, 0x44, 0xE0, 0x5F, 0xF8, 0xE5, 0xAC, 0xB5
        },
        vtable: IDXGIAdapter2Vtbl
    }

    trait DXGIAdapter2: DXGIAdapter1, DXGIAdapter, DXGIObject, Unknown {
        fn get_desc2(desc: *mut AdapterDesc2) -> HRESULT
    }
}

com_interface! {
    struct IDXGIOutput1: IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUT1 {
            0x00CDDEA8, 0x939B, 0x4B83,
            0xA3, 0x40, 0xA6, 0x85, 0x22, 0x66, 0x66, 0xCC
        },
        vtable: IDXGIOutput1Vtbl
    }

    trait DXGIOutput1: DXGIOutput, DXGIObject, Unknown {
        fn get_display_mode_list1(
            format: Format,
            flags: DisplayModeFlag,
            num_modes: *const UINT,
            desc: *mut ModeDesc1) -> HRESULT,
        fn find_closest_matching_mode1(
            mode_to_match: *const ModeDesc1,
            closest_match: *mut ModeDesc1,
            concerned_device: *const IUnknown) -> HRESULT,
        fn get_display_surface_data1(
            destination: *const IDXGIResource) -> HRESULT,
        fn duplicate_output(
            device: *const IUnknown,
            output_duplication: *mut *mut IDXGIOutputDuplication) -> HRESULT
    }
}

#[test]
fn check_dxgi1_2_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<IDXGIAdapter2Vtbl>(), 96);
        assert_eq!(size_of::<IDXGIDevice2Vtbl>(), 136);
        assert_eq!(size_of::<IDXGIDisplayControlVtbl>(), 40);
        assert_eq!(size_of::<IDXGIFactory2Vtbl>(), 200);
        assert_eq!(size_of::<IDXGIOutput1Vtbl>(), 184);
        assert_eq!(size_of::<IDXGIOutputDuplicationVtbl>(), 120);
        assert_eq!(size_of::<IDXGIResource1Vtbl>(), 112);
        assert_eq!(size_of::<IDXGISurface2Vtbl>(), 112);
        assert_eq!(size_of::<IDXGISwapChain1Vtbl>(), 232);
    } else {
        assert_eq!(size_of::<IDXGIAdapter2Vtbl>(), 48);
        assert_eq!(size_of::<IDXGIDevice2Vtbl>(), 68);
        assert_eq!(size_of::<IDXGIDisplayControlVtbl>(), 20);
        assert_eq!(size_of::<IDXGIFactory2Vtbl>(), 100);
        assert_eq!(size_of::<IDXGIOutput1Vtbl>(), 92);
        assert_eq!(size_of::<IDXGIOutputDuplicationVtbl>(), 60);
        assert_eq!(size_of::<IDXGIResource1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGISurface2Vtbl>(), 56);
        assert_eq!(size_of::<IDXGISwapChain1Vtbl>(), 116);
    }
}