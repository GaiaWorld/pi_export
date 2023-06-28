
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh(app: &mut Engine, param: &mut ActionSetScene3D, source: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let source: Entity = as_entity(source);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, id, String::from("")));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_color(app: &mut Engine, param: &mut ActionSetScene3D, instance: f64, r: f64, g: f64, b: f64, a: f64) {
    let instance: Entity = as_entity(instance);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.color.push(OpsInstanceColor::ops(instance, r as f32, g as f32, b as f32, a as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_tilloff(app: &mut Engine, param: &mut ActionSetScene3D, instance: f64, uscale: f64, vscale: f64, uoffset: f64, voffset: f64) {
    let instance: Entity = as_entity(instance);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.tilloff.push(OpsInstanceTillOff::ops(instance, uscale as f32, vscale as f32, uoffset as f32, voffset as f32));
}
