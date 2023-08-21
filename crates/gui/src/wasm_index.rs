use serde::{Serialize, Deserialize};
use pi_ui_render::components::user::Overflow;
use pi_ui_render::components::user::{RenderDirty, TextContent as TextContent1};
use pi_ui_render::components::NodeBundle;
use pi_ui_render::components::pass_2d::GraphId;
use pi_ui_render::{
    components::{
        calc::{EntityKey, InPassId, IsShow, Quad, ZRange},
        pass_2d::ParentPassId,
        user::{Aabb2, CgColor, ClearColor, Point2, Viewport},
    },
    prelude::{UiPlugin, UserCommands},
    resource::{ExtendCssCmd, NodeCmd, QuadTree},
    system::node::user_setting::user_setting,
    utils::cmd::SingleCmd,
};
use bevy::ecs::{
    prelude::Entity,
    system::{CommandQueue, Query, SystemState},
    world::WorldCell,
};
use pi_animation::{animation_group::AnimationGroupID, animation_listener::EAnimationEvent};
use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_ecs_extend::prelude::{Layer, OrDefault};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_window_renderer::PluginWindowRender;
use pi_bevy_asset::PiAssetPlugin;
use pi_hash::XHashMap;
use pi_slotmap::SecondaryMap;
use pi_style::{
    style::*,
    style_parse::{
        parse_animation, parse_class_map_from_string, parse_comma_separated, parse_text_shadow, ClassMap, KeyFrameList, StyleParse,
    },
    style_type::*,
};
use smallvec::SmallVec;

