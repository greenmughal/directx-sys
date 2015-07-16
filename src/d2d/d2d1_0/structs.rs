use winapi::{FLOAT, HWND, UINT32, UINT64};

use dxgi;
use super::enums::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PixelFormat {
    pub format: dxgi::Format,
    pub alpha_mode: AlphaMode
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Point2U {
    pub x: UINT32,
    pub y: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point2F {
    pub x: FLOAT,
    pub y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2F {
    pub x: FLOAT,
    pub y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3F {
    pub x: FLOAT,
    pub y: FLOAT,
    pub z: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector4F {
    pub x: FLOAT,
    pub y: FLOAT,
    pub z: FLOAT,
    pub w: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub left: FLOAT,
    pub top: FLOAT,
    pub right: FLOAT,
    pub bottom: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RectU {
    pub left: UINT32,
    pub top: UINT32,
    pub right: UINT32,
    pub bottom: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SizeF {
    pub width: FLOAT,
    pub height: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SizeU {
    pub width: UINT32,
    pub height: UINT32
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorF {
    pub r: FLOAT,
    pub g: FLOAT,
    pub b: FLOAT,
    pub a: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x2F {
    pub m: [[FLOAT; 2]; 3]
}

impl Default for Matrix3x2F {
    fn default() -> Matrix3x2F {
        Matrix3x2F { m: [[0.0; 2]; 3] }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x3F {
    pub m: [[FLOAT; 3]; 4]
}

impl Default for Matrix4x3F {
    fn default() -> Matrix4x3F {
        Matrix4x3F { m: [[0.0; 3]; 4] }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x4F {
    pub m: [[FLOAT; 4]; 4]
}

impl Default for Matrix4x4F {
    fn default() -> Matrix4x4F {
        Matrix4x4F { m: [[0.0; 4]; 4] }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix5x4F {
    pub m: [[FLOAT; 4]; 5]
}

impl Default for Matrix5x4F {
    fn default() -> Matrix5x4F {
        Matrix5x4F { m: [[0.0; 4]; 5] }
    }
}

pub type Tag = UINT64;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BitmapProperties {
    pub pixel_format: PixelFormat,
    pub dpi_x: FLOAT,
    pub dpi_y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GradientStop {
    pub position: FLOAT,
    pub color: ColorF
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BrushProperties {
    pub opacity: FLOAT,
    pub transform: Matrix3x2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapBrushProperties {
    pub extend_mode_x: ExtendMode,
    pub extend_mode_y: ExtendMode,
    pub interpolation_mode: BitmapInterpolationMode
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LinearGradientBrushProperties {
    pub start_point: Point2F,
    pub end_point: Point2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RadialGradientBrushProperties {
    pub center: Point2F,
    pub gradient_origin_offset: Point2F,
    pub radius_x: FLOAT,
    pub radius_y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BezierSegment {
    pub point_1: Point2F,
    pub point_2: Point2F,
    pub point_3: Point2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Triangle {
    pub point_1: Point2F,
    pub point_2: Point2F,
    pub point_3: Point2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ArcSegment {
    pub point: Point2F,
    pub size: SizeF,
    pub rotation_angle: FLOAT,
    pub sweep_direction: SweepDirection,
    pub arc_size: ArcSize
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QuadraticBezierSegment {
    pub point_1: Point2F,
    pub point_2: Point2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ellipse {
    pub point: Point2F,
    pub radius_x: FLOAT,
    pub radius_y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoundedRect {
    pub rect: RectF,
    pub radius_x: FLOAT,
    pub radius_y: FLOAT
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StrokeStyleProperties {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: FLOAT,
    pub dash_style: DashStyle,
    pub dash_offset: FLOAT
}

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Debug)]
pub struct LayerParameters {
    pub content_bounds: RectF,
    pub geometric_mask: *const super::ID2D1Geometry,
    pub mask_antialias_mode: AntialiasMode,
    pub mask_transform: Matrix3x2F,
    pub opacity: FLOAT,
    pub opacity_brush: *const super::ID2D1Brush,
    pub layer_options: LayerOptions
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RenderTargetProperties {
    pub rt_type: RenderTargetType,
    pub pixel_format: PixelFormat,
    pub dpi_x: FLOAT,
    pub dpi_y: FLOAT,
    pub usage: RenderTargetUsage,
    pub min_level: FeatureLevel
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HwndRenderTargetProperties {
    pub hwnd: HWND,
    pub pixel_size: SizeU,
    pub present_options: PresentOptions
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DrawingStateDescription {
    pub antialias_mode: AntialiasMode,
    pub text_antialias_mode: TextAntialiasMode,
    pub tag1: Tag,
    pub tag2: Tag,
    pub transform: Matrix3x2F
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FactoryOptions {
    pub debug_level: DebugLevel
}

#[test]
fn check_d2d1l_struct_sizes() {
    use std::mem::size_of;

    assert_eq!(size_of::<ArcSegment>(), 28);
    assert_eq!(size_of::<BezierSegment>(), 24);
    assert_eq!(size_of::<BitmapBrushProperties>(), 12);
    assert_eq!(size_of::<BitmapProperties>(), 16);
    assert_eq!(size_of::<BrushProperties>(), 28);
    assert_eq!(size_of::<ColorF>(), 16);
    assert_eq!(size_of::<DrawingStateDescription>(), 48);
    assert_eq!(size_of::<Ellipse>(), 16);
    assert_eq!(size_of::<FactoryOptions>(), 4);
    assert_eq!(size_of::<GradientStop>(), 20);
    assert_eq!(size_of::<LinearGradientBrushProperties>(), 16);
    assert_eq!(size_of::<Matrix3x2F>(), 24);
    assert_eq!(size_of::<Matrix4x3F>(), 48);
    assert_eq!(size_of::<Matrix4x4F>(), 64);
    assert_eq!(size_of::<Matrix5x4F>(), 80);
    assert_eq!(size_of::<PixelFormat>(), 8);
    assert_eq!(size_of::<Point2F>(), 8);
    assert_eq!(size_of::<Point2U>(), 8);
    assert_eq!(size_of::<QuadraticBezierSegment>(), 16);
    assert_eq!(size_of::<RadialGradientBrushProperties>(), 24);
    assert_eq!(size_of::<RectF>(), 16);
    assert_eq!(size_of::<RectU>(), 16);
    assert_eq!(size_of::<RenderTargetProperties>(), 28);
    assert_eq!(size_of::<RoundedRect>(), 24);
    assert_eq!(size_of::<SizeF>(), 8);
    assert_eq!(size_of::<SizeU>(), 8);
    assert_eq!(size_of::<StrokeStyleProperties>(), 28);
    assert_eq!(size_of::<Triangle>(), 24);
    assert_eq!(size_of::<Vector2F>(), 8);
    assert_eq!(size_of::<Vector3F>(), 12);
    assert_eq!(size_of::<Vector4F>(), 16);

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<HwndRenderTargetProperties>(), 24);
        assert_eq!(size_of::<LayerParameters>(), 72);
    } else {
        assert_eq!(size_of::<HwndRenderTargetProperties>(), 16);
        assert_eq!(size_of::<LayerParameters>(), 60);
    }
}
