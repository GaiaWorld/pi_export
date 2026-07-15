use std::{mem::transmute, ops::{Deref, DerefMut}};

// use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
#[cfg(any(feature = "record", feature = "replay"))]
use pi_export_base::record::{ERecord3D, ERecordCMD};
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_gltf2_load::{GLTF, PluginGLTF2Res, KeyGLTF};
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{prelude::*, PluginNodeMaterialSimple};
use pi_particle_system::{PluginParticleSystem, prelude::*};
use pi_scene_context::{prelude::*, shadow::PluginShadowGenerator};
use pi_trail_renderer::{PluginTrail, ActionSetTrailRenderer, ResTrailBuffer};
pub use pi_export_base::asset::Atom;
use pi_slotmap::Key;
use serde::{Deserialize, Serialize};

pub use pi_export_base::about_3d::engine::*;

use crate::{as_entity, as_f64};
pub use crate::commands::CommandsExchangeD3;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;
use pi_hal::*;

fn bit_ok_u32(bits: wgpu::DownlevelFlags, bit: wgpu::DownlevelFlags) -> u32 {
    if (bits & bit) == bit {
        1
    } else {
        0
    }
}
fn bit_ok(bits: wgpu::Features, bit: wgpu::Features) -> u32 {
    if (bits & bit) == bit {
        1
    } else {
        0
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_device_limis(app: &mut Engine, data: &mut [u32]) {
    pi_export_base::export::await_last_frame(app);
	let device = app.app_mut().world.get_resource::<PiRenderDevice>().unwrap();
    let limits = device.limits();
    let features = device.features();
    // let downlevelflags = device.0.downlevel();
    let mut i = 0;
    data[i] = bit_ok(features, wgpu::Features::ADDRESS_MODE_CLAMP_TO_ZERO );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::BGRA8UNORM_STORAGE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::BUFFER_BINDING_ARRAY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::CLEAR_TEXTURE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::CONSERVATIVE_RASTERIZATION );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::DEPTH32FLOAT_STENCIL8 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::DEPTH_CLIP_CONTROL );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::DUAL_SOURCE_BLENDING );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::FLOAT32_FILTERABLE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::INDIRECT_FIRST_INSTANCE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::MAPPABLE_PRIMARY_BUFFERS );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::MULTIVIEW );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::MULTI_DRAW_INDIRECT );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::MULTI_DRAW_INDIRECT_COUNT );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::PIPELINE_STATISTICS_QUERY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::POLYGON_MODE_LINE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::POLYGON_MODE_POINT );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::PUSH_CONSTANTS );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TIMESTAMP_QUERY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::RAY_TRACING_ACCELERATION_STRUCTURE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::RG11B10UFLOAT_RENDERABLE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_EARLY_DEPTH_TEST );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_F16 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_F64 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_I16 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_PRIMITIVE_INDEX );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_PRIMITIVE_INDEX );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SPIRV_SHADER_PASSTHROUGH );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_BINDING_ARRAY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_COMPRESSION_ASTC );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_COMPRESSION_ASTC_HDR );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_COMPRESSION_BC );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_COMPRESSION_ETC2 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_FORMAT_16BIT_NORM );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TEXTURE_FORMAT_NV12 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TIMESTAMP_QUERY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::VERTEX_ATTRIBUTE_64BIT );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::VERTEX_WRITABLE_STORAGE );

    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::ANISOTROPIC_FILTERING );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::BASE_VERTEX );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::COMPARISON_SAMPLERS );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::COMPUTE_SHADERS );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::CUBE_ARRAY_TEXTURES );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::DEPTH_BIAS_CLAMP );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::DEPTH_TEXTURE_AND_BUFFER_COPIES );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::FRAGMENT_STORAGE );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::FRAGMENT_WRITABLE_STORAGE );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::FULL_DRAW_INDEX_UINT32 );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::INDEPENDENT_BLEND );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::INDIRECT_EXECUTION );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::MULTISAMPLED_SHADING );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::NON_POWER_OF_TWO_MIPMAPPED_TEXTURES );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::READ_ONLY_DEPTH_STENCIL );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::SURFACE_VIEW_FORMATS );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::UNRESTRICTED_EXTERNAL_TEXTURE_COPIES );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::UNRESTRICTED_INDEX_BUFFER );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::VERTEX_STORAGE );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::VIEW_FORMATS );
    // i += 1;   data[i] = bit_ok_u32(downlevelflags.flags, wgpu::DownlevelFlags::WEBGPU_TEXTURE_FORMAT_SUPPORT );

    i += 1;   data[i] = limits.max_bind_groups ;
    i += 1;   data[i] = limits.max_bindings_per_bind_group ;
    i += 1;   data[i] = limits.max_non_sampler_bindings ;
    i += 1;   data[i] = limits.max_sampled_textures_per_shader_stage ;
    i += 1;   data[i] = limits.max_samplers_per_shader_stage ;
    i += 1;   data[i] = limits.max_storage_buffer_binding_size ;
    i += 1;   data[i] = limits.max_texture_array_layers ;
    i += 1;   data[i] = limits.max_texture_dimension_1d ;
    i += 1;   data[i] = limits.max_texture_dimension_2d ;
    i += 1;   data[i] = limits.max_texture_dimension_3d ;
    i += 1;   data[i] = limits.max_uniform_buffer_binding_size ;
    i += 1;   data[i] = limits.max_uniform_buffers_per_shader_stage ;
    i += 1;   data[i] = limits.max_vertex_attributes ;
    i += 1;   data[i] = limits.max_vertex_buffer_array_stride ;
    i += 1;   data[i] = limits.max_vertex_buffers ;
    i += 1;   data[i] = limits.min_storage_buffer_offset_alignment ;
    i += 1;   data[i] = limits.min_uniform_buffer_offset_alignment ;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_init_engine(app: &mut Engine) {
	// await_last_frame(app);
    // use pi_scene_shell::frame_time::PluginFrameTime;
    // println!("======== p3d_init_engine");

    // _init_engine(app);
}

