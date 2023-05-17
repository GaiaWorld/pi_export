
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{ColorFormat, DepthFormat}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64, geometry::GeometryMeta};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_mesh(app: &mut Engine, param: &mut ActionSetScene3D, name: &pi_export_base::export::Atom, scene: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.meshcmds.create.push(OpsMeshCreation::ops(scene, id, name.to_string()));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_mesh_geometry(app: &mut Engine, param: &mut ActionSetScene3D, mesh: f64, geometa: GeometryMeta) {
    let geo: Entity = app.world.spawn_empty().id();
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.geometrycmd.create.push(OpsGeomeryCreate::ops(mesh, geo, geometa.0, geometa.1));
}