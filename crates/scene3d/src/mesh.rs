

use std::{ops::Deref, mem::transmute};

use pi_scene_shell::prelude::*;
use pi_export_base::constants::ContextConstants;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
use pi_scene_context::prelude::*;

use crate::{constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity, as_f64, geometry::GeometryMeta};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_abstruct_mesh_enable(cmds: &mut CommandsExchangeD3, abstructmesh: f64, val: bool) {
//     // let abstructmesh: Entity = as_entity(abstructmesh);

//     // let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

//     // cmds.abstructmeshcmds_enable.push(OpsAbstructMeshEnable::ops(abstructmesh, val));
// }

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct VInstanceAttributes(bool, Vec<CustomVertexAttribute>);
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl VInstanceAttributes {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(instane_matrix: bool) -> Self {
        VInstanceAttributes(instane_matrix, vec![])
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_attribute(item: &mut VInstanceAttributes, key: &Atom, code: &Atom, foruniform: &Atom, vtype: f64) {
    let foruniform = if foruniform.as_str() != "" {
        Some(foruniform.deref().clone())
    } else { None };
    item.1.push(CustomVertexAttribute::new(key.deref().clone(), code.deref().clone(), EngineConstants::instance_attribute_vtype(vtype), foruniform))
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, instancestate: &VInstanceAttributes, instance_use_single_buffer: bool) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();
    let scene: Entity = as_entity(scene);

    cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
    let state = MeshInstanceState { instances: instancestate.1.clone(), instance_matrix: instancestate.0, use_single_instancebuffer: instance_use_single_buffer };
    // log::error!("Mesh: {:?}", instancestate);
    cmds.mesh_create.push(OpsMeshCreation::ops(scene, id, state));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_geometry(app: &mut Engine, cmds: &mut CommandsExchangeD3, mesh: f64, geometa: &GeometryMeta) -> f64 {
    let geo: Entity = app.world.entities().reserve_entity();
    let mesh: Entity = as_entity(mesh);
    // log::error!("MeshGeo: {:?}", geometa.0);

    cmds.geometry_create.push(OpsGeomeryCreate::ops(mesh, geo, geometa.0.clone(), geometa.1.clone()));

    as_f64(&geo)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_indexrange(cmds: &mut CommandsExchangeD3, mesh: f64, index_start: Option<f64>, index_end: Option<f64>) {
    let mesh: Entity = as_entity(mesh);

    if let (Some(index_start), Some(index_count)) = (index_start, index_end) {
        cmds.mesh_indexrange.push(OpsMeshRenderIndiceRange::ops(mesh, Some(index_start as u32), Some(index_count as u32)));
    } else {
        cmds.mesh_indexrange.push(OpsMeshRenderIndiceRange::ops(mesh, None, None));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_vertexrange(cmds: &mut CommandsExchangeD3, mesh: f64, vertex_start: Option<f64>, vertex_count: Option<f64>) {
    let mesh: Entity = as_entity(mesh);

    if let (Some(vertex_start), Some(vertex_count)) = (vertex_start, vertex_count) {
        cmds.mesh_vertexrange.push(OpsMeshRenderVertexRange::ops(mesh, Some(vertex_start as u32), Some(vertex_count as u32)));
    } else {
        cmds.mesh_vertexrange.push(OpsMeshRenderVertexRange::ops(mesh, None, None));
    }
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_mesh_instance_world_matrixs(
//     cmds: &mut CommandsExchangeD3, geo: f64,
//     data: &[f32], offset: f64, length: f64
// ) {
//     let geo: Entity = as_entity(geo);
//     let start = offset as usize;
//     let length = length as usize;
//     let end = length + start;
//     // let mut values: Vec<f32> = Vec::with_capacity(length);
//     // data[start..end].iter().for_each(|val| {
//     //     values.push(*val);
//     // });
//     let values = bytemuck::cast_slice(&data[start..end]).to_vec();

//     cmds.instance_ins_world_matrixs.push(OpsInstanceWorldMatrixs::ops(geo, values));
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_mesh_instance_colors(
//     cmds: &mut CommandsExchangeD3, geo: f64,
//     data: &[f32], offset: f64, length: f64
// ) {
//     let geo: Entity = as_entity(geo);
//     let start = offset as usize;
//     let length = length as usize;
//     let end = length + start;
//     // let mut values: Vec<f32> = Vec::with_capacity(length);
//     // data[start..end].iter().for_each(|val| {
//     //     values.push(*val);
//     // });
//     let values = bytemuck::cast_slice(&data[start..end]).to_vec();

//     cmds.instance_ins_colors.push(OpsInstanceColors::ops(geo, values));
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_mesh_instance_tilloffs(
//     cmds: &mut CommandsExchangeD3, geo: f64,
//     data: &[f32], offset: f64, length: f64
// ) {
//     let geo: Entity = as_entity(geo);
//     let start = offset as usize;
//     let length = length as usize;
//     let end = length + start;
//     // let mut values: Vec<f32> = Vec::with_capacity(length);
//     // data[start..end].iter().for_each(|val| {
//     //     values.push(*val);
//     // });
//     let values = bytemuck::cast_slice(&data[start..end]).to_vec();

//     cmds.instance_ins_tilloffs.push(OpsInstanceTilloffs::ops(geo, values));
// }


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
    let mesh: Entity = as_entity(mesh);

    let blend = ModelBlend {
        enable,
        src_color: ContextConstants::blend_factor(src_color),
        dst_color: ContextConstants::blend_factor(dst_color),
        src_alpha: ContextConstants::blend_factor(src_alpha),
        dst_alpha: ContextConstants::blend_factor(dst_alpha),
        opt_color: ContextConstants::blend_operation(opt_color),
        opt_alpha: ContextConstants::blend_operation(opt_alpha),
    };
    if let Some(pass) = pass {
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, EngineConstants::passtag(pass), blend));
    } else {
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_01, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_02, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_03, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_04, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_05, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_06, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_07, blend));
        cmds.mesh_blend.push(OpsRenderBlend::ops(mesh, PassTag::PASS_TAG_08, blend));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cull_mode(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_primitivestate.push(OpsPrimitiveState::ops(mesh, EngineConstants::passtag(pass), EPrimitiveState::CCullMode( ContextConstants::cull_mode(val) )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_frontface(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_primitivestate.push(OpsPrimitiveState::ops(mesh, EngineConstants::passtag(pass), EPrimitiveState::CFrontFace( ContextConstants::front_face(val) )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_topology(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_primitivestate.push(OpsPrimitiveState::ops(mesh, EngineConstants::passtag(pass), EPrimitiveState::Topology( ContextConstants::topolygon(val) )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_polygon_mode(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_primitivestate.push(OpsPrimitiveState::ops(mesh, EngineConstants::passtag(pass), EPrimitiveState::CPolygonMode( ContextConstants::polygon(val) )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_unclip_depth(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_primitivestate.push(OpsPrimitiveState::ops(mesh, EngineConstants::passtag(pass), EPrimitiveState::CUnClipDepth( val )));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cast_shadow(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_shadow.push(OpsMeshShadow::CastShadow(mesh, val));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_receive_shadow(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_shadow.push(OpsMeshShadow::ReceiveShadow(mesh, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_write(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: bool, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_depthstate.push(OpsDepthState::ops(mesh, EngineConstants::passtag(pass), EDepthState::Write(val)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_compare(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_depthstate.push(OpsDepthState::ops(mesh, EngineConstants::passtag(pass), EDepthState::Compare(ContextConstants::compare_function(val)) ));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_bias(
    cmds: &mut CommandsExchangeD3, mesh: f64, constant: f64, slope_scale: f64, clamp: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);

    let constant = (constant as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let slope_scale = (slope_scale as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let clamp = (clamp as f32 / DepthBiasState::BASE_CLAMP) as i32;

    cmds.mesh_depthstate.push(OpsDepthState::ops(mesh, EngineConstants::passtag(pass), EDepthState::Bias(DepthBiasState { constant, slope_scale, clamp } )));
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
    let mesh: Entity = as_entity(mesh);
    let compare = ContextConstants::compare_function(compare) ;
    let fail_op = ContextConstants::stencil_operation(fail_op) ;
    let depth_fail_op = ContextConstants::stencil_operation(depth_fail_op) ;
    let pass_op = ContextConstants::stencil_operation(pass_op) ;
    cmds.mesh_stencilstate.push(OpsStencilState::ops(mesh, EngineConstants::passtag(pass), EStencilState::Front(StencilFaceState{compare, fail_op, depth_fail_op, pass_op})));
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
    let mesh: Entity = as_entity(mesh);
    let compare = ContextConstants::compare_function(compare) ;
    let fail_op = ContextConstants::stencil_operation(fail_op) ;
    let depth_fail_op = ContextConstants::stencil_operation(depth_fail_op) ;
    let pass_op = ContextConstants::stencil_operation(pass_op) ;
    cmds.mesh_stencilstate.push(OpsStencilState::ops(mesh, EngineConstants::passtag(pass), EStencilState::Back(StencilFaceState{compare, fail_op, depth_fail_op, pass_op})));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_read(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_stencilstate.push(OpsStencilState::ops(mesh, EngineConstants::passtag(pass), EStencilState::Read(val as u32)));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_write(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64, pass: f64
) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_stencilstate.push(OpsStencilState::ops(mesh, EngineConstants::passtag(pass), EStencilState::Write(val as u32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bounding_box(
    cmds: &mut CommandsExchangeD3, mesh: f64,
    minx: f64, miny: f64, minz: f64,
    maxx: f64, maxy: f64, maxz: f64
) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_bounding.push(OpsMeshBounding::ops(mesh, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32)));
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
    let mesh: Entity = as_entity(mesh);
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
    cmds.mesh_boundingculling.push(OpsMeshBoundingCullingMode::ops(mesh, mode));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue(
    cmds: &mut CommandsExchangeD3, mesh: f64, group: f64, index: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_render_queue.push(OpsRenderQueue::ops(mesh, group as i32,index as i32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue_arr(
    cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    // let mesh: Entity = as_entity(mesh);    // cmds.meshcmds_render_queue.push(OpsRenderQueue::ops(mesh, group as i32,index as i32));

    let len = len as usize;
    let size = 3;
    let count = len / size;
    for i in 0..count {
        let mesh: Entity = as_entity(data[i * size + 0]);
        let group = data[i * size + 1];
        let index = data[i * size + 2];
        cmds.mesh_render_queue.push(OpsRenderQueue::ops(mesh, group as i32, index as i32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_alignment(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.mesh_render_alignment.push(OpsMeshRenderAlignment::ops(mesh, EngineConstants::render_alignment(val) ));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_scaling_mode(
    cmds: &mut CommandsExchangeD3, mesh: f64, val: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.abstructmesh_scaling_mode.push(OpsAbstructMeshScalingMode::ops(mesh, EngineConstants::scaling_mode(val) ));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity(
    cmds: &mut CommandsExchangeD3, mesh: f64, x: f64, y: f64, z: f64) {
    let mesh: Entity = as_entity(mesh);
    cmds.abstructmesh_velocity.push(OpsAbstructMeshVelocity::ops(mesh, x as f32, y as f32, z as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity_arr(
    cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {

    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let mesh: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1];
        let y = data[i * size + 2];
        let z = data[i * size + 3];
        cmds.abstructmesh_velocity.push(OpsAbstructMeshVelocity::ops(mesh, x as f32, y as f32, z as f32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_attribute_target_animation(
    cmds: &mut CommandsExchangeD3,
    abstructmesh: f64,
    group: f64,
    key: &Atom,
    curve_key: f64,
) {
    let target = as_entity(abstructmesh);
    let group = as_entity(group);
    let curve: u64 = unsafe { transmute(curve_key) };
    cmds.instance_targetanime.push(OpsTargetAnimationAttribute::ops(target, key.deref().clone(), group, curve));
}
