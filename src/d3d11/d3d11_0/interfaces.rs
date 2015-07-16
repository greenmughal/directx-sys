use std::os::raw::c_void;

use com_rs::{IUnknown, Unknown};
use winapi::{BOOL, FLOAT, HANDLE, HRESULT, INT, LPCSTR, LPSTR, REFGUID, REFIID,
             SIZE_T, UINT, UINT8};

use dxgi;
use super::enums::*;
use super::structs::*;

com_interface! {
    struct ID3D11DeviceChild: IUnknown {
        iid: IID_ID3D11DEVICECHILD {
            0x1841E5C8, 0x16B0, 0x489B,
            0xBC, 0xC8, 0x44, 0xCF, 0xB0, 0xD5, 0xDE, 0xAE
        },
        vtable: ID3D11DeviceChildVtbl
    }

    trait D3D11DeviceChild: Unknown {
        fn get_device(device: *mut *mut ID3D11Device) -> (),
        fn get_private_data(
            guid: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void) -> HRESULT,
        fn set_private_data(
            guid: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT,
        fn set_private_data_interface(
            guid: REFGUID,
            data: *const IUnknown) -> HRESULT
    }
}

com_interface! {
    struct ID3D11DepthStencilState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DEPTHSTENCILSTATE {
            0x03823EFB, 0x8D8F, 0x4E1C,
            0x9A, 0xA2, 0xF6, 0x4B, 0xB2, 0xCB, 0xFD, 0xF1
        },
        vtable: ID3D11DepthStencilStateVtbl
    }

    trait D3D11DepthStencilState: D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut DepthStencilDesc) -> ()
    }
}

com_interface! {
    struct ID3D11BlendState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11BLENDSTATE {
            0x75B68FAA, 0x347D, 0x4159,
            0x8F, 0x45, 0xA0, 0x64, 0x0F, 0x01, 0xCD, 0x9A
        },
        vtable: ID3D11BlendStateVtbl
    }

    trait D3D11BlendState: D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut BlendDesc) -> ()
    }
}

com_interface! {
    struct ID3D11RasterizerState: ID3D11DeviceChild, IUnknown {
        iid: IID {
            0x9BB4AB81, 0xAB1A, 0x4D8F,
            0xB5, 0x06, 0xFC, 0x04, 0x20, 0x0B, 0x6E, 0xE7
        },
        vtable: ID3D11RasterizerStateVtbl
    }

    trait D3D11RasterizerState: D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut RasterizerDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Resource: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11RESOURCE {
            0xDC8E63F3, 0xD12B, 0x4952,
            0xB4, 0x7B, 0x5E, 0x45, 0x02, 0x6A, 0x86, 0x2D
        },
        vtable: ID3D11ResourceVtbl
    }

    trait D3D11Resource: D3D11DeviceChild, Unknown {
        fn get_type(resource_dimension: *mut ResourceDimension) -> (),
        fn set_eviction_priority(eviction_priority: UINT) -> (),
        fn get_eviction_priority() -> UINT
    }
}

com_interface! {
    struct ID3D11Buffer: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11BUFFER {
            0x48570B85, 0xD1EE, 0x4FCD,
            0xA2, 0x50, 0xEB, 0x35, 0x07, 0x22, 0xB0, 0x37
        },
        vtable: ID3D11BufferVtbl
    }

    trait D3D11Buffer: D3D11Resource, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut BufferDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Texture1D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11TEXTURE1D {
            0xF8FB5C27, 0xC6B3, 0x4F75,
            0xA4, 0xC8, 0x43, 0x9A, 0xF2, 0xEF, 0x56, 0x4C
        },
        vtable: ID3D11Texture1DVtbl
    }

    trait D3D11Texture1D: D3D11Resource, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut Texture1DDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Texture2D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11TEXTURE2D {
            0x6F15AAF2, 0xD208, 0x4E89,
            0x9A, 0xB4, 0x48, 0x95, 0x35, 0xD3, 0x4F, 0x9C
        },
        vtable: ID3D11Texture2DVtbl
    }

    trait D3D11Texture2D: D3D11Resource, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut Texture2DDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Texture3D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11TEXTURE3D {
            0x037E866E, 0xF56D, 0x4357,
            0xA8, 0xAF, 0x9D, 0xAB, 0xBE, 0x6E, 0x25, 0x0E
        },
        vtable: ID3D11Texture3DVtbl
    }

    trait D3D11Texture3D: D3D11Resource, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut Texture3DDesc) -> ()
    }
}

