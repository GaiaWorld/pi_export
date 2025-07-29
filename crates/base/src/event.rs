// use pi_bevy_render_plugin::window_state::{WindowState, WindowStateCmd};
use pi_bevy_render_plugin::PiRenderWindow;
use pi_bevy_render_plugin::PiScreenTexture;
use pi_bevy_render_plugin::IS_RESUMED;
// use pi_bevy_winit_window::update_window_handle;
use crate::export::await_last_frame;
pub use crate::export::Engine;
use pi_bevy_render_plugin::PiRenderDevice;
pub use pi_winit::window::Window;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone, Copy)]
pub enum WindowState {
    Resumed,
    Suspended,
}

static mut WINDOW_STATE: RwLock<WindowState> = RwLock::new(WindowState::Resumed);
pub static mut IS_CHANGED: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "pi_js_export")]
pub fn on_resumed(engine: &mut Engine, window: &Arc<Window>) {
    // await_last_frame(engine);
    println!("----------on_resumed222222");
    *unsafe { WINDOW_STATE.write().unwrap() } = WindowState::Resumed;
    let _ = unsafe { IS_CHANGED.store(true, Ordering::Relaxed) };

}

#[cfg(feature = "pi_js_export")]
pub fn on_suspended(engine: &mut Engine, version: String) {
    if version.contains("Android 13") {}
    println!("----------on_suspended222222: {}", version);
    // let world = &mut engine.app.world;
    *unsafe { WINDOW_STATE.write().unwrap() } = WindowState::Suspended;
    unsafe { IS_CHANGED.store(true, Ordering::Relaxed) };
}

pub fn on_change(engine: &mut Engine) {
    engine
        .world
        .get_single_res_mut::<PiScreenTexture>()
        .unwrap()
        .0
        .take();
    match unsafe { *WINDOW_STATE.read().unwrap() } {
        WindowState::Resumed => {
            IS_RESUMED.store(true, Ordering::Relaxed);
        }
        WindowState::Suspended => {
            IS_RESUMED.store(false, Ordering::Relaxed);
        }
    }
}
