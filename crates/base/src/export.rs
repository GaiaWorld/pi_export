

use std::{mem::transmute, sync::{atomic::{AtomicBool}, Arc, OnceLock}, thread, time::Duration};

use pi_scene_context::pass::ImageTextureFrame;
use pi_share::{Share, ShareCell};
// use pi_ui_render::devtools::PluginSpectorUI;
// use  pi_bevy_render_plugin::spector::PluginSpector;
use pi_world::prelude::{App, WorldPluginExtent};
use derive_deref_rs::Deref;
use pi_bevy_asset::{PiAssetPlugin, AssetConfig, AssetDesc};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_hash::XHashMap;
use pi_render::{asset::TAssetKeyU64, renderer::sampler::SamplerRes, rhi::{asset::{RenderRes}, bind_group::BindGroup, pipeline::RenderPipeline}};
use pi_bevy_render_plugin::{FrameState, GlobalCmdTracePlugin, PiRenderPlugin};
use pi_window_renderer::PluginWindowRender;
use pi_bevy_render_plugin::PiRenderDevice;
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

use pi_bevy_render_plugin::PiRenderOptions;
// pub struct FrameEndOnceLockWrap<F: FnMut() + Send + Sync>(pub OnceLock<Box<dyn FnMut() + Send + Sync>>);

static mut FRAME_END_CB: OnceLock<Box<dyn FnMut(f64) + Send + Sync + 'static>> = OnceLock::new();
static mut FRAME_TIME: u32 = 16;

#[cfg(feature = "pi_js_export")]
pub fn set_fps(fps: u32){
	println!("==========set fps: {}", fps);
	// unsafe { FRAME_TIME = 1000 / fps };
}

/// 初始化帧结束的回调，只能设置一次
pub fn init_frame_end_cb<F: FnMut(f64) + Send + Sync + 'static>(f: F) {
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
	pub sender: crossbeam_channel::Sender<(Box<dyn FnOnce() -> () + Send>, f64)>,

	// 回应的sender和receiver
	pub back_receiver: crossbeam_channel::Receiver<()>,
	// 调试用
	pub id: f64,
}

