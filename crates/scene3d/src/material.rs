
use std::ops::Deref;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use pi_export_base::{export::{Engine, Atom}, constants::{EAddressMode, EFilterMode, EAnisotropyClamp, CompareFunction, SamplerBorderColor}};

use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum OpsPass {
    ShadowCast,
    Opaque,
    Sky,
    Water,
    AlphaTest,
    Transparent,
    OpaqueExtend,
    TransparentExtend,
}
impl OpsPass {
    pub fn val(&self) -> EPassTag {
        match self {
            OpsPass::ShadowCast => EPassTag::ShadowCast,
            OpsPass::Opaque => EPassTag::Opaque,
            OpsPass::Sky => EPassTag::Sky,
            OpsPass::Water => EPassTag::Water,
            OpsPass::AlphaTest => EPassTag::AlphaTest,
            OpsPass::Transparent => EPassTag::Transparent,
            OpsPass::OpaqueExtend => EPassTag::OpaqueExtend,
            OpsPass::TransparentExtend => EPassTag::TransparentExtend,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material(app: &mut Engine, param: &mut ActionSetScene3D) -> f64 {
    let id: Entity = app.world.spawn_empty().id();

    let result = as_f64(&id);

    result
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_shader(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64, shader: &Atom, pass: OpsPass) {
    let entity: Entity = as_entity(mat);
    // log::warn!("Create Material ShaderName: {:?}", shader.as_str());
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    // log::warn!("MaterialInit: {:?}, {}", entity, mat);
    cmds.matcmd.create.push(OpsMaterialCreate::ops(entity, shader.as_str(), pass.val()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_apply(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64, mesh: f64) {
    let mat: Entity = as_entity(mat);
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.matcmd.usemat.push(OpsMaterialUse::ops(mesh, mat));
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_material_uniform_mat4(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
//     let mat: Entity = as_entity(mat);
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     let val = [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32];
//     cmds.matcmd.mat4.push( OpsUniformMat4::ops(mat, key.deref().clone(), val) );
// }
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_material_uniform_mat2(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, m11: f64, m12: f64, m21: f64, m22: f64) {
//     let mat: Entity = as_entity(mat);
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     cmds.matcmd.mat2.push( OpsUniformMat2::ops(mat, key.deref().clone(), [m11 as f32, m12 as f32, m21 as f32, m22 as f32]) );
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_vec2(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, x: f64, y: f64) {
    let mat: Entity = as_entity(mat);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.matcmd.vec2.push( OpsUniformVec2::ops(mat, key.deref().clone(), x as f32, y as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_vec4(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, x: f64, y: f64, z: f64, w: f64) {
    let mat: Entity = as_entity(mat);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.matcmd.vec4.push( OpsUniformVec4::ops(mat, key.deref().clone(), x as f32, y as f32, z as f32, w as f32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_float(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, val: f64) {
    let mat: Entity = as_entity(mat);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.matcmd.float.push( OpsUniformFloat::ops(mat, key.deref().clone(), val as f32) );
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_material_uniform_int(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, val: f64) {
//     let mat: Entity = as_entity(mat);
//     let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
//     cmds.matcmd.int.push( OpsUniformInt::ops(mat, key.deref().clone(), val as i32) );
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_uint(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom, val: f64) {
    let mat: Entity = as_entity(mat);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.matcmd.uint.push( OpsUniformUint::ops(mat, key.deref().clone(), val as u32) );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_uniform_tex(
    app: &mut Engine, param: &mut ActionSetScene3D, mat: f64,  key: &Atom,
    url: &Atom,
    srgb: bool,
    filter: bool,
    address_mode_u: EAddressMode,
    address_mode_v: EAddressMode,
    address_mode_w: EAddressMode,
    mag_filter: EFilterMode,
    min_filter: EFilterMode,
    mipmap_filter: EFilterMode,
    compare: CompareFunction,
    anisotropy_clamp: EAnisotropyClamp,
    border_color: SamplerBorderColor,
) {
    let mat: Entity = as_entity(mat);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.matcmd.texture.push(
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
                    KeyImageTexture::File(pi_atom::Atom::from(url.to_string()), srgb),
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
