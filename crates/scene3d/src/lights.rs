
use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector3;

use crate::constants::EngineConstants;
pub use crate::commands::CommandsExchangeD3;
use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, ltype: f64) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();
    let scene: Entity = as_entity(scene);

    cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
    cmds.light_create.push(OpsLightCreate::ops(scene, id, EngineConstants::light(ltype)));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_direction(cmds: &mut CommandsExchangeD3, light: f64, x: f64, y: f64, z: f64) {
    let light: Entity = as_entity(light);
    cmds.light_param.push(OpsLightParam::ops(light, ELightModify::Directional( Vector3::new(x as f32, y as f32, z as f32) )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_strength(cmds: &mut CommandsExchangeD3, light: f64, val: f64) {
    let light: Entity = as_entity(light);
    cmds.light_param.push(OpsLightParam::ops(light, ELightModify::Strength(val as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_spot_angle(cmds: &mut CommandsExchangeD3, light: f64, inner: f64, outer: f64) {
    let light: Entity = as_entity(light);
    cmds.light_param.push(OpsLightParam::ops(light, ELightModify::SpotAngle(inner as f32, outer as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_radius(cmds: &mut CommandsExchangeD3, light: f64, val: f64) {
    let light: Entity = as_entity(light);
    cmds.light_param.push(OpsLightParam::ops(light, ELightModify::Radius(val as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_color(cmds: &mut CommandsExchangeD3, light: f64, r: f64, g: f64, b: f64) {
    let light: Entity = as_entity(light);
    cmds.light_param.push(OpsLightParam::ops(light, ELightModify::Color(r as f32, g as f32, b as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstructmesh_force_point_light(cmds: &mut CommandsExchangeD3, mesh_or_instance: f64, light: f64, is_add: bool) {
    let mesh_or_instance: Entity = as_entity(mesh_or_instance);
    let light: Entity = as_entity(light);
    cmds.forcelighting.push(OpsMeshForceLighting::ops(mesh_or_instance, light, EMeshForceLighting::ForcePointLighting(is_add)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstructmesh_force_spot_light(cmds: &mut CommandsExchangeD3, mesh_or_instance: f64, light: f64, is_add: bool) {
    let mesh_or_instance: Entity = as_entity(mesh_or_instance);
    let light: Entity = as_entity(light);
    cmds.forcelighting.push(OpsMeshForceLighting::ops(mesh_or_instance, light, EMeshForceLighting::ForceSpotLighting(is_add)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstructmesh_force_hemi_light(cmds: &mut CommandsExchangeD3, mesh_or_instance: f64, light: f64, is_add: bool) {
    let mesh_or_instance: Entity = as_entity(mesh_or_instance);
    let light: Entity = as_entity(light);
    cmds.forcelighting.push(OpsMeshForceLighting::ops(mesh_or_instance, light, EMeshForceLighting::ForceHemiLighting(is_add)));
}
