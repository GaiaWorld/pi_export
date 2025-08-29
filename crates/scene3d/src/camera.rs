
use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

pub use crate::commands::CommandsExchangeD3;
use crate::record::ERecordCMD;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, graph: Option<f64>) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());

    pi_export_base::export::await_last_frame(app);

    let id: Entity = CommandsExchangeD3::p3d_entity(app);
    
    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CAMERA(scene, as_f64(&id), graph));

    let scene: Entity = as_entity(scene);
    let graph = if let Some(graph) = graph { as_entity(graph) } else { Entity::null() };
    CommandsExchangeD3::p3d_camera(cmds, scene, id, graph);

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_size(cmds: &mut CommandsExchangeD3, camera: f64, size: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECameraModify::OrthSize(size as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fov(cmds: &mut CommandsExchangeD3, camera: f64, fov: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECameraModify::Fov(fov as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_active(cmds: &mut CommandsExchangeD3, camera: f64, active: bool) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECameraModify::Active(active);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);
    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_orthographic: bool) {
    #[cfg(feature = "replay")]
    return ;

    let mode = if as_orthographic { EFreeCameraMode::Orthograhic } else { EFreeCameraMode::Perspective };
    let val = ECameraModify::FreeMode(mode);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fixed_mode(cmds: &mut CommandsExchangeD3, camera: f64, as_horizontal: bool) {
    #[cfg(feature = "replay")]
    return ;

    let mode = if as_horizontal { EFixedMode::HorizontalFixed } else { EFixedMode::VerticalFixed };
    let val = ECameraModify::FixMode(mode);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera,  val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_target(cmds: &mut CommandsExchangeD3, camera: f64, x: f64, y: f64, z: f64) {
    #[cfg(feature = "replay")]
    return ;


    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraTarget(camera, x , y , z ));

    let camera: Entity = as_entity(camera);
    CommandsExchangeD3::p3d_camera_target(cmds, camera,  x as f32, y as f32, z as f32);
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_nearfar(cmds: &mut CommandsExchangeD3, camera: f64, near: f64, far: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECameraModify::NearFar(near as f32, far as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

///
/// 相机宽高比
/// * @tip 传入null值对应 自适应宽高比
/// * @tip 传入正小数 表示自定义宽高比
/// * @example 16/9 16/10 null
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_aspect(cmds: &mut CommandsExchangeD3, camera: f64, val: Option<f64>) {
    #[cfg(feature = "replay")]
    return ;

    let val = if let Some(aspect) = val {
        ECameraModify::Aspect(aspect as f32)
    } else {
        ECameraModify::Aspect(1.0)
    };

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::CameraParam(camera, val));

    let camera: Entity = as_entity(camera);

    CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_viewer_force_include(cmds: &mut CommandsExchangeD3, viewer: f64, entity: f64, add: bool) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ViewerForceinclude(viewer, entity, add));

    let viewer: Entity = as_entity(viewer);
    let entity: Entity = as_entity(entity);
    CommandsExchangeD3::p3d_viewer_force_include(cmds, viewer, entity, add);
}
