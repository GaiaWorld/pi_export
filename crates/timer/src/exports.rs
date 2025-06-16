use std::mem::transmute;

use js_sys::Function;
use pi_cancel_timer::Timer as Timer1;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
pub struct Timer(Timer1<Function, 128, 16, 1>);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
impl Timer {
	/// 创建定时器
	// #[pi_js_export]
	pub fn new() -> Self {
		Self(Timer1::<Function, 128, 16, 1>::default())
	}

	/// push一个定时任务
	// #[pi_js_export]
	pub fn push(&mut self, func: Function, timeout: f32) -> f64 {
		let r = self.0.push(timeout as usize, func);
		unsafe { transmute(r) }
	}

	/// 取消一个定时任务
	// #[pi_js_export]
	pub fn cancel(&mut self, key: u64) -> Option<Function> {
		let key = unsafe { transmute(key) };
		self.0.cancel(key)
	}

	/// 弹出一个定时任务
	// #[pi_js_export]
	pub fn pop(&mut self, now: u64) -> Option<Function> {
		self.0.pop(now)
	}
	

	/// 轮滚动 - 向后滚动一个最小粒度, 可能会造成轮的逐层滚动。如果滚动到底，则修正堆上全部的定时任务，并将堆上的到期任务放入轮中
	// #[pi_js_export]
	pub fn roll(&mut self) {
		self.0.roll();
	}
}


