

use std::{cell::RefCell, mem::transmute, sync::{Arc, OnceLock}};

use pi_share::{Share, ShareCell};
use pi_world::prelude::{App, WorldPluginExtent};
use derive_deref_rs::Deref;
use pi_bevy_asset::{PiAssetPlugin, AssetConfig, AssetDesc};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_hash::XHashMap;
use pi_render::{rhi::{asset::{RenderRes, TextureRes}, bind_group::BindGroup, pipeline::RenderPipeline}, renderer::sampler::SamplerRes};
use pi_bevy_render_plugin::{FrameState, PiRenderPlugin};
use pi_window_renderer::PluginWindowRender;
pub use pi_export_assets_mgr::exports::ResAllocator;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Function;
#[cfg(target_arch = "wasm32")]
use core::cell::OnceCell;
#[cfg(target_arch = "wasm32")]
use pi_async_rt::rt::serial_local_compatible_wasm_runtime::{LocalTaskRunner, LocalTaskRuntime};
#[cfg(target_arch = "wasm32")]
use pi_async_rt::prelude::AsyncRuntime;
use wgpu::{TextureView, Buffer};
#[cfg(not(target_arch = "wasm32"))]
pub use pi_winit::window::Window;

// pub struct FrameEndOnceLockWrap<F: FnMut() + Send + Sync>(pub OnceLock<Box<dyn FnMut() + Send + Sync>>);

static mut FRAME_END_CB: OnceLock<Box<dyn FnMut() + Send + Sync + 'static>> = OnceLock::new();
/// 初始化帧结束的回调，只能设置一次
pub fn init_frame_end_cb<F: FnMut() + Send + Sync + 'static>(f: F) {
	if let Err(_e) = unsafe { FRAME_END_CB.set(Box::new(f)) } {
		println!("frame end callback init failed");
	}
}

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
#[derive(Deref)]
pub struct Engine {
	#[deref]
	pub app: App,
	/// 上帧等待
	// pub last_frame_awaiting: Share<std::sync::atomic::AtomicBool>,
	pub last_frame_awaiting: bool,
	pub sender: crossbeam_channel::Sender<Box<dyn FnOnce() -> () + Send>>,

