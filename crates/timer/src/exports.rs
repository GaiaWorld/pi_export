use std::mem::transmute;

use pi_cancel_timer::Timer as Timer1;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
pub struct Timer(Timer1<f64, 250, 60, 2>, u64);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
// #[pi_js_export]
impl Timer {
	/// 创建定时器
	// #[pi_js_export]
	pub fn new() -> Self {
		Self(Timer1::<f64, 250, 60, 2>::default(), 0)
	}

	/// push一个定时任务
	// #[pi_js_export]
	pub fn push(&mut self, func: f64, timeout: f32) -> f64 {
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
	pub fn pop(&mut self, now: u64) -> Option<f64> {
		let now: u64 = now / 4; // 以4ms为精度
		self.0.pop(now + self.1)
	}
	

	/// 准备弹出， 在循环弹出前， 先调用此方法
	// #[pi_js_export]
	pub fn pop_ready(&mut self) {
		self.1 = self.0.roll_count();
	}
}


