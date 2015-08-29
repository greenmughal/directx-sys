pub use self::dxgi1_0::*;
mod dxgi1_0;

#[cfg(feature = "dxgi1_2")]
pub use self::dxgi1_2::*;
#[cfg(feature = "dxgi1_2")]
mod dxgi1_2;

#[cfg(feature = "dxgi1_3")]
pub use self::dxgi1_3::*;
#[cfg(feature = "dxgi1_3")]
mod dxgi1_3;

#[cfg(feature = "dxgi1_4")]
pub use self::dxgi1_4::*;
#[cfg(feature = "dxgi1_4")]
mod dxgi1_4;