com_interface! {
    struct ID3D11View: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VIEW {
            0x839D1216, 0xBB2E, 0x412B,
            0xB7, 0xF4, 0xA9, 0xDB, 0xEB, 0xE0, 0x8E, 0xD1
        },
        vtable: ID3D11ViewVtbl
    }

    trait D3D11View: D3D11DeviceChild, Unknown {
        fn get_resource(resource: *mut *mut ID3D11Resource) -> ()
    }
}

com_interface! {
    struct ID3D11ShaderResourceView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11SHADERRESOURCEVIEW {
            0xB0E06FE0, 0x8192, 0x4E1A,
            0xB1, 0xCA, 0x36, 0xD7, 0x41, 0x47, 0x10, 0xB2
        },
        vtable: ID3D11ShaderResourceViewVtbl
    }

    trait D3D11ShaderResourceView: D3D11View, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut ShaderResourceViewDesc) -> ()
    }
}

com_interface! {
    struct ID3D11RenderTargetView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11RENDERTARGETVIEW {
            0xDFDBA067, 0x0B8D, 0x4865,
            0x87, 0x5B, 0xD7, 0xB4, 0x51, 0x6C, 0xC1, 0x64
        },
        vtable: ID3D11RenderTargetViewVtbl
    }

    trait D3D11RenderTargetView: D3D11View, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut RenderTargetViewDesc) -> ()
    }
}

com_interface! {
    struct ID3D11DepthStencilView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DEPTHSTENCILVIEW {
            0x9FDAC92A, 0x1876, 0x48C3,
            0xAF, 0xAD, 0x25, 0xB9, 0x4F, 0x84, 0xA9, 0xB6
        },
        vtable: ID3D11DepthStencilViewVtbl
    }

    trait D3D11DepthStencilView: D3D11View, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut DepthStencilViewDesc) -> ()
    }
}

com_interface! {
    struct ID3D11UnorderedAccessView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11UNORDEREDACCESSVIEW {
            0x28ACF509, 0x7F5C, 0x48F6,
            0x86, 0x11, 0xF3, 0x16, 0x01, 0x0A, 0x63, 0x80
        },
        vtable: ID3D11UnorderedAccessViewVtbl
    }

    trait D3D11UnorderedAccessView: D3D11View, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut UnorderedAccessViewDesc) -> ()
    }
}

com_interface! {
    struct ID3D11VertexShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VERTEXSHADER {
            0x3B301D64, 0xD678, 0x4289,
            0x88, 0x97, 0x22, 0xF8, 0x92, 0x8B, 0x72, 0xF3
        },
        vtable: ID3D11VertexShaderVtbl
    }

    trait D3D11VertexShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11HullShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11HULLSHADER {
            0x8E5C6061, 0x628A, 0x4C8E,
            0x82, 0x64, 0xBB, 0xE4, 0x5C, 0xB3, 0xD5, 0xDD
        },
        vtable: ID3D11HullShaderVtbl
    }

    trait D3D11HullShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11DomainShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DOMAINSHADER {
            0xF582C508, 0x0F36, 0x490C,
            0x99, 0x77, 0x31, 0xEE, 0xCE, 0x26, 0x8C, 0xFA
        },
        vtable: ID3D11DomainShaderVtbl
    }

    trait D3D11DomainShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11GeometryShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11GEOMETRYSHADER {
            0x38325B96, 0xEFFB, 0x4022,
            0xBA, 0x02, 0x2E, 0x79, 0x5B, 0x70, 0x27, 0x5C
        },
        vtable: ID3D11GeometryShaderVtbl
    }

    trait D3D11GeometryShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11PixelShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11PIXELSHADER {
            0xEA82E40D, 0x51DC, 0x4F33,
            0x93, 0xD4, 0xDB, 0x7C, 0x91, 0x25, 0xAE, 0x8C
        },
        vtable: ID3D11PixelShaderVtbl
    }

    trait D3D11PixelShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11ComputeShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11COMPUTESHADER {
            0x4F5B196E, 0xC2BD, 0x495E,
            0xBD, 0x01, 0x1F, 0xDE, 0xD3, 0x8E, 0x49, 0x69
        },
        vtable: ID3D11ComputeShaderVtbl
    }

    trait D3D11ComputeShader: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11InputLayout: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11INPUTLAYOUT {
            0xE4819DDC, 0x4CF0, 0x4025,
            0xBD, 0x26, 0x5D, 0xE8, 0x2A, 0x3E, 0x07, 0xB7
        },
        vtable: ID3D11InputLayoutVtbl
    }

    trait D3D11InputLayout: D3D11DeviceChild, Unknown { }
}

