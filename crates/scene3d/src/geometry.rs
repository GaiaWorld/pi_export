
use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_export_base::{export::{Engine, Atom}, constants::{RenderFormat, DepthStencilFormat}};
use pi_scene_context::{
    prelude::*,
};
use pi_mesh_builder::{
    cube::CubeBuilder,
    quad::QuadBuilder,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct VBMeta(pub(crate) VertexBufferDesc);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl VBMeta {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(name: &Atom, start: Option<f64>, end: Option<f64>) -> Self {
        let range = if let (Some(start), Some(end)) = (start, end) {
            Some(Range { start: start as u64, end: end as u64 })
        } else {
            None
        };

        Self(VertexBufferDesc::vertices(KeyVertexBuffer::from(name.as_str()), range, vec![]))
    }
    // #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    // #[pi_js_export]
    // pub fn create_as_instance(name: Atom, start: Option<f64>, end: Option<f64>) -> Self {
    //     let range = if let (Some(start), Some(end)) = (start, end) {
    //         Some(Range { start: start as u64, end: end as u64 })
    //     } else {
    //         None
    //     };

    //     Self(VertexBufferDesc::vertices(KeyVertexBuffer::from(name.as_str()), range, vec![]))
    // }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum VertexDataKind {
    Position               ,
    Position2D             ,
    Color4                 ,
    UV                     ,
    Normal                 ,
    Tangent                ,
    MatricesIndices        ,
    MatricesWeights        ,
    MatricesIndicesExtra   ,
    MatricesWeightsExtra   ,
    UV2                    ,
    UV3                    ,
    UV4                    ,
    UV5                    ,
    UV6                    ,
    CustomVec4A            ,
    CustomVec4B            ,
    CustomVec3A            ,
    CustomVec3B            ,
    CustomVec2A            ,
    CustomVec2B            ,
    InsWorldRow1           ,
    InsWorldRow2           ,
    InsWorldRow3           ,
    InsWorldRow4           ,
    InsColor               ,
    InsTillOffset1         ,
    InsTillOffset2         ,
    InsCustomVec4A         ,
    InsCustomVec4B         ,
    InsCustomUVec4A        ,
    InsCustomIVec4B        ,

    MatricesIndices1       ,
    MatricesWeights1       ,

    MatricesIndices2       ,
    MatricesWeights2       ,
    MatricesIndicesExtra2  ,
    MatricesWeightsExtra2  ,

    MatricesIndices3       ,
    MatricesWeights3       ,
    MatricesIndicesExtra3  ,
    MatricesWeightsExtra3  ,
}
impl VertexDataKind {
    pub fn val(&self) -> EVertexDataKind {
        match self {
            VertexDataKind::Position => EVertexDataKind::Position,
            VertexDataKind::Position2D => EVertexDataKind::Position2D,
            VertexDataKind::Color4 => EVertexDataKind::Color4,
            VertexDataKind::UV => EVertexDataKind::UV,
            VertexDataKind::Normal => EVertexDataKind::Normal,
            VertexDataKind::Tangent => EVertexDataKind::Tangent,
            VertexDataKind::MatricesIndices => EVertexDataKind::MatricesIndices,
            VertexDataKind::MatricesWeights => EVertexDataKind::MatricesWeights,
            VertexDataKind::MatricesIndicesExtra => EVertexDataKind::MatricesIndicesExtra,
            VertexDataKind::MatricesWeightsExtra => EVertexDataKind::MatricesWeightsExtra,
            VertexDataKind::UV2 => EVertexDataKind::UV2,
            VertexDataKind::UV3 => EVertexDataKind::UV3,
            VertexDataKind::UV4 => EVertexDataKind::UV4,
            VertexDataKind::UV5 => EVertexDataKind::UV5,
            VertexDataKind::UV6 => EVertexDataKind::UV6,
            VertexDataKind::CustomVec4A => EVertexDataKind::CustomVec4A,
            VertexDataKind::CustomVec4B => EVertexDataKind::CustomVec4B,
            VertexDataKind::CustomVec3A => EVertexDataKind::CustomVec3A,
            VertexDataKind::CustomVec3B => EVertexDataKind::CustomVec3B,
            VertexDataKind::CustomVec2A => EVertexDataKind::CustomVec2A,
            VertexDataKind::CustomVec2B => EVertexDataKind::CustomVec2B,
            VertexDataKind::InsWorldRow1 => EVertexDataKind::InsWorldRow1,
            VertexDataKind::InsWorldRow2 => EVertexDataKind::InsWorldRow2,
            VertexDataKind::InsWorldRow3 => EVertexDataKind::InsWorldRow3,
            VertexDataKind::InsWorldRow4 => EVertexDataKind::InsWorldRow4,
            VertexDataKind::InsColor => EVertexDataKind::InsColor,
            VertexDataKind::InsTillOffset1 => EVertexDataKind::InsTillOffset1,
            VertexDataKind::InsTillOffset2 => EVertexDataKind::InsTillOffset2,
            VertexDataKind::InsCustomVec4A => EVertexDataKind::InsCustomVec4A,
            VertexDataKind::InsCustomVec4B => EVertexDataKind::InsCustomVec4B,
            VertexDataKind::InsCustomUVec4A => EVertexDataKind::InsCustomUVec4A,
            VertexDataKind::InsCustomIVec4B => EVertexDataKind::InsCustomIVec4B,
            VertexDataKind::MatricesIndices1 => EVertexDataKind::MatricesIndices1,
            VertexDataKind::MatricesWeights1 => EVertexDataKind::MatricesWeights1,
            VertexDataKind::MatricesIndices2 => EVertexDataKind::MatricesIndices2,
            VertexDataKind::MatricesWeights2 => EVertexDataKind::MatricesWeights2,
            VertexDataKind::MatricesIndicesExtra2 => EVertexDataKind::MatricesIndicesExtra2,
            VertexDataKind::MatricesWeightsExtra2 => EVertexDataKind::MatricesWeightsExtra2,
            VertexDataKind::MatricesIndices3 => EVertexDataKind::MatricesIndices3,
            VertexDataKind::MatricesWeights3 => EVertexDataKind::MatricesWeights3,
            VertexDataKind::MatricesIndicesExtra3 => EVertexDataKind::MatricesIndicesExtra3,
            VertexDataKind::MatricesWeightsExtra3 => EVertexDataKind::MatricesWeightsExtra3,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum VertexFormat {
    /// Two unsigned bytes (u8). `uvec2` in shaders.
    Uint8x2 = 0,
    /// Four unsigned bytes (u8). `uvec4` in shaders.
    Uint8x4 = 1,
    /// Two signed bytes (i8). `ivec2` in shaders.
    Sint8x2 = 2,
    /// Four signed bytes (i8). `ivec4` in shaders.
    Sint8x4 = 3,
    /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
    Unorm8x2 = 4,
    /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
    Unorm8x4 = 5,
    /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
    Snorm8x2 = 6,
    /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
    Snorm8x4 = 7,
    /// Two unsigned shorts (u16). `uvec2` in shaders.
    Uint16x2 = 8,
    /// Four unsigned shorts (u16). `uvec4` in shaders.
    Uint16x4 = 9,
    /// Two signed shorts (i16). `ivec2` in shaders.
    Sint16x2 = 10,
    /// Four signed shorts (i16). `ivec4` in shaders.
    Sint16x4 = 11,
    /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
    Unorm16x2 = 12,
    /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
    Unorm16x4 = 13,
    /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
    Snorm16x2 = 14,
    /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
    Snorm16x4 = 15,
    /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
    Float16x2 = 16,
    /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
    Float16x4 = 17,
    /// One single-precision float (f32). `float` in shaders.
    Float32 = 18,
    /// Two single-precision floats (f32). `vec2` in shaders.
    Float32x2 = 19,
    /// Three single-precision floats (f32). `vec3` in shaders.
    Float32x3 = 20,
    /// Four single-precision floats (f32). `vec4` in shaders.
    Float32x4 = 21,
    /// One unsigned int (u32). `uint` in shaders.
    Uint32 = 22,
    /// Two unsigned ints (u32). `uvec2` in shaders.
    Uint32x2 = 23,
    /// Three unsigned ints (u32). `uvec3` in shaders.
    Uint32x3 = 24,
    /// Four unsigned ints (u32). `uvec4` in shaders.
    Uint32x4 = 25,
    /// One signed int (i32). `int` in shaders.
    Sint32 = 26,
    /// Two signed ints (i32). `ivec2` in shaders.
    Sint32x2 = 27,
    /// Three signed ints (i32). `ivec3` in shaders.
    Sint32x3 = 28,
    /// Four signed ints (i32). `ivec4` in shaders.
    Sint32x4 = 29,
    /// One double-precision float (f64). `double` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64 = 30,
    /// Two double-precision floats (f64). `dvec2` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x2 = 31,
    /// Three double-precision floats (f64). `dvec3` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x3 = 32,
    /// Four double-precision floats (f64). `dvec4` in shaders. Requires VERTEX_ATTRIBUTE_64BIT features.
    Float64x4 = 33,
}

impl VertexFormat {
    pub fn val(&self) -> wgpu::VertexFormat {
        match self {
            VertexFormat::Uint8x2 => wgpu::VertexFormat::Uint8x2,
            VertexFormat::Uint8x4 => wgpu::VertexFormat::Uint8x4,
            VertexFormat::Sint8x2 => wgpu::VertexFormat::Sint8x2,
            VertexFormat::Sint8x4 => wgpu::VertexFormat::Sint8x4,
            VertexFormat::Unorm8x2 => wgpu::VertexFormat::Unorm8x2,
            VertexFormat::Unorm8x4 => wgpu::VertexFormat::Unorm8x4,
            VertexFormat::Snorm8x2 => wgpu::VertexFormat::Snorm8x2,
            VertexFormat::Snorm8x4 => wgpu::VertexFormat::Snorm8x4,
            VertexFormat::Uint16x2 => wgpu::VertexFormat::Uint16x2,
            VertexFormat::Uint16x4 => wgpu::VertexFormat::Uint16x4,
            VertexFormat::Sint16x2 => wgpu::VertexFormat::Sint16x2,
            VertexFormat::Sint16x4 => wgpu::VertexFormat::Sint16x4,
            VertexFormat::Unorm16x2 => wgpu::VertexFormat::Unorm16x2,
            VertexFormat::Unorm16x4 => wgpu::VertexFormat::Unorm16x4,
            VertexFormat::Snorm16x2 => wgpu::VertexFormat::Snorm16x2,
            VertexFormat::Snorm16x4 => wgpu::VertexFormat::Snorm16x4,
            VertexFormat::Float16x2 => wgpu::VertexFormat::Float16x2,
            VertexFormat::Float16x4 => wgpu::VertexFormat::Float16x4,
            VertexFormat::Float32 => wgpu::VertexFormat::Float32,
            VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Float32x3 =>wgpu::VertexFormat::Float32x3,
            VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
            VertexFormat::Uint32 => wgpu::VertexFormat::Uint32,
            VertexFormat::Uint32x2 => wgpu::VertexFormat::Uint32x2,
            VertexFormat::Uint32x3 => wgpu::VertexFormat::Uint32x3,
            VertexFormat::Uint32x4 => wgpu::VertexFormat::Uint32x4,
            VertexFormat::Sint32 => wgpu::VertexFormat::Sint32,
            VertexFormat::Sint32x2 => wgpu::VertexFormat::Sint32x2,
            VertexFormat::Sint32x3 => wgpu::VertexFormat::Sint32x3,
            VertexFormat::Sint32x4 => wgpu::VertexFormat::Sint32x4,
            VertexFormat::Float64 => wgpu::VertexFormat::Float64,
            VertexFormat::Float64x2 => wgpu::VertexFormat::Float64x2,
            VertexFormat::Float64x3 => wgpu::VertexFormat::Float64x3,
            VertexFormat::Float64x4 => wgpu::VertexFormat::Float64x4,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_vbmeta_attr(meta: &mut VBMeta, kind: VertexDataKind, format: VertexFormat) {
    meta.0.attrs.push(VertexAttribute { kind: kind.val(), format: format.val() });
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct GeometryMeta(pub(crate) Vec<VertexBufferDesc>, pub(crate) Option<IndicesBufferDesc>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl GeometryMeta {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_empty() -> Self {
        Self(vec![], None)
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_box() -> Self {
        Self(CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()))
    }
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create_quad() -> Self {
        Self(QuadBuilder::attrs_meta(), Some(QuadBuilder::indices_meta()))
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_vertex(geo: &mut GeometryMeta, vb: &VBMeta) {
    geo.0.push(vb.0.clone());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_set_indice(geo: &mut GeometryMeta, name: &Atom, start: Option<f64>, end: Option<f64>, as_u16: bool) {
    let range = if let (Some(start), Some(end)) = (start, end) {
        Some(Range { start: start as u64, end: end as u64 })
    } else {
        None
    };

    let ib = IndicesBufferDesc {
        format: if as_u16 { wgpu::IndexFormat::Uint16 } else { wgpu::IndexFormat::Uint32 },
        buffer_range: range,
        buffer: KeyVertexBuffer::from(name.as_str()),
    };
    geo.1 = Some(ib);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_vertex_buffer(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data: &[f32], length: f64) {

    let length = length as usize;
    let data = bytemuck::cast_slice::<f32, u8>(&data[0..length]).to_vec();
    
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if !ActionVertexBuffer::check(&cmds.geometrycmd.vb_mgr, KeyVertexBuffer::from(key.to_string())) {
        ActionVertexBuffer::create(&mut cmds.geometrycmd.vb_wait, KeyVertexBuffer::from(key.to_string()), data);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_indices_buffer(app: &mut Engine, param: &mut ActionSetScene3D, key: &Atom, data: &[u16], length: f64) {

    let length = length as usize;
    let data = bytemuck::cast_slice::<u16, u8>(&data[0..length]).to_vec();
    
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if !ActionVertexBuffer::check(&cmds.geometrycmd.vb_mgr, KeyVertexBuffer::from(key.to_string())) {
        // log::warn!("CubeBuilder::KEY_BUFFER_INDICES");
        ActionVertexBuffer::create_indices(&mut cmds.geometrycmd.vb_wait, KeyVertexBuffer::from(key.to_string()), data);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_instance(geo: &mut GeometryMeta) {
    geo.0.push(VertexBufferDesc::instance_world_matrix());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_instance_color(geo: &mut GeometryMeta) {
    geo.0.push(VertexBufferDesc::instance_color());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_geo_instance_tilloff(geo: &mut GeometryMeta) {
    geo.0.push(VertexBufferDesc::instance_tilloff());
}
