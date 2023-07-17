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
use pi_async::prelude::AsyncRuntime;
use pi_bevy_ecs_extend::prelude::{Layer, OrDefault};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_window_renderer::PluginWindowRender;
use pi_bevy_asset::PiAssetPlugin;
use pi_hash::XHashMap;
use pi_idtree::InsertType;
use pi_slotmap::SecondaryMap;
use pi_style::{
    style::*,
    style_parse::{
        parse_animation, parse_class_map_from_string, parse_comma_separated, parse_text_shadow, ClassMap, KeyFrameList, StyleParse,
    },
    style_type::*,
};
use smallvec::SmallVec;

pub use super::Gui;
use super::{style::PlayContext};
use pi_export_play::as_value;
// pub use pi_ui_render::gui::Gui;
pub use pi_export_base::export::{Engine, Atom};
use bevy::app::App;
use cssparser::ParseError;
use js_proxy_gen_macro::pi_js_export;
use js_sys::{Array, Function, Float64Array};
use pi_async::prelude::SingleTaskRunner;
use pi_bevy_winit_window::WinitPlugin;
use pi_hal::runtime::{RENDER_RUNTIME, RUNNER_MULTI, RUNNER_RENDER};
use pi_null::Null;
use pi_spatial::quad_helper::intersects;
use std::{
    intrinsics::transmute,
    mem::swap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use web_sys::HtmlCanvasElement;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
pub use winit::platform::web::WindowBuilderExtWebSys;
pub use winit::window::{Window, WindowBuilder};
use pi_ui_render::resource::{animation_sheet::KeyFramesSheet, FragmentCommand};



/// width、height为physical_size
#[wasm_bindgen]
pub fn create_engine(canvas: HtmlCanvasElement, width: u32, height: u32, asset_total_capacity: u32, asset_config: &str, log_filter: Option<String>) -> Engine {
	use bevy::prelude::{CoreSet, IntoSystemSetConfig};
	use pi_bevy_render_plugin::should_run;
	use crate::parse_asset_config;

    let mut app = App::default();

    let mut window_plugin = bevy::window::WindowPlugin::default();
	window_plugin.primary_window = None;

	let mut log = bevy::log::LogPlugin::default();
	web_sys::console::log_1(&wasm_bindgen::JsValue::from(&format!("log_filter========={:?}", log_filter)));
	if let Some(log_filter) = log_filter {
		log.filter = log_filter;
	}
	

	// log.filter="pi_ui_render::resource::animation_sheet=debug".to_string();
	// log.filter="pi_ui_render::system::node::user_setting=debug,pi_ui_render::system::components::user=debug".to_string();
	// log.filter="bevy=debug".to_string();
	// log.filter="wgpu=debug".to_string();
	log.level=bevy::log::Level::WARN;
    app
        .add_plugin(log)
		.add_plugin(bevy::a11y::AccessibilityPlugin)
        .add_plugin(window_plugin)
		.add_plugin(PiAssetPlugin {total_capacity: asset_total_capacity as usize, asset_config: parse_asset_config(asset_config)})
        .add_plugin(pi_bevy_winit_window::WinitPlugin::new(canvas).with_size(width, height))
        .add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugin(PluginWindowRender)
        .add_plugin(PiPostProcessPlugin)
		.add_plugin(RuntimePlugin); // 推动运行时
	app.configure_set(CoreSet::First.run_if(should_run));
    Engine::new(app)
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
) -> Gui {
    let gui = Gui {
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
    };

    engine.add_plugin(UiPlugin);

    // /// 设置动画的监听器
    // let a_callback = Box::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (Entity, pi_atom::Atom)>| {
    // 	let mut arr = Array::new();
    // 	for (group_id, ty, count) in list.iter() {
    // 		match map.get(*group_id) {
    // 			Some(r) => {
    // 				arr.push(&JsValue::from_f64(unsafe { transmute(r.0) }));
    // 				arr.push(&JsValue::from_f64(r.1.get_hash() as f64));
    // 			},
    // 			None => continue,
    // 		};
    // 		arr.push(&JsValue::from_f64(unsafe {transmute::<_, u8>(*ty)}  as f64));
    // 		arr.push(&JsValue::from_f64(*count as f64));
    // 	}
    // 	animation_event_fun.call1(&context, &JsValue::from(arr))
    //                     .expect("call animation event fail!!!");
    // });
    // gui.commands.set_event_listener(a_callback);

    gui
}

#[wasm_bindgen]
pub fn create_fragment(gui: &mut Gui, mut arr: Float64Array, count: u32, key: u32) {
	let mut index = 0;
	let mut entitys = Vec::with_capacity(count as usize);
	while index < count {
		let entity = gui.entitys.reserve_entity();
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
fn run_all(rt: &SingleTaskRunner<()>) {
    while let Ok(r) = rt.run() {
        if r == 0 {
            break;
        }
    }
}

// wasm 使用单线程运行时，需要手动推
pub struct RuntimePlugin;

fn runtime_run() {
	run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
	run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
}

impl bevy::app::Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
		use bevy::prelude::IntoSystemConfig;
        app.add_system(
			runtime_run.in_base_set(bevy::prelude::CoreSet::First)
		);
    }
}

// // wasm 使用单线程运行时，需要手动推
// pub struct RuntimePlugin;

// impl bevy::app::Plugin for RuntimePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_stage_before(
// 			bevy::prelude::CoreStage::First,
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