com_interface! {
    struct ID3D11SamplerState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11SAMPLERSTATE {
            0xDA6FEA51, 0x564C, 0x4487,
            0x98, 0x10, 0xF0, 0xD0, 0xF9, 0xB4, 0xE3, 0xA5
        },
        vtable: ID3D11SamplerStateVtbl
    }

    trait D3D11SamplerState: D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut SamplerDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Asynchronous: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11ASYNCHRONOUS {
            0x4B35D0CD, 0x1E15, 0x4258,
            0x9C, 0x98, 0x1B, 0x13, 0x33, 0xF6, 0xDD, 0x3B
        },
        vtable: ID3D11AsynchronousVtbl
    }

    trait D3D11Asynchronous: D3D11DeviceChild, Unknown {
        fn get_data_size() -> UINT
    }
}

com_interface! {
    struct ID3D11Query: ID3D11Asynchronous, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11QUERY {
            0xD6C00747, 0x87B7, 0x425E,
            0xB8, 0x4D, 0x44, 0xD1, 0x08, 0x56, 0x0A, 0xFD
        },
        vtable: ID3D11QueryVtbl
    }

    trait D3D11Query: D3D11Asynchronous, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut QueryDesc) -> ()
    }
}

com_interface! {
    struct ID3D11Predicate: ID3D11Query, ID3D11Asynchronous, ID3D11DeviceChild,
                            IUnknown {
        iid: IID_ID3D11PREDICATE {
            0x9EB576DD, 0x9F77, 0x4D86,
            0x81, 0xAA, 0x8B, 0xAB, 0x5F, 0xE4, 0x90, 0xE2
        },
        vtable: ID3D11PredicateVtbl
    }

    trait D3D11Predicate: D3D11Query, D3D11Asynchronous, D3D11DeviceChild,
                          Unknown { }
}

com_interface! {
    struct ID3D11Counter: ID3D11Asynchronous, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11COUNTER {
            0x6E8C49FB, 0xA371, 0x4770,
            0xB4, 0x40, 0x29, 0x08, 0x60, 0x22, 0xB7, 0x41
        },
        vtable: ID3D11CounterVtbl
    }

    trait D3D11Counter: D3D11Asynchronous, D3D11DeviceChild, Unknown {
        fn get_desc(desc: *mut CounterDesc) -> ()
    }
}

com_interface! {
    struct ID3D11ClassInstance: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11CLASSINSTANCE {
            0xA6CD7FAA, 0xB0B7, 0x4A2F,
            0x94, 0x36, 0x86, 0x62, 0xA6, 0x57, 0x97, 0xCB
        },
        vtable: ID3D11ClassInstanceVtbl
    }

    trait D3D11ClassInstance: D3D11DeviceChild, Unknown {
        fn get_class_linkage(linkage: *mut *mut ID3D11ClassLinkage) -> (),
        fn get_desc(desc: *mut ClassInstanceDesc) -> (),
        fn get_instance_name(
            instance_name: LPSTR,
            buffer_length: *mut SIZE_T) -> (),
        fn get_type_name(
            type_name: LPSTR,
            buffer_length: *mut SIZE_T) -> ()
    }
}

