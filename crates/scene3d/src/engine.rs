use pi_3d::PluginBundleDefault;
use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::PluginNodeMaterial;
use pi_scene_context::{
    scene::ActionSetScene,
    cameras::ActionSetCamera,
    transforms::{ActionSetTransform, ActionSetTransformNodeAnime},
    meshes::{ActionSetMesh, ActionSetInstanceMesh},
    geometry::ActionSetGeometry,
    materials::ActionSetMaterial,
    animation::ActionSetAnimationGroup, object::{ActionListDispose, OpsDispose}
};
use unlit_material::PluginUnlitMaterial;

use crate::as_entity;
use wasm_bindgen::prelude::wasm_bindgen;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_init_engine(app: &mut Engine) {
    app.add_plugins(PluginBundleDefault)
        .add_plugin(PluginNodeMaterial)
        .add_plugin(PluginCubeBuilder)
        .add_plugin(PluginQuadBuilder)
        .add_plugin(PluginUnlitMaterial)
        ;
}

#[derive(SystemParam)]
pub struct ActionSets<'w> {
    pub scene: ActionSetScene<'w>,
    pub cameracmds: ActionSetCamera<'w>,
    pub transformcmds: ActionSetTransform<'w>,
    pub transformanime: ActionSetTransformNodeAnime<'w>,
    pub meshcmds: ActionSetMesh<'w>,
    pub instancemeshcmds: ActionSetInstanceMesh<'w>,
    pub geometrycmd: ActionSetGeometry<'w>,
    pub matcmd: ActionSetMaterial<'w>,
    pub animegroupcmd: ActionSetAnimationGroup<'w>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct ActionSetScene3D(pub(crate) SystemState<ActionSets<'static>>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[cfg(feature = "pi_js_export")]
    pub fn create(app: &mut Engine) -> Self {
        Self(SystemState::<ActionSets>::new(&mut app.world))
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_dispose(app: &mut Engine, entity: f64) {
    let entity: Entity = as_entity(entity);

    let mut cmds = app.world.get_resource_mut::<ActionListDispose>().unwrap();

    cmds.push(OpsDispose::ops(entity));
}
