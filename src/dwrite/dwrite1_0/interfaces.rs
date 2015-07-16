use std::os::raw::c_void;

use com_rs::{IUnknown, Unknown};
use winapi::{BOOL, BYTE, COLORREF, FILETIME, FLOAT, HDC, HMONITOR, HRESULT,
             LOGFONTW, RECT, SIZE, UINT8, UINT16, UINT32, UINT64, WCHAR};

#[cfg(feature = "d2d")]
use d2d;

// Stub Direct2D types to use DirectWrite without it.
#[cfg(not(feature = "d2d"))]
mod d2d {
    use com_rs::IUnknown;
    pub type ID2D1SimplifiedGeometrySink = IUnknown;
}

use super::enums::*;
use super::structs::*;

com_interface! {
    struct IDWriteFontFileLoader: IUnknown {
        iid: IID_IDWRITEFONTFILELOADER {
            0x727CAD4E, 0xD6AF, 0x4C9E,
            0x8A, 0x08, 0xD6, 0x95, 0xB1, 0x1C, 0xAA, 0x49
        },
        vtable: IDWriteFontFileLoaderVtbl
    }

    trait DWriteFontFileLoader: Unknown {
        fn create_stream_from_key(
            font_file_reference_key: *const c_void,
            font_file_reference_key_size: UINT32,
            font_file_stream: *mut *mut IDWriteFontFileStream) -> HRESULT
    }
}

