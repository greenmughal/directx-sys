/*!
# directx-sys 0.0.1
Rust FFI bindings for the DirectX API.

# Components
* [DXGI](dxgi/index.html) - mostly complete but untested.
* [D3D11](d3d11/index.html) - 11.0 API complete but untested.
  11.1/11.2 not yet implemented.
* D3DCompiler - not yet implemented.
* Direct2D - not yet implemented.
* DirectWrite - not yet implemented.
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

#[cfg(feature = "d3d11")]
pub mod d3d11;