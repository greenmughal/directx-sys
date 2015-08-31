use winapi::{BOOL, UINT, UINT8, UINT16};

#[repr(C)]
#[derive(Debug)]
pub struct TiledResourceCoordinate {
    pub x: UINT,
    pub y: UINT,
    pub z: UINT,
    pub subresource: UINT,
}

#[repr(C)]
#[derive(Debug)]
pub struct TileRegionSize {
    pub num_tiles: UINT,
    pub use_box: BOOL,
    pub width: UINT,
    pub height: UINT16,
    pub depth: UINT16,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubresourceTiling {
    pub width_in_tiles: UINT,
    pub height_in_tiles: UINT16,
    pub depth_in_tiles: UINT16,
    pub start_tile_index_in_overall_resource: UINT,
}

#[repr(C)]
#[derive(Debug)]
pub struct TileShape {
    pub width_in_texels: UINT,
    pub height_in_texels: UINT,
    pub depth_in_texels: UINT,
}

#[repr(C)]
#[derive(Debug)]
pub struct PackedMipDesc {
    pub num_standard_mips: UINT8,
    pub num_packed_mips: UINT8,
    pub num_tiles_for_packed_mips: UINT,
    pub start_tile_index_in_overall_resource: UINT,
}

#[test]
fn check_d3d11_2_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<TiledResourceCoordinate>(), 16);
    assert_eq!(size_of::<TileRegionSize>(), 16);
    assert_eq!(size_of::<SubresourceTiling>(), 12);
    assert_eq!(size_of::<TileShape>(), 12);
    assert_eq!(size_of::<PackedMipDesc>(), 12);
}
