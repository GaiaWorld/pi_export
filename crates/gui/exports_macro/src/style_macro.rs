//! 将设置布局属性的接口导出到js


use std::mem::transmute;

// use crate::components::user::{
//     BorderImageSlice, BorderRadius, CgColor, ClassName, Color, ColorAndPosition, FontSize, Hsi, ImageRepeat, LengthUnit, LineHeight,
//     LinearGradientColor, MaskImage, Stroke, TextContent, TransformFunc, TransformOrigin,
// };
use pi_ui_render::components::calc::EntityKey;
use pi_ui_render::resource::ComponentCmd;
use pi_null::Null;
use pi_ui_render::components::user::ClassName;
use bevy_ecs::prelude::Entity;
use ordered_float::NotNan;
use pi_flex_layout::prelude::*;
use pi_style::style::*;
use pi_style::style_type::*;
use pi_ui_render::resource::NodeCmd;
use pi_ui_render::components::user::RadialWave;
use pi_style::style_parse::{parse_comma_separated, parse_text_shadow, parse_as_image, StyleParse};
use smallvec::SmallVec;
pub use pi_export_base::export::{Atom, Engine};
pub use crate::index::{OffsetDocument, Size, Gui};
use pi_ui_render::resource::animation_sheet::KeyFramesSheet;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
pub enum Edge {
    Left = 0,
    Top = 1,
    Right = 2,
    Bottom = 3,
    Start = 4,
    End = 5,
    Horizontal = 6,
    Vertical = 7,
    All = 8,
}

#[macro_export]
macro_rules! other_out_export {
	($func_name:ident, $context: ident, $node: ident, $expr:expr, $($mut_name_ref: ident: &mut $mut_ty_ref: ty,)*; $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="pi_js_export")]
		
		pub fn $func_name($context: &mut Gui, $node: f64, $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $node: f64, $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}
	};

	($func_name:ident, [$($context: ident: $context_ty: ty,)*], $expr:expr, $($mut_name_ref: ident: &mut $mut_ty_ref: ty,)*; $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="pi_js_export")]
		
		pub fn $func_name($($context: $context_ty,)* $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($($context: $context_ty,)* $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}
	};

	($func_name:ident, $context: ident, $expr:expr, $($mut_name_ref: ident: &mut $mut_ty_ref: ty,)*; $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="pi_js_export")]
		
		pub fn $func_name($context: &mut Gui, $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($context: &mut Gui, $($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
			$expr
		}

	};

	// ($func_name:ident, $context: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
	// 	#[cfg(feature="pi_js_export")]
		
	// 	pub fn $func_name($context: &mut Gui, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
	// 		$expr
	// 	}

	// 	#[cfg(target_arch="wasm32")]
	// 	#[wasm_bindgen]
	// 	pub fn $func_name($context: &mut Gui, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
	// 		$expr
	// 	}
	// };

	// 带返回值的接口
	(@with_return_node, $func_name:ident, $($context: ident: $context_ty: ty,)*; $node: ident, $return_ty: ty, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="pi_js_export")]
		
		pub fn $func_name($($context: $context_ty,)* $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($($context: $context_ty,)* $node: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			let $node = unsafe {Entity::from_bits(transmute::<f64, u64>($node))};
			$expr
		}
	};

	// 带返回值的接口
	(@with_return, $func_name:ident, $return_ty: ty, $expr:expr, $($mut_name_ref: ident: &mut $mut_ty_ref: ty,)*; $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		#[cfg(feature="pi_js_export")]
		
		pub fn $func_name($($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($($mut_name_ref: &mut $mut_ty_ref,)* $($name_ref: &$ty_ref,)* $($name: $ty,)*) -> $return_ty {
			$expr
		}
	};

	(@with_return1, $func_name:ident($($context: ident: $context_ty: ty,)*)($($node: ident,)*)($($other_param: ident: $other_ty: ty,)*)->$return_ty: ty {$expr:expr}) => {
		#[cfg(feature="pi_js_export")]
		pub fn $func_name($($context: $context_ty,)* $($node: &$ident: f64,)* $($other_param: $other_ty,)*) -> $return_ty {
			$expr
		}

		#[cfg(target_arch="wasm32")]
		#[wasm_bindgen]
		pub fn $func_name($($context: $context_ty,)* $($node: &$ident: f64,)* $($other_param: $other_ty,)*) -> $return_ty {
			$expr
		}
	};
}

#[macro_export]
macro_rules! style_out_export {
	(@dimension_box $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@dimension_inner  [<$attr_name _percent>], $last_ty, Dimension::Percent(v),; v: f32, );
			style_out_export!(@dimension_inner $attr_name, $last_ty, Dimension::Points(v),; v: f32, );
			style_out_export!(@dimension_inner  [<$attr_name _auto>], $last_ty, Dimension::Auto,; );
		}
	};

	(@dimension $attr_name:ident, $last_ty: ident) => {
		$crate::paste::item! {
			style_out_export!(@expr  [<$attr_name _percent>], $last_ty, Dimension::Percent(v),; v: f32, );
			style_out_export!(@expr $attr_name, $last_ty, Dimension::Points(v),; v: f32, );
			style_out_export!(@expr  [<$attr_name _auto>], $last_ty, Dimension::Auto,; );
		}
	};

	(@cenum $attr_name:ident, $last_ty: ident) => {
		style_out_export!(@expr $attr_name, $last_ty, unsafe {transmute(v as u8)},; v: f64,);
	};

	(@expr $attr_name:ident, $last_ty: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

		}
    };

	(@owner $attr_name:ident, $expr:expr, $resetexpr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				$expr;
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				$expr;
			}

			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				$resetexpr;
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				$resetexpr;
			}
		}
	};

	(@dimension_inner $attr_name:ident, $last_ty: ident, $expr: expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.commands.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.commands.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.commands.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.commands.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<$last_ty Type>]($last_ty(Rect {
					// 	top: $expr,
					// 	right: $expr,
					// 	bottom: $expr,
					// 	left: $expr,
					// }))),
					Edge::Top => gui.commands.set_style(node_id, [<$last_ty TopType>]($expr)),
					Edge::Right => gui.commands.set_style(node_id, [<$last_ty RightType>]($expr)),
					Edge::Bottom => gui.commands.set_style(node_id, [<$last_ty BottomType>]($expr)),
					Edge::Left => gui.commands.set_style(node_id, [<$last_ty LeftType>]($expr)),
					_ => return
				};
			}

			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.commands.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.commands.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.commands.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.commands.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64, edge: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				match unsafe {transmute(edge as u8)} {
					// Edge::All => gui.commands.set_style(node_id, [<Reset $last_ty Type>]),
					Edge::Top => gui.commands.set_style(node_id, [<Reset $last_ty TopType>]),
					Edge::Right => gui.commands.set_style(node_id, [<Reset $last_ty RightType>]),
					Edge::Bottom => gui.commands.set_style(node_id, [<Reset $last_ty BottomType>]),
					Edge::Left => gui.commands.set_style(node_id, [<Reset $last_ty LeftType>]),
					_ => return
				};
			}

		}
	};

	(@atom $attr_name:ident, $last_ty: ident, $expr:expr, $($name_ref: ident: &$ty_ref: ty,)*; $($name: ident: $ty: ty,)*) => {
		$crate::paste::item! {
			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<set_ $attr_name>](gui: &mut Gui, node_id: f64, $($name_ref: &$ty_ref,)* $($name: $ty,)*) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, $last_ty($expr));
			}

			#[cfg(feature="pi_js_export")]
			
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}

			#[cfg(target_arch="wasm32")]
			#[wasm_bindgen]
			#[allow(unused_attributes)]
			pub fn [<reset_ $attr_name>](gui: &mut Gui, node_id: f64) {
				let node_id = unsafe {Entity::from_bits(transmute::<f64, u64>(node_id))};
				gui.commands.set_style(node_id, [<Reset $last_ty>]);
			}
		}
    };
}

