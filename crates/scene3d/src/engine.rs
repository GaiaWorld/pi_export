use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{PluginNodeMaterial, NodeMaterialBlocks, prelude::*};
use pi_scene_context::prelude::*;
use unlit_material::PluginUnlitMaterial;

use crate::as_entity;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_init_engine(app: &mut Engine) {
    use pi_engine_shell::frame_time::PluginFrameTime;

    app
        // add_plugin(PluginWindowRender)
        .add_plugins(PluginBundleDefault)
        // .add_plugin(PluginFrameTime)
        .add_plugin(PluginNodeMaterial)
        .add_plugin(PluginCubeBuilder)
        .add_plugin(PluginQuadBuilder)
        .add_plugin(PluginUnlitMaterial)
        .add_plugins(PluginGroupNodeMaterialAnime)
        ;
}

#[derive(SystemParam)]
pub struct ActionSets<'w> {
    pub scene: ActionSetScene<'w>,
    pub scene_dispose: ResMut<'w, ActionListSceneDispose>,
    pub obj_dispose: ResMut<'w, ActionListDispose>,
    pub cameracmds: ActionSetCamera<'w>,
    pub transformcmds: ActionSetTransform<'w>,
    pub transformanime: ActionSetTransformNodeAnime<'w>,
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
    pub anime_isactive: (
        ResMut<'w, TypeAnimeContext<Enable>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<Enable>>>
    ),
    pub anime_camerafov: (
        ResMut<'w, TypeAnimeContext<CameraFov>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<CameraFov>>>
    ),
    pub anime_camerasize: (
        ResMut<'w, TypeAnimeContext<CameraOrthSize>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<CameraOrthSize>>>
    ),
    pub anime_alpha: (
        ResMut<'w, TypeAnimeContext<Alpha>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<Alpha>>>
    ),
    pub anime_alphacutoff: (
        ResMut<'w, TypeAnimeContext<Cutoff>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<Cutoff>>>
    ),
    pub anime_maintex_uscale: (
        ResMut<'w, TypeAnimeContext<MainTexUScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUScale>>>
    ),
    pub anime_maintex_vscale: (
        ResMut<'w, TypeAnimeContext<MainTexVScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVScale>>>
    ),
    pub anime_maintex_uoffset: (
        ResMut<'w, TypeAnimeContext<MainTexUOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexUOffset>>>
    ),
    pub anime_maintex_voffset: (
        ResMut<'w, TypeAnimeContext<MainTexVOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MainTexVOffset>>>
    ),
    pub anime_opacitytex_uscale: (
        ResMut<'w, TypeAnimeContext<OpacityTexUScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUScale>>>
    ),
    pub anime_opacitytex_vscale: (
        ResMut<'w, TypeAnimeContext<OpacityTexVScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVScale>>>
    ),
    pub anime_opacitytex_uoffset: (
        ResMut<'w, TypeAnimeContext<OpacityTexUOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexUOffset>>>
    ),
    pub anime_opacitytex_voffset: (
        ResMut<'w, TypeAnimeContext<OpacityTexVOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<OpacityTexVOffset>>>
    ),
    pub anime_masktex_uscale: (
        ResMut<'w, TypeAnimeContext<MaskTexUScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUScale>>>
    ),
    pub anime_masktex_vscale: (
        ResMut<'w, TypeAnimeContext<MaskTexVScale>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVScale>>>
    ),
    pub anime_masktex_uoffset: (
        ResMut<'w, TypeAnimeContext<MaskTexUOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexUOffset>>>
    ),
    pub anime_masktex_voffset: (
        ResMut<'w, TypeAnimeContext<MaskTexVOffset>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MaskTexVOffset>>>
    ),
    pub anime_maskcutoff: (
        ResMut<'w, TypeAnimeContext<MaskCutoff>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MaskCutoff>>>
    ),
    pub anime_maincolor: (
        ResMut<'w, TypeAnimeContext<MainColor>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<MainColor>>>
    ),
    pub anime_lightdiffuse: (
        ResMut<'w, TypeAnimeContext<LightDiffuse>>,
        Res<'w, ShareAssetMgr<TypeFrameCurve<LightDiffuse>>>
    ),
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D {
    pub(crate) acts: SystemState<ActionSets<'static>>,
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
            world_transform: app.world.query(),
            local_transform: app.world.query(),
            view_matrix: app.world.query(),
        }
    }
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