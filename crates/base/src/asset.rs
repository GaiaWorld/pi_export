use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_render::{rhi::asset::TextureRes, asset::TAssetKeyU64};
use js_proxy_gen_macro::pi_js_export;

pub use crate::export::{Atom, Engine};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct VertexBufferRes(Handle<pi_render::renderer::vertex_buffer::AssetVertexBuffer>);


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct TextureDefaultView(Handle<pi_render::rhi::asset::TextureRes>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl TextureDefaultView {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn width(&self) -> u32 {
        self.0.width
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn height(&self) -> u32 {
        self.0.height
    }
}

impl TextureDefaultView {
    pub fn data(&self) -> &Handle<pi_render::rhi::asset::TextureRes> {
        &self.0
    }
}
impl From<Handle<pi_render::rhi::asset::TextureRes>> for TextureDefaultView {
    fn from(value: Handle<pi_render::rhi::asset::TextureRes>) -> Self {
        Self(value)
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn query_texture_default_view(app: &mut Engine, path: &Atom) -> Option<TextureDefaultView> {
	crate::export::await_last_frame(app);
    if let Some(assets) = app.world.get_resource::<ShareAssetMgr<TextureRes>>() {
        if let Some(tex) = assets.get(&path.asset_u64()) {
            Some(TextureDefaultView(tex))
        } else {
            None
        }
    } else {
        None
    }
}