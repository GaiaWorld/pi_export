use std::ops::Deref;

use pi_export_base::engine::gltf_particle_calculator;
pub use pi_export_base::export::{Engine, Atom};
use pi_particle_system::prelude::{ECPUParticleSystemState, EParticleAttributeType, OpsCPUParticleSystem, OpsCPUParticleSystemState, ParticleAttribute};
use pi_render::asset::TAssetKeyU64;

pub use crate::commands::CommandsExchangeD3;
use crate::record::ERecordCMD;
pub use crate::{engine::{ActionSetScene3D, GLTFRes}, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    cmds: &mut CommandsExchangeD3,
    scene: f64,
    entity: f64,
    trailmesh: f64,
    trailgeo: f64,
    key: &Atom,
    color_attr_key: &Atom,
    tilloff_attr_key: &Atom,
    update_buffer_interval_frame: Option<f64>,
) {
    #[cfg(feature = "replay")]
    return ;

	pi_export_base::export::await_last_frame(app);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::PARTICLESYS(scene, entity, trailmesh, trailgeo, 
        key.deref().asset_u64(), color_attr_key.deref().clone(), tilloff_attr_key.deref().clone(), update_buffer_interval_frame
    ));

    let mut resource = param.resource.get_mut(&mut app.world);
    let scene = as_entity(scene);
    let entity = as_entity(entity);
    let trailmesh = as_entity(trailmesh);
    let trailgeo = as_entity(trailgeo);
    CommandsExchangeD3::p3d_particle_system(cmds, scene, entity, trailmesh, trailgeo, key.deref().asset_u64(), color_attr_key.deref(), tilloff_attr_key.deref(), update_buffer_interval_frame);

    // let reosurce = param.resource.get_mut(&mut app.world);
    // if let Some(calculator) = reosurce.particlesys.calcultors.get(&key.asset_u64()) {
    //     let attrs = vec![
    //         ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
    //         ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.deref().clone() },
    //         ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.deref().clone() },
    //     ];
    //     let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
    //         (update_buffer_interval_frame as u8)
    //     } else { 0 };

    //     cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator, attrs, update_buffer_interval_frame));
    // }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_with_gltf(
    cmds: &mut CommandsExchangeD3,
    scene: f64,
    entity: f64,
    trailmesh: f64,
    trailgeo: f64,
    gltf: &GLTFRes,
    index_calculator: f64,
    color_attr_key: &Atom,
    tilloff_attr_key: &Atom,
    update_buffer_interval_frame: Option<f64>,
) {
    #[cfg(feature = "replay")]
    return ;

    if let Some(calculator) = gltf_particle_calculator(&cmds, gltf, index_calculator) {
        let calculator =  *calculator.key();
        #[cfg(feature = "record")]
        cmds.record(ERecordCMD::PARTICLESYS(scene, entity, trailmesh, trailgeo, calculator, color_attr_key.deref().clone(), tilloff_attr_key.deref().clone(), update_buffer_interval_frame));

        let scene = as_entity(scene);
        let entity = as_entity(entity);
        let trailmesh = as_entity(trailmesh);
        let trailgeo = as_entity(trailgeo);

        CommandsExchangeD3::p3d_particle_system(cmds, scene, entity, trailmesh, trailgeo, calculator, color_attr_key.deref(), tilloff_attr_key.deref(), update_buffer_interval_frame);

        // let attrs = vec![
        //     ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
        //     ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.deref().clone() },
        //     ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.deref().clone() },
        // ];
        // let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
        //     (update_buffer_interval_frame as u8)
        // } else { 0 };
        // cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator.clone(), attrs, update_buffer_interval_frame));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_start(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECPUParticleSystemState::Start();

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ParticlesysState(entity, val));

    let entity = as_entity(entity);
    CommandsExchangeD3::p3d_particle_system_state(cmds, entity, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_timescale(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
    speed: f64
) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECPUParticleSystemState::TimeScale(speed as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ParticlesysState(entity, val));

    let entity = as_entity(entity);
    
    CommandsExchangeD3::p3d_particle_system_state(cmds, entity, val);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_stop(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    let val = ECPUParticleSystemState::Stop();
    
    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::ParticlesysState(entity, val));

    let entity = as_entity(entity);
    
    CommandsExchangeD3::p3d_particle_system_state(cmds, entity, val);
}
