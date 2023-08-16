use std::mem::transmute;

use bevy::prelude::Entity;
use pi_slotmap::DefaultKey;

pub mod engine;
pub mod scene;
pub mod transform_node;
pub mod camera;
pub mod mesh;
pub mod mesh_builder;
pub mod instance_mesh;
pub mod geometry;
pub mod material;
pub mod lights;
pub mod node_materials;
pub mod texture;
pub mod gltf;
pub mod animation;
pub mod particle_system;

pub fn as_entity(val: f64) -> Entity {
    Entity::from_bits(val.to_bits())
}

pub fn as_f64(val: &Entity) -> f64 {
    unsafe { transmute(val.to_bits()) }
}

pub fn as_f64_dk(val: &DefaultKey) -> f64 {
    unsafe { transmute(*val) }
}
pub fn as_dk(val: &f64) -> DefaultKey {
    unsafe { transmute(*val) }
    // DefaultKey::from(val.to_bits())
}