use winapi::DWORD;

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

#[cfg(test)]
mod tests;

pub const SHARED_RESOURCE_READ: DWORD = 0x80000000;
pub const SHARED_RESOURCE_WRITE: DWORD = 1;
