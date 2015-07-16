use com_rs::ComPtr;

use dxgi::CreateDXGIFactory;
use super::*;

#[test]
fn dxgi_create_factory2() {
    let mut factory: ComPtr<IDXGIFactory2> = ComPtr::new();
    let result = unsafe {
        CreateDXGIFactory(&factory.iid(), factory.as_mut())
    };

    assert_eq!(result, 0);
    assert!(!factory.is_null());
}
