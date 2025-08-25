
use std::ops::{Range, Deref};

use pi_export_base::export::VertexBufferRefs;
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::{Engine, Atom};
use pi_scene_context::prelude::*;
use pi_mesh_builder::{
    cube::CubeBuilder,
    quad::QuadBuilder,
};
use serde::{Deserialize, Serialize};

use crate::constants::EngineConstants;
pub use crate::commands::CommandsExchangeD3;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum EGeometry {
    Vec(Vec<VBMeta>),
    Quad,
    Cube,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum EEVertexAttribute {
    Buildin(f64, f64),
    Custom(pi_atom::Atom, pi_atom::Atom, f64, Option<pi_atom::Atom>),
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VBMeta{
    pub(crate) key: KeyVertexBuffer,
    pub(crate) range: VertexBufferDescRange,
    pub(crate) attrs: Vec<EEVertexAttribute>,
    pub(crate) instance: bool,
}
impl VBMeta {
    pub(crate) fn attrs(&self) -> Vec<EVertexAttribute> {
        let mut result = vec![];
        self.attrs.iter().for_each(|item| {
            match item {
                EEVertexAttribute::Buildin(attr, format) => result.push(EVertexAttribute::Buildin(EngineConstants::vertex_attr(*attr), EngineConstants::vertex_format(*format))),
                EEVertexAttribute::Custom(key, code, format, foruniform) => {
                    let vtype = EngineConstants::instance_attribute_vtype(*format);
                    result.push(
                        EVertexAttribute::Custom(CustomVertexAttribute::new(key.clone(), code.clone(), vtype, foruniform.clone()))
                    );
                },
            }
        });
        result
    }
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
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_vbmeta_attr(meta: &mut VBMeta, attr: f64, format: f64) {
    // meta.attrs.push(EVertexAttribute::Buildin(EngineConstants::vertex_attr(attr), EngineConstants::vertex_format(format)));
    meta.attrs.push(EEVertexAttribute::Buildin(attr, format));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_vbmeta_custom_attr(meta: &mut VBMeta, key: &Atom, format: f64, code: &Atom, foruniform: &Atom) {
    let vtype = EngineConstants::instance_attribute_vtype(format);
    let foruniform = if foruniform.as_str() != "" {
        Some(foruniform.deref().clone())
    } else { None };
    // meta.attrs.push(EVertexAttribute::Custom(CustomVertexAttribute::new(key.deref().clone(), code.deref().clone(), vtype, foruniform)));
    meta.attrs.push(EEVertexAttribute::Custom(key.deref().clone(), code.deref().clone(), format, foruniform));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Serialize, Deserialize)]
pub struct GeometryMeta(pub(crate) EGeometry, pub(crate) Option<(String, Option<f64>, Option<f64>, bool)>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl GeometryMeta {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_empty() -> Self {
        Self(EGeometry::Vec(vec![]), None)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_box() -> Self {
        Self(EGeometry::Cube, None)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_quad() -> Self {
        Self(EGeometry::Quad, None)
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_vertex(geo: &mut GeometryMeta, vb: &VBMeta) {
    match &mut geo.0 {
        EGeometry::Vec(vbmetas) => vbmetas.push( vb.clone() ),
        EGeometry::Quad => todo!(),
        EGeometry::Cube => todo!(),
    }
    // geo.0.push( VertexBufferDesc::new(vb.key.clone(), vb.range.clone(), vb.attrs(), vb.instance) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_indice(geo: &mut GeometryMeta, name: String, start: Option<f64>, end: Option<f64>, as_u16: bool) {
    geo.1 = Some((name, start, end, as_u16));
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
    let length = length as usize;

    CommandsExchangeD3::p3d_create_vertex_buffer(cmds, key, bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec());

    // let key = KeyVertexBuffer::from(key.as_str());
    // cmds.verticesbuffers.push((key, bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer(
    cmds: &mut CommandsExchangeD3,
    key: String, data: &[u16], length: f64
) {
    let length = length as usize;

    CommandsExchangeD3::p3d_create_indices_buffer(cmds, key, bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec());

    // let key = KeyVertexBuffer::from(key.as_str());
    // cmds.indicesbuffers.push((key, bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer_u32(
    cmds: &mut CommandsExchangeD3,
    key: String, data: &[u32], length: f64
) {
    let length = length as usize;

    CommandsExchangeD3::p3d_create_indices_buffer(cmds, key, bytemuck::cast_slice::<u32, u8>(&data[0..length]).to_vec());

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
