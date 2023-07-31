use pi_bevy_render_plugin::window_state::{WindowState, WindowStateCmd};
pub use pi_export_base::export::Engine;

#[cfg(feature = "pi_js_export")]
pub fn on_resumed(app: &mut Engine) {
    println!("----------on_resumed222222");
    app.world
        .resource_mut::<WindowStateCmd>()
        .0
        .push(WindowState::Resumed);
}

#[cfg(feature = "pi_js_export")]
pub fn on_suspended(app: &mut Engine) {
    use pi_bevy_render_plugin::{FrameState, PiScreenTexture};

    println!("----------on_suspended22222222222");
    {
        app.world.resource_mut::<PiScreenTexture>().0.take();
    }
    // let mut is_active = true;
    // {
    //     let mut state = app.world.resource_mut::<FrameState>();

    //     if let FrameState::UnActive = state.as_ref() {
    //         is_active = false;
    //     }
    //     if !is_active {
    //         *state.as_mut() = FrameState::Active;
    //     }
    // }
    // for _ in 0..100 {
    //     app.update();
    // }

    // if !is_active {
    //     *app.world.resource_mut::<FrameState>().as_mut() = FrameState::UnActive;
    // }
}
