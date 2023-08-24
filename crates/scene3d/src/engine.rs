use std::{mem::transmute, ops::Deref};

// use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_gltf2_load::{GLTF, PluginGLTF2Res, GLTFResLoader, KeyGLTF, TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{PluginNodeMaterial, NodeMaterialBlocks, prelude::*};
use pi_particle_system::{PluginParticleSystem, prelude::{ParticleSystemActionSet, ParticleSystemCalculatorID}};
use pi_scene_context::prelude::*;
use pi_trail_renderer::{PluginTrail, ActionSetTrailRenderer};
use unlit_material::PluginUnlitMaterial;
use pi_export_base::asset::Atom;

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

    if app.world.get_resource::<AssetMgrConfigs>().is_none() {
        app.insert_resource(AssetMgrConfigs::default());
    }

    app
        .add_plugins(PluginBundleDefault)
        .add_plugins(PluginNodeMaterial)
        .add_plugins(PluginCubeBuilder)
        .add_plugins(PluginQuadBuilder)
        .add_plugins(PluginUnlitMaterial)
        .add_plugins(PluginGroupNodeMaterialAnime)
        .add_plugins(PluginParticleSystem)
        .add_plugins(PluginGLTF2Res)
        .add_plugins(PluginTrail)
        ;
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
    pub node_material_blocks: Res<'w, NodeMaterialBlocks>,
    pub layer_mask: ResMut<'w, ActionListLayerMask>,
    pub renderer_drawcalls: Res<'w, RendererDrawCallRecord>,
    pub transform_record: Res<'w, TransformRecord>,
    pub imgtex_loader: ResMut<'w, ImageTextureLoader>,
    pub imgtex_asset: Res<'w, ShareAssetMgr<ImageTexture>>,
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
    pub state: ResMut<'w, pi_3d_state::StateGlobal>,
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D {
    pub(crate) acts: SystemState<ActionSets<'static>>,
    pub(crate) state: SystemState<GlobalState<'static>>,
    pub(crate) particlesys: SystemState<ParticleSystemActionSet<'static>>,
    pub(crate) world_transform: QueryState<&'static WorldMatrix>,
    pub(crate) local_transform: QueryState<&'static LocalMatrix>,
    pub(crate) view_matrix: QueryState<&'static ViewerViewMatrix>,
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
            particlesys: SystemState::<ParticleSystemActionSet>::new(&mut app.world),
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

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_drawcalls(app: &mut Engine, param: &mut ActionSetScene3D, renderer: f64) -> f64 {
    let entity: Entity = as_entity(renderer);

    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if let Some(count) = cmds.renderer_drawcalls.0.get(&entity) {
        *count as f64
    } else {
        0 as f64
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_world_matrix_time(app: &mut Engine, param: &mut ActionSetScene3D) -> f64 {

    let cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.transform_record.all_wmcompute as f64
}

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
    
    let cmds = param.state.get_mut(&mut app.world);

    if let Some(state) = cmds.state.scenes.get(&entity) {
        result[0] = state.count_mesh as f32;
        result[1] = state.count_drawobj as f32;
        result[2] = state.count_transform as f32;
        result[3] = state.count_particlesys as f32;
        result[4] = state.count_vertex as f32;
        result[5] = state.count_trail as f32;
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_resource_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    
    let cmds = param.state.get_mut(&mut app.world);

    result[0] = cmds.state.count_bindbuffer as f32;
    result[1] = cmds.state.count_bindgroup as f32;
    result[2] = cmds.state.count_gltf as f32;
    result[3] = cmds.state.count_imgtexture as f32;
    result[4] = cmds.state.count_shader as f32;
    result[5] = cmds.state.count_pipeline as f32;
    result[6] = (cmds.state.size_geometrybuffer / 1024) as f32;
    result[7] = cmds.state.count_geometrybuffer as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_global_state(app: &mut Engine, param: &mut ActionSetScene3D, val: bool) {
    
    let mut cmds = param.state.get_mut(&mut app.world);

    cmds.state.debug = val;
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