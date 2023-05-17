
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{ColorFormat, DepthFormat}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_transform_node(app: &mut Engine, param: &mut ActionSetScene3D, name: &pi_export_base::export::Atom, scene: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.transformcmds.create.push(OpsTransformNode::ops(scene, id, name.to_string()));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_transform_node_parent(app: &mut Engine, param: &mut ActionSetScene3D, node: f64, parent: f64) {
    let node: Entity = as_entity(node);
    let parent: Entity = as_entity(parent);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.transformcmds.tree.push(OpsTransformNodeParent::ops(node, parent));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_local_position(app: &mut Engine, param: &mut ActionSetScene3D, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(node, x as f32, y as f32, z as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_local_scaling(app: &mut Engine, param: &mut ActionSetScene3D, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(node, x as f32, y as f32, z as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_local_euler(app: &mut Engine, param: &mut ActionSetScene3D, node: f64, x: f64, y: f64, z: f64) {
    let node: Entity = as_entity(node);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(node, x as f32, y as f32, z as f32));
}
