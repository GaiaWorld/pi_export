use pi_ui_render::prelude::UiPlugin;
pub use super::index::Gui;
pub use pi_export_base::export::{Engine, Atom};
// pub use pi_ui_render::gui::Gui;
use std::intrinsics::transmute;
pub use winit::window::Window;
use std::sync::Arc;
use pi_slotmap::DefaultKey;

// #[derive(Debug, Clone)]
// #[cfg(feature="pi_js_export")]
// pub enum TraceOption {
// 	None = 0,
// 	Record = 1,
// 	Play = 2,
// }

#[allow(unused_variables)]
#[cfg(feature="pi_js_export")]
pub fn create_gui(
    context: u32,
    engine: &mut Engine,
    width: f32,
    height: f32,
    load_sdf_fun: Arc<dyn Fn(f64, f64, Vec<u8>, Option<Box<dyn FnOnce(Result<u32, String>) + Send + 'static>>) + Send  + 'static>,
    class_sheet: u32,
    font_sheet: u32,
    cur_time: u32,
    animation_event_fun: u32,
	debug: u32,
	use_sdf: bool,
) -> Gui {
    use pi_export_base::export::await_last_frame;
    use pi_render::font::FontType;

	await_last_frame(engine);

    let mut gui = Gui::new(engine);

	#[cfg(feature="record")]
	{
		let debug: pi_ui_render::system::cmd_play::TraceOption = unsafe { transmute(debug as u8) };
		engine.add_plugins(UiPlugin {cmd_trace: debug.clone(), font_type: FontType::Sdf2});
		gui.record_option = debug;
	}

	#[cfg(not(feature="record"))]
    engine.add_plugins(UiPlugin{ font_type: FontType::Sdf2 });

	// sdf
	// let fun: Arc<dyn Fn(f64, f64, Vec<u8>, Option<Box<dyn FnOnce(Result<u32, String>) + Send + Sync + 'static>>) + Send + Sync + 'static>  = unsafe { transmute(load_sdf_fun)};
	// if use_sdf {
	// 	pi_hal::font::sdf_table::init_load_cb(std::sync::Arc::new(move |key: DefaultKey, font_family: usize, chars: &[char]| {
	// 		let r: Vec<u8> = Vec::from(bytemuck::cast_slice::<_, u8>(unsafe {transmute::<_, &[u32]>(chars)}) );
	// 		fun(unsafe {transmute::<_, f64>(key)}, font_family as f64, r , None);
	// 		// let chars1 = js_sys::Uint32Array::from(unsafe {transmute::<_, &[u32]>(chars)});
	// 		// fun.call3(&JsValue::from(0), &unsafe {transmute::<_, f64>(key)}.into(), &(font_family as u32).into(), chars1.as_ref()); 
	// 	}));
	// }

	// // 设置动画的监听器
    // let a_callback = Share::new(move |list: &Vec<(AnimationGroupID, EAnimationEvent, u32)>, map: &SecondaryMap<AnimationGroupID, (ObjKey, pi_atom::Atom)>| {
    // 	let mut arr: Vec<f64> = Vec::new();
    // 	for (group_id, ty, count) in list.iter() {
    // 		match map.get(*group_id) {
    // 			Some(r) => {
	// 				arr.push(unsafe { transmute::<_, f64>(r.0.to_bits()) }); // entity
	// 				arr.push(r.1.str_hash() as f64); // name hash
    // 			},
    // 			None => continue,
    // 		};
    // 		arr.push(unsafe {transmute::<_, u8>(*ty)}  as f64); // event type
    // 		arr.push(*count as f64); // cur iter count
    // 	}
    // 	animation_event_fun(context, arr, None);
    // });
    // gui.commands.push_cmd(AnimationListenCmd(a_callback));

    gui
}

#[cfg(feature = "pi_js_export")]
pub fn create_fragment(gui: &mut Gui, arr: &mut [f64], count: u32, key: u32) {
    use pi_ui_render::resource::FragmentCommand;

	let mut index: usize = 0;
	let mut entitys = Vec::with_capacity(count as usize);
	while index < count as usize {
		let entity = gui.entitys.reserve_entity();
		arr[index] = unsafe { transmute(entity.to_bits()) };
		entitys.push(entity);
		index = index + 1;
	}
	// log::warn!("entitys=============={:?}", entitys);
	gui.commands
		.fragment_commands
		.push(FragmentCommand { key, entitys });
}