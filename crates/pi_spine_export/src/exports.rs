
use std::{mem::transmute, ops::Deref};

use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_ecs_extend::action::ActionList;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, render_cross::GraphId};
use pi_export_base::export::await_last_frame;
pub use pi_export_base::{export::Engine, constants::*, asset::TextureDefaultView};
// use pi_window_renderer::{WindowRenderer, PluginWindowRender};
use pi_hash::XHashMap;
use pi_render::renderer::texture::{ETextureViewUsage, KeyImageTexture, KeyImageTextureView, TextureViewDesc};
use pi_render::{renderer::sampler::SamplerRes, asset::TAssetKeyU64, rhi::sampler::EAnisotropyClamp};
use pi_spine_rs::ecs::WorldResourceTemp;
use pi_spine_rs::ecs::WorldPluginExtent;

// pub use pi_export_base::constants::BlendFactor;
use js_proxy_gen_macro::pi_js_export;
use pi_spine_rs::{
    KeySpineRenderer,
    SpineRenderContext,
    ActionListSpine,
    ActionSpine,
    PluginSpineRenderer,
    SpineTextureLoad
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
pub use pi_export_base::export::Atom;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySpineShader {
    Colored,
    ColoredTextured,
    TwoColoredTextured,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Default)]
pub struct CommandsExchangeSpine(pub(crate) ActionListSpine);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl CommandsExchangeSpine {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self::default()
    }
}
impl CommandsExchangeSpine {
    pub(crate) fn exchange(&mut self, cmds: &mut ActionListSpine) {
		std::mem::swap(cmds, &mut self.0);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_exchange_commands(
    engine: &mut Engine, cmds: &mut CommandsExchangeSpine,
) {
	// #[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
	// {
	// 	log::warn!("fram_call start=====");
	// 	if engine.last_frame_awaiting {
	// 		engine.back_receiver.recv().unwrap();
	// 	} else {
	// 		engine.last_frame_awaiting = true;
	// 	}
	// }
	pi_export_base::export::await_last_frame(engine);
    let mut commands = engine.world.get_resource_mut::<ActionListSpine>().unwrap();
    cmds.exchange(&mut commands);
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn init_spine_context(
    engine: &mut Engine,
) {
	// await_last_frame(engine);
    // engine
    //     .add_plugins(PluginSpineRenderer);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_renderer_create(app: &mut Engine, cmds: &mut CommandsExchangeSpine, name: String, width: Option<f64>, height: Option<f64>) -> f64 {
    let rendersize: Option<(u32, u32)> =  match (width, height) {
		(Some(w), Some(h)) => Some((w as u32, h as u32)),
		_ => None,
    };
    // log::warn!("Spine To Screen: {:?}", rendersize.is_none());

    let id_renderer = {
        let id = app.world.make_entity_editor().alloc_entity();
        let id_renderer = KeySpineRenderer(id);
    
        // // let final_render_format = app.world.get_resource::<WindowRenderer>().unwrap().format();
        // let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
        // ActionSpine::create_spine_renderer(id_renderer, rendersize, &mut ctx, wgpu::TextureFormat::Rgba8Unorm);
        
        id_renderer
    };
    
    cmds.0.push(
        pi_spine_rs::ESpineCommand::Create(id_renderer, name, rendersize, wgpu::TextureFormat::Rgba8Unorm)
    );

    // let f = {

    //     let mut render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
    //     match ActionSpine::spine_renderer_apply(id_renderer, pi_atom::Atom::from(name), rendersize.is_none(), &mut render_graph) {
    //         Ok(nodeid) => {
    //             app.world.entity_mut(id_renderer.0).insert(GraphId(nodeid));
    //         },
    //         Err(e) => {
    //             log::warn!("Spine render_graph Err {:?}", e);
    //         },
    //     }
    
    
    //     unsafe { transmute(id_renderer.0.to_bits()) }
    
    // };
    
    unsafe { transmute(id_renderer.0) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_renderer_dispose(cmds: &mut CommandsExchangeSpine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    
    cmds.0.push(
        pi_spine_rs::ESpineCommand::Dispose(id_renderer)
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_load(cmds: &mut CommandsExchangeSpine, key: &Atom, srgb: bool, compressed: bool, file: bool) -> f64 {
    let key = KeyImageTexture {
        url: key.deref().clone(),
        srgb, compressed,
        file, depth_or_array_layers: 0, useage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
    };
    let key = KeyImageTextureView::new(key, TextureViewDesc::default());
    cmds.0.push(
        pi_spine_rs::ESpineCommand::TextureLoad(key.clone())
    );

    return unsafe { transmute(key.asset_u64()) };
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct SpineTextureLoadRecord {
    success: XHashMap<pi_atom::Atom, TextureDefaultView>,
    fail: XHashMap<pi_atom::Atom, String>,
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl SpineTextureLoadRecord {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn new() -> Self {
        Self { success: XHashMap::default(), fail: XHashMap::default() }
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn success(&mut self, key: &Atom) -> Option<TextureDefaultView> {
        self.success.remove(&**key)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn fail(&mut self, key: &Atom) -> Option<String> {
        self.fail.remove(&**key)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn clear(&mut self) {
        self.fail.clear();
        self.success.clear();
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_loaded_query(app: &mut Engine, record: &mut SpineTextureLoadRecord) {
	pi_export_base::export::await_last_frame(app);
    let loader = app.world.get_resource::<SpineTextureLoad>().unwrap();
    while let Some((key, res)) = loader.success.pop() {
        record.success.insert(key.url().url.clone(), TextureDefaultView::new(ETextureViewUsage::Image(res), key.asset_u64()));
    }
    while let Some((key, res)) = loader.fail.pop() {
        record.fail.insert(key.url().url.clone(), res);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_record(cmds: &mut CommandsExchangeSpine, id_renderer: f64, texture: &TextureDefaultView) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    
    cmds.0.push(
        pi_spine_rs::ESpineCommand::TextureRecord(id_renderer, texture.key(), texture.data().clone())
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_remove_texture(cmds: &mut CommandsExchangeSpine, id_renderer: f64, key: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let key = unsafe { transmute(key) };
    cmds.0.push(
        pi_spine_rs::ESpineCommand::RemoveTextureRecord(id_renderer, key)
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_use_texture(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, texture: &TextureDefaultView, 
    address_mode_u: f64,
    address_mode_v: f64,
    address_mode_w: f64,
    mag_filter: f64,
    min_filter: f64,
    mipmap_filter: f64,
) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);

    let samplerdesc = sampler_desc(
        ContextConstants::address_mode(address_mode_u),
        ContextConstants::address_mode(address_mode_v),
        ContextConstants::address_mode(address_mode_w),
        ContextConstants::filter_mode(mag_filter),
        ContextConstants::filter_mode(min_filter),
        ContextConstants::filter_mode(mipmap_filter),
        None,
        EAnisotropyClamp::None,
        None,
    );
    ActionSpine::spine_use_texture(&mut cmds.0, id_renderer, texture.data().clone(), samplerdesc)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_shader(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, shader: KeySpineShader) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_shader(&mut cmds.0, id_renderer, unsafe {transmute(shader)});
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_blend(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, val: bool) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_blend(&mut cmds.0, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_blend_mode(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, src: f64, dst: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_blend_mode(&mut cmds.0, id_renderer, ContextConstants::blend_factor(src).val(), ContextConstants::blend_factor(dst).val());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_uniform(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, val: &[f32]) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_uniform(&mut cmds.0, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_draw(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64, vertices: &[f32], indices: &[u16], vlen: u32, ilen: u32) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_draw(&mut cmds.0, id_renderer, vertices, indices, vlen, ilen);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_reset(app: &mut Engine, cmds: &mut CommandsExchangeSpine, id_renderer: f64) {

    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    ActionSpine::spine_reset(&mut cmds.0, id_renderer);
}