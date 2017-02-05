use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, DWORD, HANDLE, HRESULT, HWND, LPCWSTR, LUID, RECT,
             SECURITY_ATTRIBUTES, UINT};

use super::super::dxgi1_0::*;

use super::enums::*;
use super::structs::*;

extern {
    static IID_IDXGIAdapter2: IID;
    static IID_IDXGIDevice2: IID;
    static IID_IDXGIDisplayControl: IID;
    static IID_IDXGIFactory2: IID;
    static IID_IDXGIOutput1: IID;
    static IID_IDXGIOutputDuplication: IID;
    static IID_IDXGIResource1: IID;
    static IID_IDXGISurface2: IID;
    static IID_IDXGISwapChain1: IID;
}

com_interface! {
    interface IDXGIDisplayControl: IUnknown {
        iid: IID_IDXGIDisplayControl,
        vtable: IDXGIDisplayControlVtbl,
        fn is_stereo_enabled() -> BOOL;
        fn set_stereo_enabled(enabled: BOOL) -> ();
    }
}

com_interface! {
    interface IDXGIOutputDuplication: IDXGIObject, IUnknown {
        iid: IID_IDXGIOutputDuplication,
        vtable: IDXGIOutputDuplicationVtbl,
        fn get_desc(desc: *mut OutDuplDesc) -> ();
        fn acquire_next_frame(
            timeout_in_milliseconds: UINT,
            frame_info: *mut OutDuplFrameInfo,
            desktop_resource: *mut *mut IDXGIResource) -> HRESULT;
        fn get_frame_dirty_rects(
            dirty_rects_buffer_size: UINT,
            dirty_rects_buffer: *mut RECT,
            dirty_rects_buffer_size_required: *mut UINT) -> HRESULT;
        fn get_frame_move_rects(
            move_rects_buffer_size: UINT,
            move_rects_buffer: *mut OutDuplMoveRect,
            move_rects_buffer_size_required: *mut UINT) -> HRESULT;
        fn get_frame_pointer_shape(
            pointer_shape_buffer_size: UINT,
            pointer_shape_buffer: *mut c_void,
            pointer_shape_buffer_size_required: *mut UINT,
            pointer_shape_info: *mut OutDuplPointerShapeInfo) -> HRESULT;
        fn map_desktop_surface(locked_rect: *mut MappedRect) -> HRESULT;
        fn un_map_desktop_surface() -> HRESULT;
        fn release_frame() -> HRESULT;
    }
}

com_interface! {
    interface IDXGISurface2: IDXGISurface1, IDXGISurface, IDXGIDeviceSubObject,
                             IDXGIObject, IUnknown {
        iid: IID_IDXGISurface2,
        vtable: IDXGISurface2Vtbl,
        fn get_resource(
            iid: &IID,
            parent_resource: *mut *mut c_void,
            subresource_index: UINT) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIResource1: IDXGIResource, IDXGIDeviceSubObject, IDXGIObject,
                              IUnknown {
        iid: IID_IDXGIResource1,
        vtable: IDXGIResource1Vtbl,
        fn create_subresource_surface(
            index: UINT,
            surface: *mut *mut IDXGISurface2) -> HRESULT;
        fn create_shared_handle(
            attributes: *const SECURITY_ATTRIBUTES,
            access: DWORD,
            name: LPCWSTR,
            handle: *mut HANDLE) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIDevice2: IDXGIDevice1, IDXGIDevice, IDXGIObject, IUnknown {
        iid: IID_IDXGIDevice2,
        vtable: IDXGIDevice2Vtbl,
        fn offer_resources(
            num_resources: UINT,
            resources: *const *const IDXGIResource,
            priority: OfferResourcePriority) -> HRESULT;
        fn reclaim_resources(
            num_resources: UINT,
            resources: *const *const IDXGIResource,
            discarded: *mut BOOL) -> HRESULT;
        fn enqueue_set_event(event: HANDLE) -> HRESULT;
    }
}

com_interface! {
    interface IDXGISwapChain1: IDXGISwapChain, IDXGIDeviceSubObject,
                               IDXGIObject, IUnknown {
        iid: IID_IDXGISwapChain1,
        vtable: IDXGISwapChain1Vtbl,
        fn get_desc1(desc: *mut SwapChainDesc1) -> HRESULT;
        fn get_fullscreen_desc(desc: *mut SwapChainFullscreenDesc) -> HRESULT;
        fn get_hwnd(hwnd: *mut HWND) -> HRESULT;
        fn get_core_window(iid: &IID, unknown: *mut *mut c_void) -> HRESULT;
        fn present1(
            sync_interval: UINT,
            present_flags: PresentFlags,
            present_parameters: *const PresentParameters) -> HRESULT;
        fn is_temporary_mono_supported() -> BOOL;
        fn get_restrict_to_output(
            restrict_to_output: *mut *mut IDXGIOutput) -> HRESULT;
        fn set_background_color(color: *const RGBA) -> HRESULT;
        fn get_background_color(color: *mut RGBA) -> HRESULT;
        fn set_rotation(rotation: RotationMode) -> HRESULT;
        fn get_rotation(rotation: *mut RotationMode) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIFactory2: IDXGIFactory1, IDXGIFactory, IDXGIObject,
                             IUnknown {
        iid: IID_IDXGIFactory2,
        vtable: IDXGIFactory2Vtbl,
        fn is_windowed_stereo_enabled() -> BOOL;
        fn create_swap_chain_for_hwnd(
            device: *const IUnknown,
            hwnd: HWND,
            desc: *const SwapChainDesc1,
            fullscreen_desc: *const SwapChainFullscreenDesc,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT;
        fn create_swap_chain_for_core_window(
            device: *const IUnknown,
            window: *const IUnknown,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT;
        fn get_shared_resource_adapter_luid(
            resource: HANDLE,
            luid: *mut LUID) -> HRESULT;
        fn register_stereo_status_window(
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD) -> HRESULT;
        fn register_stereo_status_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT;
        fn unregister_stereo_status(cookie: DWORD) -> ();
        fn register_occlusion_status_window(
            window_handle: HWND,
            msg: UINT,
            cookie: *mut DWORD) -> HRESULT;
        fn register_occlusion_status_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT;
        fn unregister_occlusion_status(cookie: DWORD) -> ();
        fn create_swap_chain_for_composition(
            device: *const IUnknown,
            desc: *const SwapChainDesc1,
            restrict_to_output: *const IDXGIOutput,
            swapchain: *mut *mut IDXGISwapChain1) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIAdapter2: IDXGIAdapter1, IDXGIAdapter, IDXGIObject,
                             IUnknown {
        iid: IID_IDXGIAdapter2,
        vtable: IDXGIAdapter2Vtbl,
        fn get_desc2(desc: *mut AdapterDesc2) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIOutput1: IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOutput1,
        vtable: IDXGIOutput1Vtbl,
        fn get_display_mode_list1(
            format: Format,
            flags: DisplayModeFlag,
            num_modes: *const UINT,
            desc: *mut ModeDesc1) -> HRESULT;
        fn find_closest_matching_mode1(
            mode_to_match: *const ModeDesc1,
            closest_match: *mut ModeDesc1,
            concerned_device: *const IUnknown) -> HRESULT;
        fn get_display_surface_data1(
            destination: *const IDXGIResource) -> HRESULT;
        fn duplicate_output(
            device: *const IUnknown,
            output_duplication: *mut *mut IDXGIOutputDuplication) -> HRESULT;
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
