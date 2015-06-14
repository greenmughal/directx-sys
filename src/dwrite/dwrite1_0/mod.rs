use com_rs::IUnknown;
use winapi::{HRESULT, REFIID};

pub use self::enums::*;
pub use self::interfaces::*;
pub use self::structs::*;

mod enums;
mod interfaces;
mod structs;

#[link(name = "dwrite")]
extern "stdcall" {
    pub fn DWriteCreateFactory(
        factory_type: FactoryType,
        iid: REFIID,
        factory: *mut *mut IUnknown) -> HRESULT;
}

#[test]
fn dwrite_test_create_factory() {
    use com_rs::ComPtr;

    let mut factory: ComPtr<IDWriteFactory> = ComPtr::new();
    let result = unsafe {
        DWriteCreateFactory(FactoryType::Shared, &factory.iid(),
                            factory.as_mut())
    };
    assert_eq!(result, 0);
    assert!(!factory.is_null());
}