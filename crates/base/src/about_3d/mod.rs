pub mod action;
pub mod animation;
pub mod constants;
pub mod geometry;
pub mod record;
pub mod engine;
pub mod mesh;
pub mod node_materials;
pub mod sprite;
pub mod commands;
pub mod cmd_call;

use pi_assets::asset::Handle;
use pi_gltf2_load::GLTF;
use pi_hash::XHashMap;
use pi_scene_context::pass::Resource;
use pi_slotmap::DefaultKey;
use std::mem::transmute;
use pi_world::world::Entity;

pub use action::*;
pub use animation::*;
pub use constants::*;

pub fn as_entity(val: f64) -> Entity {
    // Entity::from_bits(val.to_bits())
    unsafe { transmute(val) }
}

pub fn as_f64(val: &Entity) -> f64 {
    // unsafe { transmute(val.to_bits()) }
    unsafe { transmute(*val) }
}

pub fn as_f64_dk(val: &DefaultKey) -> f64 {
    unsafe { transmute(*val) }
}
pub fn as_dk(val: &f64) -> DefaultKey {
    unsafe { transmute(*val) }
    // DefaultKey::from(val.to_bits())
}