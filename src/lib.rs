/*!
# directx-sys 0.0.5
Rust FFI bindings for the DirectX API.

# Components
* [DXGI](dxgi/index.html) - mostly complete but untested.
* [D3D11](d3d11/index.html) - 11.0 API complete but untested.
  11.1/11.2 not yet implemented.
* D3DCompiler - not yet implemented.
* [Direct2D](d2d/index.html) - 1.0 API mostly complete but untested.
  1.1/1.2 not yet implemented.
* [DirectWrite](dwrite/index.html) - 1.0 API complete but untested.
  1.1/1.2 not yet implemented.
* XAudio2 - not yet implemented.
* XInput - not yet implemented.

# Unsupported components
The following DirectX components are considered deprecated so I will not be
writing bindings for them:

* D3D9/10 - use D3D11 instead.
* DirectDraw - use Direct2D instead
* DirectInput - Use XInput instead
* DirectSound - use XAudio2 instead.

*/

#![deny(missing_debug_implementations)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate com_rs;
extern crate libc;
extern crate winapi;

#[macro_use]
mod macros;

#[cfg(feature = "dxgi")]
pub mod dxgi;
#[cfg(not(feature = "dxgi"))]
pub mod dxgi {
    //! DXGI disabled, enable `dxgi` feature to use.
}

#[cfg(feature = "d3d11")]
pub mod d3d11;
#[cfg(not(feature = "d3d11"))]
pub mod d3d11 {
    //! D3D11 disabled, enable `d3d11` feature to use.
}

#[cfg(feature = "d2d")]
pub mod d2d;
#[cfg(not(feature = "d2d"))]
pub mod d2d {
    //! Direct2D disabled, enable `d2d` feature to use.
}

#[cfg(feature = "dwrite")]
pub mod dwrite;
#[cfg(not(feature = "dwrite"))]
pub mod dwrite {
    //! DirectWrite disabled, enable `dwrite` feature to use.
}
