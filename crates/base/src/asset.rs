use pi_assets::asset::Handle;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_render::{asset::TAssetKeyU64, components::view::target_alloc::{TargetDescriptor, TextureDescriptor}, rhi::asset::TextureRes};
use js_proxy_gen_macro::pi_js_export;
use pi_scene_context::pass::KeyVertexBuffer;
use pi_scene_shell::prelude::{ETextureViewUsage, ImageTextureView, Res, ResMut, PiRenderDefault};
use pi_share::Share;

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
pub fn p3d_texture_default_view_width(item: &TextureDefaultView) -> f64 {
    match &item.0 {
        ETextureViewUsage::Tex(tex) => tex.width as f64,
        ETextureViewUsage::TexWithId(_) => todo!(),
        ETextureViewUsage::Image(tex) => tex.texture().width() as f64,
        ETextureViewUsage::ImageFrame(tex) => tex.texture().width() as f64,
        _ => todo!(),
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_default_view_height(item: &TextureDefaultView) -> f64 {
    match &item.0 {
        ETextureViewUsage::Tex(tex) => tex.height as f64,
        ETextureViewUsage::TexWithId(_) => todo!(),
        ETextureViewUsage::Image(tex) => tex.texture().height() as f64,
        ETextureViewUsage::ImageFrame(tex) => tex.texture().height() as f64,
        _ => todo!(),
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

pub fn sys_screen_with_postprocess(
    atlas: Res<pi_bevy_render_plugin::PiSafeAtlasAllocator>,
    mut screenwithpostprocess: ResMut<pi_bevy_render_plugin::ScreenWithPostprocess>,
    window: Res<pi_bevy_render_plugin::PiRenderWindow>,
) {
    let mut screenfbo = None;
    let width = window.width;
    let height = window.height;
    let format = wgpu::TextureFormat::pi_render_default();
    if screenwithpostprocess.0  {
        if let Some(rt) = &screenwithpostprocess.1 {
            let w = rt.rect().width() as u32;
            let h = rt.rect().height() as u32;
            if w == width && h == height {
                screenfbo = Some(rt.clone());
            }
        }
        if screenfbo.is_none() {
            let target_type = atlas.create_type(TargetDescriptor {
                colors_descriptor: smallvec::SmallVec::from_slice(
                    &[TextureDescriptor {
                        mip_level_count: 1,
                        sample_count: 1,
                        dimension: wgpu::TextureDimension::D2,
                        format,
                        usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
                        base_mip_level: 0,
                        base_array_layer: 0,
                        array_layer_count: None,
                        view_dimension: Some(wgpu::TextureViewDimension::D2),
                    }]
                ),
                need_depth: false,
                depth_descriptor: None,
                default_width: width,
                default_height: height,
            });
            let t: Vec<Share<pi_render::components::view::target_alloc::SafeTargetView>> = vec![];
            let rt = atlas.allocate_alone_not_share(width, height, target_type, t.iter(), true);
            screenfbo = Some(Share::new(rt));
        }
    }
 
    screenwithpostprocess.1 = screenfbo;
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn onload_res_data(path: &Atom, data: &[u8]) {
    // println!("======== onload_res_data: {:?}", path);
	pi_hal::texture::insert_res_cache(*&path, data);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn delete_res_data(path: &Atom) {
	pi_hal::texture::remove_res_cache(*&path);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn have_res_data(path: &Atom) -> bool {
	let r = pi_hal::texture::have_cache(*&path);
    // println!("======== have_res_data: {:?}", (r, path));
    r
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn have_res_data_of_str(path: &str) -> bool {
    let r = pi_hal::texture::have_cache(&&Atom::from_string(path.to_string()));
    // println!("======== have_res_data_of_str: {:?}", (r, Atom::from_string(path.to_string())));
	r
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn res_data_len(path: &Atom) ->String {
    let mut res = Vec::new();
    let map = pi_hal::texture::RES_MAP.read().unwrap();
	let _ = map.iter().map(|(k, v)|res.push((k.as_str(), v.len())));
    format!("{:?}", res)
}