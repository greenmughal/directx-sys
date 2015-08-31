bitflags! {
    #[repr(C)]
    flags TileMappingFlag: u32 {
        const TILE_MAPPING_NO_OVERWRITE = 0x00000001,
    }
}

bitflags! {
    #[repr(C)]
    flags TileRangeFlag: u32 {
        const TILE_RANGE_NULL = 0x00000001,
        const TILE_RANGE_SKIP = 0x00000002,
        const TILE_RANGE_REUSE_SINGLE_TILE = 0x00000004,
    }
}

bitflags! {
    #[repr(C)]
    flags CheckMultisampleQualityLevelsFlag: u32 {
        const CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE = 0x00000001,
    }
}

bitflags! {
    #[repr(C)]
    flags TileCopyFlag: u32 {
        const TILE_COPY_NO_OVERWRITE = 0x00000001,
        const TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE = 0x00000002,
        const TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER = 0x00000004,
    }
}
