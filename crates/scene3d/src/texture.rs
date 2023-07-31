use pi_assets::asset::Handle;
use pi_engine_shell::prelude::{ImageTexture, KeyImageTexture};
use pi_export_base::export::Engine;

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