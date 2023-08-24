use std::ops::Deref;
use pi_assets::asset::Handle;
use pi_engine_shell::prelude::{ImageTexture, KeyImageTexture};
use pi_export_base::{export::{Engine, Atom}, constants::*};
// use pi_render::asset::TAssetKeyU64;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

use crate::engine::ActionSetScene3D;

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
pub fn p3d_create_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data: &[u8], width: f64, height: f64, format: EDataTextureFormat) -> Option<DataTextureRes> {
    let (format, size_per_pixel) = match format {
        EDataTextureFormat::RGBA8 => (wgpu::TextureFormat::Rgba8Unorm, 4 * 1),
        EDataTextureFormat::R8 => (wgpu::TextureFormat::R8Unorm, 1 * 1),
        EDataTextureFormat::R16UI => (wgpu::TextureFormat::R16Uint, 1 * 2),
        EDataTextureFormat::R32UI => (wgpu::TextureFormat::R32Uint, 1 * 4),
        EDataTextureFormat::R8_SNORM => (wgpu::TextureFormat::R8Unorm, 1 * 1),
        EDataTextureFormat::R16F => (wgpu::TextureFormat::R16Float, 1 * 2),
        EDataTextureFormat::R16I => (wgpu::TextureFormat::R16Sint, 1 * 2),
    };
    let width = width as u32;
    let height = height as u32;
    let dimension = wgpu::TextureViewDimension::D2;
    let is_opacity = true;
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let key = KeyImageTexture::Data(key.deref().clone(), false);
    if let Some(data) = cmds.imgtex_asset.get(&key) {
        Some(DataTextureRes(data))
    } else {
        let texture = ImageTexture::create_data_texture(&cmds.device, &cmds.queue, &key, data, width, height, format, dimension, size_per_pixel, is_opacity);
        match cmds.imgtex_asset.insert(key, texture) {
            Ok(data) => Some(DataTextureRes(data)),
            Err(_) => None,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_update_data_texture(app: &mut Engine, param: &mut ActionSetScene3D, texture: &DataTextureRes, data:&[u8]) {
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    texture.0.update(&cmds.queue, data, 0, 0, texture.0.width(), texture.0.height());
}