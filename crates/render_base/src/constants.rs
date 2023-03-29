use std::ops::Deref;

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