	// 回应的sender和receiver
	pub back_receiver: crossbeam_channel::Receiver<()>
}

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
impl Engine {
	pub fn new(app: App) -> Self { 
		let (sender, receiver) = crossbeam_channel::bounded(1);
		let (back_sender, back_receiver) = crossbeam_channel::bounded(1);
		log::warn!("create_engine=================================");
		// let last_frame_awaiting = Share::new(std::sync::atomic::AtomicBool::new(false));
		let _ = std::thread::Builder::new().name("ecs".to_string()).spawn(move || {
			loop {
				let task: Box<dyn FnOnce() -> () + Send> = receiver.recv().unwrap();
				task();
				let _ = back_sender.send(());
				if let Some(cb) = unsafe { FRAME_END_CB.get_mut() } {
					cb();
				}
			}
		});
		Self{
			app,
			last_frame_awaiting: false,
			sender,
			back_receiver,
		}
	}
	pub fn app(&self) -> &App { &self.app }
	pub fn app_mut(&mut self) -> &mut App { &mut self.app }
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
#[derive(Deref)]
pub struct Engine {
	#[deref]
	pub(crate) app: App,
}

#[cfg(target_arch="wasm32")]
impl Engine {
	pub fn new(app: App) -> Self { Self{
		app,
	} }
	pub fn app(&self) -> &App { &self.app }
	pub fn app_mut(&mut self) -> &mut App { &mut self.app }
}

#[derive(Debug, Clone, Deref)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct Atom(pi_atom::Atom);
impl Atom {
    pub fn new(value: pi_atom::Atom) -> Self { Self(value) }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl Atom {
	#[cfg(feature = "pi_js_export")]
	pub fn from_string(value: String) -> Self { Atom::new(pi_atom::Atom::from(value)) }

	#[cfg(feature = "pi_js_export")]
	pub fn get_string_by_hash(value: u32) -> Option<String> { 
		match pi_atom::get_by_hash(value as usize) {
			Some(r) => Some(r.as_ref().to_string()),
			None => None,
		} 
	}

	#[cfg(feature = "pi_js_export")]
	pub fn get_hash(&self) -> u32 { self.0.str_hash() as u32 }
}

/// 设置日志过滤器
/// 如果过滤器格式错误， 日志过滤器未初始化， 则设置将失败， 但不会panic
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn set_log_filter(engine: &mut Engine, filter: &str) {
	if let Some(handle) = engine.app_mut().world.get_single_res_mut::<pi_bevy_log::LogFilterHandle>() {
		if let Ok(filter_layer) = tracing_subscriber::EnvFilter::try_new(filter) {
			let _ = handle.0.modify(|filter| *filter = filter_layer);
		}
	}
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn set_is_not_run(app: &mut Engine, value: bool) {
	#[cfg(feature = "allow_not_run")]
	{
		let mut is_not_run = app.app_mut().world.get_single_res_mut::<pi_bevy_ecs_extend::IsNotRun>().unwrap();
		is_not_run.0 = value;
	}
}

#[cfg(target_arch = "wasm32")]
pub static mut RUNNER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();

/// width、height为physical_size
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn create_engine(canvas: web_sys::HtmlCanvasElement, width: u32, height: u32, asset_mgr: &pi_export_assets_mgr::ResAllocator, asset_total_capacity: u32, asset_config: &str, log_filter: Option<String>, log_level: u8) -> Engine {
	// 初始化运行时（全局localRuntime需要初始化）
	let runner = LocalTaskRunner::new();
    let rt = runner.get_runtime();
    //非线程安全，外部保证同一时间只有一个线程在多读或单写变量
    unsafe {
        RUNNER.set(runner);
        pi_hal::runtime::MULTI_MEDIA_RUNTIME.0.set(rt.clone());
		pi_hal::runtime::RENDER_RUNTIME.0.set(rt);
    }

	// static mut RUNNER_MULTI: OnceCell<LocalTaskRunner<()>> = OnceCell::new();
	// static mut RUNNER_RENDER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();

	let mut log = pi_bevy_log::LogPlugin::<Vec<u8>>::default();
	if let Some(log_filter) = log_filter {
		log.filter = log_filter;
	}

	log.level= match log_level {
		0 => tracing::Level::TRACE,
		1 => tracing::Level::DEBUG,
		2 => tracing::Level::INFO,
		3 => tracing::Level::WARN,
		4 => tracing::Level::ERROR,
		_ => tracing::Level::WARN,
	};

	console_log::init_with_level(log::Level::Error);

    let mut app = App::new();

    // let mut window_plugin = bevy_window::WindowPlugin::default();
	// window_plugin.primary_window = None;
	
	// let chrome_write = ShareChromeWrite::new();
	// log.chrome_write = None;
	let window ={
		use pi_winit::platform::web::WindowBuilderExtWebSys;
		use wasm_bindgen::JsCast;
		let event_loop = pi_winit::event_loop::EventLoop::new();
		Arc::new(
			pi_winit::window::WindowBuilder::new()
				.with_canvas(Some(canvas))
				.build(&event_loop)
				.unwrap(),
		)
	};
	

	// app.add_plugins(log);
	create_engine_inner(
		&mut app, 
		pi_bevy_winit_window::WinitPlugin::new(window).with_size(width, height),
		asset_total_capacity,
		asset_config,
		Some(asset_mgr.get_inner().clone()),
	);
    app.add_plugins(RuntimePlugin); // wasm需要主动推运行时

    let engine = Engine::new(app);

	engine
}

#[cfg(feature="pi_js_export")]
#[cfg(not(target_arch = "wasm32"))]
pub fn create_engine(window: &Arc<Window>, width: u32, height: u32,  asset_total_capacity: u32, asset_config: &str) -> Engine {
    use pi_bevy_render_plugin::PiRenderOptions;
    use wgpu::Backend;


    let mut app = App::new();
    // window_plugin.add_primary_window = false;
	// window_plugin.window.width = width as f32;
    // window_plugin.window.height = height as f32;
	// window_plugin.add_primary_window = false;
	// if cfg!(target_os = "android"){
	// 	println!("-=============== target_os = android");
		let mut options = PiRenderOptions::default();
		options.0.backends = Backend::Gl.into();
		options.0.present_mode = wgpu::PresentMode::Fifo;
		// options.0.backends = Backend::Vulkan.into();
		app.world.insert_single_res(options);
	// }
	
	create_engine_inner(
		&mut app, 
		pi_bevy_winit_window::WinitPlugin::new(window.clone()).with_size(width, height),
		asset_total_capacity,
		asset_config,
		// Some(asset_mgr.get_inner().clone()),
	);

    let engine = Engine::new(app);

	engine
}

pub fn create_engine_inner(
	app: &mut App, 
	winit_plugin: pi_bevy_winit_window::WinitPlugin,
	asset_total_capacity: u32,
	asset_config: &str,
	// asset_allotor: Option<Share<ShareCell<pi_assets::allocator::Allocator>>>,
) {
	// let mut window_plugin = bevy_window::WindowPlugin::default();
	// window_plugin.primary_window = None;

	app
		// .add_plugins(bevy::log::LogPlugin {
		// 	filter: "wgpu=debug".to_string(),
		// 	level: bevy::log::Level::DEBUG,
		// })
		// .add_plugins(bevy_a11y::AccessibilityPlugin)
		// .add_plugins(bevy::input::InputPlugin::default())
		// .add_plugins(window_plugin)
		.add_plugins(winit_plugin)
		// .add_plugins(WorldInspectorPlugin::new())
		.add_plugins(PiAssetPlugin {total_capacity: asset_total_capacity as usize, asset_config: parse_asset_config(asset_config), /* allocator: asset_allotor */})
		.add_plugins(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugins(PluginWindowRender)
		.add_plugins(PiPostProcessPlugin);
}

// 在wasm目标上,返回渲染图的topo图
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn dump_graphviz(engine: &Engine) -> String  {
	let g = engine.world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
	g.dump_graphviz()
}

// 在wasm目标上,返回system依赖图
// #[cfg(feature="system_graph")]
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// pub fn dump_system(engine: &mut Engine) -> String  {
// 	let label = bevy::prelude::Update;
// 	engine.0.world
// 	.resource_scope::<bevy::prelude::Schedules, _>(|world, mut schedules| {
// 		let schedule = schedules
// 			.get_mut(&bevy::prelude::Update)
// 			.ok_or_else(|| format!("schedule with label {label:?} doesn't exist"))
// 			.unwrap();

// 		bevy_mod_debugdump::schedule_graph::schedule_graph_dot(schedule, world, &Default::default())
// 	})
// 	// bevy_mod_debugdump::schedule_graph_dot(&mut engine.0, bevy::prelude::Update, &Default::default())
// }


// 帧推
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn fram_call(engine: &mut Engine, _cur_time: u32) {
	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("frame_call").entered();
	// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::Active;

	// log::warn!("fram_call start=====");
	await_last_frame(engine);
	// log::warn!("fram_call start1=====");
	#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
	{
		engine.last_frame_awaiting = true;
		let engine: &'static mut Engine = unsafe { transmute(engine) };
		let sender = engine.sender.clone();
		let _ = sender.send(Box::new(|| {
			// bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
			engine.run();
			// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
			// log::warn!("fram_call end=====");
		}));
	}
	
	#[cfg(target_arch="wasm32")]
	{
		// bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
		engine.run();
		// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
	}
}

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
// 等待上次帧运行结束
pub fn await_last_frame(engine: &mut Engine) {
	if engine.last_frame_awaiting {
		engine.back_receiver.recv().unwrap();
		engine.last_frame_awaiting = false;
	}
}

// 等待上次帧运行结束
#[cfg(all(target_arch="wasm32"))]
#[inline]
pub fn await_last_frame(engine: &mut Engine) {
}


pub fn parse_asset_config(asset_config: &str) -> AssetConfig {
	let map: XHashMap<String, AssetDesc> = match serde_json::from_str(asset_config) {
		Ok(r) => r,
		_ => {
			log::warn!("asset_config is invalid,  {:?}", asset_config);
			XHashMap::default()
		}
	};
	let mut asset_config = AssetConfig::default();
	for (key, desc) in map.into_iter() {
		match key.as_str() {
			"TEXTURE_VIEW" => asset_config.insert::<RenderRes<TextureView>>(desc),
			"BUFFER" => asset_config.insert::<RenderRes<Buffer>>(desc),
			"SAMPLER" => asset_config.insert::<SamplerRes>(desc),
			"BIND_GROUP" => asset_config.insert::<RenderRes<BindGroup>>(desc),
			"TEXTURE_RES" => asset_config.insert::<TextureRes>(desc),
			"RENDER_PIPELINE" => asset_config.insert::<RenderRes<RenderPipeline>>(desc),
			_ => {},
		}
	}
	asset_config
}

#[cfg(target_arch = "wasm32")]
#[inline]
fn run_all(rt: &LocalTaskRunner<()>) {
	while pi_hal::runtime::RENDER_RUNTIME.len() > 0 {
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
#[cfg(target_arch = "wasm32")]
pub struct RuntimePlugin;

#[cfg(target_arch = "wasm32")]
fn runtime_run() {
	run_all(unsafe{RUNNER.get().unwrap()});
	// run_all(&pi_hal::runtime::RUNNER_RENDER.lock());
}

#[cfg(target_arch = "wasm32")]
impl pi_world::prelude::Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
		use pi_world::prelude::First;
		use pi_world::prelude::IntoSystemConfigs;
        app.add_system(First,runtime_run);
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn entity_from_number(index: u32, version: u32) -> f64 {
	unsafe { transmute::<_, f64>( (version as u64) << 32 | index as u64) }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn init_engine_3d(app: &mut Engine, particlesystem: bool, skeleton: bool, shadowmapping: bool, lighting: bool, spine: bool) {
	use pi_scene_shell::prelude::WorldResourceTemp;
	use pi_scene_shell::prelude::AppResourceTemp;
	use pi_scene_shell::run_stage::EngineCustomPlugins;
use pi_world::prelude::IntoSystemConfigs;

    if app.world.get_resource::<pi_scene_shell::prelude::AssetMgrConfigs>().is_none() {
        app.insert_resource(pi_scene_shell::prelude::AssetMgrConfigs::default());
    }

	let mut engineplugins = EngineCustomPlugins::default();
	engineplugins.particle_system = particlesystem;
	engineplugins.skeleton = skeleton;
	engineplugins.shadowmapping = shadowmapping;
	engineplugins.lighting = lighting;
	app.insert_resource(engineplugins);

    pi_3d::PluginBundleDefault::add(app);
    app
        .add_plugins(pi_node_materials::PluginNodeMaterialSimple)
        .add_plugins(pi_scene_context::shadow::PluginShadowGenerator)
        .add_plugins(pi_node_materials::prelude::PluginShadowMapping)
        .add_plugins(pi_mesh_builder::cube::PluginCubeBuilder)
        .add_plugins(pi_mesh_builder::quad::PluginQuadBuilder)
        .add_plugins(pi_particle_system::PluginParticleSystem)
        .add_plugins(pi_gltf2_load::PluginGLTF2Res)
        .add_plugins(pi_trail_renderer::PluginTrail)
        ;

    app.add_systems(
        pi_world::schedule::Update,
        pi_scene_context::prelude::sys_state_transform.in_set(pi_scene_shell::prelude::ERunStageChap::StateCheck)
    );
	
	if spine {
		app
			.add_plugins(pi_spine_rs::PluginSpineRenderer);
	}
}