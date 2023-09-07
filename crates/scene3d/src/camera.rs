
use js_proxy_gen_macro::pi_js_export;
use pi_engine_shell::prelude::*;
pub use pi_export_base::constants::{DepthStencilFormat, RenderFormat};
use pi_scene_context::prelude::*;

pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;
pub use crate::engine::ActionSetScene3D;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum OpsPass {
    ShadowCast,
    Opaque,
    Sky,
    Water,
    AlphaTest,
    Transparent,
    OpaqueExtend,
    TransparentExtend,
}
impl OpsPass {
    pub fn val(&self) -> EPassTag {
        match self {
            OpsPass::ShadowCast => EPassTag::ShadowCast,
            OpsPass::Opaque => EPassTag::Opaque,
            OpsPass::Sky => EPassTag::Sky,
            OpsPass::Water => EPassTag::Water,
            OpsPass::AlphaTest => EPassTag::AlphaTest,
            OpsPass::Transparent => EPassTag::Transparent,
            OpsPass::OpaqueExtend => EPassTag::OpaqueExtend,
            OpsPass::TransparentExtend => EPassTag::TransparentExtend,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, toscreen: bool) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.transformcmds.tree.push(OpsTransformNodeParent::ops(id, scene));
    scenecmds.cameracmds.create.push(OpsCameraCreation::ops(scene, id,  toscreen));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_size(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, size: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.size.push(OpsCameraOrthSize::ops(camera, size as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_active(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, active: bool) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.active.push(OpsCameraActive::ops(camera, active));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_mode(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, as_orthographic: bool) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.mode.push(OpsCameraMode::ops(camera, as_orthographic));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_fixed_mode(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, as_horizontal: bool) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.fixmode.push(OpsCameraFixedMode::ops(camera, as_horizontal));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_target(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, x: f64, y: f64, z: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.target.push(OpsCameraTarget::ops(camera, x as f32, y as f32, z as f32));
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_pxiel_size(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, w: f64, h: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.pixelsize.push(OpsCameraPixelSize::ops(camera, w as u32, h as u32));
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_nearfar(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, near: f64, far: f64) {
    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.cameracmds.nearfar.push(OpsCameraNearFar::ops(camera, near as f32, far as f32));
}

///
/// 相机宽高比
/// * @tip 传入null值对应 自适应宽高比
/// * @tip 传入正小数 表示自定义宽高比
/// * @example 16/9 16/10 null
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_aspect(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, val: Option<f64>) {
    use pi_scene_context::viewer::prelude::ViewerAspect;

    let camera: Entity = as_entity(camera);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if let Some(aspect) = val {
        scenecmds.cameracmds.aspect.push(OpsCameraAspect::ops(camera, ViewerAspect::Custom(aspect as f32)));
    } else {
        scenecmds.cameracmds.aspect.push(OpsCameraAspect::ops(camera, ViewerAspect::Auto));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct PassOrders(Vec<EPassTag>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl PassOrders {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self(vec![])
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_pass_orders(orders: &mut PassOrders, pass: &OpsPass) {

    orders.0.push(pass.val());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_render(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, name: String, pass_orders: &PassOrders, color_format: RenderFormat, depth: DepthStencilFormat) -> f64 {

    let camera: Entity = as_entity(camera);
    // let pre_node = if let Some(pre_node) = pre_node {
    //     Some(as_entity(pre_node))
    // } else {
    //     None
    // };
    // let next_node = if let Some(next_node) = next_node {
    //     Some(as_entity(next_node))
    // } else {
    //     None
    // };

    let id_renderer: Entity = app.world.spawn_empty().id();

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    // let mut orders: Vec<EPassTag> = vec![];
    // pass_orders.0.iter().for_each(|pass| {
    //     orders.push(pass.)
    // });

    // let desc = RendererGraphicDesc {
    //     pre: pre_node,
    //     curr: name.clone(),
    //     next: next_node,
    //     passorders: PassTagOrders::new(pass_orders.0.clone())
    // };
    scenecmds.renderercmds.create.push(OpsRendererCreate::ops(id_renderer, name.clone()));
    scenecmds.cameracmds.render.push(OpsCameraRendererInit::ops(camera, id_renderer, name, pi_scene_context::pass::PassTagOrders::new(pass_orders.0.clone()), color_format.val(), depth.format()));

    as_f64(&id_renderer)
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_color(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::AutoClearColor(renderer, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_depth(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(renderer, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_stencil(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::AutoClearStencil(renderer, val));
}

/// e g b a 数值为 0 ~ 255 u8
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_color(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, r: f64, g: f64, b: f64, a: f64) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::ColorClear(renderer, RenderColorClear(r as u8, g as u8, b as u8, a as u8)));
}

/// val 数值为 0.~1.
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_depth(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, val: f64) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::DepthClear(renderer, RenderDepthClear(val as f32)));
}
///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_stencil(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, val: f64) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.renderercmds.modify.push(OpsRendererCommand::StencilClear(renderer, RenderStencilClear(val as u32)));
}