#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]
#![feature(type_name_of_val)]
#![feature(box_into_inner)]
#![feature(if_let_guard)]
#![feature(core_panic)]
#![feature(fmt_internals)]
#![feature(fmt_helpers_for_derive)]
#![feature(print_internals)]


use bevy::{ecs::{
    entity::Entities,
    query::QueryState,
    system::{Query, Res, SystemState},
}, prelude::{Deref, DerefMut}};
use pi_bevy_ecs_extend::prelude::{Down, Layer, OrDefault, Up};
use pi_null::Null;
use pi_style::style::Aabb2;
use std::mem::transmute;


#[cfg(target_arch = "wasm32")]
use pi_async::prelude::{SingleTaskRunner, SingleTaskRuntime};
// use pi_ecs::{
//     prelude::{DispatcherMgr, IntoSystem, ResMut, SingleDispatcher, StageBuilder},
//     world::World,
// };
use pi_spatialtree::quad_helper::intersects;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use bevy::app::prelude::App;

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




#[cfg(not(target_arch = "wasm32"))]
pub use native_index::*;
#[cfg(not(target_arch = "wasm32"))]
pub use native_debug::*;
#[cfg(not(target_arch = "wasm32"))]
pub use style::*;
#[cfg(target_arch = "wasm32")]
pub use wasm_index::*;

use pi_ui_render::{
    components::{
        calc::{InPassId, IsShow, LayoutResult, Quad, WorldMatrix, ZRange, EntityKey},
        pass_2d::{GraphId, ParentPassId},
        user::{Overflow, Point2},
    },
    prelude::UserCommands,
    resource::QuadTree,
};

#[cfg(feature="pi_js_export")]
pub struct Gui {
    pub entitys: &'static Entities,
    pub commands: UserCommands,
    pub down_query: QueryState<&'static Down>,
    pub up_query: QueryState<&'static Up>,
    pub layer_query: QueryState<&'static Layer>,
    pub enable_query: QueryState<&'static IsShow>,
    pub depth_query: QueryState<&'static ZRange>,
    pub layout_query: QueryState<&'static LayoutResult>,
    pub quad_query: QueryState<&'static Quad>,
    pub matrix_query: QueryState<&'static WorldMatrix>,
    pub overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    pub in_pass2d_query: QueryState<&'static InPassId>,
    pub graph_id: QueryState<&'static GraphId>,
    pub query_state: SystemState<(
        Res<'static, QuadTree>,
        Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
        Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    )>,
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub struct Gui {
    pub(crate) entitys: &'static Entities,
    pub(crate) commands: UserCommands,
    pub(crate) down_query: QueryState<&'static Down>,
    pub(crate) up_query: QueryState<&'static Up>,
    pub(crate) layer_query: QueryState<&'static Layer>,
    pub(crate) enable_query: QueryState<&'static IsShow>,
    pub(crate) depth_query: QueryState<&'static ZRange>,
    pub(crate) layout_query: QueryState<&'static LayoutResult>,
    pub(crate) quad_query: QueryState<&'static Quad>,
    pub(crate) matrix_query: QueryState<&'static WorldMatrix>,
    pub(crate) overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    pub(crate) in_pass2d_query: QueryState<&'static InPassId>,
    pub(crate) graph_id: QueryState<&'static GraphId>,
    pub(crate) query_state: SystemState<(
        Res<'static, QuadTree>,
        Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
        Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    )>,
}

impl Gui {
	pub fn new(
		entitys: &'static Entities,
		commands: UserCommands,
		down_query: QueryState<&'static Down>,
		up_query: QueryState<&'static Up>,
		layer_query: QueryState<&'static Layer>,
		enable_query: QueryState<&'static IsShow>,
		depth_query: QueryState<&'static ZRange>,
		layout_query: QueryState<&'static LayoutResult>,
		quad_query: QueryState<&'static Quad>,
		matrix_query: QueryState<&'static WorldMatrix>,
		overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
		in_pass2d_query: QueryState<&'static InPassId>,
		graph_id: QueryState<&'static GraphId>,
		query_state: SystemState<(
			Res<'static, QuadTree>,
			Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
			Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
		)>,) -> Self {

		Self {
			entitys, 
			commands,
			down_query,
			up_query,
			layer_query,
			enable_query,
			depth_query,
			layout_query,
			quad_query,
			matrix_query,
			overflow_query,
			in_pass2d_query,
			graph_id,
			query_state,
		}
	}
	pub fn entitys(&self) -> &'static Entities {
		self.entitys
	}

	pub fn commands(&self) -> &UserCommands {
		&self.commands
	}

	pub fn commands_mut(&mut self) -> &mut UserCommands {
		&mut self.commands
	}
}

/// 用点命中一个节点
#[allow(unused_attributes)]
pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
    let query = gui.query_state.get(&mut engine.world);

    let aabb = Aabb2::new(Point2::new(x, y), Point2::new(x, y));
    let mut args = AbQueryArgs {
        query: query.1,
        query_parent: query.2,
        aabb,
        result: EntityKey::null(),
        max_z: usize::MIN,
    };
    query.0.query(&aabb, intersects, &mut args, ab_query_func);
    if args.result.is_null() {
        None
    } else {
        Some(unsafe { transmute(args.result.to_bits()) })
    }
}

/// aabb的ab查询函数, aabb的oct查询函数应该使用intersects
fn ab_query_func(arg: &mut AbQueryArgs, id: EntityKey, aabb: &Aabb2, _bind: &()) {
    let (_layer, _is_show, z_range, inpass) = match arg.query.get(*id) {
        // 如果enable false 表示不接收事件, visibility为false， 也无法接收事件、不在树上也不能接收事件
        Ok(r) if (r.0.layer() != 0 && r.1.get_enable() && r.1.get_visibility()) => r,
        _ => return,
    };

    if intersects(&arg.aabb, aabb) {
        // 取最大z的node
        if z_range.start > arg.max_z {
            // 检查是否有裁剪，及是否在裁剪范围内
            let mut inpass = inpass.0;
            while !inpass.is_null() {
                if let Ok((parent, quad, oveflow)) = arg.query_parent.get(*inpass) {
                    inpass = parent.0;
                    if oveflow.0 {
                        if !intersects(&arg.aabb, quad) {
                            return; // 如果不想交，直接返回，该点不能命中该节点
                        }
                    }
                } else {
                    break;
                }
            }
            arg.result = id;
            arg.max_z = z_range.start;
        }
    }
}

pub struct AbQueryArgs<'s, 'w> {
    query: Query<'s, 'w, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
    query_parent: Query<'s, 'w, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    aabb: Aabb2,
    result: EntityKey,
    max_z: usize,
}
