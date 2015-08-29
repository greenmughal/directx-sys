use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, FLOAT, HDC, HRESULT, HWND, RECT, UINT32, WCHAR};

#[cfg(feature = "dwrite")]
use dwrite;

// Stub DirectWrite types to use D2D without it.
#[cfg(not(feature = "dwrite"))]
mod dwrite {
    use com_rs::IUnknown;
    pub type IDWriteRenderingParams = IUnknown;
    pub type IDWriteTextFormat = IUnknown;
    pub type IDWriteTextLayout = IUnknown;
    pub type MeasuringMode = u32;
    pub type GlyphRun = u32;
}

use dxgi;

use super::enums::*;
use super::structs::*;

#[link(name = "dxguid")]
extern {
    static IID_ID2D1Bitmap: IID;
    static IID_ID2D1BitmapBrush: IID;
    static IID_ID2D1BitmapRenderTarget: IID;
    static IID_ID2D1Brush: IID;
    static IID_ID2D1DCRenderTarget: IID;
    static IID_ID2D1DrawingStateBlock: IID;
    static IID_ID2D1EllipseGeometry: IID;
    static IID_ID2D1Factory: IID;
    static IID_ID2D1GdiInteropRenderTarget: IID;
    static IID_ID2D1Geometry: IID;
    static IID_ID2D1GeometryGroup: IID;
    static IID_ID2D1GeometrySink: IID;
    static IID_ID2D1GradientStopCollection: IID;
    static IID_ID2D1HwndRenderTarget: IID;
    static IID_ID2D1Image: IID;
    static IID_ID2D1Layer: IID;
    static IID_ID2D1LinearGradientBrush: IID;
    static IID_ID2D1Mesh: IID;
    static IID_ID2D1PathGeometry: IID;
    static IID_ID2D1RadialGradientBrush: IID;
    static IID_ID2D1RectangleGeometry: IID;
    static IID_ID2D1RenderTarget: IID;
    static IID_ID2D1Resource: IID;
    static IID_ID2D1RoundedRectangleGeometry: IID;
    static IID_ID2D1SimplifiedGeometrySink: IID;
    static IID_ID2D1SolidColorBrush: IID;
    static IID_ID2D1StrokeStyle: IID;
    static IID_ID2D1TessellationSink: IID;
    static IID_ID2D1TransformedGeometry: IID;
}

com_interface! {
    interface ID2D1Resource: IUnknown {
        iid: IID_ID2D1Resource,
        vtable: ID2D1ResourceVtbl,
        fn get_factory(factory: *mut *mut ID2D1Factory) -> ();
    }
}

com_interface! {
    interface ID2D1Image: ID2D1Resource, IUnknown {
        iid: IID_ID2D1Image,
        vtable: ID2D1ImageVtbl,
    }
}

