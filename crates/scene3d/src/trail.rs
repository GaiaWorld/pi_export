pub use pi_export_base::export::Engine;
use pi_trail_renderer::{OpsTrail, OpsTrailAgeControl};

pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_trail(
    cmds: &mut CommandsExchangeD3,
    scene: f64,
    entity: f64,
    linked: f64,
) {
    let entity = as_entity(entity);
    let id_scene = as_entity(scene);
    let id_linked_transform = as_entity(linked);

    cmds.trail_create.push(OpsTrail::ops(id_scene, id_linked_transform, entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_trail_age(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
    age_ms: f64,
) {
    let entity = as_entity(entity);
    let ms = age_ms as u32;

    cmds.trail_age.push(OpsTrailAgeControl::ops(entity, ms));
}