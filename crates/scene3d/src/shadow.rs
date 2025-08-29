
use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_slotmap::Key;

pub use crate::commands::CommandsExchangeD3;
use crate::{as_entity, as_f64, record::ERecordCMD};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

pub use pi_export_base::export::Engine;

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
pub fn p3d_shadow_generator(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: f64, light: f64, pass_tag: f64, graph: Option<f64>) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());


    let id: Entity = app.world.entities().reserve_entity();

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id));
    

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowGenerator(scene, light, as_f64(&id), pass_tag, graph));

    let scene: Entity = as_entity(scene);
    let light: Entity = as_entity(light);
    let graph = if let Some(graph) = graph { as_entity(graph) } else { Entity::null() };

    CommandsExchangeD3::p3d_shadow_generator(cmds, scene, light, id, pass_tag as u16, graph);
    // cmds.shadow_create.push(OpsShadowGenerator::ops(id, scene, light, PassTag::new(pass_tag as u16), graph));
    // cmds.renderer_create.push(OpsRendererCreate::ops(id, String::from("Shadow") + id.index().to_string().as_str(), id, PassTag::new(pass_tag as u16), false, false, false));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shadow_base_param(cmds: &mut CommandsExchangeD3, shadow: f64, bias: f64, normal_bias: f64, depthscale: f64) -> f64 {
    #[cfg(feature = "replay")]
    return ;

    let val =  EShadowGeneratorParam::Bias(bias as f32);
    let val1 =  EShadowGeneratorParam::NormalBias( normal_bias as f32);
    let val2: EShadowGeneratorParam =  EShadowGeneratorParam::DepthScale( depthscale as f32);


    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val));
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val1));
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val2));

    let shadow: Entity = as_entity(shadow);
    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val);
    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val1);
    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val2);

    as_f64(&shadow)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_shadow_frustum(cmds: &mut CommandsExchangeD3, shadow: f64, frustum_size: f64, minz: f64, maxz: f64) {
    #[cfg(feature = "replay")]
    return ;

    let val = EShadowGeneratorParam::ShadowFrustumSize( frustum_size as f32);
    let val1 = EShadowGeneratorParam::ShadowMinz( minz as f32);
    let val2 = EShadowGeneratorParam::ShadowMaxz( maxz as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val));
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val1));
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ShadowParam(shadow, val2));

    let shadow: Entity = as_entity(shadow);

    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val);
    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val1);
    CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val2);
}

