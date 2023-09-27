use std::{mem::transmute, ops::Deref};

// use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_gltf2_load::{GLTF, PluginGLTF2Res, GLTFResLoader, KeyGLTF, TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{PluginNodeMaterial, NodeMaterialBlocks, prelude::*};
use pi_particle_system::{PluginParticleSystem, prelude::{ParticleSystemActionSet, ParticleSystemCalculatorID}};
use pi_scene_context::prelude::*;
use pi_particle_system::prelude::*;
use pi_trail_renderer::{PluginTrail, ActionSetTrailRenderer};
pub use pi_export_base::asset::Atom;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ImageRes(Handle<pi_render::renderer::texture::ImageTexture>);

use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;
use pi_hal::*;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_init_engine(app: &mut Engine) {
    // use pi_engine_shell::frame_time::PluginFrameTime;
    println!("======== p3d_init_engine");

    if app.world.get_resource::<AssetMgrConfigs>().is_none() {
        app.insert_resource(AssetMgrConfigs::default());
    }

    app
        .add_plugins(PluginBundleDefault)
        .add_plugins(PluginNodeMaterial)
        .add_plugins(PluginCubeBuilder)
        .add_plugins(PluginQuadBuilder)
        .add_plugins(PluginGroupNodeMaterialAnime)
        .add_plugins(PluginParticleSystem)
        .add_plugins(PluginGLTF2Res)
        .add_plugins(PluginTrail)
        ;

    app.add_systems(
        Update,
        (
            sys_state_transform,
        ).in_set(ERunStageChap::StateCheck)
    );
}

#[derive(SystemParam)]
pub struct ActionSets<'w> {
    pub scene: ActionSetScene<'w>,
    pub scene_dispose: ResMut<'w, ActionListSceneDispose>,
    pub obj_dispose: ResMut<'w, ActionListDispose>,
    pub cameracmds: ActionSetCamera<'w>,
    pub transformcmds: ActionSetTransform<'w>,
    pub meshcmds: ActionSetMesh<'w>,
    pub abstructmeshcmds: ActionSetAbstructMesh<'w>,
    pub instancemeshcmds: ActionSetInstanceMesh<'w>,
    pub geometrycmd: ActionSetGeometry<'w>,
    pub matcmd: ActionSetMaterial<'w>,
    pub animegroupcmd: ActionSetAnimationGroup<'w>,
    pub renderercmds: ActionSetRenderer<'w>,
    pub default_mat: Res<'w, SingleIDBaseDefaultMaterial>,
    pub node_material_blocks: ResMut<'w, NodeMaterialBlocks>,
    // pub layer_mask: ResMut<'w, ActionListLayerMask>,
    // pub renderer_drawcalls: Res<'w, RendererDrawCallRecord>,
    // pub transform_record: Res<'w, TransformRecord>,
    pub imgtex_loader: ResMut<'w, ImageTextureLoader>,
    pub imgtex_loader_state: ResMut<'w, StateTextureLoader>,
    pub imgtex_asset: Res<'w, ShareAssetMgr<ImageTexture>>,
    pub imgtexview_asset: Res<'w, ShareAssetMgr<ImageTextureView>>,
    pub gltf2_asset: Res<'w, ShareAssetMgr<GLTF>>,
    pub gltf2_loader: ResMut<'w, GLTFResLoader>,
    pub device: Res<'w, PiRenderDevice>,
    pub queue: Res<'w, PiRenderQueue>,
    pub anime_assets: TypeAnimeAssetMgrs<'w>,
    pub anime_contexts: TypeAnimeContexts<'w>,
    pub trail: ActionSetTrailRenderer<'w>,
}

