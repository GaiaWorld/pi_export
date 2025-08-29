
use std::ops::{Range, Deref};

use pi_export_base::export::VertexBufferRefs;
#[cfg(feature = "record")]
use pi_export_base::record::ERecord3D;
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::{Engine, Atom};
use pi_scene_context::prelude::*;
use pi_mesh_builder::{
    cube::CubeBuilder,
    quad::QuadBuilder,
};
use serde::{Deserialize, Serialize};

pub use pi_export_base::about_3d::geometry::*;

use crate::{constants::EngineConstants, record::ERecordCMD};
pub use crate::commands::CommandsExchangeD3;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_vertex(geo: &mut GeometryMeta, vb: &VBMeta) {
    match geo.val1_mut() {
        EGeometry::Vec(vbmetas) => vbmetas.push( vb.clone() ),
        EGeometry::Quad => todo!(),
        EGeometry::Cube => todo!(),
    }
    // geo.0.push( VertexBufferDesc::new(vb.key.clone(), vb.range.clone(), vb.attrs(), vb.instance) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_indice(geo: &mut GeometryMeta, name: String, start: Option<f64>, end: Option<f64>, as_u16: bool) {
    *geo.val2_mut() = Some((name, start, end, as_u16));
    // let range = if let (Some(start), Some(end)) = (start, end) {
    //     Some(Range { start: start as u32, end: end as u32 })
    // } else {
    //     None
    // };

    // let ib = IndicesBufferDesc {
    //     format: if as_u16 { wgpu::IndexFormat::Uint16 } else { wgpu::IndexFormat::Uint32 },
    //     buffer_range: range,
    //     buffer: KeyVertexBuffer::from(name.as_str()),
    // };
    // geo.1 = Some(ib);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_vertex_buffer(
    cmds: &mut CommandsExchangeD3,
    key: String, data: &[f32], length: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let length = length as usize;
    let val = bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec();
    
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateVertexBuffer(key.clone(), val.clone()));


    CommandsExchangeD3::p3d_create_vertex_buffer(cmds, key, val);

    // let key = KeyVertexBuffer::from(key.as_str());
    // cmds.verticesbuffers.push((key, bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer(
    cmds: &mut CommandsExchangeD3,
    key: String, data: &[u16], length: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let length = length as usize;
    let val = bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec();
    
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateIndiceBuffer(key.clone(), val.clone()));

    CommandsExchangeD3::p3d_create_indices_buffer(cmds, key, val);

    // let key = KeyVertexBuffer::from(key.as_str());
    // cmds.indicesbuffers.push((key, bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer_u32(
    cmds: &mut CommandsExchangeD3,
    key: String, data: &[u32], length: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let length = length as usize;
    let val = bytemuck::cast_slice::<u32, u8>(&data[0..length]).to_vec();

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateIndiceBuffer(key.clone(), val.clone()));

    CommandsExchangeD3::p3d_create_indices_buffer(cmds, key, val);

    // let key = KeyVertexBuffer::from(key.as_str());
    // cmds.indicesbuffers.push((key, bytemuck::cast_slice::<u32, u8>(&data[0..length]).to_vec()));
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
