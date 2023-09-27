
use std::mem::transmute;

use bevy_ecs::{prelude::Commands, system::CommandQueue, query::QueryState};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, component::GraphId};
pub use pi_export_base::{export::Engine, constants::*, asset::TextureDefaultView};
use pi_window_renderer::{WindowRenderer, PluginWindowRender};
use pi_hash::XHashMap;
use pi_render::{renderer::sampler::SamplerRes, asset::TAssetKeyU64};

pub use pi_export_base::constants::BlendFactor;
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
pub fn init_spine_context(
    engine: &mut Engine,
) {
    engine
        .add_plugin(PluginSpineRenderer);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_renderer_create(app: &mut Engine, name: String, width: Option<f64>, height: Option<f64>) -> f64 {
    let rendersize: Option<(u32, u32)> =  match (width, height) {
		(Some(w), Some(h)) => Some((w as u32, h as u32)),
		_ => None,
    };
    log::warn!("Spine To Screen: {:?}", rendersize.is_none());

    let id_renderer = {
        let id = app.world.spawn_empty().id();
        let id_renderer = KeySpineRenderer(id);
    
        let final_render_format = app.world.get_resource::<WindowRenderer>().unwrap().format();
        let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
        ActionSpine::create_spine_renderer(id_renderer, rendersize, &mut ctx, final_render_format);
        
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
                log::warn!("Spine render_graph Ok");
            },
            Err(e) => {
                log::warn!("Spine render_graph Err {:?}", e);
            },
        }
    
    
        unsafe { transmute(id_renderer.0.to_bits()) }
    
    };

    f
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_renderer_dispose(app: &mut Engine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    
    let mut nodequery: QueryState<&'static GraphId> = app.world.query();
    
    if let Ok(nodeid) = nodequery.get(&app.world, id_renderer.0).cloned() {
        let mut render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
        render_graph.remove_node(nodeid.0);
    }

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &app.world);

    commands.entity(id_renderer.0).despawn();
    queue.apply(&mut app.world);

    let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    ActionSpine::dispose_spine_renderer(id_renderer, &mut ctx);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_texture_load(app: &mut Engine, key: &Atom) {
    let mut loader = app.world.get_resource_mut::<SpineTextureLoad>().unwrap();

    loader.load(pi_atom::Atom::from(key.as_str()));
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
    let loader = app.world.get_resource_mut::<SpineTextureLoad>().unwrap();
    while let Some((key, res)) = loader.success.pop() {
        record.success.insert(key, TextureDefaultView::from(res));
    }
    while let Some((key, res)) = loader.fail.pop() {
        record.fail.insert(key, res);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
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
#[pi_js_export]
pub fn spine_remove_texture(app: &mut Engine, id_renderer: f64, key: &Atom) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().remove_texture(key.asset_u64());
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
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
#[pi_js_export]
pub fn spine_shader(app: &mut Engine, id_renderer: f64, shader: KeySpineShader) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_shader(&mut cmds, id_renderer, unsafe {transmute(shader)});
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_blend(app: &mut Engine, id_renderer: f64, val: bool) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_blend(&mut cmds, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_blend_mode(app: &mut Engine, id_renderer: f64, src: BlendFactor, dst: BlendFactor) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_blend_mode(&mut cmds, id_renderer, src.val().val(), dst.val().val());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_uniform(app: &mut Engine, id_renderer: f64, val: &[f32]) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_uniform(&mut cmds, id_renderer, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_draw(app: &mut Engine, id_renderer: f64, vertices: &[f32], indices: &[u16], vlen: u32, ilen: u32) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_draw(&mut cmds, id_renderer, vertices, indices, vlen, ilen);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spine_reset(app: &mut Engine, id_renderer: f64) {
    use pi_spine_rs::ActionListSpine;

    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<ActionListSpine>().unwrap();
    ActionSpine::spine_reset(&mut cmds, id_renderer);
}