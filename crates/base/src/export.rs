

use std::{sync::Arc, cell::RefCell};

use derive_deref::{Deref, DerefMut};
use pi_assets::{allocator::Allocator, asset::{Asset, Handle, Size}, mgr::AssetMgr};
use pi_bevy_asset::{PiAssetPlugin, AssetConfig, AssetDesc};
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_hash::XHashMap;
use pi_render::{rhi::{asset::{RenderRes, TextureRes}, bind_group::BindGroup, pipeline::RenderPipeline}, renderer::sampler::SamplerRes};
use pi_share::Share;
use bevy_app::prelude::App;
use pi_bevy_render_plugin::{FrameState, PiRenderPlugin};
use pi_window_renderer::PluginWindowRender;
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
pub use winit::window::Window;

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
#[derive(Debug, Deref, DerefMut)]
pub struct Engine(pub App);

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
#[derive(Debug, Deref, DerefMut)]
pub struct Engine(pub(crate) App);

impl Engine {
	pub fn new(app: App) -> Self { Self(app) }
	pub fn app(&self) -> &App { &self.0 }
	pub fn app_mut(&mut self) -> &mut App { &mut self.0 }
}

#[derive(Debug, Clone, Deref, DerefMut)]
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
		match pi_atom::Atom::get(value as usize) {
			Some(r) => Some(r.as_ref().to_string()),
			None => None,
		} 
	}

	#[cfg(feature = "pi_js_export")]
	pub fn get_hash(&self) -> u32 { self.0.get_hash() as u32 }
}

pub static mut DESTROY_RES: Option<Arc<dyn Fn(u32) + Send + Sync>> = None;

