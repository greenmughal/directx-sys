use com_rs::{IUnknown, Unknown};
use libc::c_void;
use winapi::{BOOL, DWORD, HANDLE, HDC, HMODULE, HRESULT, HWND, INT,
             LARGE_INTEGER, RECT, REFGUID, REFIID, UINT, UINT64};

use super::enums::*;
use super::structs::*;

com_interface! {
    struct IDXGIObject: IUnknown {
        iid: IID_IDXGIOBJECT {
            0xAEC22FB8, 0x76F3, 0x4639,
            0x9B, 0xE0, 0x28, 0xEB, 0x43, 0xA6, 0x7A, 0x2E
        },
        vtable: IDXGIObjectVtbl
    }

    trait DXGIObject: Unknown {
        fn set_private_data(
            name: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT,
        fn set_private_data_interface(
            name: REFGUID,
            unknown: *const IUnknown) -> HRESULT,
        fn get_private_data(
            name: REFGUID,
            data_size: *const UINT,
            data: *const c_void) -> HRESULT,
        fn get_parent(
            iid: REFIID,
            parent: *mut *mut c_void) -> HRESULT
    }
}

com_interface! {
    struct IDXGIDeviceSubObject: IDXGIObject, IUnknown {
        iid: IID_IDXGIDEVICESUBOBJECT {
            0x3D3E0379, 0xF9DE, 0x4D58,
            0xBB, 0x6C, 0x18, 0xD6, 0x29, 0x92, 0xF1, 0xA6
        },
        vtable: IDXGIDeviceSubObjectVtbl
    }

    trait DXGIDeviceSubObject: DXGIObject, Unknown {
        fn get_device(
            iid: REFIID,
            device: *mut *mut c_void) -> HRESULT
    }
}

com_interface! {
    struct IDXGIResource: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGIRESOURCE {
            0x035F3AB4, 0x482E, 0x4E50,
            0xB4, 0x1F, 0x8A, 0x7F, 0x8B, 0xD8, 0x96, 0x0B
        },
        vtable: IDXGIResourceVtbl
    }

    trait DXGIResource: DXGIDeviceSubObject, DXGIObject, Unknown {
        fn get_shared_handle(shared_handle: *mut HANDLE) -> HRESULT,
        fn get_usage(usage: *mut Usage) -> HRESULT,
        fn set_eviction_priority(eviction_priority: UINT) -> HRESULT,
        fn get_eviction_priority(eviction_priority: *mut UINT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIKeyedMutex: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGIKEYEDMUTEX {
            0x9D8E1289, 0xD7B3, 0x465F,
            0x81, 0x26, 0x25, 0x0E, 0x34, 0x9A, 0xF8, 0x5D
        },
        vtable: IDXGIKeyedMutexVtbl
    }

    trait DXGIKeyedMutex: DXGIDeviceSubObject, DXGIObject, Unknown {
        fn acquire_sync(
            key: UINT64,
            milliseconds: DWORD) -> HRESULT,
        fn release_sync(key: UINT64) -> HRESULT
    }
}

com_interface! {
    struct IDXGISurface: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISURFACE {
            0xCAFCB56C, 0x6AC3, 0x4889,
            0xBF, 0x47, 0x9E, 0x23, 0xBB, 0xD2, 0x60, 0xEC
        },
        vtable: IDXGISurfaceVtbl
    }

    trait DXGISurface: DXGIDeviceSubObject, DXGIObject, Unknown {
        fn get_desc(desc: *mut SurfaceDesc) -> HRESULT,
        fn map(
            locked_rect: *mut MappedRect,
            flags: MapFlags) -> HRESULT,
        fn unmap() -> HRESULT
    }
}

