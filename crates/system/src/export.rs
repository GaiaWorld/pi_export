// use pi_bevy_render_plugin::window_state::{WindowState, WindowStateCmd};
use pi_bevy_render_plugin::PiRenderWindow;
// use pi_bevy_winit_window::update_window_handle;
pub use pi_export_base::export::Engine;
pub use winit::window::Window;
use pi_bevy_render_plugin::PiScreenTexture;
use std::sync::Arc;
use pi_bevy_render_plugin::IS_RESUMED;
use std::sync::atomic::Ordering;

#[cfg(feature = "pi_js_export")]
pub fn on_resumed(app: &mut Engine, window: &Arc<Window>) {
	// pi_export_base::export::await_last_frame(app);
    // println!("----------on_resumed222222");
    // // android 某些设备在某些情况下不会触发on_suspended再触发on_resumed
    // app.world.resource_mut::<PiScreenTexture>().0.take();
    // let w = update_window_handle(&mut app.world, window.as_ref());
    // app.world
    //     .resource_mut::<PiRenderWindow>()
    //     .0
    //     .update_handle(w);
    // IS_RESUMED.store(true, Ordering::Relaxed);
}

#[cfg(feature = "pi_js_export")]
pub fn on_suspended(app: &mut Engine) {
	// pi_export_base::export::await_last_frame(app);
    // println!("----------on_suspended222222");
    // app.world.resource_mut::<PiScreenTexture>().0.take();
}
