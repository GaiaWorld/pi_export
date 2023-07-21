#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(type_name_of_val)]
#![feature(box_into_inner)]
#![feature(if_let_guard)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(print_internals)]
#![feature(once_cell)]


use bevy::ecs::{
    entity::Entities,
    query::QueryState,
    system::{Query, Res, SystemState},
};
use pi_bevy_asset::{AssetDesc, AssetConfig};
use pi_bevy_ecs_extend::prelude::{Down, Layer, OrDefault, Up};
use pi_hash::XHashMap;
use pi_null::Null;
use pi_render::{rhi::{asset::{RenderRes, TextureRes}, buffer::Buffer, pipeline::RenderPipeline, bind_group::BindGroup}, renderer::sampler::SamplerRes};
use pi_style::style::Aabb2;
use serde::{Serialize, Deserialize};
use wgpu::TextureView;
use std::mem::transmute;
use js_proxy_gen_macro::pi_js_export;


#[cfg(target_arch = "wasm32")]
use pi_async_rt::prelude::{LocalTaskRunner, LocalTaskRuntime};
// use pi_ecs::{
//     prelude::{DispatcherMgr, IntoSystem, ResMut, SingleDispatcher, StageBuilder},
//     world::World,
// };
use pi_spatial::quad_helper::intersects;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
pub mod native_index;
#[cfg(not(target_arch = "wasm32"))]
pub mod native_debug;
pub mod rr;
#[cfg(target_arch = "wasm32")]
pub mod wasm_debug;
#[cfg(target_arch = "wasm32")]
pub mod wasm_index;

pub mod style;
pub mod index;






// // 设置资源配置
// pub fn set_asset_cfg() {

// }

// (ShareAssetMgr::<RenderRes<TextureView>>::new_with_config(
// 	|                                                       ^^^^^^^^^^^^^^^ function or associated item not found in `ShareAssetMgr<RenderRes<TextureView>>`
 
//  error[E0599]: no function or associated item named `new_with_config` found for struct `ShareAssetMgr<RenderRes<pi_render::rhi::buffer::Buffer>>` in the current scope
// 	--> D:\0_rust\pi_render_bevy\crates\render\src\plugin.rs:116:40
// 	 |
//  116 |             ShareAssetMgr::<RenderRes<Buffer>>::new_with_config(
// 	 |                                                 ^^^^^^^^^^^^^^^ function or associated item not found in `ShareAssetMgr<RenderRes<Buffer>>`
 
//  error[E0599]: no function or associated item named `new_with_config` found for struct `ShareAssetMgr<pi_render::rhi::sampler::Sampler>` in the current scope
// 	--> D:\0_rust\pi_render_bevy\crates\render\src\plugin.rs:128:33
// 	 |
//  128 |             ShareAssetMgr::<SamplerRes>::new_with_config(
// 	 |                                          ^^^^^^^^^^^^^^^ function or associated item not found in `ShareAssetMgr<Sampler>`
 
//  error[E0599]: no function or associated item named `new_with_config` found for struct `ShareAssetMgr<RenderRes<pi_render::rhi::bind_group::BindGroup>>` in the current scope
// 	--> D:\0_rust\pi_render_bevy\crates\render\src\plugin.rs:139:43
// 	 |
//  139 |             ShareAssetMgr::<RenderRes<BindGroup>>::new_with_config(
// 	 |                                                    ^^^^^^^^^^^^^^^ function or associated item not found in `ShareAssetMgr<RenderRes<BindGroup>>`
 
//  error[E0599]: no function or associated item named `new_with_config` found for struct `ShareAssetMgr<TextureRes>` in the current scope
// 	--> D:\0_rust\pi_render_bevy\crates\render\src\plugin.rs:150:33
// 	 |
//  150 |             ShareAssetMgr::<TextureRes>::new_with_config(
// 	 |                                          ^^^^^^^^^^^^^^^ function or associated item not found in `ShareAssetMgr<TextureRes>`
 
//  error[E0599]: no function or associated item named `new_with_config` found for struct `ShareAssetMgr<RenderRes<pi_render::rhi::pipeline::RenderPipeline>>` in the current scope
// 	--> D:\0_rust\pi_render_bevy\crates\render\src\plugin.rs:161:48
// 	 |
//  161 |             ShareAssetMgr::<RenderRes<RenderPipeline>>::new_with_config(

// 	error[E0599]: no function or associated item named `new` found for struct `ShareAssetMgr<D>` in the current scope
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load.rs:278:55
//     |
// 278 |             world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                                       ^^^ function or associated item not found in `ShareAssetMgr<D>`
//     |
// note: the function `new` is implemented on `std::sync::Arc<AssetMgr<D>>`
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load.rs:278:35
//     |
// 278 |             world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                   ^^^^^^^^^^^^^^^^^^

// error[E0599]: no function or associated item named `new` found for struct `ShareAssetMgr<D>` in the current scope
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load.rs:329:55
//     |
// 329 |             world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                                       ^^^ function or associated item not found in `ShareAssetMgr<D>`
//     |
// note: the function `new` is implemented on `std::sync::Arc<AssetMgr<D>>`
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load.rs:329:35
//     |
// 329 |             world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                   ^^^^^^^^^^^^^^^^^^

// error[E0599]: no function or associated item named `new` found for struct `ShareAssetMgr<D>` in the current scope
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load_option.rs:266:59
//     |
// 266 |             app.world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                                           ^^^ function or associated item not found in `ShareAssetMgr<D>`
//     |
// note: the function `new` is implemented on `std::sync::Arc<AssetMgr<D>>`
//    --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\assets\sync_load_option.rs:266:39
//     |
// 266 |             app.world.insert_resource(ShareAssetMgr::<D>::new(GarbageEmpty(), self.0, self.1, self.2));
//     |                                       ^^^^^^^^^^^^^^^^^^

// error[E0599]: no function or associated item named `new` found for struct `ShareAssetMgr<base::TypeFrameCurve<D>>` in the current scope
//   --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_engine_shell\src\animation\mod.rs:68:71
//    |
// 68 |         app.world.insert_resource(ShareAssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), self.0, self.1, self.2));
// no function or associated item named `create` found for struct `pi_engine_shell::prelude::ShareAssetMgr<pi_engine_shell::prelude::BindGroup>` in the current scope
// --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_scene_context\src\bindgroup\mod.rs:96:57
//  |
// 96 |         app.insert_resource(ShareAssetMgr::<BindGroup>::create(GarbageEmpty(), false, &cfg));
//  |                                                         ^^^^^^ function or associated item not found in `ShareAssetMgr<BindGroup>`
//  |
//  = help: items from traits can only be used if the trait is implemented and in scope
//  = note: the following traits define an item `create`, perhaps you need to implement one of them:
// 		 candidate #1: `vertex_buffer_useinfo::AsKeyVertexBuffer`
// 		 candidate #2: `geometry::geometry::RenderVerticesFrom`
// 		 candidate #3: `geometry::geometry::RenderIndicesFrom`

// error[E0599]: no function or associated item named `create` found for struct `pi_engine_shell::prelude::ShareAssetMgr<pi_engine_shell::prelude::BindGroupLayout>` in the current scope
// --> C:\Users\chuanyan\.cargo\git\checkouts\pi_3d-d047a1290caebb2b\69852a0\crates\pi_scene_context\src\bindgroup\mod.rs:97:63
//  |
// 97 |         app.insert_resource(ShareAssetMgr::<BindGroupLayout>::create(GarbageEmpty(), false, &cfg));