style_out_export!(@cenum align_content, AlignContentType);
style_out_export!(@cenum align_items, AlignItemsType);
style_out_export!(@cenum justify_content, JustifyContentType);
style_out_export!(@cenum flex_direction, FlexDirectionType);
style_out_export!(@cenum flex_wrap, FlexWrapType);
style_out_export!(@cenum align_self, AlignSelfType);
style_out_export!(@cenum position_type, PositionTypeType);

style_out_export!(@expr flex_grow, FlexGrowType, v, ; v: f32,);
style_out_export!(@expr flex_shrink, FlexGrowType, v, ; v: f32,);

style_out_export!(@dimension flex_basis, FlexBasisType);
style_out_export!(@dimension width, WidthType);
style_out_export!(@dimension height, HeightType);
style_out_export!(@dimension min_width, MinWidthType);
style_out_export!(@dimension min_height, MinHeightType);
style_out_export!(@dimension max_width, MaxWidthType);
style_out_export!(@dimension max_height, MaxHeightType);

style_out_export!(@dimension_box padding, Padding);
style_out_export!(@dimension_box margin, Margin);
style_out_export!(@dimension_box border, Border);
style_out_export!(@dimension_box position, Position);

style_out_export!(@expr background_rgba_color, BackgroundColorType, Color::RGBA(CgColor::new(r, g, b, a)), ; r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	background_linear_color, 
	BackgroundColorType, 
	Color::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )), ;
	direction: f32, color_and_positions: Vec<f32>,);

style_out_export!(@expr 
	border_color,
	BorderColorType,
	CgColor::new(r, g, b, a),;
	r: f32, g: f32, b: f32, a: f32,);

style_out_export!(@expr
	border_radius,
	BorderRadiusType,
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let border_radius = pi_style::style_parse::parse_border_radius(&mut parse);
		if let Ok(value) = border_radius {
			value
		} else {
			Default::default()
		}
	},
	s: &str,; );

style_out_export!(@expr 
	box_shadow,
	BoxShadowType,
	BoxShadow {
		h: h,
		v: v,
		blur: blur,
		spread: spread,
		color: CgColor::new(r, g, b, a)
	},;
	h: f32, v: f32, blur: f32, spread: f32, r: f32, g: f32 ,b: f32, a: f32,);
style_out_export!(@cenum object_fit, ObjectFitType);

