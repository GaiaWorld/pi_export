
use js_proxy_gen_macro::pi_js_export;
use pi_engine_shell::prelude::*;
pub use pi_export_base::constants::*;
use pi_scene_context::prelude::*;

use crate::{as_dk, constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
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
pub fn p3d_create_render(app: &mut Engine, cmds: &mut CommandsExchangeD3, viewer: f64, name: String, pass_tag: f64, transparent: bool) -> f64 {

    let viewer: Entity = as_entity(viewer);

    let id_renderer: Entity = app.world.entities().reserve_entity();
    
    cmds.renderer_create.push(OpsRendererCreate::ops(id_renderer, name.clone(), viewer, PassTag::new(pass_tag as u16), transparent));

    as_f64(&id_renderer)
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_key(cmds: &mut CommandsExchangeD3, renderer: f64, keytarget: Option<f64>) {
    let renderer: Entity = as_entity(renderer);

    match keytarget {
        Some(keytarget) => {
            let keytarget = as_dk(&keytarget);
            cmds.renderer_target.push(OpsRendererTarget::ops(renderer, KeyCustomRenderTarget::Custom(keytarget)));
        },
        None => {
            cmds.renderer_target.push(OpsRendererTarget::ops(renderer, KeyCustomRenderTarget::FinalRender));
        },
    }
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_auto(cmds: &mut CommandsExchangeD3, renderer: f64, width: f64, height: f64, colorformat: f64, depthstencilformat: f64) {
    let renderer: Entity = as_entity(renderer);
    let colorformat =  EngineConstants::render_color_format(colorformat);
    let depthstencilformat =  EngineConstants::render_depth_format(depthstencilformat);

    cmds.renderer_target.push(OpsRendererTarget::Auto(renderer, width as u16, height as u16, colorformat, depthstencilformat));
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_color(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::AutoClearColor(renderer, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_depth(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::AutoClearDepth(renderer, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_stencil(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::AutoClearStencil(renderer, val));
}

/// r g b a 数值为 0 ~ 255 u8
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_color(cmds: &mut CommandsExchangeD3, renderer: f64, r: f64, g: f64, b: f64, a: f64) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::ColorClear(renderer, RenderColorClear(r as u8, g as u8, b as u8, a as u8)));
}

/// val 数值为 0.~1.
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_depth(cmds: &mut CommandsExchangeD3, renderer: f64, val: f64) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::DepthClear(renderer, RenderDepthClear(val as f32)));
}
///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_stencil(cmds: &mut CommandsExchangeD3, renderer: f64, val: f64) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::StencilClear(renderer, RenderStencilClear(val as u32)));
}
///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_viewport(cmds: &mut CommandsExchangeD3, renderer: f64, x: f64, y: f64, w: f64, h: f64) {
    let renderer: Entity = as_entity(renderer);

    cmds.renderer_modify.push(OpsRendererCommand::Viewport(renderer, x as f32, y as f32, w as f32, h as f32));
}