
use std::{mem::transmute};

#[cfg(feature="record")]
use pi_bevy_render_plugin::{PlayState, TraceOption};
use pi_ui_render::components::user::ClassName;
#[cfg(feature="record")]
use pi_ui_render::system::base::node::cmd_play::CmdNodeCreate;
pub use crate::export::Engine;
// pub use pi_export_system::blob::Blob;
use pi_null::Null;
use pi_ui_render::system::system_set::UiSchedule;
use pi_ui_render::{
    components::{
        calc::{InPassId, IsShow, LayoutResult, Quad, ZRange, EntityKey},
        pass_2d::ParentPassId,
        user::{Overflow, NodeState},
    },
    prelude::UserCommands,
    resource::{fragment::NodeTag},
};

use pi_world::editor::EntityEditor;
use pi_world::prelude::Entity;
use pi_bevy_ecs_extend::prelude::{Down, Layer, Up};
use pi_world::world::ComponentIndex;
use js_proxy_gen_macro::pi_js_export;

pub use crate::export::Atom as Atom1;
// pub use pi_export_system::blob::Blob;


#[cfg(target_arch = "wasm32")]
use pi_async_rt::prelude::{LocalTaskRunner, LocalTaskRuntime};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct Gui {
    pub(crate) entitys: EntityEditor<'static>, // 需要保证World在内存中不被移动和销毁
    pub(crate) commands: UserCommands,
	#[cfg(feature="record")]
	pub(crate) node_cmd: CmdNodeCreate,
	#[cfg(feature="record")]
	pub(crate) record_option: TraceOption,

    pub(crate) down_component: ComponentIndex,
    pub(crate) up_component: ComponentIndex,
    pub(crate) is_show_component: ComponentIndex,
    pub(crate) layout_component: ComponentIndex,
    pub(crate) quad_component: ComponentIndex,
    pub(crate) zrange_component: ComponentIndex,
    pub(crate) inpass_component: ComponentIndex,
    pub(crate) layer_component: ComponentIndex,
    pub(crate) parentpass_component: ComponentIndex,
    pub(crate) overflow_component: ComponentIndex,
    pub(crate) nodestate_component: ComponentIndex,
    pub(crate) class_name_component: ComponentIndex,

    // pub(crate) query_state: SystemState<(
    //     Res<'static, QuadTree>,
    //     Query<'static, 'static, (&'static Layer, &'static IsShow, &'static ZRange, &'static InPassId)>,
    //     Query<'static, 'static, (&'static ParentPassId, &'static Quad, OrDefault<Overflow>)>,
    // )>,
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
		crate::export::await_last_frame(engine);
		Gui {
            down_component: engine.world.init_component::<Down>(),
            up_component: engine.world.init_component::<Up>(),
            is_show_component: engine.world.init_component::<IsShow>(),
            layout_component: engine.world.init_component::<LayoutResult>(),
            quad_component: engine.world.init_component::<Quad>(),
            zrange_component: engine.world.init_component::<ZRange>(),
            inpass_component: engine.world.init_component::<InPassId>(),
            layer_component: engine.world.init_component::<Layer>(),
            parentpass_component: engine.world.init_component::<ParentPassId>(),
            overflow_component: engine.world.init_component::<Overflow>(),
            nodestate_component: engine.world.init_component::<NodeState>(),
            class_name_component: engine.world.init_component::<ClassName>(),

            entitys: unsafe { transmute(engine.world.make_entity_editor())}, // 需要保证World在内存中不被移动和销毁
			commands: UserCommands::default(),
			#[cfg(feature="record")]
			node_cmd: CmdNodeCreate::default(),
			#[cfg(feature="record")]
			record_option: TraceOption::default(),
			res_await_list: Vec::default(),
		}
	}
	#[cfg(feature="record")]
    pub fn node_cmd(&mut self) -> &mut CmdNodeCreate { &mut self.node_cmd }
	#[cfg(feature="record")]
    pub fn record_option(&mut self) -> &mut TraceOption { &mut self.record_option }

    pub fn down_component(&self) -> ComponentIndex { self.down_component }
    pub fn up_component(&self) -> ComponentIndex { self.up_component }
    pub fn is_show_component(&self) -> ComponentIndex { self.is_show_component }
    pub fn layout_component(&self) -> ComponentIndex { self.layout_component }
    pub fn quad_component(&self) -> ComponentIndex { self.quad_component }
    pub fn zrange_component(&self) -> ComponentIndex { self.zrange_component }
    pub fn inpass_component(&self) -> ComponentIndex { self.inpass_component }
    pub fn layer_component(&self) -> ComponentIndex { self.layer_component }
    pub fn parentpass_component(&self) -> ComponentIndex { self.parentpass_component }
    pub fn overflow_component(&self) -> ComponentIndex { self.overflow_component }
    pub fn nodestate_component(&self) -> ComponentIndex { self.nodestate_component }
    pub fn class_name_component(&self) -> ComponentIndex { self.class_name_component }
	pub fn entitys_mut(&mut self) -> &mut  EntityEditor<'static> {
		&mut self.entitys
	}
	pub fn entitys(&self) -> &  EntityEditor<'static> {
		& self.entitys
	}

	pub fn commands(&self) -> &UserCommands {
		&self.commands
	}

	pub fn commands_mut(&mut self) -> &mut UserCommands {
		&mut self.commands
	}
    pub fn res_await_list_mut(&mut self) -> &mut Vec<pi_atom::Atom> {
        &mut self.res_await_list
    }
    pub fn create_node(gui: &mut Gui) -> f64 {
        let entity = gui.entitys.alloc_entity();

        #[cfg(feature="record")]
        if let TraceOption::Record = gui.record_option {
            gui.node_cmd.0.push(entity);
        }

        gui.commands.init_node(entity, NodeTag::Div);
        // log::warn!("entity :{:?}", entity);
        unsafe { transmute(entity) }
    }

    pub fn create_vnode(gui: &mut Gui) -> f64 {
        let entity = gui.entitys.alloc_entity();

        #[cfg(feature="record")]
        if let TraceOption::Record = gui.record_option {
            gui.node_cmd.0.push(entity);
        }

        gui.commands.init_node(entity, NodeTag::VNode);
        unsafe { transmute(entity) }
    }

    pub fn create_text_node(gui: &mut Gui) -> f64 {
        let entity = gui.entitys.alloc_entity();

        #[cfg(feature="record")]
        if let TraceOption::Record = gui.record_option {
            gui.node_cmd.0.push(entity);
        }

        gui.commands.init_node(entity, NodeTag::Span);
        unsafe { transmute(entity) }
    }

    pub fn create_image_node(gui: &mut Gui) -> f64 {
        let entity = gui.entitys.alloc_entity();

        #[cfg(feature="record")]
        if let TraceOption::Record = gui.record_option {
            gui.node_cmd.0.push(entity);
        }

        gui.commands.init_node(entity, NodeTag::Image);
        unsafe { transmute(entity) }
    }

    pub fn create_canvas_node(gui: &mut Gui) -> f64 {
        let entity = gui.entitys.alloc_entity();

        #[cfg(feature="record")]
        if let TraceOption::Record = gui.record_option {
            gui.node_cmd.0.push(entity);
        }

        gui.commands.init_node(entity, NodeTag::Canvas);
        unsafe { transmute(entity) }
    }

    pub fn destroy_node(gui: &mut Gui, node: f64) {
        let node = unsafe {transmute::<f64, Entity>(node)};
        gui.commands.destroy_node(node);
    }

    pub fn remove_node(gui: &mut Gui, node: f64) {
        let node = unsafe {transmute::<f64, Entity>(node)};
        gui.commands.remove_node(node);
    }

    pub fn insert_as_root(gui: &mut Gui, node_id: f64) {
        let node = unsafe {transmute::<f64, Entity>(node_id)};
        gui.commands.append(node, unsafe { transmute(EntityKey::null())});
    }

    pub fn append_child(gui: &mut Gui, node: f64, parent: f64) {
        let node = unsafe {transmute::<f64, Entity>(node)};
        let parent = unsafe {transmute::<f64, Entity>(parent)};
        gui.commands.append(node, parent);
    }

    pub fn insert_before(gui: &mut Gui, node: f64, borther: f64) {
        let node = unsafe {transmute::<f64, Entity>(node)};
        let borther = unsafe { transmute::<_, Entity>(borther) };
        gui.commands.insert_before(node,borther,);
    }
}