style_out_export!(@expr background_repeat, BackgroundRepeatType, ImageRepeat {
		x: unsafe { transmute(x as u8) },
		y: unsafe { transmute(y as u8) },
	},;
	x: u8, y: u8, );

style_out_export!(@expr 
	mask_image_linear,
	MaskImageType,
	MaskImage::LinearGradient(to_linear_gradient_color(
        color_and_positions.as_slice(),
        direction,
    )),;
	direction: f32, color_and_positions: Vec<f32>, );

style_out_export!(@expr 
	image_clip,
	BackgroundImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	mask_image_clip,
	MaskImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	border_image_clip,
	BorderImageClipType,
	NotNanRect::new(
		unsafe { NotNan::new_unchecked(v1) },
		unsafe { NotNan::new_unchecked(u2) },
		unsafe { NotNan::new_unchecked(v2) },
		unsafe { NotNan::new_unchecked(u1) },
	),;
	u1: f32, v1: f32, u2: f32, v2: f32,);

style_out_export!(@expr 
	border_image_slice,
	BorderImageSliceType,
	BorderImageSlice {
		top: unsafe { NotNan::new_unchecked(top) },
		right: unsafe { NotNan::new_unchecked(right) },
		bottom: unsafe { NotNan::new_unchecked(bottom) },
		left: unsafe { NotNan::new_unchecked(left) },
		fill,
	},;
	top: f32, right: f32, bottom: f32, left: f32, fill: bool,);

style_out_export!(@expr 
	border_image_repeat,
	BorderImageRepeatType,
	ImageRepeat {
		x: unsafe { transmute(vertical as u8) },
		y: unsafe { transmute(horizontal as u8) },
	},;
	vertical: u8, horizontal: u8, );

style_out_export!(@expr overflow, OverflowType, v,; v: bool,);
style_out_export!(@expr opacity, OpacityType, v,; v: f32,);
style_out_export!(@cenum display, DisplayType);
style_out_export!(@expr visibility, VisibilityType, v,; v: bool,);
style_out_export!(@cenum enable, EnableType);
style_out_export!(@cenum blend_mode, BlendModeType);
style_out_export!(@expr zindex, ZIndexType, v as isize,; v: i32,);
style_out_export!(@expr as_image, AsImageType, {
	let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);

    match parse_as_image(&mut parse) {
        Ok(r) => r,
        Err(e) => {
            log::error!("set_as_image fail, str: {}, err: {:?}", value, e);
            return;
        }
    }
}, value: &str,;);

// style_out_export!(@expr as_image, AsImageType, unsafe{transmute(v)},; v: u8,);
style_out_export!(@expr filter_blur, BlurType, v,; v: f32,);
style_out_export!(@expr transform_will_change, TransformWillChangeType, v,; v: bool,);

// hsi, 效果与ps一致,  h: -180 ~ 180, s: -100 ~ 100, i: -100 ~ 100
style_out_export!(@expr 
	filter_hsi,
	HsiType,
	{
		let (mut h, mut s, mut _i) = (h, s, _i);
		if h > 180.0 {
			h = 180.0;
		} else if h < -180.0 {
			h = -180.0
		}
		if s > 100.0 {
			s = 100.0;
		} else if s < -100.0 {
			s = -100.0
		}
		if _i > 100.0 {
			_i = 100.0;
		} else if _i < -100.0 {
			_i = -100.0
		}
		Hsi {
			hue_rotate: h / 360.0,
			saturate: s / 100.0,
			bright_ness: _i / 100.0,
		}
	},;
	h: f32, s: f32, _i: f32, );
/************************************ Transform **************************************/
style_out_export!(@expr 
	translate, 
	TranslateType, 
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let translate = pi_style::style_parse::parse_mult(&mut parse, [LengthUnit::default(), LengthUnit::default()], pi_style::style_parse::parse_len_or_percent);
		if let Ok(translate) = translate {
			translate
		} else {
			Default::default()
		}
	},
	s: &str,; );
style_out_export!(@expr 
	scale, 
	ScaleType, 
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let scale = pi_style::style_parse::parse_mult(&mut parse, [1.0f32, 1.0f32], pi_style::style_parse::parse_number);
		if let Ok(scale) = scale {
			scale
		} else {
			Default::default()
		}

	},
	s: &str,; );
style_out_export!(@expr 
	rotate, 
	RotateType, 
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let rotate = pi_style::style_parse::parse_angle(&mut parse);
		if let Ok(rotate) = rotate {
			rotate
		} else {
			Default::default()
		}

	},
	s: &str,; );

style_out_export!(@expr 
	transform, 
	TransformType, 
	{
		let mut input = cssparser::ParserInput::new(s);
		let mut parse = cssparser::Parser::new(&mut input);

		let transform = pi_style::style_parse::parse_transform(&mut parse);
		if let Ok(transform) = transform {
			transform
		} else {
			Default::default()
		}
	},
	s: &str,; );

