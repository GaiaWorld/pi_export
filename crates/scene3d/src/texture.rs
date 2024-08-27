use std::ops::Deref;
use pi_assets::asset::{Handle, Size};
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderDevice, PiRenderQueue};
use pi_export_base::export::{DataTextureRecord, DataTextureRefs};
use pi_hash::{XHashMap, XHashSet};
use pi_scene_context::pass::{Resource, WorldResourceTemp};
use pi_scene_shell::prelude::{ImageTexture, KeyImageTexture, Res, ResMut};
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
// use pi_render::asset::TAssetKeyU64;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;
use wgpu::TextureFormat;

use crate::constants::EngineConstants;
pub use crate::engine::ActionSetScene3D;

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub struct P3DImageTexture(Handle<ImageTexture>);

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_texture_load(app: &mut Engine, param: &mut ActionSetScene3D, key: String, srgb: bool) {
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     let key = KeyImageTexture::File(key.clone(), srgb);
//     let image_assets_mgr = &cmds.imgtex_asset;
//     let device = &cmds.device;
//     let queue = &cmds.queue;
//     cmds.imgtex_loader.load_imgtex(&key, image_assets_mgr, queue, device);
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_texture_success(app: &mut Engine, param: &mut ActionSetScene3D, key: String, srgb: bool) -> Option<P3DImageTexture> {
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     let key = KeyImageTexture::File(key.clone(), srgb);
//     let image_assets_mgr = &cmds.imgtex_asset;
//     match cmds.imgtex_loader.query_imgtex(&key, image_assets_mgr) {
//         Ok(val) => Some(P3DImageTexture(val)),
//         Err(failed) => {
//             None
//         },
//     }
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_texture_failed(app: &mut Engine, param: &mut ActionSetScene3D, key: String, srgb: bool) -> bool {
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     let key = KeyImageTexture::File(key.clone(), srgb);
//     cmds.imgtex_loader.query_failed(&key)
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_reset_texture_loader_failed(app: &mut Engine, param: &mut ActionSetScene3D) {
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     let key = KeyImageTexture::File(key.clone(), srgb);
//     cmds.imgtex_loader.reset_failed();
// }


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct DataTextureRes(Handle<ImageTexture>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data: &[u8], width: f64, height: f64, format: f64, size_per_pixel: f64) {

	pi_export_base::export::await_last_frame(app);

    let key = key.deref().clone();
    let format = EngineConstants::render_color_format(format).val();
    let size_per_pixel = size_per_pixel as u32;
    let width = width as u32;
    let height = height as u32;
    let dimension = wgpu::TextureViewDimension::D2;
    let is_opacity = true;
    let texkey = KeyImageTexture { url: key.clone(), srgb: false, file: false, compressed: false, depth_or_array_layers: 0, useage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING };

    let refs = app.world.get_resource_mut::<DataTextureRefs>().unwrap();
    refs.data.insert(key.clone(), data.to_vec());
    let record = app.world.get_resource_mut::<DataTextureRecord>().unwrap();
    record.creation.insert(key.clone(), (width, height, size_per_pixel, format, dimension, texkey));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_update_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data:&[u8]) {
	pi_export_base::export::await_last_frame(app);

    let refs = app.world.get_resource_mut::<DataTextureRefs>().unwrap();
    refs.data.insert(key.deref().clone(), data.to_vec());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_remove_data_texture(app: &mut Engine, key: &Atom) {
	pi_export_base::export::await_last_frame(app);

    let record = app.world.get_resource_mut::<DataTextureRecord>().unwrap();
    record.record.remove(key.deref());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_texture(app: &mut Engine, param: &mut ActionSetScene3D, isfile: bool, url: &Atom, srgb: bool, compressed: bool, depth_or_array_layers: f64, useage: f64, info: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);

    let resource = param.resource.get_mut(&mut app.world);

    let useage = EngineConstants::texture_usage(useage);
    let key = KeyImageTexture { url: url.deref().clone(), srgb, file: isfile, compressed, depth_or_array_layers: depth_or_array_layers as u8, useage };
    if let Some(img) = resource.imgtex_asset.get(&key) {
        info[0] = img.width() as f32;
        info[1] = img.height() as f32;
        info[2] = img.size() as f32;
        true
    } else {
        false
    }
}