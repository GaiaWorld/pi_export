
use std::mem::transmute;

use js_proxy_gen_macro::pi_js_export;
use pi_3d::TActionSet;
#[cfg(feature = "record")]
use pi_export_base::record::ERecord3D;
use pi_scene_shell::prelude::*;
pub use pi_export_base::constants::*;
use pi_scene_context::prelude::*;

use crate::record::ERecordCMD;
use crate::{as_dk, constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_render_subgraph(app: &mut Engine, cmds: &mut CommandsExchangeD3, name: String) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());


    let id_renderer: Entity = app.world.entities().reserve_entity();

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id_renderer));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderSubgraph(as_f64(&id_renderer), name.clone()));

    cmds.renderer_subgraph().push(OpsSubGraphCreate::ops(id_renderer, name));

    as_f64(&id_renderer)
}

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
pub fn p3d_create_render(app: &mut Engine, cmds: &mut CommandsExchangeD3, viewer: f64, name: String, pass_tag: f64, transparent: bool, recordinput: Option<bool>, crossrender: Option<bool>) -> f64 {

    #[cfg(feature = "replay")]
    return as_f64(Entity::null());

    let id_renderer: Entity = app.world.entities().reserve_entity();

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id_renderer));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RENDER(viewer, as_f64(&id_renderer), name.clone(), pass_tag, transparent, recordinput, crossrender));

    let viewer: Entity = as_entity(viewer);
    CommandsExchangeD3::p3d_create_render(cmds, viewer, id_renderer, name, pass_tag, transparent, recordinput, crossrender);

    // cmds.renderer_create.push(OpsRendererCreate::ops(id_renderer, name.clone(), viewer, PassTag::new(pass_tag as u16), transparent, recordinput, crossrender));

    as_f64(&id_renderer)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_enabled(cmds: &mut CommandsExchangeD3, renderer: f64, enable: bool) {
    #[cfg(feature = "replay")]
    return ;

    
    let val = ERendererCommand::Active(enable);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_link_mesh(cmds: &mut CommandsExchangeD3, renderer: f64, mesh: f64) {

}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_key(cmds: &mut CommandsExchangeD3, renderer: f64, keytarget: Option<f64>, use_as_out: Option<bool>, realtoscreen: Option<bool>) {
    #[cfg(feature = "replay")]
    return ;


    let use_as_out = if let Some(use_as_out) = use_as_out { use_as_out } else { false };
    let realtoscreen = if let Some(realtoscreen) = realtoscreen { realtoscreen } else { false };
    let val = match keytarget {
        Some(keytarget) => {
            let keytarget = unsafe { transmute(keytarget) };
            ERendererTarget::Custom(KeyCustomRenderTarget::Custom(keytarget), use_as_out)
        },
        None => {
            ERendererTarget::Custom(KeyCustomRenderTarget::FinalRender(realtoscreen), use_as_out)
        },
    };
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderTarget(renderer, val.clone()));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_render_target(cmds, renderer, val);
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_target_auto(cmds: &mut CommandsExchangeD3, renderer: f64, width: f64, height: f64, colorformat: f64, depthstencilformat: f64, force_allocate: Option<bool>) {

    #[cfg(feature = "replay")]
    return ;

    let colorformat =  EngineConstants::render_color_format(colorformat);
    let depthstencilformat =  EngineConstants::render_depth_format(depthstencilformat);

    let force_allocate = if let Some(force_allocate) = force_allocate { force_allocate } else { true };

    let val = ERendererTarget::Auto(width as u16, height as u16, colorformat, depthstencilformat, force_allocate);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderTarget(renderer, val.clone()));
    
    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_render_target(cmds, renderer, val);
}

/// 
/// Renderer Modify
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_color(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::AutoClearColor( val);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_depth(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::AutoClearDepth( val);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_auto_clear_stencil(cmds: &mut CommandsExchangeD3, renderer: f64, val: bool) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::AutoClearStencil( val);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

/// r g b a 数值为 0 ~ 255 u8
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_color(cmds: &mut CommandsExchangeD3, renderer: f64, r: f64, g: f64, b: f64, a: f64) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::ColorClear( RenderColorClear(r as u8, g as u8, b as u8, a as u8));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

/// val 数值为 0.~1.
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_depth(cmds: &mut CommandsExchangeD3, renderer: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::DepthClear( RenderDepthClear(val as f32));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}
///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_clear_stencil(cmds: &mut CommandsExchangeD3, renderer: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;


    let val = ERendererCommand::StencilClear( RenderStencilClear(val as u32));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}
///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_viewport(cmds: &mut CommandsExchangeD3, renderer: f64, x: f64, y: f64, w: f64, h: f64, mindepth: Option<f64>, maxdepth: Option<f64>) {
    #[cfg(feature = "replay")]
    return ;

    let mindepth = if let Some(mindepth) = mindepth { mindepth as f32 } else { 0. };
    let maxdepth = if let Some(maxdepth) = maxdepth { maxdepth as f32 } else { 1. };

    let val = ERendererCommand::Viewport( x as f32, y as f32, w as f32, h as f32, mindepth, maxdepth);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderModify(renderer, val));

    let renderer: Entity = as_entity(renderer);
    CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
}

///
/// val 数值为 u32
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_crossrender_link_drawlists(cmds: &mut CommandsExchangeD3, linkentity: f64, drawlistrenderers: &[f64], len: f64) {
    #[cfg(feature = "replay")]
    return ;

    let mut list = vec![];
    let len = len as usize;
    if len > 0 {
        for i in 0..len {
            let entity: Entity = as_entity(drawlistrenderers[i]);
            list.push(entity);
        }
    };

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::RenderLinkDrawlist(linkentity, drawlistrenderers[0..len].to_vec()));

    let linkentity: Entity = as_entity(linkentity);
    CommandsExchangeD3::p3d_crossrender_link_drawlists(cmds, linkentity, list);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_screenwithpostprocess(cmds: &mut CommandsExchangeD3, flag: bool) {
    #[cfg(feature = "replay")]
    return ;

    
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::RenderScreenWithPostprocess(flag));

    CommandsExchangeD3::p3d_render_screenwithpostprocess(cmds, flag);
}
