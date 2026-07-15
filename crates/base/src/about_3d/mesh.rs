
use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::{as_entity, as_f64, geometry::GeometryMeta};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VInstanceAttributes(pub(crate) bool, pub(crate) Vec<CustomVertexAttribute>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl VInstanceAttributes {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(instane_matrix: bool) -> Self {
        VInstanceAttributes(instane_matrix, vec![])
    }
}
impl VInstanceAttributes {
    pub fn v0(&self) -> bool {
        self.0
    }
    pub fn v0_mut(&mut self) -> &mut bool {
        &mut self.0
    }
    pub fn v1(&self) -> & Vec<CustomVertexAttribute> {
        &self.1
    }
    pub fn v1_mut(&mut self) -> &mut Vec<CustomVertexAttribute> {
        &mut self.1
    }
}