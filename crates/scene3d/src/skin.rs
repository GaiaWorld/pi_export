pub use pi_export_base::export::Engine;
use pi_scene_context::pass::{ESkinBonesPerVertex, WorldResourceTemp};
use pi_scene_context::skeleton::prelude::*;
use pi_scene_shell::prelude::*;

use crate::as_f64;
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_skeleton(app: &mut Engine, cmds: &mut CommandsExchangeD3, bonespervertex: f64, root: f64, bones: &[f64], bonecount: f64, cacheframe: f64) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();

    let state = match (bonespervertex as u8) {
        1 => ESkinBonesPerVertex::One,
        2 => ESkinBonesPerVertex::Two,
        3 => ESkinBonesPerVertex::Three,
        _ => ESkinBonesPerVertex::Four
    };

    let root = as_entity(root);
    let mut boneentities = vec![];
    bones.iter().for_each(|idx| {
        boneentities.push(as_entity(*idx));
    });

    cmds.skin_create.push(OpsSkinCreation::ops(id, state, root, &boneentities, cacheframe as u16, None));

    as_f64(&id)
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();

    let scene = as_entity(scene);
    let bone = id;

    cmds.skin_bonecreate.push(OpsBoneCreation::ops(bone, scene));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone_link(cmds: &mut CommandsExchangeD3, bone: f64, link: f64) {

    let bone = as_entity(bone);
    let link = as_entity(link);

    cmds.skin_use.push(OpsSkinUse::bone_link(bone, link));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_bone_pose(cmds: &mut CommandsExchangeD3, bone: f64, data: &[f32]) {

    let bone = as_entity(bone);

    let mut matrix = Matrix::new(
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]
    );
    // if matrix.is_invertible() {
    //     matrix.try_inverse_mut();
    // }
    cmds.skin_bonepose.push(OpsBonePose::ops(
        bone, 
        matrix
    ));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_skin_use(cmds: &mut CommandsExchangeD3, id_mesh: f64, skin: f64) {

    let id_mesh = as_entity(id_mesh);
    let skin = as_entity(skin);

    cmds.skin_use.push(OpsSkinUse::ops(id_mesh, skin));
}