style_out_export!(@expr 
	transform_origin, 
	TransformOriginType, 
	{
		let x_ty = unsafe { transmute(x_ty as u8) };
		let y_ty = unsafe { transmute(y_ty as u8) };
		let x_value = match x_ty {
			LengthUnitType::Pixel => LengthUnit::Pixel(x),
			LengthUnitType::Percent => LengthUnit::Percent(x),
		};
		let y_value = match y_ty {
			LengthUnitType::Pixel => LengthUnit::Pixel(y),
			LengthUnitType::Percent => LengthUnit::Percent(y),
		};
		TransformOrigin::XY(x_value, y_value)
	},;
	x_ty: f64, x: f32, y_ty: f64, y: f32,);

// 设置transform为None TODO

style_out_export!(@expr letter_spacing, LetterSpacingType, v,; v: f32,);
style_out_export!(@expr word_spacing, WordSpacingType, v,; v: f32,);

style_out_export!(@expr text_rgba_color, ColorType, Color::RGBA(CgColor::new(r, g, b, a)),; r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@expr 
	text_linear_gradient_color, 
	ColorType, 
	Color::LinearGradient(to_linear_gradient_color(
		color_and_positions.as_slice(),
		direction,
	)),; direction: f32, color_and_positions: Vec<f32>, );
style_out_export!(@expr line_height_normal, LineHeightType, LineHeight::Normal,;);
style_out_export!(@expr line_height, LineHeightType, LineHeight::Length(value),; value: f32,);
style_out_export!(@expr line_height_percent, LineHeightType, LineHeight::Percent(value), ;value: f32,);
style_out_export!(@expr text_indent, TextIndentType, v,; v: f32,);
// style_out_export!(@cenum text_align, TextAlignType);
style_out_export!(
	@owner 
	text_align, 
	{	
		let v: TextAlign = unsafe {transmute(v as u8)};
		gui.commands.set_style(node_id, TextAlignType(v));
		gui.commands.set_style(node_id, JustifyContentType(match v {
			TextAlign::Left => JustifyContent::FlexStart,
			TextAlign::Right => JustifyContent::FlexEnd,
			TextAlign::Center => JustifyContent::Center,
			TextAlign::Justify => JustifyContent::SpaceBetween,
		}));
	},
	{
		gui.commands.set_style(node_id, ResetTextAlignType);
		gui.commands.set_style(node_id, ResetJustifyContentType);
	},;
	v: f64,
);

style_out_export!(
	@owner 
	vertical_align, 
	{	
		let v: VerticalAlign = unsafe {transmute(v as u8)};
		gui.commands.set_style(node_id, VerticalAlignType(v));
		gui.commands.set_style(node_id, AlignSelfType(match v {
			VerticalAlign::Top => AlignSelf::FlexStart,
			VerticalAlign::Bottom => AlignSelf::FlexEnd,
			VerticalAlign::Middle => AlignSelf::Center,
		}));
	},
	{
		gui.commands.set_style(node_id, ResetVerticalAlignType);
		gui.commands.set_style(node_id, ResetAlignSelfType);
	},;
	v: f64,
);

style_out_export!(@expr $attr_name, $last_ty, unsafe {transmute(v as u8)},; v: f64,);

style_out_export!(@expr text_stroke, TextStrokeType, Stroke {
	width: NotNan::new(width).expect("stroke width is nan"),
	color: CgColor::new(r, g, b, a),
},; width: f32, r: f32, g: f32, b: f32, a: f32,);
style_out_export!(@cenum white_space, WhiteSpaceType);
style_out_export!(@cenum font_style, FontStyleType);
style_out_export!(@expr font_weight, FontWeightType, v as usize,; v: f64,);
style_out_export!(@expr font_size_none, FontSizeType, FontSize::None,;);
style_out_export!(@expr font_size, FontSizeType, FontSize::Length(value as usize),; value: f64,);
style_out_export!(@expr font_size_percent, FontSizeType, FontSize::Percent(value),; value: f32,);
style_out_export!(@expr text_content_utf8, TextContentType, {
	let content = unsafe{String::from_utf8_unchecked(content)};
	TextContent(content, pi_atom::Atom::from(""))
},; content: Vec<u8>,);
style_out_export!(@expr clip_path_str, ClipPathType, {
	let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);

    match BaseShape::parse(&mut parse) {
        Ok(r) => r,
        Err(e) => {
            log::error!("set_animation_str fail, animation: {}, err: {:?}", value, e);
            return;
        }
    }
}, value: &str,;);



// animation
style_out_export!(@expr animation_duration, AnimationDurationType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) },; name: Vec<usize>,);
style_out_export!(@expr animation_delay, AnimationDelayType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>()) },; name: Vec<usize>,);
style_out_export!(@expr animation_iteration_count, AnimationIterationCountType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>()) },; name: Vec<f32>,);
style_out_export!(@expr animation_direction, AnimationDirectionType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_fill_mode, AnimationFillModeType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_play_state, AnimationPlayStateType, unsafe{ transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>()) },; name: Vec<u8>,);
style_out_export!(@expr animation_name_str, AnimationNameType, {
	let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);
    let value = if let Ok(value) =
        parse_comma_separated::<_, _, cssparser::ParseError<pi_style::style_parse::TokenParseError>>(&mut parse, |input| Ok(pi_atom::Atom::from(input.expect_ident()?.as_ref())))
    {
        value
    } else {
        Default::default()
    };
	AnimationName {
		scope_hash: scope_hash as usize,
		value,
	}
},value: &str,; scope_hash: u32,);

