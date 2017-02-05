use std::os::raw::c_void;

use com_rs::{IID, IUnknown};
use winapi::{BOOL, FLOAT, HANDLE, GUID, HRESULT, INT, LPCSTR, LPSTR, RECT,
             REFGUID, SIZE, SIZE_T, UINT, UINT8};

use dxgi;
use super::enums::*;
use super::structs::*;

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
    static IID_ID3D11VideoDecoder: IID;
    static IID_ID3D11VideoProcessorEnumerator: IID;
    static IID_ID3D11VideoProcessor: IID;
    static IID_ID3D11AuthenticatedChannel: IID;
    static IID_ID3D11CryptoSession: IID;
    static IID_ID3D11VideoDecoderOutputView: IID;
    static IID_ID3D11VideoProcessorInputView: IID;
    static IID_ID3D11VideoProcessorOutputView: IID;
    static IID_ID3D11VideoContext: IID;
    static IID_ID3D11VideoDevice: IID;
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

guid!(DECODER_PROFILE_MPEG2_MOCOMP =
    0xe6a9f44b, 0x61b0, 0x4563, 0x9e, 0xa4, 0x63, 0xd2, 0xa3, 0xc6, 0xfe, 0x66);
guid!(DECODER_PROFILE_MPEG2_IDCT =
    0xbf22ad00, 0x03ea, 0x4690, 0x80, 0x77, 0x47, 0x33, 0x46, 0x20, 0x9b, 0x7e);
guid!(DECODER_PROFILE_MPEG2_VLD =
    0xee27417f, 0x5e28, 0x4e65, 0xbe, 0xea, 0x1d, 0x26, 0xb5, 0x08, 0xad, 0xc9);
guid!(DECODER_PROFILE_MPEG1_VLD =
    0x6f3ec719, 0x3735, 0x42cc, 0x80, 0x63, 0x65, 0xcc, 0x3c, 0xb3, 0x66, 0x16);
guid!(DECODER_PROFILE_MPEG2AND1_VLD =
    0x86695f12, 0x340e, 0x4f04, 0x9f, 0xd3, 0x92, 0x53, 0xdd, 0x32, 0x74, 0x60);
guid!(DECODER_PROFILE_H264_MOCOMP_NOFGT =
    0x1b81be64, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_MOCOMP_FGT =
    0x1b81be65, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_IDCT_NOFGT =
    0x1b81be66, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_IDCT_FGT =
    0x1b81be67, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_VLD_NOFGT =
    0x1b81be68, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_VLD_FGT =
    0x1b81be69, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT =
    0xd5f04ff9, 0x3418, 0x45d8, 0x95, 0x61, 0x32, 0xa7, 0x6a, 0xae, 0x2d, 0xdd);
guid!(DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT =
    0xd79be8da, 0x0cf1, 0x4c81, 0xb8, 0x2a, 0x69, 0xa4, 0xe2, 0x36, 0xf4, 0x3d);
guid!(DECODER_PROFILE_H264_VLD_STEREO_NOFGT =
    0xf9aaccbb, 0xc2b6, 0x4cfc, 0x87, 0x79, 0x57, 0x07, 0xb1, 0x76, 0x05, 0x52);
guid!(DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT =
    0x705b9d82, 0x76cf, 0x49d6, 0xb7, 0xe6, 0xac, 0x88, 0x72, 0xdb, 0x01, 0x3c);
guid!(DECODER_PROFILE_WMV8_POSTPROC =
    0x1b81be80, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_WMV8_MOCOMP =
    0x1b81be81, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_WMV9_POSTPROC =
    0x1b81be90, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_WMV9_MOCOMP =
    0x1b81be91, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_WMV9_IDCT =
    0x1b81be94, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_VC1_POSTPROC =
    0x1b81beA0, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_VC1_MOCOMP =
    0x1b81beA1, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_VC1_IDCT =
    0x1b81beA2, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_VC1_VLD =
    0x1b81beA3, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_VC1_D2010 =
    0x1b81beA4, 0xa0c7, 0x11d3, 0xb9, 0x84, 0x00, 0xc0, 0x4f, 0x2e, 0x73, 0xc5);
