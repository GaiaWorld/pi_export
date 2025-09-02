use std::ops::{Range, Deref};

use pi_scene_shell::prelude::*;
use crate::export::{Atom};
use serde::{Deserialize, Serialize};

use crate::about_3d::{constants::EngineConstants};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EGeometry {
    Vec(Vec<VBMeta>),
    Quad,
    Cube,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EEVertexAttribute {
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
    pub fn desc(&self) -> VertexBufferDesc {
        VertexBufferDesc::new(self.key.clone(), self.range.clone(), self.attrs(), self.instance)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
impl GeometryMeta {
    pub fn val2_mut(&mut self) -> &mut Option<(String, Option<f64>, Option<f64>, bool)> {
        &mut self.1
    }
    pub fn val2(& self) -> & Option<(String, Option<f64>, Option<f64>, bool)> {
        & self.1
    }
    pub fn val1_mut(&mut self) -> &mut EGeometry {
        &mut self.0
    }
    pub fn val1(& self) -> & EGeometry {
        &self.0
    }
}