com_interface! {
    struct ID3D11ClassLinkage: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11CLASSLINKAGE {
            0xDDF57CBA, 0x9543, 0x46E4,
            0xA1, 0x2B, 0xF2, 0x07, 0xA0, 0xFE, 0x7F, 0xED
        },
        vtable: ID3D11ClassLinkageVtbl
    }

    trait D3D11ClassLinkage: D3D11DeviceChild, Unknown {
        fn get_class_instance(
            class_instance_name: LPCSTR,
            instance_index: UINT,
            instance: *mut *mut ID3D11ClassInstance) -> HRESULT,
        fn create_class_instance(
            class_type_name: LPCSTR,
            constant_buffer_offset: UINT,
            constant_vector_offset: UINT,
            texture_offset: UINT,
            sampler_offset: UINT,
            instance: *mut *mut ID3D11ClassInstance) -> HRESULT
    }
}

com_interface! {
    struct ID3D11CommandList: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11COMMANDLIST {
            0xA24BC4D1, 0x769E, 0x43F7,
            0x80, 0x13, 0x98, 0xFF, 0x56, 0x6C, 0x18, 0xE2
        },
        vtable: ID3D11CommandListVtbl
    }

    trait D3D11CommandList: D3D11DeviceChild, Unknown {
        fn get_context_flags() -> UINT
    }
}

