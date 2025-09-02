

use std::ops::Deref;

use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{export::{Atom}};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMaterialBlock(pub(crate)  pi_atom::Atom, pub(crate) NodeMaterialBlockInfo);
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
        Self(key.deref().clone(), info)
    }
}
impl NodeMaterialBlock {
    pub fn v0(&self) -> &pi_atom::Atom {
        &self.0
    }
    pub fn v0_mut(&mut self) -> &mut pi_atom::Atom {
        &mut self.0
    }
    pub fn v1(&self) -> &NodeMaterialBlockInfo {
        &self.1
    }
    pub fn v1_mut(&mut self) -> &mut NodeMaterialBlockInfo {
        &mut self.1
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodematerialIncludes(pub(crate) Vec<pi_atom::Atom>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl NodematerialIncludes {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self(vec![])
    }
}
impl NodematerialIncludes {
    pub fn v0(&self) -> &Vec<pi_atom::Atom> {
        &self.0
    }
    pub fn v0_mut(&mut self) -> &mut Vec<pi_atom::Atom> {
        &mut self.0
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialUniformDefines(pub(crate) MaterialValueBindDesc, pub(crate) Vec<UniformTexture2DDesc>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl MaterialUniformDefines {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self(MaterialValueBindDesc::none(pi_render::renderer::shader_stage::EShaderStage::VERTEXFRAGMENT.mode()), vec![])
    }
}
impl MaterialUniformDefines {
    pub fn v0(&self) -> &MaterialValueBindDesc {
        &self.0
    }
    pub fn v0_mut(&mut self) -> &mut MaterialValueBindDesc {
        &mut self.0
    }
    pub fn v1(&self) -> &Vec<UniformTexture2DDesc> {
        &self.1
    }
    pub fn v1_mut(&mut self) -> &mut Vec<UniformTexture2DDesc> {
        &mut self.1
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct P3DShaderMeta(pub(crate) Handle<ShaderEffectMeta>);
impl P3DShaderMeta {
    pub fn new(v: Handle<ShaderEffectMeta>) -> Self {
        Self(v)
    }
    pub fn v0(&self) -> &Handle<ShaderEffectMeta> {
        &self.0
    }
    pub fn v0_mut(&mut self) -> &mut Handle<ShaderEffectMeta> {
        &mut self.0
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P3DShaderVaryings(pub(crate) Vec<Varying>);
impl P3DShaderVaryings {
    pub fn v0(&self) -> &Vec<Varying> {
        &self.0
    }
    pub fn v0_mut(&mut self) -> &mut Vec<Varying> {
        &mut self.0
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl P3DShaderVaryings {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        P3DShaderVaryings(vec![])
    }

}
