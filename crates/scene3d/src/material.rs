
use std::{ops::Deref, mem::transmute};
use pi_scene_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
use pi_scene_context::prelude::*;

pub use pi_export_base::{export::{Engine, Atom}, constants::* };

use crate::{constants::EngineConstants, as_dk};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;




#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material(app: &mut Engine) -> f64 {
    let id: Entity = CommandsExchangeD3::p3d_entity(app);

    let result = as_f64(&id);

    result
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_shader(cmds: &mut CommandsExchangeD3, mat: f64, shader: &Atom, usematarray: bool) {
    let mat: Entity = as_entity(mat);
    // log::warn!("Create Material ShaderName: {:?}", shader.as_str());    // log::warn!("MaterialInit: {:?}, {}", entity, mat);

    CommandsExchangeD3::p3d_material_shader(cmds, mat, shader.deref(), usematarray);
    // cmds.material_create.push(OpsMaterialCreate::ops(mat, shader.as_str(), usematarray));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_apply(cmds: &mut CommandsExchangeD3, mat: f64, mesh: f64, pass: f64) {
    let mat: Entity = as_entity(mat);
    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_material_apply(cmds, mat, mesh, pass);
    // cmds.material_usemat.push(OpsMaterialUse::ops(mesh, mat, pass));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_mat4(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
    let mat: Entity = as_entity(mat);
    let val = [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32];
    
    CommandsExchangeD3::p3d_material_uniform_mat(cmds, mat, key.deref(), val);
    // cmds.material_valb.push( OpsUniformValB::mat4(mat, key.deref().clone(), val) );
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
    let mat: Entity = as_entity(mat);    
    
    let val = EUniformVal::Vec2( key.deref().clone(), x as f32, y as f32);
    CommandsExchangeD3::p3d_material_uniform_value(cmds, mat, val);
    // cmds.material_val.push( OpsUniformVal::vec2(mat, key.deref().clone(), x as f32, y as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_vec4(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, x: f64, y: f64, z: f64, w: f64) {
    let mat: Entity = as_entity(mat);    
    
    let val = EUniformVal::Vec4(key.deref().clone(), x as f32, y as f32, z as f32, w as f32);
    CommandsExchangeD3::p3d_material_uniform_value(cmds, mat, val);
    // cmds.material_val.push( OpsUniformVal::vec4(mat, key.deref().clone(), x as f32, y as f32, z as f32, w as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_float(cmds: &mut CommandsExchangeD3, mat: f64,  key: &Atom, val: f64) {
    let mat: Entity = as_entity(mat);    
    
    let val = EUniformVal::Float( key.deref().clone(), val as f32);
    CommandsExchangeD3::p3d_material_uniform_value(cmds, mat, val);
    // cmds.material_val.push( OpsUniformVal::float(mat, key.deref().clone(), val as f32) );
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
    let mat: Entity = as_entity(mat);   
    
    let val = EUniformVal::Uint( key.deref().clone(), val as u32);
    CommandsExchangeD3::p3d_material_uniform_value(cmds, mat, val); 
    // cmds.material_val.push( OpsUniformVal::uint(mat, key.deref().clone(), val as u32) );
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
    isfile: bool,
    cancombine: bool,
    compare: Option<f64>,
) {
    let mat: Entity = as_entity(mat);  

    CommandsExchangeD3::p3d_material_uniform_tex(cmds, mat, key.deref(), url.deref(),
        srgb,
        compressed,
        filter,
        address_mode_u,
        address_mode_v,
        address_mode_w,
        mag_filter,
        min_filter,
        mipmap_filter,
        anisotropy_clamp,
        border_color,
        isfile,
        cancombine,
        compare
    );
    
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_load_texture(
    cmds: &mut CommandsExchangeD3,
    url: &Atom,
    compressed: bool,
    isfile: bool,
    cancombine: bool,
) {
    let key = KeyImageTextureFrame { url: pi_atom::Atom::from(url.to_string()), cancombine, file: isfile, compressed };
    cmds.loadtextures.push(key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_tex_from_render_target(
    cmds: &mut CommandsExchangeD3, mat: f64, key: &Atom, key_tilloff: &Atom, url: f64,
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
    
    let mat: Entity = as_entity(mat);  

    CommandsExchangeD3::p3d_material_uniform_tex_from_render_target(cmds, mat, key.deref(), key_tilloff.deref(), url,
        filter,
        address_mode_u,
        address_mode_v,
        address_mode_w,
        mag_filter,
        min_filter,
        mipmap_filter,
        anisotropy_clamp,
        border_color,
        compare
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_tex_from_renderer(
    cmds: &mut CommandsExchangeD3, mat: f64, key: &Atom, key_tilloff: &Atom, url: f64,
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
    let mat: Entity = as_entity(mat);  
    let url = as_entity(url);

    CommandsExchangeD3::p3d_material_uniform_tex_from_renderer(cmds, mat, key.deref(), key_tilloff.deref(), url,
        filter,
        address_mode_u,
        address_mode_v,
        address_mode_w,
        mag_filter,
        min_filter,
        mipmap_filter,
        anisotropy_clamp,
        border_color,
        compare
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_uniform_target_animation(
    cmds: &mut CommandsExchangeD3,
    mat: f64,
    group: f64,
    key: &Atom,
    curve_key: f64,
) {
    let target = as_entity(mat);
    let group = as_entity(group);
    // let curve: u64 = unsafe { transmute(curve_key) };

    CommandsExchangeD3::p3d_uniform_target_animation(cmds, target, group, key.deref(), curve_key);
    // cmds.material_valb.push(OpsUniformValB::targetanim(target, key.deref().clone(), group, curve));
}
