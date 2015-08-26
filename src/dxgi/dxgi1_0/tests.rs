use com_rs::ComPtr;

use super::*;

#[test]
fn dxgi_create_factory() {
    let mut factory: ComPtr<IDXGIFactory> = ComPtr::new();
    let result = unsafe {
        CreateDXGIFactory(&factory.iid(), factory.as_mut_ptr())
    };

    assert_eq!(result, 0);
    assert!(!factory.is_null());
}

#[test]
fn dxgi_create_factory1() {
    let mut factory: ComPtr<IDXGIFactory1> = ComPtr::new();
    let result = unsafe {
        CreateDXGIFactory1(&factory.iid(), factory.as_mut_ptr())
    };

    assert_eq!(result, 0);
    assert!(!factory.is_null());
}

#[test]
fn dxgi_enum_adapters() {
    let mut factory: ComPtr<IDXGIFactory> = ComPtr::new();
    unsafe {
        CreateDXGIFactory(&factory.iid(), factory.as_mut_ptr());
        assert!(!factory.is_null());

        let mut adapter: ComPtr<IDXGIAdapter> = ComPtr::new();
        let mut index = 0;
        loop {
            match factory.enum_adapters(index, adapter.as_mut_ptr()) {
                0 => {
                    assert!(!adapter.is_null());
                    index += 1;
                },
                _ => break
            }
        }
    }
}

#[test]
fn dxgi_enum_adapters1() {
    let mut factory: ComPtr<IDXGIFactory1> = ComPtr::new();
    unsafe {
        CreateDXGIFactory1(&factory.iid(), factory.as_mut_ptr());
        assert!(!factory.is_null());

        let mut adapter: ComPtr<IDXGIAdapter1> = ComPtr::new();
        let mut index = 0;
        loop {
            match factory.enum_adapters1(index, adapter.as_mut_ptr()) {
                0 => {
                    assert!(!adapter.is_null());
                    index += 1;
                },
                _ => break
            }
        }
    }
}
