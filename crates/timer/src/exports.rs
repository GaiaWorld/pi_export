use std::mem::transmute;

use pi_cancel_timer::Timer as Timer1;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
pub struct Timer(Timer1<f64, 128, 60, 2>, u64);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
impl Timer {
	/// 创建定时器
	// #[pi_js_export]
	pub fn new() -> Self {
		Self(Timer1::<f64, 128, 60, 2>::default(), 0)
	}

	/// push一个定时任务
	// #[pi_js_export]
	pub fn push(&mut self, func: f64, mut timeout: f64) -> f64 {
		if (timeout < 0) {
			timeout = 0;
		}
		let r = self.0.push(timeout as usize, func);
		unsafe { transmute(r) }
	}

	/// 取消一个定时任务
	// #[pi_js_export]
	pub fn cancel(&mut self, key: u64) -> Option<f64> {
		let key = unsafe { transmute(key) };
		self.0.cancel(key)
	}

	/// 弹出一个定时任务
	// #[pi_js_export]
	pub fn pop(&mut self, now: f64) -> Option<f64> {
		self.0.pop(now as u64)
	}
}


