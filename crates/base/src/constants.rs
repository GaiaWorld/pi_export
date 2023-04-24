
use pi_render::rhi::sampler::{EAddressMode, EFilterMode, EAnisotropyClamp, SamplerDesc};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
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

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
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

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
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
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
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
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
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
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct SamplerDescriptor {
    /// How to deal with out of bounds accesses in the u (i.e. x) direction
    address_mode_u: EAddressMode,
    /// How to deal with out of bounds accesses in the v (i.e. y) direction
    address_mode_v: EAddressMode,
    /// How to deal with out of bounds accesses in the w (i.e. z) direction
    address_mode_w: EAddressMode,
    /// How to filter the texture when it needs to be magnified (made larger)
    mag_filter: EFilterMode,
    /// How to filter the texture when it needs to be minified (made smaller)
    min_filter: EFilterMode,
    /// How to filter between mip map levels
    mipmap_filter: EFilterMode,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    compare: Option<CompareFunction>,
    /// Valid values: 1, 2, 4, 8, and 16.
    anisotropy_clamp: EAnisotropyClamp,
    /// Border color to use when address_mode is [`AddressMode::ClampToBorder`]
    border_color: Option<SamplerBorderColor>,
}

pub fn sampler_desc(desc: &SamplerDescriptor) -> SamplerDesc {
    SamplerDesc {
        address_mode_u: desc.address_mode_u,
        address_mode_v: desc.address_mode_v,
        address_mode_w: desc.address_mode_w,
        mag_filter: desc.mag_filter,
        min_filter: desc.min_filter,
        mipmap_filter: desc.mipmap_filter,
        compare: CompareFunction::val(desc.compare.as_ref()),
        anisotropy_clamp: desc.anisotropy_clamp,
        border_color: SamplerBorderColor::val(desc.border_color.as_ref()),
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl SamplerDescriptor {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn new() -> Self {
        Self {
            /// How to deal with out of bounds accesses in the u (i.e. x) direction
            address_mode_u: EAddressMode::ClampToEdge,
            /// How to deal with out of bounds accesses in the v (i.e. y) direction
            address_mode_v: EAddressMode::ClampToEdge,
            /// How to deal with out of bounds accesses in the w (i.e. z) direction
            address_mode_w: EAddressMode::ClampToEdge,
            /// How to filter the texture when it needs to be magnified (made larger)
            mag_filter: EFilterMode::Nearest,
            /// How to filter the texture when it needs to be minified (made smaller)
            min_filter: EFilterMode::Nearest,
            /// How to filter between mip map levels
            mipmap_filter: EFilterMode::Nearest,
            /// If this is enabled, this is a comparison sampler using the given comparison function.
            compare: None,
            /// Valid values: 1, 2, 4, 8, and 16.
            anisotropy_clamp: EAnisotropyClamp::One,
            /// Border color to use when address_mode is [`AddressMode::ClampToBorder`]
            border_color: None,
        }
    }
    
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn address_mode_u(&self) -> EAddressMode {
        self.address_mode_u
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_address_mode_u(&mut self, val: EAddressMode) {
        self.address_mode_u = val;
    }
    
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn address_mode_v(&self) -> EAddressMode {
        self.address_mode_v
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_address_mode_v(&mut self, val: EAddressMode) {
        self.address_mode_v = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn address_mode_w(&self) -> EAddressMode {
        self.address_mode_w
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_address_mode_w(&mut self, val: EAddressMode) {
        self.address_mode_w = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn mag_filter(&self) -> EFilterMode {
        self.mag_filter
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_mag_filter(&mut self, val: EFilterMode) {
        self.mag_filter = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn min_filter(&self) -> EFilterMode {
        self.min_filter
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_min_filter(&mut self, val: EFilterMode) {
        self.min_filter = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn mipmap_filter(&self) -> EFilterMode {
        self.mipmap_filter
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_mipmap_filter(&mut self, val: EFilterMode) {
        self.mipmap_filter = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn compare(&self) -> Option<CompareFunction> {
        self.compare
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_compare(&mut self, val: Option<CompareFunction>) {
        self.compare = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn border_color(&self) -> Option<SamplerBorderColor> {
        self.border_color
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_border_color(&mut self, val: Option<SamplerBorderColor>) {
        self.border_color = val;
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn anisotropy_clamp(&self) -> EAnisotropyClamp {
        self.anisotropy_clamp
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn set_anisotropy_clamp(&mut self, val: EAnisotropyClamp) {
        self.anisotropy_clamp = val;
    }
}
