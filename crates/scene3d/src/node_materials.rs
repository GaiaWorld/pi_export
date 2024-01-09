

use std::ops::Deref;

use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;
use crate::constants::EngineConstants;
pub use crate::engine::ActionSetScene3D;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};

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
    pub fn key(&self) -> pi_atom::Atom {
        match self {
            BuiltinNodeMaterialBlock::BlockFloat                        => pi_atom::Atom::from(BlockFloat                        ::KEY),
            BuiltinNodeMaterialBlock::BlockViewDirection                => pi_atom::Atom::from(BlockViewDirection                ::KEY),
            BuiltinNodeMaterialBlock::BlockColorSpace                   => pi_atom::Atom::from(BlockColorSpace                   ::KEY),
            BuiltinNodeMaterialBlock::BlockColorGray                    => pi_atom::Atom::from(BlockColorGray                    ::KEY),
            BuiltinNodeMaterialBlock::BlockColorHSV                     => pi_atom::Atom::from(BlockColorHSV                     ::KEY),
            BuiltinNodeMaterialBlock::BlockTextureChannel               => pi_atom::Atom::from(BlockTextureChannel               ::KEY),
            BuiltinNodeMaterialBlock::BlockUVOffsetSpeed                => pi_atom::Atom::from(BlockUVOffsetSpeed                ::KEY),
            BuiltinNodeMaterialBlock::BlockFog                          => pi_atom::Atom::from(BlockFog                          ::KEY),
            BuiltinNodeMaterialBlock::BlockCutoff                       => pi_atom::Atom::from(BlockCutoff                       ::KEY),
            BuiltinNodeMaterialBlock::BlockMainTexture                  => pi_atom::Atom::from(BlockMainTexture                  ::KEY),
            BuiltinNodeMaterialBlock::BlockMainTextureUVOffsetSpeed     => pi_atom::Atom::from(BlockMainTextureUVOffsetSpeed     ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacity                      => pi_atom::Atom::from(BlockOpacity                      ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacityTexture               => pi_atom::Atom::from(BlockOpacityTexture               ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacityTextureUVOffsetSpeed  => pi_atom::Atom::from(BlockOpacityTextureUVOffsetSpeed  ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacity2Texture              => pi_atom::Atom::from(BlockOpacity2Texture              ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacity2TextureUVOffsetSpeed => pi_atom::Atom::from(BlockOpacity2TextureUVOffsetSpeed ::KEY),
            BuiltinNodeMaterialBlock::BlockEmissiveTexture              => pi_atom::Atom::from(BlockEmissiveTexture              ::KEY),
            BuiltinNodeMaterialBlock::BlockEmissiveTextureUVOffsetSpeed => pi_atom::Atom::from(BlockEmissiveTextureUVOffsetSpeed ::KEY),
            BuiltinNodeMaterialBlock::BlockMixTexture                   => pi_atom::Atom::from(BlockMixTexture                   ::KEY),
            BuiltinNodeMaterialBlock::BlockMixTextureUVOffsetSpeed      => pi_atom::Atom::from(BlockMixTextureUVOffsetSpeed      ::KEY),
            BuiltinNodeMaterialBlock::BlockMaskTexture                  => pi_atom::Atom::from(BlockMaskTexture                  ::KEY),
            BuiltinNodeMaterialBlock::BlockMaskTextureUVOffsetSpeed     => pi_atom::Atom::from(BlockMaskTextureUVOffsetSpeed     ::KEY),
            BuiltinNodeMaterialBlock::BlockFresnel                      => pi_atom::Atom::from(BlockFresnel                      ::KEY),
            BuiltinNodeMaterialBlock::BlockEmissiveFresnel              => pi_atom::Atom::from(BlockEmissiveFresnel              ::KEY),
            BuiltinNodeMaterialBlock::BlockOpacityFresnel               => pi_atom::Atom::from(BlockOpacityFresnel               ::KEY),
            BuiltinNodeMaterialBlock::BlockPremultiplyResult            => pi_atom::Atom::from(BlockPremultiplyResult            ::KEY),
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
pub struct NodeMaterialBlock(Atom, NodeMaterialBlockInfo);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl NodeMaterialBlock {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(key: &Atom, vs_define: &str, vs_surface: &str, fs_define: &str, fs_surface: &str, bind_defines: Option<f64>) -> Self {
        let mut info = NodeMaterialBlockInfo {
            vs_define: String::from(vs_define), vs_surface: String::from(vs_surface),
            fs_define: String::from(fs_define), fs_surface: String::from(fs_surface),
            ..Default::default()
        };
        if let Some(bind_defines) = bind_defines {
            info.binddefines = bind_defines as BindDefine;
        }
        Self(key.clone(), info)
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_extend_depend(block: &mut NodeMaterialBlock, depend: &Atom) {
    block.1.depends.push(depend.deref().clone());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_extend_bind_define(block: &mut NodeMaterialBlock, bind_define: f64) {
    block.1.binddefines = block.1.binddefines | (bind_define as BindDefine)
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_mat4(block: &mut NodeMaterialBlock, key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
    block.1.mat4.push(UniformPropertyMat4(key.deref().clone(), [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32], false))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec4(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, z: f64, w: f64, caninstance: bool) {
    block.1.vec4.push(UniformPropertyVec4(key.deref().clone(), [x as f32, y as f32, z as f32, w as f32], caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec2(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, caninstance: bool) {
    block.1.vec2.push(UniformPropertyVec2(key.deref().clone(), [x as f32, y as f32], caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_float(block: &mut NodeMaterialBlock, key: &Atom, x: f64, caninstance: bool) {
    block.1.float.push(UniformPropertyFloat(key.deref().clone(), x as f32, caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_uint(block: &mut NodeMaterialBlock, key: &Atom, x: f64, caninstance: bool) {
    block.1.uint.push(UniformPropertyUint(key.deref().clone(), x as u32, caninstance))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_varying(block: &mut NodeMaterialBlock, name: &Atom, format: &Atom) {
    block.1.varyings.push(
        Varying { format: format.deref().clone(), name: name.deref().clone() }
    );
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_texture(block: &mut NodeMaterialBlock, key: &Atom, filterable: bool, stage: f64, default_texture: f64, demision: f64) {
    block.1.textures.push(UniformTexture2DDesc::new(
        key.deref().clone(),
        wgpu::TextureSampleType::Float { filterable },
        EngineConstants::texture_view_dimension(demision),
        false,
        EngineConstants::shader_stage(stage),
        EngineConstants::default_texture(default_texture),
    ))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_regist(app: &mut Engine, param: &mut ActionSetScene3D, block: &NodeMaterialBlock) {
    pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);

    resource.node_material_blocks.0.insert(block.0.deref().clone(), block.1.clone());
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct NodematerialIncludes(Vec<pi_atom::Atom>);
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
    includes.0.push(block.key());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_include_by_name(includes: &mut NodematerialIncludes, block: &Atom) {
    includes.0.push(block.deref().clone());
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
        Self(MaterialValueBindDesc::none(pi_render::renderer::shader_stage::EShaderStage::VERTEXFRAGMENT.mode()), vec![])
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
pub fn p3d_shader_uniform_float(uniforms: &mut MaterialUniformDefines, key: &str, val: f64, caninstance: bool) {
    uniforms.0.float_list.push(UniformPropertyFloat(pi_atom::Atom::from(key), val as f32, caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec2(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, caninstance: bool) {
    uniforms.0.vec2_list.push(UniformPropertyVec2(pi_atom::Atom::from(key), [x as f32, y as f32], caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec4(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, z: f64, w: f64, caninstance: bool) {
    uniforms.0.vec4_list.push(UniformPropertyVec4(pi_atom::Atom::from(key), [x as f32, y as f32, z as f32, w as f32], caninstance));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_uint(uniforms: &mut MaterialUniformDefines, key: &str, val: f64, caninstance: bool) {
    uniforms.0.uint_list.push(UniformPropertyUint(pi_atom::Atom::from(key), val as u32, caninstance));
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_int(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
//     uniforms.0.int_list.push(UniformPropertyInt(Atom::from(key), val as i32));
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_tex(uniforms: &mut MaterialUniformDefines, key: &str, filterable: bool, stage: f64, default_texture: f64, demision: f64) {
    uniforms.1.push(
        UniformTexture2DDesc::new(
            UniformPropertyName::from(key),
            wgpu::TextureSampleType::Float { filterable },
            EngineConstants::texture_view_dimension(demision),
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
pub struct P3DShaderMeta(Handle<ShaderEffectMeta>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct P3DShaderVaryings(Vec<Varying>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl P3DShaderVaryings {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        P3DShaderVaryings(vec![])
    }

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_varying(block: &mut P3DShaderVaryings, name: &Atom, format: &Atom) {
    block.0.push(
        Varying { format: format.deref().clone(), name: name.deref().clone() }
    );
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_regist_material(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
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
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    let mut nodemat = NodeMaterialBuilder::new();
    nodemat.vs_define = String::from(vs_define_code);
    nodemat.fs_define = String::from(fs_define_code);
    varyings.0.iter().for_each(|v| { nodemat.varyings.0.push(v.clone()) });
    
    nodemat.vs = String::from(vs_code);
    nodemat.fs = String::from(fs_code);

    if let Some(binds_defines_base) = binds_defines_base {
        nodemat.binddefines = binds_defines_base as BindDefine;
        // log::error!("binds_defines_base {:?}", binds_defines_base);
    }
    nodemat.material_instance_code = String::from(instance_code);
    // nodemat.check_instance = EVerticeExtendCode(instance_state_check as u32);

    nodemat.values = uniforms.0.clone();
    nodemat.textures = uniforms.1.clone();

    // let varyings = &mut nodemat.varyings;
    // let mut tempvaryings = to_varyings(varying as u32);
    
    // tempvaryings.drain(..).for_each(|item| {
    //     varyings.0.push(item);
    // });
    
    includes.0.iter().for_each(|val| {
        nodemat.include(val, &resource.node_material_blocks);
    });

    // log::warn!("Material {:?}", key);

    ActionMaterial::regist_material_meta(&resource.shader_metas, KeyShaderMeta::from(key), nodemat.meta());

    if let Some(data) = resource.shader_metas.get(&KeyShaderMeta::from(key)) {
        Some(P3DShaderMeta(data))
    } else { None }
}
