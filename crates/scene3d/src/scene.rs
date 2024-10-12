
use std::{mem::transmute, ops::Deref};

use pi_scene_shell::prelude::*;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
use pi_scene_context::prelude::*;

use crate::{as_dk, constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub struct PassCfg(RenderFormat, DepthStencilFormat, bool);
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// impl PassCfg {
//     #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
//     #[pi_js_export]
//     pub fn create(color: RenderFormat, depth_stencil: DepthStencilFormat, blend: bool) -> Self {
//         Self(color, depth_stencil, blend)
//     }
// }

/// 正向渲染
/// Pass 配置使用默认值
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene(app: &mut Engine, cmds: &mut CommandsExchangeD3, cullingmode: f64, vals: &[i32]) -> f64 {
    let scene: Entity = app.world.entities().reserve_entity();

    let mut values = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut idx = 0;
    vals.iter().for_each(|val| {
        if idx < 9 {
            values[idx] = *val as i32;
        }
        idx += 1;
    });

    cmds.scene_create.push(OpsSceneCreation::ops(
        scene,
        cullingmode as u8,
        values
    ));

    as_f64(&scene)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_animation_enable(cmds: &mut CommandsExchangeD3, scene: f64, val: bool) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::anime(scene, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_time(cmds: &mut CommandsExchangeD3, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::time(scene, val as u64));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_fogcolor(cmds: &mut CommandsExchangeD3, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::fogcolor(scene, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_fogparam(cmds: &mut CommandsExchangeD3, scene: f64, mode: f64, param0: f64, param1: f64, param2: f64) {
    let scene: Entity = as_entity(scene);

    let mode = mode as u8;
    let param = if mode == FogParam::EXP {
        FogParam::Exp(FogExpParam { density_fallof: param0 as f32 })
    } else if mode == FogParam::EXP2 {
        FogParam::Exp2(FogExp2Param { density_fallof: param0 as f32 })
    } else if mode == FogParam::LINEAR {
        FogParam::Linear(FogLinearParam { start: param0 as f32, end: param1 as f32 })
    } else if mode == FogParam::ALTITUDE_BASE {
        FogParam::AltitudeBase(FogAltitudeBaseParam { h_while_max_density: param0 as f32, density: param1 as f32, density_fallof: param2 as f32 })
    } else {
        FogParam::None
    };
    cmds.scene_options.push(OpsSceneOption::fogparam(scene, param));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientcolor(cmds: &mut CommandsExchangeD3, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::ambientcolor(scene, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientintensity(cmds: &mut CommandsExchangeD3, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::ambientinstensity(scene, val as f32));
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_layermask(cmds: &mut CommandsExchangeD3, node: f64, val: f64) {
    let node: Entity = as_entity(node);

    cmds.mesh_layermask.push(OpsLayerMask::ops(node, val as u32));
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_brdf_texture(cmds: &mut CommandsExchangeD3, scene: f64, url: &Atom, compressed: bool) {
    let scene: Entity = as_entity(scene);

    cmds.scene_options.push(OpsSceneOption::brdf(scene, url.deref().clone(), compressed));
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_env_texture(cmds: &mut CommandsExchangeD3, scene: f64, url: &Atom, data_is_image: bool) {
    let scene: Entity = as_entity(scene);

    // if let Some(url) = url {
        cmds.scene_options.push(OpsSceneOption::envtexture(scene, Some(url.deref().clone()), data_is_image));
    // } else {
    //     cmds.scene_env.push(OpsSceneEnvTexture::ops(scene, None, data_is_image));
    // }
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_shadowmap(cmds: &mut CommandsExchangeD3, scene: f64, url: Option<f64>) {
    let scene: Entity = as_entity(scene);

    if let Some(url) = url {
        let key = unsafe { transmute(url) };
        cmds.scene_options.push(OpsSceneOption::shadowmap(scene, Some(key)));
    } else {
        cmds.scene_options.push(OpsSceneOption::shadowmap(scene, None));
    }
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_boundingbox(cmds: &mut CommandsExchangeD3, scene: f64, display: bool, pass: f64) {
    let scene: Entity = as_entity(scene);
    let pass = EngineConstants::passtag(pass);

    cmds.scene_boundingbox.push(OpsBoundingBoxDisplay::ops(scene, display, pass));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_collider(cmds: &mut CommandsExchangeD3, node: f64,
    minx: f64, miny: f64, minz: f64,
    maxx: f64, maxy: f64, maxz: f64
) {
    let node: Entity = as_entity(node);

    cmds.scene_collider.push(OpsCollider::ops(node, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_pickingray(app: &mut Engine, param: &mut ActionSetScene3D, camera: f64, projectx: f64, projecty: f64, result: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let camera: Entity = as_entity(camera);

    if let Ok(tree) = param.vp_matrix.get(&app.world, camera) {
        let ray = tree.ray(projectx as f32, projecty as f32);
        result[0] = ray.origin.0;
        result[1] = ray.origin.1;
        result[2] = ray.origin.2;
        result[3] = ray.direction.0;
        result[4] = ray.direction.1;
        result[5] = ray.direction.2;
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_pick(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, viewer: f64, projectx: f64, projecty: f64, not_ray_bounding: bool, result: &mut [f64]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let scene: Entity = as_entity(scene);
    let camera: Entity = as_entity(viewer);

    param.collider.align(&app.world);
    param.vp_matrix.align(&app.world);
    if let (Ok((collider, bounding)), Ok(vp)) = (param.collider.get(&app.world, scene), param.vp_matrix.get(&app.world, camera)) {
        let ray = vp.ray(projectx as f32, projecty as f32);
        let picked = ray_cast_scene((collider, bounding), &ray, !not_ray_bounding);
        if let Some(picked) = picked {
            result[ 0] = as_f64(&picked.target);
            result[ 1] = if picked.bybounding { 1.0 } else { -1.0 };
            result[ 2] = picked.min.0 as f64;
            result[ 3] = picked.min.1 as f64;
            result[ 4] = picked.min.2 as f64;
            result[ 5] = picked.max.0 as f64;
            result[ 6] = picked.max.1 as f64;
            result[ 7] = picked.max.2 as f64;
            result[ 8] = if picked.pickdetail.is_some() { 1.0 } else { -1.0 };
            if let Some(pickdetail) = picked.pickdetail {
                result[ 9] = pickdetail.0 as f64;
                result[10] = pickdetail.1 as f64;
                result[11] = pickdetail.2 as f64;
            }
            true
        } else {
            log::error!("bbb {:?}", collider.size());
            false
        }
    } else {
        log::error!("aaa");
        false
    }
}
