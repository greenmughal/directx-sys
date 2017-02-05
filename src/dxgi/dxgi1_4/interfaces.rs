use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{DWORD, HANDLE, HRESULT, LUID, UINT, UINT64};

use super::super::dxgi1_3::*;
use super::super::dxgi1_2::*;
use super::super::dxgi1_0::*;

use super::enums::*;
use super::structs::*;

extern {
    static IID_IDXGISwapChain3: IID;
    static IID_IDXGIOutput4: IID;
    static IID_IDXGIFactory4: IID;
    static IID_IDXGIAdapter3: IID;
}

com_interface! {
    interface IDXGISwapChain3: IDXGISwapChain2, IDXGISwapChain1, IDXGISwapChain,
                               IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISwapChain3,
        vtable: IDXGISwapChain3Vtbl,
        fn get_current_back_buffer_index() -> UINT;
        fn check_color_space_support(
            color_space: ColorSpaceType,
            color_space_support: *mut SwapChainColorSpaceSupportFlags)
            -> HRESULT;
        fn set_color_space1(color_space: ColorSpaceType) -> HRESULT;
        fn resize_buffers1(
            buffer_count: UINT,
            width: UINT,
            height: UINT,
            format: Format,
            swap_chain_flags: SwapChainFlag,
            creation_node_mask: *const UINT,
            present_queue: *const *const IUnknown) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIOutput4: IDXGIOutput3, IDXGIOutput2, IDXGIOutput1,
                            IDXGIOutput, IDXGIObject, IUnknown {
        iid: IID_IDXGIOutput4,
        vtable: IDXGIOutput4Vtbl,
        fn check_overlay_color_space_support(
            format: Format,
            color_space: ColorSpaceType,
            concerned_device: *const IUnknown,
            flags: *mut OverlayColorSpaceSupportFlags) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIFactory4: IDXGIFactory3, IDXGIFactory2, IDXGIFactory1,
                             IDXGIFactory, IDXGIObject, IUnknown {
        iid: IID_IDXGIFactory4,
        vtable: IDXGIFactory4Vtbl,
        fn enum_adapter_by_luid(
            adapter_luid: LUID,
            iid: &IID,
            adapter: *mut *mut c_void) -> HRESULT;
        fn enum_warp_adapter(
            iid: &IID,
            adapter: *mut *mut c_void) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIAdapter3: IDXGIAdapter2, IDXGIAdapter1, IDXGIAdapter,
                             IDXGIObject, IUnknown {
        iid: IID_IDXGIAdapter3,
        vtable: IDXGIAdapter3Vtbl,
        fn register_hardware_content_protection_teardown_status_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT;
        fn unregister_hardware_content_protection_teardown_status(
            cookie: DWORD) -> ();
        fn query_video_memory_info(
            node_index: UINT,
            memory_segment_group: MemorySegmentGroup,
            video_memory_info: *mut QueryVideoMemoryInfo) -> HRESULT;
        fn set_video_memory_reservation(
            node_index: UINT,
            memory_segment_group: MemorySegmentGroup,
            reservation: UINT64) -> HRESULT;
        fn register_video_memory_budget_change_notification_event(
            event: HANDLE,
            cookie: *mut DWORD) -> HRESULT;
        fn unregister_video_memory_budget_change_notification(
            cookie: DWORD) -> ();
    }
}

#[test]
fn check_dxgi1_4_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<IDXGISwapChain3Vtbl>(), 320);
        assert_eq!(size_of::<IDXGIOutput4Vtbl>(), 208);
        assert_eq!(size_of::<IDXGIFactory4Vtbl>(), 224);
        assert_eq!(size_of::<IDXGIAdapter3Vtbl>(), 144);
    } else {
        assert_eq!(size_of::<IDXGISwapChain3Vtbl>(), 160);
        assert_eq!(size_of::<IDXGIOutput4Vtbl>(), 104);
        assert_eq!(size_of::<IDXGIFactory4Vtbl>(), 112);
        assert_eq!(size_of::<IDXGIAdapter3Vtbl>(), 72);
    }
}