#[derive(SystemParam)]
pub struct GlobalState<'w> {
    pub resource: Res<'w, pi_3d_state::StateResource>,
    pub performance: Res<'w, pi_engine_shell::prelude::Performance>,
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
    pub(crate) acts: SystemState<ActionSets<'static>>,
    pub(crate) state: SystemState<GlobalState<'static>>,
    pub(crate) actparticlesys: SystemState<ParticleSystemActionSet<'static>>,
    pub(crate) world_transform: QueryState<&'static WorldMatrix>,
    pub(crate) local_transform: QueryState<&'static LocalMatrix>,
    pub(crate) view_matrix: QueryState<&'static ViewerViewMatrix>,
    pub(crate) meshes: QueryState<(&'static SceneID, &'static GlobalEnable, Option<&'static RenderGeometryEable>, Option<&'static InstanceMesh>, &'static AbstructMesh)>, // StateMeshQuery,
    pub(crate) materials: QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp)>, // StateMaterialQuery,
    pub(crate) transforms: QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable)>, // StateTransformQuery,
    pub(crate) cameras: QueryState<(&'static Camera, &'static ModelList, &'static ModelListAfterCulling)>, // StateCameraQuery,
    pub(crate) renderers: QueryState<(&'static ViewerID, &'static Renderer)>,
    pub(crate) viewers: QueryState<(&'static ViewerActive, &'static SceneID)>,
    pub(crate) particlesystems: QueryState<(&'static ParticleIDs, &'static SceneID)>,
    pub(crate) trails: QueryState<(&'static pi_trail_renderer::TrailPoints, &'static SceneID)>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(app: &mut Engine) -> Self {
        Self {
            acts: SystemState::<ActionSets>::new(&mut app.world),
            state: SystemState::<GlobalState>::new(&mut app.world),
            world_transform: app.world.query(),
            local_transform: app.world.query(),
            view_matrix: app.world.query(),
            actparticlesys: SystemState::<ParticleSystemActionSet>::new(&mut app.world),
            meshes: app.world.query(),
            materials: app.world.query(),
            transforms: app.world.query(),
            cameras: app.world.query(),
            renderers: app.world.query(),
            viewers: app.world.query(),
            particlesystems: app.world.query(),
            trails: app.world.query(),
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_entity(app: &mut Engine) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    as_f64(&id)
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) {
    let entity: Entity = as_entity(entity);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.obj_dispose.push(OpsDispose::ops(entity));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_dispose(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64) {
    let entity: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.scene_dispose.push(OpsSceneDispose::ops(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_graphic(app: &mut Engine, param: &mut ActionSetScene3D, before: f64, after: f64) {
    let before: Entity = as_entity(before);
    let after: Entity = as_entity(after);

    let mut cmds = param.acts.get_mut(&mut app.world);

    cmds.renderercmds.connect.push(OpsRendererConnect::ops(before, after));
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_drawcalls(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64) -> f64 {
//     let entity: Entity = as_entity(renderer);

//     let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

//     if let Some(count) = cmds.renderer_drawcalls.0.get(&entity) {
//         *count as f64
//     } else {
//         0 as f64
//     }
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_world_matrix_time(app: &mut Engine, param: &mut ActionSetScene3D) -> f64 {

//     let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

//     cmds.transform_record.all_wmcompute as f64
// }

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_world_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
    let entity: Entity = as_entity(entity);

    if let Ok(trans) = param.world_transform.get(&app.world, entity) {
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
pub fn p3d_query_scene_state(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, result: &mut [f32]) -> bool {
    let entity: Entity = as_entity(entity);

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
    let cmds = param.acts.get_mut(&mut app.world);
    cmds.animegroupcmd.scene_ctxs.iter().for_each(|(idscene, ctx)| {
        if entity == *idscene {
            count_animegroup += ctx.0.group_mgr.groups.len();
        }
    });
    
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
pub fn p3d_query_resource_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    
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

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    
    // let mut cmds = param.materials.get(&mut app.world);
    let mut state = StateMaterial::default();
    param.materials.iter(&app.world).for_each(|(meta, texs)| {
        state.count += 1;
        let texcount = meta.textures.len() as u32;
        let mut isready = false;
        if let Some(texs) = &texs.0 {
            if texcount * 2 == texs.binding_count {
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
pub fn p3d_camera_state(app: &mut Engine, param: &mut ActionSetScene3D, camera: Option<f64>, result: &mut [f32]) {
    
    // let mut cmds = param.state.get_mut(&mut app.world);

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

    let cmds = param.acts.get_mut(&mut app.world);

    result[ 0] = cmds.imgtex_loader_state.image_count as f32;
    result[ 1] = cmds.imgtex_loader_state.image_fail as f32;
    result[ 2] = cmds.imgtex_loader_state.image_success as f32;
    result[ 3] = cmds.imgtex_loader_state.image_waiting as f32;
    result[ 4] = cmds.imgtex_loader_state.texview_count as f32;
    result[ 5] = cmds.imgtex_loader_state.texview_fail as f32;
    result[ 6] = cmds.imgtex_loader_state.texview_success as f32;
    result[ 7] = cmds.imgtex_loader_state.texview_waiting as f32;
    result[ 8] = cmds.imgtex_asset.len() as f32;
    result[ 9] = cmds.imgtex_asset.size() as f32;
    result[10] = cmds.imgtexview_asset.len() as f32;
    result[11] = cmds.imgtexview_asset.size() as f32;
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
    let entity: Entity = as_entity(entity);

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
    let entity: Entity = as_entity(entity);

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
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let entity: Entity = as_entity(entity);

    let param = KeyGLTF { base_url: baseurl.deref().clone(), dyn_desc: pi_atom::Atom::from(dyndesc) };

    cmds.gltf2_loader.create_load(entity, param);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let max = success.len();
    let mut item = cmds.gltf2_loader.success.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = cmds.gltf2_loader.success.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = cmds.gltf2_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = cmds.gltf2_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<GLTFRes> {
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let entity: Entity = as_entity(entity);
    if let Some(val) = cmds.gltf2_loader.get_success(entity) {
        Some(GLTFRes(val))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<String> {
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let entity: Entity = as_entity(entity);
    cmds.gltf2_loader.get_fail_reason(entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_image_load(app: &mut Engine, param: &mut ActionSetScene3D, url: &Atom, srgb: bool) -> f64 {
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let id = cmds.imgtex_loader.create_load(KeyImageTexture::File(url.deref().clone(), srgb));

    unsafe { transmute(id) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_image_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let max = success.len();
    let mut item = cmds.imgtex_loader.success_load.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = cmds.imgtex_loader.success_load.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = cmds.imgtex_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = cmds.imgtex_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<ImageRes> {
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    if let Some(img) = cmds.imgtex_loader.query_success(id) {
        Some(ImageRes(img))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<String> {
    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    cmds.imgtex_loader.query_failed_reason(id)
}

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_global_position(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, position: &mut [f32]) -> bool {
//     let entity: Entity = as_entity(entity);

//     let mut cmds = param.acts.get_mut(&mut app.world);

//     if let Ok(mut trans) = cmds.query.global_transform.get_mut(entity) {
//         let mut i = 0;
//         trans.position().as_slice().iter().for_each(|val| {
//             matrix[i] = *val;
//             i += 1;
//         });
//         true
//     } else {
//         false
//     }
// }

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_query_global_scaling(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, scaling: &mut [f32]) -> bool {
//     let entity: Entity = as_entity(entity);

//     let mut cmds = param.acts.get_mut(&mut app.world);

//     if let Ok(mut trans) = cmds.query.global_transform.get_mut(entity) {
//         let mut i = 0;
//         trans.scaling().as_slice().iter().for_each(|val| {
//             matrix[i] = *val;
//             i += 1;
//         });
//         true
//     } else {
//         false
//     }
// }