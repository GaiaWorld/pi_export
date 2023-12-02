
use pi_engine_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
use pi_render::rhi::sampler::{EAddressMode, EAnisotropyClamp};
use pi_scene_context::prelude::*;
use wgpu::TextureUsages;

pub struct EngineConstants;
impl EngineConstants {
    pub const RENDER_ALIGNMENT_VIEW: u8 = 0;
    pub const RENDER_ALIGNMENT_WORLD: u8 = 1;
    pub const RENDER_ALIGNMENT_LOCAL: u8 = 2;
    pub const RENDER_ALIGNMENT_FACING: u8 = 3;
    pub const RENDER_ALIGNMENT_VELOCITY: u8 = 4;
    pub const RENDER_ALIGNMENT_STRENTCHED_BILLBOARD: u8 = 5;
    pub const RENDER_ALIGNMENT_HORIZONTAL_BILLBOARD: u8 = 6;
    pub const RENDER_ALIGNMENT_VERTICAL_BILLBOARD: u8 = 7;

    pub const SCALING_MODE_HIERARCHY: u8 = 0;
    pub const SCALING_MODE_LOCAL: u8 = 1;
    pub const SCALING_MODE_SHAPE: u8 = 2;

    pub const RGBA8_SRGB: u32 = 0;
    pub const BGRA8: u32 = 1;
    pub const BGRA8_SRGB: u32 = 2;

    pub const DEPTH32F: u32 = 1;
    pub const DEPTH24: u32 = 2;
    pub const DEPTH16_UNORM: u32 = 3;
    pub const STENCIL8: u32 = 3;

    pub const CLAMP_TO_BORDER: u32 = 3;

    pub const ANISOTROPY_NONE: u8 = 0;
    pub const ANISOTROPY_ONE: u8 = 1;
    pub const ANISOTROPY_TWO: u8 = 2;
    pub const ANISOTROPY_FOUR: u8 = 3;
    pub const ANISOTROPY_EIGHT: u8 = 4;
    pub const ANISOTROPY_SIXTEEN: u8 = 5;

    pub const PASS_TAG_01: PassTagValue = 0b0000_0000_0000_0001;
    pub const PASS_TAG_02: PassTagValue = 0b0000_0000_0000_0010;
    pub const PASS_TAG_03: PassTagValue = 0b0000_0000_0000_0100;
    pub const PASS_TAG_04: PassTagValue = 0b0000_0000_0000_1000;
    pub const PASS_TAG_05: PassTagValue = 0b0000_0000_0001_0000;
    pub const PASS_TAG_06: PassTagValue = 0b0000_0000_0010_0000;
    pub const PASS_TAG_07: PassTagValue = 0b0000_0000_0100_0000;
    pub const PASS_TAG_08: PassTagValue = 0b0000_0000_1000_0000;

    /// [0, 0, 0, 0]
    pub const BORDER_TRANSPARENT_BLACK: u8 = 1;
    /// [0, 0, 0, 1]
    pub const BORDER_OPAQUE_BLACK: u8 = 2;
    /// [1, 1, 1, 1]
    pub const BORDER_OPAQUE_WHITE: u8 = 3;

    /// On the Metal backend, this is equivalent to `TransparentBlack` for
    /// textures that have an alpha component, and equivalent to `OpaqueBlack`
    /// for textures that do not have an alpha component. On other backends,
    /// this is equivalent to `TransparentBlack`. Requires
    /// [`Features::ADDRESS_MODE_CLAMP_TO_ZERO`]. Not supported on the web.
    pub const BORDER_ZERO: u8 = 4;

    pub const BIND_SCENE_EFFECT: u32                = 1 << 0;
    pub const BIND_LIGHTING: u32                    = 1 << 1;
    pub const BIND_SHADOWMAP: u32                   = 1 << 2;
    pub const BIND_ENVIRONMENT_BRDF_TEXTURE: u32    = 1 << 3;
    pub const BIND_ENVIRONMENT_LIGHTING: u32        = 1 << 4;
    pub const BIND_SCREEN_OPAQUE_TARGET: u32        = 1 << 5;
    pub const BIND_SCREEN_DEPTH_TARGET: u32         = 1 << 6;
    pub const BIND_EFFECT_TEXTURE: u32              = 1 << 7;
    pub const BIND_EFFECT_VALUE: u32                = 1 << 8;
    pub const BIND_VIEWER: u32                      = 1 << 9;
    pub const BIND_MODEL: u32                       = 1 << 10;

