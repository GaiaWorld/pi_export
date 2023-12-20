
use pi_engine_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;

use crate::{constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh(app: &mut Engine, cmds: &mut CommandsExchangeD3, source: f64) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();
    let source: Entity = as_entity(source);

    cmds.instance_create.push(OpsInstanceMeshCreation::ops(source, id));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_color(cmds: &mut CommandsExchangeD3, instance: f64, r: f64, g: f64, b: f64) {
    let instance: Entity = as_entity(instance);

    cmds.instance_color.push(OpsInstanceColor::ops(instance, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_color_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let instance: Entity = as_entity(instance);    // cmds.instancemeshcmds_color.push(OpsInstanceColor::ops(instance, r as f32, g as f32, b as f32));

    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let instance: Entity = as_entity(data[i * size + 0]);
        let r = data[i * size + 1];
        let g = data[i * size + 2];
        let b = data[i * size + 3];
        cmds.instance_color.push(OpsInstanceColor::ops(instance, r as f32, g as f32, b as f32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_alpha(cmds: &mut CommandsExchangeD3, instance: f64, val: f64) {
    let instance: Entity = as_entity(instance);    cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_alpha_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let instance: Entity = as_entity(instance);    // cmds.instancemeshcmds_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));

    let len = len as usize;
    let size = 2;
    let count = len / size;
    for i in 0..count {
        let instance: Entity = as_entity(data[i * size + 0]);
        let alpha = data[i * size + 1];
        cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, alpha as f32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bone_offset(cmds: &mut CommandsExchangeD3, instance: f64, val: f64) {
    let instance: Entity = as_entity(instance);    cmds.abstructmesh_boneoffset.push(OpsBoneOffset::ops(instance, val as u32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bone_offset_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let instance: Entity = as_entity(instance);    // cmds.abstructmeshcmds_boneoffset.push(OpsBoneOffset::ops(instance, val as u32));

    let len = len as usize;
    let size = 2;
    let count = len / size;
    for i in 0..count {
        let instance: Entity = as_entity(data[i * size + 0]);
        let val = data[i * size + 1];
        cmds.abstructmesh_boneoffset.push(OpsBoneOffset::ops(instance, val as u32));
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_tilloff(cmds: &mut CommandsExchangeD3, instance: f64, uscale: f64, vscale: f64, uoffset: f64, voffset: f64) {
    let instance: Entity = as_entity(instance);    cmds.instance_tilloff.push(OpsInstanceTillOff::ops(instance, uscale as f32, vscale as f32, uoffset as f32, voffset as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_tilloff_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let instance: Entity = as_entity(instance);    // cmds.instancemeshcmds_tilloff.push(OpsInstanceTillOff::ops(instance, uscale as f32, vscale as f32, uoffset as f32, voffset as f32));

    let len = len as usize;
    let size = 5;
    let count = len / size;
    for i in 0..count {
        let instance: Entity = as_entity(data[i * size + 0]);
        let uscale = data[i * size + 1];
        let vscale = data[i * size + 2];
        let uoffset = data[i * size + 3];
        let voffset = data[i * size + 4];
        cmds.instance_tilloff.push(OpsInstanceTillOff::ops(instance, uscale as f32, vscale as f32, uoffset as f32, voffset as f32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_attr_float(cmds: &mut CommandsExchangeD3, instance: f64, vtype: f64, val: f64) {
    let instance: Entity = as_entity(instance);    cmds.instance_floats.push(OpsInstanceFloat::ops(instance, val as f32, EngineConstants::instance_attr_float(vtype)));
}