style_out_export!(
	@owner 
	runtime_animation, 
	gui.commands.add_runtime_animation(node_id, animation, key_frames, scope_hash as usize),
	{},;
	animation: &str,
	key_frames: &str,
	scope_hash: u32,
);

#[cfg(feature="pi_js_export")]
pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    set_animation_str_inner(gui, node_id, value, scope_hash);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    set_animation_str_inner(gui, node_id, value, scope_hash);
}

#[cfg(feature="pi_js_export")]
pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
    reset_animation_str_inner(gui, node_id);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
    reset_animation_str_inner(gui, node_id);
}



// #[cfg(feature="pi_js_export")]
// pub fn set_animation_name(_gui: &mut Gui, _scope_hash: u32, _name: &Vec<String>) {

// }
// pub enum AnimationTimingFunction {
//     /// 匀速
//     Linear,
//     /// 淡入淡出
//     Ease(EEasingMode),
//     /// 跳跃
//     Step(usize, EStepMode),
//     /// 贝塞尔曲线
//     CubicBezier(f32, f32, f32, f32),
// }

// impl_style!(
//     AnimationTimingFunctionType,
//     animation,
//     timing_function,
//     AnimationTimingFunction,
//     SmallVec<[AnimationTimingFunction; 1]>
// );
// impl_style!(AnimationDelayType, animation, delay, AnimationDelay, SmallVec<[Time; 1]>);
// impl_style!(
//     AnimationIterationCountType,
//     animation,
//     iteration_count,
//     AnimationIterationCount,
//     SmallVec<[IterationCount; 1]>
// );
// impl_style!(
//     AnimationDirectionType,
//     animation,
//     direction,
//     AnimationDirection,
//     SmallVec<[AnimationDirection; 1]>
// );
// impl_style!(
//     AnimationFillModeType,
//     animation,
//     fill_mode,
//     AnimationFillMode,
//     SmallVec<[AnimationFillMode; 1]>
// );
// impl_style!(
//     AnimationPlayStateType,
//     animation,
//     play_state,
//     AnimationPlayState,
//     SmallVec<[AnimationPlayState; 1]>
// );

style_out_export!(@atom 
	mask_image,
	MaskImageType,
	MaskImage::Path((**image_hash).clone()),
	image_hash: &Atom,; );

style_out_export!(@atom 
	background_image,
	BackgroundImageType,
	(**image_hash).clone(),
	image_hash: &Atom,; );
style_out_export!(@atom 
	border_image,
	BorderImageType,
	(**image_hash).clone(),
	image_hash: &Atom,; );
style_out_export!(@expr text_shadow, TextShadowType, {
	let mut input = cssparser::ParserInput::new(s);
	let mut parse = cssparser::Parser::new(&mut input);

	let shadows = parse_text_shadow(&mut parse);
	if let Ok(value) = shadows {
		value
	} else {
		Default::default()
	}
}, s: &str,;);
style_out_export!(@atom font_family, FontFamilyType, (**name).clone(), name: &Atom,;);
style_out_export!(@expr text_content, TextContentType,  TextContent(content, pi_atom::Atom::from("")), ;content: String,);
style_out_export!(@expr animation_timing_function_str, AnimationTimingFunctionType, { 
	let mut input = cssparser::ParserInput::new(value);
	let mut parse = cssparser::Parser::new(&mut input);

	if let Ok(value) = parse_comma_separated(&mut parse, <AnimationTimingFunction as StyleParse>::parse) {
		value
	} else {
		Default::default()
	}
}, value: &str,;);

other_out_export!(set_default_style, gui, {gui.commands.set_default_style_by_str(value, 0);},; value: &str,;);

other_out_export!(
    create_class_by_str,
    gui,
    {
        gui.commands.add_css(css, scope_hash as usize);
    },;
	css: &str,
	;
    scope_hash: u32,
);

other_out_export!(
    set_class,
    gui,
    node,
    {
        let mut s = SmallVec::with_capacity(class_name.len());
        for i in class_name.iter() {
            s.push(*i as usize);
        }
        gui.commands.set_class(node, ClassName(s));
    },;;
    class_name: Vec<u32>,
);

// /// 创建容器节点， 容器节点可设置背景颜色
// #[cfg(not(target_arch = "wasm32"))]
// #[cfg(feature = "pi_js_export")]
// pub fn create_node(gui: &mut Gui) -> f64 {
//     use crate::components::NodeBundle;

//     let entity = gui.entitys.reserve_entity();
//     gui.commands.push_cmd(NodeCmd(NodeBundle::default(), entity));
//     // log::warn!("entity :{:?}", entity);
//     unsafe { transmute(entity.to_bits()) }
// }

