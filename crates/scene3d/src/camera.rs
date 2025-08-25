
use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, graph: Option<f64>) -> f64 {
    pi_export_base::export::await_last_frame(app);

    let id: Entity = CommandsExchangeD3::p3d_entity(app);

    let scene: Entity = as_entity(scene);
    let graph = if let Some(graph) = graph { as_entity(graph) } else { Entity::null() };

    CommandsExchangeD3::p3d_camera(cmds, scene, id, graph);

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_size(cmds: &mut CommandsExchangeD3, camera: f64, size: f64) {
    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::OrthSize(size as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fov(cmds: &mut CommandsExchangeD3, camera: f64, fov: f64) {
    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::Fov(fov as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_active(cmds: &mut CommandsExchangeD3, camera: f64, active: bool) {
    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::Active(active));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_orthographic: bool) {
    let camera: Entity = as_entity(camera);
    let mode = if as_orthographic { EFreeCameraMode::Orthograhic } else { EFreeCameraMode::Perspective };

    CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::FreeMode(mode));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fixed_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_horizontal: bool) {
    let camera: Entity = as_entity(camera);
    let mode = if as_horizontal { EFixedMode::HorizontalFixed } else { EFixedMode::VerticalFixed };

    CommandsExchangeD3::p3d_camera_param(cmds, camera,  ECameraModify::FixMode(mode));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_target(cmds: &mut CommandsExchangeD3, camera: f64, x: f64, y: f64, z: f64) {
    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_target(cmds, camera,  x as f32, y as f32, z as f32);
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_nearfar(cmds: &mut CommandsExchangeD3, camera: f64, near: f64, far: f64) {
    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::NearFar(near as f32, far as f32));
}

///
/// 相机宽高比
/// * @tip 传入null值对应 自适应宽高比
/// * @tip 传入正小数 表示自定义宽高比
/// * @example 16/9 16/10 null
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_aspect(cmds: &mut CommandsExchangeD3, camera: f64, val: Option<f64>) {

    let camera: Entity = as_entity(camera);

    if let Some(aspect) = val {
        CommandsExchangeD3::p3d_camera_param(cmds, camera,  ECameraModify::Aspect(aspect as f32));
    } else {
        CommandsExchangeD3::p3d_camera_param(cmds, camera, ECameraModify::Aspect(1.0));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_viewer_force_include(cmds: &mut CommandsExchangeD3, viewer: f64, entity: f64, add: bool) {
    let viewer: Entity = as_entity(viewer);
    let entity: Entity = as_entity(entity);

    CommandsExchangeD3::p3d_viewer_force_include(cmds, viewer, entity, add);
}
