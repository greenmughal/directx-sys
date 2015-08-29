use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, FLOAT, HANDLE, HRESULT, INT, LPCSTR, LPSTR, REFGUID,
             SIZE_T, UINT, UINT8};

use dxgi;
use super::enums::*;
use super::structs::*;

#[link(name = "dxguid")]
extern {
    static IID_ID3D11DeviceChild: IID;
    static IID_ID3D11DepthStencilState: IID;
    static IID_ID3D11BlendState: IID;
    static IID_ID3D11RasterizerState: IID;
    static IID_ID3D11Resource: IID;
    static IID_ID3D11Buffer: IID;
    static IID_ID3D11Texture1D: IID;
    static IID_ID3D11Texture2D: IID;
    static IID_ID3D11Texture3D: IID;
    static IID_ID3D11View: IID;
    static IID_ID3D11ShaderResourceView: IID;
    static IID_ID3D11RenderTargetView: IID;
    static IID_ID3D11DepthStencilView: IID;
    static IID_ID3D11UnorderedAccessView: IID;
    static IID_ID3D11VertexShader: IID;
    static IID_ID3D11HullShader: IID;
    static IID_ID3D11DomainShader: IID;
    static IID_ID3D11GeometryShader: IID;
    static IID_ID3D11PixelShader: IID;
    static IID_ID3D11ComputeShader: IID;
    static IID_ID3D11InputLayout: IID;
    static IID_ID3D11SamplerState: IID;
    static IID_ID3D11Asynchronous: IID;
    static IID_ID3D11Query: IID;
    static IID_ID3D11Predicate: IID;
    static IID_ID3D11Counter: IID;
    static IID_ID3D11ClassInstance: IID;
    static IID_ID3D11ClassLinkage: IID;
    static IID_ID3D11CommandList: IID;
    static IID_ID3D11DeviceContext: IID;
    static IID_ID3D11Device: IID;
}

com_interface! {
    interface ID3D11DeviceChild: IUnknown {
        iid: IID_ID3D11DeviceChild,
        vtable: ID3D11DeviceChildVtbl,
        fn get_device(device: *mut *mut ID3D11Device) -> ();
        fn get_private_data(
            guid: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void) -> HRESULT;
        fn set_private_data(
            guid: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT;
        fn set_private_data_interface(
            guid: REFGUID,
            data: *const IUnknown) -> HRESULT;
    }
}

com_interface! {
    interface ID3D11DepthStencilState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DepthStencilState,
        vtable: ID3D11DepthStencilStateVtbl,
        fn get_desc(desc: *mut DepthStencilDesc) -> ();
    }
}
com_interface! {
    interface ID3D11BlendState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11BlendState,
        vtable: ID3D11BlendStateVtbl,
        fn get_desc(desc: *mut BlendDesc) -> ();
    }
}

com_interface! {
    interface ID3D11RasterizerState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11RasterizerState,
        vtable: ID3D11RasterizerStateVtbl,
        fn get_desc(desc: *mut RasterizerDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Resource: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Resource,
        vtable: ID3D11ResourceVtbl,
        fn get_type(resource_dimension: *mut ResourceDimension) -> ();
        fn set_eviction_priority(eviction_priority: UINT) -> ();
        fn get_eviction_priority() -> UINT;
    }
}

