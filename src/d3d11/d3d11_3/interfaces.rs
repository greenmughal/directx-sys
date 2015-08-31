use std::os::raw::c_void;

use com_rs::IUnknown;
use winapi::{BOOL, HANDLE, HRESULT, UINT};

use super::super::d3d11_0::*;
use super::super::d3d11_1::*;
use super::super::d3d11_2::*;

use super::enums::*;
use super::structs::*;

iid!(IID_ID3D11TEXTURE2D1 =
    0x51218251, 0x1E33, 0x4617, 0x9C, 0xCB, 0x4D, 0x3A, 0x43, 0x67, 0xE7, 0xBB);
com_interface! {
    interface ID3D11Texture2D1: ID3D11Texture2D, ID3D11Resource,
                                ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11TEXTURE2D1,
        vtable: ID3D11Texture2D1Vtbl,
        fn get_desc1(desc: *mut Texture2DDesc1) -> ();
    }
}

iid!(IID_ID3D11TEXTURE3D1 =
    0x0C711683, 0x2853, 0x4846, 0x9B, 0xB0, 0xF3, 0xE6, 0x06, 0x39, 0xE4, 0x6A);
com_interface! {
    interface ID3D11Texture3D1: ID3D11Texture3D, ID3D11Resource,
                                ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11TEXTURE3D1,
        vtable: ID3D11Texture3D1Vtbl,
        fn get_desc1(desc: *mut Texture3DDesc1) -> ();
    }
}

iid!(IID_ID3D11RASTERIZERSTATE2 =
    0x6fbd02fb, 0x209f, 0x46c4, 0xb0, 0x59, 0x2e, 0xd1, 0x55, 0x86, 0xa6, 0xac);
com_interface! {
    interface ID3D11RasterizerState2: ID3D11RasterizerState1,
                                      ID3D11RasterizerState,
                                      ID3D11DeviceChild,
                                      IUnknown {
        iid: IID_ID3D11RASTERIZERSTATE2,
        vtable: ID3D11RasterizerState2Vtbl,
        fn get_desc2(desc: *mut RasterizerDesc2) -> ();
    }
}

iid!(IID_ID3D11SHADERRESOURCEVIEW1 =
    0x91308b87, 0x9040, 0x411d, 0x8c, 0x67, 0xc3, 0x92, 0x53, 0xce, 0x38, 0x02);
com_interface! {
    interface ID3D11ShaderResourceView1: ID3D11ShaderResourceView,
                                         ID3D11View,
                                         ID3D11DeviceChild,
                                         IUnknown {
        iid: IID_ID3D11SHADERRESOURCEVIEW1,
        vtable: ID3D11ShaderResourceView1Vtbl,
        fn get_desc1(desc1: *mut ShaderResourceViewDesc1) -> ();
    }
}

iid!(IID_ID3D11RENDERTARGETVIEW1 =
    0xffbe2e23, 0xf011, 0x418a, 0xac, 0x56, 0x5c, 0xee, 0xd7, 0xc5, 0xb9, 0x4b);
com_interface! {
    interface ID3D11RenderTargetView1: ID3D11RenderTargetView,
                                       ID3D11View,
                                       ID3D11DeviceChild,
                                       IUnknown {
        iid: IID_ID3D11RENDERTARGETVIEW1,
        vtable: ID3D11RenderTargetView1Vtbl,
        fn get_desc1(desc1: *mut RenderTargetViewDesc1) -> ();
    }
}

iid!(IID_ID3D11UNORDEREDACCESSVIEW1 =
    0x7b3b6153, 0xa886, 0x4544, 0xab, 0x37, 0x65, 0x37, 0xc8, 0x50, 0x04, 0x03);
com_interface! {
    interface ID3D11UnorderedAccessView1: ID3D11UnorderedAccessView,
                                          ID3D11View,
                                          ID3D11DeviceChild,
                                          IUnknown {
        iid: IID_ID3D11UNORDEREDACCESSVIEW1,
        vtable: ID3D11UnorderedAccessView1Vtbl,
        fn get_desc1(desc1: *mut UnorderedAccessViewDesc1) -> ();
    }
}

iid!(IID_ID3D11QUERY1 =
    0x631b4766, 0x36dc, 0x461d, 0x8d, 0xb6, 0xc4, 0x7e, 0x13, 0xe6, 0x09, 0x16);
com_interface! {
    interface ID3D11Query1: ID3D11Query,
                            ID3D11Asynchronous,
                            ID3D11DeviceChild,
                            IUnknown {
        iid: IID_ID3D11QUERY1,
        vtable: ID3D11Query1Vtbl,
        fn get_desc1(desc: *mut QueryDesc1) -> ();
    }
}

iid!(IID_ID3D11DEVICECONTEXT3 =
    0xb4e3c01d, 0xe79e, 0x4637, 0x91, 0xb2, 0x51, 0x0e, 0x9f, 0x4c, 0x9b, 0x8f);
