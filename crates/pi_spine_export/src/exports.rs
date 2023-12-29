
use std::{mem::transmute, ops::Deref};

use bevy_ecs::{prelude::Commands, system::CommandQueue, query::QueryState};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, component::GraphId};
use pi_export_base::export::await_last_frame;
pub use pi_export_base::{export::Engine, constants::*, asset::TextureDefaultView};
// use pi_window_renderer::{WindowRenderer, PluginWindowRender};
use pi_hash::XHashMap;
use pi_render::{renderer::sampler::SamplerRes, asset::TAssetKeyU64, rhi::sampler::EAnisotropyClamp};

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
	await_last_frame(engine);
    engine
        .add_plugins(PluginSpineRenderer);
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
        let id = app.world.entities().reserve_entity();
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

    unsafe { transmute(id_renderer.0.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_renderer_dispose(cmds: &mut CommandsExchangeSpine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    
    cmds.0.push(
        pi_spine_rs::ESpineCommand::Dispose(id_renderer)
    );
    
    // let mut nodequery: QueryState<&'static GraphId> = app.world.query();
    
    // if let Ok(nodeid) = nodequery.get(&app.world, id_renderer.0).cloned() {
    //     let mut render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
    //     render_graph.remove_node(nodeid.0);
    // }

    // let mut queue = CommandQueue::default();
    // let mut commands = Commands::new(&mut queue, &app.world);

    // commands.entity(id_renderer.0).despawn();
    // queue.apply(&mut app.world);

    // let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    // ActionSpine::dispose_spine_renderer(id_renderer, &mut ctx);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_load(cmds: &mut CommandsExchangeSpine, key: &Atom) {
    cmds.0.push(
        pi_spine_rs::ESpineCommand::TextureLoad(key.deref().clone())
    );
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
        self.success.remove(key)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn fail(&mut self, key: &Atom) -> Option<String> {
        self.fail.remove(key)
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
        record.success.insert(key, TextureDefaultView::from(res));
    }
    while let Some((key, res)) = loader.fail.pop() {
        record.fail.insert(key, res);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_record(cmds: &mut CommandsExchangeSpine, id_renderer: f64, texture: &TextureDefaultView) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    
    cmds.0.push(
        pi_spine_rs::ESpineCommand::TextureRecord(id_renderer, texture.data().clone())
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_remove_texture(cmds: &mut CommandsExchangeSpine, id_renderer: f64, key: &Atom) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    cmds.0.push(
        pi_spine_rs::ESpineCommand::RemoveTextureRecord(id_renderer, key.asset_u64())
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
	pi_export_base::export::await_last_frame(app);
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let device = app.world.get_resource::<PiRenderDevice>().unwrap().clone();
    let asset_samplers = app.world.get_resource::<ShareAssetMgr<SamplerRes>>().unwrap();

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
    let sampler = if let Some(sampler) = asset_samplers.get(&samplerdesc) {
        sampler
    } else {
        if let Ok(sampler) = asset_samplers.insert(samplerdesc.clone(), SamplerRes::new(&device, &samplerdesc)) {
            sampler
        } else {
            // log::warn!("sampler Err");
            return;
        }
    };
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().record_sampler(samplerdesc, sampler.clone());
    }

    ActionSpine::spine_use_texture(&mut cmds.0, id_renderer, texture.data().clone(), sampler)
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