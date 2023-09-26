

use std::ops::Deref;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_node_materials::prelude::*;
pub use crate::engine::ActionSetScene3D;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EShaderStage {
    /// Binding is not visible from any shader stage.
    NONE,
    /// Binding is visible from the vertex shader of a render pipeline.
    VERTEX,
    /// Binding is visible from the fragment shader of a render pipeline.
    FRAGMENT,
    /// Binding is visible from the compute shader of a compute pipeline.
    COMPUTE,
    /// Binding is visible from the vertex and fragment shaders of a render pipeline.
    VERTEXFRAGMENT,
}
impl EShaderStage {
    pub fn val(&self) -> pi_render::renderer::shader_stage::EShaderStage {
        match self {
            EShaderStage::NONE              => pi_render::renderer::shader_stage::EShaderStage::NONE,
            EShaderStage::VERTEX            => pi_render::renderer::shader_stage::EShaderStage::VERTEX,
            EShaderStage::FRAGMENT          => pi_render::renderer::shader_stage::EShaderStage::FRAGMENT,
            EShaderStage::COMPUTE           => pi_render::renderer::shader_stage::EShaderStage::COMPUTE,
            EShaderStage::VERTEXFRAGMENT    => pi_render::renderer::shader_stage::EShaderStage::VERTEXFRAGMENT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EDefaultTexture {
    Black,
    White,
}
impl EDefaultTexture {
    pub fn val(&self) -> pi_render::renderer::buildin_data::EDefaultTexture {
        match self {
            EDefaultTexture::Black => pi_render::renderer::buildin_data::EDefaultTexture::Black,
            EDefaultTexture::White => pi_render::renderer::buildin_data::EDefaultTexture::White,
        }
    }
}
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
    pub fn create(key: &Atom, vs: &str, fs: &str) -> Self {
        Self(key.clone(), NodeMaterialBlockInfo { fs: String::from(fs), vs: String::from(vs), mat4: vec![], vec4: vec![], vec2: vec![], float: vec![], uint: vec![], textures: vec![], varyings: vec![], depends: vec![] })
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_mat4(block: &mut NodeMaterialBlock, key: &Atom, m11: f64, m12: f64, m13: f64, m14: f64, m21: f64, m22: f64, m23: f64, m24: f64, m31: f64, m32: f64, m33: f64, m34: f64, m41: f64, m42: f64, m43: f64, m44: f64) {
    block.1.mat4.push(UniformPropertyMat4(key.deref().clone(), [m11 as f32, m12 as f32, m13 as f32, m14 as f32, m21 as f32, m22 as f32, m23 as f32, m24 as f32, m31 as f32, m32 as f32, m33 as f32, m34 as f32, m41 as f32, m42 as f32, m43 as f32, m44 as f32]))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec4(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64, z: f64, w: f64) {
    block.1.vec4.push(UniformPropertyVec4(key.deref().clone(), [x as f32, y as f32, z as f32, w as f32]))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_vec2(block: &mut NodeMaterialBlock, key: &Atom, x: f64, y: f64) {
    block.1.vec2.push(UniformPropertyVec2(key.deref().clone(), [x as f32, y as f32]))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_float(block: &mut NodeMaterialBlock, key: &Atom, x: f64) {
    block.1.float.push(UniformPropertyFloat(key.deref().clone(), x as f32))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_uint(block: &mut NodeMaterialBlock, key: &Atom, x: f64) {
    block.1.uint.push(UniformPropertyUint(key.deref().clone(), x as u32))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_varying(block: &mut NodeMaterialBlock, varying: f64) {
    block.1.varyings = to_varyings(varying as u32);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_texture(block: &mut NodeMaterialBlock, key: &Atom, filterable: bool, stage: EShaderStage, default_r: EDefaultTexture) {
    block.1.textures.push(UniformTexture2DDesc::new(
        key.deref().clone(),
        wgpu::TextureSampleType::Float { filterable },
        false,
        stage.val(),
        default_r.val()
    ))
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_node_material_block_regist(app: &mut Engine, param: &mut ActionSetScene3D, block: &NodeMaterialBlock) {
    
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.node_material_blocks.0.insert(block.0.deref().clone(), block.1.clone());
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
pub fn p3d_shader_uniform_float(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
    uniforms.0.float_list.push(UniformPropertyFloat(pi_atom::Atom::from(key), val as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec2(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64) {
    uniforms.0.vec2_list.push(UniformPropertyVec2(pi_atom::Atom::from(key), [x as f32, y as f32]));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_vec4(uniforms: &mut MaterialUniformDefines, key: &str, x: f64, y: f64, z: f64, w: f64) {
    uniforms.0.vec4_list.push(UniformPropertyVec4(pi_atom::Atom::from(key), [x as f32, y as f32, z as f32, w as f32]));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_uint(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
    uniforms.0.uint_list.push(UniformPropertyUint(pi_atom::Atom::from(key), val as u32));
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_shader_uniform_int(uniforms: &mut MaterialUniformDefines, key: &str, val: f64) {
//     uniforms.0.int_list.push(UniformPropertyInt(Atom::from(key), val as i32));
// }
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shader_uniform_tex(uniforms: &mut MaterialUniformDefines, key: &str, filterable: bool, stage: EShaderStage, default_r: EDefaultTexture) {
    uniforms.1.push(
        UniformTexture2DDesc::new(
            UniformPropertyName::from(key),
            wgpu::TextureSampleType::Float { filterable },
            false,
            stage.val(),
            default_r.val()
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
    let mut tempvaryings = to_varyings(varying as u32);
    
    tempvaryings.drain(..).for_each(|item| {
        varyings.0.push(item);
    });
    
    includes.0.iter().for_each(|val| {
        nodemat.include(val, &cmds.node_material_blocks);
    });

    // log::warn!("Material {:?}", key);

    ActionMaterial::regist_material_meta(&cmds.matcmd.metas, KeyShaderMeta::from(key), nodemat.meta());
}

fn to_varyings(varying: u32) -> Vec<Varying> {
    let mut result = vec![];
    if varying & VARYING_POSITION_WS  ==  VARYING_POSITION_WS   { result.push(Varying { format: pi_atom::Atom::from("vec3"), name: pi_atom::Atom::from("v_pos") }); }
    if varying & VARYING_POSITION_OS  ==  VARYING_POSITION_OS   { result.push(Varying { format: pi_atom::Atom::from("vec3"), name: pi_atom::Atom::from("v_pos_os") }); }
    if varying & VARYING_POSITION_SS  ==  VARYING_POSITION_SS   { result.push(Varying { format: pi_atom::Atom::from("vec3"), name: pi_atom::Atom::from("v_pos_ss") }); }
    if varying & VARYING_NORMAL_WS    ==  VARYING_NORMAL_WS     { result.push(Varying { format: pi_atom::Atom::from("vec3"), name: pi_atom::Atom::from("v_normal")    }); }
    if varying & VARYING_NORMAL_OS    ==  VARYING_NORMAL_OS     { result.push(Varying { format: pi_atom::Atom::from("vec3"), name: pi_atom::Atom::from("v_normal_os") }); }
    if varying & VARYING_UV           ==  VARYING_UV            { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv")  }); }
    if varying & VARYING_UV2          ==  VARYING_UV2           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv2") }); }
    if varying & VARYING_UV3          ==  VARYING_UV3           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv3") }); }
    if varying & VARYING_UV4          ==  VARYING_UV4           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv4") }); }
    if varying & VARYING_UV5          ==  VARYING_UV5           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv5") }); }
    if varying & VARYING_UV6          ==  VARYING_UV6           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv6") }); }
    if varying & VARYING_UV7          ==  VARYING_UV7           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv7") }); }
    if varying & VARYING_UV8          ==  VARYING_UV8           { result.push(Varying { format: pi_atom::Atom::from("vec2"), name: pi_atom::Atom::from("v_uv8") }); }
    if varying & VARYING_COLOR        ==  VARYING_COLOR         { result.push(Varying { format: pi_atom::Atom::from("vec4"), name: pi_atom::Atom::from("v_color") }); }
    if varying & VARYING_V4A          ==  VARYING_V4A           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4B          ==  VARYING_V4B           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4C          ==  VARYING_V4C           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4D          ==  VARYING_V4D           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4E          ==  VARYING_V4E           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4F          ==  VARYING_V4F           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4G          ==  VARYING_V4G           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }
    if varying & VARYING_V4H          ==  VARYING_V4H           { result.push(Varying { format: pi_atom::Atom::from(""), name: pi_atom::Atom::from("") }); }

    return result;
}