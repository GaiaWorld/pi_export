pub use pi_export_base::export::Engine;
use pi_scene_context::pass::{ESkinBonesPerVertex, WorldResourceTemp};
use pi_scene_context::skeleton::prelude::*;
use pi_scene_shell::prelude::*;

use crate::as_f64;
pub use crate::commands::CommandsExchangeD3;
pub use crate::as_entity;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_skeleton(app: &mut Engine, cmds: &mut CommandsExchangeD3, bonespervertex: f64, root: f64, bones: &[f64], bonecount: f64, cacheframe: f64) -> f64 {

    let id: Entity = app.world.entities().reserve_entity();

    let root = as_entity(root);
    let mut boneentities = vec![];
    for idx in 0..(bonecount as usize) {
        boneentities.push(as_entity(bones[idx]));
    }
    // bones.iter().for_each(|idx| {
    // });

    CommandsExchangeD3::p3d_skeleton(cmds, id, bonespervertex as u8, root, &boneentities, bonecount, cacheframe);
    // cmds.skin_create.push(OpsSkinCreation::ops(id, state, root, &boneentities, cacheframe as u16, None));

    as_f64(&id)
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64) -> f64 {

    let bone: Entity = app.world.entities().reserve_entity();

    let scene = as_entity(scene);

    CommandsExchangeD3::p3d_bone(cmds, bone, scene);
    // cmds.skin_bonecreate.push(OpsBoneCreation::ops(bone, scene));

    as_f64(&bone)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone_link(cmds: &mut CommandsExchangeD3, bone: f64, link: f64) {

    let bone = as_entity(bone);
    let link = as_entity(link);

    CommandsExchangeD3::p3d_bone_link(cmds, bone, link);
    // cmds.skin_use.push(OpsSkinUse::bone_link(bone, link));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone_pose(cmds: &mut CommandsExchangeD3, bone: f64, data: &[f32]) {

    let bone = as_entity(bone);

    let data = data[0..16].to_vec();
    // if matrix.is_invertible() {
    //     matrix.try_inverse_mut();
    // }
    CommandsExchangeD3::p3d_bone_pose(cmds, bone, &data);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_skin_use(cmds: &mut CommandsExchangeD3, id_mesh: f64, skin: f64) {

    let id_mesh = as_entity(id_mesh);
    let skin = as_entity(skin);

    CommandsExchangeD3::p3d_skin_use(cmds, id_mesh, skin);
}
