
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{ColorFormat, DepthFormat}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_scene(app: &mut Engine, param: &mut ActionSetScene3D) -> f64 {
    let scene: Entity = app.world.spawn_empty().id();

    let mut scenecmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    scenecmds.scene.create.push(scene);

    // scenecmds.get(&app.world).

    as_f64(&scene)
}