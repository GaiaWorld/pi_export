use std::{mem::transmute, ops::Deref};

// use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_assets::asset::Handle;
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_export_base::export::await_last_frame;
use pi_gltf2_load::{GLTF, PluginGLTF2Res, KeyGLTF};
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{prelude::*, NodeMaterialBlocks, PluginNodeMaterial, PluginNodeMaterialSimple};
use pi_particle_system::{PluginParticleSystem, prelude::*};
use pi_scene_context::{prelude::*, shadow::PluginShadowGenerator};
use pi_particle_system::prelude::*;
use pi_trail_renderer::{PluginTrail, ActionSetTrailRenderer, ResTrailBuffer};
pub use pi_export_base::asset::Atom;
use pi_slotmap::Key;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ImageRes(Handle<pi_render::renderer::texture::ImageTexture>);

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
	let device = app.world.get_resource::<PiRenderDevice>().unwrap();
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
    i += 1;   data[i] = bit_ok(features, wgpu::Features::RAY_QUERY );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::RAY_TRACING_ACCELERATION_STRUCTURE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::RG11B10UFLOAT_RENDERABLE );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_EARLY_DEPTH_TEST );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_F16 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_F64 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_I16 );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_PRIMITIVE_INDEX );
    i += 1;   data[i] = bit_ok(features, wgpu::Features::SHADER_UNUSED_VERTEX_OUTPUT );
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
    i += 1;   data[i] = bit_ok(features, wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING );
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
    if app.world.get_resource::<AssetMgrConfigs>().is_none() {
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

#[derive(SystemParam)]
pub struct GlobalState<'w> {
    pub resource: Res<'w, pi_3d::StateResource>,
    pub performance: Res<'w, pi_scene_shell::prelude::Performance>,
    pub psperformance: Res<'w, pi_particle_system::prelude::ParticleSystemPerformance>,
    // pub statemesh: ResMut<'w, pi_scene_context::prelude::StateMesh>,
    pub statetransform: Res<'w, pi_scene_context::prelude::StateTransform>,
    pub statecamera: Res<'w, pi_scene_context::prelude::StateCamera>,
    pub statelight: Res<'w, pi_scene_context::prelude::StateLight>,
    // pub statecamera: ResMut<'w, pi_scene_context::prelude::StateCamera>,
    // pub statematerial: ResMut<'w, pi_scene_context::prelude::StateMaterial>,
    pub statetrail: Res<'w, pi_trail_renderer::StateTrail>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D {
    pub(crate) acts: SystemState<pi_3d::ActionSets<'static>>,
    pub(crate) resource: SystemState<pi_3d::ResourceSets<'static>>,
    pub(crate) state: SystemState<GlobalState<'static>>,
    pub(crate) tree: SystemState<EntityTree<'static>>,
    pub(crate) treedown: QueryState<&'static Down, (With<Enable>)>,
    pub(crate) world_transform: QueryState<&'static GlobalMatrix, ()>,
    pub(crate) local_transform: QueryState<&'static LocalMatrix, ()>,
    pub(crate) view_matrix: QueryState<&'static ViewerViewMatrix, ()>,
    pub(crate) project_matrix: QueryState<&'static ViewerProjectionMatrix, ()>,
    pub(crate) vp_matrix: QueryState<&'static ViewerTransformMatrix, ()>,
    pub(crate) meshes: QueryState<(&'static SceneID, &'static GlobalEnable, Option<&'static RenderGeometryEable>, Option<&'static InstanceMesh>, &'static AbstructMesh), ()>, // StateMeshQuery,
    pub(crate) materials: QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp, Option<&'static TextureKeyList>), ()>, // StateMaterialQuery,
    pub(crate) transforms: QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable), ()>, // StateTransformQuery,
    pub(crate) cameras: QueryState<(&'static Camera, &'static ModelList, &'static ModelListAfterCulling), ()>, // StateCameraQuery,
    pub(crate) renderers: QueryState<(&'static ViewerID, &'static Renderer), ()>,
    pub(crate) viewers: QueryState<(&'static ViewerActive, &'static SceneID), ()>,
    pub(crate) particlesystems: QueryState<(&'static ParticleIDs, &'static SceneID), ()>,
    pub(crate) animectxs: QueryState<&'static SceneAnimationContext, ()>,
    pub(crate) trails: QueryState<(&'static pi_trail_renderer::TrailPoints, &'static SceneID), ()>,
    pub(crate) model: QueryState<(&'static RenderGeometryEable, &'static PassIDs), ()>,
    pub(crate) pass: QueryState<(&'static PassRendererID, &'static PassMaterialID), ()>,
    pub(crate) passactive: QueryState<(&'static PassBindGroups, &'static PassShader, &'static PassDraw), ()>,
    pub(crate) nodes: QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable, &'static Layer), ()>, // StateTransformQuery,
    pub(crate) nodesinstance: QueryState<(&'static InstanceMesh), ()>,
    pub(crate) nodesmesh: QueryState<(&'static Mesh), ()>,
    pub(crate) nodescamera: QueryState<(&'static Camera), ()>,
    pub(crate) nodesdirectlight: QueryState<(&'static DirectLight), ()>,
    pub(crate) nodespointlight: QueryState<(&'static PointLight), ()>,
    pub(crate) collider: QueryState<(&'static SceneColliderPool, &'static SceneBoundingPool), ()>,
    
    // pub(crate) uniforms: QueryState<&'static BindEffect>,
    // pub(crate) animatorablefloat: QueryState<&'static AnimatorableFloat>,
    // pub(crate) animatorablevec2s: QueryState<&'static AnimatorableVec2>,
    // pub(crate) animatorablevec3s: QueryState<&'static AnimatorableVec3>,
    // pub(crate) animatorablevec4s: QueryState<&'static AnimatorableVec4>,
    // pub(crate) animatorableuints: QueryState<&'static AnimatorableUint>,
    // pub(crate) animatorablesints: QueryState<&'static AnimatorableSint>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(app: &mut Engine) -> Self {
		pi_export_base::export::await_last_frame(app);
        Self {
            acts: SystemState::<pi_3d::ActionSets>::new(&mut app.world),
            resource: SystemState::<pi_3d::ResourceSets>::new(&mut app.world),
            state: SystemState::<GlobalState>::new(&mut app.world),
            tree: SystemState::<EntityTree<'static>>::new(&mut app.world),
            treedown: app.world.query(),
            world_transform: app.world.query(),
            local_transform: app.world.query(),
            view_matrix: app.world.query(),
            project_matrix: app.world.query(),
            vp_matrix: app.world.query(),
            meshes: app.world.query(),
            materials: app.world.query(),
            transforms: app.world.query(),
            cameras: app.world.query(),
            renderers: app.world.query(),
            viewers: app.world.query(),
            particlesystems: app.world.query(),
            trails: app.world.query(),
            model: app.world.query(),
            pass: app.world.query(),
            nodes: app.world.query(),
            animectxs: app.world.query(),
            passactive: app.world.query(),
            nodesinstance: app.world.query(),
            nodesmesh: app.world.query(),
            nodescamera: app.world.query(),
            nodesdirectlight: app.world.query(),
            nodespointlight: app.world.query(),
            collider: app.world.query(),
            
            // uniforms: app.world.query(),
            // animatorablefloat: app.world.query(),
            // animatorablevec2s: app.world.query(),
            // animatorablevec3s: app.world.query(),
            // animatorablevec4s: app.world.query(),
            // animatorableuints: app.world.query(),
            // animatorablesints: app.world.query(),
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_entity(app: &mut Engine) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();
    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose(cmds: &mut CommandsExchangeD3, entity: f64) {
    let entity: Entity = as_entity(entity);

    cmds.obj_dispose.push(OpsDispose::ops(entity));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_dispose(cmds: &mut CommandsExchangeD3, scene: f64) {
    let entity: Entity = as_entity(scene);

    cmds.scene_dispose.push(OpsSceneDispose::ops(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_lighting_shadow_limit(app: &mut Engine, param: &mut ActionSetScene3D, 
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

	pi_export_base::export::await_last_frame(app);
    
    let mut resource = param.resource.get_mut(&mut app.world);

    resource.scene_lighting_limit.0.max_direct_light_count = scene_max_direct_light_count as u32;
    resource.scene_lighting_limit.0.max_point_light_count = scene_max_point_light_count as u32;
    resource.scene_lighting_limit.0.max_spot_light_count = scene_max_spot_light_count as u32;
    resource.scene_lighting_limit.0.max_hemi_light_count = scene_max_hemi_light_count as u32;
    
    resource.scene_shadow_limit.0.max_count = scene_max_shadow_count as u32;

    resource.model_lighting_limit.0.max_direct_light_count = model_max_direct_light_count as u32;
    resource.model_lighting_limit.0.max_point_light_count = model_max_point_light_count as u32;
    resource.model_lighting_limit.0.max_spot_light_count = model_max_spot_light_count as u32;
    resource.model_lighting_limit.0.max_hemi_light_count = model_max_hemi_light_count as u32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_graphic(cmds: &mut CommandsExchangeD3, before: f64, after: f64, isdisconnect: bool) {
    let before: Entity = as_entity(before);
    let after: Entity = as_entity(after);

    cmds.renderer_connect.push(OpsRendererConnect::ops(before, after, isdisconnect));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_world_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    param.world_transform.align(&app.world);
    if let Ok(trans) = param.world_transform.get(&app.world, entity) {
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
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    param.renderers.align(&app.world);
    param.particlesystems.align(&app.world);
    param.trails.align(&app.world);
    param.animectxs.align(&app.world);

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
    param.particlesystems.iter(&app.world).for_each(|(particles, idscene)| {
        if idscene.0 == entity {
            count_particlesys += 1;
            count_particle += particles.count();
        }
    });
    
    let mut count_trail = 0;
    let mut count_trail_point = 0;
    param.trails.iter(&app.world).for_each(|(trail, idscene)| {
        if idscene.0 == entity {
            count_trail += 1;
            count_trail_point += trail.0.len();
        }
    });

    let mut count_animegroup = 0;
    if let Ok(ctx) = param.animectxs.get(&app.world, entity) {
        count_animegroup += ctx.0.group_mgr.groups.len();
    }
    
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
pub fn p3d_query_performance_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	pi_export_base::export::await_last_frame(app);
    
    let cmds = param.state.get_mut(&mut app.world);

    result[0] = cmds.performance.animation as f32;
    result[1] = cmds.performance.animationgroup as f32;
    result[2] = cmds.psperformance.total() as f32;
    result[3] = cmds.statetransform.calc_world_time as f32;
    result[4] = cmds.statecamera.culling_time as f32 + cmds.statelight.culling_time as f32;
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
    
    let cmds = param.state.get_mut(&mut app.world);

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
    result[24] = (app.world.mem_size() / 1024) as f32;
    
    result[25] = cmds.resource.capcity_inscommon as f32;
    result[26] = cmds.resource.capcity_combindata as f32;
    result[27] = cmds.resource.capcity_transformcalc as f32;
    result[28] = commands.capacity(&param.acts.get(&app.world)) as f32;

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    
    param.materials.align(&app.world);

    // let mut cmds = param.materials.get(&mut app.world);
    let mut state = StateMaterial::default();
    param.materials.iter(&app.world).for_each(|(meta, texs, _)| {
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
    
    // let mut cmds = param.state.get_mut(&mut app.world);
    param.meshes.align(&app.world);

    let mut state = StateMesh::default();
    if let Some(scene) = scene {
        let scene = as_entity(scene);
        param.meshes.iter(&app.world).for_each(|(idscene, enable, geoenable, instance, _)| {
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
    param.transforms.align(&app.world);

    let mut state = StateTransform::default();
    if let Some(scene) = scene {
        let scene = as_entity(scene);
        param.transforms.iter(&app.world).for_each(|(idscene, enable, globalenable)| {
            if idscene.0 == scene {
                state.count += 1;
                if enable.bool() { state.enable += 1; }
                if globalenable.0 { state.global_enable += 1; }
            }
        });

        let cmds = param.state.get(&mut app.world);
        state.calc_local_time   = cmds.statetransform.calc_local_time;
        state.calc_world_time   = cmds.statetransform.calc_world_time;
        state.max_level         = cmds.statetransform.max_level;
    }
    result[0] = state.count as f32;
    result[1] = state.enable as f32;
    result[2] = state.global_enable as f32;
    result[3] = state.calc_local_time as f32;
    result[4] = state.calc_world_time as f32;
    result[5] = state.max_level as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_transform_state(app: &mut Engine, param: &mut ActionSetScene3D, transform: Option<f64>, result: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);

    param.transforms.align(&app.world);

    let mut state = StateTransform::default();
    if let Some(transform) = transform {
        let transform = as_entity(transform);
        if let Ok((idscene, enable, globalenable)) = param.transforms.get(&app.world, transform) {
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
    
    // let mut cmds = param.state.get_mut(&mut app.world);
    param.cameras.align(&app.world);

    let mut state = StateCamera::default();
    if let Some(camera) = camera {
        let camera = as_entity(camera);
        if let Ok((_camera, includes, cullings)) = param.cameras.get(&app.world, camera) {
            state.includes  = includes.0.len() as u32;
            state.culling   = cullings.0.len() as u32;
        }
        let cmds = param.state.get(&mut app.world);
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
    let resource = param.resource.get_mut(&mut app.world);

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
    let count = info.len();
    let mut resource = param.resource.get_mut(&mut app.world);
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

    // let cmds = param.state.get(&mut app.world);
    // cmds.resource.debug = val;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_local_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);
    param.local_transform.align(&app.world);

    if let Ok(trans) = param.local_transform.get(&app.world, entity) {
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
    param.view_matrix.align(&app.world);

    if let Ok(trans) = param.view_matrix.get(&app.world, entity) {
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
    param.project_matrix.align(&app.world);

    if let Ok(trans) = param.project_matrix.get(&app.world, entity) {
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
    param.vp_matrix.align(&app.world);

    if let Ok(trans) = param.vp_matrix.get(&app.world, entity) {
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
pub struct GLTFRes(Handle<GLTF>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_gltf_val(item: &GLTFRes) -> String {
    item.0.output.clone()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_curve_id_bygltf(
    gltf: &GLTFRes,
    group_index: f64,
    channel_index: f64,
) -> f64 {
    let key = gltf.0.key_anime_curve(group_index as usize, channel_index as usize);
    unsafe { transmute(key) }
}

pub fn gltf_particle_calculator(item: &GLTFRes, index: f64) -> Option<&Handle<ParticleSystemCalculatorID>> {
    let index = index as usize;
    item.0.particlesys_calculators.get(&index)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, baseurl: &Atom, dyndesc: String) {
    let resource = param.resource.get_mut(&mut app.world);

    let entity: Entity = as_entity(entity);

    let param = KeyGLTF { base_url: baseurl.deref().clone(), dyn_desc: pi_atom::Atom::from(dyndesc) };

    resource.gltf2_loader.create_load(entity, param);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    let max = success.len();
    let mut item = resource.gltf2_loader.success.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = resource.gltf2_loader.success.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = resource.gltf2_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = resource.gltf2_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<GLTFRes> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    let entity: Entity = as_entity(entity);
    if let Some(val) = resource.gltf2_loader.get_success(entity) {
        Some(GLTFRes(val))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<String> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    let entity: Entity = as_entity(entity);
    resource.gltf2_loader.get_fail_reason(entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_image_load(app: &mut Engine, param: &mut ActionSetScene3D, url: &Atom, srgb: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);

    let id = resource.imgtex_loader.create_load(KeyImageTexture { 
        url: url.deref().clone(),
        srgb,
        file: true,
        compressed,
        depth_or_array_layers: depth_or_array_layers as u8,
        useage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING
    });

    unsafe { transmute(id) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_image_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    let max = success.len();
    let mut item = resource.imgtex_loader.success_load.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = resource.imgtex_loader.success_load.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = resource.imgtex_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = resource.imgtex_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<ImageRes> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    if let Some(img) = resource.imgtex_loader.query_success(id) {
        Some(ImageRes(img))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<String> {
	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    resource.imgtex_loader.query_failed_reason(id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_children(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [f64]) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let id = as_entity(id);

    param.tree.align(&app.world);
    let tree = param.tree.get(&app.world);
    let mut idx = 0;
    param.treedown.align(&app.world);
    param.nodes.align(&app.world);

    match param.treedown.get(&app.world, id) {
        Ok(down) => tree.iter(down.head()).for_each(|child| {
            match param.nodes.get(&app.world, child) {
                Ok((idscene, enable, genable, layer)) => {
                    let mut ntype = 1;
                    ntype |= if enable.bool()   { 2 } else { 0 };
                    ntype |= if genable.0       { 4 } else { 0 };
                    if param.nodesinstance.get(&app.world, child).is_ok()       { ntype |= 8 };
                    if param.nodesmesh.get(&app.world, child).is_ok()           { ntype |= 16 };
                    if param.nodescamera.get(&app.world, child).is_ok()         { ntype |= 32 };
                    if param.nodesdirectlight.get(&app.world, child).is_ok()    { ntype |= 64 };
                    if param.nodespointlight.get(&app.world, child).is_ok()     { ntype |= 128 };
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
    param.model.align(&app.world);
    param.pass.align(&app.world);
    param.renderers.align(&app.world);
    param.passactive.align(&app.world);

    let id = as_entity(id);
    if let Ok((geoenable, passids)) = param.model.get(&app.world, id) {
        let temp = passids.0;
        for i in 0..8 {
            if let Ok((idrenderer, idrmaterial, )) = param.pass.get(&app.world, temp[i]) {
                info[i * 3 + 0] = 0;
                if let Ok((idviewer, _)) = param.renderers.get(&app.world, idrenderer.0) {
                    info[i * 3 + 0] = idviewer.0.index() as u32;
                }
                info[i * 3 + 1] = idrenderer.0.index() as u32;
                let mut state: u32 = 0;
                if let Ok((bindgroups, shader, draw)) = param.passactive.get(&app.world, temp[i]) {
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
    param.materials.align(&app.world);
    let id = as_entity(id);
    if let Ok((
        meta, textures
        , slots
    )) = param.materials.get(&app.world, id) {

        let mut idx = 0;
        if let Some(slots) = slots {
            slots.0.iter().for_each(|slot| {
                match &slot.url {
                    EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
                    EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
                    EKeyTexture::SRT(key) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = *key as u32; },
                }
                idx += 1;
            })
        } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot02 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot03 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot04 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;

        // if let Some(texs) = &textures.0 {
        //     if let Some(slot) = &texs.textures.0 {
        //         match &slot.0.0.key() {
        //             KeyTextureViewUsage::Tex(_, _) => todo!(),
        //             KeyTextureViewUsage::Image(_, _) => todo!(),
        //             KeyTextureViewUsage::Render(_, _) => todo!(),
        //             KeyTextureViewUsage::SRT(_, _, _) => todo!(),
        //             KeyTextureViewUsage::Temp(_, _) => todo!(),
        //         }
        //     } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // }

        true
    } else {
        false
    }
}

// pub fn p3d_query_uniforms(app: &mut Engine, param: &mut ActionSetScene3D, material: f64, key: &Atom, info: &mut [f64]) -> bool {
// 	pi_export_base::export::await_last_frame(app);
//     let material = as_entity(material);

//     if let Ok(bindeffect) = param.uniforms.get(&app.world, material) {
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

//     if let Ok(value) = param.animatorablefloat.get(&app.world, target) {
//        true
//     } else {
//         false
//     }
// }

fn texture_view_usage_info(key: &KeyTextureViewUsage) {
    match key {
        KeyTextureViewUsage::Tex(_, _) => todo!(),
        KeyTextureViewUsage::Image(_, _) => todo!(),
        KeyTextureViewUsage::Render(_, _) => todo!(),
        KeyTextureViewUsage::SRT(_, _, _) => todo!(),
        KeyTextureViewUsage::Temp(_, _) => todo!(),
    }
}