// // 创建模板
// other_out_export!(
// 	@with_return,
//     create_fragment,
// 	u32,
// 	{
// 		let mut arr = Float64ArrayMut(arr);
// 		let mut index = 0;
// 		let mut entitys = Vec::with_capacity(count as usize);
// 		while index < count {
// 			let entity = gui.entitys.reserve_entity();
// 			arr.set_index(index, unsafe { transmute(entity.to_bits()) });
// 			entitys.push(entity);
// 			index = index + 1;
// 		}
// 		gui.commands
// 			.fragment_commands.push(FragmentCommand {
// 				key,
// 				entitys
// 			});
// 		0
// 	},
// 	gui: &mut Gui,;
// 	;
// 	arr: Float64Array,
// 	count: u32,
// 	key: u32,
// );

other_out_export!(
    set_view_port,
    gui,
    {
		let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
		gui.commands.set_view_port(root, pi_ui_render::components::user::Viewport(Aabb2::new(Point2::new(x as f32, y as f32), Point2::new(width as f32, height as f32))));
	},;;
	x: i32, y: i32, width: i32, height: i32, root: f64,
);

// 设置画笔（绘canvas的图节点所在的实体）
other_out_export!(
    set_brush,
    gui,
    {
		let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
		let brush = unsafe { Entity::from_bits(transmute::<f64, u64>(brush)) };
		gui.commands.push_cmd(ComponentCmd(
			pi_ui_render::components::user::Canvas(brush),
			node,
		));
	},;;
	node: f64, brush: f64,
);

/// 设置水波纹效果
other_out_export!(
	set_radial_wave,
	gui,
	{
		let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
		gui.commands.push_cmd(NodeCmd(RadialWave(pi_postprocess::prelude::RadialWave {
			aspect_ratio,
			start,
			end,
			center_x,
			center_y,
			cycle,
			weight
		}), node));
	},;;
	node: f64,
	aspect_ratio: bool,
	start: f32,
	end: f32,
	center_x: f32,
	center_y: f32,
	cycle: u8,
	weight: f32,);

// 设置
other_out_export!(
    set_rendertarget_type,
    gui,
    {
		let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
		gui.commands.set_target_type(node, unsafe { transmute::<_, pi_ui_render::components::user::RenderTargetType>(target_ty) });
	},;;
	node: f64, target_ty: u8,
);

other_out_export!(
    set_clear_color,
    gui,
    {
		gui.commands.set_clear_color(CgColor::new(r, g, b, a));
	},;;
	r: f32, g: f32, b: f32, a: f32,
);


other_out_export!(
    create_class_by_bin,
    gui,
    match postcard::from_bytes::<Vec<pi_style::style_parse::ClassMap>>(bin) {
		Ok(r) => {
			gui.commands.add_css_bin(pi_ui_render::resource::ExtendCssCmd(r));
		}
		Err(e) => {
			log::warn!("deserialize_class_map error: {:?}, {:?}", e, bin);
			return;
		}
	},;
	bin: &[u8],;
	
);


// 调试使用， 设置渲染脏， 使渲染系统在下一帧进行渲染
other_out_export!(
    set_render_dirty,
    gui,
    {
		let node: Entity = Entity::from_bits(unsafe { transmute(root) });
    	gui.commands.set_render_dirty(node, pi_ui_render::components::user::RenderDirty(true));
	},;;
	root: f64,
);

// TODO
other_out_export!(
    bind_render_target,
    _gui,
    {},;;
);

// TODO
other_out_export!(
    set_pixel_ratio,
    _gui,
    {
		
	},;;
	_pixel_ratio: f32,
);

// TODO
other_out_export!(
    nullify_clear_color,
    _gui,
    {
		
	},;;
);

// TODO
other_out_export!(
    set_scissor,
    _gui,
    {
		
	},;;
	_x: i32, _y: i32, _width: i32, _height: i32,
);

// TODO
other_out_export!(
    set_project_transfrom,
    _gui,
    {
		
	},;;
	_scale_x: f32, _scale_y: f32, _translate_x: f32, _translate_y: f32, _rotate: f32, _width: f64, _height: f64,
);

// TODO
other_out_export!(
    force_update_text,
    _gui,
	_node,
    {
		
	},;;
);

// TODO
//设置shader
other_out_export!(
    set_shader,
    _gui,
    {
	
	},;
	_shader_name: &str, _shader_code: &str,
	;
	
);

// TODO
other_out_export!(
	@with_return, 
    get_font_sheet,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_class_sheet,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    create_render_target,
	u32,
    0,
	_gui: &mut Gui,;
	;
	_fbo: f64,
);

// TODO
other_out_export!(
	@with_return, 
    destroy_render_target,
	u32,
    0,
	_gui: &mut Gui,;
	;
	_fbo: f64,
);

// TODO
other_out_export!(
	@with_return, 
    clone_engine,
	Gui,
    todo!(),
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_text_texture_width,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    get_text_texture_height,
	u32,
    0,
	_gui: &mut Gui,;
	;
);

// TODO
other_out_export!(
	@with_return, 
    node_is_exist,
	bool,
    true,
	_gui: &mut Gui,;
	;
	_node: f64,
);

// TODO
other_out_export!(
	@with_return, 
    node_is_visibility,
	bool,
    true,
	_gui: &mut Gui,;
	_engine: &Engine,;
	_node: f64,
);

type ReturnNode = Option<f64>;

