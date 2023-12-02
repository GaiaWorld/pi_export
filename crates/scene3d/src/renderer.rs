
use js_proxy_gen_macro::pi_js_export;
use pi_engine_shell::prelude::*;
pub use pi_export_base::constants::*;
use pi_scene_context::prelude::*;

use crate::{as_dk, constants::EngineConstants};
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;
pub use crate::engine::ActionSetScene3D;

/// * pass_tag:
///     * 0b0000_0000_0000_0001
///     * 0b0000_0000_0000_0010
///     * 0b0000_0000_0000_0100
///     * 0b0000_0000_0000_1000
///     * 0b0000_0000_0001_0000
///     * 0b0000_0000_0010_0000
///     * 0b0000_0000_0100_0000
///     * 0b0000_0000_1000_0000
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_render(app: &mut Engine, param: &mut ActionSetScene3D, viewer: f64, name: String, pass_tag: f64, transparent: bool) -> f64 {

    let viewer: Entity = as_entity(viewer);
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
    scenecmds.renderercmds.create.push(OpsRendererCreate::ops(id_renderer, name.clone(), viewer, PassTag::new(pass_tag as u16), transparent));
    // scenecmds.cameracmds.render.push(OpsCameraRendererInit::ops(camera, id_renderer, name, pi_scene_context::pass::PassTagOrders::new(pass_orders.0.clone()), color_format.val(), depth.format()));

    as_f64(&id_renderer)
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_key(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, keytarget: Option<f64>) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    match keytarget {
        Some(keytarget) => {
            let keytarget = as_dk(&keytarget);
            scenecmds.renderercmds.target.push(OpsRendererTarget::ops(renderer, KeyCustomRenderTarget::Custom(keytarget)));
        },
        None => {
            scenecmds.renderercmds.target.push(OpsRendererTarget::ops(renderer, KeyCustomRenderTarget::FinalRender));
        },
    }
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_auto(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64, width: f64, height: f64, colorformat: f64, depthstencilformat: f64) {
    let renderer: Entity = as_entity(renderer);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let colorformat =  EngineConstants::render_color_format(colorformat);
    let depthstencilformat =  EngineConstants::render_depth_format(depthstencilformat);

    scenecmds.renderercmds.target.push(OpsRendererTarget::Auto(renderer, width as u16, height as u16, colorformat, depthstencilformat));
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

/// r g b a 数值为 0 ~ 255 u8
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