    pub const LIGHT_DIRECTION: u8 = 0;
    pub const LIGHT_POINT: u8 = 1;
    pub const LIGHT_SPOT: u8 = 2;
    pub const LIGHT_HEMISPHERE: u8 = 3;
    
    pub const TEXTURE_USEAGE_COPY_DST: u8 = 1;
    pub const TEXTURE_USEAGE_COPY_SRC: u8 = 2;
    pub const TEXTURE_USEAGE_BINDING: u8 = 4;
    pub const TEXTURE_USEAGE_STORAGE: u8 = 8;

    pub fn render_alignment(val: f64) -> ERenderAlignment {
        match (val as u8) {
            Self::RENDER_ALIGNMENT_VIEW => { ERenderAlignment::View }
            Self::RENDER_ALIGNMENT_WORLD => { ERenderAlignment::World }
            Self::RENDER_ALIGNMENT_LOCAL => { ERenderAlignment::Local }
            Self::RENDER_ALIGNMENT_FACING => { ERenderAlignment::Facing }
            Self::RENDER_ALIGNMENT_VELOCITY => { ERenderAlignment::Velocity }
            Self::RENDER_ALIGNMENT_STRENTCHED_BILLBOARD => { ERenderAlignment::StretchedBillboard }
            Self::RENDER_ALIGNMENT_HORIZONTAL_BILLBOARD => { ERenderAlignment::HorizontalBillboard }
            Self::RENDER_ALIGNMENT_VERTICAL_BILLBOARD => { ERenderAlignment::VerticalBillboard },
            _ => { ERenderAlignment::Local }
        }
    }
    pub fn scaling_mode(val: f64) -> EScalingMode {
        match (val as u8) {
            Self::SCALING_MODE_HIERARCHY => { EScalingMode::Hierarchy }
            Self::SCALING_MODE_LOCAL => { EScalingMode::Local }
            Self::SCALING_MODE_SHAPE => { EScalingMode::Shape }
            _ => { EScalingMode::Hierarchy }
        }
    }
    pub fn render_color_format(val: f64) -> ColorFormat {
        match (val as u32) {
            ContextConstants::R8 => { ColorFormat::R8Unorm },
            ContextConstants::R8_SNORM => { ColorFormat::R8Snorm },
            ContextConstants::R8UI => { ColorFormat::R8Uint },
            ContextConstants::R8I => { ColorFormat::R8Sint },
    
            ContextConstants::R16UI => { ColorFormat::R16Uint },
            ContextConstants::R16I => { ColorFormat::R16Sint },
            ContextConstants::R16F => { ColorFormat::R16Float },

            ContextConstants::RG8 => { ColorFormat::Rg8Unorm },
            ContextConstants::RG8_SNORM => { ColorFormat::Rg8Snorm },
            ContextConstants::RG8UI => { ColorFormat::Rg8Uint },
            ContextConstants::RG8I => { ColorFormat::Rg8Sint },
            
            ContextConstants::R32UI => { ColorFormat::R32Uint },
            ContextConstants::R32I => { ColorFormat::R32Sint },
            ContextConstants::R32F => { ColorFormat::R32Float },
            
            ContextConstants::RG16UI => { ColorFormat::Rg16Uint },
            ContextConstants::RG16I => { ColorFormat::Rg16Sint },
            ContextConstants::RG16F => { ColorFormat::Rg16Float },

            ContextConstants::RGBA8 => { ColorFormat::Rgba8Unorm },
            ContextConstants::RGBA8UI => { ColorFormat::Rgba8Uint },
            ContextConstants::RGBA8I => { ColorFormat::Rgba8Sint },
            ContextConstants::RGBA8_SNORM => { ColorFormat::Rgba8Snorm },
            
            Self::RGBA8_SRGB => { ColorFormat::Rgba8UnormSrgb },
            Self::BGRA8 => { ColorFormat::Bgra8Unorm },
            Self::BGRA8_SRGB => { ColorFormat::Bgra8UnormSrgb },

            _ => { ColorFormat::Rgba8Unorm },
        }
    }
    pub fn render_depth_format(val: f64) -> DepthStencilFormat {
        match (val as u32) {
            ContextConstants::NONE => { DepthStencilFormat::None },
            ContextConstants::DEPTH24_STENCIL8 => { DepthStencilFormat::Depth24PlusStencil8 },
            ContextConstants::DEPTH32F_STENCIL8 => { DepthStencilFormat::Depth32FloatStencil8 },
            ContextConstants::DEPTH_COMPONENT32F => { DepthStencilFormat::Depth32Float },
            ContextConstants::DEPTH_COMPONENT24 => { DepthStencilFormat::Depth24Plus },
            ContextConstants::DEPTH_COMPONENT16 => { DepthStencilFormat::Depth16Unorm },
            Self::STENCIL8 => { DepthStencilFormat::Stencil8 },
            Self::DEPTH16_UNORM => { DepthStencilFormat::Depth16Unorm },
            Self::DEPTH24 => { DepthStencilFormat::Depth24Plus },
            Self::DEPTH32F => { DepthStencilFormat::Depth32Float },
            _ => { DepthStencilFormat::None },
        }
    }
    pub fn address_mode(val: f64) -> EAddressMode {
        match (val as u32) {
            ContextConstants::REPEAT => { EAddressMode::Repeat },
            ContextConstants::CLAMP_TO_EDGE => { EAddressMode::ClampToEdge },
            ContextConstants::MIRRORED_REPEAT => { EAddressMode::MirrorRepeat },
            Self::CLAMP_TO_BORDER => { EAddressMode::ClampToBorder },
            _ => { EAddressMode::ClampToEdge },
        }
    }
    pub fn anisotropy_clamp(val: f64) -> EAnisotropyClamp {
        match (val as u8) {
            Self::ANISOTROPY_NONE => { EAnisotropyClamp::None },
            Self::ANISOTROPY_ONE => { EAnisotropyClamp::One },
            Self::ANISOTROPY_TWO => { EAnisotropyClamp::Two },
            Self::ANISOTROPY_FOUR => { EAnisotropyClamp::Four },
            Self::ANISOTROPY_EIGHT => { EAnisotropyClamp::Eight },
            Self::ANISOTROPY_SIXTEEN => { EAnisotropyClamp::Sixteen },
            _ => { EAnisotropyClamp::None },
        }
    }
    pub fn border_color(val: f64) -> Option<SamplerBorderColor> {
        match (val as u8) {
            Self::BORDER_TRANSPARENT_BLACK => { Some(SamplerBorderColor::TransparentBlack) },
            Self::BORDER_OPAQUE_BLACK => { Some(SamplerBorderColor::OpaqueBlack) },
            Self::BORDER_OPAQUE_WHITE => { Some(SamplerBorderColor::OpaqueWhite) },
            Self::BORDER_ZERO => { Some(SamplerBorderColor::Zero) },
            _ => { None },
        }
    }
    pub fn passtag(val: f64) -> PassTag {
        match (val as PassTagValue) {
            Self::PASS_TAG_01 => { PassTag::PASS_TAG_01 },
            Self::PASS_TAG_02 => { PassTag::PASS_TAG_02 },
            Self::PASS_TAG_03 => { PassTag::PASS_TAG_03 },
            Self::PASS_TAG_04 => { PassTag::PASS_TAG_04 },
            Self::PASS_TAG_05 => { PassTag::PASS_TAG_05 },
            Self::PASS_TAG_06 => { PassTag::PASS_TAG_06 },
            Self::PASS_TAG_07 => { PassTag::PASS_TAG_07 },
            Self::PASS_TAG_08 => { PassTag::PASS_TAG_08 },
            _ => { PassTag::PASS_TAG_08 },
        }
    }
    pub fn light(val: f64) -> ELightType {
        match (val as u8) {
            Self::LIGHT_DIRECTION => { ELightType::Direct },
            Self::LIGHT_HEMISPHERE => { ELightType::Hemispheric },
            Self::LIGHT_POINT => { ELightType::Point },
            Self::LIGHT_SPOT => { ELightType::Spot },
            _ => { ELightType::Direct },
        }
    }
    pub fn texture_usage(val: f64) -> TextureUsages {
        let mut usages = TextureUsages::empty();
        let val = val as u8;
        if val & EngineConstants::TEXTURE_USEAGE_BINDING == EngineConstants::TEXTURE_USEAGE_BINDING {
            usages = usages | TextureUsages::TEXTURE_BINDING;
        }
        if val & EngineConstants::TEXTURE_USEAGE_COPY_DST == EngineConstants::TEXTURE_USEAGE_COPY_DST {
            usages = usages | TextureUsages::COPY_DST;
        }
        if val & EngineConstants::TEXTURE_USEAGE_COPY_SRC == EngineConstants::TEXTURE_USEAGE_COPY_SRC {
            usages = usages | TextureUsages::COPY_SRC;
        }
        if val & EngineConstants::TEXTURE_USEAGE_STORAGE == EngineConstants::TEXTURE_USEAGE_STORAGE {
            usages = usages | TextureUsages::STORAGE_BINDING;
        }
        usages
    }
}