#[cfg(target_os = "android")]
fn panic_with_backtrace_rs() {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        println!("====== arg = {}", arg);
    }
    
    std::panic::set_hook(Box::new(|panic_info| {
        print!(
            "thread '{}' panicked",
            std::thread::current().name().unwrap_or("unknown")
        );

        if let Some(location) = panic_info.location() {
            println!(" at {}:{}:", location.file(), location.line(),);
        } else {
            println!("");
        }

        // if let Some(msg) = panic_info.message(){
        //     println!("{:?}", msg);
        // }

        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            println!("{}", payload);
        }

        println!("{:?}", backtrace::Backtrace::new());
    }));
}

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
impl Engine {
    pub fn new(mut app: App) -> Self {
		let (sender, receiver) = crossbeam_channel::bounded(1);
		let (back_sender, back_receiver) = crossbeam_channel::bounded(1);
		// app.world.insert_single_res(FrameSender(back_sender));
		log::warn!("create_engine=================================");
		// let last_frame_awaiting = Share::new(std::sync::atomic::AtomicBool::new(false));
        let _ = std::thread::Builder::new()
            .name("ecs".to_string())
            .spawn(move || {
                #[cfg(target_os = "android")]
                panic_with_backtrace_rs();
                let mut begin = std::time::Instant::now();
                let mut fps = 0;
                let mut min_fps = 60;
                loop {
                    let begin2 = std::time::Instant::now();
                    let (task, id): (Box<dyn FnOnce() -> () + Send>, f64) = receiver.recv().unwrap();
                    // let begin3 = std::time::Instant::now();
                    // println!("============ ecs");
                    task();
                    let _ = back_sender.send(());
					if let Some(cb) = unsafe { FRAME_END_CB.get_mut() } {
						// println!("========= ondraw cb: {}", id);
                        cb(id);
                    }

                    fps += 1;
                    if begin.elapsed().as_millis() >= 1000 {
                        println!("ECS: FPS: {}, MIN_FPS: {}", fps, min_fps);
                        min_fps = fps;
                        fps = 0;
                        begin = std::time::Instant::now();
                    }

                    let time = begin2.elapsed().as_millis();
					let frame_time = unsafe { FRAME_TIME } as u128;
                    if time < frame_time {
                        thread::sleep(Duration::from_millis((frame_time - time) as u64));
                    }
                    min_fps = min_fps.min(1000 / begin2.elapsed().as_millis());
					
                }
            });
        Self {
            app,
            last_frame_awaiting: false,
            sender,
            back_receiver,
			id: 0.0,
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
	pub fn new(app: App) -> Self {Self{ app } }
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
	pub fn get_string_by_hash(value: f64) -> Option<String> { 
		match pi_atom::get_by_hash(unsafe {transmute(value)}) {
			Some(r) => Some(r.as_ref().to_string()),
			None => None,
		} 
	}

	#[cfg(feature = "pi_js_export")]
	pub fn get_hash(&self) -> f64 { unsafe {transmute(self.0.str_hash())} }
}

/// 设置日志过滤器
/// 如果过滤器格式错误， 日志过滤器未初始化， 则设置将失败， 但不会panic
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn set_log_filter(engine: &mut Engine, filter: &str) {
	#[cfg(feature = "spector")]
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
pub fn create_engine(canvas: web_sys::HtmlCanvasElement, width: u32, height: u32, asset_mgr: &ResAllocator, asset_total_capacity: u32, asset_config: &str, log_filter: Option<String>, log_level: u8, collect_interval: u32,
	trace_record_or_play: Option<f64>
) -> Engine {
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

	// console_log::init_with_level(log::Level::Error);

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
	

	app.add_plugins(log);
	create_engine_inner(
		&mut app, 
		pi_bevy_winit_window::WinitPlugin::new(window).with_size(width, height),
		asset_total_capacity,
		asset_config,
		collect_interval as u64,
		Some(asset_mgr.get_inner().clone()),
		trace_record_or_play
	);
    app.add_plugins(RuntimePlugin); // wasm需要主动推运行时

    let engine = Engine::new(app);

	engine
}

#[cfg(feature="pi_js_export")]
#[cfg(not(target_arch = "wasm32"))]
pub fn create_engine(window: &Arc<Window>, width: u32, height: u32, asset_mgr: &ResAllocator, asset_total_capacity: u32, asset_config: &str, collect_interval: u32,
	trace_record_or_play: Option<f64>
) -> Engine {
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
		collect_interval as u64,
		Some(asset_mgr.get_inner().clone()),
		trace_record_or_play
	);

    let engine = Engine::new(app);

	engine
}

pub fn create_engine_inner(
	app: &mut App, 
	winit_plugin: pi_bevy_winit_window::WinitPlugin,
	asset_total_capacity: u32,
	asset_config: &str,
	collect_interval: u64,
	asset_allotor: Option<Share<ShareCell<pi_assets::allocator::Allocator>>>,
	trace_record_or_play: Option<f64>
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
		.add_plugins(PiAssetPlugin {total_capacity: asset_total_capacity as usize, collect_interval, asset_config: parse_asset_config(asset_config), allocator: asset_allotor})
		.add_plugins(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugins(PluginWindowRender)
		.add_plugins(PiPostProcessPlugin);

	let trace_record_or_play = if let Some(trace_record_or_play) = trace_record_or_play {
		unsafe { transmute(trace_record_or_play as u8) }
	} else { pi_bevy_render_plugin::cmd_play::TraceOption::None };
	app.add_plugins(GlobalCmdTracePlugin { option: trace_record_or_play });
	// #[cfg(not(target_arch="wasm32"))]
	// app.add_plugins(PluginSpector);
	// #[cfg(not(target_arch="wasm32"))]
	// app.add_plugins(PluginSpectorUI);
}

// 在wasm目标上,返回渲染图的topo图
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn dump_graphviz(engine: &Engine) -> String  {
	let g = engine.world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
	g.dump_graphviz()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn dump_toop_graphviz(engine: &Engine) -> String  {
	let g = engine.world.get_single_res::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
	g.dump_toop_graphviz()
}

// 在wasm目标上,返回system依赖图
#[cfg(feature="system_graph")]
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn dump_system(engine: &mut Engine) -> String  {
	let label = bevy::prelude::Update;
	engine.0.world
	.resource_scope::<bevy::prelude::Schedules, _>(|world, mut schedules| {
		let schedule = schedules
			.get_mut(&bevy::prelude::Update)
			.ok_or_else(|| format!("schedule with label {label:?} doesn't exist"))
			.unwrap();

		bevy_mod_debugdump::schedule_graph::schedule_graph_dot(schedule, world, &Default::default())
	})
	// bevy_mod_debugdump::schedule_graph_dot(&mut engine.0, bevy::prelude::Update, &Default::default())
}

static IS_FIRST: AtomicBool = AtomicBool::new(true);

// 帧推
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn fram_call(engine: &mut Engine, reset_state: bool) {
    use std::sync::atomic::Ordering;

    use crate::event::{on_change, IS_CHANGED};
    

    // 推动高性能低精度本地时钟
    pi_time::tick_clock();

	#[cfg(feature = "trace")]
	let _span = tracing::warn_span!("frame_call").entered();
	// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::Active;

	// log::warn!("fram_call start=====");
	// await_last_frame(engine);
	// log::warn!("fram_call start1=====");
	#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
	{
		engine.last_frame_awaiting = true;
		let engine: &'static mut Engine = unsafe { transmute(engine) };

		if IS_FIRST.load(Ordering::Relaxed){
			// IS_FIRST.store(false, Ordering::Relaxed);
			let device = engine.world.get_single_res_mut::<PiRenderDevice>().unwrap();
			device.unmake_current();
		}
		
		let sender = engine.sender.clone();
		let id =  engine.id;
		engine.id += 1.0;
		// println!("================ send fram_call: {}", id);
		let _ = sender.send((Box::new(move || {

			if IS_FIRST.load(Ordering::Relaxed){
				IS_FIRST.store(false, Ordering::Relaxed);
				let device = engine.world.get_single_res::<PiRenderDevice>().unwrap();
				device.make_current();
			}

			// if reset_state {
				// device.reset_state();
			// }
			
			// bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
			engine.run();

			if IS_CHANGED.load(Ordering::Relaxed) {
				IS_CHANGED.store(false, Ordering::Relaxed);
				on_change(engine);
			}
			// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
			// log::warn!("fram_call end=====");
		}), id));
	}
	
	#[cfg(target_arch="wasm32")]
	{
		if reset_state {
			let device = engine.world.get_single_res_mut::<PiRenderDevice>().unwrap();
			// log::warn!("reset_state==================");
			device.reset_state();
		}
		// bevy_ecs::system::CommandQueue::default().apply(&mut engine.world);
		engine.run();

		if reset_state {
			let device = engine.world.get_single_res_mut::<PiRenderDevice>().unwrap();
			// log::warn!("reset_state==================");
			device.reset_state();
		}
		// *engine.world.get_single_res_mut::<FrameState>().unwrap() = FrameState::UnActive;
	}
}

// 将实体id转化为asimage的url
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn entity_to_asimage_url(entity: f64) -> String {
	let entity = unsafe { transmute::<f64, pi_world::world::Entity>(entity) };
	pi_bevy_render_plugin::asimage_url::entity_to_asimage_url(entity)
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
			"TEXTURE_RES" => asset_config.insert::<ImageTextureFrame>(desc),
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
		// rt.poll();
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


#[derive(pi_scene_shell::prelude::Resource, Default)]
pub struct VertexBufferRefs {
	pub vertexs: XHashMap<pi_scene_shell::prelude::KeyVertexBuffer, Vec<u8>>,
	pub indices: XHashMap<pi_scene_shell::prelude::KeyVertexBuffer, Vec<u8>>,
	pub indicesu32: XHashMap<pi_scene_shell::prelude::KeyVertexBuffer, Vec<u8>>,
}
pub fn sys_vertex_buffer(
	mut refs: pi_scene_shell::prelude::ResMut<VertexBufferRefs>,
    device: pi_scene_shell::prelude::Res<pi_scene_shell::prelude::PiRenderDevice>,
    queue: pi_scene_shell::prelude::Res<pi_scene_shell::prelude::PiRenderQueue>,
    vb_mgr: pi_scene_shell::prelude::Res<pi_scene_shell::prelude::ShareAssetMgr<pi_scene_shell::prelude::EVertexBufferRange>>,
    mut vb_wait: pi_scene_shell::prelude::ResMut<pi_scene_shell::prelude::VertexBufferDataMap3D>,
) {
	
	refs.vertexs.drain().for_each(|(key, data)| {
		let key_u64 = key.asset_u64();
		if let Some(buffer) = vb_mgr.get(&key_u64) {
			queue.write_buffer(buffer.buffer(), 0, &data);
		} else {
			pi_scene_context::prelude::ActionVertexBuffer::create(&mut vb_wait, key, data);
		}
	});
	refs.indices.drain().for_each(|(key, data)| {
		let key_u64 = key.asset_u64();
		if let Some(buffer) = vb_mgr.get(&key_u64) {
			queue.write_buffer(buffer.buffer(), 0, &data);
		} else {
			pi_scene_context::prelude::ActionVertexBuffer::create_indices(&mut vb_wait, key, data);
		}
	});
	refs.indicesu32.drain().for_each(|(key, data)| {
		let key_u64 = key.asset_u64();
		if let Some(buffer) = vb_mgr.get(&key_u64) {
			queue.write_buffer(buffer.buffer(), 0, &data);
		} else {
			pi_scene_context::prelude::ActionVertexBuffer::create_indices(&mut vb_wait, key, data);
		}
	});
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn init_engine_3d(app: &mut Engine, spine: bool, param: &[u32]) {
use pi_3d::DisplayBoxs;
use pi_bevy_render_plugin::FrameDataPrepare;
use pi_bevy_render_plugin::GraphBuild;
use pi_scene_shell::prelude::WorldResourceTemp;
	use pi_scene_shell::prelude::AppResourceTemp;
	use pi_scene_shell::run_stage::EngineCustomPlugins;
	use pi_world::prelude::IntoSystemConfigs;

use crate::asset::sys_screen_with_postprocess;
use crate::record::Records3D;

    if app.world.get_resource::<pi_scene_shell::prelude::AssetMgrConfigs>().is_none() {
        app.insert_resource(pi_scene_shell::prelude::AssetMgrConfigs::default());
    }
	
    if app.world.get_resource::<pi_bevy_render_plugin::render_cross::CrossRenderDrawListEntities>().is_none() {
        app.insert_resource(pi_bevy_render_plugin::render_cross::CrossRenderDrawListEntities::default());
    }

	let engineplugins = EngineCustomPlugins::new(param);
	app.insert_resource(engineplugins);

	app.insert_resource(Records3D::default());

	
	#[cfg(target_arch = "wasm32")]
	app.insert_resource(DisplayBoxs::default());

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

	app.insert_resource(VertexBufferRefs::default());
	app.add_systems(
        pi_world::schedule::Update,
        sys_vertex_buffer.in_set(pi_scene_shell::prelude::ERunStageChap::New)
	);
    app.add_systems(
        pi_world::schedule::Update,
        pi_scene_context::prelude::sys_state_transform.in_set(pi_scene_shell::prelude::ERunStageChap::StateCheck)
    );
	app.add_systems(
		pi_world::schedule::PreUpdate,
		sys_screen_with_postprocess.in_set(FrameDataPrepare).before(GraphBuild)
	);
}

#[cfg(feature = "pi_js_export")]
pub fn bind_context(app: &mut Engine) {
	use pi_bevy_render_plugin::PiRenderDevice;
	let device = app.world.get_single_res_mut::<PiRenderDevice>().unwrap();
	// device.make_current();
}

#[cfg(feature = "pi_js_export")]
pub fn unbind_context(app: &mut Engine) {
	use pi_bevy_render_plugin::PiRenderDevice;
	let device = app.world.get_single_res_mut::<PiRenderDevice>().unwrap();
	// device.unmake_current();
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn set_frame_pixel_ratio(app: &mut Engine, pixel_ratio: f32) {
	use pi_bevy_render_plugin::system::PixelRatio;
	{
		if let Some(ratio) = app.world.get_single_res_mut::<PixelRatio>(){
			ratio.0 = pixel_ratio.clamp(0.2, 1.0);
			return;
		}
	}
	
	{
		app.world.insert_single_res(PixelRatio(pixel_ratio.clamp(0.2, 1.0)));
	}
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn init_brotli_dictionary(data: &[u8]) {
	use brotli_decompressor::dictionary;
	// dictionary::init_brotli_dictionary(data.to_vec());
}

