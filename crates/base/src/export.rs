

use std::{sync::Arc, cell::RefCell};

use derive_deref::{Deref, DerefMut};
use pi_assets::{allocator::Allocator, asset::{Asset, Handle}, mgr::AssetMgr};
use pi_share::Share;
use bevy::app::App;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Function;

#[cfg(all(feature="pi_js_export", not(target_arch="wasm32")))]
#[derive(Debug, Deref, DerefMut)]
pub struct Engine(pub App);

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
#[derive(Debug, Deref, DerefMut)]
pub struct Engine(pub(crate) App);

impl Engine {
	pub fn new(app: App) -> Self { Self(app) }
	pub fn app(&self) -> &App { &self.0 }
	pub fn app_mut(&mut self) -> &mut App { &mut self.0 }
}

#[derive(Debug, Clone, Deref, DerefMut)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct Atom(pi_atom::Atom);
impl Atom {
    pub fn new(value: pi_atom::Atom) -> Self { Self(value) }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl Atom {
	#[cfg(feature = "pi_js_export")]
	pub fn from_string(value: String) -> Self { Atom::new(pi_atom::Atom::from(value)) }

	#[cfg(feature = "pi_js_export")]
	pub fn get_string_by_hash(value: u32) -> Option<String> { 
		match pi_atom::Atom::get(value as usize) {
			Some(r) => Some(r.as_ref().to_string()),
			None => None,
		} 
	}

	#[cfg(feature = "pi_js_export")]
	pub fn get_hash(&self) -> u32 { self.0.get_hash() as u32 }
}

pub static mut DESTROY_RES: Option<Arc<dyn Fn(u32) + Send + Sync>> = None;

#[cfg(target_arch = "wasm32")]
pub struct CanSyncFunction(Function);
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
pub fn set_destroy_callback(f: Function) {
	let f1 = CanSyncFunction(f);
	unsafe {DESTROY_RES = Some(Arc::new(move |value: u32| {
		f1.call(&JsValue::from_f64(0.0), &value.into());
	}))};
}
#[cfg(feature = "pi_js_export")]
#[cfg(not(target_arch = "wasm32"))]
pub fn set_destroy_callback(f: Arc<dyn Fn(u32, Option<Box<dyn FnOnce(Result<u32, String>) + Send + 'static>>) + Send + Sync + 'static>) {
	unsafe { DESTROY_RES = Some(Arc::new(move |value: u32| {
		(f)(value, None);
	}))};
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
/// 资源包装
pub struct ResRef(Handle<JsRes>);

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct ResMgr {
	inner: Share<AssetMgr<JsRes>>,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl ResMgr {
	/// 创建一个资源， 如果资源已经存在，旧的资源将被覆盖
	/// 如果创建的资源类型未注册，将崩溃
	#[cfg(feature = "pi_js_export")]
	pub fn create_res(&mut self, key: u32, cost: u32) -> ResRef {
		match self.inner.insert(key, JsRes {key, cost: cost as usize}) {
			Some(r) => ResRef(r),
			None => self.get_res(key).unwrap()
		}
	}

	/// 获取资源
	#[cfg(feature = "pi_js_export")]
	pub fn get_res(&mut self, key: u32) -> Option<ResRef> {
		match self.inner.get(&key) {
			Some(r) => Some(ResRef(r)),
			None => None
		}
	}
}

/// 资源管理器
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct ResAllocator {
	inner: Share<RefCell<Allocator>>,
}

impl ResAllocator {
	pub fn get_inner(&self) -> &Share<RefCell<Allocator>>{
		&self.inner
	}

	pub fn get_inner_mut(&mut self) -> &mut Share<RefCell<Allocator>>{
		&mut self.inner
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
impl ResAllocator {
	/// 创建资源管理器的实例
	#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
	#[cfg(feature = "pi_js_export")]
	pub fn new(total_capacity: u32) -> Self {
		let r = if total_capacity > 0 {
			Allocator::new(total_capacity as usize)
		} else {
			Allocator::new(16 * 1024 * 1024)
		};
		Self{inner: Share::new(RefCell::new(r))}
	}

	/// 整理方法， 将无人使用的资源放入到LruCache， 清理过时的资源
	/// 就是LruMgr有总内存上限， 按权重分给其下的LRU。 如果有LRU有空闲， 则会减少其max_size, 按权重提高那些满的LRU的max_size
	#[cfg(feature = "pi_js_export")]
	pub fn collect(&mut self, now: u32) {
		self.inner.borrow_mut().collect(now as u64);
	}

	// 10 * 1024 * 1024,
	// 		50 * 1024 * 1024,
	// 		5 * 60000,
	/// 创建一个资源， 如果资源已经存在，则会修改资源的配置
	#[cfg(feature = "pi_js_export")]
	pub fn register_to_resmgr(&mut self, _ty: u32, min_capacity: u32, max_capacity: u32, time_out: u32) -> ResMgr {
		let m = pi_assets::mgr::AssetMgr::<JsRes>::new(pi_assets::asset::GarbageEmpty(),
		false,
		max_capacity as usize,
		time_out as usize,);
		self.inner.borrow_mut().register(m.clone(), min_capacity as usize, max_capacity as usize);
		ResMgr{
			inner: m,
		}
	}
}

/// 资源包装
pub struct JsRes {
	key: u32,
	cost: usize,
}

impl Asset for JsRes {
    type Key = u32;
	/// 资产的大小
	fn size(&self) -> usize {
		self.cost
	}
}

impl std::ops::Drop for JsRes {
	fn drop(&mut self) {
		unsafe { 
			if let Some(r) = &DESTROY_RES {
				(r)(self.key)
			};
		}
    }
}