pub use super::{index::Gui, ShareChromeWrite};
use pi_export_play::as_value;
// pub use pi_ui_render::gui::Gui;
pub use pi_export_base::export::{Engine, Atom};
use bevy::app::App;
use cssparser::ParseError;
use js_proxy_gen_macro::pi_js_export;
use js_sys::{Array, Function, Float64Array};
use pi_async_rt::rt::serial_local_compatible_wasm_runtime::LocalTaskRunner;
use pi_bevy_winit_window::WinitPlugin;
use pi_hal::runtime::{RENDER_RUNTIME, MULTI_MEDIA_RUNTIME};
use pi_null::Null;
use pi_spatial::quad_helper::intersects;
use std::{
    intrinsics::transmute,
    mem::swap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
	cell::OnceCell,
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::HtmlCanvasElement;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
pub use winit::platform::web::WindowBuilderExtWebSys;
pub use winit::window::{Window, WindowBuilder};
use pi_ui_render::resource::{animation_sheet::KeyFramesSheet, FragmentCommand};

pub static mut RUNNER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();

/// width、height为physical_size
#[wasm_bindgen]
pub fn create_engine(canvas: HtmlCanvasElement, width: u32, height: u32, asset_total_capacity: u32, asset_config: &str, log_filter: Option<String>, log_level: u8) -> Engine {
	use bevy::prelude::IntoSystemSetConfigs;
	use crate::index::parse_asset_config;

	// 初始化运行时（全局localRuntime需要初始化）
	let runner = LocalTaskRunner::new();
    let rt = runner.get_runtime();
    //非线程安全，外部保证同一时间只有一个线程在多读或单写变量
    unsafe {
        RUNNER.set(runner);
        MULTI_MEDIA_RUNTIME.0.set(rt.clone());
		RENDER_RUNTIME.0.set(rt);
    }

	// static mut RUNNER_MULTI: OnceCell<LocalTaskRunner<()>> = OnceCell::new();
	// static mut RUNNER_RENDER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();


    let mut app = App::default();

    let mut window_plugin = bevy::window::WindowPlugin::default();
	window_plugin.primary_window = None;

	let mut log = pi_bevy_log::LogPlugin::default();
	web_sys::console::log_1(&wasm_bindgen::JsValue::from(&format!("log_filter========={:?}", log_filter)));
	if let Some(log_filter) = log_filter {
		log.filter = log_filter;
	}

	
// static mut MULTI_MEDIA_RUNTIME: OnceCell<LocalTaskRuntime<()>> = OnceCell::new();

// static mut RUNNER_RENDER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();
// static mut RENDER_RUNTIME: OnceCell<LocalTaskRuntime<()>> = OnceCell::new();
	

	// log.filter="pi_ui_render::resource::animation_sheet=debug".to_string();
	// log.filter="pi_ui_render::system::node::user_setting=debug,pi_ui_render::system::components::user=debug".to_string();
	// log.filter="bevy=debug".to_string();
	// log.filter="wgpu=debug".to_string();
	// bevy::log::Level::INFO
	// Trace = 0,
    // Debug = 1,
    // Info = 2,
    // Warn = 3,
    // Error = 4,
	log.level= match log_level{
		0 => bevy::log::Level::TRACE,
		1 => bevy::log::Level::DEBUG,
		2 => bevy::log::Level::INFO,
		3 => bevy::log::Level::WARN,
		4 => bevy::log::Level::ERROR,
		_ => bevy::log::Level::WARN,
	};
	let chrome_write = ShareChromeWrite::new();
	log.chrome_write = Some(chrome_write.clone());
    app
		.insert_resource(chrome_write)
        .add_plugin(log)
		.add_plugin(bevy::a11y::AccessibilityPlugin)
        .add_plugin(window_plugin)
		.add_plugin(PiAssetPlugin {total_capacity: asset_total_capacity as usize, asset_config: parse_asset_config(asset_config)})
        .add_plugin(pi_bevy_winit_window::WinitPlugin::new(canvas).with_size(width, height))
        .add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugin(PluginWindowRender)
        .add_plugin(PiPostProcessPlugin)
		.add_plugin(RuntimePlugin); // 推动运行时
	// app.configure_set(CoreSet::First.run_if(should_run));
    Engine::new(app)
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum TraceOption {
	None,
	Record,
	Play,
}

#[wasm_bindgen]
pub fn create_gui(
    context: JsValue,
    engine: &mut Engine,
    width: f32,
    height: f32,
    load_image_fun: Option<Function>,
    class_sheet: u32,
    font_sheet: u32,
    cur_time: u32,
    animation_event_fun: Function,
	debug: TraceOption,
) -> Gui {
    let mut gui = Gui::new(engine);

    #[cfg(feature="record")]
	{
		let debug: pi_ui_render::system::cmd_play::TraceOption = unsafe { transmute(debug) };
		engine.add_plugin(UiPlugin {cmd_trace: debug.clone()});
		gui.record_option = debug;
	}

	#[cfg(not(feature="record"))]
    engine.add_plugin(UiPlugin::default());

    gui
}

#[wasm_bindgen]
pub fn create_fragment(gui: &mut Gui, mut arr: Float64Array, count: u32, key: u32) {
	let mut index = 0;
	let mut entitys = Vec::with_capacity(count as usize);
	while index < count {
		let entity = gui.entitys.reserve_entity();
		#[cfg(feature="record")]
		gui.node_cmd.0.push(entity);

		arr.set_index(index, unsafe { transmute(entity.to_bits()) });
		entitys.push(entity);
		index = index + 1;
	}
	gui.commands
		.fragment_commands
		.push(FragmentCommand { key, entitys });
}


#[wasm_bindgen]
pub fn log_animation(
    engine: &Engine,
) {
	let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
	key_frames.log();
}

// 开始录制性能
#[wasm_bindgen]
pub fn start_record_performance(
    engine: &mut Engine,
) {
	let mut flush_guard = engine.world.get_non_send_resource::<tracing_chrome_wasm::FlushGuard<crate::ShareChromeWrite>>().unwrap();
	flush_guard.start();
}

// 结束录制性能
#[wasm_bindgen]
pub fn end_record_performance(
    engine: &mut Engine,
) -> Vec<u8> {
	let mut flush_guard = engine.world.get_non_send_resource::<tracing_chrome_wasm::FlushGuard<crate::ShareChromeWrite>>().unwrap();
	flush_guard.end();

	let mut chrome_write = engine.world.get_resource_mut::<ShareChromeWrite>().unwrap();
	chrome_write.take()
}

// /**
//  * 获取canvas资源
//  */
// #[wasm_bindgen]
// pub fn get_canvas_source(
//     gui: &mut Gui,
//     soruce: u32, // 是否缓存
// ) -> i32 {
//     -1
// }

// /**
//  * canvas宽高改变时调用(分配纹理成功，返回对应索引，否则返回-1)
//  * @return __jsObj 纹理
// */
// #[wasm_bindgen]
// pub fn set_canvas_size(
//     gui: &mut Gui,
//     node: f64,
//     width: u32,
//     height: u32,
//     soruce: u32, // 是否缓存
//     need_depth: bool, // 是否需要深度缓冲区
//                  // avail_width: u32,
//                  // avail_height: u32,
// ) -> i32 {
//     1
// }

// #[wasm_bindgen]
// pub fn get_canvas_target(gui: &mut Gui, index: usize) -> Option<usize> { Some(1) }

// #[wasm_bindgen]
// pub fn get_canvas_rect(gui: &mut Gui, index: usize) -> JsValue { JsValue::from_serde(&CanvasRect(0, 0, 0, 0)).unwrap() }

// /**
//  * canvas内容发生改变时，应该调用此方法更新gui渲染
// */
// #[wasm_bindgen]
// pub fn update_canvas(gui: &mut Gui, _node: u32) {}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Rect {
//     pub left: f32,
//     pub top: f32,
//     pub width: f32,
//     pub height: f32,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Size {
//     pub width: f32,
//     pub height: f32,
// }

// #[derive(Serialize)]
// pub struct CanvasRect(u32, u32, u32, u32);

#[inline]
fn run_all(rt: &LocalTaskRunner<()>) {
	while RENDER_RUNTIME.len() > 0 {
		rt.poll();
		rt.run_once();
	}
    // while let Ok(r) = rt.run() {
    //     if r == 0 {
    //         break;
    //     }
    // }
}

// wasm 使用单线程运行时，需要手动推
pub struct RuntimePlugin;

fn runtime_run() {
	run_all(unsafe{RUNNER.get().unwrap()});
	// run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
}

impl bevy::app::Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
		use bevy::prelude::{First, IntoSystemConfigs};
		use pi_bevy_render_plugin::should_run;
        app.add_systems(First,
			runtime_run.run_if(should_run)
		);
    }
}

// // wasm 使用单线程运行时，需要手动推
// pub struct RuntimePlugin;

// impl bevy::app::Plugin for RuntimePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_stage_before(
// 			RuntimeStage::Start,
// 			bevy::prelude::SystemStage::single(|| {
// 				run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
// 				run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
// 			}),
// 		);

// 		let last_stage = app.schedule.iter_stages();
// 		let mut last = None;
// 		for i in last_stage {
// 			last = Some(i.0);
// 		}
// 		let last = last.unwrap();
// 		app.add_stage_after(
// 			last,
// 			RuntimeStage::End,
// 			bevy::prelude::SystemStage::single(|| {
// 				run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
// 				run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
// 			}),
// 		);
//     }
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone, bevy::ecs::schedule::StageLabel)]
// pub enum RuntimeStage {
// 	Start,
// 	End
// }