com_interface! {
    interface ID3D11Buffer: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Buffer,
        vtable: ID3D11BufferVtbl,
        fn get_desc(desc: *mut BufferDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Texture1D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Texture1D,
        vtable: ID3D11Texture1DVtbl,
        fn get_desc(desc: *mut Texture1DDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Texture2D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Texture2D,
        vtable: ID3D11Texture2DVtbl,
        fn get_desc(desc: *mut Texture2DDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Texture3D: ID3D11Resource, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Texture3D,
        vtable: ID3D11Texture3DVtbl,
        fn get_desc(desc: *mut Texture3DDesc) -> ();
    }
}

com_interface! {
    interface ID3D11View: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11View,
        vtable: ID3D11ViewVtbl,
        fn get_resource(resource: *mut *mut ID3D11Resource) -> ();
    }
}

com_interface! {
    interface ID3D11ShaderResourceView: ID3D11View, ID3D11DeviceChild,
                                        IUnknown {
        iid: IID_ID3D11ShaderResourceView,
        vtable: ID3D11ShaderResourceViewVtbl,
        fn get_desc(desc: *mut ShaderResourceViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11RenderTargetView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11RenderTargetView,
        vtable: ID3D11RenderTargetViewVtbl,
        fn get_desc(desc: *mut RenderTargetViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11DepthStencilView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DepthStencilView,
        vtable: ID3D11DepthStencilViewVtbl,
        fn get_desc(desc: *mut DepthStencilViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11UnorderedAccessView: ID3D11View, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11UnorderedAccessView,
        vtable: ID3D11UnorderedAccessViewVtbl,
        fn get_desc(desc: *mut UnorderedAccessViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11VertexShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VertexShader,
        vtable: ID3D11VertexShaderVtbl,
    }
}

com_interface! {
    interface ID3D11HullShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11HullShader,
        vtable: ID3D11HullShaderVtbl,
    }
}

com_interface! {
    interface ID3D11DomainShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DomainShader,
        vtable: ID3D11DomainShaderVtbl,
    }
}

com_interface! {
    interface ID3D11GeometryShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11GeometryShader,
        vtable: ID3D11GeometryShaderVtbl,
    }
}

com_interface! {
    interface ID3D11PixelShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11PixelShader,
        vtable: ID3D11PixelShaderVtbl,
    }
}

com_interface! {
    interface ID3D11ComputeShader: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11ComputeShader,
        vtable: ID3D11ComputeShaderVtbl,
    }
}

com_interface! {
    interface ID3D11InputLayout: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11InputLayout,
        vtable: ID3D11InputLayoutVtbl,
    }
}

com_interface! {
    interface ID3D11SamplerState: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11SamplerState,
        vtable: ID3D11SamplerStateVtbl,
        fn get_desc(desc: *mut SamplerDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Asynchronous: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Asynchronous,
        vtable: ID3D11AsynchronousVtbl,
        fn get_data_size() -> UINT;
    }
}

com_interface! {
    interface ID3D11Query: ID3D11Asynchronous, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Query,
        vtable: ID3D11QueryVtbl,
        fn get_desc(desc: *mut QueryDesc) -> ();
    }
}

com_interface! {
    interface ID3D11Predicate: ID3D11Query, ID3D11Asynchronous,
                               ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Predicate,
        vtable: ID3D11PredicateVtbl,
    }
}

com_interface! {
    interface ID3D11Counter: ID3D11Asynchronous, ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11Counter,
        vtable: ID3D11CounterVtbl,
        fn get_desc(desc: *mut CounterDesc) -> ();
    }
}

com_interface! {
    interface ID3D11ClassInstance: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11ClassInstance,
        vtable: ID3D11ClassInstanceVtbl,
        fn get_class_linkage(linkage: *mut *mut ID3D11ClassLinkage) -> ();
        fn get_desc(desc: *mut ClassInstanceDesc) -> ();
        fn get_instance_name(
            instance_name: LPSTR,
            buffer_length: *mut SIZE_T) -> ();
        fn get_type_name(
            type_name: LPSTR,
            buffer_length: *mut SIZE_T) -> ();
    }
}

com_interface! {
    interface ID3D11ClassLinkage: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11ClassLinkage,
        vtable: ID3D11ClassLinkageVtbl,
        fn get_class_instance(
            class_instance_name: LPCSTR,
            instance_index: UINT,
            instance: *mut *mut ID3D11ClassInstance) -> HRESULT;
        fn create_class_instance(
            class_type_name: LPCSTR,
            constant_buffer_offset: UINT,
            constant_vector_offset: UINT,
            texture_offset: UINT,
            sampler_offset: UINT,
            instance: *mut *mut ID3D11ClassInstance) -> HRESULT;
    }
}

com_interface! {
    interface ID3D11CommandList: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11CommandList,
        vtable: ID3D11CommandListVtbl,
        fn get_context_flags() -> UINT;
    }
}

