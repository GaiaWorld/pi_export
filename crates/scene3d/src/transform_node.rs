
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;

pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_transform_node(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();
    let scene: Entity = as_entity(scene);

    CommandsExchangeD3::p3d_transform_node( cmds, scene, id);

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_transform_node_parent(cmds: &mut CommandsExchangeD3, node: f64, parent: f64) {
    let node: Entity = as_entity(node);
    let parent: Entity = as_entity(parent);

    CommandsExchangeD3::p3d_transform_node_parent(cmds, node, parent);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_transform_node_parent_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    // let parent: Entity = as_entity(parent);

    let len = len as usize;
    let count = len / 2;
    for i in 0..count {
        let node: Entity = as_entity(data[i * 2 + 0]);
        let parent: Entity = as_entity(data[i * 2 + 1]);
        CommandsExchangeD3::p3d_transform_node_parent(cmds, node, parent);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_enable(cmds: &mut CommandsExchangeD3, node: f64, val: bool) {
    let node: Entity = as_entity(node);

    CommandsExchangeD3::p3d_node_enable(cmds, node, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_enable_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    let len = len as usize;
    let size = 2;
    let count = len / size;
    for i in 0..count {
        let node: Entity = as_entity(data[i * size + 0]);
        let val = data[i * size + 1] > 0.;

        CommandsExchangeD3::p3d_node_enable(cmds, node, val);
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_position(cmds: &mut CommandsExchangeD3, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let val = ETransformSRT::Translation(x as f32, y as f32, z as f32);
    CommandsExchangeD3::p3d_local_srt(cmds, node, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_position_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    
    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let node: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1] as f32;
        let y = data[i * size + 2] as f32;
        let z = data[i * size + 3] as f32;
        let val =  ETransformSRT::Translation(x as f32, y as f32, z as f32);
        CommandsExchangeD3::p3d_local_srt(cmds, node, val);
    }

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_scaling(cmds: &mut CommandsExchangeD3, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let val = ETransformSRT::Scaling(x as f32, y as f32, z as f32);
    CommandsExchangeD3::p3d_local_srt(cmds, node, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_scaling_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    
    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let node: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1] as f32;
        let y = data[i * size + 2] as f32;
        let z = data[i * size + 3] as f32;
        let val = ETransformSRT::Scaling(x as f32, y as f32, z as f32);
        CommandsExchangeD3::p3d_local_srt(cmds, node, val);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_euler(cmds: &mut CommandsExchangeD3, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let val = ETransformSRT::Euler(x as f32, y as f32, z as f32);
    CommandsExchangeD3::p3d_local_srt(cmds, node, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_euler_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    
    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let node: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1] as f32;
        let y = data[i * size + 2] as f32;
        let z = data[i * size + 3] as f32;
        let val = ETransformSRT::Euler(x as f32, y as f32, z as f32);
        CommandsExchangeD3::p3d_local_srt(cmds, node, val);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_quaternion(cmds: &mut CommandsExchangeD3, node: f64, x: f64, y: f64, z: f64, w: f64) {
    let node: Entity = as_entity(node);

    CommandsExchangeD3::p3d_local_quaternion(cmds, node, x as f32, y as f32, z as f32, w as f32);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_local_quaternion_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let node: Entity = as_entity(node);
    
    let len = len as usize;
    let size = 5;
    let count = len / size;
    for i in 0..count {
        let node: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1] as f32;
        let y = data[i * size + 2] as f32;
        let z = data[i * size + 3] as f32;
        let w = data[i * size + 4] as f32;
        CommandsExchangeD3::p3d_local_quaternion(cmds, node, x as f32, y as f32, z as f32, w as f32);
    }
}