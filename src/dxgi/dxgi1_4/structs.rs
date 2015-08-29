use winapi::UINT64;

#[repr(C)]
#[derive(Debug)]
pub struct QueryVideoMemoryInfo {
    pub budget: UINT64,
    pub current_usage: UINT64,
    pub available_for_reservation: UINT64,
    pub current_reservation: UINT64,
}

#[test]
fn check_dxgi1_4_struct_sizes() {
    use std::mem::size_of;
    assert_eq!(size_of::<QueryVideoMemoryInfo>(), 32);
}