pub fn _init_engine(app: &mut Engine) {
    if app.app_mut().world.get_resource::<AssetMgrConfigs>().is_none() {
        app.insert_resource(AssetMgrConfigs::default());
    }

    log::error!(">>>>> p3d_init_engine");

    PluginBundleDefault::add(app);
    app
        .add_plugins(PluginNodeMaterialSimple)
        .add_plugins(PluginShadowGenerator)
        .add_plugins(PluginShadowMapping)
        .add_plugins(PluginCubeBuilder)
        .add_plugins(PluginQuadBuilder)
        .add_plugins(PluginParticleSystem)
        .add_plugins(PluginGLTF2Res)
        .add_plugins(PluginTrail)
        ;

    app.add_systems(
        Update,
        sys_state_transform.in_set(ERunStageChap::StateCheck)
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_entity(app: &mut Engine, cmds: &mut CommandsExchangeD3) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());


    let id: Entity = CommandsExchangeD3::p3d_entity(app);

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.app_mut().world, as_f64(&id));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose(cmds: &mut CommandsExchangeD3, entity: f64) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::Dispose(entity));

    let entity: Entity = as_entity(entity);

    CommandsExchangeD3::p3d_dispose(cmds, entity);
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_dispose(cmds: &mut CommandsExchangeD3, scene: f64) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::SceneDispose(scene));

    let entity: Entity = as_entity(scene);

    CommandsExchangeD3::p3d_scene_dispose(cmds, entity);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_lighting_shadow_limit(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, 
    scene_max_direct_light_count: f64,
    scene_max_point_light_count: f64,
    scene_max_spot_light_count: f64,
    scene_max_hemi_light_count: f64,
    scene_max_shadow_count: f64,
    model_max_direct_light_count: f64,
    model_max_point_light_count: f64,
    model_max_spot_light_count: f64,
    model_max_hemi_light_count: f64,
) {
    #[cfg(feature = "replay")]
    return ;


	pi_export_base::export::await_last_frame(app);

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::LightingShadowLimit(
        scene_max_direct_light_count,
        scene_max_point_light_count,
        scene_max_spot_light_count,
        scene_max_hemi_light_count,
        scene_max_shadow_count,
        model_max_direct_light_count,
        model_max_point_light_count,
        model_max_spot_light_count,
        model_max_hemi_light_count
    ));

    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    CommandsExchangeD3::p3d_lighting_shadow_limit(&mut resource,  
        scene_max_direct_light_count as u16,
        scene_max_point_light_count as u16,
        scene_max_spot_light_count as u16,
        scene_max_hemi_light_count as u16,
        scene_max_shadow_count as u16,
        model_max_direct_light_count as u16,
        model_max_point_light_count as u16,
        model_max_spot_light_count as u16,
        model_max_hemi_light_count as u16,
    );
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_graphic(cmds: &mut CommandsExchangeD3, before: f64, after: f64, isdisconnect: bool) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::RenderGraphic(before, after, isdisconnect));

    let before: Entity = as_entity(before);
    let after: Entity = as_entity(after);

    CommandsExchangeD3::p3d_render_graphic(cmds, before, after, isdisconnect);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_world_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    param.world_transform.align();
    if let Ok(trans) = param.world_transform.get(&app.app_mut().world, entity) {
        let mut i = 0;
        trans.matrix.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_scene_state(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, result: &mut [f32]) -> bool {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    param.renderers.align();
    param.particlesystems.align();
    param.trails.align();
    param.animectxs.align();
    param.viewers.align();

    let param = param.deref_mut();
    let mut drawcalls = 0;
    let mut count_vertex = 0;
    param.renderers.iter(&app.world).for_each(|(idviewer, renderer)| {
        if let Ok((_, idscene)) = param.viewers.get(&app.world, idviewer.0) {
            if idscene.0 == entity {
                drawcalls += renderer.draws.list.len();
                count_vertex += renderer.vertexs;
            }
        }
    });

    let mut count_particlesys = 0;
    let mut count_particle = 0;
    param.particlesystems.iter(&app.app_mut().world).for_each(|(particles, idscene)| {
        if idscene.0 == entity {
            count_particlesys += 1;
            count_particle += particles.count();
        }
    });
    
    let mut count_trail = 0;
    let mut count_trail_point = 0;
    param.trails.iter(&app.app_mut().world).for_each(|(trail, idscene)| {
        if idscene.0 == entity {
            count_trail += 1;
            count_trail_point += trail.0.len();
        }
    });

    let count_animegroup = app.world.get_resource::<GlobalAnimationGroupsAmout>().unwrap().0.group_mgr.groups.len();
    
    result[0] = drawcalls as f32;
    result[1] = count_vertex as f32;
    result[2] = count_particlesys as f32;
    result[3] = count_particle as f32;
    result[4] = count_animegroup as f32;
    result[5] = count_trail as f32;
    result[6] = count_trail_point as f32;

    // result[5] = state.count_trail as f32;

    // if let Some(state) = cmds.state.scenes.get(&entity) {
    //     result[0] = state.count_mesh as f32;
    //     result[1] = state.count_drawobj as f32;
    //     result[2] = state.count_transform as f32;
    //     result[3] = state.count_particlesys as f32;
    //     result[4] = state.count_vertex as f32;
    //     result[5] = state.count_trail as f32;
    //     result[6] = state.count_material as f32;
    //     result[7] = state.count_animationgroup as f32;
    //     result[8] = state.count_geometry as f32;
    //     result[9] = state.count_mesh_ok as f32;
    //     true
    // } else {
    //     false
    // }
    true
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_engine_state(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, active: bool) {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::EngineState(active));

    let mut cmds = param.state.get_mut(&mut app.app_mut().world);
    CommandsExchangeD3::p3d_engine_state(&mut cmds, active);
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_engine_debug(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, debug: bool) {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::EngineDebug(debug));
    
    let mut cmds = param.state.get_mut(&mut app.app_mut().world);
    CommandsExchangeD3::p3d_engine_debug(&mut cmds, debug);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_performance_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	pi_export_base::export::await_last_frame(app);
    
    let cmds = param.state.get_mut(&mut app.app_mut().world);

    result[0] = cmds.performance.animation as f32;
    result[1] = cmds.performance.animationgroup as f32;
    result[2] = cmds.psperformance.total() as f32;
    result[3] = cmds.performance.worldmatrix as f32;
    result[4] = cmds.performance.culling as f32;
    result[5] = cmds.performance.gltfanaly as f32;
    result[6] = cmds.performance.drawobjs as f32;
    result[7] = cmds.performance.uniformupdate as f32;
    result[8] = cmds.performance.uniformbufferupdate as f32;
    result[9] = cmds.statetrail.calc_time as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_resource_state(app: &mut Engine, param: &mut ActionSetScene3D, commands: &mut CommandsExchangeD3, result: &mut [f32]) {
	pi_export_base::export::await_last_frame(app);
    
    let cmds = param.state.get_mut(&mut app.app_mut().world);

    result[ 0] = cmds.resource.count_bindbuffer as f32;
    result[ 1] = cmds.resource.count_bindgroup as f32;
    result[ 2] = cmds.resource.count_gltf as f32;
    result[ 3] = cmds.resource.count_imgtexture as f32;
    result[ 4] = cmds.resource.count_shader as f32;
    result[ 5] = cmds.resource.count_pipeline as f32;
    result[ 6] = (cmds.resource.size_geometrybuffer / 1024) as f32;
    result[ 7] = cmds.resource.count_geometrybuffer as f32;
    result[ 8] = cmds.resource.count_shadermeta as f32;
    result[ 9] = cmds.resource.mem_shadermeta as f32;
    result[10] = cmds.resource.mem_shader as f32;
    result[11] = cmds.resource.mem_bindbuffer as f32;
    result[12] = cmds.resource.mem_imgtexture as f32;

    result[13] = cmds.resource.count_material as f32;
    result[14] = cmds.resource.count_passset0 as f32;
    result[15] = cmds.resource.count_passset1 as f32;
    result[16] = cmds.resource.count_passset2 as f32;
    result[17] = cmds.resource.count_passbindgroups as f32;
    result[18] = cmds.resource.count_passshader as f32;
    result[19] = cmds.resource.count_passpipeline as f32;
    result[20] = cmds.resource.count_passdraw as f32;

    result[21] = cmds.resource.count_passmat as f32;
    result[22] = cmds.resource.count_passtexs as f32;
    result[23] = cmds.resource.count_vertex as f32;
    // result[24] = (app.app_mut().world.mem_size() / 1024) as f32;
    
    result[25] = cmds.resource.capcity_inscommon as f32;
    result[26] = cmds.resource.capcity_combindata as f32;
    result[27] = cmds.resource.capcity_transformcalc as f32;
    result[28] = commands.capacity(&param.acts.get(&app.app_mut().world)) as f32;

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_resource_memory(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f64]) -> f64 {
	pi_export_base::export::await_last_frame(app);
    
    let mut offset = 0;
    let cmds = param.resource.get_mut(&mut app.app_mut().world);
    offset = cmds.record(result, offset);
    let cmds = param.acts.get_mut(&mut app.app_mut().world);
    offset = cmds.record(result, offset);
    offset += 0; result[offset] = app.app_mut().world.mem_size() as f64;

    (offset + 1) as f64
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    pi_export_base::export::await_last_frame(app);
    param.materials.align();

    // let mut cmds = param.materials.get(&mut app.app_mut().world);
    let mut state = StateMaterial::default();
    param.materials.iter(&app.app_mut().world).for_each(|(meta, texs, _)| {
        state.count += 1;
        let texcount = meta.0.as_ref().unwrap().textures.len();
        let mut isready = false;
        if let Some(texs) = &texs.0 {
            if texcount == texs.textures.len() {
                isready = true;
            }
        } else if texcount == 0 {
            isready = false;
        }
        if isready { state.count_ready += 1; }

        if texcount == 0 {
            state.count_tex0 += 1;
            if isready { state.count_tex0_ready += 1; }
        }
        else if texcount == 1 {
            state.count_tex1 += 1;
            if isready { state.count_tex1_ready += 1; }
        }
        else if texcount == 2 {
            state.count_tex2 += 1;
            if isready { state.count_tex2_ready += 1; }
        }
        else if texcount == 3 {
            state.count_tex3 += 1;
            if isready { state.count_tex3_ready += 1; }
        }
        else if texcount == 4 {
            state.count_tex4 += 1;
            if isready { state.count_tex4_ready += 1; }
        }
        else if texcount == 5 {
            state.count_tex5 += 1;
            if isready { state.count_tex5_ready += 1; }
        }
        else if texcount == 6 {
            state.count_tex6 += 1;
            if isready { state.count_tex6_ready += 1; }
        }
    });

    result[ 0] = state.count as f32;
    result[ 1] = state.count_ready as f32;
    result[ 2] = state.count_tex0 as f32;
    result[ 3] = state.count_tex0_ready as f32;
    result[ 4] = state.count_tex1 as f32;
    result[ 5] = state.count_tex1_ready as f32;
    result[ 6] = state.count_tex2 as f32;
    result[ 7] = state.count_tex2_ready as f32;
    result[ 8] = state.count_tex3 as f32;
    result[ 9] = state.count_tex3_ready as f32;
    result[10] = state.count_tex4 as f32;
    result[11] = state.count_tex4_ready as f32;
    result[12] = state.count_tex5 as f32;
    result[13] = state.count_tex5_ready as f32;
    result[14] = state.count_tex6 as f32;
    result[15] = state.count_tex6_ready as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_state(app: &mut Engine, param: &mut ActionSetScene3D, scene: Option<f64>, result: &mut [f32]) {
    pi_export_base::export::await_last_frame(app);
    // let mut cmds = param.state.get_mut(&mut app.app_mut().world);
    param.meshes.align();

    let mut state = StateMesh::default();
    if let Some(scene) = scene {
        let scene = as_entity(scene);
        param.meshes.iter(&app.app_mut().world).for_each(|(idscene, enable, geoenable, instance, _)| {
            if idscene.0 == scene {
                state.abstructmesh += 1;
                if enable.0 { state.abstructenable_count += 1; }
                if let Some(geoenable) = geoenable { 
                    state.meshes += 1;
                    if geoenable.0 { state.geometry_enable += 1; }
                }
                if instance.is_some() { state.instances += 1; }
            }
        });
    }
    result[0] = state.abstructmesh as f32;
    result[1] = state.abstructenable_count as f32;
    result[2] = state.meshes as f32;
    result[3] = state.geometry_enable as f32;
    result[4] = state.instances as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_transform_state(app: &mut Engine, param: &mut ActionSetScene3D, scene: Option<f64>, result: &mut [f32]) {
	pi_export_base::export::await_last_frame(app);
    param.transforms.align();

    let mut state = StateTransform::default();
    let mut calc_local_time = 0;
    let mut calc_world_time = 0;
    if let Some(scene) = scene {
        let scene = as_entity(scene);
        param.transforms.iter(&app.app_mut().world).for_each(|(idscene, enable, globalenable)| {
            if idscene.0 == scene {
                state.count += 1;
                if enable.bool() { state.enable += 1; }
                if globalenable.0 { state.global_enable += 1; }
            }
        });

        let cmds = param.state.get(&mut app.app_mut().world);
        // calc_local_time   = cmds.statetransform.calc_local_time;
        calc_world_time         = cmds.performance.worldmatrix;
        state.max_level         = cmds.statetransform.max_level;
    }
    result[0] = state.count as f32;
    result[1] = state.enable as f32;
    result[2] = state.global_enable as f32;
    result[3] = calc_local_time as f32;
    result[4] = calc_world_time as f32;
    result[5] = state.max_level as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_transform_state(app: &mut Engine, param: &mut ActionSetScene3D, transform: Option<f64>, result: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);

    param.transforms.align();

    let mut state = StateTransform::default();
    if let Some(transform) = transform {
        let transform = as_entity(transform);
        if let Ok((idscene, enable, globalenable)) = param.transforms.get(&app.app_mut().world, transform) {
            result[0] = enable.0;
            result[1] = if globalenable.0 { 1. } else { 0. };
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_state(app: &mut Engine, param: &mut ActionSetScene3D, camera: Option<f64>, result: &mut [f32]) {
    pi_export_base::export::await_last_frame(app);
    // let mut cmds = param.state.get_mut(&mut app.app_mut().world);
    param.cameras.align();

    let mut state = StateCamera::default();
    if let Some(camera) = camera {
        let camera = as_entity(camera);
        if let Ok((_camera, includes, cullings)) = param.cameras.get(&app.app_mut().world, camera) {
            state.includes  = includes.0.len() as u32;
            state.culling   = cullings.0.len() as u32;
        }
        let cmds = param.state.get(&mut app.app_mut().world);
        state.culling_time = cmds.statecamera.culling_time;
    }

    result[0] = state.includes as f32;
    result[1] = state.culling as f32;
    result[2] = state.culling_time as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_loader_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.app_mut().world);

    result[ 0] = resource.imgtex_loader_state.image_count as f32;
    result[ 1] = resource.imgtex_loader_state.image_fail as f32;
    result[ 2] = resource.imgtex_loader_state.image_success as f32;
    result[ 3] = resource.imgtex_loader_state.image_waiting as f32;
    result[ 4] = resource.imgtex_loader_state.texview_count as f32;
    result[ 5] = resource.imgtex_loader_state.texview_fail as f32;
    result[ 6] = resource.imgtex_loader_state.texview_success as f32;
    result[ 7] = resource.imgtex_loader_state.texview_waiting as f32;
    result[ 8] = resource.imgtex_asset.len() as f32;
    result[ 9] = resource.imgtex_asset.size() as f32;
    result[10] = resource.imgtexview_asset.len() as f32;
    result[11] = resource.imgtexview_asset.size() as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_errors(app: &mut Engine, param: &mut ActionSetScene3D, info: &mut [u32], flag: bool) -> f64 {
    pi_export_base::export::await_last_frame(app);
    let count = info.len();
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    resource.error_record.1 = flag;
    let mut idx = 0;
    resource.error_record.drain(count).for_each(|v| {
        info[idx] = v;
        idx += 1;
    });

    idx as f64
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_global_state(app: &mut Engine, param: &mut ActionSetScene3D, val: bool) {

    // let cmds = param.state.get(&mut app.app_mut().world);
    // cmds.resource.debug = val;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_local_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);
    param.local_transform.align();

    if let Ok(trans) = param.local_transform.get(&app.app_mut().world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_view_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);
    param.view_matrix.align();

    if let Ok(trans) = param.view_matrix.get(&app.app_mut().world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_project_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);
    param.project_matrix.align();

    if let Ok(trans) = param.project_matrix.get(&app.app_mut().world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_viewproject_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);
    param.vp_matrix.align();

    if let Ok(trans) = param.vp_matrix.get(&app.app_mut().world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_gltf_val(cmds: &CommandsExchangeD3, item: &GLTFRes) -> String {
    if let Some(gltf) = cmds.gltfs().get(item.val()) {
        gltf.output.clone()
    } else {
        String::from("")
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_curve_id_bygltf(
    cmds: &CommandsExchangeD3,
    gltf: &GLTFRes,
    group_index: f64,
    channel_index: f64,
) -> f64 {
    #[cfg(feature = "replay")]
    return 0.;

    return CommandsExchangeD3::p3d_animation_curve_id_bygltf(cmds, gltf, group_index as usize, channel_index as usize);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, entity: f64, baseurl: &Atom, dyndesc: String) {
    #[cfg(feature = "replay")]
    return ;

    pi_export_base::export::await_last_frame(app);
    
    let baseurl = baseurl.deref().clone();

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateGltfLoad(entity, baseurl.clone(), dyndesc.clone()));

    let entity: Entity = as_entity(entity);
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    CommandsExchangeD3::p3d_create_gltf_load(&mut resource, entity, baseurl, dyndesc);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);

    let max = success.len();
    let mut item = resource.gltf2_loader.successquerys.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = resource.gltf2_loader.successquerys.pop();
    }
    success[idx] = 0.;
    
    let max = failed.len();
    let mut item = resource.gltf2_loader.failquerys.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = resource.gltf2_loader.failquerys.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, entity: f64) -> Option<GLTFRes> {
    #[cfg(feature = "replay")]
    return None;

	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    let result = CommandsExchangeD3::p3d_get_gltf(&mut resource, entity);
    *cmds.gltfs_mut() = resource.gltf2_records.0.clone();
    result
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose_gltf(cmds: &mut CommandsExchangeD3, entity: &GLTFRes) {
    #[cfg(feature = "replay")]
    return ;


    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::DisposeGltf(entity.clone()));

    CommandsExchangeD3::p3d_dispose_gltf(cmds, entity);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<String> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    let entity: Entity = as_entity(entity);
    resource.gltf2_loader.get_fail_reason(entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_image_load(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, url: &Atom, cancombine: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
	
    #[cfg(feature = "replay")]
    return 0.;
    
    pi_export_base::export::await_last_frame(app);

    let key = KeyImageTextureFrame { 
        url: url.deref().clone(),
        file: true,
        compressed,
        cancombine
    };
    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateImageLoad(key.clone()));

    let mut resource = param.resource.get_mut(&mut app.app_mut().world);

    let id = resource.imgtex_loader.create_load(key);

    unsafe { transmute(id) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_image_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
	// pi_export_base::export::await_last_frame(app);
    // let resource = param.resource.get_mut(&mut app.app_mut().world);

    // let max = success.len();
    // let mut item = resource.imgtex_loader.success_load.pop();
    let mut idx = 0;
    // while let Some(entity) = item {
    //     success[idx] = unsafe { transmute(entity) };

    //     idx += 1;
    //     if idx >= max {
    //         break;
    //     }

    //     item = resource.imgtex_loader.success_load.pop();
    // }
    success[idx] = 0.;
    
    // let max = success.len();
    // let mut item = resource.imgtex_loader.fails.pop();
    // let mut idx = 0;
    // while let Some(entity) = item {
    //     failed[idx] = unsafe { transmute(entity) };

    //     idx += 1;
    //     if idx >= max {
    //         break;
    //     }

    //     item = resource.imgtex_loader.fails.pop();
    // }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<ImageRes> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    if let Some(img) = resource.imgtex_loader.query_success(id) {
        Some(ImageRes::create(img))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<f64> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.app_mut().world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    if let Some(err) = resource.imgtex_loader.query_failed_reason(id) {
        Some(err as f64)
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_children(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [f64]) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let id = as_entity(id);

    param.tree.align();
    let tree = param.tree.get(&app.app_mut().world);
    let mut idx = 0;
    param.treedown.align();
    param.nodes.align();

    match param.treedown.get(&app.app_mut().world, id) {
        Ok(down) => tree.iter(down.head()).for_each(|child| {
            match param.nodes.get(&app.app_mut().world, child) {
                Ok((idscene, enable, genable, layer)) => {
                    let mut ntype = 1;
                    ntype |= if enable.bool()   { 2 } else { 0 };
                    ntype |= if genable.0       { 4 } else { 0 };
                    if param.nodesinstance.get(&app.app_mut().world, child).is_ok()       { ntype |= 8 };
                    if param.nodesmesh.get(&app.app_mut().world, child).is_ok()           { ntype |= 16 };
                    if param.nodescamera.get(&app.app_mut().world, child).is_ok()         { ntype |= 32 };
                    if param.nodesdirectlight.get(&app.app_mut().world, child).is_ok()    { ntype |= 64 };
                    if param.nodespointlight.get(&app.app_mut().world, child).is_ok()     { ntype |= 128 };
                    let id = as_f64(&child);
    
                    info[idx] = id; idx += 1;
                    info[idx] = ntype as f64; idx += 1;
                },
                Err(e) => {
                    log::error!("p3d_query_children {:?}", e);
                }
            }
        }),
        Err(e) => {
            log::error!("p3d_query_children treedown {:?}", e);
        }
    }

    idx as f64
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_mesh_info(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [u32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    param.model.align();
    param.pass.align();
    param.renderers.align();
    param.passactive.align();

    let id = as_entity(id);
    if let Ok((geoenable, passids)) = param.model.get(&app.app_mut().world, id) {
        let temp = passids.0;
        for i in 0..8 {
            if let Ok((idrenderer, idrmaterial, )) = param.pass.get(&app.app_mut().world, temp[i]) {
                info[i * 3 + 0] = 0;
                if let Ok((idviewer, _)) = param.renderers.get(&app.app_mut().world, idrenderer.0) {
                    info[i * 3 + 0] = idviewer.0.index() as u32;
                }
                info[i * 3 + 1] = idrenderer.0.index() as u32;
                let mut state: u32 = 0;
                if let Ok((bindgroups, shader, draw)) = param.passactive.get(&app.app_mut().world, temp[i]) {
                    // if let Some(set0) = set0 {
                        // if set0.val().is_some() { state |= 1 << 0; }
                    // }
                    // if let Some(set0) = set1 {
                        // if set1.val().is_some() { state |= 1 << 1; }
                    // }
                    // if let Some(set0) = set2 {
                        // if set2.val().is_some() { state |= 1 << 2; }
                    // }
                    // if let Some(set0) = bindgroups {
                        if bindgroups.val().is_some() { state |= 1 << 3; }
                    // }
                    // if let Some(set0) = shader {
                        if shader.val().is_some() { state |= 1 << 4; }
                    // }
                    // if let Some(set0) = draw {
                        if draw.val() { state |= 1 << 5; }
                    // }
                    // if let Some(set0) = set3 {
                        // if set3.val().is_some() { state |= 1 << 6; }
                    // }
                }
                info[i * 3 + 2] = state;
            }
        }
        info[8 * 3 + 0] = if geoenable.0 { 1 } else { 0 };

        true
    } else {
        false
    }
    
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_material_info(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [u32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    param.materials.align();
    let id = as_entity(id);
    if let Ok((
        meta, textures
        , slots
    )) = param.materials.get(&app.app_mut().world, id) {

        let mut idx = 0;
        if let Some(slots) = slots {
            slots.0.iter().for_each(|slot| {
                match &slot.url {
                    EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
                    EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
                    EKeyTexture::ImageFrame(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
                    EKeyTexture::SRT(key) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = *key as u32; },
                }
                idx += 1;
            })
        } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }

        true
    } else {
        false
    }
}

// pub fn p3d_query_uniforms(app: &mut Engine, param: &mut ActionSetScene3D, material: f64, key: &Atom, info: &mut [f64]) -> bool {
// 	pi_export_base::export::await_last_frame(app);
//     let material = as_entity(material);

//     if let Ok(bindeffect) = param.uniforms.get(&app.app_mut().world, material) {
//         if let Some(info) = &bindeffect.0 {
//             if let Some(offset) = info.offset(key.deref()) {
//                 if let Some(entity) = offset.entity() {
//                     info[0] = 1.; info[1] = as_f64(&entity);
//                 } else {
//                     info[0] = 0.;
//                 }
//                 true
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     } else {
//         false
//     }
// }

// pub fn p3d_query_animator_value(app: &mut Engine, param: &mut ActionSetScene3D, target: f64, info: &mut [f64]) -> bool {
// 	pi_export_base::export::await_last_frame(app);
//     let target = as_entity(target);

//     if let Ok(value) = param.animatorablefloat.get(&app.app_mut().world, target) {
//        true
//     } else {
//         false
//     }
// }

