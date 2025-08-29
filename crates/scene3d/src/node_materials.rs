

use std::ops::Deref;

use pi_assets::asset::Handle;
#[cfg(feature = "record")]
use pi_export_base::record::ERecord3D;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{constants::EngineConstants, mesh::CommandsExchangeD3, record::ERecordCMD};
pub use crate::engine::ActionSetScene3D;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
pub use pi_export_base::about_3d::node_materials::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_extend_depend(block: &mut NodeMaterialBlock, depend: &Atom) {
    block.v1_mut().depends.push(depend.deref().clone());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_extend_bind_define(block: &mut NodeMaterialBlock, bind_define: f64) {
    block.v1_mut().binddefines = block.v1().binddefines | (bind_define as BindDefine)
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_mat4(block: &mut NodeMaterialBlock, key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
    block.v1_mut().mat4.push(UniformPropertyMat4(key.deref().clone(), [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32], false))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec4(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, z: f64, w: f64, caninstance: bool) {
    block.v1_mut().vec4.push(UniformPropertyVec4(key.deref().clone(), [x as f32, y as f32, z as f32, w as f32], caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec3(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, z: f64, caninstance: bool) {
    block.v1_mut().vec3.push(UniformPropertyVec3(key.deref().clone(), [x as f32, y as f32, z as f32], caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec2(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, caninstance: bool) {
    block.v1_mut().vec2.push(UniformPropertyVec2(key.deref().clone(), [x as f32, y as f32], caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_float(block: &mut NodeMaterialBlock, key: &Atom, x: f64, caninstance: bool) {
    block.v1_mut().float.push(UniformPropertyFloat(key.deref().clone(), x as f32, caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_uint(block: &mut NodeMaterialBlock, key: &Atom, x: f64, caninstance: bool) {
    block.v1_mut().uint.push(UniformPropertyUint(key.deref().clone(), x as u32, caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_varying(block: &mut NodeMaterialBlock, name: &Atom, format: &Atom) {
    block.v1_mut().varyings.push(
        Varying { format: format.deref().clone(), name: name.deref().clone() }
    );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_texture(block: &mut NodeMaterialBlock, key: &Atom, filterable: bool, stage: f64, default_texture: f64, demision: f64) {
    block.v1_mut().textures.push(UniformTexture2DDesc::new(
        key.deref().clone(),
        if filterable { ESamplerType::FloatFilter } else { ESamplerType::Float  },
        false,
        EngineConstants::shader_stage(stage),
        EngineConstants::default_texture(default_texture),
    ))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_regist(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, block: &NodeMaterialBlock) {
    #[cfg(feature = "replay")]
    return ;

    pi_export_base::export::await_last_frame(app);
    
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::NodeMaterialBlockRegist(block.clone()));

    let mut resource = param.resource.get_mut(&mut app.world);
    CommandsExchangeD3::p3d_node_material_block_regist(&mut resource, block);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_include(includes: &mut NodematerialIncludes, block: BuiltinNodeMaterialBlock) {
    includes.v0_mut().push(block.key());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_include_by_name(includes: &mut NodematerialIncludes, block: &Atom) {
    includes.v0_mut().push(block.deref().clone());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_includes_reset(includes: &mut NodematerialIncludes) {
    includes.v0_mut().clear();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_float(uniforms: &mut MaterialUniformDefines, key: &str, val: f64, caninstance: bool) {
    uniforms.v0_mut().float_list.push(UniformPropertyFloat(pi_atom::Atom::from(key), val as f32, caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec2(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, caninstance: bool) {
    uniforms.v0_mut().vec2_list.push(UniformPropertyVec2(pi_atom::Atom::from(key), [x as f32, y as f32], caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec4(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, z: f64, w: f64, caninstance: bool) {
    uniforms.v0_mut().vec4_list.push(UniformPropertyVec4(pi_atom::Atom::from(key), [x as f32, y as f32, z as f32, w as f32], caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_uint(uniforms: &mut MaterialUniformDefines, key: &str, val: f64, caninstance: bool) {
    uniforms.v0_mut().uint_list.push(UniformPropertyUint(pi_atom::Atom::from(key), val as u32, caninstance));
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_int(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
//     uniforms.0.int_list.push(UniformPropertyInt(Atom::from(key), val as i32));
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_tex(uniforms: &mut MaterialUniformDefines, key: &str, filterable: bool, stage: f64, default_texture: f64, demision: f64) {
    uniforms.v1_mut().push(
        UniformTexture2DDesc::new(
            UniformPropertyName::from(key),
        if filterable { ESamplerType::FloatFilter } else { ESamplerType::Float  },
            false,
            EngineConstants::shader_stage(stage),
            EngineConstants::default_texture(default_texture),
        )
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_check_shader(
    app: &mut Engine, param: &mut ActionSetScene3D,
    key: &str
) -> bool {
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    resource.shader_metas.get(&KeyShaderMeta::from(key)).is_some()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_varying(block: &mut P3DShaderVaryings, name: &Atom, format: &Atom) {
    block.v0_mut().push(
        Varying { format: format.deref().clone(), name: name.deref().clone() }
    );
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_regist_material(
    app: &Engine,
    cmds: &mut CommandsExchangeD3,
    key: &str,
    uniforms: &MaterialUniformDefines,
    vs_define_code: &str,
    fs_define_code: &str,
    vs_code: &str,
    fs_code: &str,
    includes: &NodematerialIncludes,
    instance_code: &str,
    varyings: &P3DShaderVaryings,
    binds_defines_base: Option<f64>,
) -> Option<P3DShaderMeta> {
    #[cfg(feature = "replay")]
    return None;

    
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::MaterialRegist(key.to_string(), uniforms.clone(), 
        vs_define_code.to_string(), fs_define_code.to_string(), vs_code.to_string(), fs_code.to_string(), 
        includes.clone(), instance_code.to_string(), varyings.clone(), binds_defines_base
    ));

    CommandsExchangeD3::p3d_regist_material(&app.world, key, uniforms, vs_define_code, fs_define_code, vs_code, fs_code, includes, instance_code, varyings, binds_defines_base)

    // let mut nodemat = NodeMaterialBuilder::new();
    // varyings.0.iter().for_each(|v| { nodemat.varyings.0.push(v.clone()) });

    // if let Some(binds_defines_base) = binds_defines_base {
    //     nodemat.binddefines = binds_defines_base as BindDefine;
    //     // log::error!("binds_defines_base {:?}", binds_defines_base);
    // }
    // nodemat.material_instance_code = String::from(instance_code);
    // // nodemat.check_instance = EVerticeExtendCode(instance_state_check as u32);

    // nodemat.values = uniforms.0.clone();
    // nodemat.textures = uniforms.1.clone();

    // // let varyings = &mut nodemat.varyings;
    // // let mut tempvaryings = to_varyings(varying as u32);
    
    // // tempvaryings.drain(..).for_each(|item| {
    // //     varyings.0.push(item);
    // // });
    // let node_material_blocks = app.world.get_resource::<NodeMaterialBlocks>().unwrap();
    // let shader_metas = app.world.get_resource::<ShareAssetMgr::<ShaderEffectMeta>>().unwrap();
    // let enginopt = app.world.get_resource::<EngineCustomPlugins>().unwrap();
    
    // includes.0.iter().for_each(|val| {
    //     nodemat.include(val, node_material_blocks);
    // });

    // // log::warn!("Material {:?}", key);

    // nodemat.vs_define += vs_define_code;
    // nodemat.fs_define += fs_define_code;
    // nodemat.vs = String::from(vs_code);
    // nodemat.fs = String::from(fs_code);

    // // log::error!("Material {:?} {:?}", key, &nodemat.fs);
    // ActionMaterial::regist_material_meta(shader_metas, KeyShaderMeta::from(key), nodemat.meta(enginopt));

    // if let Some(data) = shader_metas.get(&KeyShaderMeta::from(key)) {
    //     Some(P3DShaderMeta(data))
    // } else { None }
}
