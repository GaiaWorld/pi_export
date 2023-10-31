use std::mem::transmute;

#[cfg(debug_assertions)]
use pi_ui_render::resource::DebugEntity;
pub use pi_export_base::export::Engine;
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::{InPassId, IsShow, LayoutResult, Quad, WorldMatrix, ZRange, EntityKey},
        pass_2d::{GraphId, ParentPassId},
        user::{Overflow, Point2}, NodeBundle,
    },
    prelude::UserCommands,
    resource::{QuadTree, fragment::NodeTag},
};
use pi_ui_render::system::RunState;
use pi_bevy_render_plugin::FrameState;

use bevy_ecs::{
	prelude::Entity,
    entity::Entities,
    query::QueryState,
    system::{Query, Res, SystemState},
};
use pi_bevy_ecs_extend::prelude::{Down, Layer, OrDefault, Up};
use pi_style::style::Aabb2;
use serde::{Serialize, Deserialize};
use js_proxy_gen_macro::pi_js_export;
#[cfg(feature="record")]
use pi_ui_render::system::cmd_play::{Records, CmdNodeCreate, PlayState, TraceOption };


#[cfg(target_arch = "wasm32")]
use pi_async_rt::prelude::{LocalTaskRunner, LocalTaskRuntime};
use pi_spatial::quad_helper::intersects;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct Gui {
    pub(crate) entitys: &'static Entities,
    pub(crate) commands: UserCommands,
	#[cfg(feature="record")]
	pub(crate) node_cmd: CmdNodeCreate,
	#[cfg(feature="record")]
	pub(crate) record_option: TraceOption,

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
		engine: &mut Engine,
	) -> Self {
		Gui {
			down_query: engine.world.query(),
			up_query: engine.world.query(),
			layer_query: engine.world.query(),
			enable_query: engine.world.query(),
			depth_query: engine.world.query(),
			layout_query: engine.world.query(),
			quad_query: engine.world.query(),
			matrix_query: engine.world.query(),
			overflow_query: engine.world.query(),
			in_pass2d_query: engine.world.query(),
			graph_id: engine.world.query(),
			query_state: SystemState::new(&mut engine.world),
			// 这里使用非安全的方法，将entities转为静态声明周期，外部需要保证entities使用期间， app的指针不能更改（如将App放入堆中就不可行）
			entitys: unsafe { transmute(engine.world.entities()) },
			commands: UserCommands::default(),
			#[cfg(feature="record")]
			node_cmd: CmdNodeCreate::default(),
			#[cfg(feature="record")]
			record_option: TraceOption::default(),
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

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_node(gui: &mut Gui) -> f64 {
	let entity = gui.entitys.reserve_entity();

	#[cfg(feature="record")]
	gui.node_cmd.0.push(entity);

	gui.commands.init_node(entity, NodeTag::Div);
	// log::warn!("entity :{:?}", entity);
	unsafe { transmute(entity.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_vnode(gui: &mut Gui) -> f64 {
	let entity = gui.entitys.reserve_entity();

	#[cfg(feature="record")]
	gui.node_cmd.0.push(entity);

	gui.commands.init_node(entity, NodeTag::VNode);
	unsafe { transmute(entity.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_text_node(gui: &mut Gui) -> f64 {
	let entity = gui.entitys.reserve_entity();

	#[cfg(feature="record")]
	gui.node_cmd.0.push(entity);

	gui.commands.init_node(entity, NodeTag::Span);
	unsafe { transmute(entity.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_image_node(gui: &mut Gui) -> f64 {
	let entity = gui.entitys.reserve_entity();

	#[cfg(feature="record")]
	gui.node_cmd.0.push(entity);

	gui.commands.init_node(entity, NodeTag::Image);
	unsafe { transmute(entity.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_canvas_node(gui: &mut Gui) -> f64 {
	let entity = gui.entitys.reserve_entity();

	#[cfg(feature="record")]
	gui.node_cmd.0.push(entity);

	gui.commands.init_node(entity, NodeTag::Canvas);
	unsafe { transmute(entity.to_bits()) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn destroy_node(gui: &mut Gui, node: f64) {
	let node = unsafe {Entity::from_bits(transmute::<f64, u64>(node))};
	gui.commands.destroy_node(node);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn remove_node(gui: &mut Gui, node: f64) {
	let node = unsafe {Entity::from_bits(transmute::<f64, u64>(node))};
	gui.commands.remove_node(node);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn insert_as_root(gui: &mut Gui, node: f64) {
	let node = unsafe {Entity::from_bits(transmute::<f64, u64>(node))};
	gui.commands.append(node, unsafe { transmute(EntityKey::null().to_bits())});
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn append_child(gui: &mut Gui, node: f64, parent: f64) {
	let node = unsafe {Entity::from_bits(transmute::<f64, u64>(node))};
	let parent = Entity::from_bits(unsafe { transmute(parent) });
	gui.commands.append(node, parent);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn insert_before(gui: &mut Gui, node: f64, borther: f64) {
	let node = unsafe {Entity::from_bits(transmute::<f64, u64>(node))};
	let borther = Entity::from_bits(unsafe { transmute(borther) });
	gui.commands.insert_before(node,borther,);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn create_fragment_by_bin(gui: &mut Gui, bin: &[u8]) {
	match postcard::from_bytes::<pi_ui_render::resource::fragment::Fragments>(bin) {
		Ok(r) => {
			gui.commands
				.extend_fragment_bin(pi_ui_render::resource::ExtendFragmentCmd(r));
		}
		Err(e) => {
			log::warn!("deserialize_fragment error: {:?}", e);
			return;
		}
	}
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn render_gui(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature="record")]
	if let TraceOption::Play = gui.record_option {
		loop {
			let records = engine.world.get_resource::<Records>().unwrap();
			let play_state = engine.world.get_resource::<PlayState>().unwrap();
			let cur_frame_count = play_state.cur_frame_count + 1;
			let next_state_index = play_state.next_state_index;
				
			if next_state_index < records.run_state.len() {
				let state = &records.run_state[play_state.next_state_index];
				if state.1 >= cur_frame_count {
					match state.0 {
						RunState::LAYOUT => calc_layout(gui, engine),
						RunState::MATRIX => calc_geo(gui, engine),
						_ => break,
					};
				} else {
					break;
				}
			} else {
				break;
			}
			
			
			let mut play_state = engine.world.get_resource_mut::<PlayState>().unwrap();
			play_state.next_state_index += 1;
		}
	}
	


	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("flush").entered();
	bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
	flush_data(gui, engine);
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::RENDER;
	
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn calc(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc").entered();
	bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
	flush_data(gui, engine);
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.update();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn calc_layout(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc_layout").entered();
	bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
	flush_data(gui, engine);
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::LAYOUT;
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.update();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn calc_geo(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc_geo").entered();
	bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
	flush_data(gui, engine);
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.update();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn set_is_run(engine: &mut Engine, value: bool) {
	// #[cfg(feature = "debug")]
	engine.world.get_resource_mut::<pi_ui_render::system::draw_obj::calc_text::IsRun>().unwrap().0 = value;
}

// 每帧取record
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_record(engine: &mut Engine) -> Vec<u8> {
	#[cfg(feature="record")]
	{
		let mut records = engine.world.get_resource_mut::<Records>().unwrap();

		let r = &*records;
		let r = match postcard::to_stdvec::<Records>(r) {
			Ok(bin) => bin,
			Err(r) =>{
				log::error!("serialize fail!!, {:?}", r);
				Vec::<u8>::default()
			},
		};
		records.clear();
		r
	}
	#[cfg(not(feature="record"))]
	Vec::<u8>::default()
}

// 取record长度, 单位：字节， 高层可根据长度来决定是否将record全部取出
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_record_len(engine: &mut Engine) -> u32 {
	#[cfg(feature="record")]
	{
		let records = engine.world.get_resource_mut::<Records>().unwrap();
		records.len() as u32
	}
	#[cfg(not(feature="record"))]
	0
}

// 设置下一帧的指令记录
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn set_next_record(engine: &mut Engine, bin: &[u8]) {
    // use pi_ui_render::system::cmd_play::PlayState;
	#[cfg(feature="record")]
	{
		match postcard::from_bytes::<Records>(bin) {
			Ok(r) => {
				engine.insert_resource(r);
				// 重设播放状态
				let mut play_state = engine.world.get_resource_mut::<PlayState>().unwrap();
				play_state.is_running = true;
				play_state.next_reord_index = 0;
				play_state.next_state_index = 0;
				play_state.cur_frame_count = 0;
				
			}
			Err(_e) => {
				();
				return;
			}
		}
	}
	
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn is_play_end(engine: &mut Engine) -> bool {
	#[cfg(feature="record")]
	match engine.world.get_resource_mut::<PlayState>() {
		Some(r) => !r.is_running,
		None => false,
	}
	#[cfg(not(feature="record"))]
	false
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn set_debug_entity(engine: &mut Engine, node_id: f64) {
	#[cfg(debug_assertions)]
	{
		let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
		engine.world.insert_resource(DebugEntity(EntityKey(node_id)));
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetDocument {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
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

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum BlendMode {
    Normal,
    AlphaAdd,
    Subtract,
    Multiply,
    OneOne,
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

#[inline]
fn flush_data(gui: &mut Gui, engine: &mut Engine) {
	let mut com = engine.world.get_resource_mut::<pi_ui_render::prelude::UserCommands>().unwrap();
	std::mem::swap(&mut gui.commands, &mut *com);
	
	#[cfg(feature="record")]
	if let TraceOption::Record = gui.record_option {
		if let Some(mut node_cmd) =  engine.world.get_resource_mut::<pi_ui_render::system::cmd_play::CmdNodeCreate>() {
			std::mem::swap(&mut gui.node_cmd, &mut *node_cmd);
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


