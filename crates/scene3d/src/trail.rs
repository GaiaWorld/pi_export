pub use pi_export_base::export::Engine;
use pi_trail_renderer::{OpsTrail, OpsTrailAgeControl};

pub use crate::{engine::ActionSetScene3D, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_trail(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    entity: f64,
    linked: f64,
) {
    let mut cmds = param.acts.get_mut(&mut app.world);
    let entity = as_entity(entity);
    let id_scene = as_entity(scene);
    let id_linked_transform = as_entity(linked);

    cmds.trail.create.push(OpsTrail::ops(id_scene, id_linked_transform, entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_trail_age(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    entity: f64,
    age_ms: f64,
) {
    let mut cmds = param.acts.get_mut(&mut app.world);
    let entity = as_entity(entity);
    let ms = age_ms as u32;

    cmds.trail.age.push(OpsTrailAgeControl::ops(entity, ms));
}