com_interface! {
    interface ID3D11DeviceContext: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11DeviceContext,
        vtable: ID3D11DeviceContextVtbl,
        fn vs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> ();
        fn ps_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn ps_set_shader(
            pixel_shader: *const ID3D11PixelShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn ps_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn vs_set_shader(
            vertex_shader: *const ID3D11VertexShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn draw_indexed(
            index_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT) -> ();
        fn draw(vertex_count: UINT, start_vertex_location: UINT) -> ();
        fn map(
            resource: *const ID3D11Resource,
            subresource: UINT,
            map_type: Map,
            map_flags: MapFlag,
            mapped_resource: *mut MappedSubresource) -> HRESULT;
        fn unmap(resource: *const ID3D11Resource, subresource: UINT) -> ();
        fn ps_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer,
            input_layout: *const ID3D11InputLayout) -> ();
        fn ia_set_input_layout(input_layout: *const ID3D11InputLayout) -> ();
        fn ia_set_vertex_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            vertex_buffers: *const *const ID3D11Buffer,
            strides: *const UINT,
            offsets: *const UINT) -> ();
        fn ia_set_index_buffer(
            index_buffer: *const ID3D11Buffer,
            format: dxgi::Format,
            offset: UINT) -> ();
        fn draw_indexed_instanced(
            index_count_per_instance: UINT,
            instance_count: UINT,
            start_index_location: UINT,
            base_vertex_location: INT,
            start_instance_location: UINT) -> ();
        fn draw_instanced(
            vertex_count_per_instance: UINT,
            instance_count: UINT,
            start_vertex_location: UINT,
            start_instance_location: UINT) -> ();
        fn gs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> ();
        fn gs_set_shader(
            geometry_shader: *const ID3D11GeometryShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn ia_set_primitive_topology(topology: PrimitiveTopology) -> ();
        fn vs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn vs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn begin(async: *const ID3D11Asynchronous) -> ();
        fn end(async: *const ID3D11Asynchronous) -> ();
        fn get_data(
            async: *const ID3D11Asynchronous,
            data: *mut c_void,
            data_size: UINT,
            get_data_flags: AsyncGetDataFlag) -> HRESULT;
        fn set_predication(
            predicate: *const ID3D11Predicate,
            predicate_value: BOOL) -> ();
        fn gs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn gs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn om_set_render_targets(
            num_views: UINT,
            render_target_views: *const *const ID3D11RenderTargetView,
            depth_stencil_view: *const ID3D11DepthStencilView) -> ();
        fn om_set_render_targets_and_unordered_access_views(
            num_rtvs: UINT,
            render_target_views: *const *const ID3D11RenderTargetView,
            depth_stencil_view: *const ID3D11DepthStencilView,
            uav_start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *const *const ID3D11UnorderedAccessView,
            uav_initial_counts: *const UINT) -> ();
        fn om_set_blend_state(
            blend_state: *const ID3D11BlendState,
            blend_factor: &[FLOAT; 4],
            sample_mask: UINT) -> ();
        fn om_set_depth_stencil_state(
            depth_stencil_state: *const ID3D11DepthStencilState,
            stencil_ref: UINT) -> ();
        fn so_set_targets(
            num_buffers: UINT,
            so_targets: *const *const ID3D11Buffer,
            offsets: *const UINT) -> ();
        fn draw_auto() -> ();
        fn draw_indexed_instanced_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> ();
        fn draw_instanced_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> ();
        fn dispatch(
            thread_group_count_x: UINT,
            thread_group_count_y: UINT,
            thread_group_count_z: UINT) -> ();
        fn dispatch_indirect(
            buffer_for_args: *const ID3D11Buffer,
            aligned_byte_offset_for_args: UINT) -> ();
        fn rs_set_state(rasterizer_state: *const ID3D11RasterizerState) -> ();
        fn rs_set_viewports(
            num_viewports: UINT,
            viewports: *const Viewport) -> ();
        fn rs_set_scissor_rects(
            num_rects: UINT,
            rects: *const Rect) -> ();
        fn copy_subresource_region(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_x: UINT,
            dst_y: UINT,
            dst_z: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            src_box: *const BoxRegion) -> ();
        fn copy_resource(
            dst_resource: *const ID3D11Resource,
            src_resource: *const ID3D11Resource) -> ();
        fn update_subresource(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            dst_box: *const BoxRegion,
            src_data: *const c_void,
            src_row_pitch: UINT,
            src_depth_pitch: UINT) -> ();
        fn copy_structure_count(
            dst_buffer: *const ID3D11Buffer,
            dst_aligned_byte_offset: UINT,
            src_view: *const ID3D11UnorderedAccessView) -> ();
        fn clear_render_target_view(
            render_target_view: *const ID3D11RenderTargetView,
            color_rgba: &[FLOAT; 4]) -> ();
        fn clear_unordered_access_view_uint(
            unordered_access_view: *const ID3D11UnorderedAccessView,
            values: &[UINT; 4]) -> ();
        fn clear_unordered_access_view_float(
            unordered_access_view: *const ID3D11UnorderedAccessView,
            values: &[FLOAT; 4]) -> ();
        fn clear_depth_stencil_view(
            depth_stencil_view: *const ID3D11DepthStencilView,
            clear_flags: ClearFlag,
            depth: FLOAT,
            stencil: UINT8) -> ();
        fn generate_mips(
            shader_resource_view: *const ID3D11ShaderResourceView) -> ();
        fn set_resource_min_lod(
            resource: *const ID3D11Resource,
            min_lod: FLOAT) -> ();
        fn get_resource_min_lod(resource: *const ID3D11Resource) -> FLOAT;
        fn resolve_subresource(
            dst_resource: *const ID3D11Resource,
            dst_subresource: UINT,
            src_resource: *const ID3D11Resource,
            src_subresource: UINT,
            format: dxgi::Format) -> ();
        fn execute_command_list(
            command_list: *const ID3D11CommandList,
            restore_context_state: BOOL) -> ();
        fn hs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn hs_set_shader(
            hull_shader: *const ID3D11HullShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn hs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn hs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> ();
        fn ds_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn ds_set_shader(
            domain_shader: *const ID3D11DomainShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn ds_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn ds_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> ();
        fn cs_set_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *const *const ID3D11ShaderResourceView)
            -> ();
        fn cs_set_unordered_access_views(
            start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *const *const ID3D11UnorderedAccessView,
            uav_initial_counts: *const UINT) -> ();
        fn cs_set_shader(
            compute_shader: *const ID3D11ComputeShader,
            class_instances: *const *const ID3D11ClassInstance,
            num_class_instances: UINT) -> ();
        fn cs_set_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *const *const ID3D11SamplerState) -> ();
        fn cs_set_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *const *const ID3D11Buffer) -> ();
        fn vs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn ps_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn ps_get_shader(
            pixel_shader: *mut *mut ID3D11PixelShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn ps_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn vs_get_shader(
            vertex_shader: *mut *mut ID3D11VertexShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn ps_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn ia_get_input_layout(input_layout: *mut *mut ID3D11InputLayout) -> ();
        fn ia_get_vertex_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            vertex_buffers: *mut *mut ID3D11Buffer,
            strides: *mut UINT,
            offsets: *mut UINT) -> ();
        fn ia_get_index_buffer(
            index_buffer: *mut *mut ID3D11Buffer,
            format: *mut dxgi::Format,
            offset: *mut UINT) -> ();
        fn gs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn gs_get_shader(
            geometry_shader: *mut *mut ID3D11GeometryShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn ia_get_primitive_topology(topology: *mut PrimitiveTopology) -> ();
        fn vs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn vs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn get_predication(
            predicate: *mut *mut ID3D11Predicate,
            predicate_value: *mut BOOL) -> ();
        fn gs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn gs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn om_get_render_targets(
            num_views: UINT,
            render_target_views: *mut *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut *mut ID3D11DepthStencilView) -> ();
        fn om_get_render_targets_and_unordered_access_views(
            num_rtvs: UINT,
            render_target_views: *mut *mut ID3D11RenderTargetView,
            depth_stencil_view: *mut *mut ID3D11DepthStencilView,
            uav_start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *mut *mut ID3D11UnorderedAccessView) -> ();
        fn om_get_blend_state(
            blend_state: *mut *mut ID3D11BlendState,
            blend_factor: *mut [FLOAT; 4],
            sample_mask: *mut UINT) -> ();
        fn om_get_depth_stencil_state(
            depth_stencil_state: *mut *mut ID3D11DepthStencilState,
            stencil_ref: *mut UINT) -> ();
        fn so_get_targets(
            num_buffers: UINT,
            so_targets: *mut *mut ID3D11Buffer) -> ();
        fn rs_get_state(
            rasterizer_state: *mut *mut ID3D11RasterizerState) -> ();
        fn rs_get_viewports(
            num_viewports: *mut UINT,
            viewports: *mut Viewport) -> ();
        fn rs_get_scissor_rects(
            num_rects: *mut UINT,
            rects: *mut Rect) -> ();
        fn hs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn hs_get_shader(
            hull_shader: *mut *mut ID3D11HullShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn hs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn hs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn ds_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn ds_get_shader(
            domain_shader: *mut *mut ID3D11DomainShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn ds_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn ds_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn cs_get_shader_resources(
            start_slot: UINT,
            num_views: UINT,
            shader_resource_views: *mut *mut ID3D11ShaderResourceView) -> ();
        fn cs_get_unordered_access_views(
            start_slot: UINT,
            num_uavs: UINT,
            unordered_access_views: *mut *mut ID3D11UnorderedAccessView) -> ();
        fn cs_get_shader(
            compute_shader: *mut *mut ID3D11ComputeShader,
            class_instances: *mut *mut ID3D11ClassInstance,
            num_class_instances: *mut UINT) -> ();
        fn cs_get_samplers(
            start_slot: UINT,
            num_samplers: UINT,
            samplers: *mut *mut ID3D11SamplerState) -> ();
        fn cs_get_constant_buffers(
            start_slot: UINT,
            num_buffers: UINT,
            constant_buffers: *mut *mut ID3D11Buffer) -> ();
        fn clear_state() -> ();
        fn flush() -> ();
        fn get_type() -> DeviceContextType;
        fn get_context_flags() -> UINT;
        fn finish_command_list(
            restore_deferred_context_state: BOOL,
            command_list: *mut *mut ID3D11CommandList) -> HRESULT;
    }
}

com_interface! {
    interface ID3D11Device: IUnknown {
        iid: IID_ID3D11Device,
        vtable: ID3D11DeviceVtbl,
        fn create_buffer(
            desc: *const BufferDesc,
            initial_data: *const SubresourceData,
            buffer: *mut *mut ID3D11Buffer) -> HRESULT;
        fn create_texture_1d(
            desc: *const Texture1DDesc,
            initial_data: *const SubresourceData,
            texture_1d: *mut *mut ID3D11Texture1D) -> HRESULT;
        fn create_texture_2d(
            desc: *const Texture2DDesc,
            initial_data: *const SubresourceData,
            texture_2d: *mut *mut ID3D11Texture2D) -> HRESULT;
        fn create_texture_3d(
            desc: *const Texture3DDesc,
            initial_data: *const SubresourceData,
            texture_3d: *mut *mut ID3D11Texture3D) -> HRESULT;
        fn create_shader_resource_view(
            resource: *const ID3D11Resource,
            desc: *const ShaderResourceViewDesc,
            view: *mut *mut ID3D11ShaderResourceView) -> HRESULT;
        fn create_unordered_access_view(
            resource: *const ID3D11Resource,
            desc: *const UnorderedAccessViewDesc,
            view: *mut *mut ID3D11UnorderedAccessView) -> HRESULT;
        fn create_render_target_view(
            resource: *const ID3D11Resource,
            desc: *const RenderTargetViewDesc,
            view: *mut *mut ID3D11RenderTargetView) -> HRESULT;
        fn create_depth_stencil_view(
            resource: *const ID3D11Resource,
            desc: *const DepthStencilViewDesc,
            view: *mut *mut ID3D11DepthStencilView) -> HRESULT;
        fn create_input_layout(
            input_element_descs: *const InputElementDesc,
            num_elements: UINT,
            shader_bytecode_with_input_signature: *const c_void,
            bytecode_length: SIZE_T,
            input_layout: *mut *mut ID3D11InputLayout) -> HRESULT;
        fn create_vertex_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            vertex_shader: *mut *mut ID3D11VertexShader) -> HRESULT;
        fn create_geometry_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT;
        fn create_geometry_shader_with_stream_output(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            so_declaration: *const SODeclarationEntry,
            num_entries: UINT,
            buffer_strides: *const UINT,
            num_strides: UINT,
            rasterized_stream: UINT,
            class_linkage: *const ID3D11ClassLinkage,
            geometry_shader: *mut *mut ID3D11GeometryShader) -> HRESULT;
        fn create_pixel_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            pixel_shader: *mut *mut ID3D11PixelShader) -> HRESULT;
        fn create_hull_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            hull_shader: *mut *mut ID3D11HullShader) -> HRESULT;
        fn create_domain_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            domain_shader: *mut *mut ID3D11DomainShader) -> HRESULT;
        fn create_compute_shader(
            shader_bytecode: *const c_void,
            bytecode_length: SIZE_T,
            class_linkage: *const ID3D11ClassLinkage,
            compute_shader: *mut *mut ID3D11ComputeShader) -> HRESULT;
        fn create_class_linkage(
            linkage: *mut *mut ID3D11ClassLinkage) -> HRESULT;
        fn create_blend_state(
            blend_state_desc: *const BlendDesc,
            blend_state: *mut *mut ID3D11BlendState) -> HRESULT;
        fn create_depth_stencil_state(
            depth_stencil_desc: *const DepthStencilDesc,
            depth_stencil_state: *mut *mut ID3D11DepthStencilState) -> HRESULT;
        fn create_rasterizer_state(
            rasterizer_desc: *const RasterizerDesc,
            rasterizer_state: *mut *mut ID3D11RasterizerState) -> HRESULT;
        fn create_sampler_state(
            sampler_desc: *const dxgi::SampleDesc,
            sampler_state: *mut *mut ID3D11SamplerState) -> HRESULT;
        fn create_query(
            query_desc: *const QueryDesc,
            query: *mut *mut ID3D11Query) -> HRESULT;
        fn create_predicate(
            predicate_desc: *const QueryDesc,
            predicate: *mut *mut ID3D11Predicate) -> HRESULT;
        fn create_counter(
            counter_desc: *const CounterDesc,
            counter: *mut *mut ID3D11Counter) -> HRESULT;
        fn create_deferred_context(
            context_flags: UINT,
            deferred_context: *mut *mut ID3D11DeviceContext) -> HRESULT;
        fn open_shared_resource(
            resource_handle: HANDLE,
            returned_interface: &IID,
            resource: *mut *mut c_void) -> HRESULT;
        fn check_format_support(
            format: dxgi::Format,
            format_support: *mut FormatSupport) -> HRESULT;
        fn check_multisample_quality_levels(
            format: dxgi::Format,
            sample_count: UINT,
            num_quality_levels: *mut UINT) -> HRESULT;
        fn check_counter_info(counter_info: *mut CounterInfo) -> ();
        fn check_counter(
            desc: *const CounterDesc,
            counter_type: *mut CounterType,
            active_counters: *mut UINT,
            name: LPSTR,
            name_length: *mut UINT,
            units: LPSTR,
            units_length: *mut UINT,
            description: LPSTR,
            description_length: *mut UINT) -> HRESULT;
        fn check_feature_support(
            feature: Feature,
            feature_support_data: *mut c_void,
            feature_support_data_size: UINT) -> HRESULT;
        fn get_private_data(
            guid: REFGUID,
            data_size: *mut UINT,
            data: *mut c_void) -> HRESULT;
        fn set_private_data(
            guid: REFGUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT;
        fn set_private_data_interface(
            guid: REFGUID,
            data: *const IUnknown) -> HRESULT;
        fn get_feature_level() -> FeatureLevel;
        fn get_creation_flags() -> CreateDeviceFlag;
        fn get_device_removed_reason() -> HRESULT;
        fn get_immediate_context(
            immediate_context: *mut *mut ID3D11DeviceContext) -> HRESULT;
        fn set_exception_mode(raise_flags: RaiseFlag) -> HRESULT;
        fn get_exception_mode() -> RaiseFlag;
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
        assert_eq!(size_of::<ID3D11DeviceVtbl>(), 344);
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
        assert_eq!(size_of::<ID3D11DeviceVtbl>(), 172);
    }
}
