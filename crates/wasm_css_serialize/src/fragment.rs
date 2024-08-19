use std::collections::VecDeque;
use std::str::FromStr;
use cssparser::{ParserInput, Parser};
use pi_style::style_parse::parser_style_items;
use pi_hash::XHashMap;
use pi_ui_render::resource::fragment::{NodeFragment, NodeTag, Fragments};
use serde::{Serialize, Deserialize};
use pi_null::Null;
use wasm_bindgen::prelude::wasm_bindgen;
use smallvec::SmallVec;

// NodeFragment的json描述
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFragmentJson {
	pub tag: String,
	pub style: String,
	pub class: SmallVec<[u32; 1]>,
	pub parent: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFragmentWithScope {
	pub value: Vec<NodeFragmentJson>,
	pub scope: u32,
}

#[wasm_bindgen]
pub fn serde_fragment_as_bin(json: &str) -> Vec<u8> {
	let value = serde_json::from_str::<XHashMap<String, NodeFragmentWithScope>>(json).unwrap();
	let mut map = XHashMap::default();
	let mut fragments = Vec::new();
	for (key, nodes) in value.into_iter() {
		println!("key===={:?}", key);
		let k = u32::from_str(key.as_str()).unwrap();
		let index = fragments.len();
		for json in nodes.value.iter() {
			parse_node(json, nodes.scope, &mut fragments);
		}
		// log::warn!("k================={:?}, {:?}, {:?}", k, index, &fragments[index..fragments.len()]);
		map.insert(k, index..fragments.len());
	}

	let t = Fragments {
		fragments,
		map,
	};

	match postcard::to_stdvec(&t) {
		Ok(bin) => return bin.to_vec(),
		Err(r) =>{
			log::error!("serialize fail!!, {:?}", r);
			return Vec::<u8>::default()
		},
	};

}


fn parse_node(node: &NodeFragmentJson, scope_hash: u32, fragments: &mut Vec<NodeFragment>) {
	let tag = match NodeTag::try_from(node.tag.as_str()) {
		Ok(r) => r,
		Err(e) => {
			log::warn!("{:?}", e);
			NodeTag::Div
		},
	};

	let mut input = ParserInput::new(&node.style);
	let mut parse = Parser::new(&mut input);
	let mut style = VecDeque::new();
	parser_style_items(&mut parse, &mut style, scope_hash as usize);

	fragments.push(NodeFragment {
		tag,
		style,
		parent: match node.parent {
			Some(r) => r,
			None => u32::null(),
		},
		class: node.class.clone(),
	});
}

#[test]
fn test() {
	let _json = r#"{
		"c6546848": [{
			"style": "width: 10px;height: 10px;",
			"tag": "div",
			"class": [],
			"parent": null
		},
		{
			"style": "width:10px;",
			"tag": "div",
			"class": [],
			"parent": 0
		}
		]
	}"#;

	let json = r#"
	{"c615477543":[{"tag":"div","style":"width: 100%;height: 100%;flex-wrap: wrap;","parent":null,"class":[1508762967]},{"tag":"div","style":"width: 100%;height: 20px;flex-wrap: wrap;","parent":0,"class":[1508762967]},{"tag":"span","style":"","parent":1,"class":[]},{"tag":"div","style":"","parent":1,"class":[]},{"tag":"span","style":"","parent":3,"class":[]},{"tag":"div","style":"width: 100%;height: 20px;flex-wrap: wrap;","parent":0,"class":[1508762967]},{"tag":"span","style":"","parent":5,"class":[]},{"tag":"div","style":"","parent":5,"class":[]},{"tag":"span","style":"","parent":7,"class":[]}],"1452801992":[{"tag":"div","style":"width: 100%;height: 100%;flex-wrap: wrap;","parent":null,"class":[1865427204]},{"tag":"div","style":"width: 100%;height: 100%;flex-wrap: wrap;","parent":0,"class":[1865427204]},{"tag":"span","style":"","parent":1,"class":[]},{"tag":"div","style":"","parent":1,"class":[]},{"tag":"span","style":"","parent":3,"class":[]},{"tag":"div","style":"width: 100%;height: 30px;flex-wrap: wrap;","parent":0,"class":[1865427204]},{"tag":"span","style":"","parent":5,"class":[]},{"tag":"div","style":"","parent":5,"class":[]},{"tag":"span","style":"","parent":7,"class":[]}]}
	"#;

	let json = r#"
	{
		"1234": {
			"value": [
				{
					"tag":"div",
					"style":"text-overflow: ellipsis;",
					"parent":null,
					"class":[1508762967]
				}
			],
			"scope": 0
		}
	}
	"#;


	let r = serde_fragment_as_bin(json);

	let t = postcard::from_bytes::<Fragments>(r.as_slice());
	println!("r======{:?}, {:?}", r, t)
}