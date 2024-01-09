
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
    pub const STENCIL8: u32 = 4;

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

    pub const DEFAULT_TEXTURE_BLACK: u8 = 0;
    pub const DEFAULT_TEXTURE_WHITE: u8 = 1;

    pub const TEXTURE_USEAGE_COPY_DST: u8 = 1;
    pub const TEXTURE_USEAGE_COPY_SRC: u8 = 2;
    pub const TEXTURE_USEAGE_BINDING: u8 = 4;
    pub const TEXTURE_USEAGE_STORAGE: u8 = 8;

    pub const TEXTURE_VIEW_DIMENSION_D1 : u8        = 1;
    pub const TEXTURE_VIEW_DIMENSION_D2 : u8        = 2;
    pub const TEXTURE_VIEW_DIMENSION_D2ARRAY : u8   = 3;
    pub const TEXTURE_VIEW_DIMENSION_CUBE : u8      = 4;
    pub const TEXTURE_VIEW_DIMENSION_CUBEARRAY : u8 = 5;
    pub const TEXTURE_VIEW_DIMENSION_D3 : u8        = 6;

    pub const  SHADER_STAGE_NONE : u8 = 0;
    pub const  SHADER_STAGE_VERTEX : u8 = 1;
    pub const  SHADER_STAGE_FRAGMENT : u8 = 2;
    pub const  SHADER_STAGE_COMPUTE : u8 = 4;
    pub const  SHADER_STAGE_VERTEXFRAGMENT : u8 = 3;

    pub const VERTEX_ATTR_POSITION               : u8 = 01;
    pub const VERTEX_ATTR_POSITION_D2            : u8 = 02;
    pub const VERTEX_ATTR_COLOR4                 : u8 = 03;
    pub const VERTEX_ATTR_UV                     : u8 = 04;
    pub const VERTEX_ATTR_NORMAL                 : u8 = 05;
    pub const VERTEX_ATTR_TANGENT                : u8 = 06;
    pub const VERTEX_ATTR_MATRICESINDICES        : u8 = 07;
    pub const VERTEX_ATTR_MATRICESWEIGHTS        : u8 = 08;
    pub const VERTEX_ATTR_MATRICESINDICESEXTRA   : u8 = 09;
    pub const VERTEX_ATTR_MATRICESWEIGHTSEXTRA   : u8 = 10;
    pub const VERTEX_ATTR_UV2                    : u8 = 11;
    pub const VERTEX_ATTR_UV3                    : u8 = 12;
    pub const VERTEX_ATTR_UV4                    : u8 = 13;
    pub const VERTEX_ATTR_UV5                    : u8 = 14;
    pub const VERTEX_ATTR_UV6                    : u8 = 15;
    pub const VERTEX_ATTR_CUSTOMVEC4A            : u8 = 16;
    pub const VERTEX_ATTR_CUSTOMVEC4B            : u8 = 17;
    pub const VERTEX_ATTR_CUSTOMVEC4C            : u8 = 18;
    pub const VERTEX_ATTR_CUSTOMVEC4D            : u8 = 19;
    // pub const VERTEX_ATTR_CUSTOMVEC2A            : u8 = 20;
    // pub const VERTEX_ATTR_CUSTOMVEC2B            : u8 = 21;
    pub const VERTEX_ATTR_INSWORLDROW1           : u8 = 22;
    pub const VERTEX_ATTR_INSWORLDROW2           : u8 = 23;
    pub const VERTEX_ATTR_INSWORLDROW3           : u8 = 24;
    pub const VERTEX_ATTR_INSWORLDROW4           : u8 = 25;
    pub const VERTEX_ATTR_INSCOLOR               : u8 = 26;
    pub const VERTEX_ATTR_INSTILLOFFSET1         : u8 = 27;
    pub const VERTEX_ATTR_INSTILLOFFSET2         : u8 = 28;
    pub const VERTEX_ATTR_INSCUSTOMVEC4A         : u8 = 29;
    pub const VERTEX_ATTR_INSCUSTOMVEC4B         : u8 = 30;
    pub const VERTEX_ATTR_INSCUSTOMVEC4C         : u8 = 31;
    pub const VERTEX_ATTR_INSCUSTOMVEC4D         : u8 = 32;
    pub const VERTEX_ATTR_MATRICESINDICES1       : u8 = 33;
    pub const VERTEX_ATTR_MATRICESWEIGHTS1       : u8 = 34;
    pub const VERTEX_ATTR_MATRICESINDICES2       : u8 = 35;
    pub const VERTEX_ATTR_MATRICESWEIGHTS2       : u8 = 36;
    pub const VERTEX_ATTR_MATRICESINDICESEXTRA2  : u8 = 37;
    pub const VERTEX_ATTR_MATRICESWEIGHTSEXTRA2  : u8 = 38;
    pub const VERTEX_ATTR_MATRICESINDICES3       : u8 = 39;
    pub const VERTEX_ATTR_MATRICESWEIGHTS3       : u8 = 40;
    pub const VERTEX_ATTR_MATRICESINDICESEXTRA3  : u8 = 41;
    pub const VERTEX_ATTR_MATRICESWEIGHTSEXTRA3  : u8 = 42;
    
    // 不支持
    // pub const VERTEX_FORMAT_BYTE                        : u16 = 0x1400;
    // pub const VERTEX_FORMAT_UNSIGNED_BYTE               : u16 = 0x1401;
    // pub const VERTEX_FORMAT_SHORT                       : u16 = 0x1402;
    // pub const VERTEX_FORMAT_UNSIGNED_SHORT              : u16 = 0x1403;
    // pub const VERTEX_FORMAT_U8X2                        : u16 = 01;
    pub const VERTEX_FORMAT_U8X4                        : u16 = 02;
    // pub const VERTEX_FORMAT_S8X2                        : u16 = 03;
    pub const VERTEX_FORMAT_S8X4                        : u16 = 04;
    pub const VERTEX_FORMAT_UNORM8X2                    : u16 = 05;
    pub const VERTEX_FORMAT_UNORM8X4                    : u16 = 06;
    pub const VERTEX_FORMAT_SNORM8X2                    : u16 = 07;
    pub const VERTEX_FORMAT_SNORM8X4                    : u16 = 08;
    pub const VERTEX_FORMAT_U16X2                       : u16 = 09;
    pub const VERTEX_FORMAT_U16X4                       : u16 = 10;
    pub const VERTEX_FORMAT_S16X2                       : u16 = 11;
    pub const VERTEX_FORMAT_S16X4                       : u16 = 12;
    pub const VERTEX_FORMAT_UNORM16X2                   : u16 = 13;
    pub const VERTEX_FORMAT_UNORM16X4                   : u16 = 14;
    pub const VERTEX_FORMAT_SNORM16X2                   : u16 = 15;
    pub const VERTEX_FORMAT_SNORM16X4                   : u16 = 16;
    pub const VERTEX_FORMAT_F32                         : u16 = 0x1406;
    pub const VERTEX_FORMAT_F32X2                       : u16 = 18;
    pub const VERTEX_FORMAT_F32X3                       : u16 = 19;
    pub const VERTEX_FORMAT_F32X4                       : u16 = 20;
    pub const VERTEX_FORMAT_U32                         : u16 = 0x1405;
    pub const VERTEX_FORMAT_U32X2                       : u16 = 22;
    pub const VERTEX_FORMAT_U32X3                       : u16 = 23;
    pub const VERTEX_FORMAT_U32X4                       : u16 = 24;
    pub const VERTEX_FORMAT_S32                         : u16 = 0x1404;
    pub const VERTEX_FORMAT_S32X2                       : u16 = 26;
    pub const VERTEX_FORMAT_S32X3                       : u16 = 27;
    pub const VERTEX_FORMAT_S32X4                       : u16 = 28;
    pub const VERTEX_FORMAT_F64                         : u16 = 29;
    pub const VERTEX_FORMAT_F64X2                       : u16 = 30;
    pub const VERTEX_FORMAT_F64X3                       : u16 = 31;
    pub const VERTEX_FORMAT_F64X4                       : u16 = 32;
    pub const VERTEX_FORMAT_F16X2                       : u16 = 33;
    pub const VERTEX_FORMAT_F16X4                       : u16 = 34;

    pub const INSTANCE_ATTR_F00: u8 = 00 + 1;
    pub const INSTANCE_ATTR_F01: u8 = 01 + 1;
    pub const INSTANCE_ATTR_F02: u8 = 02 + 1;
    pub const INSTANCE_ATTR_F03: u8 = 03 + 1;
    pub const INSTANCE_ATTR_F04: u8 = 04 + 1;
    pub const INSTANCE_ATTR_F05: u8 = 05 + 1;
    pub const INSTANCE_ATTR_F06: u8 = 06 + 1;
    pub const INSTANCE_ATTR_F07: u8 = 07 + 1;
    pub const INSTANCE_ATTR_F08: u8 = 08 + 1;
    pub const INSTANCE_ATTR_F09: u8 = 09 + 1;
    pub const INSTANCE_ATTR_F10: u8 = 10 + 1;
    pub const INSTANCE_ATTR_F11: u8 = 11 + 1;
    pub const INSTANCE_ATTR_F12: u8 = 12 + 1;
    pub const INSTANCE_ATTR_F13: u8 = 13 + 1;
    pub const INSTANCE_ATTR_F14: u8 = 14 + 1;
    pub const INSTANCE_ATTR_F15: u8 = 15 + 1;
    
    pub const INSTANCE_ATTR_VEC4: u8    = 1;
    pub const INSTANCE_ATTR_VEC3: u8    = 2;
    pub const INSTANCE_ATTR_VEC2: u8    = 3;
    pub const INSTANCE_ATTR_FLOAT: u8   = 4;
    pub const INSTANCE_ATTR_UINT: u8    = 5;
    pub const INSTANCE_ATTR_SINT: u8    = 6;

    pub fn instance_attribute_vtype(val: f64) -> ECustomVertexType {
        match val as u8 {
            Self::INSTANCE_ATTR_VEC4 => { ECustomVertexType::Vec4 }
            Self::INSTANCE_ATTR_VEC3 => { ECustomVertexType::Vec3 }
            Self::INSTANCE_ATTR_VEC2 => { ECustomVertexType::Vec2 }
            Self::INSTANCE_ATTR_UINT => { ECustomVertexType::Uint }
            Self::INSTANCE_ATTR_SINT => { ECustomVertexType::Int }
            _ => { ECustomVertexType::Float }
        }
    }

    pub fn render_alignment(val: f64) -> ERenderAlignment {
        match val as u8 {
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
        match val as u8 {
            Self::SCALING_MODE_HIERARCHY => { EScalingMode::Hierarchy }
            Self::SCALING_MODE_LOCAL => { EScalingMode::Local }
            Self::SCALING_MODE_SHAPE => { EScalingMode::Shape }
            _ => { EScalingMode::Hierarchy }
        }
    }
    pub fn render_color_format(val: f64) -> ColorFormat {
        match val as u32 {
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
        match val as u32 {
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
        match val as u32 {
            ContextConstants::REPEAT => { EAddressMode::Repeat },
            ContextConstants::CLAMP_TO_EDGE => { EAddressMode::ClampToEdge },
            ContextConstants::MIRRORED_REPEAT => { EAddressMode::MirrorRepeat },
            Self::CLAMP_TO_BORDER => { EAddressMode::ClampToBorder },
            _ => { EAddressMode::ClampToEdge },
        }
    }
    pub fn anisotropy_clamp(val: f64) -> EAnisotropyClamp {
        match val as u8 {
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
        match val as u8 {
            Self::BORDER_TRANSPARENT_BLACK => { Some(SamplerBorderColor::TransparentBlack) },
            Self::BORDER_OPAQUE_BLACK => { Some(SamplerBorderColor::OpaqueBlack) },
            Self::BORDER_OPAQUE_WHITE => { Some(SamplerBorderColor::OpaqueWhite) },
            Self::BORDER_ZERO => { Some(SamplerBorderColor::Zero) },
            _ => { None },
        }
    }
    pub fn passtag(val: f64) -> PassTag {
        match val as PassTagValue {
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
        match val as u8 {
            Self::LIGHT_DIRECTION => { ELightType::Direct },
            Self::LIGHT_HEMISPHERE => { ELightType::Hemispheric },
            Self::LIGHT_POINT => { ELightType::Point },
            Self::LIGHT_SPOT => { ELightType::Spot },
            _ => { ELightType::Direct },
        }
    }
    pub fn default_texture(val: f64) -> EDefaultTexture {
        match val as u8 {
            Self::DEFAULT_TEXTURE_BLACK => { EDefaultTexture::Black },
            Self::DEFAULT_TEXTURE_WHITE => { EDefaultTexture::White },
            _ => { EDefaultTexture::Black },
        }
    }
    pub fn shader_stage(val: f64) -> EShaderStage {
        match val as u8 {
            Self::SHADER_STAGE_NONE => { EShaderStage::NONE },
            Self::SHADER_STAGE_VERTEX => { EShaderStage::VERTEX },
            Self::SHADER_STAGE_FRAGMENT => { EShaderStage::FRAGMENT },
            Self::SHADER_STAGE_COMPUTE => { EShaderStage::COMPUTE },
            Self::SHADER_STAGE_VERTEXFRAGMENT => { EShaderStage::VERTEXFRAGMENT },
            _ => { EShaderStage::NONE },
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
    pub fn texture_view_dimension(val: f64) -> wgpu::TextureViewDimension {
        match val as u8 {
            Self::TEXTURE_VIEW_DIMENSION_D1 => wgpu::TextureViewDimension::D1,
            Self::TEXTURE_VIEW_DIMENSION_D2 => wgpu::TextureViewDimension::D2,
            Self::TEXTURE_VIEW_DIMENSION_D2ARRAY => wgpu::TextureViewDimension::D2Array,
            Self::TEXTURE_VIEW_DIMENSION_CUBE => wgpu::TextureViewDimension::Cube,
            Self::TEXTURE_VIEW_DIMENSION_CUBEARRAY => wgpu::TextureViewDimension::CubeArray,
            Self::TEXTURE_VIEW_DIMENSION_D3 => wgpu::TextureViewDimension::D3,
            _ => wgpu::TextureViewDimension::D1,
        }
    }
    pub fn vertex_attr(attr: f64) -> EBuildinVertexAtribute {
        match attr as u8 {
            Self::VERTEX_ATTR_POSITION               => EBuildinVertexAtribute::Position,
            Self::VERTEX_ATTR_POSITION_D2            => EBuildinVertexAtribute::Position2D,
            Self::VERTEX_ATTR_COLOR4                 => EBuildinVertexAtribute::Color4,
            Self::VERTEX_ATTR_UV                     => EBuildinVertexAtribute::UV,
            Self::VERTEX_ATTR_NORMAL                 => EBuildinVertexAtribute::Normal,
            Self::VERTEX_ATTR_TANGENT                => EBuildinVertexAtribute::Tangent,
            Self::VERTEX_ATTR_MATRICESINDICES        => EBuildinVertexAtribute::MatricesIndices,
            Self::VERTEX_ATTR_MATRICESWEIGHTS        => EBuildinVertexAtribute::MatricesWeights,
            Self::VERTEX_ATTR_MATRICESINDICESEXTRA   => EBuildinVertexAtribute::MatricesIndicesExtra,
            Self::VERTEX_ATTR_MATRICESWEIGHTSEXTRA   => EBuildinVertexAtribute::MatricesWeightsExtra,
            Self::VERTEX_ATTR_UV2                    => EBuildinVertexAtribute::UV2,
            Self::VERTEX_ATTR_UV3                    => EBuildinVertexAtribute::UV3,
            Self::VERTEX_ATTR_UV4                    => EBuildinVertexAtribute::UV4,
            Self::VERTEX_ATTR_UV5                    => EBuildinVertexAtribute::UV5,
            Self::VERTEX_ATTR_UV6                    => EBuildinVertexAtribute::UV6,
            Self::VERTEX_ATTR_INSWORLDROW1           => EBuildinVertexAtribute::InsWorldRow1,
            Self::VERTEX_ATTR_INSWORLDROW2           => EBuildinVertexAtribute::InsWorldRow2,
            Self::VERTEX_ATTR_INSWORLDROW3           => EBuildinVertexAtribute::InsWorldRow3,
            Self::VERTEX_ATTR_INSWORLDROW4           => EBuildinVertexAtribute::InsWorldRow4,
            _  => EBuildinVertexAtribute::MatricesWeightsExtra,
        }
    }
    // pub fn instance_attr_float(val: f64) -> InstanceFloatType {
    //     match val as u8 {
    //         Self::INSTANCE_ATTR_F00 => EInstanceFloatType::F00 ,
    //         Self::INSTANCE_ATTR_F01 => EInstanceFloatType::F01 ,
    //         Self::INSTANCE_ATTR_F02 => EInstanceFloatType::F02 ,
    //         Self::INSTANCE_ATTR_F03 => EInstanceFloatType::F03 ,
    //         Self::INSTANCE_ATTR_F04 => EInstanceFloatType::F04 ,
    //         Self::INSTANCE_ATTR_F05 => EInstanceFloatType::F05 ,
    //         Self::INSTANCE_ATTR_F06 => EInstanceFloatType::F06 ,
    //         Self::INSTANCE_ATTR_F07 => EInstanceFloatType::F07 ,
    //         Self::INSTANCE_ATTR_F08 => EInstanceFloatType::F08 ,
    //         Self::INSTANCE_ATTR_F09 => EInstanceFloatType::F09 ,
    //         Self::INSTANCE_ATTR_F10 => EInstanceFloatType::F10 ,
    //         Self::INSTANCE_ATTR_F11 => EInstanceFloatType::F11 ,
    //         Self::INSTANCE_ATTR_F12 => EInstanceFloatType::F12 ,
    //         Self::INSTANCE_ATTR_F13 => EInstanceFloatType::F13 ,
    //         Self::INSTANCE_ATTR_F14 => EInstanceFloatType::F14 ,
    //         Self::INSTANCE_ATTR_F15 => EInstanceFloatType::F15 ,
    //         _ => EInstanceFloatType::F15,
    //     }
    // }
    pub fn vertex_format(val: f64) -> wgpu::VertexFormat {
        match val as u16 {
            // Self::VERTEX_FORMAT_BYTE                        => wgpu::VertexFormat::Uint8,
            // Self::VERTEX_FORMAT_UNSIGNED_BYTE               => wgpu::VertexFormat::Uint8,
            // Self::VERTEX_FORMAT_SHORT                       => wgpu::VertexFormat::Uint8,
            // Self::VERTEX_FORMAT_UNSIGNED_SHORT              => wgpu::VertexFormat::Uint8,
            // Self::VERTEX_FORMAT_U8X2                        => wgpu::VertexFormat::Uint8x2,
            Self::VERTEX_FORMAT_U8X4                        => wgpu::VertexFormat::Uint8x4,
            // Self::VERTEX_FORMAT_S8X2                        => wgpu::VertexFormat::Sint8x2,
            Self::VERTEX_FORMAT_S8X4                        => wgpu::VertexFormat::Sint8x4,
            // Self::VERTEX_FORMAT_UNORM8X2                    => wgpu::VertexFormat::Unorm8x2,
            Self::VERTEX_FORMAT_UNORM8X4                    => wgpu::VertexFormat::Unorm8x4,
            // Self::VERTEX_FORMAT_SNORM8X2                    => wgpu::VertexFormat::Snorm8x2,
            Self::VERTEX_FORMAT_SNORM8X4                    => wgpu::VertexFormat::Snorm8x4,
            Self::VERTEX_FORMAT_U16X2                       => wgpu::VertexFormat::Uint16x2,
            Self::VERTEX_FORMAT_U16X4                       => wgpu::VertexFormat::Uint16x4,
            Self::VERTEX_FORMAT_S16X2                       => wgpu::VertexFormat::Sint16x2,
            Self::VERTEX_FORMAT_S16X4                       => wgpu::VertexFormat::Sint16x4,
            Self::VERTEX_FORMAT_UNORM16X2                   => wgpu::VertexFormat::Uint16x2,
            Self::VERTEX_FORMAT_UNORM16X4                   => wgpu::VertexFormat::Uint16x4,
            Self::VERTEX_FORMAT_SNORM16X2                   => wgpu::VertexFormat::Sint16x2,
            Self::VERTEX_FORMAT_SNORM16X4                   => wgpu::VertexFormat::Sint16x4,
            Self::VERTEX_FORMAT_F32                         => wgpu::VertexFormat::Float32,
            Self::VERTEX_FORMAT_F32X2                       => wgpu::VertexFormat::Float32x2,
            Self::VERTEX_FORMAT_F32X3                       => wgpu::VertexFormat::Float32x3,
            Self::VERTEX_FORMAT_F32X4                       => wgpu::VertexFormat::Float32x4,
            Self::VERTEX_FORMAT_U32                         => wgpu::VertexFormat::Uint32,
            Self::VERTEX_FORMAT_U32X2                       => wgpu::VertexFormat::Uint32x2,
            Self::VERTEX_FORMAT_U32X3                       => wgpu::VertexFormat::Uint32x3,
            Self::VERTEX_FORMAT_U32X4                       => wgpu::VertexFormat::Uint32x4,
            Self::VERTEX_FORMAT_S32                         => wgpu::VertexFormat::Sint32,
            Self::VERTEX_FORMAT_S32X2                       => wgpu::VertexFormat::Sint32x2,
            Self::VERTEX_FORMAT_S32X3                       => wgpu::VertexFormat::Sint32x3,
            Self::VERTEX_FORMAT_S32X4                       => wgpu::VertexFormat::Sint32x4,
            Self::VERTEX_FORMAT_F64                         => wgpu::VertexFormat::Float64,
            Self::VERTEX_FORMAT_F64X2                       => wgpu::VertexFormat::Float64x2,
            Self::VERTEX_FORMAT_F64X3                       => wgpu::VertexFormat::Float64x3,
            Self::VERTEX_FORMAT_F64X4                       => wgpu::VertexFormat::Float64x4,
            Self::VERTEX_FORMAT_F16X2                       => wgpu::VertexFormat::Float16x2,
            Self::VERTEX_FORMAT_F16X4                       => wgpu::VertexFormat::Float16x4,
            _ => { wgpu::VertexFormat::Float32x4 }
        }
    }
}