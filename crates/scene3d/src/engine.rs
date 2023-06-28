use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{PluginNodeMaterial, NodeMaterialBlocks};
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

    app.
        add_plugin(PluginWindowRender)
        .add_plugins(PluginBundleDefault)
        .add_plugin(PluginFrameTime)
        .add_plugin(PluginNodeMaterial)
        .add_plugin(PluginCubeBuilder)
        .add_plugin(PluginQuadBuilder)
        .add_plugin(PluginUnlitMaterial)
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
    pub default_mat: Res<'w, SingleIDBaseDefaultMaterial>,
    pub node_material_blocks: Res<'w, NodeMaterialBlocks>,
    pub renderermodifycmds: ResMut<'w, ActionListRendererModify>,
    pub layer_mask: ResMut<'w, ActionListLayerMask>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D(pub(crate) SystemState<ActionSets<'static>>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(app: &mut Engine) -> Self {
        Self(SystemState::<ActionSets>::new(&mut app.world))
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) {
    let entity: Entity = as_entity(entity);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.obj_dispose.push(OpsDispose::ops(entity));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_dispose(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64) {
    let entity: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.scene_dispose.push(OpsSceneDispose::ops(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_graphic(app: &mut Engine, pre: f64, next: f64, next_is_finish: bool) {
    let pre: Entity = as_entity(pre);
    let next: Entity = as_entity(next);

    // let mut cmds = app.world.get_resource_mut::<PiRenderGraph>().unwrap();

    // cmds.add_depend(pre, next);
}
