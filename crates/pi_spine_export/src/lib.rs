use std::mem::replace;

use bevy::prelude::{ResMut, App, Deref};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, PiRenderQueue};
use pi_final_render_target::FinalRenderTarget;
use pi_hash::XHashMap;
use pi_render::{rhi::{device::{RenderDevice as RenderDeviceO}, sampler::SamplerDesc, asset::TextureRes}, renderer::sampler::SamplerRes, asset::TAssetKeyU64};
use pi_render_base_export::constants::BlendFactor;
use pi_spine_rs::{renderer::{Renderer, RendererAsync}, shaders::{SingleSpineBindGroupLayout as SingleSpineBindGroupLayoutO, SingleSpinePipelinePool as SingleSpinePipelinePoolO, KeySpineShader}, KeySpineRenderer, SpineRenderContext, TInterfaceSpine, SingleSpineCommands};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Deref)]
#[wasm_bindgen]
pub struct SpineShell(pub(crate) App);

impl SpineShell {
	pub fn new(app: App) -> Self { Self(app) }
	pub fn app(&self) -> &App { &self.0 }
	pub fn app_mut(&mut self) -> &mut App { &mut self.0 }
    
}
impl TInterfaceSpine for SpineShell {}

#[wasm_bindgen]
pub fn spine_renderer(app: &mut SpineShell, name: String, next_render_node: Option<String>) -> KeySpineRenderer {
    let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    let render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
    let final_render = app.world.get_resource::<FinalRenderTarget>().unwrap();
    let next_render_node = if let Some(next_render_node) = next_render_node {
        Some(Atom::from(next_render_node))
    } else {
        None
    };
    SpineShell::create_spine_renderer(Atom::from(name), next_render_node, &mut ctx, &mut render_graph, &final_render)
}

#[wasm_bindgen]
pub fn spine_renderer_dispose(app: &mut SpineShell, id_renderer: KeySpineRenderer) {
    let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    SpineShell::dispose_spine_renderer(id_renderer, &mut ctx)
}

#[wasm_bindgen]
pub fn spine_texture(app: &mut SpineShell, id_renderer: KeySpineRenderer, key: String, img_data: &[u8], width: u32, height: u32) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    let device = app.world.get_resource::<PiRenderDevice>().unwrap();
    let queue = app.world.get_resource::<PiRenderQueue>().unwrap();
    let asset_textures = app.world.get_resource::<ShareAssetMgr<TextureRes>>().unwrap();
    let asset_samplers = app.world.get_resource::<ShareAssetMgr<SamplerRes>>().unwrap();
    SpineShell::spine_texture(&mut cmds.0, id_renderer, key.as_str(), img_data, width, height, device, queue, asset_textures, asset_samplers);
}

#[wasm_bindgen]
pub fn spine_use_texture(app: &mut SpineShell, id_renderer: KeySpineRenderer, key: String) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_use_texture(&mut cmds.0, id_renderer, key.asset_u64())
}

#[wasm_bindgen]
pub fn spine_shader(app: &mut SpineShell, id_renderer: KeySpineRenderer, shader: KeySpineShader) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_shader(&mut cmds.0, id_renderer, shader);
}

#[wasm_bindgen]
pub fn spine_blend(app: &mut SpineShell, id_renderer: KeySpineRenderer, val: bool) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_blend(&mut cmds.0, id_renderer, val);
}

#[wasm_bindgen]
pub fn spine_blend_mode(app: &mut SpineShell, id_renderer: KeySpineRenderer, src: BlendFactor, dst: BlendFactor) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_blend_mode(&mut cmds.0, id_renderer, src.val(), dst.val());
}

#[wasm_bindgen]
pub fn spine_uniform(app: &mut SpineShell, id_renderer: KeySpineRenderer, val: &[f32]) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_uniform(&mut cmds.0, id_renderer, val);
}

#[wasm_bindgen]
pub fn spine_draw(app: &mut SpineShell, id_renderer: KeySpineRenderer, vertices: &[f32], indices: &[u16], vlen: u32, ilen: u32) {
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    SpineShell::spine_draw(&mut cmds.0, id_renderer, vertices, indices, vlen, ilen);
}