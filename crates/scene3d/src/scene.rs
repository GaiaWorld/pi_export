
use std::ops::Deref;

use pi_engine_shell::prelude::*;
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
use pi_scene_context::prelude::*;

use crate::{as_dk, commands::CommandsExchangeD3};
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
pub fn p3d_scene(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, cullingmode: f64, vals: &[i32]) -> f64 {
    let scene: Entity = app.world.spawn_empty().id();

    let mut values = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut idx = 0;
    vals.iter().for_each(|val| {
        if idx < 9 {
            values[idx] = *val as i32;
        }
        idx += 1;
    });

    let mut resource = param.resource.get_mut(&mut app.world);
    resource.anime_scene_ctxs.init_scene(scene);
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

    cmds.scene_animeenable.push(OpsSceneAnimationEnable::ops(scene, val));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_time(cmds: &mut CommandsExchangeD3, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_time.push(OpsSceneTime::ops(scene, val as u64));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_fogcolor(cmds: &mut CommandsExchangeD3, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_fogcolor.push(OpsSceneFogColor::ops(scene, r as f32, g as f32, b as f32));
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
    cmds.scene_fogparam.push(OpsSceneFogParam::ops(scene, param));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientcolor(cmds: &mut CommandsExchangeD3, scene: f64, r: f64, g: f64, b: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_ambientcolor.push(OpsSceneAmbientColor::ops(scene, r as f32, g as f32, b as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_ambientintensity(cmds: &mut CommandsExchangeD3, scene: f64, val: f64) {
    let scene: Entity = as_entity(scene);

    cmds.scene_ambientintensity.push(OpsSceneAmbientIntensity::ops(scene, val as f32));
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

    cmds.scene_brdf.push(OpsSceneBRDF::ops(scene, url.deref().clone(), compressed));
}

///
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_env_texture(cmds: &mut CommandsExchangeD3, scene: f64, url: &Atom, data_is_image: bool) {
    let scene: Entity = as_entity(scene);

    // if let Some(url) = url {
        cmds.scene_env.push(OpsSceneEnvTexture::ops(scene, Some(url.deref().clone()), data_is_image));
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
        let key = as_dk(&url);
        cmds.scene_shadowmap.push(OpsSceneShadowMap::ops(scene, Some(key)));
    } else {
        cmds.scene_shadowmap.push(OpsSceneShadowMap::ops(scene, None));
    }
}