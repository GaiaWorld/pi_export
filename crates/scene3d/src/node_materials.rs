

use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;

use crate::engine::ActionSetScene3D;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

/// blocks.regist::<(.*)>\(\);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum BuiltinNodeMaterialBlock {
    BlockFloat,
    BlockViewDirection,

    BlockColorSpace,
    BlockColorGray,
    BlockColorHSV,
    BlockTextureChannel,
    BlockUVOffsetSpeed,
    
    BlockFog,
    BlockCutoff,

    BlockMainTexture,
    BlockMainTextureUVOffsetSpeed,

    BlockOpacity,
    BlockOpacityTexture,
    BlockOpacityTextureUVOffsetSpeed,
    
    BlockOpacity2Texture,
    BlockOpacity2TextureUVOffsetSpeed,

    BlockEmissiveTexture,
    BlockEmissiveTextureUVOffsetSpeed,
    
    BlockMixTexture,
    BlockMixTextureUVOffsetSpeed,

    BlockMaskTexture,
    BlockMaskTextureUVOffsetSpeed,

    BlockFresnel,
    BlockEmissiveFresnel,
    BlockOpacityFresnel,
    
    BlockPremultiplyResult,
}
impl BuiltinNodeMaterialBlock {
    pub fn key(&self) -> &'static str {
        match self {
            BuiltinNodeMaterialBlock::BlockFloat                        => BlockFloat                        ::KEY,
            BuiltinNodeMaterialBlock::BlockViewDirection                => BlockViewDirection                ::KEY,
            BuiltinNodeMaterialBlock::BlockColorSpace                   => BlockColorSpace                   ::KEY,
            BuiltinNodeMaterialBlock::BlockColorGray                    => BlockColorGray                    ::KEY,
            BuiltinNodeMaterialBlock::BlockColorHSV                     => BlockColorHSV                     ::KEY,
            BuiltinNodeMaterialBlock::BlockTextureChannel               => BlockTextureChannel               ::KEY,
            BuiltinNodeMaterialBlock::BlockUVOffsetSpeed                => BlockUVOffsetSpeed                ::KEY,
            BuiltinNodeMaterialBlock::BlockFog                          => BlockFog                          ::KEY,
            BuiltinNodeMaterialBlock::BlockCutoff                       => BlockCutoff                       ::KEY,
            BuiltinNodeMaterialBlock::BlockMainTexture                  => BlockMainTexture                  ::KEY,
            BuiltinNodeMaterialBlock::BlockMainTextureUVOffsetSpeed     => BlockMainTextureUVOffsetSpeed     ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacity                      => BlockOpacity                      ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacityTexture               => BlockOpacityTexture               ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacityTextureUVOffsetSpeed  => BlockOpacityTextureUVOffsetSpeed  ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacity2Texture              => BlockOpacity2Texture              ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacity2TextureUVOffsetSpeed => BlockOpacity2TextureUVOffsetSpeed ::KEY,
            BuiltinNodeMaterialBlock::BlockEmissiveTexture              => BlockEmissiveTexture              ::KEY,
            BuiltinNodeMaterialBlock::BlockEmissiveTextureUVOffsetSpeed => BlockEmissiveTextureUVOffsetSpeed ::KEY,
            BuiltinNodeMaterialBlock::BlockMixTexture                   => BlockMixTexture                   ::KEY,
            BuiltinNodeMaterialBlock::BlockMixTextureUVOffsetSpeed      => BlockMixTextureUVOffsetSpeed      ::KEY,
            BuiltinNodeMaterialBlock::BlockMaskTexture                  => BlockMaskTexture                  ::KEY,
            BuiltinNodeMaterialBlock::BlockMaskTextureUVOffsetSpeed     => BlockMaskTextureUVOffsetSpeed     ::KEY,
            BuiltinNodeMaterialBlock::BlockFresnel                      => BlockFresnel                      ::KEY,
            BuiltinNodeMaterialBlock::BlockEmissiveFresnel              => BlockEmissiveFresnel              ::KEY,
            BuiltinNodeMaterialBlock::BlockOpacityFresnel               => BlockOpacityFresnel               ::KEY,
            BuiltinNodeMaterialBlock::BlockPremultiplyResult            => BlockPremultiplyResult            ::KEY,
        }
    }
}

