
use std::ops::Range;

use pi_engine_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;
use pi_mesh_builder::{
    cube::CubeBuilder,
    quad::QuadBuilder,
};

use crate::constants::EngineConstants;
pub use crate::engine::ActionSetScene3D;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct VBMeta{
    pub(crate) key: KeyVertexBuffer,
    pub(crate) range: VertexBufferDescRange,
    pub(crate) attrs: Vec<VertexAttribute>,
    pub(crate) instance: bool,

}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl VBMeta {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(name: String, start: Option<f64>, end: Option<f64>) -> Self {
        let range = if let (Some(start), Some(end)) = (start, end) {
            VertexBufferDescRange::new(start as VertexBufferRangeVType, end as VertexBufferRangeVType)
        } else {
            VertexBufferDescRange::default()
        };

        Self {
            key: KeyVertexBuffer::from(name.as_str()),
            range,
            attrs: vec![],
            instance: false,
        }

        // Self(VertexBufferDesc::vertices(KeyVertexBuffer::from(name.as_str()), range, vec![]))
    }
    // #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    // #[pi_js_export]
    // pub fn create_as_instance(name: Atom, start: Option<f64>, end: Option<f64>) -> Self {
    //     let range = if let (Some(start), Some(end)) = (start, end) {
    //         Some(Range { start: start as u64, end: end as u64 })
    //     } else {
    //         None
    //     };

    //     Self(VertexBufferDesc::vertices(KeyVertexBuffer::from(name.as_str()), range, vec![]))
    // }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_vbmeta_attr(meta: &mut VBMeta, attr: f64, format: f64) {
    meta.attrs.push(VertexAttribute { kind: EngineConstants::vertex_attr(attr), format: EngineConstants::vertex_format(format) });
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct GeometryMeta(pub(crate) Vec<VertexBufferDesc>, pub(crate) Option<IndicesBufferDesc>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl GeometryMeta {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_empty() -> Self {
        Self(vec![], None)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_box() -> Self {
        Self(CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()))
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_quad() -> Self {
        Self(QuadBuilder::attrs_meta(), Some(QuadBuilder::indices_meta()))
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_vertex(geo: &mut GeometryMeta, vb: &VBMeta) {
    geo.0.push( VertexBufferDesc::new(vb.key.clone(), vb.range.clone(), vb.attrs.clone(), vb.instance) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_indice(geo: &mut GeometryMeta, name: String, start: Option<f64>, end: Option<f64>, as_u16: bool) {
    let range = if let (Some(start), Some(end)) = (start, end) {
        Some(Range { start: start as u64, end: end as u64 })
    } else {
        None
    };

    let ib = IndicesBufferDesc {
        format: if as_u16 { wgpu::IndexFormat::Uint16 } else { wgpu::IndexFormat::Uint32 },
        buffer_range: range,
        buffer: KeyVertexBuffer::from(name.as_str()),
    };
    geo.1 = Some(ib);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_vertex_buffer(app: &mut Engine, param: &mut ActionSetScene3D, key: String, data: &[f32], length: f64) {
	pi_base_export::await_last_frame(app);
    let length = length as usize;

    let queue = app.world.get_resource::<PiRenderQueue>().unwrap().clone();
    let queue = queue.0.clone();
    let key = KeyVertexBuffer::from(key.as_str());
    let key_u64 = key.asset_u64();
    
    let mut resource = param.resource.get_mut(&mut app.world);
    if let Some(buffer) = resource.vb_mgr.get(&key_u64) {
        queue.write_buffer(buffer.buffer(), 0, bytemuck::cast_slice(data));
    } else {
        let data = bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec();
        ActionVertexBuffer::create(&mut resource.vb_wait, key, data);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer(app: &mut Engine, param: &mut ActionSetScene3D, key: String, data: &[u16], length: f64) {
	pi_base_export::await_last_frame(app);
    let length = length as usize;

    let queue = app.world.get_resource::<PiRenderQueue>().unwrap().clone();
    let queue = queue.0.clone();
    let key = KeyVertexBuffer::from(key.as_str());
    let key_u64 = key.asset_u64();

    let mut resource = param.resource.get_mut(&mut app.world);
    if let Some(buffer) = resource.vb_mgr.get(&key_u64) {
        queue.write_buffer(buffer.buffer(), 0, bytemuck::cast_slice(data));
    } else {
        let data = bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec();
        ActionVertexBuffer::create_indices(&mut resource.vb_wait, key, data);
    }
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_geo_instance(_geo: &mut GeometryMeta) {
//     // geo.0.push(VertexBufferDesc::instance_world_matrix());
// }
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_geo_instance_color(_geo: &mut GeometryMeta) {
//     // geo.0.push(VertexBufferDesc::instance_color());
// }
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_geo_instance_tilloff(_geo: &mut GeometryMeta) {
//     // geo.0.push(VertexBufferDesc::instance_tilloff());
// }