com_interface! {
    interface ID3D11DeviceContext3: ID3D11DeviceContext2,
                                    ID3D11DeviceContext1,
                                    ID3D11DeviceContext,
                                    ID3D11DeviceChild,
                                    IUnknown {
        iid: IID_ID3D11DEVICECONTEXT3,
        vtable: ID3D11DeviceContext3Vtbl,
        fn flush1(context_type: ContextType, event: HANDLE) -> ();
        fn set_hardware_protection_state(hw_protection_enable: BOOL) -> ();
        fn get_hardware_protection_state(hw_protection_enable: *mut BOOL) -> ();
    }
}

iid!(IID_ID3D11DEVICE3 =
    0xA05C8C37, 0xD2C6, 0x4732, 0xB3, 0xA0, 0x9C, 0xE0, 0xB0, 0xDC, 0x9A, 0xE6);
com_interface! {
    interface ID3D11Device3: ID3D11Device2,
                             ID3D11Device1,
                             ID3D11Device,
                             IUnknown {
        iid: IID_ID3D11DEVICE3,
        vtable: ID3D11Device3Vtbl,
        fn create_texture2d1(
            desc1: *const Texture2DDesc1,
            initial_data: *const SubresourceData,
            texture_2d: *mut *mut ID3D11Texture2D1) -> HRESULT;
        fn create_texture3d1(
            desc1: *const Texture3DDesc1,
            initial_data: *const SubresourceData,
            texture_3d: *mut *mut ID3D11Texture3D1) -> HRESULT;
        fn create_rasterizer_state2(
            rasterizer_desc: *const RasterizerDesc2,
            rasterizer_state: *mut *mut ID3D11RasterizerState2) -> HRESULT;
        fn create_shader_resource_view1(
            resource: *const ID3D11Resource,
            desc1: *const ShaderResourceViewDesc1,
            view1: *mut *mut ID3D11ShaderResourceView1) -> HRESULT;
        fn create_unordered_access_view1(
            resource: *const ID3D11Resource,
            desc1: *const UnorderedAccessViewDesc1,
            view1: *mut *mut ID3D11UnorderedAccessView1) -> HRESULT;
        fn create_render_target_view1(
            resource: *const ID3D11Resource,
            desc1: *const RenderTargetViewDesc1,
            view1: *mut *mut ID3D11RenderTargetView1) -> HRESULT;
        fn create_query1(
            query_desc1: *const QueryDesc1,
            query1: *mut *mut ID3D11Query1) -> HRESULT;
        fn get_immediate_context3(
            immediate_context: *mut *mut ID3D11DeviceContext3) -> ();
        fn create_deferred_context3(
            context_flags: UINT,
            deferred_context: *mut *mut ID3D11DeviceContext3) -> HRESULT;
        fn write_to_subresource(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_box: *const BoxRegion,
            src_data: *const c_void,
            src_row_pitch: UINT,
            src_depth_pitch: UINT) -> ();
        fn read_from_subresource(
            dst_data: *mut c_void,
            dst_row_pitch: UINT,
            dst_depth_pitch: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            src_box: *const BoxRegion) -> ();
    }
}

#[test]
fn check_d3d11_3_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID3D11Texture2D1Vtbl>(), 96);
        assert_eq!(size_of::<ID3D11Texture3D1Vtbl>(), 96);
        assert_eq!(size_of::<ID3D11RasterizerState2Vtbl>(), 80);
        assert_eq!(size_of::<ID3D11ShaderResourceView1Vtbl>(), 80);
        assert_eq!(size_of::<ID3D11RenderTargetView1Vtbl>(), 80);
        assert_eq!(size_of::<ID3D11UnorderedAccessView1Vtbl>(), 80);
        assert_eq!(size_of::<ID3D11Query1Vtbl>(), 80);
        assert_eq!(size_of::<ID3D11DeviceContext3Vtbl>(), 1176);
        assert_eq!(size_of::<ID3D11Device3Vtbl>(), 520);
    } else {
        assert_eq!(size_of::<ID3D11Texture2D1Vtbl>(), 48);
        assert_eq!(size_of::<ID3D11Texture3D1Vtbl>(), 48);
        assert_eq!(size_of::<ID3D11RasterizerState2Vtbl>(), 40);
        assert_eq!(size_of::<ID3D11ShaderResourceView1Vtbl>(), 40);
        assert_eq!(size_of::<ID3D11RenderTargetView1Vtbl>(), 40);
        assert_eq!(size_of::<ID3D11UnorderedAccessView1Vtbl>(), 40);
        assert_eq!(size_of::<ID3D11Query1Vtbl>(), 40);
        assert_eq!(size_of::<ID3D11DeviceContext3Vtbl>(), 588);
        assert_eq!(size_of::<ID3D11Device3Vtbl>(), 260);
    }
}