// TODO
other_out_export!(
	@with_return_node, 
    first_child,
    _gui: &Gui,;
	_parent,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    last_child,
    _gui: &Gui,;
	_parent,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    next_sibling,
    _gui: &Gui,;
	_node,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    previous_sibling,
    _gui: &Gui,;
	_node,
	ReturnNode,
    None,;
);

// TODO
other_out_export!(
	@with_return_node, 
    get_layer,
    _gui: &Gui,;
	_node,
	u32,
    0,;
);
other_out_export!(
	@with_return_node, 
    get_enable,
    gui: &mut Gui,
	engine: &mut Engine,;
	node,
	bool,
	{
		pi_export_base::export::await_last_frame(engine);
		if let Ok(is_show) = gui.enable_query.get(&engine.world, node) {
			is_show.get_enable()
		} else {
			false
		}
	}
   ,;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点到gui的上边界的距离
other_out_export!(
	@with_return_node, 
    offset_top,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	u32,
    {
		let mut r = 0.0;
		if let Ok(parent) = gui.up_query.get(&engine.world, node) {
			if let Ok(parent_layout) = gui.layout_query.get(&engine.world, node) {
				r += parent_layout.padding.top + parent_layout.border.top;
			}
		}
		if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
			r += layout.rect.top;
		}
		r.round() as u32
	},;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点到gui的左边界的距离
other_out_export!(
	@with_return_node, 
    offset_left,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	u32,
    {
		let mut r = 0.0;
		if let Ok(parent) = gui.up_query.get(&engine.world, node) {
			if let Ok(parent_layout) = gui.layout_query.get(&engine.world, node) {
				r += parent_layout.padding.left + parent_layout.border.left;
			}
		}
		if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
			r += layout.rect.left;
		}
		r.round() as u32
	},;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点的布局宽度
other_out_export!(
	@with_return_node, 
    offset_width,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	u32,
    {
		let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
			layout.rect.right - layout.rect.left
		} else {
			0.0
		}
		r.round() as u32
	},;
);

// 返回值原类型为f32,这里之所以返回u32，是因为在iphonex以上的机型的浏览器上多次连续调用返回值为浮点数时，浏览器会自动刷新或白屏，原因未知
// 节点布局高度
other_out_export!(
	@with_return_node, 
    offset_height,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	u32,
    {
		let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
			layout.rect.bottom - layout.rect.top
		} else {
			0.0
		}
		r.round() as u32
	},;
);

/// 等同于html的getBoundingClientRect TODO
/// left top width height
other_out_export!(
	@with_return_node, 
    get_class_name,
    _gui: &mut Gui,
	engine: &mut Engine,;
	node,
	String,
    {
		pi_export_base::export::await_last_frame(engine);
		let value = match engine.world.get::<ClassName>( node){
			Some(r) => Some(&r.0),
			_ => None,
		};
		serde_json::to_string(&value).unwrap()
	},;
);


/// 等同于html的getBoundingClientRect TODO
/// left top width height
other_out_export!(
	@with_return_node, 
    offset_document,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	String,
    {
		let value = match gui.quad_query.get(&engine.world, node) {
			Ok(quad) => OffsetDocument {
				left: quad.mins.x,
				top: quad.mins.y,
				width: quad.maxs.x - quad.mins.x,
				height: quad.maxs.y - quad.mins.y,
			},
			_ => OffsetDocument {
				left: 0.0,
				top: 0.0,
				width: 0.0,
				height: 0.0,
			},
		};
		serde_json::to_string(&value).unwrap()
	},;
);

other_out_export!(
	@with_return_node, 
    content_box,
    gui: &mut Gui,
	engine: &Engine,;
	node,
	String,
    {
		let mut cur_child = match gui.down_query.get(&engine.world, node) {
			Ok(down) => down.head(),
			_ => return serde_json::to_string(&Size { width: 0.0, height: 0.0 }).unwrap(),
		};
	
		let (mut left, mut right, mut top, mut bottom) = (std::f32::MAX, 0.0, std::f32::MAX, 0.0);
		while !EntityKey(cur_child).is_null() {
			let l = match gui.layout_query.get(&engine.world, cur_child) {
				Ok(r) => r,
				_ => break,
			};
			let r = l.rect.right;
			let b = l.rect.bottom;
			if l.rect.left < left {
				left = l.rect.left;
			}
			if r > right {
				right = r;
			}
			if b > bottom {
				bottom = b;
			}
			if l.rect.top < top {
				top = l.rect.top;
			}
	
			cur_child = match gui.up_query.get(&engine.world, cur_child) {
				Ok(r) => r.next(),
				_ => break,
			};
		}
		serde_json::to_string(&Size {
			width: right - left,
			height: bottom - top,
		}).unwrap()
	},;
);

// 取出动画事件的buffer长度
other_out_export!(
	@with_return, 
    get_animation_events_max_len,
	u32,
    {
		pi_export_base::export::await_last_frame(engine);
		let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
		let events = key_frames.get_animation_events();

		return (events.len() * 5) as u32;
	},engine: &mut Engine,;;
);

// // 取出动画事件的buffer长度
// other_out_export!(
// 	@with_return, 
//     get_animation_events_max_len,
// 	u32,
//     {
// 		pi_export_base::export::await_last_frame(engine);
// 		let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
// 		let events = key_frames.get_animation_events();

