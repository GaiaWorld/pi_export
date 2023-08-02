use pi_ui_render::{prelude::UiPlugin};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_window_renderer::PluginWindowRender;
use pi_bevy_asset::PiAssetPlugin;

use crate::index::insert_as_root;

pub use super::index::Gui;
pub use pi_export_base::export::{Engine, Atom};
use pi_export_play::as_value;
use super::{index::{remove_node, append_child, destroy_node, insert_before, create_node, create_text_node, create_image_node, create_canvas_node, create_vnode}};
use pi_ui_render::components::calc::EntityKey;
use bevy::{app::prelude::App};
use bevy::ecs::{prelude::Entity};
// pub use pi_ui_render::gui::Gui;
use pi_null::Null;
use std::{intrinsics::transmute, sync::Arc};
pub use winit::window::Window;

#[cfg(feature="pi_js_export")]
pub fn create_engine(window: &Arc<Window>, width: u32, height: u32, asset_total_capacity: u32, asset_config: &str) -> Engine {
    use bevy::prelude::{CoreSet, IntoSystemSetConfig};
    use pi_bevy_render_plugin::{should_run, PiRenderOptions};
    use wgpu::Backend;
    use crate::index::parse_asset_config;


    let mut app = App::default();

	let mut window_plugin = bevy::window::WindowPlugin::default();
	window_plugin.primary_window = None;
    // window_plugin.add_primary_window = false;
	// window_plugin.window.width = width as f32;
    // window_plugin.window.height = height as f32;
	// window_plugin.add_primary_window = false;
	if cfg!(target_os = "android"){
		println!("-=============== target_os = android");
		let mut options = PiRenderOptions::default();
		options.0.backends = Backend::Gl.into();
		app.insert_resource(options);
	}
	
    app
		// .add_plugin(bevy::log::LogPlugin {
		// 	filter: "wgpu=debug".to_string(),
		// 	level: bevy::log::Level::DEBUG,
		// })
		.add_plugin(bevy::a11y::AccessibilityPlugin)
		// .add_plugin(bevy::input::InputPlugin::default())
		.add_plugin(window_plugin)
		.add_plugin(pi_bevy_winit_window::WinitPlugin::new(window.clone()).with_size(width, height))
		// .add_plugin(WorldInspectorPlugin::new())
		.add_plugin(PiAssetPlugin {total_capacity: asset_total_capacity as usize, asset_config: parse_asset_config(asset_config)})
		.add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugin(PluginWindowRender)
		.add_plugin(PiPostProcessPlugin);
	app.configure_set(CoreSet::First.run_if(should_run));
    Engine(app)
}

// #[derive(Debug, Clone)]
// #[cfg(feature="pi_js_export")]
// pub enum TraceOption {
// 	None = 0,
// 	Record = 1,
// 	Play = 2,
// }

#[allow(unused_variables)]
#[cfg(feature="pi_js_export")]
pub fn create_gui(
    context: u32,
    engine: &mut Engine,
    width: f32,
    height: f32,
    load_image_fun: u32,
    class_sheet: u32,
    font_sheet: u32,
    cur_time: u32,
    animation_event_fun: u32,
	debug: u32,
) -> Gui {
	println!("width====================!!!!!, {:?}", width);
    let mut gui = Gui::new(engine);

	#[cfg(feature="record")]
	{
		let debug: pi_ui_render::system::cmd_play::TraceOption = unsafe { transmute(debug as u8) };
		engine.add_plugin(UiPlugin {cmd_trace: debug.clone()});
		gui.record_option = debug;
	}

	#[cfg(not(feature="record"))]
    engine.add_plugin(UiPlugin::default());

	// // 设置动画的监听器
    // let a_callback = Share::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (ObjKey, pi_atom::Atom)>| {
    // 	let mut arr: Vec<f64> = Vec::new();
    // 	for (group_id, ty, count) in list.iter() {
    // 		match map.get(*group_id) {
    // 			Some(r) => {
	// 				arr.push(unsafe { transmute::<_, f64>(r.0.to_bits()) }); // entity
	// 				arr.push(r.1.get_hash() as f64); // name hash
    // 			},
    // 			None => continue,
    // 		};
    // 		arr.push(unsafe {transmute::<_, u8>(*ty)}  as f64); // event type
    // 		arr.push(*count as f64); // cur iter count
    // 	}
    // 	animation_event_fun(context, arr, None);
    // });
    // gui.commands.push_cmd(AnimationListenCmd(a_callback));

    gui
}

#[cfg(feature = "pi_js_export")]
pub fn create_fragment(gui: &mut Gui, arr: &mut [f64], count: u32, key: u32) {
    use pi_ui_render::resource::FragmentCommand;

	let mut index: usize = 0;
	let mut entitys = Vec::with_capacity(count as usize);
	while index < count as usize {
		let entity = gui.entitys.reserve_entity();
		arr[index] = unsafe { transmute(entity.to_bits()) };
		entitys.push(entity);
		index = index + 1;
	}
	log::warn!("entitys=============={:?}", entitys);
	gui.commands
		.fragment_commands
		.push(FragmentCommand { key, entitys });
}