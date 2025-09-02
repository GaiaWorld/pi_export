

use std::{ops::Deref, mem::transmute};

use pi_mesh_builder::cube::CubeBuilder;
use pi_mesh_builder::quad::QuadBuilder;
use pi_scene_context::geometry::instance::EInstanceSortMode;
use pi_scene_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
pub use pi_export_base::export::{Engine, Atom};
use pi_scene_context::prelude::*;
use serde::{Deserialize, Serialize};
pub use pi_export_base::about_3d::mesh::*;

use crate::constants::EngineConstants;
pub use crate::commands::CommandsExchangeD3;
use crate::record::ERecordCMD;
pub use crate::{as_entity, as_f64, geometry::GeometryMeta};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_attribute(item: &mut VInstanceAttributes, key: &Atom, code: &Atom, foruniform: &Atom, vtype: f64) {
    let foruniform = if foruniform.as_str() != "" {
        Some(foruniform.deref().clone())
    } else { None };
    item.v1_mut().push(CustomVertexAttribute::new(key.deref().clone(), code.deref().clone(), EngineConstants::instance_attribute_vtype(vtype), foruniform))
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, instancestate: &VInstanceAttributes, instance_use_single_buffer: bool) -> f64 {

    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());

    let id: Entity = app.world.entities().reserve_entity();

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MESH(scene, as_f64(&id), instancestate.clone(), instance_use_single_buffer));

    let scene: Entity = as_entity(scene);
    CommandsExchangeD3::p3d_mesh(cmds, scene, id, instancestate, instance_use_single_buffer);

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_geometry(app: &mut Engine, cmds: &mut CommandsExchangeD3, mesh: f64, geometa: &GeometryMeta, geoid: Option<f64>) -> f64 {
    #[cfg(feature = "replay")]
    return ;


    let geoid: Entity = if let Some(geo) = geoid {

        as_entity(geo)
    } else {
        let result = app.world.entities().reserve_entity();

        #[cfg(feature = "record")]
        CommandsExchangeD3::record_create(&mut app.world, as_f64(&result));

        result
    };

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshGeometry(mesh, geometa.clone(), as_f64(&geoid)));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_geometry(cmds, mesh, geometa, geoid);

    as_f64(&geoid)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_indexrange(cmds: &mut CommandsExchangeD3, mesh: f64, index_start: Option<f64>, index_end: Option<f64>) {
    #[cfg(feature = "replay")]
    return ;

    let val = if let (Some(index_start), Some(index_count)) = (index_start, index_end) {
        EMeshValueStateModify::IndiceRange( Some((index_start as u32, index_count as u32)) )
    } else {
        EMeshValueStateModify::IndiceRange( None )
    };

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshValueState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_morphtargetinfluence(cmds: &mut CommandsExchangeD3, mesh: f64, v0: f64, v1: f64, v2: f64, v3: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshValueStateModify::MorphInfluence(v0 as f32, v1 as f32, v2 as f32, v3 as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshValueState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_vertexrange(cmds: &mut CommandsExchangeD3, mesh: f64, vertex_start: Option<f64>, vertex_count: Option<f64>) {
    #[cfg(feature = "replay")]
    return ;

    let val = if let (Some(vertex_start), Some(vertex_count)) = (vertex_start, vertex_count) {
        EMeshValueStateModify::VertexRange( Some((vertex_start as u32, vertex_count as u32)) )
    } else {
        EMeshValueStateModify::VertexRange(None)
    };

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshValueState(mesh, val));
    
    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_blend(
    cmds: &mut CommandsExchangeD3, mesh: f64, enable: bool,
    src_color: f64,
    dst_color: f64,
    src_alpha: f64,
    dst_alpha: f64,
    opt_color: f64,
    opt_alpha: f64,
    pass: Option<f64>,
) {
    #[cfg(feature = "replay")]
    return ;

    let blend = ModelBlend {
        enable,
        src_color: ContextConstants::blend_factor(src_color as u32),
        dst_color: ContextConstants::blend_factor(dst_color as u32),
        src_alpha: ContextConstants::blend_factor(src_alpha as u32),
        dst_alpha: ContextConstants::blend_factor(dst_alpha as u32),
        opt_color: ContextConstants::blend_operation(opt_color as u32),
        opt_alpha: ContextConstants::blend_operation(opt_alpha as u32),
    };

    #[cfg(feature = "record")]
    if let Some(pass) = pass {
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend( EngineConstants::passtag(pass), blend)) );

    } else {
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_01, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_02, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_03, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_04, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_05, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_06, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_07, blend)) );
        cmds.record(ERecordCMD::MeshRenderState(mesh, ERenderState::Blend(PassTag::PASS_TAG_08, blend)) );
    };


    let mesh: Entity = as_entity(mesh);
    if let Some(pass) = pass {
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh,  ERenderState::Blend( EngineConstants::passtag(pass), blend));
    } else {
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_01, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_02, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_03, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_04, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_05, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_06, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_07, blend));
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, ERenderState::Blend(PassTag::PASS_TAG_08, blend));
    };

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cull_mode(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    
    let val = ERenderState::PrimitiveState(EngineConstants::passtag(pass), EPrimitiveState::CCullMode( ContextConstants::cull_mode(val as u32) ));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_frontface(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::PrimitiveState( EngineConstants::passtag(pass), EPrimitiveState::CFrontFace( ContextConstants::front_face(val as u32) ));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_topology(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::PrimitiveState( EngineConstants::passtag(pass), EPrimitiveState::Topology( ContextConstants::topolygon(val as u32) ));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_polygon_mode(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::PrimitiveState( EngineConstants::passtag(pass), EPrimitiveState::CPolygonMode( ContextConstants::polygon(val as u32) ));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_unclip_depth(cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::PrimitiveState( EngineConstants::passtag(pass), EPrimitiveState::CUnClipDepth( val ));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cast_shadow(cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshStateModify::CastShadow(val);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_receive_shadow(cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshStateModify::ReceiveShadow(val);
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_write(cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::DepthState( EngineConstants::passtag(pass), EDepthState::Write(val));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_compare(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::DepthState( EngineConstants::passtag(pass), EDepthState::Compare(ContextConstants::compare_function(val as u32)) );

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_bias(cmds: &mut CommandsExchangeD3, mesh: f64, constant: f64, slope_scale: f64, clamp: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let constant = (constant as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let slope_scale = (slope_scale as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let clamp = (clamp as f32 / DepthBiasState::BASE_CLAMP) as i32;

    let val = ERenderState::DepthState( EngineConstants::passtag(pass), EDepthState::Bias(DepthBiasState { constant, slope_scale, clamp } ));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_front(
    cmds: &mut CommandsExchangeD3,
    mesh: f64, 
    compare: f64,
    fail_op: f64,
    depth_fail_op: f64,
    pass_op: f64,
    pass: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let compare = ContextConstants::compare_function(compare as u32) ;
    let fail_op = ContextConstants::stencil_operation(fail_op as u32) ;
    let depth_fail_op = ContextConstants::stencil_operation(depth_fail_op as u32) ;
    let pass_op = ContextConstants::stencil_operation(pass_op as u32) ;
    let val = EStencilState::Front(StencilFaceState{compare, fail_op, depth_fail_op, pass_op});

    let val = ERenderState::StencilState(EngineConstants::passtag(pass), EStencilState::Front(StencilFaceState{compare, fail_op, depth_fail_op, pass_op}));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_back(
    cmds: &mut CommandsExchangeD3,
    mesh: f64, 
    compare: f64,
    fail_op: f64,
    depth_fail_op: f64,
    pass_op: f64,
    pass: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let compare = ContextConstants::compare_function(compare as u32) ;
    let fail_op = ContextConstants::stencil_operation(fail_op as u32) ;
    let depth_fail_op = ContextConstants::stencil_operation(depth_fail_op as u32) ;
    let pass_op = ContextConstants::stencil_operation(pass_op as u32) ;

    let val = ERenderState::StencilState( EngineConstants::passtag(pass), EStencilState::Back(StencilFaceState{compare, fail_op, depth_fail_op, pass_op}));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_read(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::StencilState( EngineConstants::passtag(pass), EStencilState::Read(val as u32));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_write(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let val = ERenderState::StencilState( EngineConstants::passtag(pass), EStencilState::Write(val as u32));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bounding_box(
    cmds: &mut CommandsExchangeD3, mesh: f64,
    minx: f64, miny: f64, minz: f64,
    maxx: f64, maxy: f64, maxz: f64
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshBoundingBox(mesh, minx, miny, minz, maxx, maxy, maxz));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_bounding_box(cmds, mesh, minx, miny, minz, maxx, maxy, maxz);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
///
/// * `mode` = `1`: ECullingStrategy::Optimistic
/// * `mode` = `2`: ECullingStrategy::STANDARD
/// * `mode` = `_`: ECullingStrategy::None
pub fn p3d_mesh_bounding_cullingmode(
    cmds: &mut CommandsExchangeD3, mesh: f64,
    mode: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let mode = match mode as u8 {
        1 => {
            ECullingStrategy::Optimistic
        }
        2 => {
            ECullingStrategy::STANDARD
        }
        _ => {
            ECullingStrategy::None
        }
    };
    
    let val = EMeshStateModify::BoundingCullingMode(mode);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue(cmds: &mut CommandsExchangeD3, mesh: f64, group: f64, index: f64) {
    #[cfg(feature = "replay")]
    return ;

    
    let val = ERenderState::RenderQueue( RenderQueueSortParam { group: group as i32, index: index as i32 } );

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshRenderState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    #[cfg(feature = "replay")]
    return ;

    // let mesh: Entity = as_entity(mesh);    // cmds.meshcmds_render_queue.push(OpsRenderQueue::ops(mesh, group as i32,index as i32));

    let len = len as usize;
    let size = 3;
    let count = len / size;
    for i in 0..count {
        let mesh: f64 = data[i * size + 0];
        let group = data[i * size + 1];
        let index = data[i * size + 2];
    
        let val = ERenderState::RenderQueue( RenderQueueSortParam { group: group as i32, index: index as i32 }) ;

        #[cfg(feature = "record")]
        cmds.record(ERecordCMD::MeshRenderState(mesh, val));

        let mesh: Entity = as_entity(mesh);
        CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_alignment(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshStateModify::Alignment(EngineConstants::render_alignment(val)) ;
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_instance_sort_mode(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshStateModify::InstanceSortMode(EInstanceSortMode::from_u8(val as u8));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);

    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_scaling_mode(cmds: &mut CommandsExchangeD3, mesh: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshStateModify::ScalingMode(EngineConstants::scaling_mode(val));
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    
    CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity(cmds: &mut CommandsExchangeD3, mesh: f64, x: f64, y: f64, z: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EMeshValueStateModify::Velocity(x as f32, y as f32, z as f32);
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshValueState(mesh, val));

    let mesh: Entity = as_entity(mesh);
    
    CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    #[cfg(feature = "replay")]
    return ;


    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let mesh: f64 = data[i * size + 0];
        let x = data[i * size + 1];
        let y = data[i * size + 2];
        let z = data[i * size + 3];
        
        let val = EMeshValueStateModify::Velocity(x as f32, y as f32, z as f32);
        
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshValueState(mesh, val));
    
        let mesh: Entity = as_entity(mesh);
        CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_attribute_target_animation(
    cmds: &mut CommandsExchangeD3,
    abstructmesh: f64,
    group: f64,
    key: &Atom,
    curve_key: &str,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshAttributeTargetAnim(abstructmesh, group, key.deref().clone(), String::from(curve_key)));

    let target = as_entity(abstructmesh);
    let group = as_entity(group);
    let curve_key: u64 = pi_atom::Atom::from(curve_key).asset_u64();
    CommandsExchangeD3::p3d_attribute_target_animation(cmds, target, group, key.deref(), curve_key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_pose_matrix(cmds: &mut CommandsExchangeD3, mesh: f64, data: &[f32]) {
    #[cfg(feature = "replay")]
    return ;


    let val = data[0..16].to_vec();

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshPoseMatrix(mesh, val.clone()));
    
    let mesh: Entity = as_entity(mesh);
    CommandsExchangeD3::p3d_abstruct_pose_matrix(cmds, mesh, val);
}
