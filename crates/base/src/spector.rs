use js_proxy_gen_macro::pi_js_export;
use pi_3d::{_request_camera, _request_meshpass, _request_select3d, _request_transform};
use pi_bevy_render_plugin::_request_document;

use pi_ui_render::tools::{
    _request_computed, _request_global_info, 
    _request_global_interface, _request_right_key_element,
    _request_showbox, _request_style, 
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use crate::{as_entity, Engine};


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_document(app: &mut Engine) -> String {
	crate::export::await_last_frame(app);

    serde_json::to_string(&_request_document(&mut app.world)).unwrap()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_style(app: &mut Engine, entity: f64) -> String {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_style(&mut app.world, entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_computed(app: &mut Engine, entity: f64) -> String {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_computed(&mut app.world, entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_global_info(app: &mut Engine, cmd: &str) -> String {
	crate::export::await_last_frame(app);

    _request_global_info(&mut app.world, cmd)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_global_interface(app: &mut Engine) -> String {
	crate::export::await_last_frame(app);

    _request_global_interface(&mut app.world)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_right_key_element(app: &mut Engine, x: f64, y: f64) -> String {
	crate::export::await_last_frame(app);

    _request_right_key_element(&mut app.world, x as f32, y as f32)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_showbox(app: &mut Engine, entity: f64) {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_showbox(&mut app.world, entity);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_transform(app: &mut Engine, entity: f64) -> String {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_transform(&mut app.world, entity)
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_meshpass(app: &mut Engine, entity: f64) -> String {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_meshpass(&mut app.world, entity)
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_camera(app: &mut Engine, entity: f64) -> String {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_camera(&mut app.world, entity)
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn spector_request_select3d(app: &mut Engine, entity: f64) {
	crate::export::await_last_frame(app);

    let entity =as_entity(entity);
    _request_select3d(&mut app.world, entity)
}