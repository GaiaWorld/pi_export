
use std::mem::transmute;

use pi_hal::font::sdf_brush::{FontCfg, GlyphInfo, MetricsInfo};
use pi_hash::XHashMap;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
	pub err: Option<String>,
	pub bin: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlyphInfo1 {
	unicode: u32,
	glyph: GlyphInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontAtlasData {
    name: String,
    metrics: MetricsInfo,
    glyphs: Vec<GlyphInfo1>,
	// atlas: Vec<CharSdf>,
}

// 序列化sdf配置的json格式的文件
#[wasm_bindgen]
pub fn serialize_sdf_json(s: &str) -> Vec<u8> {
	let info: FontAtlasData = match serde_json::from_str(s) {
		Ok(r) => r,
		Err(e) => {
			log::error!("serialize_sdf_json fail, {:?}", e);
			return Vec::new();
		}
	};
	let mut map = XHashMap::default();
	for item in info.glyphs.into_iter() {
		map.insert(unsafe{transmute::<u32, char>(item.unicode)}, item.glyph);
	}

	// let r = FontAtlasData1 {
	// 	cfg: FontCfg {
	// 		name: info.name,
	// 		metrics: info.metrics,
	// 		glyphs: map,
	// 	},
	// 	atlas: info.atlas,
	// };
	let r = FontCfg {
		name: info.name,
		metrics: info.metrics,
		glyphs: map,
	};

	match postcard::to_stdvec(&r) {
        Ok(r) => r,
        Err(e) => {
            log::error!("serialize_sdf_json error: {:?}", e);
            return Vec::new();
        }
    }
}