com_interface! {
    struct IDXGISurface1: IDXGISurface, IDXGIDeviceSubObject, IDXGIObject,
                          IUnknown {
        iid: IID_IDXGISURFACE1 {
            0x4AE63092, 0x6327, 0x4C1B,
            0x80, 0xAE, 0xBF, 0xE1, 0x2E, 0xA3, 0x2B, 0x86
        },
        vtable: IDXGISurface1Vtbl
    }

    trait DXGISurface1: DXGISurface, DXGIDeviceSubObject, DXGIObject, Unknown {
        fn get_dc(
            discard: BOOL,
            hdc: *mut HDC) -> HRESULT,
        fn release_dc(dirty_rect: *const RECT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIAdapter: IDXGIObject, IUnknown {
        iid: IID_IDXGIADAPTER {
            0x2411E7E1, 0x12AC, 0x4CCF,
            0xBD, 0x14, 0x97, 0x98, 0xE8, 0x53, 0x4D, 0xC0
        },
        vtable: IDXGIAdapterVtbl
    }

    trait DXGIAdapter: DXGIObject, Unknown {
        fn enum_outputs(
            index: UINT,
            output: *mut *mut IDXGIOutput) -> HRESULT,
        fn get_desc(desc: *mut AdapterDesc) -> HRESULT,
        fn check_interface_support(
            interface_name: REFGUID,
            umd_version: *mut LARGE_INTEGER) -> HRESULT
    }
}

com_interface! {
    struct IDXGIOutput: IDXGIObject, IUnknown {
        iid: IID_IDXGIOUTPUT {
            0xAE02EEDB, 0xC735, 0x4690,
            0x8D, 0x52, 0x5A, 0x8D, 0xC2, 0x02, 0x13, 0xAA
        },
        vtable: IDXGIOutputVtbl
    }

    trait DXGIOutput: DXGIObject, Unknown {
        fn get_desc(desc: *mut OutputDesc) -> HRESULT,
        fn get_display_mode_list(
            format: Format,
            flags: DisplayModeFlag,
            num_modes: *mut UINT,
            desc: *mut ModeDesc) -> HRESULT,
        fn find_closest_matching_mode(
            mode_to_match: *const ModeDesc,
            closest_match: *mut ModeDesc,
            concerned_device: *const IUnknown) -> HRESULT,
        fn wait_for_vblank() -> HRESULT,
        fn take_ownership(
            device: *const IUnknown,
            exclusive: BOOL) -> HRESULT,
        fn release_ownership() -> (),
        fn get_gamma_control_capabilities(
            caps: *mut GammaControlCapabilities) -> HRESULT,
        fn set_gamma_control(array: *const GammaControl) -> HRESULT,
        fn get_gamma_control(array: *mut GammaControl) -> HRESULT,
        fn set_display_surface(scanout_surface: *const IDXGISurface) -> HRESULT,
        fn get_display_surface_data(
            destination: *const IDXGISurface) -> HRESULT,
        fn get_frame_statistics(stats: *mut FrameStatistics) -> HRESULT
    }
}

com_interface! {
    struct IDXGISwapChain: IDXGIDeviceSubObject, IDXGIObject, IUnknown {
        iid: IID_IDXGISWAPCHAIN {
            0x310D36A0, 0xD2E7, 0x4C0A,
            0xAA, 0x04, 0x6A, 0x9D, 0x23, 0xB8, 0x88, 0x6A
        },
        vtable: IDXGISwapChainVtbl
    }

    trait DXGISwapChain: DXGIDeviceSubObject, DXGIObject, Unknown {
        fn present(
            sync_interval: UINT,
            flags: PresentFlags) -> HRESULT,
        fn get_buffer(
            buffer: UINT,
            iid: REFIID,
            surface: *mut *mut c_void) -> HRESULT,
        fn set_fullscreen_state(
            fullscreen: BOOL,
            target: *const IDXGIOutput) -> HRESULT,
        fn get_fullscreen_state(
            fullscreen: *mut BOOL,
            target: *mut *mut IDXGIOutput) -> HRESULT,
        fn get_desc(desc: *mut SwapChainDesc) -> HRESULT,
        fn resize_buffers(
            buffer_count: UINT,
            width: UINT,
            height: UINT,
            new_format: Format,
            swap_chain_flags: SwapChainFlag) -> HRESULT,
        fn resize_target(new_target_parameters: *const ModeDesc) -> HRESULT,
        fn get_containing_output(output: *mut *mut IDXGIOutput) -> HRESULT,
        fn get_frame_statistics(stats: *mut FrameStatistics) -> HRESULT,
        fn get_last_present_count(last_present_count: *mut UINT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIFactory: IDXGIObject, IUnknown {
        iid: IID_IDXGIFACTORY {
            0x7B7166EC, 0x21C7, 0x44AE,
            0xB2, 0x1A, 0xC9, 0xAE, 0x32, 0x1A, 0xE3, 0x69
        },
        vtable: IDXGIFactoryVtbl
    }

    trait DXGIFactory: DXGIObject, Unknown {
        fn enum_adapters(
            index: UINT,
            adapter: *mut *mut IDXGIAdapter) -> HRESULT,
        fn make_window_association(
            window_handle: HWND,
            flags: WindowAssociationFlags) -> HRESULT,
        fn get_window_association(window_handle: *mut HWND) -> HRESULT,
        fn create_swap_chain(
            device: *const IUnknown,
            desc: *const SwapChainDesc,
            swapchain: *mut *mut IDXGISwapChain) -> HRESULT,
        fn create_software_adapter(
            module: HMODULE,
            adapter: *mut *mut IDXGIAdapter) -> HRESULT
    }
}

com_interface! {
    struct IDXGIDevice: IDXGIObject, IUnknown {
        iid: IID_IDXGIDEVICE {
            0x54EC77FA, 0x1377, 0x44E6,
            0x8C, 0x32, 0x88, 0xFD, 0x5F, 0x44, 0xC8, 0x4C
        },
        vtable: IDXGIDeviceVtbl
    }

    trait DXGIDevice: DXGIObject, Unknown {
        fn get_adapter(adapter: *mut *mut IDXGIAdapter) -> HRESULT,
        fn create_surface(
            desc: *const SurfaceDesc,
            num_surfaces: UINT,
            usage: Usage,
            shared_resource: *const SharedResource,
            surface: *mut *mut IDXGISurface) -> HRESULT,
        fn query_resource_residency(
            resources: *const *const IUnknown,
            residency_status: *mut Residency,
            num_resources: UINT) -> HRESULT,
        fn set_gpu_thread_priority(priority: INT) -> HRESULT,
        fn get_gpu_thread_priority(priority: *mut INT) -> HRESULT
    }
}

com_interface! {
    struct IDXGIFactory1: IDXGIFactory, IDXGIObject, IUnknown {
        iid: IID_IDXGIFACTORY1 {
            0x770AAE78, 0xF26F, 0x4DBA,
            0xA8, 0x29, 0x25, 0x3C, 0x83, 0xD1, 0xB3, 0x87
        },
        vtable: IDXGIFactory1Vtbl
    }

    trait DXGIFactory1: DXGIFactory, DXGIObject, Unknown {
        fn enum_adapters1(
            index: UINT,
            adapter: *mut *mut IDXGIAdapter1) -> HRESULT,
        fn is_current() -> BOOL
    }
}

com_interface! {
    struct IDXGIAdapter1: IDXGIAdapter, IDXGIObject, IUnknown {
        iid: IID_IDXGIADAPTER1 {
            0x29038F61, 0x3839, 0x4626,
            0x91, 0xFD, 0x08, 0x68, 0x79, 0x01, 0x1A, 0x05
        },
        vtable: IDXGIAdapter1Vtbl
    }

    trait DXGIAdapter1: DXGIAdapter, DXGIObject, Unknown {
        fn get_desc1(desc: *mut AdapterDesc1) -> HRESULT
    }
}

com_interface! {
    struct IDXGIDevice1: IDXGIDevice, IDXGIObject, IUnknown {
        iid: IID_IDXGIDEVICE1 {
            0x77DB970F, 0x6276, 0x48BA,
            0xBA, 0x28, 0x07, 0x01, 0x43, 0xB4, 0x39, 0x2C
        },
        vtable: IDXGIDevice1Vtbl
    }

    trait DXGIDevice1: DXGIDevice, DXGIObject, Unknown {
        fn set_maximum_frame_latency(max_latency: UINT) -> HRESULT,
        fn get_maximum_frame_latency(max_latency: *mut UINT) -> HRESULT
    }
}

#[test]
fn check_dxgi_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<IDXGIAdapterVtbl>(), 80);
        assert_eq!(size_of::<IDXGIAdapter1Vtbl>(), 88);
        assert_eq!(size_of::<IDXGIDeviceVtbl>(), 96);
        assert_eq!(size_of::<IDXGIDevice1Vtbl>(), 112);
        assert_eq!(size_of::<IDXGIDeviceSubObjectVtbl>(), 64);
        assert_eq!(size_of::<IDXGIFactoryVtbl>(), 96);
        assert_eq!(size_of::<IDXGIFactory1Vtbl>(), 112);
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
        assert_eq!(size_of::<IDXGIDeviceVtbl>(), 48);
        assert_eq!(size_of::<IDXGIDevice1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGIDeviceSubObjectVtbl>(), 32);
        assert_eq!(size_of::<IDXGIFactoryVtbl>(), 48);
        assert_eq!(size_of::<IDXGIFactory1Vtbl>(), 56);
        assert_eq!(size_of::<IDXGIKeyedMutexVtbl>(), 40);
        assert_eq!(size_of::<IDXGIObjectVtbl>(), 28);
        assert_eq!(size_of::<IDXGIOutputVtbl>(), 76);
        assert_eq!(size_of::<IDXGIResourceVtbl>(), 48);
        assert_eq!(size_of::<IDXGISurfaceVtbl>(), 44);
        assert_eq!(size_of::<IDXGISurface1Vtbl>(), 52);
        assert_eq!(size_of::<IDXGISwapChainVtbl>(), 72);
    }
}