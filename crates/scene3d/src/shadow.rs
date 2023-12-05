
use js_proxy_gen_macro::pi_js_export;
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use crate::commands::CommandsExchangeD3;
use crate::{as_entity, as_f64};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use pi_export_base::export::Engine;

/// * pass_tag:
///     * 0b0000_0000_0000_0001
///     * 0b0000_0000_0000_0010
///     * 0b0000_0000_0000_0100
///     * 0b0000_0000_0000_1000
///     * 0b0000_0000_0001_0000
///     * 0b0000_0000_0010_0000
///     * 0b0000_0000_0100_0000
///     * 0b0000_0000_1000_0000
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shadow_generator(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, light: f64, pass_tag: f64) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    let scene: Entity = as_entity(scene);
    let light: Entity = as_entity(light);

    cmds.shadow_create.push(OpsShadowGenerator::ops(id, scene, light, PassTag::new(pass_tag as u16)));
    cmds.renderer_create.push(OpsRendererCreate::ops(id, String::from("Shadow") + id.to_bits().to_string().as_str(), id, PassTag::new(pass_tag as u16), false));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shadow_base_param(cmds: &mut CommandsExchangeD3, shadow: f64, bias: f64, normal_bias: f64, depthscale: f64) -> f64 {
    let shadow: Entity = as_entity(shadow);

    cmds.shadow_param.push(OpsShadowGeneratorParam::Bias(shadow, bias as f32));
    cmds.shadow_param.push(OpsShadowGeneratorParam::NormalBias(shadow, normal_bias as f32));
    cmds.shadow_param.push(OpsShadowGeneratorParam::DepthScale(shadow, depthscale as f32));

    as_f64(&shadow)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shadow_frustum(cmds: &mut CommandsExchangeD3, shadow: f64, frustum_size: f64, minz: f64, maxz: f64) {
    let shadow: Entity = as_entity(shadow);

    cmds.shadow_param.push(OpsShadowGeneratorParam::ShadowFrustumSize(shadow, frustum_size as f32));
    cmds.shadow_param.push(OpsShadowGeneratorParam::ShadowMinz(shadow, minz as f32));
    cmds.shadow_param.push(OpsShadowGeneratorParam::ShadowMaxz(shadow, maxz as f32));
}

