
use std::{num::NonZeroU32, mem::transmute};

use bevy::{prelude::{Deref, DerefMut, App, Commands, Resource}, ecs::system::CommandQueue};
use pi_atom::Atom;
use pi_assets::{mgr::AssetMgr, asset::Handle};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, PiRenderQueue, GraphError, component::GraphId};
use pi_export_base::{export::Engine, constants::*, asset::TextureDefaultView};
use pi_window_renderer::{WindowRenderer, PluginWindowRender};
use pi_hal::image::{load_from_url, DynamicImage};
use pi_hash::XHashMap;
use pi_render::{rhi::{asset::TextureRes, device::RenderDevice, RenderQueue}, renderer::sampler::SamplerRes, asset::TAssetKeyU64};

use pi_export_base::constants::{ColorFormat, BlendFactor};
use pi_spine_rs::{
    shaders::{KeySpineShader},
    KeySpineRenderer,
    SpineRenderContext,
    ActionListSpine,
    SAMPLER_DESC,
    SpineRenderNode,
    ActionSpine,
    PluginSpineRenderer,
    SpineTextureLoad
};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct SpineTextureSize(pub u32, pub u32);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl SpineTextureSize {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn new(w: u32, h: u32) -> Self {
        Self(w, h)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn width(&self) -> u32 {
        self.0
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn height(&self) -> u32 {
        self.1
    }
}
pub fn render_size(val: &Option<SpineTextureSize>) -> Option<(u32, u32)> {
    match val {
        Some(val) => Some((val.0, val.1)),
        None => None,
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn init_spine_context(
    engine: &mut Engine,
) {
    engine
        .add_plugin(PluginWindowRender)
        .add_plugin(PluginSpineRenderer);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_renderer_create(app: &mut Engine, name: String, width: Option<f64>, height: Option<f64>) -> f64 {

    let rendersize =  if let (Some(width), Some(height)) = (width, height) {
        Some(SpineTextureSize(width as u32, height as u32))
    } else {
        None
    };
    log::warn!("To Screen: {:?}", rendersize.is_none());

    let id_renderer = {
        let id = app.world.spawn_empty().id();
        let id_renderer = KeySpineRenderer(id);
    
        let final_render_format = app.world.get_resource::<WindowRenderer>().unwrap().format();
        let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
        ActionSpine::create_spine_renderer(id_renderer, render_size(&rendersize), &mut ctx, final_render_format);
        
        id_renderer
    };

    let f = {

        let mut render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
        match ActionSpine::spine_renderer_apply(id_renderer, pi_atom::Atom::from(name), rendersize.is_none(), &mut render_graph) {
            Ok(nodeid) => {
                // let mut actions = app.world.get_resource_mut::<ActionListSpine>().unwrap();
                // let mut queue = CommandQueue::default();
                // let mut commands = Commands::new(&mut queue, &app.world);
                // // log::warn!("SpineRenderer Add GraphId()");
                // commands.entity(id_renderer.0).insert(GraphId(nodeid));
                // queue.apply(&mut app.world);
                app.world.entity_mut(id_renderer.0).insert(GraphId(nodeid));
                // actions.push(ESpineCommand::Graph(id_renderer, nodeid));
                log::warn!("render_graph Ok");
            },
            Err(e) => {
                log::warn!("render_graph Err {:?}", e);
            },
        }
    
    
        unsafe { transmute(id_renderer.0.to_bits()) }
    
    };

    f
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_renderer_dispose(app: &mut Engine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &app.world);
    commands.entity(id_renderer.0).despawn();
    queue.apply(&mut app.world);

    let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    ActionSpine::dispose_spine_renderer(id_renderer, &mut ctx);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_texture_load(app: &mut Engine, key: &pi_export_base::export::Atom) {
    let mut loader = app.world.get_resource_mut::<SpineTextureLoad>().unwrap();

    loader.load(pi_atom::Atom::from(key.as_str()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct SpineTextureLoadRecord {
    success: XHashMap<Atom, TextureDefaultView>,
    fail: XHashMap<Atom, String>,
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl SpineTextureLoadRecord {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn new() -> Self {
        Self { success: XHashMap::default(), fail: XHashMap::default() }
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn success(&mut self, key: &pi_export_base::export::Atom) -> Option<TextureDefaultView> {
        self.success.remove(key)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn fail(&mut self, key: &pi_export_base::export::Atom) -> Option<String> {
        self.fail.remove(key)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn clear(&mut self) {
        self.fail.clear();
        self.success.clear();
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_texture_loaded_query(app: &mut Engine, record: &mut SpineTextureLoadRecord) {
    let loader = app.world.get_resource_mut::<SpineTextureLoad>().unwrap();
    while let Some((key, res)) = loader.success.pop() {
        record.success.insert(key, TextureDefaultView::from(res));
    }
    while let Some((key, res)) = loader.fail.pop() {
        record.fail.insert(key, res);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_texture_record(app: &mut Engine, id_renderer: f64, texture: &TextureDefaultView) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        let key_u64 = texture.data().key();
        renderer.render_mut().record_texture(*key_u64, texture.data().clone());
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_remove_texture(app: &mut Engine, id_renderer: f64, key: &pi_export_base::export::Atom) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().remove_texture(key.asset_u64());
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_use_texture(app: &mut Engine, id_renderer: f64, texture: &TextureDefaultView, 
    address_mode_u: EAddressMode,
    address_mode_v: EAddressMode,
    address_mode_w: EAddressMode,
    mag_filter: EFilterMode,
    min_filter: EFilterMode,
    mipmap_filter: EFilterMode,
    compare: CompareFunction,
    anisotropy_clamp: EAnisotropyClamp,
    border_color: SamplerBorderColor,
) {

    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let device = app.world.get_resource::<PiRenderDevice>().unwrap().clone();
    let asset_samplers = app.world.get_resource::<ShareAssetMgr<SamplerRes>>().unwrap();

    let samplerdesc = sampler_desc(
        address_mode_u,
        address_mode_v,
        address_mode_w,
        mag_filter,
        min_filter,
        mipmap_filter,
        compare,
        anisotropy_clamp,
        border_color,
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

    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_use_texture(&mut cmds, id_renderer, texture.data().clone(), sampler)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_shader(app: &mut Engine, id_renderer: f64, shader: KeySpineShader) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_shader(&mut cmds, id_renderer, shader);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_blend(app: &mut Engine, id_renderer: f64, val: bool) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_blend(&mut cmds, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_blend_mode(app: &mut Engine, id_renderer: f64, src: BlendFactor, dst: BlendFactor) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_blend_mode(&mut cmds, id_renderer, src.val(), dst.val());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_uniform(app: &mut Engine, id_renderer: f64, val: &[f32]) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_uniform(&mut cmds, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_draw(app: &mut Engine, id_renderer: f64, vertices: &[f32], indices: &[u16], vlen: u32, ilen: u32) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_draw(&mut cmds, id_renderer, vertices, indices, vlen, ilen);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_reset(app: &mut Engine, id_renderer: f64) {
    use pi_spine_rs::ActionListSpine;

    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_reset(&mut cmds, id_renderer);
}