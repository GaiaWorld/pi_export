// use pi_bevy_render_plugin::window_state::{WindowState, WindowStateCmd};
use pi_bevy_render_plugin::PiRenderWindow;
use pi_bevy_render_plugin::PiScreenTexture;
use pi_bevy_render_plugin::IS_RESUMED;
// use pi_bevy_winit_window::update_window_handle;
use crate::export::await_last_frame;
pub use crate::export::Engine;
pub use pi_winit::window::Window;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use pi_bevy_render_plugin::PiRenderDevice;

#[cfg(feature = "pi_js_export")]
pub fn on_resumed(engine: &mut Engine, window: &Arc<Window>) {

    await_last_frame(engine);
    println!("----------on_resumed222222");
    // android 某些设备在某些情况下不会触发on_suspended再触发on_resumed
    let world = &mut engine.app.world;
    // let device = engine.world.get_single_res_mut::<PiRenderDevice>().unwrap();
    // device.0.make_current();
    world
        .get_single_res_mut::<PiScreenTexture>()
        .unwrap()
        .0
        .take();
    // let w = update_window_handle(&mut app.world, window.as_ref());
    // let raw_handle = pi_bevy_winit_window::HandleWrapper {
    //     handle: Arc::new(WindowWrapper(window.clone())),
    // };

    // world
    //     .get_single_res_mut::<PiRenderWindow>().unwrap()
    //     .update_handle(Arc::new(WindowWrapper(window.clone())));

    IS_RESUMED.store(true, Ordering::Relaxed);
}

#[cfg(feature = "pi_js_export")]
pub fn on_suspended(engine: &mut Engine, version: String) {

    if version.contains("Android 13"){
        
    }
    await_last_frame(engine);
    let device = engine.world.get_single_res_mut::<PiRenderDevice>().unwrap();
    device.0.unmake_current();

    println!("----------on_suspended222222: {}", version);
    // let world = &mut engine.app.world;

    engine.world
        .get_single_res_mut::<PiScreenTexture>()
        .unwrap()
        .0
        .take();
    IS_RESUMED.store(false, Ordering::Relaxed);
}
