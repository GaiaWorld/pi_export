
use std::mem::transmute;

use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
pub use pi_export_base::constants::*;

use crate::{as_dk, constants::EngineConstants, as_f64_dk};
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;
pub use crate::engine::ActionSetScene3D;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_render_target(app: &mut Engine, param: &mut ActionSetScene3D, color_format: f64, depth_stencil_format: f64, width: f64, height: f64, filter: f64, address: f64, anisotropy_clamp: f64) -> Option<f64> {
    pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    
    let filter = ContextConstants::filter_mode(filter);
    let address = EngineConstants::address_mode(address);
    let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
    let sampler = KeySampler {
        address_mode_u: address, address_mode_v: address, address_mode_w: address, 
        mag_filter: filter, min_filter: filter, mipmap_filter: filter,
        border_color: None, anisotropy_clamp, compare: None
    };
    let color_format = EngineConstants::render_color_format(color_format);
    let depth_stencil_format = EngineConstants::render_depth_format(depth_stencil_format);

    if let Some(key) = resource.render_targets.create(
        sampler, color_format, depth_stencil_format, width as u32, height as u32
    ) {
        Some(unsafe { transmute(key) })
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose_render_target(app: &mut Engine, param: &mut ActionSetScene3D, key: f64) {
    pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    
    resource.render_targets.delete(unsafe { transmute(key) });
}