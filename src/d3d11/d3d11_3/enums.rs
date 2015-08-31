#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextType {
    All,
    ThreeD,
    Compute,
    Copy,
    Video,
}

impl Default for ContextType {
    fn default() -> ContextType { ContextType::All }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureLayout {
    Undefined,
    RowMajor,
    StandardSwizzle64K,
}

impl Default for TextureLayout {
    fn default() -> TextureLayout { TextureLayout::Undefined }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConservativeRasterizationMode {
    Off,
    On,
}

impl Default for ConservativeRasterizationMode {
    fn default() -> Self { ConservativeRasterizationMode::Off }
}
