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

pub type ActionListCustomBuffer = pi_scene_shell::prelude::ActionList<(KeyVertexBuffer, Vec<u8>, bool, bool)>;

pub fn sys_custom_buffer(
    mut actions: ResMut<ActionListCustomBuffer>,
    queue: Res<PiRenderQueue>,
    mut vb_wait: ResMut<pi_scene_shell::prelude::VertexBufferDataMap3D>,
    vb_mgr: Res<pi_scene_shell::prelude::ShareAssetMgr<pi_scene_shell::prelude::EVertexBufferRange>>,
) {
    actions.drain().for_each(|(key, data, isindices, isu32)| {

		let key_u64 = key.asset_u64();
        if isindices {
            if isu32 {
                if let Some(buffer) = vb_mgr.get(&key_u64) {
                    queue.write_buffer(buffer.buffer(), 0, &data);
                } else {
                    pi_scene_context::prelude::ActionVertexBuffer::create_indices(&mut vb_wait, key, data);
                }
            } else {
                if let Some(buffer) = vb_mgr.get(&key_u64) {
                    queue.write_buffer(buffer.buffer(), 0, &data);
                } else {
                    pi_scene_context::prelude::ActionVertexBuffer::create_indices(&mut vb_wait, key, data);
                }
            }
        } else {
            if let Some(buffer) = vb_mgr.get(&key_u64) {
                queue.write_buffer(buffer.buffer(), 0, &data);
            } else {
                pi_scene_context::prelude::ActionVertexBuffer::create(&mut vb_wait, key, data);
            }
        }
    });
}