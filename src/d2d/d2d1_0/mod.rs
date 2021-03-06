use std::os::raw::c_void;

use com_rs::IID;
use winapi::{BOOL, FLOAT, HRESULT};

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

extern "stdcall" {
    pub fn D2D1CreateFactory(
        factory_type: FactoryType,
        iid: &IID,
        factory_options: *const FactoryOptions,
        factory: *mut *mut c_void) -> HRESULT;
    pub fn D2D1MakeRotateMatrix(
        angle: FLOAT,
        center: Point2F,
        matrix: *mut Matrix3x2F);
    pub fn D2D1MakeSkewMatrix(
        angle_x: FLOAT,
        angle_y: FLOAT,
        center: Point2F,
        matrix: *mut Matrix3x2F);
    pub fn D2D1IsMatrixInvertible(matrix: *const Matrix3x2F) -> BOOL;
    pub fn D2D1InvertMatrix(matrix: *mut Matrix3x2F) -> BOOL;
}

// TODO #include <d2d1helper.h>