guid!(DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE =
    0xefd64d74, 0xc9e8, 0x41d7, 0xa5, 0xe9, 0xe9, 0xb0, 0xe3, 0x9f, 0xa3, 0x19);
guid!(DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC =
    0xed418a9f, 0x010d, 0x4eda, 0x9a, 0xe3, 0x9a, 0x65, 0x35, 0x8d, 0x8d, 0x2e);
guid!(DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC =
    0xab998b5b, 0x4258, 0x44a9, 0x9f, 0xeb, 0x94, 0xe5, 0x97, 0xa6, 0xba, 0xae);
guid!(DECODER_PROFILE_HEVC_VLD_MAIN =
    0x5b11d51b, 0x2f4c, 0x4452, 0xbc, 0xc3, 0x09, 0xf2, 0xa1, 0x16, 0x0c, 0xc0);
guid!(DECODER_PROFILE_HEVC_VLD_MAIN10 =
    0x107af0e0, 0xef1a, 0x4d19, 0xab, 0xa8, 0x67, 0xa1, 0x63, 0x07, 0x3d, 0x13);

com_interface! {
    interface ID3D11VideoDecoder: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VideoDecoder,
        vtable: ID3D11VideoDecoderVtbl,
        fn get_creation_parameters(
            video_desc: *mut VideoDecoderDesc,
            config: *mut VideoDecoderConfig) -> HRESULT;
        fn get_driver_handle(driver_handle: *mut HANDLE) -> HRESULT;
    }
}

guid!(CRYPTO_TYPE_AES128_CTR =
    0x9b6bd711, 0x4f74, 0x41c9, 0x9e, 0x7b, 0x0b, 0xe2, 0xd7, 0xd9, 0x3b, 0x4f);
guid!(DECODER_ENCRYPTION_HW_CENC =
    0x89d6ac4f, 0x09f2, 0x4229, 0xb2, 0xcd, 0x37, 0x74, 0x0a, 0x6d, 0xfd, 0x81);
guid!(KEY_EXCHANGE_HW_PROTECTION =
    0xb1170d8a, 0x628d, 0x4da3, 0xad, 0x3b, 0x82, 0xdd, 0xb0, 0x8b, 0x49, 0x70);

com_interface! {
    interface ID3D11VideoProcessorEnumerator: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VideoProcessorEnumerator,
        vtable: ID3D11VideoProcessorEnumeratorVtbl,
        fn get_video_processor_content_desc(
            content_desc: *mut VideoProcessorContentDesc) -> HRESULT;
        fn check_video_processor_format(
            format: dxgi::Format,
            flags: *mut UINT) -> HRESULT;
        fn get_video_processor_caps(
            caps: *mut VideoProcessorCaps) -> HRESULT;
        fn get_video_processor_rate_conversion_caps(
            type_index: UINT,
            caps: *mut VideoProcessorRateConversionCaps) -> HRESULT;
        fn get_video_processor_custom_rate(
            type_index: UINT,
            custom_rate_index: UINT,
            rate: *mut VideoProcessorCustomRate) -> HRESULT;
        fn get_video_processor_filter_range(
            filter: VideoProcessorFilter,
            range: *mut VideoProcessorFilterRange) -> HRESULT;
    }
}

com_interface! {
    interface ID3D11VideoProcessor: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VideoProcessor,
        vtable: ID3D11VideoProcessorVtbl,
        fn get_content_desc(desc: *mut VideoProcessorContentDesc) -> ();
        fn get_rate_conversion_caps(
            caps: *mut VideoProcessorRateConversionCaps) -> ();
    }
}

com_interface! {
    interface ID3D11AuthenticatedChannel: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11AuthenticatedChannel,
        vtable: ID3D11AuthenticatedChannelVtbl,
        fn get_certificate_size(certificate_size: *mut UINT) -> HRESULT;
        fn get_certificate(
            certificate_size: UINT,
            certificate: *mut u8) -> HRESULT;
        fn get_channel_handle(channel_handle: *mut HANDLE) -> ();
    }
}