// 		return (events.len() * 5) as u32;
// 	},;engine: &mut Engine,;
// );

// 填充动画事件的buffer， 并返回buffer长度
// 注意， 先调用get_animation_events_max_len获得事件buffer的长度， 将传入的buffer设置为该长度， 否则该函数可能panic
other_out_export!(
	@with_return, 
    get_animation_events,
	u32,
    {
		let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
		// log::warn!("get_animation_events=======");

		let events = key_frames.get_animation_events();
		let map = key_frames.get_group_bind();
		
		let mut i = 0;
		for (group_id, ty, count) in events.iter() {
			// if let pi_animation::animation_listener::EAnimationEvent::End = *ty {
			// 	log::warn!("end=========={:?}", group_id);
			// }
			match map.get(*group_id) {
				Some(r) => {
					log::trace!(target: format!("animationevent_{}", &r.1.1.as_str()).as_str(), "ty: {:?}", ty);
					arr[i] = r.0.index(); // entity
					arr[i + 1] = r.0.generation();
					arr[i + 2] = r.1.1.get_hash() as u32; // name hash
				},
				None => continue,
			};
			arr[i + 3] = unsafe {transmute::<_, u8>(*ty)}  as u32; // event type
			arr[i + 4] = *count;  // cur iter count
			i += 5;
		}
		i as u32
	},arr: &mut [u32],;engine: &Engine,;
);

other_out_export!(
	@with_return, 
    get_entity_offset,
	u32,
    {
		let r = unsafe {Entity::from_bits(transmute::<f64, u64>(value))};
    	r.index()
	},;;value: f64,
);

other_out_export!(
	@with_return, 
    get_entity_version,
	u32,
    {
		let r = unsafe {Entity::from_bits(transmute::<f64, u64>(value))};
    	r.generation()
	},;;value: f64,
);

// other_out_export!(
// 	@with_return1,
//     query(engine: &mut Engine, gui: &mut Gui,)()(x: f32, y: f32,)-> Option<f64> {
// 		pi_export_base::export::await_last_frame(engine);
// 		crate::index::query(engine, gui, x, y)
// 	}
// );

other_out_export!(
	@with_return1,
    query(engine: &mut Engine, gui: &mut Gui,)()(x: f32, y: f32,)-> Option<f64> {
		{
			pi_export_base::export::await_last_frame(engine);
			crate::index::query(engine, gui, x, y)
		}
	}
);

pub fn to_linear_gradient_color(color_and_positions: &[f32], direction: f32) -> LinearGradientColor {
    let arr = color_and_positions;
    let len = arr.len();
    let count = len / 5;
    let mut list = Vec::with_capacity(count);
    for i in 0..count {
        let start = i * 5;
        let color_pos = ColorAndPosition {
            rgba: CgColor::new(arr[start], arr[start + 1], arr[start + 2], arr[start + 3]),
            position: arr[start + 4],
        };
        list.push(color_pos);
    }
    LinearGradientColor {
        direction: direction,
        list: list,
    }
}


pub enum LengthUnitType {
    Pixel,
    Percent,
}

#[inline]
fn set_animation_str_inner(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
    use pi_style::style_parse::parse_animation;

    let node_id = Entity::from_bits(unsafe { transmute(node_id) });
    let mut input = cssparser::ParserInput::new(value);
    let mut parse = cssparser::Parser::new(&mut input);

    let mut animations = match parse_animation(&mut parse) {
        Ok(r) => r,
        Err(e) => {
            log::error!("set_animation_str fail, animation: {}, err: {:?}", value, e);
            return;
        }
    };
    animations.name.scope_hash = scope_hash as usize;
    log::debug!("set_animation_str: {:?}", animations);
    if animations.name.value.len() > 0 {
        gui.commands.set_style(node_id, AnimationNameType(animations.name));
        gui.commands.set_style(node_id, AnimationDurationType(animations.duration));
        gui.commands.set_style(node_id, AnimationTimingFunctionType(animations.timing_function));
        gui.commands.set_style(node_id, AnimationIterationCountType(animations.iteration_count));
        gui.commands.set_style(node_id, AnimationDelayType(animations.delay));
        gui.commands.set_style(node_id, AnimationDirectionType(animations.direction));
        gui.commands.set_style(node_id, AnimationFillModeType(animations.fill_mode));
        gui.commands.set_style(node_id, AnimationPlayStateType(animations.play_state));
    }
}

#[inline]
fn reset_animation_str_inner(gui: &mut Gui, node_id: f64) {
    let node_id = Entity::from_bits(unsafe { transmute(node_id) });
    gui.commands.set_style(node_id, ResetAnimationNameType);
    gui.commands.set_style(node_id, ResetAnimationDurationType);
    gui.commands.set_style(node_id, ResetAnimationIterationCountType);
    gui.commands.set_style(node_id, ResetAnimationDelayType);
    gui.commands.set_style(node_id, ResetAnimationDirectionType);
    gui.commands.set_style(node_id, ResetAnimationFillModeType);
    gui.commands.set_style(node_id, ResetAnimationPlayStateType);
}


pub mod debug {
	
}