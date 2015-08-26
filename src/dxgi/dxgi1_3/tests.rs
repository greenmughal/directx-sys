use com_rs::ComPtr;

use super::*;

#[test]
fn dxgi_create_factory3() {
    let mut factory: ComPtr<IDXGIFactory3> = ComPtr::new();
    let result = unsafe {
        CreateDXGIFactory2(CreateFactoryFlags::empty(),
                          &factory.iid(), factory.as_mut_ptr())
    };

    assert_eq!(result, 0);
    assert!(!factory.is_null());
}
