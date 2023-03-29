use std::time::Duration;
use std::{sync::Arc, time::Instant, mem::transmute};

use async_trait::async_trait;
use bevy::ecs::{
    schedule::{StageLabel},
    system::{SystemState},
};
use bevy::prelude::App;
use bevy::window::WindowId;
use bevy::winit::WinitPlugin;
use pi_async::prelude::AsyncRuntime;
use pi_bevy_post_process::PiPostProcessPlugin;
use pi_bevy_render_plugin::{PiRenderPlugin, FrameState};
use pi_flex_layout::prelude::Size;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_share::{Share, ShareMutex};
use pi_export_gui::{Engine, Gui, fram_call};
use pi_ui_render::system::RunState;
use pi_ui_render::{prelude::UiPlugin, resource::UserCommands};

#[async_trait]
pub trait Example: 'static + Sized {
    fn init(&mut self, gui: &mut Gui, engine: &mut Engine, size: (usize, usize));
    fn fram_call(&mut self, gui: &mut Gui, engine: &mut Engine);
    fn get_init_size(&self) -> Option<Size<u32>> {
        // None表示使用默认值
        None
    }
}

pub fn start<T: Example + Sync + Send + 'static>(example: T) {
    init_load_cb(Arc::new(|path: String| {
        MULTI_MEDIA_RUNTIME
            .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                if let Ok(dynamic_image) = std::fs::read(path.clone()) {
                    on_load(path.as_str(), dynamic_image);
                } else {
                    log::warn!("not find image,path: {:?}", path);
                }
            })
            .unwrap();
    }));

    let size = example.get_init_size();
    // let mut window_plugin = bevy_window::WindowPlugin::default();
    let (width, height) = if let Some(size) = size {
        // window_plugin.window.width = size.width as f32;
        // window_plugin.window.height = size.height as f32;
		(size.width, size.height)
    } else {
		(450, 720)
	};

    let exmple = Share::new(ShareMutex::new(example));
    // let exmple_run = move |world: &mut World, commands: &mut SystemState<(ResMut<UserCommands>, Commands)>| {
    //     // log::warn!("zzzzzzzzzzzzzzzzzzzzzzzzbbbbbb");
    //     let mut commands = commands.get_mut(world);
    //     exmple.lock().render(&mut commands.0, &mut commands.1);
    // };

	let mut engine = create_engine(width, height);

	engine.world.insert_resource(RunState::RENDER);
    engine.add_plugin(UiPlugin);

    // engine.0
	// 	// .add_system_to_stage(CoreStage::First, exmple_run.before(user_setting))
	// 	.add_stage_before(
	// 		CoreStage::First,
	// 		InitStartupStage::Startup,
	// 		SystemStage::parallel().with_run_criteria(ShouldRun::once),
	// 	);
	engine.app_mut().update();
	let world = &mut engine.world;
	let mut gui = Gui {
		down_query: world.query(),
		up_query: world.query(),
		layer_query: world.query(),
		enable_query: world.query(),
		depth_query: world.query(),
		layout_query: world.query(),
		quad_query: world.query(),
		matrix_query: world.query(),
		overflow_query: world.query(),
		in_pass2d_query: world.query(),
		graph_id: world.query(),
		query_state: SystemState::new(world),
		entitys: unsafe { transmute(world.entities()) },
		commands: UserCommands::default(),
	};
	exmple.lock().init(&mut gui, &mut engine, (500, 500));
	loop {
		exmple.lock().fram_call(&mut gui, &mut engine);
		fram_call(&mut engine, 0);
		std::thread::sleep(Duration::from_millis(16));
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum InitStartupStage {
    /// The [`Stage`](bevy::ecs::schedule::Stage) that runs once before [`StartupStage::Startup`].
    Startup,
}

pub struct PreFrameTime(pub Arc<ShareMutex<Instant>>);
pub struct FrameStartTime(pub Instant);
impl Default for FrameStartTime {
    fn default() -> Self { Self(Instant::now()) }
}

impl Default for PreFrameTime {
    fn default() -> Self { Self(Arc::new(ShareMutex::new(Instant::now()))) }
}

#[allow(dead_code)]
fn main() {}

pub fn create_engine(width: u32, height: u32) -> Engine {
	use winit::{event_loop::{EventLoopBuilder}, platform::windows::EventLoopBuilderExtWindows};
    let mut app = App::default();

	// let mut window_plugin = bevy::window::WindowPlugin::default();
	// window_plugin.window.width = width as f32;
	// window_plugin.window.height = height as f32;
	
	// app
	// 	.add_plugin(bevy::log::LogPlugin {
	// 		filter: "wgpu=info,pi_ui_render::components::user=debug".to_string(),
	// 		level: bevy::log::Level::INFO,
	// 	})
	// 	.add_plugin(bevy::input::InputPlugin::default())
	// 	.add_plugin(window_plugin)
	// 	.add_plugin(WinitPlugin::default())
	// 	// .add_plugin(WorldInspectorPlugin::new())
	// 	.add_plugin(PiRenderPlugin::default())
	// 	.add_plugin(PiPostProcessPlugin);
	let mut window_plugin = bevy::window::WindowPlugin::default();
    window_plugin.add_primary_window = false;
	// window_plugin.window.width = width as f32;
    // window_plugin.window.height = height as f32;
	// window_plugin.add_primary_window = false;

	let event_loop =  EventLoopBuilder::new().with_any_thread(true).build();
	let window = Arc::new(winit::window::Window::new(&event_loop).unwrap());
	
    app
		.add_plugin(bevy::log::LogPlugin {
			// filter: "pi_ui_render::components::user=debug".to_string(),
			filter: "".to_string(),
			level: bevy::log::Level::WARN,
		})
		// .add_plugin(bevy::input::InputPlugin::default())
		.add_plugin(window_plugin)
		.add_plugin(pi_bevy_winit_window::WinitPlugin::new(window.clone(), WindowId::primary()).with_size(width, height))
		// .add_plugin(WorldInspectorPlugin::new())
		.add_plugin(PiRenderPlugin {frame_init_state: FrameState::UnActive})
		.add_plugin(PiPostProcessPlugin);
    Engine(app)
}
