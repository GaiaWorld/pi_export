
use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;
pub use crate::engine::ActionSetScene3D;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64) -> f64 {
    let id: Entity = app.world.make_entity_editor().alloc_entity();
    let scene: Entity = as_entity(scene);

    cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
    cmds.camera_create.push(OpsCameraCreation::ops(scene, id));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_size(cmds: &mut CommandsExchangeD3, camera: f64, size: f64) {
    let camera: Entity = as_entity(camera);

    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::OrthSize(size as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fov(cmds: &mut CommandsExchangeD3, camera: f64, fov: f64) {
    let camera: Entity = as_entity(camera);

    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::Fov(fov as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_active(cmds: &mut CommandsExchangeD3, camera: f64, active: bool) {
    let camera: Entity = as_entity(camera);

    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::Active(active)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_orthographic: bool) {
    let camera: Entity = as_entity(camera);

    let mode = if as_orthographic { EFreeCameraMode::Orthograhic } else { EFreeCameraMode::Perspective };
    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::FreeMode(mode)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fixed_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_horizontal: bool) {
    let camera: Entity = as_entity(camera);

    let mode = if as_horizontal { EFixedMode::HorizontalFixed } else { EFixedMode::VerticalFixed };
    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::FixMode(mode)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_target(cmds: &mut CommandsExchangeD3, camera: f64, x: f64, y: f64, z: f64) {
    let camera: Entity = as_entity(camera);

    cmds.camera_target.push(OpsCameraTarget::ops(camera, x as f32, y as f32, z as f32));
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_nearfar(cmds: &mut CommandsExchangeD3, camera: f64, near: f64, far: f64) {
    let camera: Entity = as_entity(camera);

    cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::NearFar(near as f32, far as f32)));
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
        cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::Aspect(aspect as f32)));
    } else {
        cmds.camera_param.push(OpsCameraModify::ops(camera, ECameraModify::Aspect(1.0)));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_viewer_force_include(cmds: &mut CommandsExchangeD3, viewer: f64, entity: f64, add: bool) {
    let viewer: Entity = as_entity(viewer);
    let entity: Entity = as_entity(entity);

    cmds.camera_forceinclude.push(OpsViewerForceInclude::ops(viewer, entity, add));
}