/// Varying 枚举 - 32 种数据
/// 0b_0000_0000_0000_0000_0000_0000_0000_0000

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub enum EVarying {
//     VARYING_POSITION_WS         = 0b_0000_0000_0000_0000_0000_0000_0000_0001,
//     VARYING_POSITION_OS         = 0b_0000_0000_0000_0000_0000_0000_0000_0010,
//     VARYING_POSITION_SS         = 0b_0000_0000_0000_0000_0000_0000_0000_0100,
//     VARYING_NORMAL_WS           = 0b_0000_0000_0000_0000_0000_0000_0000_1000,
//     VARYING_NORMAL_OS           = 0b_0000_0000_0000_0000_0000_0000_0001_0000,
//     VARYING_UV                  = 0b_0000_0000_0000_0000_0000_0000_0010_0000,
//     VARYING_UV2                 = 0b_0000_0000_0000_0000_0000_0000_0100_0000,
//     VARYING_UV3                 = 0b_0000_0000_0000_0000_0000_0000_1000_0000,
//     VARYING_UV4                 = 0b_0000_0000_0000_0000_0000_0001_0000_0000,
//     VARYING_UV5                 = 0b_0000_0000_0000_0000_0000_0010_0000_0000,
//     VARYING_UV6                 = 0b_0000_0000_0000_0000_0000_0100_0000_0000,
//     VARYING_UV7                 = 0b_0000_0000_0000_0000_0000_1000_0000_0000,
//     VARYING_UV8                 = 0b_0000_0000_0000_0000_0001_0000_0000_0000,
//     VARYING_COLOR               = 0b_0000_0000_0000_0000_0010_0000_0000_0000,
//     VARYING_UNUSE_0             = 0b_0000_0000_0000_0000_0100_0000_0000_0000,
//     VARYING_UNUSE_1             = 0b_0000_0000_0000_0000_1000_0000_0000_0000,
//     VARYING_UNUSE_2             = 0b_0000_0000_0000_0001_0000_0000_0000_0000,
//     VARYING_UNUSE_3             = 0b_0000_0000_0000_0010_0000_0000_0000_0000,
//     VARYING_UNUSE_4             = 0b_0000_0000_0000_0100_0000_0000_0000_0000,
//     VARYING_UNUSE_5             = 0b_0000_0000_0000_1000_0000_0000_0000_0000,
//     VARYING_UNUSE_6             = 0b_0000_0000_0001_0000_0000_0000_0000_0000,
//     VARYING_UNUSE_7             = 0b_0000_0000_0010_0000_0000_0000_0000_0000,
//     VARYING_UNUSE_8             = 0b_0000_0000_0100_0000_0000_0000_0000_0000,
//     VARYING_UNUSE_9             = 0b_0000_0000_1000_0000_0000_0000_0000_0000,
//     VARYING_V4A                 = 0b_0000_0001_0000_0000_0000_0000_0000_0000,
//     VARYING_V4B                 = 0b_0000_0010_0000_0000_0000_0000_0000_0000,
//     VARYING_V4C                 = 0b_0000_0100_0000_0000_0000_0000_0000_0000,
//     VARYING_V4D                 = 0b_0000_1000_0000_0000_0000_0000_0000_0000,
//     VARYING_V4E                 = 0b_0001_0000_0000_0000_0000_0000_0000_0000,
//     VARYING_V4F                 = 0b_0010_0000_0000_0000_0000_0000_0000_0000,
//     VARYING_V4G                 = 0b_0100_0000_0000_0000_0000_0000_0000_0000,
//     VARYING_V4H                 = 0b_1000_0000_0000_0000_0000_0000_0000_0000,
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_POSITION_WS       : u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0001;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_POSITION_OS       : u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0010;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_POSITION_SS       : u32 = 0b_0000_0000_0000_0000_0000_0000_0000_0100;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_NORMAL_WS         : u32 = 0b_0000_0000_0000_0000_0000_0000_0000_1000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_NORMAL_OS         : u32 = 0b_0000_0000_0000_0000_0000_0000_0001_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV                : u32 = 0b_0000_0000_0000_0000_0000_0000_0010_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV2               : u32 = 0b_0000_0000_0000_0000_0000_0000_0100_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV3               : u32 = 0b_0000_0000_0000_0000_0000_0000_1000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV4               : u32 = 0b_0000_0000_0000_0000_0000_0001_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV5               : u32 = 0b_0000_0000_0000_0000_0000_0010_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV6               : u32 = 0b_0000_0000_0000_0000_0000_0100_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV7               : u32 = 0b_0000_0000_0000_0000_0000_1000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UV8               : u32 = 0b_0000_0000_0000_0000_0001_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_COLOR             : u32 = 0b_0000_0000_0000_0000_0010_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
//     /// 待定 
pub const VARYING_UNUSE_0           : u32 = 0b_0000_0000_0000_0000_0100_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_1           : u32 = 0b_0000_0000_0000_0000_1000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_2           : u32 = 0b_0000_0000_0000_0001_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_3           : u32 = 0b_0000_0000_0000_0010_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_4           : u32 = 0b_0000_0000_0000_0100_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_5           : u32 = 0b_0000_0000_0000_1000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_6           : u32 = 0b_0000_0000_0001_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_7           : u32 = 0b_0000_0000_0010_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_8           : u32 = 0b_0000_0000_0100_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_UNUSE_9           : u32 = 0b_0000_0000_1000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
//     /// 预留 8 个vec4 做自定义数据 
pub const VARYING_V4A               : u32 = 0b_0000_0001_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4B               : u32 = 0b_0000_0010_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4C               : u32 = 0b_0000_0100_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4D               : u32 = 0b_0000_1000_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4E               : u32 = 0b_0001_0000_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4F               : u32 = 0b_0010_0000_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4G               : u32 = 0b_0100_0000_0000_0000_0000_0000_0000_0000;
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
pub const VARYING_V4H               : u32 = 0b_1000_0000_0000_0000_0000_0000_0000_0000;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct NodematerialIncludes(Vec<BuiltinNodeMaterialBlock>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl NodematerialIncludes {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self(vec![])
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_include(includes: &mut NodematerialIncludes, block: BuiltinNodeMaterialBlock) {
    includes.0.push(block);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_includes_reset(includes: &mut NodematerialIncludes) {
    includes.0.clear();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct MaterialUniformDefines(MaterialValueBindDesc, Vec<UniformTexture2DDesc>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl MaterialUniformDefines {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self(MaterialValueBindDesc::none(EShaderStage::VERTEXFRAGMENT.mode()), vec![])
    }
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_mat4(uniforms: &mut MaterialUniformDefines, key: &str, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
//     uniforms.0.mat4_list.push(UniformPropertyMat4(Atom::from(key), [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32]));
// }
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_mat2(uniforms: &mut MaterialUniformDefines, key: &str, m11: f64, m12: f64, m21: f64, m22: f64) {
//     uniforms.0.mat2_list.push(UniformPropertyMat2(Atom::from(key), [m11 as f32, m12 as f32, m21 as f32, m22 as f32]));
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_float(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
    uniforms.0.float_list.push(UniformPropertyFloat(Atom::from(key), val as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec2(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64) {
    uniforms.0.vec2_list.push(UniformPropertyVec2(Atom::from(key), [x as f32, y as f32]));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec4(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, z: f64, w: f64) {
    uniforms.0.vec4_list.push(UniformPropertyVec4(Atom::from(key), [x as f32, y as f32, z as f32, w as f32]));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_uint(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
    uniforms.0.uint_list.push(UniformPropertyUint(Atom::from(key), val as u32));
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_int(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
//     uniforms.0.int_list.push(UniformPropertyInt(Atom::from(key), val as i32));
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_tex(uniforms: &mut MaterialUniformDefines, key: &str, filterable: bool, stage: pi_export_base::constants::EShaderStage, default: pi_export_base::constants::EDefaultTexture) {
    uniforms.1.push(
        UniformTexture2DDesc::new(
            UniformPropertyName::from(key),
            wgpu::TextureSampleType::Float { filterable },
            false,
            stage.val(),
            default.val()
        )
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_check_shader(
    app: &mut Engine, param: &mut ActionSetScene3D,
    key: &str
) -> bool {
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.matcmd.metas.get(&KeyShaderMeta::from(key)).is_some()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_regist_material(
    app: &mut Engine, param: &mut ActionSetScene3D,
    key: &str,
    uniforms: &MaterialUniformDefines,
    vs_define_code: &str,
    fs_define_code: &str,
    vs_code: &str,
    fs_code: &str,
    varying: f64,
    includes: &NodematerialIncludes,
) {
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let mut nodemat = NodeMaterialBuilder::new();
    nodemat.vs_define = String::from(vs_define_code);
    nodemat.fs_define = String::from(fs_define_code);
    
    nodemat.vs = String::from(vs_code);
    nodemat.fs = String::from(fs_code);

    nodemat.values = uniforms.0.clone();
    nodemat.textures = uniforms.1.clone();

    let varyings = &mut nodemat.varyings;
    let varying = varying as u32;

    if varying & VARYING_POSITION_WS  ==  VARYING_POSITION_WS   { varyings.0.push(Varying { format: Atom::from("vec3"), name: Atom::from("v_pos") }); }
    if varying & VARYING_POSITION_OS  ==  VARYING_POSITION_OS   { varyings.0.push(Varying { format: Atom::from("vec3"), name: Atom::from("v_pos_os") }); }
    if varying & VARYING_POSITION_SS  ==  VARYING_POSITION_SS   { varyings.0.push(Varying { format: Atom::from("vec3"), name: Atom::from("v_pos_ss") }); }
    if varying & VARYING_NORMAL_WS    ==  VARYING_NORMAL_WS     { varyings.0.push(Varying { format: Atom::from("vec3"), name: Atom::from("v_normal")    }); }
    if varying & VARYING_NORMAL_OS    ==  VARYING_NORMAL_OS     { varyings.0.push(Varying { format: Atom::from("vec3"), name: Atom::from("v_normal_os") }); }
    if varying & VARYING_UV           ==  VARYING_UV            { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv")  }); }
    if varying & VARYING_UV2          ==  VARYING_UV2           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv2") }); }
    if varying & VARYING_UV3          ==  VARYING_UV3           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv3") }); }
    if varying & VARYING_UV4          ==  VARYING_UV4           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv4") }); }
    if varying & VARYING_UV5          ==  VARYING_UV5           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv5") }); }
    if varying & VARYING_UV6          ==  VARYING_UV6           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv6") }); }
    if varying & VARYING_UV7          ==  VARYING_UV7           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv7") }); }
    if varying & VARYING_UV8          ==  VARYING_UV8           { varyings.0.push(Varying { format: Atom::from("vec2"), name: Atom::from("v_uv8") }); }
    if varying & VARYING_COLOR        ==  VARYING_COLOR         { varyings.0.push(Varying { format: Atom::from("vec4"), name: Atom::from("v_color") }); }

    if varying & VARYING_V4A          ==  VARYING_V4A           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4B          ==  VARYING_V4B           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4C          ==  VARYING_V4C           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4D          ==  VARYING_V4D           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4E          ==  VARYING_V4E           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4F          ==  VARYING_V4F           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4G          ==  VARYING_V4G           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    if varying & VARYING_V4H          ==  VARYING_V4H           { varyings.0.push(Varying { format: Atom::from(""), name: Atom::from("") }); }
    
    includes.0.iter().for_each(|val| {
        nodemat.include(val.key(), &cmds.node_material_blocks);
    });

    // log::warn!("Material {:?}", key);

    ActionMaterial::regist_material_meta(&cmds.matcmd.metas, KeyShaderMeta::from(key), nodemat.meta());
}