com_interface! {
    struct IDWriteLocalFontFileLoader: IDWriteFontFileLoader, IUnknown {
        iid: IID_IDWRITELOCALFONTFILELOADER {
            0xB2D9F3EC, 0xC9FE, 0x4A11,
            0xA2, 0xEC, 0xD8, 0x62, 0x08, 0xF7, 0xC0, 0xA2
        },
        vtable: IDWriteLocalFontFileLoaderVtbl
    }

    trait DWriteLocalFontFileLoader: DWriteFontFileLoader, Unknown {
        fn get_file_path_length_from_key(
            font_file_reference_key: *const c_void,
            font_file_reference_key_size: UINT32,
            file_path_length: *mut UINT32) -> HRESULT,
        fn get_file_path_from_key(
            font_file_reference_key: *const c_void,
            font_file_reference_key_size: UINT32,
            file_path: *mut WCHAR,
            file_path_size: UINT32) -> HRESULT,
        fn get_last_write_time_from_key(
            font_file_reference_key: *const c_void,
            font_file_reference_key_size: UINT32,
            last_write_time: *mut FILETIME) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontFileStream: IUnknown {
        iid: IID_IDWRITEFONTFILESTREAM {
            0x6D4865FE, 0x0AB8, 0x4D91,
            0x8F, 0x62, 0x5D, 0xD6, 0xBE, 0x34, 0xA3, 0xE0
        },
        vtable: IDWriteFontFileStreamVtbl
    }

    trait DWriteFontFileStream: Unknown {
        fn read_file_fragment(
            fragment_start: *mut *mut c_void,
            file_offset: UINT64,
            fragment_size: UINT64,
            fragment_context: *mut *mut c_void) -> HRESULT,
        fn release_file_fragment(fragment_context: *const c_void) -> (),
        fn get_file_size(file_size: *mut UINT64) -> HRESULT,
        fn get_last_write_time(last_write_time: *mut UINT64) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontFile: IUnknown {
        iid: IID_IDWRITEFONTFILE {
            0x739D886A, 0xCEF5, 0x47DC,
            0x87, 0x69, 0x1A, 0x8B, 0x41, 0xBE, 0xBB, 0xB0
        },
        vtable: IDWriteFontFileVtbl
    }

    trait DWriteFontFile: Unknown {
        fn get_reference_key(
            font_file_reference_key: *const *const c_void,
            font_file_reference_key_size: *mut UINT32) -> HRESULT,
        fn get_loader(
            font_file_loader: *mut *mut IDWriteFontFileLoader) -> HRESULT,
        fn analyze(
            is_supported_font_type: *mut BOOL,
            font_file_type: *mut FontFileType,
            font_face_type: *mut FontFaceType,
            number_of_faces: *mut UINT32) -> HRESULT
    }
}

com_interface! {
    struct IDWriteRenderingParams: IUnknown {
        iid: IID_IDWRITERENDERINGPARAMS {
            0x2F0DA53A, 0x2ADD, 0x47CD,
            0x82, 0xEE, 0xD9, 0xEC, 0x34, 0x68, 0x8E, 0x75
        },
        vtable: IDWriteRenderingParamsVtbl
    }

    trait DWriteRenderingParams: Unknown {
        fn get_gamma() -> FLOAT,
        fn get_enhanced_contrast() -> FLOAT,
        fn get_clear_type_level() -> FLOAT,
        fn get_pixel_geometry() -> PixelGeometry,
        fn get_rendering_mode() -> RenderingMode
    }
}

com_interface! {
    struct IDWriteFontFace: IUnknown {
        iid: IID_IDWRITEFONTFACE {
            0x5F49804D, 0x7024, 0x4D43,
            0xBF, 0xA9, 0xD2, 0x59, 0x84, 0xF5, 0x38, 0x49
        },
        vtable: IDWriteFontFaceVtbl
    }

    trait DWriteFontFace: Unknown {
        fn get_type() -> FontFaceType,
        fn get_files(
            number_of_files: *mut UINT32,
            font_files: *mut *mut IDWriteFontFile) -> HRESULT,
        fn get_index() -> UINT32,
        fn get_simulations() -> FontSimulations,
        fn is_symbol_font() -> BOOL,
        fn get_metrics(font_face_metrics: *mut FontMetrics) -> (),
        fn get_glyph_count() -> UINT16,
        fn get_design_glyph_metrics(
            glyph_indices: *const UINT16,
            glyph_count: UINT32,
            glyph_metrics: *mut GlyphMetrics,
            is_sideways: BOOL) -> HRESULT,
        fn get_glyph_indices(
            code_points: *const UINT32,
            code_point_count: UINT32,
            glyph_indices: *mut UINT16) -> HRESULT,
        fn try_get_font_table(
            open_type_table_tag: UINT32,
            table_data: *mut *mut c_void,
            table_size: *mut UINT32,
            table_context: *mut *mut c_void,
            exists: *mut BOOL) -> HRESULT,
        fn release_font_table(table_context: *const c_void) -> (),
        fn get_glyph_run_outline(
            em_size: FLOAT,
            glyph_indices: *const UINT16,
            glyph_advances: *const FLOAT,
            glyph_offsets: *const GlyphOffset,
            glyph_count: UINT32,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            geometry_sink: *const d2d::ID2D1SimplifiedGeometrySink) -> HRESULT,
        fn get_recommended_rendering_mode(
            em_size: FLOAT,
            pixels_per_dip: FLOAT,
            measuring_mode: MeasuringMode,
            rendering_params: *const IDWriteRenderingParams,
            rendering_mode: *mut RenderingMode) -> HRESULT,
        fn get_gdi_compatible_metrics(
            em_size: FLOAT,
            pixels_per_dip: FLOAT,
            transform: *const Matrix,
            font_face_metrics: *mut FontMetrics) -> HRESULT,
        fn get_gdi_compatible_glyph_metrics(
            em_size: FLOAT,
            pixels_per_dip: FLOAT,
            transform: *const Matrix,
            use_gdi_natural: BOOL,
            glyph_indices: *const UINT16,
            glyph_count: UINT32,
            glyph_metrics: *mut GlyphMetrics,
            is_sideways: BOOL) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontCollectionLoader: IUnknown {
        iid: IID_IDWRITEFONTCOLLECTIONLOADER {
            0xCCA920E4, 0x52F0, 0x492B,
            0xBF, 0xA8, 0x29, 0xC7, 0x2E, 0xE0, 0xA4, 0x68
        },
        vtable: IDWriteFontCollectionLoaderVtbl
    }

    trait DWriteFontCollectionLoader: Unknown {
        fn create_enumerator_from_key(
            factory: *const IDWriteFactory,
            collection_key: *const c_void,
            collection_key_size: UINT32,
            font_file_enumerator: *mut *mut IDWriteFontFileEnumerator)
            -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontFileEnumerator: IUnknown {
        iid: IID_IDWRITEFONTFILEENUMERATOR {
            0x72755049, 0x5FF7, 0x435D,
            0x83, 0x48, 0x4B, 0xE9, 0x7C, 0xFA, 0x6C, 0x7C
        },
        vtable: IDWriteFontFileEnumeratorVtbl
    }

    trait DWriteFontFileEnumerator: Unknown {
        fn move_next(has_current_file: *mut BOOL) -> HRESULT,
        fn get_current_font_file(
            font_file: *mut *mut IDWriteFontFile) -> HRESULT
    }
}

com_interface! {
    struct IDWriteLocalizedStrings: IUnknown {
        iid: IID_IDWRITELOCALIZEDSTRINGS {
            0x08256209, 0x099A, 0x4B34,
            0xB8, 0x6D, 0xC2, 0x2B, 0x11, 0x0E, 0x77, 0x71
        },
        vtable: IDWriteLocalizedStringsVtbl
    }

    trait DWriteLocalizedStrings: Unknown {
        fn get_count() -> UINT32,
        fn find_locale_name(
            locale_name: *const WCHAR,
            index: *mut UINT32,
            exists: *mut BOOL) -> HRESULT,
        fn get_locale_name_length(
            index: UINT32,
            length: *mut UINT32) -> HRESULT,
        fn get_locale_name(
            index: UINT32,
            locale_name: *mut WCHAR,
            size: UINT32) -> HRESULT,
        fn get_string_length(
            index: UINT32,
            length: *mut UINT32) -> HRESULT,
        fn get_string(
            index: UINT32,
            string_buffer: *mut WCHAR,
            size: UINT32) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontCollection: IUnknown {
        iid: IID_IDWRITEFONTCOLLECTION {
            0xA84CEE02, 0x3EEA, 0x4EEE,
            0xA8, 0x27, 0x87, 0xC1, 0xA0, 0x2A, 0x0F, 0xCC
        },
        vtable: IDWriteFontCollectionVtbl
    }

    trait DWriteFontCollection: Unknown {
        fn get_font_family_count() -> UINT32,
        fn get_font_family(
            index: UINT32,
            font_family: *mut *mut IDWriteFontFamily) -> HRESULT,
        fn find_family_name(
            family_name: *const WCHAR,
            index: *mut UINT32,
            exists: *mut BOOL) -> HRESULT,
        fn get_font_from_font_face(
            font_face: *const IDWriteFontFace,
            font: *mut *mut IDWriteFont) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontList: IUnknown {
        iid: IID_IDWRITEFONTLIST {
            0x1A0D8438, 0x1D97, 0x4EC1,
            0xAE, 0xF9, 0xA2, 0xFB, 0x86, 0xED, 0x6A, 0xCB
        },
        vtable: IDWriteFontListVtbl
    }

    trait DWriteFontList: Unknown {
        fn get_font_collection(
            font_collection: *mut *mut IDWriteFontCollection) -> HRESULT,
        fn get_font_count() -> UINT32,
        fn get_font(
            index: UINT32,
            font: *mut *mut IDWriteFont) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFontFamily: IDWriteFontList, IUnknown {
        iid: IID_IDWRITEFONTFAMILY {
            0xDA20D8EF, 0x812A, 0x4C43,
            0x98, 0x02, 0x62, 0xEC, 0x4A, 0xBD, 0x7A, 0xDD
        },
        vtable: IDWriteFontFamilyVtbl
    }

    trait DWriteFontFamily: DWriteFontList, Unknown {
        fn get_family_names(
            names: *mut *mut IDWriteLocalizedStrings) -> HRESULT,
        fn get_first_matching_font(
            weight: FontWeight,
            stretch: FontStretch,
            style: FontStyle,
            matching_font: *mut *mut IDWriteFont) -> HRESULT,
        fn get_matching_fonts(
            weight: FontWeight,
            stretch: FontStretch,
            style: FontStyle,
            matching_fonts: *mut *mut IDWriteFontList) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFont: IUnknown {
        iid: IID_IDWRITEFONT {
            0xACD16696, 0x8C14, 0x4F5D,
            0x87, 0x7E, 0xFE, 0x3F, 0xC1, 0xD3, 0x27, 0x37
        },
        vtable: IDWriteFontVtbl
    }

    trait DWriteFont: Unknown {
        fn get_font_family(font_family: *mut *mut IDWriteFontFamily) -> HRESULT,
        fn get_weight() -> FontWeight,
        fn get_stretch() -> FontStretch,
        fn get_style() -> FontStyle,
        fn is_symbol_font() -> BOOL,
        fn get_face_names(names: *mut *mut IDWriteLocalizedStrings) -> HRESULT,
        fn get_informational_strings(
            informational_string_id: InformationalStringID,
            informational_strings: *mut *mut IDWriteLocalizedStrings,
            exists: *mut BOOL) -> HRESULT,
        fn get_simulations() -> FontSimulations,
        fn get_metrics(font_metrics: *mut FontMetrics) -> (),
        fn has_character(unicode_value: UINT32, exists: *mut BOOL) -> HRESULT,
        fn create_font_face(font_face: *mut *mut IDWriteFontFace) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTextFormat: IUnknown {
        iid: IID_IDWRITETEXTFORMAT {
            0x9C906818, 0x31D7, 0x4FD3,
            0xA1, 0x51, 0x7C, 0x5E, 0x22, 0x5D, 0xB5, 0x5A
        },
        vtable: IDWriteTextFormatVtbl
    }

    trait DWriteTextFormat: Unknown {
        fn set_text_alignment(text_alignment: TextAlignment) -> HRESULT,
        fn set_paragraph_alignment(
            paragraph_alignment: ParagraphAlignment) -> HRESULT,
        fn set_word_wrapping(word_wrapping: WordWrapping) -> HRESULT,
        fn set_reading_direction(
            reading_direction: ReadingDirection) -> HRESULT,
        fn set_flow_direction(flow_direction: FlowDirection) -> HRESULT,
        fn set_incremental_tab_stop(incremental_tab_stop: FLOAT) -> HRESULT,
        fn set_trimming(
            trimming_options: *const Trimming,
            trimming_sign: *const IDWriteInlineObject) -> HRESULT,
        fn set_line_spacing(
            line_spacing_method: LineSpacingMethod,
            line_spacing: FLOAT,
            baseline: FLOAT) -> HRESULT,
        fn get_text_alignment() -> TextAlignment,
        fn get_paragraph_alignment() -> ParagraphAlignment,
        fn get_word_wrapping() -> WordWrapping,
        fn get_reading_direction() -> ReadingDirection,
        fn get_flow_direction() -> FlowDirection,
        fn get_incremental_tab_stop() -> FLOAT,
        fn get_trimming(
            trimming_options: *const Trimming,
            trimming_sign: *mut *mut IDWriteInlineObject) -> HRESULT,
        fn get_line_spacing(
            line_spacing_method: *mut LineSpacingMethod,
            line_spacing: *mut FLOAT,
            baseline: *mut FLOAT) -> HRESULT,
        fn get_font_collection(
            font_collection: *mut *mut IDWriteFontCollection) -> HRESULT,
        fn get_font_family_name_length() -> UINT32,
        fn get_font_family_name(
            font_family_name: *mut WCHAR,
            name_size: UINT32) -> HRESULT,
        fn get_font_weight() -> FontWeight,
        fn get_font_style() -> FontStyle,
        fn get_font_stretch() -> FontStretch,
        fn get_font_size() -> FLOAT,
        fn get_locale_name_length() -> UINT32,
        fn get_locale_name(
            locale_name: *mut WCHAR,
            name_size: UINT32) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTypography: IUnknown {
        iid: IID_IDWRITETYPOGRAPHY {
            0x55F1112B, 0x1DC2, 0x4B3C,
            0x95, 0x41, 0xF4, 0x68, 0x94, 0xED, 0x85, 0xB6
        },
        vtable: IDWriteTypographyVtbl
    }

    trait DWriteTypography: Unknown {
        fn add_font_feature(font_feature: FontFeature) -> HRESULT,
        fn get_font_feature_count() -> UINT32,
        fn get_font_feature(
            font_feature_index: UINT32,
            font_feature: *mut FontFeature) -> HRESULT
    }
}

com_interface! {
    struct IDWriteNumberSubstitution: IUnknown {
        iid: IID_IDWRITENUMBERSUBSTITUTION {
            0x14885CC9, 0xBAB0, 0x4F90,
            0xB6, 0xED, 0x5C, 0x36, 0x6A, 0x2C, 0xD0, 0x3D
        },
        vtable: IDWriteNumberSubstitutionVtbl
    }

    trait DWriteNumberSubstitution: Unknown { }
}

com_interface! {
    struct IDWriteTextAnalysisSource: IUnknown {
        iid: IID_IDWRITETEXTANALYSISSOURCE {
            0x688E1A58, 0x5094, 0x47C8,
            0xAD, 0xC8, 0xFB, 0xCE, 0xA6, 0x0A, 0xE9, 0x2B
        },
        vtable: IDWriteTextAnalysisSourceVtbl
    }

    trait DWriteTextAnalysisSource: Unknown {
        fn get_text_at_position(
            text_position: UINT32,
            text_string: *mut *mut WCHAR,
            text_length: *mut UINT32) -> HRESULT,
        fn get_text_before_position(
            text_position: UINT32,
            text_string: *mut *mut WCHAR,
            text_length: *mut UINT32) -> HRESULT,
        fn get_paragraph_reading_direction() -> ReadingDirection,
        fn get_locale_name(
            text_position: UINT32,
            text_length: *mut UINT32,
            locale_name: *mut *mut WCHAR) -> HRESULT,
        fn get_number_substitution(
            text_position: UINT32,
            text_length: *mut UINT32,
            number_substitution: *mut *mut IDWriteNumberSubstitution) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTextAnalysisSink: IUnknown {
        iid: IID_IDWRITETEXTANALYSISSINK {
            0x5810CD44, 0x0CA0, 0x4701,
            0xB3, 0xFA, 0xBE, 0xC5, 0x18, 0x2A, 0xE4, 0xF6
        },
        vtable: IDWriteTextAnalysisSinkVtbl
    }

    trait DWriteTextAnalysisSink: Unknown {
        fn set_script_analysis(
            text_position: UINT32,
            text_length: UINT32,
            script_analysis: *const ScriptAnalysis) -> HRESULT,
        fn set_line_breakpoints(
            text_position: UINT32,
            text_length: UINT32,
            line_breakpoints: *const LineBreakpoint) -> HRESULT,
        fn set_bidi_level(
            text_position: UINT32,
            text_length: UINT32,
            explicit_level: UINT8,
            resolved_level: UINT8) -> HRESULT,
        fn set_number_substitution(
            text_position: UINT32,
            text_length: UINT32,
            number_substitution: *const IDWriteNumberSubstitution) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTextAnalyzer: IUnknown {
        iid: IID_IDWRITETEXTANALYZER {
            0xB7E6163E, 0x7F46, 0x43B4,
            0x84, 0xB3, 0xE4, 0xE6, 0x24, 0x9C, 0x36, 0x5D
        },
        vtable: IDWriteTextAnalyzerVtbl
    }

    trait DWriteTextAnalyzer: Unknown {
        fn analyze_script(
            analysis_source: *const IDWriteTextAnalysisSource,
            text_position: UINT32,
            text_length: UINT32,
            analysis_sink: *const IDWriteTextAnalysisSink) -> HRESULT,
        fn analyze_bidi(
            analysis_source: *const IDWriteTextAnalysisSource,
            text_position: UINT32,
            text_length: UINT32,
            analysis_sink: *const IDWriteTextAnalysisSink) -> HRESULT,
        fn analyze_number_substitution(
            analysis_source: *const IDWriteTextAnalysisSource,
            text_position: UINT32,
            text_length: UINT32,
            analysis_sink: *const IDWriteTextAnalysisSink) -> HRESULT,
        fn analyze_line_breakpoints(
            analysis_source: *const IDWriteTextAnalysisSource,
            text_position: UINT32,
            text_length: UINT32,
            analysis_sink: *const IDWriteTextAnalysisSink) -> HRESULT,
        fn get_glyphs(
            text_string: *const WCHAR,
            text_length: UINT32,
            font_face: *const IDWriteFontFace,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            script_analysis: *const ScriptAnalysis,
            locale_name: *const WCHAR,
            number_substitution: *const IDWriteNumberSubstitution,
            features: *const *const TypographicFeatures,
            feature_range_lengths: *const UINT32,
            feature_ranges: UINT32,
            max_glyph_count: UINT32,
            cluster_map: *mut UINT16,
            text_props: *mut ShapingTextProperties,
            glyph_indices: *mut UINT16,
            glyph_props: *mut ShapingGlyphProperties,
            actual_glyph_count: *mut UINT32) -> HRESULT,
        fn get_glyph_placements(
            text_string: *const WCHAR,
            cluster_map: *const UINT16,
            text_props: *const ShapingTextProperties,
            text_length: UINT32,
            glyph_indices: *const UINT16,
            glyph_props: *const ShapingGlyphProperties,
            glyph_count: UINT32,
            font_face: *const IDWriteFontFace,
            font_em_size: FLOAT,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            script_analysis: *const ScriptAnalysis,
            locale_name: *const WCHAR,
            features: *const *const TypographicFeatures,
            feature_range_lengths: *const UINT32,
            feature_ranges: UINT32,
            glyph_advances: *mut FLOAT,
            glyph_offsets: *mut GlyphOffset) -> HRESULT,
        fn get_gdi_compatible_glyph_placements(
            text_string: *const WCHAR,
            cluster_map: *const UINT16,
            text_props: *const ShapingTextProperties,
            text_length: UINT32,
            glyph_indices: *const UINT16,
            glyph_props: *const ShapingGlyphProperties,
            glyph_count: UINT32,
            font_face: *const IDWriteFontFace,
            font_em_size: FLOAT,
            pixels_per_dip: FLOAT,
            transform: *const Matrix,
            use_gdi_natural: BOOL,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            script_analysis: *const ScriptAnalysis,
            locale_name: *const WCHAR,
            features: *const *const TypographicFeatures,
            feature_range_lengths: *const UINT32,
            feature_ranges: UINT32,
            glyph_advances: *mut FLOAT,
            glyph_offsets: *mut GlyphOffset) -> HRESULT
    }
}

com_interface! {
    struct IDWriteInlineObject: IUnknown {
        iid: IID {
            0x8339FDE3, 0x106F, 0x47AB,
            0x83, 0x73, 0x1C, 0x62, 0x95, 0xEB, 0x10, 0xB3
        },
        vtable: IDWriteInlineObjectVtbl
    }

    trait DWriteInlineObject: Unknown {
        fn draw(
            client_drawing_context: *const c_void,
            renderer: *const IDWriteTextRenderer,
            origin_x: FLOAT,
            origin_y: FLOAT,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            client_drawing_effect: *const IUnknown) -> HRESULT,
        fn get_metrics(metrics: *mut InlineObjectMetrics) -> HRESULT,
        fn get_overhang_metrics(overhangs: *mut OverhangMetrics) -> HRESULT,
        fn get_break_conditions(
            break_condition_before: *mut BreakCondition,
            break_condition_after: *mut BreakCondition) -> HRESULT
    }
}

com_interface! {
    struct IDWritePixelSnapping: IUnknown {
        iid: IID_IDWRITEPIXELSNAPPING {
            0xEAF3A2DA, 0xECF4, 0x4D24,
            0xB6, 0x44, 0xB3, 0x4F, 0x68, 0x42, 0x02, 0x4B
        },
        vtable: IDWritePixelSnappingVtbl
    }

    trait DWritePixelSnapping: Unknown {
        fn is_pixel_snapping_disabled(
            client_drawing_context: *const c_void,
            is_disabled: *mut BOOL) -> HRESULT,
        fn get_current_transform(
            client_drawing_context: *const c_void,
            transform: *mut Matrix) -> HRESULT,
        fn get_pixels_per_dip(
            client_drawing_context: *const c_void,
            pixels_per_dip: *mut FLOAT) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTextRenderer: IDWritePixelSnapping, IUnknown {
        iid: IID_IDWRITETEXTRENDERER {
            0xEF8A8135, 0x5CC6, 0x45FE,
            0x88, 0x25, 0xC5, 0xA0, 0x72, 0x4E, 0xB8, 0x19
        },
        vtable: IDWriteTextRendererVtbl
    }

    trait DWriteTextRenderer: DWritePixelSnapping, Unknown {
        fn draw_glyph_run(
            client_drawing_context: *const c_void,
            baseline_origin_x: FLOAT,
            baseline_origin_y: FLOAT,
            measuring_mode: MeasuringMode,
            glyph_run: *const GlyphRun,
            glyph_run_description: *const GlyphRunDescription,
            client_drawing_effect: *const IUnknown) -> HRESULT,
        fn draw_underline(
            client_drawing_context: *const c_void,
            baseline_origin_x: FLOAT,
            baseline_origin_y: FLOAT,
            underline: *const Underline,
            client_drawing_effect: *const IUnknown) -> HRESULT,
        fn draw_strikethrough(
            client_drawing_context: *const c_void,
            baseline_origin_x: FLOAT,
            baseline_origin_y: FLOAT,
            strikethrough: *const Strikethrough,
            client_drawing_effect: *const IUnknown) -> HRESULT,
        fn draw_inline_object(
            client_drawing_context: *const c_void,
            origin_x: FLOAT,
            origin_y: FLOAT,
            inline_object: *const IDWriteInlineObject,
            is_sideways: BOOL,
            is_right_to_left: BOOL,
            client_drawing_effect: *const IUnknown) -> HRESULT
    }
}

com_interface! {
    struct IDWriteTextLayout: IDWriteTextFormat, IUnknown {
        iid: IID_IDWRITETEXTLAYOUT {
            0x53737037, 0x6D14, 0x410B,
            0x9B, 0xFE, 0x0B, 0x18, 0x2B, 0xB7, 0x09, 0x61
        },
        vtable: IDWriteTextLayoutVtbl
    }

    trait DWriteTextLayout: DWriteTextFormat, Unknown {
        fn set_max_width(max_width: FLOAT) -> HRESULT,
        fn set_max_height(max_height: FLOAT) -> HRESULT,
        fn set_font_collection(
            font_collection: *const IDWriteFontCollection,
            text_range: TextRange) -> HRESULT,
        fn set_font_family_name(
            font_family_name: *const WCHAR,
            text_range: TextRange) -> HRESULT,
        fn set_font_weight(
            font_weight: FontWeight,
            text_range: TextRange) -> HRESULT,
        fn set_font_style(
            font_style: FontStyle,
            text_range: TextRange) -> HRESULT,
        fn set_font_stretch(
            font_stretch: FontStretch,
            text_range: TextRange) -> HRESULT,
        fn set_font_size(
            font_size: FLOAT,
            text_range: TextRange) -> HRESULT,
        fn set_underline(
            has_underline: BOOL,
            text_range: TextRange) -> HRESULT,
        fn set_strikethrough(
            has_strikethrough: BOOL,
            text_range: TextRange) -> HRESULT,
        fn set_drawing_effect(
            drawing_effect: *const IUnknown,
            text_range: TextRange) -> HRESULT,
        fn set_inline_object(
            inline_object: *const IDWriteInlineObject,
            text_range: TextRange) -> HRESULT,
        fn set_typography(
            typography: *const IDWriteTypography,
            text_range: TextRange) -> HRESULT,
        fn set_locale_name(
            locale_name: *const WCHAR,
            text_range: TextRange) -> HRESULT,
        fn get_max_width() -> FLOAT,
        fn get_max_height() -> FLOAT,
        fn get_font_collection(
            current_position: UINT32,
            font_collection: *mut *mut IDWriteFontCollection,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_family_name_length(
            current_position: UINT32,
            name_length: *mut UINT32,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_family_name(
            current_position: UINT32,
            font_family_name: *mut WCHAR,
            name_size: UINT32,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_weight(
            current_position: UINT32,
            font_weight: *mut FontWeight,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_style(
            current_position: UINT32,
            font_style: *mut FontStyle,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_stretch(
            current_position: UINT32,
            font_stretch: *mut FontStretch,
            text_range: *mut TextRange) -> HRESULT,
        fn get_font_size(
            current_position: UINT32,
            font_size: *mut FLOAT,
            text_range: *mut TextRange) -> HRESULT,
        fn get_underline(
            current_position: UINT32,
            has_underline: *mut BOOL,
            text_range: *mut TextRange) -> HRESULT,
        fn get_strikethrough(
            current_position: UINT32,
            has_strikethrough: *mut BOOL,
            text_range: *mut TextRange) -> HRESULT,
        fn get_drawing_effect(
            current_position: UINT32,
            drawing_effect: *mut *mut IUnknown,
            text_range: *mut TextRange) -> HRESULT,
        fn get_inline_object(
            current_position: UINT32,
            inline_object: *mut *mut IDWriteInlineObject,
            text_range: *mut TextRange) -> HRESULT,
        fn get_typography(
            current_position: UINT32,
            typography: *mut *mut IDWriteTypography,
            text_range: *mut TextRange) -> HRESULT,
        fn get_locale_name_length(
            current_position: UINT32,
            name_length: *mut UINT32,
            text_range: *mut TextRange) -> HRESULT,
        fn get_locale_name(
            current_position: UINT32,
            locale_name: *mut WCHAR,
            name_size: UINT32,
            text_range: *mut TextRange) -> HRESULT,
        fn draw(
            client_drawing_context: *const c_void,
            renderer: *const IDWriteTextRenderer,
            origin_x: FLOAT,
            origin_y: FLOAT) -> HRESULT,
        fn get_line_metrics(
            line_metrics: *mut LineMetrics,
            max_line_count: UINT32,
            actual_line_count: *mut UINT32) -> HRESULT,
        fn get_metrics(text_metrics: *mut TextMetrics) -> HRESULT,
        fn get_overhang_metrics(overhangs: *mut OverhangMetrics) -> HRESULT,
        fn get_cluster_metrics(
            cluster_metrics: *mut ClusterMetrics,
            max_cluster_count: UINT32,
            actual_cluster_count: *mut UINT32) -> HRESULT,
        fn determine_min_width(min_width: *mut FLOAT) -> HRESULT,
        fn hit_test_point(
            point_x: FLOAT,
            point_y: FLOAT,
            is_trailing_hit: *mut BOOL,
            is_inside: *mut BOOL,
            hit_test_metrics: *mut HitTestMetrics) -> HRESULT,
        fn hit_test_text_position(
            text_position: UINT32,
            is_trailing_hit: BOOL,
            point_x: *mut FLOAT,
            point_y: *mut FLOAT,
            hit_test_metrics: *mut HitTestMetrics) -> HRESULT,
        fn hit_test_text_range(
            text_position: UINT32,
            text_length: UINT32,
            origin_x: FLOAT,
            origin_y: FLOAT,
            hit_test_metrics: *mut HitTestMetrics,
            max_hit_test_metrics_count: UINT32,
            actual_hit_test_metrics_count: *mut UINT32) -> HRESULT
    }
}

com_interface! {
    struct IDWriteBitmapRenderTarget: IUnknown {
        iid: IID_IDWRITEBITMAPRENDERTARGET {
            0x5E5A32A3, 0x8DFF, 0x4773,
            0x9F, 0xF6, 0x06, 0x96, 0xEA, 0xB7, 0x72, 0x67
        },
        vtable: IDWriteBitmapRenderTargetVtbl
    }

    trait DWriteBitmapRenderTarget: Unknown {
        fn draw_glyph_run(
            baseline_origin_x: FLOAT,
            baseline_origin_y: FLOAT,
            measuring_mode: MeasuringMode,
            glyph_run: *const GlyphRun,
            rendering_params: *const IDWriteRenderingParams,
            text_color: COLORREF,
            black_box_rect: *mut RECT) -> HRESULT,
        fn get_memory_dc() -> HDC,
        fn get_pixels_per_dip() -> FLOAT,
        fn set_pixels_per_dip(pixels_per_dip: FLOAT) -> HRESULT,
        fn get_current_transform(transform: *mut Matrix) -> HRESULT,
        fn set_current_transform(transform: *const Matrix) -> HRESULT,
        fn get_size(size: *mut SIZE) -> HRESULT,
        fn resize(
            width: UINT32,
            height: UINT32) -> HRESULT
    }
}

com_interface! {
    struct IDWriteGdiInterop: IUnknown {
        iid: IID_IDWRITEGDIINTEROP {
            0x1EDD9491, 0x9853, 0x4299,
            0x89, 0x8F, 0x64, 0x32, 0x98, 0x3B, 0x6F, 0x3A
        },
        vtable: IDWriteGdiInteropVtbl
    }

    trait DWriteGdiInterop: Unknown {
        fn create_font_from_log_font(
            log_font: *const LOGFONTW,
            font: *mut *mut IDWriteFont) -> HRESULT,
        fn convert_font_to_log_font(
            font: *const IDWriteFont,
            log_font: *mut LOGFONTW,
            is_system_font: *mut BOOL) -> HRESULT,
        fn convert_font_face_to_log_font(
            font: *const IDWriteFontFace,
            log_font: *mut LOGFONTW) -> HRESULT,
        fn create_font_face_from_hdc(
            hdc: HDC,
            font_face: *mut *mut IDWriteFontFace) -> HRESULT,
        fn create_bitmap_render_target(
            hdc: HDC,
            width: UINT32,
            height: UINT32,
            render_target: *mut *mut IDWriteBitmapRenderTarget) -> HRESULT
    }
}

com_interface! {
    struct IDWriteGlyphRunAnalysis: IUnknown {
        iid: IID_IDWRITEGLYPHRUNANALYSIS {
            0x7D97DBF7, 0xE085, 0x42D4,
            0x81, 0xE3, 0x6A, 0x88, 0x3B, 0xDE, 0xD1, 0x18
        },
        vtable: IDWriteGlyphRunAnalysisVtbl
    }

    trait DWriteGlyphRunAnalysis: Unknown {
        fn get_alpha_texture_bounds(
            texture_type: TextureType,
            texture_bounds: *mut RECT) -> HRESULT,
        fn create_alpha_texture(
            texture_type: TextureType,
            texture_bounds: *const RECT,
            alpha_values: *mut BYTE,
            buffer_size: UINT32) -> HRESULT,
        fn get_alpha_blend_params(
            rendering_params: *const IDWriteRenderingParams,
            blend_gamma: *mut FLOAT,
            blend_enhanced_contrast: *mut FLOAT,
            blend_clear_type_level: *mut FLOAT) -> HRESULT
    }
}

com_interface! {
    struct IDWriteFactory: IUnknown {
        iid: IID_IDWRITEFACTORY {
            0xB859EE5A, 0xD838, 0x4B5B,
            0xA2, 0xE8, 0x1A, 0xDC, 0x7D, 0x93, 0xDB, 0x48
        },
        vtable: IDWriteFactoryVtbl
    }

    trait DWriteFactory: Unknown {
        fn get_system_font_collection(
            font_collection: *mut *mut IDWriteFontCollection,
            check_for_updates: BOOL) -> HRESULT,
        fn create_custom_font_collection(
            collection_loader: *const IDWriteFontCollectionLoader,
            collection_key: *const c_void,
            collection_key_size: UINT32,
            font_collection: *mut *mut IDWriteFontCollection) -> HRESULT,
        fn register_font_collection_loader(
            font_collection_loader: *const IDWriteFontCollectionLoader)
            -> HRESULT,
        fn unregister_font_collection_loader(
            font_collection_loader: *const IDWriteFontCollectionLoader)
            -> HRESULT,
        fn create_font_file_reference(
            file_path: *const WCHAR,
            last_write_time: *const FILETIME,
            font_file: *mut *mut IDWriteFontFile) -> HRESULT,
        fn create_custom_font_file_reference(
            font_file_reference_key: *const c_void,
            font_file_reference_key_size: UINT32,
            font_file_loader: *const IDWriteFontFileLoader,
            font_file: *mut *mut IDWriteFontFile) -> HRESULT,
        fn create_font_face(
            font_face_type: FontFaceType,
            number_of_files: UINT32,
            font_files: *const *const IDWriteFontFile,
            face_index: UINT32,
            font_face_simulation_flags: FontSimulations,
            font_face: *mut *mut IDWriteFontFace) -> HRESULT,
        fn create_rendering_params(
            rendering_params: *mut *mut IDWriteRenderingParams) -> HRESULT,
        fn create_monitor_rendering_params(
            monitor: HMONITOR,
            rendering_params: *mut *mut IDWriteRenderingParams) -> HRESULT,
        fn create_custom_rendering_params(
            gamma: FLOAT,
            enhanced_contrast: FLOAT,
            clear_type_level: FLOAT,
            pixel_geometry: PixelGeometry,
            rendering_mode: RenderingMode,
            rendering_params: *mut *mut IDWriteRenderingParams) -> HRESULT,
        fn register_font_file_loader(
            font_file_loader: *const IDWriteFontFileLoader) -> HRESULT,
        fn unregister_font_file_loader(
            font_file_loader: *const IDWriteFontFileLoader) -> HRESULT,
        fn create_text_format(
            font_family_name: *const WCHAR,
            font_collection: *const IDWriteFontCollection,
            font_weight: FontWeight,
            font_style: FontStyle,
            font_stretch: FontStretch,
            font_size: FLOAT,
            locale_name: *const WCHAR,
            text_format: *mut *mut IDWriteTextFormat) -> HRESULT,
        fn create_typography(
            typography: *mut *mut IDWriteTypography) -> HRESULT,
        fn get_gdi_interop(gdi_interop: *mut *mut IDWriteGdiInterop) -> HRESULT,
        fn create_text_layout(
            string: *const WCHAR,
            string_length: UINT32,
            text_format: *const IDWriteTextFormat,
            max_width: FLOAT,
            max_height: FLOAT,
            text_layout: *mut *mut IDWriteTextLayout) -> HRESULT,
        fn create_gdi_compatible_text_layout(
            string: *const WCHAR,
            string_length: UINT32,
            text_format: *const IDWriteTextFormat,
            layout_width: FLOAT,
            layout_height: FLOAT,
            pixels_per_dip: FLOAT,
            transform: *const Matrix,
            use_gdi_natural: BOOL,
            text_layout: *mut *mut IDWriteTextLayout) -> HRESULT,
        fn create_ellipsis_trimming_sign(
            text_format: *const IDWriteTextFormat,
            trimming_sign: *mut *mut IDWriteInlineObject) -> HRESULT,
        fn create_text_analyzer(
            text_analyzer: *mut *mut IDWriteTextAnalyzer) -> HRESULT,
        fn create_number_substitution(
            substitution_method: NumberSubstitutionMethod,
            locale_name: *const WCHAR,
            ignore_user_override: BOOL,
            number_substitution: *mut *mut IDWriteNumberSubstitution)
            -> HRESULT,
        fn create_glyph_run_analysis(
            glyph_run: *const GlyphRun,
            pixels_per_dip: FLOAT,
            transform: *const Matrix,
            rendering_mode: RenderingMode,
            measuring_mode: MeasuringMode,
            baseline_origin_x: FLOAT,
            baseline_origin_y: FLOAT,
            glyph_run_analysis: *mut *mut IDWriteGlyphRunAnalysis) -> HRESULT
    }
}
