use winapi::UINT;

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

pub const PACKED_TILE: UINT = 0xffffffff;
