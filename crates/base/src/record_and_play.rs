
use js_proxy_gen_macro::pi_js_export;
use pi_bevy_render_plugin::{PlayState, Records};
use crate::asset::Engine;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

// 每帧取record
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_record(engine: &mut Engine) -> Vec<u8> {
	crate::export::await_last_frame(engine);
	#[cfg(feature="record")]
	{
    use pi_bevy_render_plugin::Records;

		let records = engine.world.get_single_res_mut::<Records>().unwrap();

		let r = &*records;
		let r = r.bin();
		records.clear();
		r
	}
	#[cfg(not(feature="record"))]
	Vec::<u8>::default()
}

// 取record长度, 单位：字节， 高层可根据长度来决定是否将record全部取出
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn get_record_len(engine: &mut Engine) -> u32 {
	crate::export::await_last_frame(engine);
	#[cfg(feature="record")]
	{
		let records = engine.world.get_single_res_mut::<Records>().unwrap();
		records.list.len() as u32
	}
	#[cfg(not(feature="record"))]
	0
}

// 设置下一帧的指令记录
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn set_next_record(engine: &mut Engine, bin: &[u8]) {
    // use pi_ui_render::system::base::node::cmd_play::PlayState;
	#[cfg(feature="record")]
	{
        use pi_bevy_render_plugin::Records;

        engine.world.init_single_res::<Records>();
        let records = engine.world.get_single_res_mut::<Records>().unwrap();
        if let Some(list) = Records::frames(bin) {
            records.list = list;
        }
        // log::warn!("set_next_record===={:?}", r.list.len());
        // 重设播放状态
        let play_state = engine.world.get_single_res_mut::<PlayState>().unwrap();
        play_state.is_running = true;
        play_state.next_reord_index = 0;
        play_state.next_state_index = 0;
        play_state.cur_frame_count = 0;
	}
	
}

// 设置下一帧的指令记录为最后一次设置的记录（重复播放最后一次）
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn set_next_record_last(engine: &mut Engine) {
    #[cfg(feature="record")]
    {
        use pi_bevy_render_plugin::{PlayState, Records};

        let records = engine.world.get_single_res_mut::<Records>().unwrap();
        records.cur_frame_count = 0;
        // log::warn!("set_next_record===={:?}", r.list.len());
        // 重设播放状态
        let play_state = engine.world.get_single_res_mut::<PlayState>().unwrap();
        play_state.is_running = true;
        play_state.next_reord_index = 0;
        play_state.next_state_index = 0;
        play_state.cur_frame_count = 0;
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn is_play_end(engine: &mut Engine) -> bool {
	crate::export::await_last_frame(engine);
	#[cfg(feature="record")]
	match engine.world.get_single_res_mut::<PlayState>() {
		Some(r) => {
            !r.is_running},
		None => false,
	}
	#[cfg(not(feature="record"))]
	false
}