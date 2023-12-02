
use js_proxy_gen_macro::pi_js_export;
use pi_engine_shell::prelude::*;
pub use pi_export_base::constants::*;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector3;

use crate::{as_dk, constants::EngineConstants};
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;
pub use crate::engine::ActionSetScene3D;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, ltype: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.transformcmds.tree.push(OpsTransformNodeParent::ops(id, scene));
    cmds.lightcmds.create.push(OpsLightCreate::ops(scene, id, EngineConstants::light(ltype)));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_direction(app: &mut Engine, param: &mut ActionSetScene3D, light: f64, x: f64, y: f64, z: f64) {
    let light: Entity = as_entity(light);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.lightcmds.param.push(ELightModifyCommand::Directional(light, Vector3::new(x as f32, y as f32, z as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_strength(app: &mut Engine, param: &mut ActionSetScene3D, light: f64, val: f64) {
    let light: Entity = as_entity(light);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.lightcmds.strength.push(OpsLightStrength::ops(light, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_spot_angle(app: &mut Engine, param: &mut ActionSetScene3D, light: f64, inner: f64, outer: f64) {
    let light: Entity = as_entity(light);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.lightcmds.spotangle.push(OpsSpotLightAngle::ops(light, inner as f32, outer as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_radius(app: &mut Engine, param: &mut ActionSetScene3D, light: f64, val: f64) {
    let light: Entity = as_entity(light);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.lightcmds.radius.push(OpsLightRadius::ops(light, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_light_color(app: &mut Engine, param: &mut ActionSetScene3D, light: f64, r: f64, g: f64, b: f64) {
    let light: Entity = as_entity(light);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.lightcmds.color.push(OpsLightColor::ops(light, r as f32, g as f32, b as f32));
}

