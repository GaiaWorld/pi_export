use std::ops::Deref;
use pi_assets::asset::{Handle, Size};
use pi_engine_shell::prelude::{ImageTexture, KeyImageTexture};
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
// use pi_render::asset::TAssetKeyU64;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

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
pub fn p3d_create_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data: &[u8], width: f64, height: f64, format: f64, size_per_pixel: f64) -> Option<DataTextureRes> {

    let format = EngineConstants::render_color_format(format).val();
    let size_per_pixel = size_per_pixel as u32;
    let width = width as u32;
    let height = height as u32;
    let dimension = wgpu::TextureViewDimension::D2;
    let is_opacity = true;
    let resource = param.resource.get_mut(&mut app.world);
    let key = KeyImageTexture { url: key.deref().clone(), srgb: false, file: false, compressed: false, depth_or_array_layers: 0, useage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING };
    if let Some(data) = resource.imgtex_asset.get(&key) {
        Some(DataTextureRes(data))
    } else {
        let texture = ImageTexture::create_data_texture(&resource.device, &resource.queue, &key, data, width, height, format, dimension, size_per_pixel, is_opacity);
        match resource.imgtex_asset.insert(key, texture) {
            Ok(data) => Some(DataTextureRes(data)),
            Err(_) => None,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_update_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, texture: &DataTextureRes, data:&[u8]) {
    let resource = param.resource.get_mut(&mut app.world);
    texture.0.update(&resource.queue, data, 0, 0, texture.0.width(), texture.0.height());
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_texture(app: &mut Engine, param: &mut ActionSetScene3D, isfile: bool, url: &Atom, srgb: bool, compressed: bool, depth_or_array_layers: f64, useage: f64, info: &mut [f32]) -> bool {
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