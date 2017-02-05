use std::env;

fn main() {
    println!("cargo:rustc-link-lib=dylib=dxguid");
    if env::var("CARGO_FEATURE_D3D11").is_ok() {
        println!("cargo:rustc-link-lib=dylib=d3d11");
    }
    if env::var("CARGO_FEATURE_D2D").is_ok() {
        println!("cargo:rustc-link-lib=dylib=d2d1");
    }
    if env::var("CARGO_FEATURE_DWRITE").is_ok() {
        println!("cargo:rustc-link-lib=dylib=dwrite");
    }
    if env::var("CARGO_FEATURE_DXGI").is_ok() {
        println!("cargo:rustc-link-lib=dylib=dxgi");
    }
}
