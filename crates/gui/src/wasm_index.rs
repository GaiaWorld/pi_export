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
use bevy_ecs::{
    prelude::Entity,
    system::{CommandQueue, Query, SystemState},
    world::WorldCell,
};
use pi_animation::{animation_group::AnimationGroupID, animation_listener::EAnimationEvent};
use pi_async_rt::prelude::AsyncRuntime;
use pi_bevy_ecs_extend::prelude::{Layer, OrDefault};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
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
use pi_slotmap::DefaultKey;

pub use super::{index::Gui, ShareChromeWrite};
use pi_export_play::as_value;
// pub use pi_ui_render::gui::Gui;
pub use pi_export_base::export::{Engine, Atom};
use bevy_app::prelude::App;
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
use pi_render::font::FontType;

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
    load_sdf_fun: Option<Function>,
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
		engine.add_plugin(UiPlugin {cmd_trace: debug.clone(), font_type: FontType::Sdf2});
		gui.record_option = debug;
	}

	#[cfg(not(feature="record"))]
    engine.add_plugin(UiPlugin {font_type: FontType::Sdf2});

	// if let Some(fun) = load_sdf_fun {
	// 	pi_hal::font::sdf_brush::init_load_cb(std::rc::Rc::new(move|key: DefaultKey, font_family: usize, chars: &[char]| {
	// 		let chars1 = js_sys::Uint32Array::from(unsafe {transmute::<_, &[u32]>(chars)});
	// 		fun.call3(&JsValue::from(0), &unsafe {transmute::<_, f64>(key)}.into(), &(font_family as u32).into(), chars1.as_ref()); 
	// 	}));
	// }

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

// // 开始录制性能
// #[wasm_bindgen]
// pub fn start_record_performance(
//     engine: &mut Engine,
// ) {
// 	let mut flush_guard = engine.world.get_non_send_resource::<tracing_chrome_wasm::FlushGuard<crate::ShareChromeWrite>>().unwrap();
// 	flush_guard.start();
// }

// // 结束录制性能
// #[wasm_bindgen]
// pub fn end_record_performance(
//     engine: &mut Engine,
// ) -> Vec<u8> {
// 	let mut flush_guard = engine.world.get_non_send_resource::<tracing_chrome_wasm::FlushGuard<crate::ShareChromeWrite>>().unwrap();
// 	flush_guard.end();

// 	let mut chrome_write = engine.world.get_resource_mut::<ShareChromeWrite>().unwrap();
// 	chrome_write.take()
// }

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





// // wasm 使用单线程运行时，需要手动推
// pub struct RuntimePlugin;

// impl bevy_app::Plugin for RuntimePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_stage_before(
// 			RuntimeStage::Start,
// 			bevy_ecs::prelude::SystemStage::single(|| {
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
// 			bevy_ecs::prelude::SystemStage::single(|| {
// 				run_all(&pi_hal::runtime::RUNNER_MULTI.lock());
// 				run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
// 			}),
// 		);
//     }
// }

// #[derive(Debug, Hash, PartialEq, Eq, Clone, bevy_ecs::schedule::StageLabel)]
// pub enum RuntimeStage {
// 	Start,
// 	End
// }
