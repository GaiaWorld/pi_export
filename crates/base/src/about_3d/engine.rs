
use js_proxy_gen_macro::pi_js_export;
use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;
use serde::{Deserialize, Serialize};
use crate::commands::CommandsExchangeD3;
use pi_particle_system::prelude::ParticleSystemCalculatorID;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use super::action::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ImageRes(Handle<pi_render::renderer::texture::ImageTextureFrame>);
impl ImageRes {
    pub fn create(v: Handle<pi_render::renderer::texture::ImageTextureFrame>) -> Self {
        Self(v)
    }
    pub fn val(&self) -> &Handle<pi_render::renderer::texture::ImageTextureFrame> {
        &self.0
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GLTFRes(pub(crate) Entity);
impl GLTFRes {
    pub fn new(v: Entity) -> Self {
        Self(v)
    }
    pub fn val(&self) -> &Entity {
        &self.0
    }
    pub fn val_mut(&mut self) -> &mut Entity {
        &mut self.0
    }
}

pub fn gltf_particle_calculator<'a>(
    cmds: &'a CommandsExchangeD3, item: &'a GLTFRes, index: f64) -> Option<&'a Handle<ParticleSystemCalculatorID>> {
    if let Some(gltf) = cmds.gltfs.get(item.val()) {
        let index = index as usize;
        gltf.particlesys_calculators.get(&index)
    } else {
        None
    }
}
