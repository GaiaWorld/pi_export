use std::sync::Arc;

use pi_assets::{allocator::Allocator, asset::{Asset, Handle, Size}, mgr::AssetMgr};
use pi_share::{Share, ShareCell};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use js_proxy_gen_macro::pi_js_export;

pub static mut DESTROY_RES: Option<Arc<dyn Fn(u32) + Send + Sync>> = None;

#[cfg(target_arch = "wasm32")]
pub struct CanSyncFunction(js_sys::Function);
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for CanSyncFunction {}
#[cfg(target_arch = "wasm32")]
unsafe impl Send for CanSyncFunction {}
#[cfg(target_arch = "wasm32")]
impl CanSyncFunction {
	fn call(&self, v: &JsValue, v1: &JsValue) {
		self.0.call1(v, v1);
	}
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub fn set_destroy_callback(f: js_sys::Function) {
	let f1 = CanSyncFunction(f);
	unsafe {DESTROY_RES = Some(Arc::new(move |value: u32| {
		f1.call(&JsValue::from_f64(0.0), &value.into());
	}))};
}
#[cfg(not(target_arch = "wasm32"))]
#[pi_js_export]
pub fn set_destroy_callback(f: Arc<dyn Fn(u32, Option<Box<dyn FnOnce(Result<u32, String>) + Send + 'static>>) + Send + Sync + 'static>) {
	unsafe { DESTROY_RES = Some(Arc::new(move |value: u32| {
		(f)(value, None);
	}))};
}

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pi_js_export]
/// 资源包装
pub struct ResRef(Handle<JsRes>);

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ResMgr {
	inner: Share<AssetMgr<JsRes>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pi_js_export]
impl ResMgr {
	/// 创建一个资源， 如果资源已经存在，旧的资源将被覆盖
	/// 如果创建的资源类型未注册，将崩溃
	#[pi_js_export]
	pub fn create_res(&mut self, key: u32, cost: u32) -> ResRef {
		match self.inner.insert(key, JsRes {key, cost: cost as usize}) {
			Ok(r) => ResRef(r),
			_ => unreachable!()
		}
	}

	/// 获取资源
	#[pi_js_export]
	pub fn get_res(&mut self, key: u32) -> Option<ResRef> {
		match self.inner.get(&key) {
			Some(r) => Some(ResRef(r)),
			None => None
		}
	}
}

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ResAllocator {
	inner: Share<ShareCell<Allocator>>,
}

impl ResAllocator {
	pub fn get_inner(&self) -> &Share<ShareCell<Allocator>>{
		&self.inner
	}

	pub fn get_inner_mut(&mut self) -> &mut Share<ShareCell<Allocator>>{
		&mut self.inner
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[pi_js_export]
impl ResAllocator {
	/// 创建资源管理器的实例
	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
	#[pi_js_export]
	pub fn new(total_capacity: u32) -> Self {
		let r = if total_capacity > 0 {
			Allocator::new(total_capacity as usize)
		} else {
			Allocator::new(16 * 1024 * 1024)
		};
		Self{inner: Share::new(ShareCell::new(r))}
	}

	/// 整理方法， 将无人使用的资源放入到LruCache， 清理过时的资源
	/// 就是LruMgr有总内存上限， 按权重分给其下的LRU。 如果有LRU有空闲， 则会减少其max_size, 按权重提高那些满的LRU的max_size
	#[pi_js_export]
	pub fn collect(&mut self) {
		self.inner.borrow_mut().collect(pi_time::now_millisecond());
	}

	// 10 * 1024 * 1024,
	// 		50 * 1024 * 1024,
	// 		5 * 60000,
	/// 创建一个资源， 如果资源已经存在，则会修改资源的配置
	#[pi_js_export]
	pub fn register_to_resmgr(&mut self, ty: u32, min_capacity: u32, weight: u32, time_out: u32) -> ResMgr {
		let mut m = pi_assets::mgr::AssetMgr::<JsRes>::new(pi_assets::asset::GarbageEmpty(),
		false,
		min_capacity as usize,
		time_out as usize,);
		let m1 = Share::get_mut(&mut m).unwrap();
		m1.ty = ty; // 标记类型
		self.inner.borrow_mut().register(m.clone(), min_capacity as usize, weight as usize);
		ResMgr{
			inner: m,
		}
	}
	// 资产管理器总大小
    #[pi_js_export]
	pub fn size(&self) -> f32 {
		let account = self.inner.borrow().account();
		account.cur_total_size
	}

	// 统计详情
    #[pi_js_export]
	pub fn account_details(&self) -> String {
		let account = self.inner.borrow().account();
		serde_json::to_string(&account).unwrap()
	}
}

/// 资源包装
pub struct JsRes {
	key: u32,
	cost: usize,
}

impl Asset for JsRes {
    type Key = u32;
}

impl Size for JsRes {
	/// 资产的大小
	fn size(&self) -> usize {
		self.cost
	}
}

impl std::ops::Drop for JsRes {
    #[allow(static_mut_ref)]
	fn drop(&mut self) {
		unsafe { 
			if let Some(r) = &DESTROY_RES {
				(r)(self.key)
			};
		}
    }
}