pub fn replay_calc(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature="record")]
	if let TraceOption::Play = gui.record_option {
		loop {
    use pi_bevy_render_plugin::Records;


			let records = engine.world.get_single_res::<Records>().unwrap();
			let play_state = engine.world.get_single_res::<PlayState>().unwrap();
			let cur_frame_count = play_state.cur_frame_count + 1;
			let next_state_index = play_state.next_state_index;
				
			if next_state_index < records.run_state.len() {
				let state = &records.run_state[play_state.next_state_index];
				if state.1 >= cur_frame_count {

                    use pi_bevy_render_plugin::RunState;

                    use crate::gui::{calc_layout, calc_geo};

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
			
			
			let play_state = engine.world.get_single_res_mut::<PlayState>().unwrap();
			play_state.next_state_index += 1;
		}
	}
}


pub fn render_gui(gui: &mut Gui, engine: &mut Engine) {
	replay_calc(gui, engine);
	
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("flush").entered();
	// pi_world::prelude::CommandQueue::default().apply(&mut engine.world); 实体缓冲刷新
	flush_data(gui, engine);
}


pub fn calc(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc").entered();
	// pi_world::prelude::CommandQueue::default().apply(&mut engine.world); //实体缓冲刷新
	flush_data(gui, engine);
	// *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::MATRIX;
	// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.run_schedule(UiSchedule::Calc);
    // *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::NONE;
}

pub fn calc_layout(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc_layout").entered();
	// pi_world::prelude::CommandQueue::default().apply(&mut engine.world); //实体缓冲刷新
	flush_data(gui, engine);
	// *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::LAYOUT;
	// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.run_schedule(UiSchedule::Layout);
    // *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::NONE;
}

pub fn calc_geo(gui: &mut Gui, engine: &mut Engine) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("calc_geo").entered();
	// pi_world::prelude::CommandQueue::default().apply(&mut engine.world); //实体缓冲刷新
	flush_data(gui, engine);
	// *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::MATRIX;
	// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
	engine.run_schedule(UiSchedule::Geo);
    // *engine.world.get_single_res_mut::<RunState>().unwrap() = RunState::NONE;
}

#[inline]
fn flush_data(gui: &mut Gui, engine: &mut Engine) {
	let com = engine.app.world.get_single_res_mut::<pi_ui_render::prelude::UserCommands>().unwrap();
	std::mem::swap(&mut gui.commands, &mut *com);

	if let Some(com) = engine.app.world.get_single_res_mut::<pi_ui_render::system::res_load::ResList>() {
		std::mem::swap(&mut gui.res_await_list, &mut com.await_list);
	};
	
	#[cfg(feature="record")]
	if let TraceOption::Record = engine.app.world.get_single_res_mut::<PlayState>().unwrap().option {
		if let Some(node_cmd) =  engine.app.world.get_single_res_mut::<pi_ui_render::system::base::node::cmd_play::CmdNodeCreate>() {
			std::mem::swap(&mut gui.node_cmd, &mut *node_cmd);
		}
	}
}

