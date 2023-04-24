use bevy::prelude::App;
use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_render::{rhi::asset::TextureRes, asset::TAssetKeyU64};

use crate::export::{Atom, Engine};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct TextureDefaultView(Handle<pi_render::rhi::asset::TextureRes>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl TextureDefaultView {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn width(&self) -> u32 {
        self.0.width
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
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
#[cfg(feature = "pi_js_export")]
pub fn query_texture_default_view(app: &mut Engine, path: &Atom) -> Option<TextureDefaultView> {

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