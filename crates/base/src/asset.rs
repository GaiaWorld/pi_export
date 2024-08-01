use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_render::{rhi::asset::TextureRes, asset::TAssetKeyU64};
use js_proxy_gen_macro::pi_js_export;
use pi_scene_context::pass::{ETextureViewUsage, ImageTextureView};

pub use crate::export::{Atom, Engine};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct VertexBufferRes(Handle<pi_render::renderer::vertex_buffer::AssetVertexBuffer>);


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct TextureDefaultView(pi_render::renderer::texture::ETextureViewUsage, u64);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl TextureDefaultView {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn width(&self) -> u32 {
        match &self.0 {
            ETextureViewUsage::Tex(tex) => tex.width,
            ETextureViewUsage::TexWithId(_) => todo!(),
            ETextureViewUsage::Image(tex) => tex.texture().width(),
            ETextureViewUsage::SRT(_) => todo!(),
        }
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn height(&self) -> u32 {
        match &self.0 {
            ETextureViewUsage::Tex(tex) => tex.height,
            ETextureViewUsage::TexWithId(_) => todo!(),
            ETextureViewUsage::Image(tex) => tex.texture().height(),
            ETextureViewUsage::SRT(_) => todo!(),
        }
    }
}

impl TextureDefaultView {
    pub fn data(&self) -> &pi_render::renderer::texture::ETextureViewUsage {
        &self.0
    }
    pub fn key(&self) -> u64 {
        self.1
    }
    pub fn new(data: pi_render::renderer::texture::ETextureViewUsage, key: u64) -> Self {
        Self(data, key)
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn query_texture_default_view(app: &mut Engine, key: u64) -> Option<TextureDefaultView> {
	crate::export::await_last_frame(app);
    if let Some(assets) = app.world.get_single_res_mut::<ShareAssetMgr<TextureRes>>() {
        if let Some(tex) = assets.get(&key) {
            Some(TextureDefaultView(ETextureViewUsage::Tex(tex), key))
        } else {
            if let Some(assets) = app.world.get_single_res_mut::<ShareAssetMgr<ImageTextureView>>() {
                if let Some(tex) = assets.get(&key) {
                    Some(TextureDefaultView(ETextureViewUsage::Image(tex), key))
                } else {
                    None
                }
            } else {
                None
            }
        }
    } else {
        None
    }
}