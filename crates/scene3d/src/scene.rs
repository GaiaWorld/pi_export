
use std::ops::Deref;

use pi_engine_shell::prelude::*;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
use pi_scene_context::prelude::*;

use crate::as_dk;
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
pub fn p3d_scene(app: &mut Engine, param: &mut ActionSetScene3D, cullingmode: f64, vals: &[i32]) -> f64 {
    let scene: Entity = app.world.spawn_empty().id();

    let mut values = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut idx = 0;
    vals.iter().for_each(|val| {
        if idx < 9 {
            values[idx] = *val as i32;
        }
        idx += 1;
    });

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    scenecmds.animegroupcmd.scene_ctxs.init_scene(scene);
    scenecmds.scene.create.push(OpsSceneCreation::ops(
        scene,
        cullingmode as u8,
        values
    ));

    as_f64(&scene)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_animation_enable(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, val: bool) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.animeenable.push(OpsSceneAnimationEnable::ops(scene, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_time(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.time.push(OpsSceneTime::ops(scene, val as u64));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_fogcolor(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.fogcolor.push(OpsSceneFogColor::ops(scene, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_fogparam(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, mode: f64, param0: f64, param1: f64, param2: f64) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

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
    scenecmds.scene.fogparam.push(OpsSceneFogParam::ops(scene, param));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientcolor(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.ambientcolor.push(OpsSceneAmbientColor::ops(scene, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientintensity(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.ambientintensity.push(OpsSceneAmbientIntensity::ops(scene, val as f32));
}

///
/// 相机渲染像素宽高
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_layermask(app: &mut Engine, param: &mut ActionSetScene3D, node: f64, val: f64) {
    let node: Entity = as_entity(node);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.meshcmds.layermask.push(OpsLayerMask::ops(node, val as u32));
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_brdf_texture(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, url: &Atom, compressed: bool) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    scenecmds.scene.brdf.push(OpsSceneBRDF::ops(scene, url.deref().clone(), compressed));
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_env_texture(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, url: &Atom, data_is_image: bool) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    // if let Some(url) = url {
        scenecmds.scene.env.push(OpsSceneEnvTexture::ops(scene, Some(url.deref().clone()), data_is_image));
    // } else {
    //     scenecmds.scene.env.push(OpsSceneEnvTexture::ops(scene, None, data_is_image));
    // }
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_shadowmap(app: &mut Engine, param: &mut ActionSetScene3D, scene: f64, url: Option<f64>) {
    let scene: Entity = as_entity(scene);

    let mut scenecmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if let Some(url) = url {
        let key = as_dk(&url);
        scenecmds.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, Some(key)));
    } else {
        scenecmds.scene.shadowmap.push(OpsSceneShadowMap::ops(scene, None));
    }
}