

use pi_bevy_render_plugin::constant::texture_sampler::*;
use js_proxy_gen_macro::pi_js_export;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
pub enum BlendFactor {
    /// 0.0
    Zero = 0,
    /// 1.0
    One = 1,
    /// S.component
    Src = 2,
    /// 1.0 - S.component
    OneMinusSrc = 3,
    /// S.alpha
    SrcAlpha = 4,
    /// 1.0 - S.alpha
    OneMinusSrcAlpha = 5,
    /// D.component
    Dst = 6,
    /// 1.0 - D.component
    OneMinusDst = 7,
    /// D.alpha
    DstAlpha = 8,
    /// 1.0 - D.alpha
    OneMinusDstAlpha = 9,
    /// min(S.alpha, 1.0 - D.alpha)
    SrcAlphaSaturated = 10,
    /// Constant
    Constant = 11,
    /// 1.0 - Constant
    OneMinusConstant = 12,
}
impl BlendFactor {
    pub fn val(&self) -> pi_bevy_render_plugin::constant::render_state::BlendFactor {
        match self {
            BlendFactor::Zero               => pi_bevy_render_plugin::constant::render_state::BlendFactor::Zero               ,
            BlendFactor::One                => pi_bevy_render_plugin::constant::render_state::BlendFactor::One                ,
            BlendFactor::Src                => pi_bevy_render_plugin::constant::render_state::BlendFactor::Src                ,
            BlendFactor::OneMinusSrc        => pi_bevy_render_plugin::constant::render_state::BlendFactor::OneMinusSrc        ,
            BlendFactor::SrcAlpha           => pi_bevy_render_plugin::constant::render_state::BlendFactor::SrcAlpha           ,
            BlendFactor::OneMinusSrcAlpha   => pi_bevy_render_plugin::constant::render_state::BlendFactor::OneMinusSrcAlpha   ,
            BlendFactor::Dst                => pi_bevy_render_plugin::constant::render_state::BlendFactor::Dst                ,
            BlendFactor::OneMinusDst        => pi_bevy_render_plugin::constant::render_state::BlendFactor::OneMinusDst        ,
            BlendFactor::DstAlpha           => pi_bevy_render_plugin::constant::render_state::BlendFactor::DstAlpha           ,
            BlendFactor::OneMinusDstAlpha   => pi_bevy_render_plugin::constant::render_state::BlendFactor::OneMinusDstAlpha   ,
            BlendFactor::SrcAlphaSaturated  => pi_bevy_render_plugin::constant::render_state::BlendFactor::SrcAlphaSaturated  ,
            BlendFactor::Constant           => pi_bevy_render_plugin::constant::render_state::BlendFactor::Constant           ,
            BlendFactor::OneMinusConstant   => pi_bevy_render_plugin::constant::render_state::BlendFactor::OneMinusConstant   ,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum BlendOperation {
    /// Src + Dst
    Add,
    /// Src - Dst
    Subtract,
    /// Dst - Src
    ReverseSubtract,
    /// min(Src, Dst)
    Min,
    /// max(Src, Dst)
    Max,
}
impl BlendOperation {
    pub fn val(&self) -> pi_bevy_render_plugin::constant::render_state::BlendOperation {
        match self {
            BlendOperation::Add                 => pi_bevy_render_plugin::constant::render_state::BlendOperation::Add            ,
            BlendOperation::Subtract            => pi_bevy_render_plugin::constant::render_state::BlendOperation::Subtract       ,
            BlendOperation::ReverseSubtract     => pi_bevy_render_plugin::constant::render_state::BlendOperation::ReverseSubtract,
            BlendOperation::Min                 => pi_bevy_render_plugin::constant::render_state::BlendOperation::Min            ,
            BlendOperation::Max                 => pi_bevy_render_plugin::constant::render_state::BlendOperation::Max            ,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum ColorFormat {
    RGBA,
    RGB,
}
impl ColorFormat {
    pub fn val(&self) -> wgpu::TextureFormat {
        match self {
            ColorFormat::RGBA => wgpu::TextureFormat::Rgba8Unorm,
            ColorFormat::RGB => wgpu::TextureFormat::Rgba8Unorm,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum DepthFormat {
    None,
    Depth32,
    Depth24Stencil8,
}
impl DepthFormat {
    pub fn val(&self) -> Option<wgpu::TextureFormat> {
        match self {
            DepthFormat::None => None,
            DepthFormat::Depth32 => Some(wgpu::TextureFormat::Depth32Float),
            DepthFormat::Depth24Stencil8 => Some(wgpu::TextureFormat::Depth24PlusStencil8),
        }
    }
    pub fn format(&self) -> DepthStencilFormat {
        match self {
            DepthFormat::None => DepthStencilFormat::None,
            DepthFormat::Depth32 => DepthStencilFormat::Depth32Float,
            DepthFormat::Depth24Stencil8 => DepthStencilFormat::Depth24PlusStencil8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EColorSpace {
    GAMMA,
    LINEAR,
}
impl EColorSpace {
    pub fn target_color_format(&self) -> wgpu::TextureFormat {
        match self {
            EColorSpace::GAMMA => wgpu::TextureFormat::Rgba8UnormSrgb,
            EColorSpace::LINEAR => wgpu::TextureFormat::Rgba8Unorm,
        }
    }
    pub fn format(&self) -> pi_bevy_render_plugin::constant::texture_sampler::ColorFormat {
        match self {
            EColorSpace::GAMMA => pi_bevy_render_plugin::constant::texture_sampler::ColorFormat::Rgba8UnormSrgb,
            EColorSpace::LINEAR => pi_bevy_render_plugin::constant::texture_sampler::ColorFormat::Rgba8Unorm,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum CompareFunction {
    None,
    /// Function never passes
    Never,
    /// Function passes if new value less than existing value
    Less,
    /// Function passes if new value is equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    Equal,
    /// Function passes if new value is less than or equal to existing value
    LessEqual,
    /// Function passes if new value is greater than existing value
    Greater,
    /// Function passes if new value is not equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    NotEqual,
    /// Function passes if new value is greater than or equal to existing value
    GreaterEqual,
    /// Function always passes
    Always,
}
impl CompareFunction {
    pub fn val(&self) -> Option<wgpu::CompareFunction> {
        match self {
            CompareFunction::None           => None,
            CompareFunction::Never          => Some(wgpu::CompareFunction::Never),
            CompareFunction::Less           => Some(wgpu::CompareFunction::Less),
            CompareFunction::Equal          => Some(wgpu::CompareFunction::Equal),
            CompareFunction::LessEqual      => Some(wgpu::CompareFunction::LessEqual),
            CompareFunction::Greater        => Some(wgpu::CompareFunction::Greater),
            CompareFunction::NotEqual       => Some(wgpu::CompareFunction::NotEqual),
            CompareFunction::GreaterEqual   => Some(wgpu::CompareFunction::GreaterEqual),
            CompareFunction::Always         => Some(wgpu::CompareFunction::Always),
        }
    }
}

pub fn sampler_desc(
    address_mode_u: EAddressMode,
    address_mode_v: EAddressMode,
    address_mode_w: EAddressMode,
    mag_filter: EFilterMode,
    min_filter: EFilterMode,
    mipmap_filter: EFilterMode,
    compare: CompareFunction,
    anisotropy_clamp: EAnisotropyClamp,
    border_color: SamplerBorderColor,
) -> pi_render::rhi::sampler::SamplerDesc {
    pi_render::rhi::sampler::SamplerDesc {
        address_mode_u: address_mode_u.val(),
        address_mode_v: address_mode_v.val(),
        address_mode_w: address_mode_w.val(),
        mag_filter: mag_filter.val(),
        min_filter: min_filter.val(),
        mipmap_filter: mipmap_filter.val(),
        compare: compare.val(),
        anisotropy_clamp: anisotropy_clamp.val(),
        border_color: border_color.val(),
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EFilterMode {
    Nearest,
    Linear,
}
impl EFilterMode {
    pub fn val(&self) -> pi_render::rhi::sampler::EFilterMode {
        match self {
            EFilterMode::Nearest            => pi_render::rhi::sampler::EFilterMode::Nearest  ,
            EFilterMode::Linear             => pi_render::rhi::sampler::EFilterMode::Linear   ,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAddressMode {
    /// Clamp the value to the edge of the texture
    ///
    /// -0.25 -> 0.0
    /// 1.25  -> 1.0
    ClampToEdge,
    /// Repeat the texture in a tiling fashion
    ///
    /// -0.25 -> 0.75
    /// 1.25 -> 0.25
    Repeat,
    /// Repeat the texture, mirroring it every repeat
    ///
    /// -0.25 -> 0.25
    /// 1.25 -> 0.75
    MirrorRepeat,
    /// Clamp the value to the border of the texture
    /// Requires feature [`Features::ADDRESS_MODE_CLAMP_TO_BORDER`]
    ///
    /// -0.25 -> border
    /// 1.25 -> border
    ClampToBorder,
}
impl EAddressMode {
    pub fn val(&self) -> pi_render::rhi::sampler::EAddressMode {
        match self {
            EAddressMode::ClampToEdge           => pi_render::rhi::sampler::EAddressMode::ClampToEdge  ,
            EAddressMode::Repeat                => pi_render::rhi::sampler::EAddressMode::Repeat       ,
            EAddressMode::MirrorRepeat          => pi_render::rhi::sampler::EAddressMode::MirrorRepeat ,
            EAddressMode::ClampToBorder         => pi_render::rhi::sampler::EAddressMode::ClampToBorder,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum SamplerBorderColor {
    None,
    /// [0, 0, 0, 0]
    TransparentBlack,
    /// [0, 0, 0, 1]
    OpaqueBlack,
    /// [1, 1, 1, 1]
    OpaqueWhite,

    /// On the Metal backend, this is equivalent to `TransparentBlack` for
    /// textures that have an alpha component, and equivalent to `OpaqueBlack`
    /// for textures that do not have an alpha component. On other backends,
    /// this is equivalent to `TransparentBlack`. Requires
    /// [`Features::ADDRESS_MODE_CLAMP_TO_ZERO`]. Not supported on the web.
    Zero,
}
impl SamplerBorderColor {
    pub fn val(&self) -> Option<wgpu::SamplerBorderColor> {
        match self {
            Self::TransparentBlack  => Some(wgpu::SamplerBorderColor::TransparentBlack  ),
            Self::OpaqueBlack       => Some(wgpu::SamplerBorderColor::OpaqueBlack       ),
            Self::OpaqueWhite       => Some(wgpu::SamplerBorderColor::OpaqueWhite       ),
            Self::Zero              => Some(wgpu::SamplerBorderColor::Zero              ),
            Self::None              => None              
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAnisotropyClamp {
    None,
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}
impl EAnisotropyClamp {
    pub fn val(&self) -> pi_render::rhi::sampler::EAnisotropyClamp {
        match self {
            EAnisotropyClamp::None      => pi_render::rhi::sampler::EAnisotropyClamp::None,
            EAnisotropyClamp::One       => pi_render::rhi::sampler::EAnisotropyClamp::One,
            EAnisotropyClamp::Two       => pi_render::rhi::sampler::EAnisotropyClamp::Two,
            EAnisotropyClamp::Four      => pi_render::rhi::sampler::EAnisotropyClamp::Four,
            EAnisotropyClamp::Eight     => pi_render::rhi::sampler::EAnisotropyClamp::Eight,
            EAnisotropyClamp::Sixteen   => pi_render::rhi::sampler::EAnisotropyClamp::Sixteen,
        }
    }
}