#[cfg(target_arch = "wasm32")]
pub struct CanSyncFunction(Function);
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for CanSyncFunction {}
#[cfg(target_arch = "wasm32")]
unsafe impl Send for CanSyncFunction {}
#[cfg(target_arch = "wasm32")]
impl CanSyncFunction {
	fn call(&self, v: &JsValue, v1: &JsValue) {
		self.0.call1(v, v1);
	}
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub fn set_destroy_callback(f: Function) {
	let f1 = CanSyncFunction(f);
	unsafe {DESTROY_RES = Some(Arc::new(move |value: u32| {
		f1.call(&JsValue::from_f64(0.0), &value.into());
	}))};
}
#[cfg(feature = "pi_js_export")]
#[cfg(not(target_arch = "wasm32"))]
pub fn set_destroy_callback(f: Arc<dyn Fn(u32, Option<Box<dyn FnOnce(Result<u32, String>) + Send + 'static>>) + Send + Sync + 'static>) {
	unsafe { DESTROY_RES = Some(Arc::new(move |value: u32| {
		(f)(value, None);
	}))};
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
/// 资源包装
pub struct ResRef(Handle<JsRes>);

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct ResMgr {
	inner: Share<AssetMgr<JsRes>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl ResMgr {
	/// 创建一个资源， 如果资源已经存在，旧的资源将被覆盖
	/// 如果创建的资源类型未注册，将崩溃
	#[cfg(feature = "pi_js_export")]
	pub fn create_res(&mut self, key: u32, cost: u32) -> ResRef {
		match self.inner.insert(key, JsRes {key, cost: cost as usize}) {
			Ok(r) => ResRef(r),
			_ => unreachable!()
		}
	}

	/// 获取资源
	#[cfg(feature = "pi_js_export")]
	pub fn get_res(&mut self, key: u32) -> Option<ResRef> {
		match self.inner.get(&key) {
			Some(r) => Some(ResRef(r)),
			None => None
		}
	}
}

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct ResAllocator {
	inner: Share<RefCell<Allocator>>,
}

impl ResAllocator {
	pub fn get_inner(&self) -> &Share<RefCell<Allocator>>{
		&self.inner
	}

	pub fn get_inner_mut(&mut self) -> &mut Share<RefCell<Allocator>>{
		&mut self.inner
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl ResAllocator {
	/// 创建资源管理器的实例
	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
	#[cfg(feature = "pi_js_export")]
	pub fn new(total_capacity: u32) -> Self {
		let r = if total_capacity > 0 {
			Allocator::new(total_capacity as usize)
		} else {
			Allocator::new(16 * 1024 * 1024)
		};
		Self{inner: Share::new(RefCell::new(r))}
	}

	/// 整理方法， 将无人使用的资源放入到LruCache， 清理过时的资源
	/// 就是LruMgr有总内存上限， 按权重分给其下的LRU。 如果有LRU有空闲， 则会减少其max_size, 按权重提高那些满的LRU的max_size
	#[cfg(feature = "pi_js_export")]
	pub fn collect(&mut self, now: u32) {
		self.inner.borrow_mut().collect(now as u64);
	}

	// 10 * 1024 * 1024,
	// 		50 * 1024 * 1024,
	// 		5 * 60000,
	/// 创建一个资源， 如果资源已经存在，则会修改资源的配置
	#[cfg(feature = "pi_js_export")]
	pub fn register_to_resmgr(&mut self, _ty: u32, min_capacity: u32, max_capacity: u32, time_out: u32) -> ResMgr {
		let m = pi_assets::mgr::AssetMgr::<JsRes>::new(pi_assets::asset::GarbageEmpty(),
		false,
		max_capacity as usize,
		time_out as usize,);
		self.inner.borrow_mut().register(m.clone(), min_capacity as usize, max_capacity as usize);
		ResMgr{
			inner: m,
		}
	}
}

/// 资源包装
pub struct JsRes {
	key: u32,
	cost: usize,
}

impl Asset for JsRes {
    type Key = u32;
}

impl Size for JsRes {
	/// 资产的大小
	fn size(&self) -> usize {
		self.cost
	}
}

impl std::ops::Drop for JsRes {
	fn drop(&mut self) {
		unsafe { 
			if let Some(r) = &DESTROY_RES {
				(r)(self.key)
			};
		}
    }
}

/// 设置日志过滤器
/// 如果过滤器格式错误， 日志过滤器未初始化， 则设置将失败， 但不会panic
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn set_log_filter(engine: &mut Engine, filter: &str) {
	if let Some(handle) = engine.app_mut().world.get_resource_mut::<pi_bevy_log::LogFilterHandle>() {
		if let Ok(filter_layer) = tracing_subscriber::EnvFilter::try_new(filter) {
			let _ = handle.0.modify(|filter| *filter = filter_layer);
		}
	}
}

#[cfg(target_arch = "wasm32")]
pub static mut RUNNER: OnceCell<LocalTaskRunner<()>> = OnceCell::new();

/// width、height为physical_size
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn create_engine(canvas: web_sys::HtmlCanvasElement, width: u32, height: u32, asset_total_capacity: u32, asset_config: &str, log_filter: Option<String>, log_level: u8) -> Engine {
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


    let mut app = App::default();

    let mut window_plugin = bevy_window::WindowPlugin::default();
	window_plugin.primary_window = None;

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
	// let chrome_write = ShareChromeWrite::new();
	// log.chrome_write = None;
	create_engine_inner(
		&mut app, 
		pi_bevy_winit_window::WinitPlugin::new(canvas).with_size(width, height),
		asset_total_capacity,
		asset_config,
	);
	app.add_plugins(log);
    app.add_plugins(RuntimePlugin); // wasm需要主动推运行时
    Engine::new(app)
}

#[cfg(feature="pi_js_export")]
#[cfg(not(target_arch = "wasm32"))]
pub fn create_engine(window: &Arc<Window>, width: u32, height: u32, asset_total_capacity: u32, asset_config: &str) -> Engine {
    use pi_bevy_render_plugin::PiRenderOptions;
    use wgpu::Backend;


    let mut app = App::default();
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
	
	create_engine_inner(
		&mut app, 
		pi_bevy_winit_window::WinitPlugin::new(window.clone()).with_size(width, height),
		asset_total_capacity,
		asset_config,
	);

    Engine(app)
}

fn create_engine_inner(
	app: &mut App, 
	winit_plugin: pi_bevy_winit_window::WinitPlugin,
	asset_total_capacity: u32,
	asset_config: &str,
) {
	let mut window_plugin = bevy_window::WindowPlugin::default();
	window_plugin.primary_window = None;

	app
		// .add_plugins(bevy::log::LogPlugin {
		// 	filter: "wgpu=debug".to_string(),
		// 	level: bevy::log::Level::DEBUG,
		// })
		.add_plugins(bevy_a11y::AccessibilityPlugin)
		// .add_plugins(bevy::input::InputPlugin::default())
		.add_plugins(window_plugin)
		.add_plugins(winit_plugin)
		// .add_plugins(WorldInspectorPlugin::new())
		.add_plugins(PiAssetPlugin {total_capacity: asset_total_capacity as usize, asset_config: parse_asset_config(asset_config)})
		.add_plugins(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugins(PluginWindowRender)
		.add_plugins(PiPostProcessPlugin);
}

// 在wasm目标上,返回渲染图的topo图
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
pub fn dump_graphviz(engine: &Engine) -> String  {
	let g = engine.world.get_resource::<pi_bevy_render_plugin::PiRenderGraph>().unwrap();
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
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::Active;
	engine.update();
	*engine.world.get_resource_mut::<FrameState>().unwrap() = FrameState::UnActive;
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
			"texture_view" => asset_config.insert::<RenderRes<TextureView>>(desc),
			"buffer" => asset_config.insert::<RenderRes<Buffer>>(desc),
			"sampler" => asset_config.insert::<SamplerRes>(desc),
			"bind_group" => asset_config.insert::<RenderRes<BindGroup>>(desc),
			"texture" => asset_config.insert::<TextureRes>(desc),
			"render_pipeline" => asset_config.insert::<RenderRes<RenderPipeline>>(desc),
			
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
impl bevy_app::Plugin for RuntimePlugin {
    fn build(&self, app: &mut App) {
		use bevy_app::prelude::First;
		use bevy_ecs::prelude::IntoSystemConfigs;
		use pi_bevy_render_plugin::should_run;
        app.add_systems(First,
			runtime_run.run_if(should_run)
		);
    }
}
