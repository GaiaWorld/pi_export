use std::cell::RefCell;
use std::rc::Rc;
use std::{mem::transmute, sync::Arc};
use std::any::Any;

use pi_bevy_asset::ShareAssetMgr;
use pi_export_base::export::await_last_frame;
use pi_flex_layout::{prelude::CharNode, style::{PositionType, FlexWrap, FlexDirection, AlignContent, AlignItems, AlignSelf, JustifyContent, Display, Dimension}};
use pi_render::rhi::asset::TextureRes;
use pi_slotmap::DefaultKey;
#[cfg(debug_assertions)]
use pi_ui_render::resource::DebugEntity;
pub use pi_export_base::export::Engine;
use pi_null::Null;
use pi_ui_render::{
    components::{
        calc::{InPassId, IsShow, LayoutResult, Quad, WorldMatrix, ZRange, EntityKey},
        pass_2d::ParentPassId,
        user::{Overflow, Point2, NodeState, Vector4},
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
use pi_style::{style::{Aabb2, FitType, ImageRepeatOption, TextAlign, VerticalAlign, WhiteSpace, FontStyle, LineHeight, Color}, style_parse::Attribute};
use serde::{Serialize, Deserialize};
use js_proxy_gen_macro::pi_js_export;
#[cfg(feature="record")]
use pi_ui_render::system::cmd_play::{Records, CmdNodeCreate, PlayState, TraceOption };
pub use pi_export_base::export::Atom as Atom1;
use pi_ui_render::system::res_load::ResSuccess;


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
    pub(crate) enable_query: QueryState<&'static IsShow>,
    pub(crate) layout_query: QueryState<&'static LayoutResult>,
    pub(crate) quad_query: QueryState<&'static Quad>,
    pub(crate) query_state: SystemState<(
        Res<'static, QuadTree>,
        Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
        Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    )>,
	pub (crate) res_await_list: Vec<pi_atom::Atom>,

	// pub(crate) depth_query: QueryState<&'static ZRange>,
	// pub(crate) layer_query: QueryState<&'static Layer>,
    // pub(crate) matrix_query: QueryState<&'static WorldMatrix>,
    // pub(crate) overflow_query: QueryState<(&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    // pub(crate) in_pass2d_query: QueryState<&'static InPassId>,
    // pub(crate) graph_id: QueryState<&'static GraphId>,
}

impl Gui {
	pub fn new(
		engine: &mut Engine,
	) -> Self {
		pi_export_base::export::await_last_frame(engine);
		Gui {
			down_query: engine.world.query(),
			up_query: engine.world.query(),
			enable_query: engine.world.query(),
			layout_query: engine.world.query(),
			quad_query: engine.world.query(),
			query_state: SystemState::new(&mut engine.world),
			// 这里使用非安全的方法，将entities转为静态声明周期，外部需要保证entities使用期间， app的指针不能更改（如将App放入堆中就不可行）
			entitys: unsafe { transmute(engine.world.entities()) },
			commands: UserCommands::default(),
			#[cfg(feature="record")]
			node_cmd: CmdNodeCreate::default(),
			#[cfg(feature="record")]
			record_option: TraceOption::default(),
			res_await_list: Vec::default(),

			// depth_query: engine.world.query(),
			// layer_query: engine.world.query(),
			// matrix_query: engine.world.query(),
			// overflow_query: engine.world.query(),
			// in_pass2d_query: engine.world.query(),
			// graph_id: engine.world.query(),
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
	await_last_frame(engine);
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
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
	
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn calc(gui: &mut Gui, engine: &mut Engine) {
	await_last_frame(engine);

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
	await_last_frame(engine);
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
	await_last_frame(engine);
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc_geo").entered();
	bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
	flush_data(gui, engine);
	*engine.world.get_resource_mut::<RunState>().unwrap() = RunState::MATRIX;
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.update();
}

// 取到keyframes(应该在每帧开始前取上一帧产生的事件， 因为在本地平台帧推被异步出去，合适完成帧推目前没有设计回调， 因此事件总是延迟一帧，但应该问题不大)
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_keyframes(engine: &mut Engine, name: &Atom1, scope_hash: u32) -> String {
	pi_export_base::export::await_last_frame(engine);
	let sheet = engine.world.get_resource::<pi_ui_render::resource::animation_sheet::KeyFramesSheet>().unwrap();
	match sheet.get_keyframes((**name).clone(), scope_hash as usize) {
		Some(r) => {
			let mut temp_str_arr = Vec::new();
			let mut ret = Vec::new();
			for (_process, attrs) in r.iter() {
				for attr in attrs.iter() {
					let rr = to_css_str(attr);
					if rr.0 != "" {
						temp_str_arr.push("\"".to_string() + rr.0 + "\": \"" + rr.1.as_str() + "\"");
					}
				}
				ret.push("{".to_string() + temp_str_arr.join(",").as_str() + "}");
				temp_str_arr.clear();
			}
			
			return "[".to_string() + ret.join(",").as_str() + "]";
		},
		None => return "".to_string(),
	};
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
	pi_export_base::export::await_last_frame(engine);
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
	pi_export_base::export::await_last_frame(engine);
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
	pi_export_base::export::await_last_frame(engine);
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

/**
 * -buffer Vec<Vec<u8>>
 */
#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn sdf_on_load(key: f64, buffer: js_sys::Array) {
	let mut v = Vec::new();
	for i in buffer.iter() {
		v.push(js_sys::Uint8Array::from(i).to_vec());
	}
	pi_hal::font::sdf_brush::on_load(unsafe {transmute::<_, DefaultKey>(key)}, v);
}

#[pi_js_export]
pub struct SdfVec(Vec<Vec<u8>>);

#[pi_js_export]
pub fn create_vec_sdf() -> SdfVec {
	SdfVec(Vec::new())
}

#[pi_js_export]
pub fn sdf_push(vec: &mut SdfVec, buffer: Vec<u8>) {
	vec.0.push(buffer);
}

#[cfg(not(target_arch="wasm32"))]
#[pi_js_export]
pub fn sdf_on_load(key: f64, buffer: &mut SdfVec) {
	let r = std::mem::replace(&mut buffer.0, Vec::new());
	pi_hal::font::sdf_brush::on_load(unsafe {transmute::<_, DefaultKey>(key)}, r);
}

// 添加sdf字体
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn add_sdf_font(gui: &mut Gui, bin: &[u8]) {
	let cfg = match postcard::from_bytes::<pi_hal::font::sdf_brush::FontCfg>(bin) {
		Ok(r) => r,
		Err(e) => {
			log::info!("parse sdf cfg fail, {:?}", e);
			return;
		}
	};
	gui.commands.add_sdf_font(cfg);
	// font_sheet = 
	// let mut v = Vec::new();
	// for i in buffer.iter() {
	// 	v.push(js_sys::Uint8Array::from(i).to_vec());
	// }
	// pi_hal::font::sdf_brush::on_load(unsafe {transmute::<_, DefaultKey>(key)}, v);
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


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn query_text(engine: &mut Engine, node_id: f64, x: f32, y: f32) -> CharPos {
	pi_export_base::export::await_last_frame(engine);
	let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
	query_text1(engine, node, x, y)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_text_pos(gui: &mut Gui, engine: &mut Engine, node_id: f64, index: u32) -> CharPos {
	let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
	get_text_pos1(gui, engine, node, index as usize)
}

/// 是否存在资源
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn has_res(engine: &mut Engine, path: &Atom1) -> bool {
	// 暂时只支持纹理资源访问
	if path.ends_with(".png") || path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".ktx") || path.ends_with(".ktx2") {
		let reses = engine.world.get_resource_mut::<ShareAssetMgr<TextureRes>>().unwrap();
		return reses.get(&(path.get_hash() as u64)).is_some()
	}
	false
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ResHandle(Arc<dyn Any + Send + Sync + 'static>);

/// 加载资源
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn load_res(gui: &mut Gui, path: &Atom1) {
	gui.res_await_list.push((**path).clone());
}

/// 获取已经加载成功的资源
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_success_res_len(engine: &mut Engine) -> u32 {
	if let Some(res_success) = engine.world.get_resource::<ResSuccess>() {
		return( res_success.async_list.len() + res_success.sync_list.len()) as u32
	} else {
		0
	}
}

// pub fn get_success_res(engine: &mut Engine, result: &mut [ResHandle], result_keys: &mut [u32]) {
// 	let mut res_success = engine.world.get_resource_mut::<ResSuccess>().unwrap();
// 	let res_success = &mut *res_success;
// 	let mut i = 0;
// 	while let Some(r) = res_success.async_list.pop() {
// 		result[i] = ResHandle(r.1);
// 		result_keys[i] = r.0.get_hash() as u32;
// 		i += 1;
// 	}

// 	for r in res_success.sync_list.drain(..) {
// 		result[i] = ResHandle(r.1);
// 		result_keys[i] = r.0.get_hash() as u32;
// 		i += 1;
// 	}
// }

#[cfg(not(target_arch="wasm32"))]
fn get_vid(scope: &mut v8::HandleScope) -> usize {
    let v8c = scope
        .get_slot::<Rc<RefCell<vm_builtin::V8InstanceContext>>>()
        .unwrap()
        .clone();

    let vid = v8c.borrow().get_vid();
    vid
}

#[cfg(not(target_arch="wasm32"))]
pub fn get_success_res(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
	let context = scope.get_current_context();
    let scope = &mut v8::ContextScope::new(scope, context);

	let engine: v8::Local<'_, v8::Value> = args.get(0);
    if !engine.is_object() {
        let msg = v8::String::new(scope, "Invalid arguments 0th param!!!").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }
	let vid = get_vid(scope);
	let engine = match vm_builtin::NativeObjectValue::from_native_object(scope, vid, vm_builtin::ContextHandle(0 as usize), engine) {
		Err(_e) => panic!("arg error"),
		Ok(arg) => if let vm_builtin::NativeObjectValue::NatObj(o) = arg {o} else {panic!("arg error");},
	};
	let engine = if let Some(o) = engine.get_mut::<Engine>(vid, 0){
		o
	} else {
		panic!("arg error");
	};

	let result: v8::Local<'_, v8::Value> = args.get(1);
    if !result.is_array() {
        let msg = v8::String::new(scope, "Invalid arguments 1th param!!!").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }
	let result: v8::Local<v8::Array> = unsafe { v8::Local::cast(result) };

	let result_keys: v8::Local<'_, v8::Value> = args.get(2);
    if !result_keys.is_uint32_array() {
        let msg = v8::String::new(scope, "Invalid arguments 2th param!!!").unwrap();
        let exception = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exception);
        return;
    }
	let result_keys: v8::Local<v8::Uint32Array> = unsafe { v8::Local::cast(result_keys) };

	let mut res_success = engine.world.get_resource_mut::<ResSuccess>().unwrap();
	let res_success = &mut *res_success;
	let mut i = 0;
	while let Some(r) = res_success.async_list.pop() {
		let o = vm_builtin::NativeObjectValue::NatObj(vm_builtin::external::NativeObject::new_owned(ResHandle(r.1))).into_native_object(scope);
		result.set_index(scope, i, o.into());
		let n = v8::Number::new(scope,  r.0.get_hash() as f64).into();
		result_keys.set_index(scope, i, n);
		i += 1;
	}

	for r in res_success.sync_list.drain(..) {
		let o = vm_builtin::NativeObjectValue::NatObj(vm_builtin::external::NativeObject::new_owned(ResHandle(r.1))).into_native_object(scope);
		result.set_index(scope, i, o.into());
		let n = v8::Number::new(scope,  r.0.get_hash() as f64).into();
		result_keys.set_index(scope, i, n);
		i += 1;
	}
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn get_success_res(engine: &mut Engine, result: js_sys::Array, result_keys: &mut [u32]) {
	let mut res_success = engine.world.get_resource_mut::<ResSuccess>().unwrap();
	let res_success = &mut *res_success;
	let mut i = 0;
	while let Some(r) = res_success.async_list.pop() {
		result.set(i, ResHandle(r.1).into());
		result_keys[i as usize] = r.0.get_hash() as u32;
		i += 1;
	}

	for r in res_success.sync_list.drain(..) {
		result.set(i, ResHandle(r.1).into());
		result_keys[i as usize] = r.0.get_hash() as u32;
		i += 1;
	}
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct CharPos {
	index: i32,
	x: f32,
	y: f32,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
impl CharPos {
	#[pi_js_export]
	pub fn index(&self) -> i32 {
		self.index
	}
	#[pi_js_export]
	pub fn x(&self) -> f32 {
		self.x
	}
	#[pi_js_export]
	pub fn y(&self) -> f32 {
		self.y
	}
}

// #[wasm_bindgen]
pub fn get_text_pos1(gui: &mut Gui, engine: &mut Engine, node: Entity, index: usize) -> CharPos {
	calc_layout(gui, engine);
	// log::info!("get_text_pos====={}", index);

	let mut char_pos = CharPos {
		index: 0,
		x: 0.0,
		y:0.0,
	};

	let node_state = match engine.world.get::<NodeState>(node) {
		Some(r) => r,
		None => return char_pos,
	};

	let len = node_state.text.len();
	if len == 0 {
		return char_pos
	}

	let text = &node_state.text;
	let mut i = index;
	if index >= len {
		i = len - 1;
	}
	let mut char = &text[i];

	// log::info!("get_text_pos start=====i: {}, char_i:{}, len:{}", i, char.char_i, len);

	// 跳过char_i为-1的节点
	while char.char_i == -1 && i < len-1 {
		i += 1;
		char =  &text[i]
	}
	while char.char_i == -1 && i > 0 {
		i -= 1;
		char =  &text[i]
	}
	if char.char_i == -1 {
		return char_pos
	}

	// log::info!("get_text_pos start i====={}, {}, {:?}", i, char.char_i, text);

	if char.char_i != index as isize {
		let diff;
		if index as isize > char.char_i {
			diff = 1;
		} else {
			diff = -1;
		}
		while char.char_i != index as isize {
			// log::info!("get_text_pos loop i====={}, {}, {}", i, index, char.char_i);
			let r = i as isize + diff;
			if r < 0 {
				i = 0;
				break;
			}
			i = r as usize;
			if i < len {
				char = &text[i];
			} else {
				break;
			}
		}
	}

	// log::info!("get_text_pos end i====={}, {}, {}", i, index, char.char_i);
	if char.char_i == -1 {
		return char_pos
	}

	let pos = calc_text_pos(char, text);
	set_text_pos(i, &mut char_pos, text, pos);
	char_pos
	
	
	// match JsValue::from_serde(&char_pos) {
	// 	Ok(r) => r,
	// 	Err(_e) => {
	// 		log::info!("serde char_pos fail");
	// 		panic!();
	// 	}
	// }
}

fn query_text1(engine: &mut Engine, node: Entity, x: f32, y: f32) -> CharPos {

	let mut query_state = engine.world.query::<(&NodeState, &WorldMatrix)>();
	let mut char_pos = CharPos {
		index: 0,
		x: 0.0,
		y:0.0,
	};

	let (node_state, matrix) = match query_state.get(&mut engine.world, node) {
		Ok(r) => r,
		_ => return char_pos,
	};
	let mut start = 0;
	let mut end = node_state.text.len();

	let text = &node_state.text;
	// log::info!("world_matrix======{:?}", world_matrix);
	let invert = matrix.invert().unwrap();
	// log::info!("invert======{:?}", invert);
	let p = invert.0 * Vector4::new(x, y, 1.0, 1.0);  // 得到相对于当前节点布局左上角的位置
	let mut pos = (0.0, 0.0,0.0,0.0);
	while start < end {
		let diff = (end - start)/2 + 1;
		let mut cur = end - diff;
		// log::info!("text======{:?}, {}, {}", cur, start, end);
		let mut char = &text[cur];
		// 跳过没有意义的字符
		while char.char_i == -1 && cur > start {
			cur = cur - 1;
			char = &text[cur];
		}
		if char.char_i == -1 && cur == start {
			start = (end - diff) + 1;
			continue;
		}

		pos = calc_text_pos(char, text);
		let center_x = (pos.0 + pos.2)/2.0;
		// let center_y = (pos.1 + pos.3)/2.0;

		// log::info!("p: {}, {}, char_pos:{:?}, char_size: {:?}, index:{:?}, char_i:{}, context_id:{}, pos:{:?}, cur:{:?}", p.x, p.y, char.pos, char.size, cur, char.char_i, char.context_id, pos, cur);
		if pos.0 > p.x {
			if pos.3 >= p.y {
				end = cur;
			} else {
				start = cur + 1;
			}
		} else if pos.2 < p.x{
			if pos.1 <= p.y {
				start = cur + 1;
			} else {
				end = cur;
			}
			
		} else {
			if pos.1 > p.y {
				end = cur;
			} else if pos.3 < p.y {
				start = cur + 1;
			} else if center_x > p.x {
				start = cur;
				break;
			} else {
				start = cur + 1;
			}
		}
	}

	// log::info!("start: {}, pos:{:?}", start, pos);
	set_text_pos(start, &mut char_pos, text, pos);

	char_pos
}

fn set_text_pos(index: usize, char_pos: &mut CharPos, text: &Vec<CharNode>, r: (f32,f32,f32,f32)) {
	let len = text.len();
	// log::info!("set_text_pos: {:?}, index: {:?}, text:{:?}", len, index, text);
	if len > 0 {
		let char;
		if index < len {
			// log::info!("set_text_pos1: {:?}, index: {:?}", len, index);
			char = &text[index];
			// log::info!("set_text_pos1 end: {:?}, index: {:?}", len, index);
			char_pos.x = r.0;
			char_pos.y = r.1;
			char_pos.index = char.char_i as i32;
		} else {
			// log::info!("set_text_pos2: {:?}, index: {:?}", len, len - 1);
			char = &text[len - 1];
			// log::info!("set_text_pos2 end: {:?}, index: {:?}", len, len - 1);
			char_pos.x = r.2;
			char_pos.y = r.1;
			char_pos.index = (char.char_i + 1) as i32;
		}
	}
}

fn calc_text_pos(char: &CharNode, text: &Vec<CharNode>) -> (f32, f32, f32, f32) {
	let mut r = (char.pos.left, char.pos.top, char.pos.right, char.pos.bottom);
	if char.context_id > -1 {
		let context_s = &text[char.context_id as usize];
		// log::info!("calc_text_pos====context_id: {}, {:?}", char.context_id, context_s);
		r.0 += context_s.pos.left;
		r.1 += context_s.pos.top;
		r.2 += context_s.pos.left;
		r.3 += context_s.pos.top;
	}
	r
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

	if let Some(mut com) = engine.world.get_resource_mut::<pi_ui_render::system::res_load::ResList>() {
		std::mem::swap(&mut gui.res_await_list, &mut com.await_list);
	};
	
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

pub fn to_css_str(attr: &Attribute) -> (&'static str, String) {
    match attr {
        Attribute::ClipPath(_) => todo!(),
		Attribute::AsImage(r) => ("as-image", match r.0 {
			pi_style::style::AsImage::None => "none".to_string(),
			pi_style::style::AsImage::Advise => "advise".to_string(),
			pi_style::style::AsImage::Force => "force".to_string(),
		}),
        Attribute::PositionType(r) => ("position", match r.0 {
            PositionType::Relative => "relative".to_string(),
            PositionType::Absolute => "absolute".to_string(),
        }),
        Attribute::FlexWrap(r) => ("flex-wrap", match r.0 {
            FlexWrap::NoWrap => "nowrap".to_string(),
            FlexWrap::Wrap => "wrap".to_string(),
            FlexWrap::WrapReverse => "wrapreverse".to_string(),
        }),
        Attribute::FlexDirection(r) => ("flex-direction", match r.0 {
            FlexDirection::Column => "column".to_string(),
            FlexDirection::ColumnReverse => "columnreverse".to_string(),
            FlexDirection::Row => "row".to_string(),
            FlexDirection::RowReverse => "rowreverse".to_string(),
        }),
        Attribute::AlignContent(r) => ("align-content", match r.0 {
            // AlignContent::Auto => "auto".to_string(),
            AlignContent::FlexStart => "flex-start".to_string(),
            AlignContent::Center => "center".to_string(),
            AlignContent::FlexEnd => "flex-end".to_string(),
            AlignContent::Stretch => "stretch".to_string(),
            // AlignContent::Baseline => "baseline".to_string(),
            AlignContent::SpaceBetween => "space-between".to_string(),
            AlignContent::SpaceAround => "space-around".to_string(),
        }),
        Attribute::AlignItems(r) => ("align-items", match r.0 {
            // AlignItems::Auto => "auto".to_string(),
            AlignItems::FlexStart => "flex-start".to_string(),
            AlignItems::Center => "center".to_string(),
            AlignItems::FlexEnd => "flex-end".to_string(),
            AlignItems::Stretch => "stretch".to_string(),
            AlignItems::Baseline => "baseline".to_string(),
            // AlignItems::SpaceBetween => "space-between".to_string(),
            // AlignItems::SpaceAround => "space-around".to_string(),
        }),
        Attribute::AlignSelf(r) => ("align-self", match r.0 {
            AlignSelf::Auto => "auto".to_string(),
            AlignSelf::FlexStart => "flex-start".to_string(),
            AlignSelf::Center => "center".to_string(),
            AlignSelf::FlexEnd => "flex-end".to_string(),
            AlignSelf::Stretch => "stretch".to_string(),
            AlignSelf::Baseline => "baseline".to_string(),
            // AlignSelf::SpaceBetween => "space-between".to_string(),
            // AlignSelf::SpaceAround => "space-around".to_string(),
        }),
        Attribute::JustifyContent(r) => ("justify-content", match r.0 {
            JustifyContent::FlexStart => "flex-start".to_string(),
            JustifyContent::Center => "center".to_string(),
            JustifyContent::FlexEnd => "flex-end".to_string(),
            JustifyContent::SpaceBetween => "space-between".to_string(),
            JustifyContent::SpaceAround => "space-around".to_string(),
            JustifyContent::SpaceEvenly => "space-evenly".to_string(),
        }),

        Attribute::ObjectFit(r) => ("object-fit", match r.0 {
            FitType::None => "none".to_string(),
            FitType::Fill => "fill".to_string(),
            FitType::Contain => "contain".to_string(),
            FitType::Cover => "cover".to_string(),
            FitType::ScaleDown => "scale-down".to_string(),
            // FitType::Repeat => "repeat".to_string(),
            // FitType::RepeatX => "repeat-x".to_string(),
            // FitType::RepeatY => "repeat-y".to_string(),
        }),

        Attribute::BackgroundRepeat(r) => ("background-repeat", {
                match r.x {
                    ImageRepeatOption::Stretch => "stretch ",
                    ImageRepeatOption::Repeat => "repeat ",
                    ImageRepeatOption::Round => "round ",
                    ImageRepeatOption::Space => "space ",
                }.to_string()
                + match r.y {
                    ImageRepeatOption::Stretch => "stretch",
                    ImageRepeatOption::Repeat => "repeat",
                    ImageRepeatOption::Round => "round",
                    ImageRepeatOption::Space => "space",
                }
        }),
        Attribute::TextAlign(r) =>("text-align", match r.0 {
            TextAlign::Left => "left".to_string(),
            TextAlign::Right => "right".to_string(),
            TextAlign::Center => "center".to_string(),
            TextAlign::Justify => "justify".to_string(),
        }),
        Attribute::VerticalAlign(r) => ("vertical-align", match r.0 {
            VerticalAlign::Top => "top".to_string(),
            VerticalAlign::Middle => "middle".to_string(),
            VerticalAlign::Bottom => "bottom".to_string(),
        }),
        Attribute::WhiteSpace(r) => ("white-space", match r.0 {
            WhiteSpace::Normal => "normal".to_string(),
            WhiteSpace::Nowrap => "nowrap".to_string(),
            WhiteSpace::PreWrap => "pre-wrap".to_string(),
            WhiteSpace::Pre => "pre".to_string(),
            WhiteSpace::PreLine => "pre-line".to_string(),
        }),
        Attribute::FontStyle(r) => ("font-style", match r.0 {
            FontStyle::Normal => "normal".to_string(),
            FontStyle::Ttalic => "ttalic".to_string(),
            FontStyle::Oblique => "oblique".to_string(),
        }),
        Attribute::Enable(r) => ("enable", match r.0 {
            pi_style::style::Enable::Auto => "auto".to_string(),
            pi_style::style::Enable::None => "none".to_string(),
            pi_style::style::Enable::Visible => "visible".to_string(),
        }),
        Attribute::Display(r) => ("display", match r.0 {
            Display::Flex => "flex".to_string(),
            Display::None => "none".to_string(),
        }),
        Attribute::Visibility(r) => ("visibility", match r.0 {
            true => "visible".to_string(),
            false => "hidden".to_string(),
        }),
        Attribute::Overflow(r) => ("overflow", match r.0 {
            true => "hidden".to_string(),
            false => "visible".to_string(),
        }),
		// "[-a-zA-Z]*:
        Attribute::LetterSpacing(r) => ("letter-spacing", r.to_string()),
        Attribute::LineHeight(r) => ("line-height",match r.0 {
            LineHeight::Normal => "normal".to_string(),
            LineHeight::Length(r) => r.to_string() + "px",
            LineHeight::Number(r) => r.to_string(),
            LineHeight::Percent(r) => (r * 100.0).to_string() + "%",
        }),
        Attribute::TextIndent(r) => ("text-indent", r.to_string() + "px"),
        Attribute::WordSpacing(r) => ("word-space", r.to_string() + "px"),
        Attribute::FontWeight(r) => ("font-weight", r.to_string()),
        Attribute::FontSize(_r) => ("","".to_string()), // TODO
        Attribute::FontFamily(r) => ("font-family", r.to_string()),
        Attribute::ZIndex(r) => ("z-index", r.to_string()),
        Attribute::Opacity(r) => ("opacity", r.0.to_string()),
        // Attribute::BorderImageRepeat(BorderImageRepeat)(x, y) => "" + r.to_string() + " " +,
        Attribute::BackgroundImage(r) => ("baskground-image-source", r.to_string()),
        Attribute::BorderImage(r) => ("border-image-source", r.to_string()),

        Attribute::FlexShrink(r) => ("flex-shrink", r.to_string()),
        Attribute::FlexGrow(r) => ("flex-grow", r.to_string()),
        Attribute::Width(r) => ("width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::Height(r) => ("height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginLeft(r) => ("margin-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginTop(r) => ("margin-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginBottom(r) => ("margin-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MarginRight(r) => ("margin-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingLeft(r) => ("padding-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingTop(r) => ("padding-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingBottom(r) => ("padding-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PaddingRight(r) => ("padding-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderLeft(r) => ("border-left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderTop(r) => ("border-top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderBottom(r) => ("border-bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BorderRight(r) => ("border-right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        // Attribute::Border(r) => ("visibility",match r.0 {
        //     Dimension::Undefined => "".to_string(),
        //     Dimension::Auto => "auto".to_string(),
        //     Dimension::Points(r) =>  r.to_string() + "px",
        //     Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        // },
        Attribute::MinWidth(r) => ("min-width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MinHeight(r) => ("min-height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MaxHeight(r) => ("max-height",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::MaxWidth(r) => ("max-width",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::FlexBasis(r) => ("flex-basis",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionLeft(r) => ("left",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionTop(r) => ("top",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionRight(r) => ("right",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::PositionBottom(r) => ("bottom",match r.0 {
            Dimension::Undefined => "".to_string(),
            Dimension::Auto => "auto".to_string(),
            Dimension::Points(r) =>  r.to_string() + "px",
            Dimension::Percent(r) =>  (r * 100.0).to_string() + "%",
        }),
        Attribute::BackgroundColor(color) => ("background-color", match &color.0 {
            Color::RGBA(r) => {
                "rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "linear-gradient".to_string(),
        }),

        Attribute::BorderColor(r) => ("border-color",{
            let r = &r.0;
            "rgba(".to_string()
                + r.x.to_string().as_str()
                + ","
                + r.y.to_string().as_str()
                + ","
                + r.z.to_string().as_str()
                + ","
                + r.w.to_string().as_str()
                + ")"
        }),
        Attribute::BoxShadow(r) => ("box-shadow",{
            r.h.to_string()
                + " "
                + r.v.to_string().as_str()
                + " "
                + r.blur.to_string().as_str()
                + " "
                + r.spread.to_string().as_str()
                + " rgba("
                + r.color.x.to_string().as_str()
                + ","
                + r.color.y.to_string().as_str()
                + ","
                + r.color.z.to_string().as_str()
                + ","
                + r.color.w.to_string().as_str()
                + ")"
            // pub h: f32,         // 水平偏移，正右负左
            // pub v: f32,         // 垂直偏移，正下负上
            // pub blur: f32,      // 模糊半径，0代表不模糊，
            // pub spread: f32,    // 阴影扩展，上下左右各加上这个值
            // pub color: CgColor, // 阴影颜色
        }),

        Attribute::BackgroundImageClip(r) => ("image-clip",{
           	(r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),
        Attribute::MaskImageClip(r) => ("mask-image-clip",{
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),

        Attribute::BorderImageClip(r) => ("border-image-clip",{
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
        }),
        Attribute::BorderImageSlice(r) => ("border-image-slice",{
            let mut f = "";
            if r.fill {
                f = " fill";
            }
            (r.top * 100.0).to_string()
                + "% "
                + (r.right * 100.0).to_string().as_str()
                + "% "
                + (r.bottom * 100.0).to_string().as_str()
                + "% "
                + (r.left * 100.0).to_string().as_str()
                + "%"
                + f
        }),

        Attribute::Color(r) => ("color",match &r.0 {
            Color::RGBA(r) => {
                "rgba(".to_string()
                    + r.x.to_string().as_str()
                    + ","
                    + r.y.to_string().as_str()
                    + ","
                    + r.z.to_string().as_str()
                    + ","
                    + r.w.to_string().as_str()
                    + ")"
            }
            Color::LinearGradient(_r) => "linear-gradient".to_string(),
        }),
        Attribute::TextShadow(r) => ("text-shadow",{
            let mut rr = "".to_string();
            for shadow in r.iter() {
                rr = rr
                    + shadow.h.to_string().as_str()
                    + " "
                    + shadow.v.to_string().as_str()
                    + " "
                    + shadow.blur.to_string().as_str()
                    + " rgba("
                    + shadow.color.x.to_string().as_str()
                    + ","
                    + shadow.color.y.to_string().as_str()
                    + ","
                    + shadow.color.z.to_string().as_str()
                    + ","
                    + shadow.color.w.to_string().as_str()
                    + ","
                    + ")";
            }
            rr
        }),
        Attribute::TextStroke(r) => ("text-stroke",{
            "rgba(".to_string()
                + r.0.color.x.to_string().as_str()
                + ","
                + r.0.color.y.to_string().as_str()
                + ","
                + r.0.color.z.to_string().as_str()
                + ","
                + r.0.color.w.to_string().as_str()
                + ")"
        }),

        Attribute::BorderRadius(_r) => ("", "".to_string()),    // TODO
        Attribute::TransformFunc(_r) => ("", "".to_string()),   // TODO
        Attribute::TransformOrigin(_r) => ("", "".to_string()), // TODO
        Attribute::Hsi(_r) => ("", "".to_string()),
        Attribute::BorderImageRepeat(r) => ("border-image-repeat", format!("{:?}", r.x) + " " + format!("{:?}", r.y).as_str()),
        Attribute::Blur(r) => ("blur", r.0.to_string() + "px"),
        Attribute::MaskImage(r) => ("mask-image",format!("{:?}", r.0)),
        Attribute::Transform(_r) => ("", "".to_string()),               // TODO
		Attribute::Translate(_r) => ("", "".to_string()),               // TODO
		Attribute::Scale(_r) => ("", "".to_string()),               // TODO
		Attribute::Rotate(_r) => ("", "".to_string()),               // TODO
        Attribute::TransformWillChange(_r) => ("", "".to_string()),     // TODO
        Attribute::BlendMode(_r) => ("", "".to_string()),               // TODO
        Attribute::Direction(_r) => ("", "".to_string()),               // TODO
        Attribute::AspectRatio(_r) => ("", "".to_string()),             // TODO
        Attribute::Order(_r) => ("", "".to_string()),                   // TODO
        Attribute::TextContent(_r) => ("", "".to_string()),             // TODO
        Attribute::VNode(_r) => ("", "".to_string()),                   // TODO
        Attribute::AnimationName(_r) => ("", "".to_string()),           // TODO
        Attribute::AnimationDuration(_r) => ("", "".to_string()),       // TODO
        Attribute::AnimationTimingFunction(_r) => ("", "".to_string()), // TODO
        Attribute::AnimationDelay(_r) => ("", "".to_string()),          // TODO
        Attribute::AnimationIterationCount(_r) => ("", "".to_string()), // TODO
        Attribute::AnimationDirection(_r) => ("", "".to_string()),      // TODO
        Attribute::AnimationFillMode(_r) => ("", "".to_string()),       // TODO
        Attribute::AnimationPlayState(_r) => ("", "".to_string()),
        Attribute::TextOverflow(r) =>  ("text-overflow", match &r.0 {
            pi_style::style::TextOverflow::None => "none".to_string(),
            pi_style::style::TextOverflow::Clip => "clip".to_string(),
            pi_style::style::TextOverflow::Ellipsis => "ellipsis".to_string(),
            pi_style::style::TextOverflow::Custom(r) => r.clone(),
        }),
        Attribute::OverflowWrap(r) => ("overflow-wrap", match &r.0 {
            pi_flex_layout::style::OverflowWrap::Normal => "normal".to_string(),
            pi_flex_layout::style::OverflowWrap::Anywhere => "anywhere".to_string(),
            pi_flex_layout::style::OverflowWrap::BreakWord => "break-word".to_string(),
        }),      // TODO
    }
}


