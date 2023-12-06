
use std::ops::Deref;
use pi_engine_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
use pi_scene_context::prelude::*;

pub use pi_export_base::{export::{Engine, Atom}, constants::* };

use crate::{constants::EngineConstants, as_dk};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;




#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material(app: &mut Engine) -> f64 {
    let id: Entity = app.world.spawn_empty().id();

    let result = as_f64(&id);

    result
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_shader(cmds: &mut CommandsExchangeD3, mat: f64, shader: &Atom) {
    let entity: Entity = as_entity(mat);
    // log::warn!("Create Material ShaderName: {:?}", shader.as_str());    // log::warn!("MaterialInit: {:?}, {}", entity, mat);
    cmds.material_create.push(OpsMaterialCreate::ops(entity, shader.as_str()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_apply(cmds: &mut CommandsExchangeD3, mat: f64, mesh: f64, pass: f64) {
    let mat: Entity = as_entity(mat);
    let mesh: Entity = as_entity(mesh);
    let pass = EngineConstants::passtag(pass);

    cmds.material_usemat.push(OpsMaterialUse::ops(mesh, mat, pass));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_mat4(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
    let mat: Entity = as_entity(mat);
    let val = [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32];
    cmds.material_mat4.push( OpsUniformMat4::ops(mat, key.deref().clone(), val) );
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_material_uniform_mat2(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, m11: f64, m12: f64, m21: f64, m22: f64) {
//     let mat: Entity = as_entity(mat);
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     cmds.matcmd_mat2.push( OpsUniformMat2::ops(mat, key.deref().clone(), [m11 as f32, m12 as f32, m21 as f32, m22 as f32]) );
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_vec2(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, x: f64, y: f64) {
    let mat: Entity = as_entity(mat);    cmds.material_vec2.push( OpsUniformVec2::ops(mat, key.deref().clone(), x as f32, y as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_vec4(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, x: f64, y: f64, z: f64, w: f64) {
    let mat: Entity = as_entity(mat);    cmds.material_vec4.push( OpsUniformVec4::ops(mat, key.deref().clone(), x as f32, y as f32, z as f32, w as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_float(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, val: f64) {
    let mat: Entity = as_entity(mat);    cmds.material_float.push( OpsUniformFloat::ops(mat, key.deref().clone(), val as f32) );
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_material_uniform_int(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, val: f64) {
//     let mat: Entity = as_entity(mat);
//     cmds.material_int.push( OpsUniformInt::ops(mat, key.deref().clone(), val as i32) );
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_uint(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, val: f64) {
    let mat: Entity = as_entity(mat);    cmds.material_uint.push( OpsUniformUint::ops(mat, key.deref().clone(), val as u32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_tex(
    cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom,
    url: &Atom,
    srgb: bool,
    compressed: bool,
    filter: bool,
    address_mode_u: f64,
    address_mode_v: f64,
    address_mode_w: f64,
    mag_filter: f64,
    min_filter: f64,
    mipmap_filter: f64,
    anisotropy_clamp: f64,
    border_color: f64,
    compare: Option<f64>,
) {
    let address_mode_u = EngineConstants::address_mode(address_mode_u);
    let address_mode_v = EngineConstants::address_mode(address_mode_v);
    let address_mode_w = EngineConstants::address_mode(address_mode_w);
    let mag_filter = ContextConstants::filter_mode(mag_filter);
    let min_filter = ContextConstants::filter_mode(min_filter);
    let mipmap_filter = ContextConstants::filter_mode(mipmap_filter);
    let compare = if let Some(compare) = compare { Some(ContextConstants::compare_function(compare).val2()) } else { None };
    let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
    let border_color = EngineConstants::border_color(border_color);
    let mat: Entity = as_entity(mat);    cmds.material_texture.push(
        OpsUniformTexture::ops(
            mat,
            UniformTextureWithSamplerParam {
                slotname: key.deref().clone(),
                filter,
                sample: pi_export_base::constants::sampler_desc(address_mode_u,
                    address_mode_v,
                    address_mode_w,
                    mag_filter,
                    min_filter,
                    mipmap_filter,
                    compare,
                    anisotropy_clamp,
                    border_color,
                ),
                url: EKeyTexture::Image(KeyImageTextureView::new(
                    KeyImageTexture { url: pi_atom::Atom::from(url.to_string()), srgb, file: true, compressed, depth_or_array_layers: 0, useage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST },
                    TextureViewDesc {
                        aspect: wgpu::TextureAspect::All,
                        base_mip_level: 0,
                        mip_level_count: None,
                        base_array_layer: 0,
                        array_layer_count: None,
                    }
                )),
            }
        )
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_tex_from_render_target(
    cmds: &mut CommandsExchangeD3, mat: f64, key: &Atom, key_tilloff: &Atom, url: f64,
) {
    let texparam = UniformTextureWithSamplerParam { slotname: key.deref().clone(), ..Default::default() };

    // let address_mode_u = EngineConstants::address_mode(address_mode_u);
    // let address_mode_v = EngineConstants::address_mode(address_mode_v);
    // let address_mode_w = EngineConstants::address_mode(address_mode_w);
    // let mag_filter = ContextConstants::filter_mode(mag_filter);
    // let min_filter = ContextConstants::filter_mode(min_filter);
    // let mipmap_filter = ContextConstants::filter_mode(mipmap_filter);
    // let compare = ContextConstants::compare_function(compare);
    // let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
    // let border_color = EngineConstants::border_color(border_color);
    let mat: Entity = as_entity(mat);
    let key = as_dk(&url);    cmds.material_texturefromtarget.push(OpsUniformTextureFromRenderTarget::ops(mat, texparam, key, key_tilloff.deref().clone()));
}
