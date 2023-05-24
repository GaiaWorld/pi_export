
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{ColorFormat, DepthFormat}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64, material::OpsPass};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera(app: &mut Engine, param: &mut ActionSetScene3D, name: &pi_export_base::export::Atom, scene: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.cameracmds.create.push(OpsCameraCreation::ops(scene, id, name.to_string()));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera_size(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, size: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.cameracmds.size.push(OpsCameraOrthSize::ops(camera, size as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera_active(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, active: bool) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.cameracmds.active.push(OpsCameraActive::ops(camera, active));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera_mode(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, as_orthographic: bool) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.cameracmds.mode.push(OpsCameraMode::ops(camera, as_orthographic));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera_target(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, x: f64, y: f64, z: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.cameracmds.target.push(OpsCameraTarget::ops(camera, x as f32, y as f32, z as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct PassOrders(Vec<EPassTag>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl PassOrders {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn create() -> Self {
        Self(vec![])
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_pass_orders(orders: &mut PassOrders, pass: OpsPass) {

    orders.0.push(pass.val());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_camera_render(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, pre_node: Option<f64>, next_node: Option<f64>, pass_orders: PassOrders, color_format: ColorFormat, depth: DepthFormat) -> f64 {
    let camera: Entity = as_entity(camera);
    let pre_node = if let Some(pre_node) = pre_node {
        Some(Atom::from(as_entity(pre_node).to_bits().to_string()))
    } else {
        None
    };
    let next_node = if let Some(next_node) = next_node {
        Some(Atom::from(as_entity(next_node).to_bits().to_string()))
    } else {
        None
    };

    let id_renderer: Entity = app.world.spawn_empty().id();

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    let desc = RendererGraphicDesc {
        pre: pre_node,
        curr: Atom::from(id_renderer.to_bits().to_string()),
        next: next_node,
        passorders: PassTagOrders::new(pass_orders.0)
    };
    scenecmds.cameracmds.render.push(OpsCameraRendererInit::ops(camera, id_renderer, desc, color_format.val(), depth.val()));

    as_f64(&id_renderer)
}
