use std::panic;

pub use pi_export_gui::*;
// pub use pi_export_astar::export:: *;
pub use pi_export_base::export::*;
// pub use pi_export_quad_tree::export::*;
pub use pi_spine_export:: *;
pub use scene3d_export::{
	engine::*,
	scene::*,
	transform_node::*,
	camera::*,
	mesh::*,
	instance_mesh::*,
	material::*,
	geometry::*,
	lights::*,
	node_materials::*,
	animation::*,
 	trail::*,
 	skin::*,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	type Error;

	#[wasm_bindgen(constructor)]
	fn new() -> Error;

	#[wasm_bindgen(structural, method, getter)]
	fn stack(error: &Error) -> String;
}


#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn init_logger(_level: pi_web_logger::Level) {
    // pi_web_logger::init_with_level(level);
	// panic::set_hook(Box::new(|info: &panic::PanicInfo| {
	// 	let mut msg = info.to_string();
	// 	msg.push_str("\n\nStack:\n\n");
	// 	let e = Error::new();
	// 	// let stack = e.stack();
	// 	// msg.push_str(&stack);

	// 	// Safari's devtools, on the other hand, _do_ mess with logged
	// 	// messages' contents, so we attempt to break their heuristics for
	// 	// doing that by appending some whitespace.
	// 	// https://github.com/rustwasm/console_error_panic_hook/issues/7
	// 	// msg.push_str("\n\n");
	// 	log::error!("{}\n\nStack:\n\n{:?}\n\n",info,  e.stack());
	// }));
	// info!("init_logger ok!");
}