guid!(AUTHENTICATED_QUERY_PROTECTION =
    0xa84eb584, 0xc495, 0x48aa, 0xb9, 0x4d, 0x8b, 0xd2, 0xd6, 0xfb, 0xce, 0x05);
guid!(AUTHENTICATED_QUERY_CHANNEL_TYPE =
    0xbc1b18a5, 0xb1fb, 0x42ab, 0xbd, 0x94, 0xb5, 0x82, 0x8b, 0x4b, 0xf7, 0xbe);
guid!(AUTHENTICATED_QUERY_DEVICE_HANDLE =
    0xec1c539d, 0x8cff, 0x4e2a, 0xbc, 0xc4, 0xf5, 0x69, 0x2f, 0x99, 0xf4, 0x80);
guid!(AUTHENTICATED_QUERY_CRYPTO_SESSION =
    0x2634499e, 0xd018, 0x4d74, 0xac, 0x17, 0x7f, 0x72, 0x40, 0x59, 0x52, 0x8d);
guid!(AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT =
    0x0db207b3, 0x9450, 0x46a6, 0x82, 0xde, 0x1b, 0x96, 0xd4, 0x4f, 0x9c, 0xf2);
guid!(AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS =
    0x649bbadb, 0xf0f4, 0x4639, 0xa1, 0x5b, 0x24, 0x39, 0x3f, 0xc3, 0xab, 0xac);
guid!(AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT =
    0x012f0bd6, 0xe662, 0x4474, 0xbe, 0xfd, 0xaa, 0x53, 0xe5, 0x14, 0x3c, 0x6d);
guid!(AUTHENTICATED_QUERY_OUTPUT_ID_COUNT =
    0x2c042b5e, 0x8c07, 0x46d5, 0xaa, 0xbe, 0x8f, 0x75, 0xcb, 0xad, 0x4c, 0x31);
guid!(AUTHENTICATED_QUERY_OUTPUT_ID =
    0x839ddca3, 0x9b4e, 0x41e4, 0xb0, 0x53, 0x89, 0x2b, 0xd2, 0xa1, 0x1e, 0xe7);
guid!(AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES =
    0x6214d9d2, 0x432c, 0x4abb, 0x9f, 0xce, 0x21, 0x6e, 0xea, 0x26, 0x9e, 0x3b);
guid!(AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT =
    0xb30f7066, 0x203c, 0x4b07, 0x93, 0xfc, 0xce, 0xaa, 0xfd, 0x61, 0x24, 0x1e);
guid!(AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID =
    0xf83a5958, 0xe986, 0x4bda, 0xbe, 0xb0, 0x41, 0x1f, 0x6a, 0x7a, 0x01, 0xb7);
guid!(AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE =
    0xec1791c7, 0xdad3, 0x4f15, 0x9e, 0xc3, 0xfa, 0xa9, 0x3d, 0x60, 0xd4, 0xf0);
guid!(AUTHENTICATED_CONFIGURE_INITIALIZE =
    0x06114bdb, 0x3523, 0x470a, 0x8d, 0xca, 0xfb, 0xc2, 0x84, 0x51, 0x54, 0xf0);
guid!(AUTHENTICATED_CONFIGURE_PROTECTION =
    0x50455658, 0x3f47, 0x4362, 0xbf, 0x99, 0xbf, 0xdf, 0xcd, 0xe9, 0xed, 0x29);
guid!(AUTHENTICATED_CONFIGURE_CRYPTO_SESSION =
    0x6346cc54, 0x2cfc, 0x4ad4, 0x82, 0x24, 0xd1, 0x58, 0x37, 0xde, 0x77, 0x00);
guid!(AUTHENTICATED_CONFIGURE_SHARED_RESOURCE =
    0x0772d047, 0x1b40, 0x48e8, 0x9c, 0xa6, 0xb5, 0xf5, 0x10, 0xde, 0x9f, 0x01);
guid!(AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE =
    0x41fff286, 0x6ae0, 0x4d43, 0x9d, 0x55, 0xa4, 0x6e, 0x9e, 0xfd, 0x15, 0x8a);
