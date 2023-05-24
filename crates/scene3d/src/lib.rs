use std::mem::transmute;

use bevy::prelude::Entity;

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

#[cfg(target_arch = "wasm32")]
pub use engine::*;
#[cfg(target_arch = "wasm32")]
pub use scene::*;
#[cfg(target_arch = "wasm32")]
pub use transform_node::*;
#[cfg(target_arch = "wasm32")]
pub use camera::*;
#[cfg(target_arch = "wasm32")]
pub use mesh::*;
#[cfg(target_arch = "wasm32")]
pub use mesh_builder::*;
#[cfg(target_arch = "wasm32")]
pub use instance_mesh::*;
#[cfg(target_arch = "wasm32")]
pub use geometry::*;
#[cfg(target_arch = "wasm32")]
pub use material::*;
#[cfg(target_arch = "wasm32")]
pub use lights::*;

pub fn as_entity(val: f64) -> Entity {
    Entity::from_bits(val.to_bits())
}

pub fn as_f64(val: &Entity) -> f64 {
    unsafe { transmute(val.to_bits()) }
}