com_interface! {
    interface ID2D1Bitmap: ID2D1Image, ID2D1Resource, IUnknown {
        iid: IID_ID2D1Bitmap,
        vtable: ID2D1BitmapVtbl,
        fn get_size() -> SizeF;
        fn get_pixel_size() -> SizeU;
        fn get_pixel_format() -> PixelFormat;
        fn get_dpi(dpi_x: *mut FLOAT, dpi_y: *mut FLOAT) -> ();
        fn copy_from_bitmap(
            dest_point: *const Point2U,
            bitmap: *const ID2D1Bitmap,
            src_rect: *const RectU) -> HRESULT;
        fn copy_from_render_target(
            dest_point: *const Point2U,
            render_target: *const ID2D1RenderTarget,
            src_rect: *const RectU) -> HRESULT;
        fn copy_from_memory(
            dst_rect: *const RectU,
            src_data: *const c_void,
            pitch: UINT32) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1GradientStopCollection: ID2D1Resource, IUnknown {
        iid: IID_ID2D1GradientStopCollection,
        vtable: ID2D1GradientStopCollectionVtbl,
        fn get_gradient_stop_count() -> UINT32;
        fn get_gradient_stops(
            gradient_stops: *mut GradientStop,
            gradient_stop_count: UINT32) -> ();
        fn get_color_interpolation_gamma() -> Gamma;
        fn get_extend_mode() -> ExtendMode;
    }
}

com_interface! {
    interface ID2D1Brush: ID2D1Resource, IUnknown {
        iid: IID_ID2D1Brush,
        vtable: ID2D1BrushVtbl,
        fn set_opacity(opacity: FLOAT) -> ();
        fn set_transform(transform: *const Matrix3x2F) -> ();
        fn get_opacity() -> FLOAT;
        fn get_transform(transform: *mut Matrix3x2F) -> ();
    }
}

com_interface! {
    interface ID2D1BitmapBrush: ID2D1Brush, ID2D1Resource, IUnknown {
        iid: IID_ID2D1BitmapBrush,
        vtable: ID2D1BitmapBrushVtbl,
        fn set_extend_mode_x(extend_mode_x: ExtendMode) -> ();
        fn set_extend_mode_y(extend_mode_y: ExtendMode) -> ();
        fn set_interpolation_mode(
            interpolation_mode: BitmapInterpolationMode) -> ();
        fn set_bitmap(bitmap: *const ID2D1Bitmap) -> ();
        fn get_extend_mode_x() -> ExtendMode;
        fn get_extend_mode_y() -> ExtendMode;
        fn get_interpolation_mode() -> BitmapInterpolationMode;
        fn get_bitmap(bitmap: *mut *mut ID2D1Bitmap) -> ();
    }
}

com_interface! {
    interface ID2D1SolidColorBrush: ID2D1Brush, ID2D1Resource, IUnknown {
        iid: IID_ID2D1SolidColorBrush,
        vtable: ID2D1SolidColorBrushVtbl,
        fn set_color(color: *const ColorF) -> ();
        fn get_color() -> ColorF;
    }
}

com_interface! {
    interface ID2D1LinearGradientBrush: ID2D1Brush, ID2D1Resource, IUnknown {
        iid: IID_ID2D1LinearGradientBrush,
        vtable: ID2D1LinearGradientBrushVtbl,
        fn set_start_point(start_point: Point2F) -> ();
        fn set_end_point(end_point: Point2F) -> ();
        fn get_start_point() -> Point2F;
        fn get_end_point() -> Point2F;
        fn get_gradient_stop_collection(
            collection: *mut *mut ID2D1GradientStopCollection) -> ();
    }
}

com_interface! {
    interface ID2D1RadialGradientBrush: ID2D1Brush, ID2D1Resource, IUnknown {
        iid: IID_ID2D1RadialGradientBrush,
        vtable: ID2D1RadialGradientBrushVtbl,
        fn set_center(center: Point2F) -> ();
        fn set_gradient_origin_offset(gradient_origin_offset: Point2F) -> ();
        fn set_radius_x(radius_x: FLOAT) -> ();
        fn set_radius_y(radius_y: FLOAT) -> ();
        fn get_center() -> Point2F;
        fn get_gradient_origin_offset() -> Point2F;
        fn get_radius_x() -> FLOAT;
        fn get_radius_y() -> FLOAT;
        fn get_gradient_stop_collection(
            collection: *mut *mut ID2D1GradientStopCollection) -> ();
    }
}

com_interface! {
    interface ID2D1StrokeStyle: ID2D1Resource, IUnknown {
        iid: IID_ID2D1StrokeStyle,
        vtable: ID2D1StrokeStyleVtbl,
        fn get_start_cap() -> CapStyle;
        fn get_end_cap() -> CapStyle;
        fn get_dash_cap() -> CapStyle;
        fn get_miter_limit() -> FLOAT;
        fn get_line_join() -> LineJoin;
        fn get_dash_offset() -> FLOAT;
        fn get_dash_style() -> DashStyle;
        fn get_dashes_count() -> UINT32;
        fn get_dashes(
            dashes: *mut FLOAT,
            dashes_count: UINT32) -> ();
    }
}

com_interface! {
    interface ID2D1Geometry: ID2D1Resource, IUnknown {
        iid: IID_ID2D1Geometry,
        vtable: ID2D1GeometryVtbl,
        fn get_bounds(
            world_transform: *const Matrix3x2F,
            bounds: *mut RectF) -> HRESULT;
        fn get_widened_bounds(
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            bounds: *mut RectF) -> HRESULT;
        fn stroke_contains_point(
            point: Point2F,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            contains: *mut BOOL) -> HRESULT;
        fn fill_contains_point(
            point: Point2F,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            contains: *mut BOOL) -> HRESULT;
        fn compare_with_geometry(
            input_geometry: *const ID2D1Geometry,
            input_geometry_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            relation: *mut GeometryRelation) -> HRESULT;
        fn simplify(
            simplification_option: GeometrySimplificationOption,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            geometry_sink: *const ID2D1SimplifiedGeometrySink) -> HRESULT;
        fn tessellate(
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            tessellation_sink: *const ID2D1TessellationSink) -> HRESULT;
        fn combine_with_geometry(
            input_geometry: *const ID2D1Geometry,
            combine_mode: CombineMode,
            input_geometry_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            geometry_sink: *const ID2D1SimplifiedGeometrySink) -> HRESULT;
        fn outline(
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            geometry_sink: *const ID2D1SimplifiedGeometrySink) -> HRESULT;
        fn compute_area(
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            area: *mut FLOAT) -> HRESULT;
        fn compute_length(
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            length: *mut FLOAT) -> HRESULT;
        fn compute_point_at_length(
            length: FLOAT,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            point: *mut Point2F,
            unit_tangent_vector: *mut Point2F) -> HRESULT;
        fn widen(
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle,
            world_transform: *const Matrix3x2F,
            flattening_tolerance: FLOAT,
            geometry_sink: *const ID2D1GeometrySink) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1RectangleGeometry: ID2D1Geometry, ID2D1Resource, IUnknown {
        iid: IID_ID2D1RectangleGeometry,
        vtable: ID2D1RectangleGeometryVtbl,
        fn get_rect(rect: *mut RectF) -> ();
    }
}

com_interface! {
    interface ID2D1RoundedRectangleGeometry: ID2D1Geometry, ID2D1Resource,
                                             IUnknown {
        iid: IID_ID2D1RoundedRectangleGeometry,
        vtable: ID2D1RoundedRectangleGeometryVtbl,
        fn get_rounded_rect(rounded_rect: *mut RoundedRect) -> ();
    }
}

com_interface! {
    interface ID2D1EllipseGeometry: ID2D1Geometry, ID2D1Resource, IUnknown {
        iid: IID_ID2D1EllipseGeometry,
        vtable: ID2D1EllipseGeometryVtbl,
        fn get_ellipse(ellipse: *mut Ellipse) -> ();
    }
}

com_interface! {
    interface ID2D1GeometryGroup: ID2D1Geometry, ID2D1Resource, IUnknown {
        iid: IID_ID2D1GeometryGroup,
        vtable: ID2D1GeometryGroupVtbl,
        fn get_fill_mode() -> FillMode;
        fn get_source_geometry_count() -> UINT32;
        fn get_source_geometries(
            geometries: *mut *mut ID2D1Geometry,
            geometries_count: UINT32) -> ();
    }
}

com_interface! {
    interface ID2D1TransformedGeometry: ID2D1Geometry, ID2D1Resource, IUnknown {
        iid: IID_ID2D1TransformedGeometry,
        vtable: ID2D1TransformedGeometryVtbl,
        fn get_source_geometry(source_geometry: *mut *mut ID2D1Geometry) -> ();
        fn get_transform(transform: *mut Matrix3x2F) -> ();
    }
}

com_interface! {
    interface ID2D1SimplifiedGeometrySink: IUnknown {
        iid: IID_ID2D1SimplifiedGeometrySink,
        vtable: ID2D1SimplifiedGeometrySinkVtbl,
        fn set_fill_mode(fill_mode: FillMode) -> ();
        fn set_segment_flags(vertex_flags: PathSegment) -> ();
        fn begin_figure(
            start_point: Point2F,
            figure_begin: FigureBegin) -> ();
        fn add_lines(
            points: *const Point2F,
            points_count: UINT32) -> ();
        fn add_beziers(
            beziers: *const BezierSegment,
            beziers_count: UINT32) -> ();
        fn end_figure(figure_end: FigureEnd) -> ();
        fn close() -> HRESULT;
    }
}

com_interface! {
    interface ID2D1GeometrySink: ID2D1SimplifiedGeometrySink, IUnknown {
        iid: IID_ID2D1GeometrySink,
        vtable: ID2D1GeometrySinkVtbl,
        fn add_line(point: Point2F) -> ();
        fn add_bezier(bezier: *const BezierSegment) -> ();
        fn add_quadratic_bezier(bezier: *const QuadraticBezierSegment) -> ();
        fn add_quadratic_beziers(
            beziers: *const QuadraticBezierSegment,
            beziers_count: UINT32) -> ();
        fn add_arc(arc: *const ArcSegment) -> ();
    }
}

com_interface! {
    interface ID2D1TessellationSink: IUnknown {
        iid: IID_ID2D1TessellationSink,
        vtable: ID2D1TessellationSinkVtbl,
        fn add_triangles(
            triangles: *const Triangle,
            triangles_count: UINT32) -> ();
        fn close() -> HRESULT;
    }
}

com_interface! {
    interface ID2D1PathGeometry: ID2D1Geometry, ID2D1Resource, IUnknown {
        iid: IID_ID2D1PathGeometry,
        vtable: ID2D1PathGeometryVtbl,
        fn open(geometry_sink: *mut *mut ID2D1GeometrySink) -> HRESULT;
        fn stream(geometry_sink: *const ID2D1GeometrySink) -> HRESULT;
        fn get_segment_count(count: *mut UINT32) -> HRESULT;
        fn get_figure_count(count: *mut UINT32) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1Mesh: ID2D1Resource, IUnknown {
        iid: IID_ID2D1Mesh,
        vtable: ID2D1MeshVtbl,
        fn open(tessellation_sink: *mut *mut ID2D1TessellationSink) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1Layer: ID2D1Resource, IUnknown {
        iid: IID_ID2D1Layer,
        vtable: ID2D1LayerVtbl,
        fn get_size() -> SizeF;
    }
}

com_interface! {
    interface ID2D1DrawingStateBlock: ID2D1Resource, IUnknown {
        iid: IID_ID2D1DrawingStateBlock,
        vtable: ID2D1DrawingStateBlockVtbl,
        fn get_description(
            state_description: *mut DrawingStateDescription) -> ();
        fn set_description(
            state_description: *const DrawingStateDescription) -> ();
        fn set_text_rendering_params(
            text_rendering_params: *const dwrite::IDWriteRenderingParams) -> ();
        fn get_text_rendering_params(
            text_rendering_params: *mut *mut dwrite::IDWriteRenderingParams)
            -> ();
    }
}

com_interface! {
    interface ID2D1RenderTarget: ID2D1Resource, IUnknown {
        iid: IID_ID2D1RenderTarget,
        vtable: ID2D1RenderTargetVtbl,
        fn create_bitmap(
            size: SizeU,
            src_data: *const c_void,
            pitch: UINT32,
            bitmap_properties: *const BitmapProperties,
            bitmap: *mut *mut ID2D1Bitmap) -> HRESULT;
        fn create_bitmap_from_wic_bitmap(
            wic_bitmap_source: *const IUnknown, // TODO IWICBitmapSource
            bitmap_properties: *const BitmapProperties,
            bitmap: *mut *mut ID2D1Bitmap) -> HRESULT;
        fn create_shared_bitmap(
            iid: &IID,
            data: *mut c_void,
            bitmap_properties: *const BitmapProperties,
            bitmap: *mut *mut ID2D1Bitmap) -> HRESULT;
        fn create_bitmap_brush(
            bitmap: *const ID2D1Bitmap,
            bitmap_brush_properties: *const BitmapBrushProperties,
            brush_properties: *const BrushProperties,
            brush: *mut *mut ID2D1BitmapBrush) -> HRESULT;
        fn create_solid_color_brush(
            color: *const ColorF,
            brush_properties: *const BrushProperties,
            brush: *mut *mut ID2D1SolidColorBrush) -> HRESULT;
        fn create_gradient_stop_collection(
            gradient_stops: *const GradientStop,
            gradient_stops_count: UINT32,
            color_interpolation_gamma: Gamma,
            extend_mode: ExtendMode,
            collection: *mut *mut ID2D1GradientStopCollection) -> HRESULT;
        fn create_linear_gradient_brush(
            properties: *const LinearGradientBrushProperties,
            brush_properties: *const BrushProperties,
            gradient_stop_collection: *const ID2D1GradientStopCollection,
            brush: *mut *mut ID2D1LinearGradientBrush)-> HRESULT;
        fn create_radial_gradient_brush(
            properties: *const RadialGradientBrushProperties,
            brush_properties: *const BrushProperties,
            gradient_stop_collection: *const ID2D1GradientStopCollection,
            brush: *mut *mut ID2D1RadialGradientBrush) -> HRESULT;
        fn create_compatible_render_target(
            desired_size: *const SizeF,
            desired_pixel_size: *const SizeU,
            desired_format: PixelFormat,
            options: CompatibleRenderTargetOptions,
            bitmap_render_target: *mut *mut ID2D1BitmapRenderTarget) -> HRESULT;
        fn create_layer(
            size: *const SizeF,
            layer: *mut *mut ID2D1Layer) -> HRESULT;
        fn create_mesh(mesh: *mut *mut ID2D1Mesh) -> HRESULT;
        fn draw_line(
            point_0: Point2F,
            point_1: Point2F,
            brush: *const ID2D1Brush,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle) -> ();
        fn draw_rectangle(
            rect: *const RectF,
            brush: *const ID2D1Brush,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle) -> ();
        fn fill_rectangle(
            rect: *const RectF,
            brush: *const ID2D1Brush) -> ();
        fn draw_rounded_rectangle(
            rounded_rect: *const RoundedRect,
            brush: *const ID2D1Brush,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle) -> ();
        fn fill_rounded_rectangle(
            rounded_rect: *const RoundedRect,
            brush: *const ID2D1Brush) -> ();
        fn draw_ellipse(
            ellipse: *const Ellipse,
            brush: *const ID2D1Brush,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle) -> ();
        fn fill_ellipse(
            ellipse: *const Ellipse,
            brush: *const ID2D1Brush) -> ();
        fn draw_geometry(
            geometry: *const ID2D1Geometry,
            brush: *const ID2D1Brush,
            stroke_width: FLOAT,
            stroke_style: *const ID2D1StrokeStyle) -> ();
        fn fill_geometry(
            geometry: *const ID2D1Geometry,
            brush: *const ID2D1Brush,
            opacity_brush: *const ID2D1Brush) -> ();
        fn fill_mesh(
            mesh: *const ID2D1Mesh,
            brush: *const ID2D1Brush) -> ();
        fn fill_opacity_mask(
            opacity_mask: *const ID2D1Bitmap,
            brush: *const ID2D1Brush,
            content: OpacityMaskContent,
            destination_rectangle: *const RectF,
            source_rectangle: *const RectF) -> ();
        fn draw_bitmap(
            bitmap: *const ID2D1Bitmap,
            destination_rectangle: *const RectF,
            opacity: FLOAT,
            interpolation_mode: BitmapInterpolationMode,
            source_rectangle: *const RectF) -> ();
        fn draw_text(
            string: *const WCHAR,
            string_length: UINT32,
            text_format: *const dwrite::IDWriteTextFormat,
            layout_rect: *const RectF,
            default_foreground_brush: *const ID2D1Brush,
            options: DrawTextOptions,
            measuring_mode: dwrite::MeasuringMode) -> ();
        fn draw_text_layout(
            origin: Point2F,
            text_layout: *const dwrite::IDWriteTextLayout,
            default_foreground_brush: *const ID2D1Brush,
            options: DrawTextOptions) -> ();
        fn draw_glyph_run(
            baseline_origin: Point2F,
            glyph_run: *const dwrite::GlyphRun,
            foreground_brush: *const ID2D1Brush,
            measuring_mode: dwrite::MeasuringMode) -> ();
        fn set_transform(transform: *const Matrix3x2F) -> ();
        fn get_transform(transform: *mut Matrix3x2F) -> ();
        fn set_antialias_mode(antialias_mode: AntialiasMode) -> ();
        fn get_antialias_mode() -> AntialiasMode;
        fn set_text_antialias_mode(
            text_antialias_mode: TextAntialiasMode) -> ();
        fn get_text_antialias_mode() -> TextAntialiasMode;
        fn set_text_rendering_params(
            text_rendering_params: *const dwrite::IDWriteRenderingParams) -> ();
        fn get_text_rendering_params(
            text_rendering_params: *mut *mut dwrite::IDWriteRenderingParams)
            -> ();
        fn set_tags(
            tag1: Tag,
            tag2: Tag) -> ();
        fn get_tags(
            tag1: *mut Tag,
            tag2: *mut Tag) -> ();
        fn push_layer(
            layer_parameters: *const LayerParameters,
            layer: *const ID2D1Layer) -> ();
        fn pop_layer() -> ();
        fn flush(
            tag1: *mut Tag,
            tag2: *mut Tag) -> HRESULT;
        fn save_drawing_state(
            drawing_state_block: *mut ID2D1DrawingStateBlock) -> ();
        fn restore_drawing_state(
            drawing_state_block: *const ID2D1DrawingStateBlock) -> ();
        fn push_axis_aligned_clip(
            clip_rect: *const RectF,
            antialias_mode: AntialiasMode) -> ();
        fn pop_axis_aligned_clip() -> ();
        fn clear(clear_color: *const ColorF) -> ();
        fn begin_draw() -> ();
        fn end_draw(
            tag1: *mut Tag,
            tag2: *mut Tag) -> HRESULT;
        fn get_pixel_format() -> PixelFormat;
        fn set_dpi(
            dpi_x: FLOAT,
            dpi_y: FLOAT) -> ();
        fn get_dpi(
            dpi_x: *mut FLOAT,
            dpi_y: *mut FLOAT) -> ();
        fn get_size() -> SizeF;
        fn get_pixel_size() -> SizeU;
        fn get_maximum_bitmap_size() -> UINT32;
        fn is_supported(
            render_target_properties: *const RenderTargetProperties) -> BOOL;
    }
}

com_interface! {
    interface ID2D1BitmapRenderTarget: ID2D1RenderTarget, ID2D1Resource,
                                       IUnknown {
        iid: IID_ID2D1BitmapRenderTarget,
        vtable: ID2D1BitmapRenderTargetVtbl,
        fn get_bitmap(bitmap: *mut *mut ID2D1Bitmap) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1HwndRenderTarget: ID2D1RenderTarget, ID2D1Resource, IUnknown {
        iid: IID_ID2D1HwndRenderTarget,
        vtable: ID2D1HwndRenderTargetVtbl,
        fn check_window_state() -> WindowState;
        fn resize(pixel_size: *const SizeU) -> HRESULT;
        fn get_hwnd() -> HWND;
    }
}

com_interface! {
    interface ID2D1GdiInteropRenderTarget: IUnknown {
        iid: IID_ID2D1GdiInteropRenderTarget,
        vtable: ID2D1GdiInteropRenderTargetVtbl,
        fn get_dc(mode: DCInitializeMode, hdc: *mut HDC) -> HRESULT;
        fn release_dc(update: *const RECT) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1DCRenderTarget: ID2D1RenderTarget, ID2D1Resource, IUnknown {
        iid: IID_ID2D1DCRenderTarget,
        vtable: ID2D1DCRenderTargetVtbl,
        fn bind_dc(hdc: HDC, sub_rect: *const RECT) -> HRESULT;
    }
}

com_interface! {
    interface ID2D1Factory: IUnknown {
        iid: IID_ID2D1Factory,
        vtable: ID2D1FactoryVtbl,
        fn reload_system_metrics() -> HRESULT;
        fn get_desktop_dpi(dpi_x: *mut FLOAT, dpi_y: *mut FLOAT) -> ();
        fn create_rectangle_geometry(
            rectangle: *const RectF,
            rectangle_geometry: *mut *mut ID2D1RectangleGeometry) -> HRESULT;
        fn create_rounded_rectangle_geometry(
            rounded_rectangle: *const RoundedRect,
            rounded_rectangle_geometry: *mut *mut ID2D1RoundedRectangleGeometry)
            -> HRESULT;
        fn create_ellipse_geometry(
            ellipse: *const Ellipse,
            ellipse_geometry: *mut *mut ID2D1EllipseGeometry) -> HRESULT;
        fn create_geometry_group(
            fill_mode: FillMode,
            geometries: *const *const ID2D1Geometry,
            geometries_count: UINT32,
            geometry_group: *mut *mut ID2D1GeometryGroup) -> HRESULT;
        fn create_transformed_geometry(
            source_geometry: *const ID2D1Geometry,
            transform: *const Matrix3x2F,
            transformed_geometry: *mut *mut ID2D1TransformedGeometry)
            -> HRESULT;
        fn create_path_geometry(
            path_geometry: *mut *mut ID2D1PathGeometry) -> HRESULT;
        fn create_stroke_style(
            stroke_style_properties: *const StrokeStyleProperties,
            dashes: *const FLOAT,
            dashes_count: UINT32,
            stroke_style: *mut *mut ID2D1StrokeStyle) -> HRESULT;
        fn create_drawing_state_block(
            drawing_state_description: *const DrawingStateDescription,
            text_rendering_params: *const dwrite::IDWriteRenderingParams,
            drawing_state_block: *mut *mut ID2D1DrawingStateBlock) -> HRESULT;
        fn create_wic_bitmap_render_target(
            target: *const IUnknown, // TODO IWICBitmap,
            render_target_properties: *const RenderTargetProperties,
            render_target: *mut *mut ID2D1RenderTarget) -> HRESULT;
        fn create_hwnd_render_target(
            render_target_properties: *const RenderTargetProperties,
            hwnd_render_target_properties: *const HwndRenderTargetProperties,
            hwnd_render_target: *mut *mut ID2D1HwndRenderTarget) -> HRESULT;
        fn create_dxgi_surface_render_target(
            dxgi_surface: *const dxgi::IDXGISurface,
            render_target_properties: *const RenderTargetProperties,
            render_target: *mut *mut ID2D1RenderTarget) -> HRESULT;
        fn create_dc_render_target(
            render_target_properties: *const RenderTargetProperties,
            dc_render_target: *mut *mut ID2D1DCRenderTarget) -> HRESULT;
    }
}

#[test]
fn check_d2d1_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID2D1BitmapVtbl>(), 88);
        assert_eq!(size_of::<ID2D1BitmapBrushVtbl>(), 128);
        assert_eq!(size_of::<ID2D1BitmapRenderTargetVtbl>(), 464);
        assert_eq!(size_of::<ID2D1BrushVtbl>(), 64);
        assert_eq!(size_of::<ID2D1DCRenderTargetVtbl>(), 464);
        assert_eq!(size_of::<ID2D1DrawingStateBlockVtbl>(), 64);
        assert_eq!(size_of::<ID2D1EllipseGeometryVtbl>(), 144);
        assert_eq!(size_of::<ID2D1FactoryVtbl>(), 136);
        assert_eq!(size_of::<ID2D1GdiInteropRenderTargetVtbl>(), 40);
        assert_eq!(size_of::<ID2D1GeometryVtbl>(), 136);
        assert_eq!(size_of::<ID2D1GeometryGroupVtbl>(), 160);
        assert_eq!(size_of::<ID2D1GeometrySinkVtbl>(), 120);
        assert_eq!(size_of::<ID2D1GradientStopCollectionVtbl>(), 64);
        assert_eq!(size_of::<ID2D1HwndRenderTargetVtbl>(), 480);
        assert_eq!(size_of::<ID2D1ImageVtbl>(), 32);
        assert_eq!(size_of::<ID2D1LayerVtbl>(), 40);
        assert_eq!(size_of::<ID2D1LinearGradientBrushVtbl>(), 104);
        assert_eq!(size_of::<ID2D1MeshVtbl>(), 40);
        assert_eq!(size_of::<ID2D1PathGeometryVtbl>(), 168);
        assert_eq!(size_of::<ID2D1RadialGradientBrushVtbl>(), 136);
        assert_eq!(size_of::<ID2D1RectangleGeometryVtbl>(), 144);
        assert_eq!(size_of::<ID2D1RenderTargetVtbl>(), 456);
        assert_eq!(size_of::<ID2D1ResourceVtbl>(), 32);
        assert_eq!(size_of::<ID2D1RoundedRectangleGeometryVtbl>(), 144);
        assert_eq!(size_of::<ID2D1SimplifiedGeometrySinkVtbl>(), 80);
        assert_eq!(size_of::<ID2D1SolidColorBrushVtbl>(), 80);
        assert_eq!(size_of::<ID2D1StrokeStyleVtbl>(), 104);
        assert_eq!(size_of::<ID2D1TessellationSinkVtbl>(), 40);
        assert_eq!(size_of::<ID2D1TransformedGeometryVtbl>(), 152);
    } else {
        assert_eq!(size_of::<ID2D1BitmapVtbl>(), 44);
        assert_eq!(size_of::<ID2D1BitmapBrushVtbl>(), 64);
        assert_eq!(size_of::<ID2D1BitmapRenderTargetVtbl>(), 232);
        assert_eq!(size_of::<ID2D1BrushVtbl>(), 32);
        assert_eq!(size_of::<ID2D1DCRenderTargetVtbl>(), 232);
        assert_eq!(size_of::<ID2D1DrawingStateBlockVtbl>(), 32);
        assert_eq!(size_of::<ID2D1EllipseGeometryVtbl>(), 72);
        assert_eq!(size_of::<ID2D1FactoryVtbl>(), 68);
        assert_eq!(size_of::<ID2D1GdiInteropRenderTargetVtbl>(), 20);
        assert_eq!(size_of::<ID2D1GeometryVtbl>(), 68);
        assert_eq!(size_of::<ID2D1GeometryGroupVtbl>(), 80);
        assert_eq!(size_of::<ID2D1GeometrySinkVtbl>(), 60);
        assert_eq!(size_of::<ID2D1GradientStopCollectionVtbl>(), 32);
        assert_eq!(size_of::<ID2D1HwndRenderTargetVtbl>(), 240);
        assert_eq!(size_of::<ID2D1ImageVtbl>(), 16);
        assert_eq!(size_of::<ID2D1LayerVtbl>(), 20);
        assert_eq!(size_of::<ID2D1LinearGradientBrushVtbl>(), 52);
        assert_eq!(size_of::<ID2D1MeshVtbl>(), 20);
        assert_eq!(size_of::<ID2D1PathGeometryVtbl>(), 84);
        assert_eq!(size_of::<ID2D1RadialGradientBrushVtbl>(), 68);
        assert_eq!(size_of::<ID2D1RectangleGeometryVtbl>(), 72);
        assert_eq!(size_of::<ID2D1RenderTargetVtbl>(), 228);
        assert_eq!(size_of::<ID2D1ResourceVtbl>(), 16);
        assert_eq!(size_of::<ID2D1RoundedRectangleGeometryVtbl>(), 72);
        assert_eq!(size_of::<ID2D1SimplifiedGeometrySinkVtbl>(), 40);
        assert_eq!(size_of::<ID2D1SolidColorBrushVtbl>(), 40);
        assert_eq!(size_of::<ID2D1StrokeStyleVtbl>(), 52);
        assert_eq!(size_of::<ID2D1TessellationSinkVtbl>(), 20);
        assert_eq!(size_of::<ID2D1TransformedGeometryVtbl>(), 76);
    }
}
