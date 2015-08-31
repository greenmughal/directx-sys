use std::os::raw::c_void;

use com_rs::IUnknown;
use winapi::{BOOL, HRESULT, INT, LPCWSTR, UINT, UINT64};

use super::super::d3d11_0::*;
use super::super::d3d11_1::*;

use super::enums::*;
use super::structs::*;

use dxgi;

iid!(IID_ID3D11DEVICECONTEXT2 =
    0x420d5b32, 0xb90c, 0x4da4, 0xbe, 0xf0, 0x35, 0x9f, 0x6a, 0x24, 0xa8, 0x3a);
com_interface! {
    interface ID3D11DeviceContext2: ID3D11DeviceContext1, ID3D11DeviceContext,
                                    ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DEVICECONTEXT2,
        vtable: ID3D11DeviceContext2Vtbl,
        fn update_tile_mappings(
            tiled_resource: *const ID3D11Resource,
            num_tiled_resource_regions: UINT,
            tiled_resource_region_start_coordinates: *const TiledResourceCoordinate,
            tile_pool: *const ID3D11Buffer,
            num_ranges: UINT,
            range_flags: *const TileRangeFlag,
            tile_pool_start_offsets: *const UINT,
            range_tile_counts: *const UINT,
            flags: TileMappingFlag) -> HRESULT;
        fn copy_tile_mappings(
            dest_tiled_resourced: *const ID3D11Resource,
            dest_region_start_coordinate: *const TiledResourceCoordinate,
            source_tiled_resource: *const ID3D11Resource,
            source_region_start_coordinate: *const TiledResourceCoordinate,
            tile_region_size: *const TileRegionSize,
            flags: TileMappingFlag) -> HRESULT;
        fn copy_tiles(
            tiled_resource: *const ID3D11Resource,
            tile_region_start_coordinate: *const TiledResourceCoordinate,
            tile_region_size: *const TileRegionSize,
            buffer: *const ID3D11Buffer,
            buffer_start_offset_in_bytes: UINT64,
            flags: TileCopyFlag) -> ();
        fn update_tiles(
            dest_tiled_resource: *const ID3D11Resource,
            dest_tile_region_start_coordinate: *const TiledResourceCoordinate,
            dest_tile_region_size: *const TileRegionSize,
            source_tile_data: *const c_void,
            flags: TileCopyFlag) -> ();
        fn resize_tile_pool(
            tile_pool: *const ID3D11Buffer,
            new_size_in_bytes: UINT64) -> HRESULT;
        fn tiled_resource_barrier(
            tiled_resource_or_view_access_before_barrier: *const ID3D11DeviceChild,
            tiled_resource_or_view_access_after_barrier: *const ID3D11DeviceChild) -> ();
        fn is_annotation_enabled() -> BOOL;
        fn set_marker_int(label: LPCWSTR, data: INT) -> ();
        fn begin_event_int(label: LPCWSTR, data: INT) -> ();
        fn end_event() -> ();
    }
}

iid!(IID_ID3D11DEVICE2 =
    0x9d06dffa, 0xd1e5, 0x4d07, 0x83, 0xa8, 0x1b, 0xb1, 0x23, 0xf2, 0xf8, 0x41);
com_interface! {
    interface ID3D11Device2: ID3D11Device1, ID3D11Device, IUnknown {
        iid: IID_ID3D11DEVICE2,
        vtable: ID3D11Device2Vtbl,
        fn get_immediate_context2(
            immediate_context: *mut *mut ID3D11DeviceContext2) -> ();
        fn create_deferred_context2(
            context_flags: UINT,
            deferred_context: *mut *mut ID3D11DeviceContext2) -> HRESULT;
        fn get_resource_tiling(
            tiled_resource: *const ID3D11Resource,
            num_tiles_for_entire_resource: *mut UINT,
            packed_mip_desc: *mut PackedMipDesc,
            standard_tile_shape_for_non_packed_mips: *mut TileShape,
            num_subresource_tilings: *mut UINT,
            first_subresource_tiling_to_get: UINT,
            subresource_tilings_for_non_packed_mips: *mut SubresourceTiling)
            -> ();
        fn check_multisample_quality_levels1(
            format: dxgi::Format,
            sample_count: UINT,
            flags: CheckMultisampleQualityLevelsFlag,
            num_quality_levels: *mut UINT) -> HRESULT;
    }
}

#[test]
fn check_d3d11_2_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID3D11DeviceContext2Vtbl>(), 1152);
        assert_eq!(size_of::<ID3D11Device2Vtbl>(), 432);
    } else {
        assert_eq!(size_of::<ID3D11DeviceContext2Vtbl>(), 576);
        assert_eq!(size_of::<ID3D11Device2Vtbl>(), 216);
    }
}
