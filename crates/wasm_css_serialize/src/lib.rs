

use pi_style::style_parse::ClassMap;
use pi_style::style_parse::parse_class_map_from_string;
use wasm_bindgen::prelude::*;
pub use fragment::serde_fragment_as_bin;

mod fragment;
mod sdf;
pub use sdf::*;

#[derive(Default)]
#[wasm_bindgen]
pub struct ClassMapList(Vec<ClassMap>);

#[wasm_bindgen]
pub fn create_class_map() -> ClassMapList {
	ClassMapList::default()
}


#[wasm_bindgen]
pub fn paser_class_map(value: &str, scope_hash: u32, class_map: &mut ClassMapList) {
	log::debug!("start parse css!, value len: {}", value.len());
    match parse_class_map_from_string(value, scope_hash as usize) {
        Ok(r) => class_map.0.push(r),
        Err(_r) => ()
    }
}

#[wasm_bindgen]
pub fn serialize_class_map_list(class_map: &ClassMapList) -> Vec<u8> {
	log::debug!("start serialize_class_map!, value len: {}", class_map.0.len());
    match postcard::to_stdvec(&class_map.0) {
		Ok(bin) => return bin,
		Err(r) =>{
			log::error!("serialize fail!!, {:?}", r);
			return Vec::<u8>::default()
		},
	};
}


// /**
//  * 在指定上下文中创建一个 文本样式表
//  * __jsObj: class样式的文本描述
//  */
// #[wasm_bindgen]
// pub fn serialize_class_map(value: &str, scope_hash: u32) -> JsValue {
// 	log::warn!("start serialize_class_map!, value len: {}", value.len());
//     let r = match parse_class_map_from_string(value, scope_hash as usize) {
//         Ok(r) => match postcard::serialize(&r) {
//             Ok(bin) => Result{err: None, bin: Some(bin)},
//             Err(r) => Result{err: Some(r.to_string()), bin: None},
//         },
//         Err(r) => Result{err: Some(r), bin: None}
//     };

// 	JsValue::from_serde(&r).unwrap()
// }

#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn deserialize_class_map(bin: &[u8]) {
    match postcard::from_bytes::<Vec<ClassMap>>(bin) {
        Ok(r) => println!(" deserialize_class_map success: {:?}", r),
        Err(e) => {
            log::error!("deserialize_class_map error: {:?}", e);
            return;
        }
    };
    // println!("r: {:?}", r);
}

#[wasm_bindgen]
pub fn init_log(level: pi_web_logger::Level) {
    let _r = pi_web_logger::init_with_level(level);
	log::info!("init_logger ok!");
}

#[test]
fn test() {
   let css =  "
.c1643896642{
    color: #000000;
}
.c967772491{
    text-gradient: linear-gradient(180deg, #FF453D 0%, #FFC017 50%, #FFF1B4 100%);
}";

let mut class_map_list = ClassMapList::default();
paser_class_map(css, 0, &mut class_map_list);
let r = serialize_class_map_list(&class_map_list);
deserialize_class_map(r.as_slice());
println!("map: {:?}", &class_map_list.0);
let mut class_sheet = pi_style::style_type::ClassSheet::default();
for v in class_map_list.0.iter(){
    v.clone().to_class_sheet(&mut class_sheet);
}
println!("class_sheet: {:?}", class_sheet)


}

