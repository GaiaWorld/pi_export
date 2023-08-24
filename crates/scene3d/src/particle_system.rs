use pi_export_base::export::{Engine, Atom};
use pi_particle_system::prelude::{OpsCPUParticleSystem, OpsCPUParticleSystemState};
use pi_render::asset::TAssetKeyU64;

use crate::{engine::{ActionSetScene3D, GLTFRes, gltf_particle_calculator}, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    entity: f64,
    trailmesh: f64,
    trailgeo: f64,
    key: Atom,
) {
    let mut cmds = param.particlesys.get_mut(&mut app.world);
    let scene = as_entity(scene);
    let entity = as_entity(entity);
    let trailmesh = as_entity(trailmesh);
    let trailgeo = as_entity(trailgeo);
    if let Some(calculator) = cmds.calcultors.get(&key.asset_u64()) {
        cmds.particlesys_cmds.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_with_gltf(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    entity: f64,
    trailmesh: f64,
    trailgeo: f64,
    gltf: &GLTFRes,
    index_calculator: f64,
) {
    if let Some(calculator) = gltf_particle_calculator(gltf, index_calculator) {
        let mut cmds = param.particlesys.get_mut(&mut app.world);
        let scene = as_entity(scene);
        let entity = as_entity(entity);
        let trailmesh = as_entity(trailmesh);
        let trailgeo = as_entity(trailgeo);
        cmds.particlesys_cmds.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator.clone()));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_start(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    entity: f64,
) {
    let mut cmds = param.particlesys.get_mut(&mut app.world);
    let entity = as_entity(entity);
    
    cmds.particlesys_state_cmds.push(OpsCPUParticleSystemState::ops_start(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_timescale(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    entity: f64,
    speed: f64
) {
    let mut cmds = param.particlesys.get_mut(&mut app.world);
    let entity = as_entity(entity);
    
    cmds.particlesys_state_cmds.push(OpsCPUParticleSystemState::ops_speed(entity, speed as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_particle_system_stop(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    entity: f64,
) {
    let mut cmds = param.particlesys.get_mut(&mut app.world);
    let entity = as_entity(entity);
    
    cmds.particlesys_state_cmds.push(OpsCPUParticleSystemState::ops_stop(entity));
}