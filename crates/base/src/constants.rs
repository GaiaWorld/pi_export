
use pi_render::rhi::sampler::{EAddressMode, EFilterMode, EAnisotropyClamp, SamplerDesc};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
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
    pub fn val(&self) -> wgpu::BlendFactor {
        match self {
            BlendFactor::Zero               => wgpu::BlendFactor::Zero               ,
            BlendFactor::One                => wgpu::BlendFactor::One                ,
            BlendFactor::Src                => wgpu::BlendFactor::Src                ,
            BlendFactor::OneMinusSrc        => wgpu::BlendFactor::OneMinusSrc        ,
            BlendFactor::SrcAlpha           => wgpu::BlendFactor::SrcAlpha           ,
            BlendFactor::OneMinusSrcAlpha   => wgpu::BlendFactor::OneMinusSrcAlpha   ,
            BlendFactor::Dst                => wgpu::BlendFactor::Dst                ,
            BlendFactor::OneMinusDst        => wgpu::BlendFactor::OneMinusDst        ,
            BlendFactor::DstAlpha           => wgpu::BlendFactor::DstAlpha           ,
            BlendFactor::OneMinusDstAlpha   => wgpu::BlendFactor::OneMinusDstAlpha   ,
            BlendFactor::SrcAlphaSaturated  => wgpu::BlendFactor::SrcAlphaSaturated  ,
            BlendFactor::Constant           => wgpu::BlendFactor::Constant           ,
            BlendFactor::OneMinusConstant   => wgpu::BlendFactor::OneMinusConstant   ,
        }
    }
}

#[wasm_bindgen]
pub enum TextureFormat {
    RGBA,
    RGB,
}
impl TextureFormat {
    pub fn val(&self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::RGBA => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::RGB => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
}

#[wasm_bindgen]
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
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum CompareFunction {
    /// Function never passes
    Never = 1,
    /// Function passes if new value less than existing value
    Less = 2,
    /// Function passes if new value is equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    Equal = 3,
    /// Function passes if new value is less than or equal to existing value
    LessEqual = 4,
    /// Function passes if new value is greater than existing value
    Greater = 5,
    /// Function passes if new value is not equal to existing value. When using
    /// this compare function, make sure to mark your Vertex Shader's `@builtin(position)`
    /// output as `@invariant` to prevent artifacting.
    NotEqual = 6,
    /// Function passes if new value is greater than or equal to existing value
    GreaterEqual = 7,
    /// Function always passes
    Always = 8,
}
impl CompareFunction {
    pub fn val(val: Option<&Self>) -> Option<wgpu::CompareFunction> {
        match val {
            Some(val) => {
                match val {
                    CompareFunction::Never          => Some(wgpu::CompareFunction::Never),
                    CompareFunction::Less           => Some(wgpu::CompareFunction::Less),
                    CompareFunction::Equal          => Some(wgpu::CompareFunction::Equal),
                    CompareFunction::LessEqual      => Some(wgpu::CompareFunction::LessEqual),
                    CompareFunction::Greater        => Some(wgpu::CompareFunction::Greater),
                    CompareFunction::NotEqual       => Some(wgpu::CompareFunction::NotEqual),
                    CompareFunction::GreaterEqual   => Some(wgpu::CompareFunction::GreaterEqual),
                    CompareFunction::Always         => Some(wgpu::CompareFunction::Always),
                }
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum SamplerBorderColor {
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
    pub fn val(val: Option<&Self>) -> Option<wgpu::SamplerBorderColor> {
        match val {
            Some(val) => {
                match val {
                    SamplerBorderColor::TransparentBlack    => Some(wgpu::SamplerBorderColor::TransparentBlack),
                    SamplerBorderColor::OpaqueBlack         => Some(wgpu::SamplerBorderColor::OpaqueBlack),
                    SamplerBorderColor::OpaqueWhite         => Some(wgpu::SamplerBorderColor::OpaqueWhite),
                    SamplerBorderColor::Zero                => Some(wgpu::SamplerBorderColor::Zero),
                }
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct SamplerDescriptor {
    /// How to deal with out of bounds accesses in the u (i.e. x) direction
    pub address_mode_u: EAddressMode,
    /// How to deal with out of bounds accesses in the v (i.e. y) direction
    pub address_mode_v: EAddressMode,
    /// How to deal with out of bounds accesses in the w (i.e. z) direction
    pub address_mode_w: EAddressMode,
    /// How to filter the texture when it needs to be magnified (made larger)
    pub mag_filter: EFilterMode,
    /// How to filter the texture when it needs to be minified (made smaller)
    pub min_filter: EFilterMode,
    /// How to filter between mip map levels
    pub mipmap_filter: EFilterMode,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    pub compare: Option<CompareFunction>,
    /// Valid values: 1, 2, 4, 8, and 16.
    pub anisotropy_clamp: EAnisotropyClamp,
    /// Border color to use when address_mode is [`AddressMode::ClampToBorder`]
    pub border_color: Option<SamplerBorderColor>,
}
impl SamplerDescriptor {
    pub fn val(&self) -> SamplerDesc {
        SamplerDesc {
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            compare: CompareFunction::val(self.compare.as_ref()),
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: SamplerBorderColor::val(self.border_color.as_ref()),
        }
    }
}