guid!(KEY_EXCHANGE_RSAES_OAEP =
    0xc1949895, 0xd72a, 0x4a1d, 0x8e, 0x5d, 0xed, 0x85, 0x7d, 0x17, 0x15, 0x20);


com_interface! {
    interface ID3D11CryptoSession: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11CryptoSession,
        vtable: ID3D11CryptoSessionVtbl,
        fn get_crypto_type(crypto_type: *mut GUID) -> ();
        fn get_decoder_profile(decoder_profile: *mut GUID) -> ();
        fn get_certificate_size(certificate_size: *mut UINT) -> HRESULT;
        fn get_certificate(
            certificate_size: UINT,
            certificate: *mut u8) -> HRESULT;
        fn get_crypto_session_handle(crypto_session_handle: *mut HANDLE) -> ();
    }
}

com_interface! {
    interface ID3D11VideoDecoderOutputView: ID3D11View, ID3D11DeviceChild,
                                            IUnknown {
        iid: IID_ID3D11VideoDecoderOutputView,
        vtable: ID3D11VideoDecoderOutputViewVtbl,
        fn get_desc(desc: *mut VideoDecoderOutputViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11VideoProcessorInputView: ID3D11View, ID3D11DeviceChild,
                                             IUnknown {
        iid: IID_ID3D11VideoProcessorInputView,
        vtable: ID3D11VideoProcessorInputViewVtbl,
        fn get_desc(desc: *mut VideoProcessorInputViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11VideoProcessorOutputView: ID3D11View, ID3D11DeviceChild,
                                              IUnknown {
        iid: IID_ID3D11VideoProcessorOutputView,
        vtable: ID3D11VideoProcessorOutputViewVtbl,
        fn get_desc(desc: *mut VideoProcessorOutputViewDesc) -> ();
    }
}

com_interface! {
    interface ID3D11VideoContext: ID3D11DeviceChild, IUnknown {
        iid: IID_ID3D11VideoContext,
        vtable: ID3D11VideoContextVtbl,
        fn get_decoder_buffer(
            decoder: *const ID3D11VideoDecoder,
            decoder_type: VideoDecoderBufferType,
            buffer_size: *mut UINT,
            buffer: *mut *mut c_void) -> HRESULT;
        fn release_decoder_buffer(
            decoder: *const ID3D11VideoDecoder,
            decoder_type: VideoDecoderBufferType) -> HRESULT;
        fn decoder_begin_frame(
            decoder: *const ID3D11VideoDecoder,
            view: *const ID3D11VideoDecoderOutputView,
            content_key_size: UINT,
            content_key: *const c_void) -> HRESULT;
        fn decoder_end_frame(decoder: *const ID3D11VideoDecoder) -> HRESULT;
        fn submit_decoder_buffers(
            decoder: *const ID3D11VideoDecoder,
            num_buffers: UINT,
            buffer_desc: *const VideoDecoderBufferDesc) -> HRESULT;
        fn decoder_extension(
            decoder: *const ID3D11VideoDecoder,
            extension_data: *const VideoDecoderExtension) -> HRESULT;
        fn video_processor_set_output_target_rect(
            video_processor: *const ID3D11VideoProcessor,
            enable: BOOL,
            rect: *const RECT) -> ();
        fn video_processor_set_output_background_color(
            video_processor: *const ID3D11VideoProcessor,
            ycbcr: BOOL,
            color: *const VideoColor) -> ();
        fn video_processor_set_output_color_space(
            video_processor: *const ID3D11VideoProcessor,
            color_space: *const VideoProcessorColorSpace) -> ();
        fn video_processor_set_output_alpha_fill_mode(
            video_processor: *const ID3D11VideoProcessor,
            alpha_fill_mode: VideoProcessorAlphaFillMode,
            stream_index: UINT) -> ();
        fn video_processor_set_output_constriction(
            video_processor: *const ID3D11VideoProcessor,
            enable: BOOL,
            size: SIZE) -> ();
        fn video_processor_set_output_stereo_mode(
            video_processor: *const ID3D11VideoProcessor,
            enable: BOOL) -> ();
        fn video_processor_set_output_extension(
            video_processor: *const ID3D11VideoProcessor,
            extension_guid: *const GUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT;
        fn video_processor_get_output_target_rect(
            video_processor: *const ID3D11VideoProcessor,
            enabled: *mut BOOL,
            rect: *mut RECT) -> ();
        fn video_processor_get_output_background_color(
            video_processor: *const ID3D11VideoProcessor,
            ycbcr: *mut BOOL,
            color: *mut VideoColor) -> ();
        fn video_processor_get_output_color_space(
            video_processor: *const ID3D11VideoProcessor,
            color_space: *mut VideoProcessorColorSpace) -> ();
        fn video_processor_get_output_alpha_fill_mode(
            video_processor: *const ID3D11VideoProcessor,
            alpha_fill_mode: *mut VideoProcessorAlphaFillMode,
            stream_index: *mut UINT) -> ();
        fn video_processor_get_output_constriction(
            video_processor: *const ID3D11VideoProcessor,
            enabled: *mut BOOL,
            size: *mut SIZE) -> ();
        fn video_processor_get_output_stereo_mode(
            video_processor: *const ID3D11VideoProcessor,
            enabled: *mut BOOL) -> ();
        fn video_processor_get_output_extension(
            video_processor: *const ID3D11VideoProcessor,
            extension_guid: *const GUID,
            data_size: UINT,
            data: *mut c_void) -> HRESULT;
        fn video_processor_set_stream_frame_format(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            frame_format: VideoFrameFormat) -> ();
        fn video_processor_set_stream_color_space(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            color_space: *const VideoProcessorColorSpace) -> ();
        fn video_processor_set_stream_output_rate(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            output_rate: VideoProcessorOutputRate,
            repeat_frame: BOOL,
            custom_rate: *const dxgi::Rational) -> ();
        fn video_processor_set_stream_source_rect(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            rect: *const RECT) -> ();
        fn video_processor_set_stream_dest_rect(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            rect: *const RECT) -> ();
        fn video_processor_set_stream_alpha(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            alpha: FLOAT) -> ();
        fn video_processor_set_stream_palette(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            count: UINT,
            entries: *const UINT) -> ();
        fn video_processor_set_stream_pixel_aspect_ratio(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            source_aspect_ratio: *const dxgi::Rational,
            destination_aspect_ratio: *const dxgi::Rational) -> ();
        fn video_processor_set_stream_luma_key(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            lower: FLOAT,
            upper: FLOAT) -> ();
        fn video_processor_set_stream_stereo_format(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            format: VideoProcessorStereoFormat,
            left_view_frame0: BOOL,
            base_view_frame0: BOOL,
            flip_mode: VideoProcessorStereoFlipMode,
            mono_offset: i32) -> ();
        fn video_processor_set_stream_auto_processing_mode(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL) -> ();
        fn video_processor_set_stream_filter(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            filter: VideoProcessorFilter,
            enable: BOOL,
            level: i32) -> ();
        fn video_processor_set_stream_extension(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            extension_guid: *const GUID,
            data_size: UINT,
            data: *const c_void) -> HRESULT;
        fn video_processor_get_stream_frame_format(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            frame_format: *mut VideoFrameFormat) -> ();
        fn video_processor_get_stream_color_space(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            color_space: *mut VideoProcessorColorSpace) -> ();
        fn video_processor_get_stream_output_rate(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            output_rate: *mut VideoProcessorOutputRate,
            repeat_frame: *mut BOOL,
            custom_rate: *mut dxgi::Rational) -> ();
        fn video_processor_get_stream_source_rect(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL,
            rect: *mut RECT) -> ();
        fn video_processor_get_stream_dest_rect(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL,
            rect: *mut RECT) -> ();
        fn video_processor_get_stream_alpha(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL,
            alpha: *mut FLOAT) -> ();
        fn video_processor_get_stream_palette(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            count: UINT,
            entries: *mut UINT) -> ();
        fn video_processor_get_stream_pixel_aspect_ratio(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL,
            source_aspect_ratio: *mut dxgi::Rational,
            destination_aspect_ratio: *mut dxgi::Rational) -> ();
        fn video_processor_get_stream_luma_key(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL,
            lower: *mut FLOAT,
            upper: *mut FLOAT) -> ();
        fn video_processor_get_stream_stereo_format(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: *mut BOOL,
            format: *mut VideoProcessorStereoFormat,
            left_view_frame0: *mut BOOL,
            base_view_frame0: *mut BOOL,
            flip_mode: *mut VideoProcessorStereoFlipMode,
            mono_offset: *mut i32) -> ();
        fn video_processor_get_stream_auto_processing_mode(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enabled: *mut BOOL) -> ();
        fn video_processor_get_stream_filter(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            filter: VideoProcessorFilter,
            enabled: *mut BOOL,
            level: *mut i32) -> ();
        fn video_processor_get_stream_extension(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            extension_guid: *const GUID,
            data_size: UINT,
            data: *mut c_void) -> HRESULT;
        fn video_processor_blt(
            video_processor: *const ID3D11VideoProcessor,
            view: *const ID3D11VideoProcessorOutputView,
            output_frame: UINT,
            stream_count: UINT,
            streams: *const VideoProcessorStream) -> HRESULT;
        fn negotiate_crypto_session_key_exchange(
            crypto_session: *const ID3D11CryptoSession,
            data_size: UINT,
            data: *mut c_void) -> HRESULT;
        fn encryption_blt(
            crypto_session: *const ID3D11CryptoSession,
            src_surface: *const ID3D11Texture2D,
            dst_surface: *const ID3D11Texture2D,
            iv_size: UINT,
            iv: *mut c_void) -> ();
        fn decryption_blt(
            crypto_session: *const ID3D11CryptoSession,
            src_surface: *const ID3D11Texture2D,
            dst_surface: *const ID3D11Texture2D,
            encrypted_block_info: *const EncryptedBlockInfo,
            content_key_size: UINT,
            content_key: *const c_void,
            iv_size: UINT,
            iv: *mut c_void) -> ();
        fn start_session_key_refresh(
            crypto_session: *const ID3D11CryptoSession,
            random_number_size: UINT,
            random_number: *mut c_void) -> ();
        fn finish_session_key_refresh(
            crypto_session: *const ID3D11CryptoSession) -> ();
        fn get_encryption_blt_key(
            crypto_session: *const ID3D11CryptoSession,
            key_size: UINT,
            readback_key: *mut c_void) -> HRESULT;
        fn negotiate_authenticated_channel_key_exchange(
            channel: *const ID3D11AuthenticatedChannel,
            data_size: UINT,
            data: *mut c_void) -> HRESULT;
        fn query_authenticated_channel(
            channel: *const ID3D11AuthenticatedChannel,
            input_size: UINT,
            input: *const c_void,
            output_size: UINT,
            output: *mut c_void) -> HRESULT;
        fn configure_authenticated_channel(
            channel: *const ID3D11AuthenticatedChannel,
            input_size: UINT,
            input: *const c_void,
            output: *mut AuthenticatedConfigureOutput) -> HRESULT;
        fn video_processor_set_stream_rotation(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: BOOL,
            rotation: VideoProcessorRotation) -> ();
        fn video_processor_get_stream_rotation(
            video_processor: *const ID3D11VideoProcessor,
            stream_index: UINT,
            enable: *mut BOOL,
            rotation: *mut VideoProcessorRotation) -> ();
    }
}

com_interface! {
    interface ID3D11VideoDevice: IUnknown {
        iid: IID_ID3D11VideoDevice,
        vtable: ID3D11VideoDeviceVtbl,
        fn create_video_decoder(
            video_desc: *const VideoDecoderDesc,
            config: *const VideoDecoderConfig,
            decoder: *mut *mut ID3D11VideoDecoder) -> HRESULT;
        fn create_video_processor(
            enumerator: *const ID3D11VideoProcessorEnumerator,
            rate_conversion_index: UINT,
            video_processor: *mut *mut ID3D11VideoProcessor) -> HRESULT;
        fn create_authenticated_channel(
            channel_type: AuthenticatedChannelType,
            authenticated_channel: *mut *mut ID3D11AuthenticatedChannel)
            -> HRESULT;
        fn create_crypto_session(
            crypto_type: *const GUID,
            decoder_profile: *const GUID,
            key_exchange_type: *const GUID,
            crypto_session: *mut *mut ID3D11CryptoSession) -> HRESULT;
        fn create_video_decoder_output_view(
            resource: *const ID3D11Resource,
            desc: *const VideoDecoderOutputViewDesc,
            vdo_view: *mut *mut ID3D11VideoDecoderOutputView) -> HRESULT;
        fn create_video_processor_input_view(
            resource: *const ID3D11Resource,
            enumerator: *const ID3D11VideoProcessorEnumerator,
            desc: *const VideoProcessorInputViewDesc,
            vpi_view: *mut *mut ID3D11VideoProcessorInputView) -> HRESULT;
        fn create_video_processor_output_view(
            resource: *const ID3D11Resource,
            enumerator: *const ID3D11VideoProcessorEnumerator,
            desc: *const VideoProcessorOutputViewDesc,
            vpo_view: *mut *mut ID3D11VideoProcessorOutputView) -> HRESULT;
        fn create_video_processor_enumerator(
            desc: *const VideoProcessorContentDesc,
            enumerator: *mut *mut ID3D11VideoProcessorEnumerator) -> HRESULT;
        fn get_video_decoder_profile_count() -> UINT;
        fn get_video_decoder_profile(
            index: UINT,
            decoder_profile: *mut GUID) -> HRESULT;
        fn check_video_decoder_format(
            decoder_profile: *const GUID,
            format: dxgi::Format,
            supported: *mut BOOL) -> HRESULT;
        fn get_video_decoder_config_count(
            desc: *const VideoDecoderDesc,
            count: *mut UINT) -> HRESULT;
        fn get_video_decoder_config(
            desc: *const VideoDecoderDesc,
            index: UINT,
            config: *mut VideoDecoderConfig) -> HRESULT;
        fn get_content_protection_caps(
            crypto_type: *const GUID,
            decoder_profile: *const GUID,
            caps: *mut VideoContentProtectionCaps) -> HRESULT;
        fn check_crypto_key_exchange(
            crypto_type: *const GUID,
            decoder_profile: *const GUID,
            index: UINT,
            key_exchange_type: *mut GUID) -> HRESULT;
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
        assert_eq!(size_of::<ID3D11VideoDecoderVtbl>(), 72);
        assert_eq!(size_of::<ID3D11VideoProcessorEnumeratorVtbl>(), 104);
        assert_eq!(size_of::<ID3D11VideoProcessorVtbl>(), 72);
        assert_eq!(size_of::<ID3D11AuthenticatedChannelVtbl>(), 80);
        assert_eq!(size_of::<ID3D11CryptoSessionVtbl>(), 96);
        assert_eq!(size_of::<ID3D11VideoDecoderOutputViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11VideoProcessorInputViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11VideoProcessorOutputViewVtbl>(), 72);
        assert_eq!(size_of::<ID3D11VideoContextVtbl>(), 520);
        assert_eq!(size_of::<ID3D11VideoDeviceVtbl>(), 160);
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
        assert_eq!(size_of::<ID3D11VideoDecoderVtbl>(), 36);
        assert_eq!(size_of::<ID3D11VideoProcessorEnumeratorVtbl>(), 52);
        assert_eq!(size_of::<ID3D11VideoProcessorVtbl>(), 36);
        assert_eq!(size_of::<ID3D11AuthenticatedChannelVtbl>(), 40);
        assert_eq!(size_of::<ID3D11CryptoSessionVtbl>(), 48);
        assert_eq!(size_of::<ID3D11VideoDecoderOutputViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11VideoProcessorInputViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11VideoProcessorOutputViewVtbl>(), 36);
        assert_eq!(size_of::<ID3D11VideoContextVtbl>(), 260);
        assert_eq!(size_of::<ID3D11VideoDeviceVtbl>(), 80);
    }
}
