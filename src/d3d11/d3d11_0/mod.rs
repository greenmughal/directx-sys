use winapi::{HMODULE, HRESULT, UINT};

use dxgi;

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

pub const SDK_VERSION: UINT = 7;

#[link(name = "d3d11")]
extern "stdcall" {
    pub fn D3D11CreateDevice(
        adapter: *const dxgi::IDXGIAdapter,
        driver_type: DriverType,
        software: HMODULE,
        flags: CreateDeviceFlag,
        feature_levels: *const FeatureLevel,
        num_feature_levels: UINT,
        sdk_version: UINT,
        device: *mut *mut ID3D11Device,
        feature_level: *mut FeatureLevel,
        immediate_context: *mut *mut ID3D11DeviceContext) -> HRESULT;
    pub fn D3D11CreateDeviceAndSwapChain(
        adapter: *const dxgi::IDXGIAdapter,
        driver_type: DriverType,
        software: HMODULE,
        flags: CreateDeviceFlag,
        feature_levels: *const FeatureLevel,
        num_feature_levels: UINT,
        sdk_version: UINT,
        swap_chain_desc: *const dxgi::SwapChainDesc,
        swap_chain: *mut *mut dxgi::IDXGISwapChain,
        device: *mut *mut ID3D11Device,
        feature_level: *mut FeatureLevel,
        immediate_context: *mut *mut ID3D11DeviceContext) -> HRESULT;
}