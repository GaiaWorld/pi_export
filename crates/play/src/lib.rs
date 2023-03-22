mod json_parse;

pub use json_parse::*; 

// #[macro_export]
// macro_rules! export_with_play {
// 	// 不需要返回值的接口导出
// 	($func_name:ident, $expr:expr, [$($name: ident: $ty: ty,)*]) => {
// 		#[cfg(feature="wasm_bindgen")]
// 		#[wasm_bindgen]
// 		pub fn $func_name($($name: $ty,)*) {
// 			$expr
// 		}

// 		#[cfg(not(target_arch = "wasm32"))]
// 		#[cfg(feature="pi_js_export")]
// 		pub fn $func_name($($context: $context_ty,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
// 			$expr
// 		}
// 	};

// 	// 带返回值的接口导出
// 	(@with_return, $func_name:ident, $return_ty: ty, $expr:expr, $($name: ident: $ty: ty,)*) => {
// 		#[cfg(feature="wasm_bindgen")]
// 		#[wasm_bindgen]
// 		pub fn $func_name($($name: $ty,)*) -> $return_ty {
// 			$expr
// 		}

// 		#[cfg(not(target_arch = "wasm32"))]
// 		#[cfg(feature="pi_js_export")]
// 		pub fn $func_name($($name: $ty,)*) -> $return_ty {
// 			$expr
// 		}
// 	};
// }