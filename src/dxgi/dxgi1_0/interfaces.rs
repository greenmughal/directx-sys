use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, DWORD, GUID, HANDLE, HDC, HMODULE, HRESULT, HWND, INT,
             LARGE_INTEGER, LPCSTR, RECT, REFGUID, SIZE_T, UINT, UINT64};

use super::enums::*;
use super::structs::*;

#[link(name = "dxguid")]
extern {
    static IID_IDXGIAdapter: IID;
    static IID_IDXGIAdapter1: IID;
    static IID_IDXGIDevice: IID;
    static IID_IDXGIDevice1: IID;
    static IID_IDXGIDeviceSubObject: IID;
    static IID_IDXGIFactory: IID;
    static IID_IDXGIFactory1: IID;
    static IID_IDXGIKeyedMutex: IID;
    static IID_IDXGIObject: IID;
    static IID_IDXGIOutput: IID;
    static IID_IDXGIResource: IID;
    static IID_IDXGISurface: IID;
    static IID_IDXGISurface1: IID;
    static IID_IDXGISwapChain: IID;
}

com_interface! {
    interface IDXGIObject: IUnknown {
        iid: IID_IDXGIObject,
        vtable: IDXGIObjectVtbl,
        fn set_private_data(
            name: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT;
        fn set_private_data_interface(
            name: REFGUID,
            unknown: *const IUnknown) -> HRESULT;
        fn get_private_data(
            name: REFGUID,
            data_size: *const UINT,
            data: *const c_void) -> HRESULT;
        fn get_parent(iid: &IID, parent: *mut *mut c_void) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIDeviceSubObject: IDXGIObject, IUnknown {
        iid: IID_IDXGIDeviceSubObject,
        vtable: IDXGIDeviceSubObjectVtbl,
        fn get_device(iid: &IID, device: *mut *mut c_void) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIResource: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGIResource,
        vtable: IDXGIResourceVtbl,
        fn get_shared_handle(shared_handle: *mut HANDLE) -> HRESULT;
        fn get_usage(usage: *mut Usage) -> HRESULT;
        fn set_eviction_priority(eviction_priority: UINT) -> HRESULT;
        fn get_eviction_priority(eviction_priority: *mut UINT) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIKeyedMutex: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGIKeyedMutex,
        vtable: IDXGIKeyedMutexVtbl,
        fn acquire_sync(key: UINT64, milliseconds: DWORD) -> HRESULT;
        fn release_sync(key: UINT64) -> HRESULT;
    }
}

com_interface! {
    interface IDXGISurface: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISurface,
        vtable: IDXGISurfaceVtbl,
        fn get_desc(desc: *mut SurfaceDesc) -> HRESULT;
        fn map(locked_rect: *mut MappedRect, flags: MapFlags) -> HRESULT;
        fn unmap() -> HRESULT;
    }
}

com_interface! {
    interface IDXGISurface1: IDXGISurface, IDXGIDeviceSubObject, IDXGIObject,
                             IUnknown {
        iid: IID_IDXGISurface1,
        vtable: IDXGISurface1Vtbl,
        fn get_dc(discard: BOOL, hdc: *mut HDC) -> HRESULT;
        fn release_dc(dirty_rect: *const RECT) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIAdapter: IDXGIObject, IUnknown {
        iid: IID_IDXGIAdapter,
        vtable: IDXGIAdapterVtbl,
        fn enum_outputs(index: UINT, output: *mut *mut IDXGIOutput) -> HRESULT;
        fn get_desc(desc: *mut AdapterDesc) -> HRESULT;
        fn check_interface_support(
            interface_name: REFGUID,
            umd_version: *mut LARGE_INTEGER) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIOutput: IDXGIObject, IUnknown {
        iid: IID_IDXGIOutput,
        vtable: IDXGIOutputVtbl,
        fn get_desc(desc: *mut OutputDesc) -> HRESULT;
        fn get_display_mode_list(
            format: Format,
            flags: DisplayModeFlag,
            num_modes: *mut UINT,
            desc: *mut ModeDesc) -> HRESULT;
        fn find_closest_matching_mode(
            mode_to_match: *const ModeDesc,
            closest_match: *mut ModeDesc,
            concerned_device: *const IUnknown) -> HRESULT;
        fn wait_for_vblank() -> HRESULT;
        fn take_ownership(device: *const IUnknown, exclusive: BOOL) -> HRESULT;
        fn release_ownership() -> ();
        fn get_gamma_control_capabilities(
            caps: *mut GammaControlCapabilities) -> HRESULT;
        fn set_gamma_control(array: *const GammaControl) -> HRESULT;
        fn get_gamma_control(array: *mut GammaControl) -> HRESULT;
        fn set_display_surface(scanout_surface: *const IDXGISurface) -> HRESULT;
        fn get_display_surface_data(
            destination: *const IDXGISurface) -> HRESULT;
        fn get_frame_statistics(stats: *mut FrameStatistics) -> HRESULT;
    }
}

com_interface! {
    interface IDXGISwapChain: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISwapChain,
        vtable: IDXGISwapChainVtbl,
        fn present(sync_interval: UINT, flags: PresentFlags) -> HRESULT;
        fn get_buffer(
            buffer: UINT,
            iid: &IID,
            surface: *mut *mut c_void) -> HRESULT;
        fn set_fullscreen_state(
            fullscreen: BOOL,
            target: *const IDXGIOutput) -> HRESULT;
        fn get_fullscreen_state(
            fullscreen: *mut BOOL,
            target: *mut *mut IDXGIOutput) -> HRESULT;
        fn get_desc(desc: *mut SwapChainDesc) -> HRESULT;
        fn resize_buffers(
            buffer_count: UINT,
            width: UINT,
            height: UINT,
            new_format: Format,
            swap_chain_flags: SwapChainFlag) -> HRESULT;
        fn resize_target(new_target_parameters: *const ModeDesc) -> HRESULT;
        fn get_containing_output(output: *mut *mut IDXGIOutput) -> HRESULT;
        fn get_frame_statistics(stats: *mut FrameStatistics) -> HRESULT;
        fn get_last_present_count(last_present_count: *mut UINT) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIFactory: IDXGIObject, IUnknown {
        iid: IID_IDXGIFactory,
        vtable: IDXGIFactoryVtbl,
        fn enum_adapters(
            index: UINT,
            adapter: *mut *mut IDXGIAdapter) -> HRESULT;
        fn make_window_association(
            window_handle: HWND,
            flags: WindowAssociationFlags) -> HRESULT;
        fn get_window_association(window_handle: *mut HWND) -> HRESULT;
        fn create_swap_chain(
            device: *const IUnknown,
            desc: *const SwapChainDesc,
            swapchain: *mut *mut IDXGISwapChain) -> HRESULT;
        fn create_software_adapter(
            module: HMODULE,
            adapter: *mut *mut IDXGIAdapter) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIDevice: IDXGIObject, IUnknown {
        iid: IID_IDXGIDevice,
        vtable: IDXGIDeviceVtbl,
        fn get_adapter(adapter: *mut *mut IDXGIAdapter) -> HRESULT;
        fn create_surface(
            desc: *const SurfaceDesc,
            num_surfaces: UINT,
            usage: Usage,
            shared_resource: *const SharedResource,
            surface: *mut *mut IDXGISurface) -> HRESULT;
        fn query_resource_residency(
            resources: *const *const IUnknown,
            residency_status: *mut Residency,
            num_resources: UINT) -> HRESULT;
        fn set_gpu_thread_priority(priority: INT) -> HRESULT;
        fn get_gpu_thread_priority(priority: *mut INT) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIFactory1: IDXGIFactory, IDXGIObject, IUnknown {
        iid: IID_IDXGIFactory1,
        vtable: IDXGIFactory1Vtbl,
        fn enum_adapters1(
            index: UINT,
            adapter: *mut *mut IDXGIAdapter1) -> HRESULT;
        fn is_current() -> BOOL;
    }
}

com_interface! {
    interface IDXGIAdapter1: IDXGIAdapter, IDXGIObject, IUnknown {
        iid: IID_IDXGIAdapter1,
        vtable: IDXGIAdapter1Vtbl,
        fn get_desc1(desc: *mut AdapterDesc1) -> HRESULT;
    }
}

com_interface! {
    interface IDXGIDevice1: IDXGIDevice, IDXGIObject, IUnknown {
        iid: IID_IDXGIDevice1,
        vtable: IDXGIDevice1Vtbl,
        fn set_maximum_frame_latency(max_latency: UINT) -> HRESULT;
        fn get_maximum_frame_latency(max_latency: *mut UINT) -> HRESULT;
    }
}

iid!(IID_IDXGIINFOQUEUE =
    0xD67441C7, 0x672A, 0x476f, 0x9E, 0x82, 0xCD, 0x55, 0xB4, 0x49, 0x49, 0xCE);
com_interface! {
    interface IDXGIInfoQueue: IUnknown {
        iid: IID_IDXGIINFOQUEUE,
        vtable: IDXGIInfoQueueVtbl,
        fn set_message_count_limit(
            producer: GUID,
            message_count_limit: UINT64) -> HRESULT;
        fn clear_stored_messages(producer: GUID) -> ();
        fn get_message(
            producer: GUID,
            message_index: UINT64,
            message: *mut InfoQueueMessage,
            message_byte_length: *mut SIZE_T) -> HRESULT;
        fn get_num_stored_messages_allowed_by_retrieval_filters(
            producer: GUID) -> UINT64;
        fn get_num_stored_messages(producer: GUID) -> UINT64;
        fn get_num_messages_discarded_by_message_count_limit(
            producer: GUID) -> UINT64;
        fn get_message_count_limit(producer: GUID) -> UINT64;
        fn get_num_messages_allowed_by_storage_filter(producer: GUID) -> UINT64;
        fn get_num_messages_denied_by_storage_filter(producer: GUID) -> UINT64;
        fn add_storage_filter_entries(
            producer: GUID,
            filter: *const InfoQueueFilter) -> HRESULT;
        fn get_storage_filter(
            producer: GUID,
            filter: *mut InfoQueueFilter,
            filter_byte_length: *mut SIZE_T) -> HRESULT;
        fn clear_storage_filter(producer: GUID) -> ();
        fn push_empty_storage_filter(producer: GUID) -> HRESULT;
        fn push_deny_all_storage_filter(producer: GUID) -> HRESULT;
        fn push_copy_of_storage_filter(producer: GUID) -> HRESULT;
        fn push_storage_filter(
            producer: GUID,
            filter: *const InfoQueueFilter) -> HRESULT;
        fn pop_storage_filter(producer: GUID) -> ();
        fn get_storage_filter_stack_size(producer: GUID) -> UINT;
        fn add_retrieval_filter_entries(
            producer: GUID,
            filter: *const InfoQueueFilter) -> HRESULT;
        fn get_retrieval_filter(
            producer: GUID,
            filter: *mut InfoQueueFilter,
            filter_byte_length: *mut SIZE_T) -> HRESULT;
        fn clear_retrieval_filter(producer: GUID) -> ();
        fn push_empty_retrieval_filter(producer: GUID) -> HRESULT;
        fn push_deny_all_retrieval_filter(producer: GUID) -> HRESULT;
        fn push_copy_of_retrieval_filter(producer: GUID) -> HRESULT;
        fn push_retrieval_filter(
            producer: GUID,
            filter: *const InfoQueueFilter) -> HRESULT;
        fn pop_retrieval_filter(producer: GUID) -> ();
        fn get_retrieval_filter_stack_size(producer: GUID) -> UINT;
        fn add_message(
            producer: GUID,
            category: InfoQueueMessageCategory,
            severity: InfoQueueMessageSeverity,
            id: INT,
            description: LPCSTR) -> HRESULT;
        fn add_application_message(
            severity: InfoQueueMessageSeverity,
            description: LPCSTR) -> HRESULT;
        fn set_break_on_category(
            producer: GUID,
            category: InfoQueueMessageCategory,
            enable: BOOL) -> HRESULT;
        fn set_break_on_severity(
            producer: GUID,
            severity: InfoQueueMessageSeverity,
            enable: BOOL) -> HRESULT;
        fn set_break_on_id(
            producer: GUID,
            id: INT,
            enable: BOOL) -> HRESULT;
        fn get_break_on_category(
            producer: GUID,
            category: InfoQueueMessageCategory) -> BOOL;
        fn get_break_on_severity(
            producer: GUID,
            category: InfoQueueMessageSeverity) -> BOOL;
        fn get_break_on_id(
            producer: GUID,
            id: INT) -> BOOL;
        fn set_mute_debug_output(
            producer: GUID,
            mute: BOOL) -> ();
        fn get_mute_debug_output(producer: GUID) -> BOOL;
    }
}

iid!(IID_IDXGIDEBUG =
    0x119E7452, 0xDE9E, 0x40fe, 0x88, 0x06, 0x88, 0xF9, 0x0C, 0x12, 0xB4, 0x41);
com_interface! {
    interface IDXGIDebug: IUnknown {
        iid: IID_IDXGIDEBUG,
        vtable: IDXGIDebugVtbl,
        fn report_live_objects(
            api_id: GUID,
            flags: DebugRLOFlags) -> HRESULT;
    }
}

iid!(IID_IDXGIDEBUG1 =
    0xc5a05f0c, 0x16f2, 0x4adf, 0x9f, 0x4d, 0xa8, 0xc4, 0xd5, 0x8a, 0xc5, 0x50);
com_interface! {
    interface IDXGIDebug1: IDXGIDebug, IUnknown {
        iid: IID_IDXGIDEBUG1,
        vtable: IDXGIDebug1Vtbl,
        fn enable_leak_tracking_for_thread() -> ();
        fn disable_leak_tracking_for_thread() -> ();
        fn is_leak_tracking_enabled_for_thread() -> BOOL;
    }
}

#[test]
fn check_dxgi_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<IDXGIAdapterVtbl>(), 80);
        assert_eq!(size_of::<IDXGIAdapter1Vtbl>(), 88);
        assert_eq!(size_of::<IDXGIDebugVtbl>(), 32);
        assert_eq!(size_of::<IDXGIDebug1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGIDeviceVtbl>(), 96);
        assert_eq!(size_of::<IDXGIDevice1Vtbl>(), 112);
        assert_eq!(size_of::<IDXGIDeviceSubObjectVtbl>(), 64);
        assert_eq!(size_of::<IDXGIFactoryVtbl>(), 96);
        assert_eq!(size_of::<IDXGIFactory1Vtbl>(), 112);
        assert_eq!(size_of::<IDXGIInfoQueueVtbl>(), 320);
        assert_eq!(size_of::<IDXGIKeyedMutexVtbl>(), 80);
        assert_eq!(size_of::<IDXGIObjectVtbl>(), 56);
        assert_eq!(size_of::<IDXGIOutputVtbl>(), 152);
        assert_eq!(size_of::<IDXGIResourceVtbl>(), 96);
        assert_eq!(size_of::<IDXGISurfaceVtbl>(), 88);
        assert_eq!(size_of::<IDXGISurface1Vtbl>(), 104);
        assert_eq!(size_of::<IDXGISwapChainVtbl>(), 144);
    } else {
        assert_eq!(size_of::<IDXGIAdapterVtbl>(), 40);
        assert_eq!(size_of::<IDXGIAdapter1Vtbl>(), 44);
        assert_eq!(size_of::<IDXGIDebugVtbl>(), 16);
        assert_eq!(size_of::<IDXGIDebug1Vtbl>(), 28);
        assert_eq!(size_of::<IDXGIDeviceVtbl>(), 48);
        assert_eq!(size_of::<IDXGIDevice1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGIDeviceSubObjectVtbl>(), 32);
        assert_eq!(size_of::<IDXGIFactoryVtbl>(), 48);
        assert_eq!(size_of::<IDXGIFactory1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGIInfoQueueVtbl>(), 160);
        assert_eq!(size_of::<IDXGIKeyedMutexVtbl>(), 40);
        assert_eq!(size_of::<IDXGIObjectVtbl>(), 28);
        assert_eq!(size_of::<IDXGIOutputVtbl>(), 76);
        assert_eq!(size_of::<IDXGIResourceVtbl>(), 48);
        assert_eq!(size_of::<IDXGISurfaceVtbl>(), 44);
        assert_eq!(size_of::<IDXGISurface1Vtbl>(), 52);
        assert_eq!(size_of::<IDXGISwapChainVtbl>(), 72);
    }
}
