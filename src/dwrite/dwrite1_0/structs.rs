use winapi::{BOOL, FLOAT, INT16, INT32, UINT16, UINT32, WCHAR};

use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontMetrics {
    pub design_units_per_em: UINT16,
    pub ascent: UINT16,
    pub descent: UINT16,
    pub line_gap: INT16,
    pub cap_height: UINT16,
    pub x_height: UINT16,
    pub underline_position: INT16,
    pub underline_thickness: UINT16,
    pub strikethrough_position: INT16,
    pub strikethrough_thickness: UINT16
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlyphMetrics {
    pub left_side_bearing: INT32,
    pub advance_width: UINT32,
    pub right_side_bearing: INT32,
    pub top_side_bearing: INT32,
    pub advance_height: UINT32,
    pub bottom_side_bearing: INT32,
    pub vertical_origin_y: INT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphOffset {
    pub advance_offset: FLOAT,
    pub ascender_offset: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix {
    pub m11: FLOAT,
    pub m12: FLOAT,
    pub m21: FLOAT,
    pub m22: FLOAT,
    pub dx: FLOAT,
    pub dy: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextRange {
    pub start_position: UINT32,
    pub length: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontFeature {
    pub name_tag: FontFeatureTag,
    pub parameter: UINT32
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct TypographicFeatures {
    pub features: *mut FontFeature,
    pub feature_count: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Trimming {
    pub granularity: TrimmingGranularity,
    pub delimiter: UINT32,
    pub delimiter_count: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScriptAnalysis {
    pub script: UINT16,
    pub shapes: ScriptShapes
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineBreakpoint {
    bits: u8
}

bitfields! {
    LineBreakpoint.bits: u8 {
        (0, 2 => struct BreakCondition, break_condition_before,
                                        set_break_condition_before),
        (2, 2 => struct BreakCondition, break_condition_after,
                                        set_break_condition_after),
        (4, 1 => bool, is_whitespace, set_is_whitespace),
        (5, 1 => bool, is_soft_hyphen, set_is_soft_hyphen),
        (6, 2 => u8, padding, set_padding)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapingTextProperties {
    bits: u16
}

bitfields! {
    ShapingTextProperties.bits: u16 {
        (0, 1 => bool, is_shaped_alone, set_is_shaped_alone)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShapingGlyphProperties {
    bits: u16
}

bitfields! {
    ShapingGlyphProperties.bits: u16 {
        (0, 4 => u16, justification, set_justification),
        (4, 1 => bool, is_cluster_start, set_is_cluster_start),
        (5, 1 => bool, is_diacritic, set_is_diacritic),
        (6, 1 => bool, is_zero_width_space, set_is_zero_width_space)
    }
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct GlyphRun {
    pub font_face: *const super::IDWriteFontFace,
    pub font_em_size: FLOAT,
    pub glyph_count: UINT32,
    pub glyph_indices: *const UINT16,
    pub glyph_advances: *const FLOAT,
    pub glyph_offsets: *const GlyphOffset,
    pub is_sideways: BOOL,
    pub bidi_level: UINT32
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct GlyphRunDescription {
    pub locale_name: *const WCHAR,
    pub string: *const WCHAR,
    pub string_length: UINT32,
    pub cluster_map: *const UINT16,
    pub text_position: UINT32
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Underline {
    pub width: FLOAT,
    pub thickness: FLOAT,
    pub offset: FLOAT,
    pub run_height: FLOAT,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: *const WCHAR,
    pub measuring_mode: MeasuringMode
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct Strikethrough {
    pub width: FLOAT,
    pub thickness: FLOAT,
    pub offset: FLOAT,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: *const WCHAR,
    pub measuring_mode: MeasuringMode
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineMetrics {
    pub length: UINT32,
    pub trailing_whitespace_length: UINT32,
    pub newline_length: UINT32,
    pub height: FLOAT,
    pub baseline: FLOAT,
    pub is_trimmed: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClusterMetrics {
    pub width: FLOAT,
    pub length: UINT16,
    bits: u16
}

bitfields! {
    ClusterMetrics.bits: u16 {
        (0, 1 => bool, can_wrap_line_after, set_can_wrap_line_after),
        (1, 1 => bool, is_whitespace, set_is_whitespace),
        (2, 1 => bool, is_newline, set_is_newline),
        (3, 1 => bool, is_soft_hyphen, set_is_soft_hyphen),
        (4, 1 => bool, is_right_to_left, set_is_right_to_left)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextMetrics {
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub width_including_trailing_whitespace: FLOAT,
    pub height: FLOAT,
    pub layout_width: FLOAT,
    pub layout_height: FLOAT,
    pub max_bidi_reordering_depth: UINT32,
    pub line_count: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InlineObjectMetrics {
    pub width: FLOAT,
    pub height: FLOAT,
    pub baseline: FLOAT,
    pub supports_sideways: BOOL
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OverhangMetrics {
    pub left: FLOAT,
    pub top: FLOAT,
    pub right: FLOAT,
    pub bottom: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HitTestMetrics {
    pub text_position: UINT32,
    pub length: UINT32,
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub height: FLOAT,
    pub bidi_level: UINT32,
    pub is_text: BOOL,
    pub is_trimmed: BOOL
}

#[test]
fn check_dwrite_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<ClusterMetrics>(), 8);
    assert_eq!(size_of::<FontFeature>(), 8);
    assert_eq!(size_of::<FontMetrics>(), 20);
    assert_eq!(size_of::<FontSimulations>(), 4);
    assert_eq!(size_of::<GlyphMetrics>(), 28);
    assert_eq!(size_of::<GlyphOffset>(), 8);
    assert_eq!(size_of::<HitTestMetrics>(), 36);
    assert_eq!(size_of::<InlineObjectMetrics>(), 16);
    assert_eq!(size_of::<LineBreakpoint>(), 1);
    assert_eq!(size_of::<LineMetrics>(), 24);
    assert_eq!(size_of::<Matrix>(), 24);
    assert_eq!(size_of::<OverhangMetrics>(), 16);
    assert_eq!(size_of::<ScriptAnalysis>(), 8);
    assert_eq!(size_of::<ScriptShapes>(), 4);
    assert_eq!(size_of::<ShapingGlyphProperties>(), 2);
    assert_eq!(size_of::<ShapingTextProperties>(), 2);
    assert_eq!(size_of::<TextMetrics>(), 36);
    assert_eq!(size_of::<TextRange>(), 8);
    assert_eq!(size_of::<Trimming>(), 12);

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<GlyphRun>(), 48);
        assert_eq!(size_of::<GlyphRunDescription>(), 40);
        assert_eq!(size_of::<Strikethrough>(), 40);
        assert_eq!(size_of::<TypographicFeatures>(), 16);
        assert_eq!(size_of::<Underline>(), 40);
    } else {
        assert_eq!(size_of::<GlyphRun>(), 32);
        assert_eq!(size_of::<GlyphRunDescription>(), 20);
        assert_eq!(size_of::<Strikethrough>(), 28);
        assert_eq!(size_of::<TypographicFeatures>(), 8);
        assert_eq!(size_of::<Underline>(), 32);
    }
}