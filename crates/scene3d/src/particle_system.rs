use std::ops::Deref;

pub use pi_export_base::export::{Engine, Atom};
use pi_particle_system::prelude::{OpsCPUParticleSystem, OpsCPUParticleSystemState, ParticleAttribute, EParticleAttributeType};
use pi_render::asset::TAssetKeyU64;

pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::{ActionSetScene3D, GLTFRes, gltf_particle_calculator}, as_entity};

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
) {
	pi_export_base::export::await_last_frame(app);
    let scene = as_entity(scene);
    let entity = as_entity(entity);
    let trailmesh = as_entity(trailmesh);
    let trailgeo = as_entity(trailgeo);
    let reosurce = param.resource.get_mut(&mut app.world);
    if let Some(calculator) = reosurce.particlesys.calcultors.get(&key.asset_u64()) {
        let attrs = vec![
            ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
            ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.deref().clone() },
            ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.deref().clone() },
        ];
        cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator, attrs));
    }
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
) {
    if let Some(calculator) = gltf_particle_calculator(gltf, index_calculator) {
        let scene = as_entity(scene);
        let entity = as_entity(entity);
        let trailmesh = as_entity(trailmesh);
        let trailgeo = as_entity(trailgeo);
        let attrs = vec![
            ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
            ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.deref().clone() },
            ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.deref().clone() },
        ];
        cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator.clone(), attrs));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_start(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
) {
    let entity = as_entity(entity);
    
    cmds.parsys_state.push(OpsCPUParticleSystemState::ops_start(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_timescale(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
    speed: f64
) {
    let entity = as_entity(entity);
    
    cmds.parsys_state.push(OpsCPUParticleSystemState::ops_speed(entity, speed as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_stop(
    cmds: &mut CommandsExchangeD3,
    entity: f64,
) {
    let entity = as_entity(entity);
    
    cmds.parsys_state.push(OpsCPUParticleSystemState::ops_stop(entity));
}
