
use std::mem::transmute;

use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{RenderFormat, DepthStencilFormat, BlendFactor, BlendOperation, CullMode, FrontFace, PrimitiveTopology, PolygonMode, CompareFunction, StencilOperation, ERenderAlignment, EScalingMode}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64, geometry::GeometryMeta};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_enable(app: &mut Engine, param: &mut ActionSetScene3D, abstructmesh: f64, val: bool) {
    // let abstructmesh: Entity = as_entity(abstructmesh);

    // let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    // cmds.abstructmeshcmds.enable.push(OpsAbstructMeshEnable::ops(abstructmesh, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.transformcmds.tree.push(OpsTransformNodeParent::ops(id, scene));
    cmds.meshcmds.create.push(OpsMeshCreation::ops(scene, id, String::from("")));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_geometry(app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, geometa: &GeometryMeta) -> f64 {
    let geo: Entity = app.world.spawn_empty().id();
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.geometrycmd.create.push(OpsGeomeryCreate::ops(mesh, geo, geometa.0.clone(), geometa.1.clone()));

    as_f64(&geo)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_indexrange(app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, index_start: Option<f64>, index_count: Option<f64>) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if let (Some(index_start), Some(index_count)) = (index_start, index_count) {
        cmds.meshcmds.indexrange.push(OpsMeshRenderIndiceRange::ops(mesh, Some(index_start as u32), Some(index_count as u32)));
    } else {
        cmds.meshcmds.indexrange.push(OpsMeshRenderIndiceRange::ops(mesh, None, None));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_instance_world_matrixs(
    app: &mut Engine, param: &mut ActionSetScene3D, geo: f64,
    data: &[f32], offset: f64, length: f64
) {
    let geo: Entity = as_entity(geo);
    let start = offset as usize;
    let length = length as usize;
    let end = length + start;
    // let mut values: Vec<f32> = Vec::with_capacity(length);
    // data[start..end].iter().for_each(|val| {
    //     values.push(*val);
    // });
    let values = bytemuck::cast_slice(&data[start..end]).to_vec();

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.ins_world_matrixs.push(OpsInstanceWorldMatrixs::ops(geo, values));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_instance_colors(
    app: &mut Engine, param: &mut ActionSetScene3D, geo: f64,
    data: &[f32], offset: f64, length: f64
) {
    let geo: Entity = as_entity(geo);
    let start = offset as usize;
    let length = length as usize;
    let end = length + start;
    // let mut values: Vec<f32> = Vec::with_capacity(length);
    // data[start..end].iter().for_each(|val| {
    //     values.push(*val);
    // });
    let values = bytemuck::cast_slice(&data[start..end]).to_vec();

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.ins_colors.push(OpsInstanceColors::ops(geo, values));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_instance_tilloffs(
    app: &mut Engine, param: &mut ActionSetScene3D, geo: f64,
    data: &[f32], offset: f64, length: f64
) {
    let geo: Entity = as_entity(geo);
    let start = offset as usize;
    let length = length as usize;
    let end = length + start;
    // let mut values: Vec<f32> = Vec::with_capacity(length);
    // data[start..end].iter().for_each(|val| {
    //     values.push(*val);
    // });
    let values = bytemuck::cast_slice(&data[start..end]).to_vec();

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.instancemeshcmds.ins_tilloffs.push(OpsInstanceTilloffs::ops(geo, values));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_blend(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, enable: bool,
    src_color: BlendFactor,
    dst_color: BlendFactor,
    src_alpha: BlendFactor,
    dst_alpha: BlendFactor,
    opt_color: BlendOperation,
    opt_alpha: BlendOperation,
) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let blend = ModelBlend {
        enable,
        src_color: unsafe { transmute(src_color)},
        dst_color: unsafe { transmute(dst_color)},
        src_alpha: unsafe { transmute(src_alpha)},
        dst_alpha: unsafe { transmute(dst_alpha)},
        opt_color: unsafe { transmute(opt_color)},
        opt_alpha: unsafe { transmute(opt_alpha)},
    };
    cmds.meshcmds.blend.push(OpsRenderBlend::ops(mesh, blend));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cull_mode(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: CullMode) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.cullmode.push(OpsCullMode::ops(mesh, unsafe { transmute(val) }));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_frontface(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: FrontFace) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.frontface.push(OpsFrontFace::ops(mesh, unsafe { transmute(val) }));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_topology(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: PrimitiveTopology) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.topology.push(OpsTopology::ops(mesh, unsafe { transmute(val) }));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_polygon_mode(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: PolygonMode) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.polygonmode.push(OpsPolygonMode::ops(mesh, unsafe { transmute(val) }));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_unclip_depth(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: bool) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.unclip_depth.push(OpsUnClipDepth::ops(mesh, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_cast_shadow(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: bool) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.shadow.push(OpsMeshShadow::CastShadow(mesh, val));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_receive_shadow(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: bool) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.shadow.push(OpsMeshShadow::ReceiveShadow(mesh, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_write(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: bool) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.depth_write.push(OpsDepthWrite::ops(mesh, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_compare(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: CompareFunction) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.depth_compare.push(OpsDepthCompare::ops(mesh, unsafe { transmute(val) }));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_depth_bias(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, constant: f64, slope_scale: f64, clamp: f64) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let constant = (constant as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let slope_scale = (slope_scale as f32 / DepthBiasState::BASE_SLOPE_SCALE) as i32;
    let clamp = (clamp as f32 / DepthBiasState::BASE_CLAMP) as i32;

    cmds.meshcmds.depth_bias.push(OpsDepthBias::ops(mesh, constant, slope_scale, clamp));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_front(
    app: &mut Engine, param: &mut ActionSetScene3D,
    mesh: f64, 
    compare: CompareFunction,
    fail_op: StencilOperation,
    depth_fail_op: StencilOperation,
    pass_op: StencilOperation,
) {
    let mesh: Entity = as_entity(mesh);
    let compare = unsafe { transmute(compare) };
    let fail_op = unsafe { transmute(fail_op) };
    let depth_fail_op = unsafe { transmute(depth_fail_op) };
    let pass_op = unsafe { transmute(pass_op) };

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.stencil_front.push(OpsStencilFront::ops(mesh, compare, fail_op, depth_fail_op, pass_op));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_back(
    app: &mut Engine, param: &mut ActionSetScene3D,
    mesh: f64, 
    compare: CompareFunction,
    fail_op: StencilOperation,
    depth_fail_op: StencilOperation,
    pass_op: StencilOperation,
) {
    let mesh: Entity = as_entity(mesh);
    let compare = unsafe { transmute(compare) };
    let fail_op = unsafe { transmute(fail_op) };
    let depth_fail_op = unsafe { transmute(depth_fail_op) };
    let pass_op = unsafe { transmute(pass_op) };

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.stencil_back.push(OpsStencilBack::ops(mesh, compare, fail_op, depth_fail_op, pass_op));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_read(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: f64) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.stencil_read.push(OpsStencilRead::ops(mesh, val as u32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_stencil_write(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: f64
) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.stencil_write.push(OpsStencilWrite::ops(mesh, val as u32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, group: f64, index: f64) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.render_queue.push(OpsRenderQueue::ops(mesh, group as i32,index as i32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_queue_arr(
    app: &mut Engine, param: &mut ActionSetScene3D, data: &[f64], len: f64) {
    // let mesh: Entity = as_entity(mesh);
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    // cmds.meshcmds.render_queue.push(OpsRenderQueue::ops(mesh, group as i32,index as i32));

    let len = len as usize;
    let size = 3;
    let count = len / size;
    for i in 0..count {
        let mesh: Entity = as_entity(data[i * size + 0]);
        let group = data[i * size + 1];
        let index = data[i * size + 2];
        cmds.meshcmds.render_queue.push(OpsRenderQueue::ops(mesh, group as i32, index as i32));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_render_alignment(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: ERenderAlignment) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.meshcmds.render_alignment.push(OpsMeshRenderAlignment::ops(mesh, unsafe { transmute(val) }));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_scaling_mode(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, val: EScalingMode) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.abstructmeshcmds.scaling_mode.push(OpsAbstructMeshScalingMode::ops(mesh, unsafe { transmute(val) }));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity(
    app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, x: f64, y: f64, z: f64) {
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.abstructmeshcmds.velocity.push(OpsAbstructMeshVelocity::ops(mesh, x as f32, y as f32, z as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_abstruct_mesh_velocity_arr(
    app: &mut Engine, param: &mut ActionSetScene3D, data: &[f64], len: f64) {
        // let mesh: Entity = as_entity(mesh);
    
        let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
        // cmds.abstructmeshcmds.velocity.push(OpsAbstructMeshVelocity::ops(mesh, x as f32, y as f32, z as f32));

    let len = len as usize;
    let size = 4;
    let count = len / size;
    for i in 0..count {
        let mesh: Entity = as_entity(data[i * size + 0]);
        let x = data[i * size + 1];
        let y = data[i * size + 2];
        let z = data[i * size + 3];
        cmds.abstructmeshcmds.velocity.push(OpsAbstructMeshVelocity::ops(mesh, x as f32, y as f32, z as f32));
    }
}