pub use pi_export_base::export::{Engine, Atom};
use pi_particle_system::prelude::{OpsCPUParticleSystem, OpsCPUParticleSystemState};
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
) {
    let scene = as_entity(scene);
    let entity = as_entity(entity);
    let trailmesh = as_entity(trailmesh);
    let trailgeo = as_entity(trailgeo);
    let reosurce = param.resource.get_mut(&mut app.world);
    if let Some(calculator) = reosurce.particlesys.calcultors.get(&key.asset_u64()) {
        cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator));
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
) {
    if let Some(calculator) = gltf_particle_calculator(gltf, index_calculator) {
        let scene = as_entity(scene);
        let entity = as_entity(entity);
        let trailmesh = as_entity(trailmesh);
        let trailgeo = as_entity(trailgeo);
        cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator.clone()));
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
