use std::panic;

pub use pi_export_assets_mgr::*;
pub use pi_export_gui::*;
// pub use pi_export_astar::export:: *;
pub use pi_export_base::export::*;
// pub use pi_export_quad_tree::export::*;
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
 	sprite::*,
};

pub use pi_spatial::*;
pub use pi_path_finding::*;
pub use pi_orca::*;
pub use pi_export_task_pool::exports::*;
#[cfg(target_arch = "wasm32")]
pub use pi_bon_decode::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	type Error;

	#[wasm_bindgen(constructor)]
	fn new() -> Error;

	#[wasm_bindgen(structural, method, getter)]
	fn stack(error: &Error) -> String;
}

#[cfg(feature="const_memory")]
#[cfg(not(debug_assertions))]
#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = unsafe {
    static mut MEMORY: [u8; 96 * 1024 * 1024] = [0; 96 * 1024 * 1024];
    let span = talc::Span::from_const_array(std::ptr::addr_of!(MEMORY));
    talc::Talc::new(talc::ClaimOnOom::new(span)).lock()
};

#[cfg(not(feature="const_memory"))]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn get_counters() -> String {
	"". to_string()
}

#[cfg(feature="const_memory")]
#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn get_counters() -> String {
	#[cfg(not(debug_assertions))]
	return format!("{:?}", ALLOCATOR.lock().get_counters());
	#[cfg(debug_assertions)]
	"debug is not talc!!!".to_string()
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