com_interface! {
    struct ID3D11DeviceContext: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DEVICECONTEXT {
            0xC0BFA96C, 0xE089, 0x44FB,
            0x8E, 0xAF, 0x26, 0xF8, 0x79, 0x61, 0x90, 0xDA
        },
        vtable: ID3D11DeviceContextVtbl
    }

    trait D3D11DeviceContext: D3D11DeviceChild, Unknown {
        fn vs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> (),
        fn ps_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn ps_set_shader(
            pixel_shader: *const ID3D11PixelShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn ps_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn vs_set_shader(
            vertex_shader: *const ID3D11VertexShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn draw_indexed(
            index_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT) -> (),
        fn draw(
            vertex_count: UINT,
            start_vertex_location: UINT) -> (),
        fn map(
            resource: *const ID3D11Resource,
            subresource: UINT,
            map_type: Map,
            map_flags: MapFlag,
            mapped_resource: *mut MappedSubresource) -> HRESULT,
        fn unmap(
            resource: *const ID3D11Resource,
            subresource: UINT) -> (),
        fn ps_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            input_layout: *const ID3D11InputLayout) -> (),
        fn ia_set_input_layout(input_layout: *const ID3D11InputLayout) -> (),
        fn ia_set_vertex_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            vertex_buffers: *const *const ID3D11Buffer,
            strides: *const UINT,
            offsets: *const UINT) -> (),
        fn ia_set_index_buffer(
            index_buffer: *const ID3D11Buffer,
            format: dxgi::Format,
            offset: UINT) -> (),
        fn draw_indexed_instanced(
            index_count_per_instance: UINT,
            instance_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT,
            start_instance_location: UINT) -> (),
        fn draw_instanced(
            vertex_count_per_instance: UINT,
            instance_count: UINT,
            start_vertex_location: UINT,
            start_instance_location: UINT) -> (),
        fn gs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> (),
        fn gs_set_shader(
            geometry_shader: *const ID3D11GeometryShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn ia_set_primitive_topology(topology: PrimitiveTopology) -> (),
        fn vs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn vs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn begin(async: *const ID3D11Asynchronous) -> (),
        fn end(async: *const ID3D11Asynchronous) -> (),
        fn get_data(
            async: *const ID3D11Asynchronous,
            data: *mut c_void,
            data_size: UINT,
            get_data_flags: AsyncGetDataFlag) -> HRESULT,
        fn set_predication(
            predicate: *const ID3D11Predicate,
            predicate_value: BOOL) -> (),
        fn gs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn gs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn om_set_render_targets(
            num_views: UINT,
            render_target_views: *const *const ID3D11RenderTargetView,
            depth_stencil_view: *const ID3D11DepthStencilView) -> (),
        fn om_set_render_targets_and_unordered_access_views(
            num_rtvs: UINT,
            render_target_views: *const *const ID3D11RenderTargetView,
            depth_stencil_view: *const ID3D11DepthStencilView,
            uav_start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *const *const ID3D11UnorderedAccessView,
            uav_initial_counts: *const UINT) -> (),
        fn om_set_blend_state(
            blend_state: *const ID3D11BlendState,
            blend_factor: &[FLOAT; 4],
            sample_mask: UINT) -> (),
        fn om_set_depth_stencil_state(
            depth_stencil_state: *const ID3D11DepthStencilState,
            stencil_ref: UINT) -> (),
        fn so_set_targets(
            num_buffers: UINT,
            so_targets: *const *const ID3D11Buffer,
            offsets: *const UINT) -> (),
        fn draw_auto() -> (),
        fn draw_indexed_instanced_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> (),
        fn draw_instanced_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> (),
        fn dispatch(
            thread_group_count_x: UINT,
            thread_group_count_y: UINT,
            thread_group_count_z: UINT) -> (),
        fn dispatch_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> (),
        fn rs_set_state(rasterizer_state: *const ID3D11RasterizerState) -> (),
        fn rs_set_viewports(
            num_viewports: UINT,
            viewports: *const Viewport) -> (),
        fn rs_set_scissor_rects(
            num_rects: UINT,
            rects: *const Rect) -> (),
        fn copy_subresource_region(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_x: UINT,
            dst_y: UINT,
            dst_z: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            src_box: *const BoxRegion) -> (),
        fn copy_resource(
            dst_resource: *const ID3D11Resource,
            src_resource: *const ID3D11Resource) -> (),
        fn update_subresource(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_box: *const BoxRegion,
            src_data: *const c_void,
            src_row_pitch: UINT,
            src_depth_pitch: UINT) -> (),
        fn copy_structure_count(
            dst_buffer: *const ID3D11Buffer,
            dst_aligned_byte_offset: UINT,
            src_view: *const ID3D11UnorderedAccessView) -> (),
        fn clear_render_target_view(
            render_target_view: *const ID3D11RenderTargetView,
            color_rgba: &[FLOAT; 4]) -> (),
        fn clear_unordered_access_view_uint(
            unordered_access_view: *const ID3D11UnorderedAccessView,
            values: &[UINT; 4]) -> (),
        fn clear_unordered_access_view_float(
            unordered_access_view: *const ID3D11UnorderedAccessView,
            values: &[FLOAT; 4]) -> (),
        fn clear_depth_stencil_view(
            depth_stencil_view: *const ID3D11DepthStencilView,
            clear_flags: ClearFlag,
            depth: FLOAT,
            stencil: UINT8) -> (),
        fn generate_mips(
            shader_resource_view: *const ID3D11ShaderResourceView) -> (),
        fn set_resource_min_lod(
            resource: *const ID3D11Resource,
            min_lod: FLOAT) -> (),
        fn get_resource_min_lod(resource: *const ID3D11Resource) -> FLOAT,
        fn resolve_subresource(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            format: dxgi::Format) -> (),
        fn execute_command_list(
            command_list: *const ID3D11CommandList,
            restore_context_state: BOOL) -> (),
        fn hs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn hs_set_shader(
            hull_shader: *const ID3D11HullShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn hs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn hs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> (),
        fn ds_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn ds_set_shader(
            domain_shader: *const ID3D11DomainShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn ds_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn ds_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> (),
        fn cs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> (),
        fn cs_set_unordered_access_views(
            start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *const *const ID3D11UnorderedAccessView,
            uav_initial_counts: *const UINT) -> (),
        fn cs_set_shader(
            compute_shader: *const ID3D11ComputeShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> (),
        fn cs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> (),
        fn cs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> (),
        fn vs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn ps_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn ps_get_shader(
            pixel_shader: *mut *mut ID3D11PixelShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn ps_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn vs_get_shader(
            vertex_shader: *mut *mut ID3D11VertexShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn ps_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn ia_get_input_layout(input_layout: *mut *mut ID3D11InputLayout) -> (),
        fn ia_get_vertex_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            vertex_buffers: *mut *mut ID3D11Buffer,
            strides: *mut UINT,
            offsets: *mut UINT) -> (),
        fn ia_get_index_buffer(
            index_buffer: *mut *mut ID3D11Buffer,
            format: *mut dxgi::Format,
            offset: *mut UINT) -> (),
        fn gs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn gs_get_shader(
            geometry_shader: *mut *mut ID3D11GeometryShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn ia_get_primitive_topology(topology: *mut PrimitiveTopology) -> (),
        fn vs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn vs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn get_predication(
            predicate: *mut *mut ID3D11Predicate,
            predicate_value: *mut BOOL) -> (),
        fn gs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn gs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn om_get_render_targets(
            num_views: UINT,
            render_target_views: *mut *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut *mut ID3D11DepthStencilView) -> (),
        fn om_get_render_targets_and_unordered_access_views(
            num_rtvs: UINT,
            render_target_views: *mut *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut *mut ID3D11DepthStencilView,
            uav_start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *mut *mut ID3D11UnorderedAccessView) -> (),
        fn om_get_blend_state(
            blend_state: *mut *mut ID3D11BlendState,
            blend_factor: *mut [FLOAT; 4],
            sample_mask: *mut UINT) -> (),
        fn om_get_depth_stencil_state(
            depth_stencil_state: *mut *mut ID3D11DepthStencilState,
            stencil_ref: *mut UINT) -> (),
        fn so_get_targets(
            num_buffers: UINT,
            so_targets: *mut *mut ID3D11Buffer) -> (),
        fn rs_get_state(
            rasterizer_state: *mut *mut ID3D11RasterizerState) -> (),
        fn rs_get_viewports(
            num_viewports: *mut UINT,
            viewports: *mut Viewport) -> (),
        fn rs_get_scissor_rects(
            num_rects: *mut UINT,
            rects: *mut Rect) -> (),
        fn hs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn hs_get_shader(
            hull_shader: *mut *mut ID3D11HullShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn hs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn hs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn ds_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn ds_get_shader(
            domain_shader: *mut *mut ID3D11DomainShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn ds_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn ds_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn cs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> (),
        fn cs_get_unordered_access_views(
            start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *mut *mut ID3D11UnorderedAccessView) -> (),
        fn cs_get_shader(
            compute_shader: *mut *mut ID3D11ComputeShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> (),
        fn cs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> (),
        fn cs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> (),
        fn clear_state() -> (),
        fn flush() -> (),
        fn get_type() -> DeviceContextType,
        fn get_context_flags() -> UINT,
        fn finish_command_list(
            restore_deferred_context_state: BOOL,
            command_list: *mut *mut ID3D11CommandList) -> HRESULT
    }
}

com_interface! {
    struct ID3D11Device: IUnknown {
        iid: IID_ID3D11DEVICE {
            0xDB6F6DDB, 0xAC77, 0x4E88,
            0x82, 0x53, 0x81, 0x9D, 0xF9, 0xBB, 0xF1, 0x40
        },
        vtable: ID3D11DeviceVtbl
    }

    trait D3D11Device: Unknown {
        fn create_buffer(
            desc: *const BufferDesc,
            initial_data: *const SubresourceData,
            buffer: *mut *mut ID3D11Buffer) -> HRESULT,
        fn create_texture_1d(
            desc: *const Texture1DDesc,
            initial_data: *const SubresourceData,
            texture_1d: *mut *mut ID3D11Texture1D) -> HRESULT,
        fn create_texture_2d(
            desc: *const Texture2DDesc,
            initial_data: *const SubresourceData,
            texture_2d: *mut *mut ID3D11Texture2D) -> HRESULT,
        fn create_texture_3d(
            desc: *const Texture3DDesc,
            initial_data: *const SubresourceData,
            texture_3d: *mut *mut ID3D11Texture3D) -> HRESULT,
        fn create_shader_resource_view(
            resource: *const ID3D11Resource,
            desc: *const ShaderResourceViewDesc,
            view: *mut *mut ID3D11ShaderResourceView) -> HRESULT,
        fn create_unordered_access_view(
            resource: *const ID3D11Resource,
            desc: *const UnorderedAccessViewDesc,
            view: *mut *mut ID3D11UnorderedAccessView) -> HRESULT,
        fn create_render_target_view(
            resource: *const ID3D11Resource,
            desc: *const RenderTargetViewDesc,
            view: *mut *mut ID3D11RenderTargetView) -> HRESULT,
        fn create_depth_stencil_view(
            resource: *const ID3D11Resource,
            desc: *const DepthStencilViewDesc,
            view: *mut *mut ID3D11DepthStencilView) -> HRESULT,
        fn create_input_layout(
            input_element_descs: *const InputElementDesc,
            num_elements: UINT,
            shader_bytecode_with_input_signature: *const c_void,
            bytecode_length: SIZE_T,
            input_layout: *mut *mut ID3D11InputLayout) -> HRESULT,
        fn create_vertex_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            vertex_shader: *mut *mut ID3D11VertexShader) -> HRESULT,
        fn create_geometry_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
        fn create_geometry_shader_with_stream_output(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            so_declaration: *const SODeclarationEntry,
            num_entries: UINT,
            buffer_strides: *const UINT,
            num_strides: UINT,
            rasterized_stream: UINT,
            class_linkage: *const ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT,
        fn create_pixel_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            pixel_shader: *mut *mut ID3D11PixelShader) -> HRESULT,
        fn create_hull_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            hull_shader: *mut *mut ID3D11HullShader) -> HRESULT,
        fn create_domain_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            domain_shader: *mut *mut ID3D11DomainShader) -> HRESULT,
        fn create_compute_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            compute_shader: *mut *mut ID3D11ComputeShader) -> HRESULT,
        fn create_class_linkage(
            linkage: *mut *mut ID3D11ClassLinkage) -> HRESULT,
        fn create_blend_state(
            blend_state_desc: *const BlendDesc,
            blend_state: *mut *mut ID3D11BlendState) -> HRESULT,
        fn create_depth_stencil_state(
            depth_stencil_desc: *const DepthStencilDesc,
            depth_stencil_state: *mut *mut ID3D11DepthStencilState) -> HRESULT,
        fn create_rasterizer_state(
            rasterizer_desc: *const RasterizerDesc,
            rasterizer_state: *mut *mut ID3D11RasterizerState) -> HRESULT,
        fn create_sampler_state(
            sampler_desc: *const dxgi::SampleDesc,
            sampler_state: *mut *mut ID3D11SamplerState) -> HRESULT,
        fn create_query(
            query_desc: *const QueryDesc,
            query: *mut *mut ID3D11Query) -> HRESULT,
        fn create_predicate(
            predicate_desc: *const QueryDesc,
            predicate: *mut *mut ID3D11Predicate) -> HRESULT,
        fn create_counter(
            counter_desc: *const CounterDesc,
            counter: *mut *mut ID3D11Counter) -> HRESULT,
        fn create_deferred_context(
            context_flags: UINT,
            deferred_context: *mut *mut ID3D11DeviceContext) -> HRESULT,
        fn open_shared_resource(
            resource_handle: HANDLE,
            returned_interface: REFIID,
            resource: *mut *mut c_void) -> HRESULT,
        fn check_format_support(
            format: dxgi::Format,
            format_support: *mut FormatSupport) -> HRESULT,
        fn check_multisample_quality_levels(
            format: dxgi::Format,
            sample_count: UINT,
            num_quality_levels: *mut UINT) -> HRESULT,
        fn check_counter_info(counter_info: *mut CounterInfo) -> (),
        fn check_counter(
            desc: *const CounterDesc,
            counter_type: *mut CounterType,
            active_counters: *mut UINT,
            name: LPSTR,
            name_length: *mut UINT,
            units: LPSTR,
            units_length: *mut UINT,
            description: LPSTR,
            description_length: *mut UINT) -> HRESULT,
        fn check_feature_support(
            feature: Feature,
            feature_support_data: *mut c_void,
            feature_support_data_size: UINT) -> HRESULT,
        fn get_private_data(
            guid: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void) -> HRESULT,
        fn set_private_data(
            guid: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT,
        fn set_private_data_interface(
            guid: REFGUID,
            data: *const IUnknown) -> HRESULT,
        fn get_feature_level() -> FeatureLevel,
        fn get_creation_flags() -> CreateDeviceFlag,
        fn get_device_removed_reason() -> HRESULT,
        fn get_immediate_context(
            immediate_context: *mut *mut ID3D11DeviceContext) -> HRESULT,
        fn set_exception_mode(raise_flags: RaiseFlag) -> HRESULT,
        fn get_exception_mode() -> RaiseFlag
    }
}

#[test]
fn check_d3d11_vtable_sizes() {
    use std::mem::size_of;

    if cfg!(target_arch = "x86_64") {
        assert_eq!(size_of::<ID3D11DeviceChildVtbl>(), 56);
        assert_eq!(size_of::<ID3D11DepthStencilStateVtbl>(), 64);
        assert_eq!(size_of::<ID3D11BlendStateVtbl>(), 64);
        assert_eq!(size_of::<ID3D11RasterizerStateVtbl>(), 64);
        assert_eq!(size_of::<ID3D11ResourceVtbl>(), 80);
        assert_eq!(size_of::<ID3D11BufferVtbl>(), 88);
        assert_eq!(size_of::<ID3D11Texture1DVtbl>(), 88);
        assert_eq!(size_of::<ID3D11Texture2DVtbl>(), 88);
        assert_eq!(size_of::<ID3D11Texture3DVtbl>(), 88);
        assert_eq!(size_of::<ID3D11ViewVtbl>(), 64);
        assert_eq!(size_of::<ID3D11ShaderResourceViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11RenderTargetViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11DepthStencilViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11UnorderedAccessViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11VertexShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11HullShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11DomainShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11GeometryShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11PixelShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11ComputeShaderVtbl>(), 56);
        assert_eq!(size_of::<ID3D11InputLayoutVtbl>(), 56);
        assert_eq!(size_of::<ID3D11SamplerStateVtbl>(), 64);
        assert_eq!(size_of::<ID3D11AsynchronousVtbl>(), 64);
        assert_eq!(size_of::<ID3D11QueryVtbl>(), 72);
        assert_eq!(size_of::<ID3D11PredicateVtbl>(), 72);
        assert_eq!(size_of::<ID3D11CounterVtbl>(), 72);
        assert_eq!(size_of::<ID3D11ClassInstanceVtbl>(), 88);
        assert_eq!(size_of::<ID3D11ClassLinkageVtbl>(), 72);
        assert_eq!(size_of::<ID3D11CommandListVtbl>(), 64);
        assert_eq!(size_of::<ID3D11DeviceContextVtbl>(), 920);
    } else {
        assert_eq!(size_of::<ID3D11DeviceChildVtbl>(), 28);
        assert_eq!(size_of::<ID3D11DepthStencilStateVtbl>(), 32);
        assert_eq!(size_of::<ID3D11BlendStateVtbl>(), 32);
        assert_eq!(size_of::<ID3D11RasterizerStateVtbl>(), 32);
        assert_eq!(size_of::<ID3D11ResourceVtbl>(), 40);
        assert_eq!(size_of::<ID3D11BufferVtbl>(), 44);
        assert_eq!(size_of::<ID3D11Texture1DVtbl>(), 44);
        assert_eq!(size_of::<ID3D11Texture2DVtbl>(), 44);
        assert_eq!(size_of::<ID3D11Texture3DVtbl>(), 44);
        assert_eq!(size_of::<ID3D11ViewVtbl>(), 32);
        assert_eq!(size_of::<ID3D11ShaderResourceViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11RenderTargetViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11DepthStencilViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11UnorderedAccessViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11VertexShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11HullShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11DomainShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11GeometryShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11PixelShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11ComputeShaderVtbl>(), 28);
        assert_eq!(size_of::<ID3D11InputLayoutVtbl>(), 28);
        assert_eq!(size_of::<ID3D11SamplerStateVtbl>(), 32);
        assert_eq!(size_of::<ID3D11AsynchronousVtbl>(), 32);
        assert_eq!(size_of::<ID3D11QueryVtbl>(), 36);
        assert_eq!(size_of::<ID3D11PredicateVtbl>(), 36);
        assert_eq!(size_of::<ID3D11CounterVtbl>(), 36);
        assert_eq!(size_of::<ID3D11ClassInstanceVtbl>(), 44);
        assert_eq!(size_of::<ID3D11ClassLinkageVtbl>(), 36);
        assert_eq!(size_of::<ID3D11CommandListVtbl>(), 32);
        assert_eq!(size_of::<ID3D11DeviceContextVtbl>(), 460);
    }
}
