pub mod style_macro {
    //! 将设置布局属性的接口导出到js
    use std::mem::transmute;
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
    use pi_style::style_parse::{
        parse_comma_separated, parse_text_shadow, parse_as_image, StyleParse,
    };
    use smallvec::SmallVec;
    pub use pi_export_base::export::{Atom, Engine};
    pub use crate::index::{OffsetDocument, Size, Gui};
    use pi_ui_render::resource::animation_sheet::KeyFramesSheet;
    #[cfg(target_arch = "wasm32")]
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
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_align_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_align_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_align_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignContentType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_align_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignContentType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_align_items(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignItemsType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_align_items(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignItemsType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_align_items(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignItemsType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_align_items(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignItemsType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_justify_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, JustifyContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_justify_content(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, JustifyContentType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_justify_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetJustifyContentType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_justify_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetJustifyContentType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_direction(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexDirectionType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_direction(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, FlexDirectionType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexDirectionType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexDirectionType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_wrap(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexWrapType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_wrap(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexWrapType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_wrap(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexWrapType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_wrap(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexWrapType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_align_self(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignSelfType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_align_self(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, AlignSelfType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_align_self(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignSelfType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_align_self(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAlignSelfType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_position_type(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, PositionTypeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_position_type(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, PositionTypeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_position_type(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetPositionTypeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_position_type(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetPositionTypeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_grow(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_grow(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_grow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_grow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_shrink(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_shrink(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexGrowType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_shrink(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_shrink(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexGrowType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_basis_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_basis_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_basis(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_basis(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FlexBasisType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_flex_basis_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFlexBasisType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WidthType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, HeightType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinWidthType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MinHeightType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_min_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMinHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_width_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_width_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_width(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_width(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxWidthType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_width_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxWidthType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Percent(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_height_percent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Percent(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Points(v)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_height(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Points(v)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Auto));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, MaxHeightType(Dimension::Auto));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_max_height_auto(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaxHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_padding_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PaddingTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_padding_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PaddingTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_padding_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_padding_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_padding(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PaddingTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_padding(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PaddingTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_padding(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_padding(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, PaddingTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, PaddingTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, PaddingRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PaddingBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PaddingLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_padding_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPaddingTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPaddingRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPaddingBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPaddingLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_margin_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, MarginTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_margin_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, MarginTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_margin_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_margin_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_margin(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, MarginTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_margin(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, MarginTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_margin(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_margin(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, MarginTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, MarginTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, MarginRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, MarginBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, MarginLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_margin_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetMarginTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetMarginRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetMarginBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetMarginLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, BorderTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, BorderTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, BorderTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, BorderTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, BorderTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, BorderTopType(Dimension::Auto)),
            Edge::Right => {
                gui.commands.set_style(node_id, BorderRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, BorderBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, BorderLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetBorderTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetBorderRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetBorderBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetBorderLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_position_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands
                    .set_style(node_id, PositionBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_position_percent(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Percent(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Percent(v)))
            }
            Edge::Bottom => {
                gui.commands
                    .set_style(node_id, PositionBottomType(Dimension::Percent(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Percent(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_position_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_position_percent(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_position(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PositionBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_position(gui: &mut Gui, node_id: f64, edge: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Points(v)))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Points(v)))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PositionBottomType(Dimension::Points(v)))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Points(v)))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_position(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_position(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Auto))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PositionBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => {
                gui.commands.set_style(node_id, PositionTopType(Dimension::Auto))
            }
            Edge::Right => {
                gui.commands.set_style(node_id, PositionRightType(Dimension::Auto))
            }
            Edge::Bottom => {
                gui.commands.set_style(node_id, PositionBottomType(Dimension::Auto))
            }
            Edge::Left => {
                gui.commands.set_style(node_id, PositionLeftType(Dimension::Auto))
            }
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_position_auto(gui: &mut Gui, node_id: f64, edge: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        match unsafe { transmute(edge as u8) } {
            Edge::Top => gui.commands.set_style(node_id, ResetPositionTopType),
            Edge::Right => gui.commands.set_style(node_id, ResetPositionRightType),
            Edge::Bottom => gui.commands.set_style(node_id, ResetPositionBottomType),
            Edge::Left => gui.commands.set_style(node_id, ResetPositionLeftType),
            _ => return,
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_background_rgba_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundColorType(Color::RGBA(CgColor::new(r, g, b, a))),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_background_rgba_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundColorType(Color::RGBA(CgColor::new(r, g, b, a))),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_background_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_background_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_background_linear_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundColorType(
                    Color::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_background_linear_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundColorType(
                    Color::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_background_linear_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_background_linear_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundColorType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BorderColorType(CgColor::new(r, g, b, a)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BorderColorType(CgColor::new(r, g, b, a)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderColorType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderColorType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_radius(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderRadiusType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let border_radius = pi_style::style_parse::parse_border_radius(
                        &mut parse,
                    );
                    if let Ok(value) = border_radius {
                        value
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_radius(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderRadiusType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let border_radius = pi_style::style_parse::parse_border_radius(
                        &mut parse,
                    );
                    if let Ok(value) = border_radius {
                        value
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_radius(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderRadiusType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_radius(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderRadiusType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_box_shadow(
        gui: &mut Gui,
        node_id: f64,
        h: f32,
        v: f32,
        blur: f32,
        spread: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BoxShadowType(BoxShadow {
                    h: h,
                    v: v,
                    blur: blur,
                    spread: spread,
                    color: CgColor::new(r, g, b, a),
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_box_shadow(
        gui: &mut Gui,
        node_id: f64,
        h: f32,
        v: f32,
        blur: f32,
        spread: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BoxShadowType(BoxShadow {
                    h: h,
                    v: v,
                    blur: blur,
                    spread: spread,
                    color: CgColor::new(r, g, b, a),
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_box_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBoxShadowType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_box_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBoxShadowType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_object_fit(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ObjectFitType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_object_fit(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ObjectFitType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_object_fit(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetObjectFitType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_object_fit(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetObjectFitType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_background_repeat(gui: &mut Gui, node_id: f64, x: u8, y: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundRepeatType(ImageRepeat {
                    x: unsafe { transmute(x as u8) },
                    y: unsafe { transmute(y as u8) },
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_background_repeat(gui: &mut Gui, node_id: f64, x: u8, y: u8) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundRepeatType(ImageRepeat {
                    x: unsafe { transmute(x as u8) },
                    y: unsafe { transmute(y as u8) },
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_background_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundRepeatType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_background_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundRepeatType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_mask_image_linear(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                MaskImageType(
                    MaskImage::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_mask_image_linear(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                MaskImageType(
                    MaskImage::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_mask_image_linear(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_mask_image_linear(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BackgroundImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageClipType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageClipType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_mask_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                MaskImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_mask_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                MaskImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_mask_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageClipType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_mask_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageClipType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_image_clip(
        gui: &mut Gui,
        node_id: f64,
        u1: f32,
        v1: f32,
        u2: f32,
        v2: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageClipType(
                    NotNanRect::new(
                        unsafe { NotNan::new_unchecked(v1) },
                        unsafe { NotNan::new_unchecked(u2) },
                        unsafe { NotNan::new_unchecked(v2) },
                        unsafe { NotNan::new_unchecked(u1) },
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageClipType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_image_clip(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageClipType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_image_slice(
        gui: &mut Gui,
        node_id: f64,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        fill: bool,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageSliceType(BorderImageSlice {
                    top: unsafe { NotNan::new_unchecked(top) },
                    right: unsafe { NotNan::new_unchecked(right) },
                    bottom: unsafe { NotNan::new_unchecked(bottom) },
                    left: unsafe { NotNan::new_unchecked(left) },
                    fill,
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_image_slice(
        gui: &mut Gui,
        node_id: f64,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        fill: bool,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageSliceType(BorderImageSlice {
                    top: unsafe { NotNan::new_unchecked(top) },
                    right: unsafe { NotNan::new_unchecked(right) },
                    bottom: unsafe { NotNan::new_unchecked(bottom) },
                    left: unsafe { NotNan::new_unchecked(left) },
                    fill,
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_image_slice(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageSliceType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_image_slice(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageSliceType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_image_repeat(
        gui: &mut Gui,
        node_id: f64,
        vertical: u8,
        horizontal: u8,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageRepeatType(ImageRepeat {
                    x: unsafe { transmute(vertical as u8) },
                    y: unsafe { transmute(horizontal as u8) },
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_image_repeat(
        gui: &mut Gui,
        node_id: f64,
        vertical: u8,
        horizontal: u8,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                BorderImageRepeatType(ImageRepeat {
                    x: unsafe { transmute(vertical as u8) },
                    y: unsafe { transmute(horizontal as u8) },
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_image_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageRepeatType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_image_repeat(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageRepeatType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_overflow(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OverflowType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_overflow(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OverflowType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_overflow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOverflowType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_overflow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOverflowType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_opacity(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OpacityType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_opacity(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, OpacityType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_opacity(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOpacityType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_opacity(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetOpacityType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_display(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, DisplayType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_display(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, DisplayType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_display(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetDisplayType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_display(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetDisplayType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_visibility(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, VisibilityType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_visibility(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, VisibilityType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_visibility(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetVisibilityType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_visibility(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetVisibilityType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_enable(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, EnableType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_enable(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, EnableType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_enable(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetEnableType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_enable(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetEnableType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_blend_mode(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlendModeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_blend_mode(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlendModeType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_blend_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlendModeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_blend_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlendModeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_zindex(gui: &mut Gui, node_id: f64, v: i32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ZIndexType(v as isize));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_zindex(gui: &mut Gui, node_id: f64, v: i32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ZIndexType(v as isize));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_zindex(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetZIndexType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_zindex(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetZIndexType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_as_image(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AsImageType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    match parse_as_image(&mut parse) {
                        Ok(r) => r,
                        Err(e) => {
                            ();
                            return;
                        }
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_as_image(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AsImageType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    match parse_as_image(&mut parse) {
                        Ok(r) => r,
                        Err(e) => {
                            ();
                            return;
                        }
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_as_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAsImageType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_as_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAsImageType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_filter_blur(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlurType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_filter_blur(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BlurType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_filter_blur(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlurType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_filter_blur(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBlurType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_transform_will_change(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TransformWillChangeType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_transform_will_change(gui: &mut Gui, node_id: f64, v: bool) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TransformWillChangeType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_transform_will_change(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformWillChangeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_transform_will_change(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformWillChangeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_filter_hsi(gui: &mut Gui, node_id: f64, h: f32, s: f32, _i: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                HsiType({
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
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_filter_hsi(gui: &mut Gui, node_id: f64, h: f32, s: f32, _i: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                HsiType({
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
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_filter_hsi(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHsiType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_filter_hsi(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetHsiType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_translate(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TranslateType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let translate = pi_style::style_parse::parse_mult(
                        &mut parse,
                        [LengthUnit::default(), LengthUnit::default()],
                        pi_style::style_parse::parse_len_or_percent,
                    );
                    if let Ok(translate) = translate {
                        translate
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_translate(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TranslateType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let translate = pi_style::style_parse::parse_mult(
                        &mut parse,
                        [LengthUnit::default(), LengthUnit::default()],
                        pi_style::style_parse::parse_len_or_percent,
                    );
                    if let Ok(translate) = translate {
                        translate
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_translate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTranslateType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_translate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTranslateType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_scale(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ScaleType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let scale = pi_style::style_parse::parse_mult(
                        &mut parse,
                        [1.0f32, 1.0f32],
                        pi_style::style_parse::parse_number,
                    );
                    if let Ok(scale) = scale { scale } else { Default::default() }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_scale(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ScaleType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let scale = pi_style::style_parse::parse_mult(
                        &mut parse,
                        [1.0f32, 1.0f32],
                        pi_style::style_parse::parse_number,
                    );
                    if let Ok(scale) = scale { scale } else { Default::default() }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_scale(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetScaleType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_scale(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetScaleType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_rotate(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                RotateType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let rotate = pi_style::style_parse::parse_angle(&mut parse);
                    if let Ok(rotate) = rotate { rotate } else { Default::default() }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_rotate(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                RotateType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let rotate = pi_style::style_parse::parse_angle(&mut parse);
                    if let Ok(rotate) = rotate { rotate } else { Default::default() }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_rotate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetRotateType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_rotate(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetRotateType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_transform(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TransformType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let transform = pi_style::style_parse::parse_transform(&mut parse);
                    if let Ok(transform) = transform {
                        transform
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_transform(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TransformType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let transform = pi_style::style_parse::parse_transform(&mut parse);
                    if let Ok(transform) = transform {
                        transform
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_transform(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_transform_origin(
        gui: &mut Gui,
        node_id: f64,
        x_ty: f64,
        x: f32,
        y_ty: f64,
        y: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TransformOriginType({
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
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_transform_origin(
        gui: &mut Gui,
        node_id: f64,
        x_ty: f64,
        x: f32,
        y_ty: f64,
        y: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TransformOriginType({
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
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_transform_origin(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformOriginType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_transform_origin(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTransformOriginType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_letter_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LetterSpacingType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_letter_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LetterSpacingType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_letter_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLetterSpacingType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_letter_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLetterSpacingType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_word_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WordSpacingType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_word_spacing(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WordSpacingType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_word_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWordSpacingType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_word_spacing(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWordSpacingType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_rgba_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ColorType(Color::RGBA(CgColor::new(r, g, b, a))));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_rgba_color(
        gui: &mut Gui,
        node_id: f64,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, ColorType(Color::RGBA(CgColor::new(r, g, b, a))));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_rgba_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_linear_gradient_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ColorType(
                    Color::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_linear_gradient_color(
        gui: &mut Gui,
        node_id: f64,
        direction: f32,
        color_and_positions: Vec<f32>,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ColorType(
                    Color::LinearGradient(
                        to_linear_gradient_color(
                            color_and_positions.as_slice(),
                            direction,
                        ),
                    ),
                ),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_linear_gradient_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_linear_gradient_color(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetColorType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Normal));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Normal));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_line_height_normal(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_line_height(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Length(value)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_line_height(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Length(value)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_line_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_line_height(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_line_height_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Percent(value)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_line_height_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, LineHeightType(LineHeight::Percent(value)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_line_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_line_height_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetLineHeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_indent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TextIndentType(v));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_indent(gui: &mut Gui, node_id: f64, v: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, TextIndentType(v));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_indent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextIndentType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_indent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextIndentType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            let v: TextAlign = unsafe { transmute(v as u8) };
            gui.commands.set_style(node_id, TextAlignType(v));
            gui.commands
                .set_style(
                    node_id,
                    JustifyContentType(
                        match v {
                            TextAlign::Left => JustifyContent::FlexStart,
                            TextAlign::Right => JustifyContent::FlexEnd,
                            TextAlign::Center => JustifyContent::Center,
                            TextAlign::Justify => JustifyContent::SpaceBetween,
                        },
                    ),
                );
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            let v: TextAlign = unsafe { transmute(v as u8) };
            gui.commands.set_style(node_id, TextAlignType(v));
            gui.commands
                .set_style(
                    node_id,
                    JustifyContentType(
                        match v {
                            TextAlign::Left => JustifyContent::FlexStart,
                            TextAlign::Right => JustifyContent::FlexEnd,
                            TextAlign::Center => JustifyContent::Center,
                            TextAlign::Justify => JustifyContent::SpaceBetween,
                        },
                    ),
                );
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            gui.commands.set_style(node_id, ResetTextAlignType);
            gui.commands.set_style(node_id, ResetJustifyContentType);
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            gui.commands.set_style(node_id, ResetTextAlignType);
            gui.commands.set_style(node_id, ResetJustifyContentType);
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_vertical_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            let v: VerticalAlign = unsafe { transmute(v as u8) };
            gui.commands.set_style(node_id, VerticalAlignType(v));
            gui.commands
                .set_style(
                    node_id,
                    AlignSelfType(
                        match v {
                            VerticalAlign::Top => AlignSelf::FlexStart,
                            VerticalAlign::Bottom => AlignSelf::FlexEnd,
                            VerticalAlign::Middle => AlignSelf::Center,
                        },
                    ),
                );
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_vertical_align(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            let v: VerticalAlign = unsafe { transmute(v as u8) };
            gui.commands.set_style(node_id, VerticalAlignType(v));
            gui.commands
                .set_style(
                    node_id,
                    AlignSelfType(
                        match v {
                            VerticalAlign::Top => AlignSelf::FlexStart,
                            VerticalAlign::Bottom => AlignSelf::FlexEnd,
                            VerticalAlign::Middle => AlignSelf::Center,
                        },
                    ),
                );
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_vertical_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            gui.commands.set_style(node_id, ResetVerticalAlignType);
            gui.commands.set_style(node_id, ResetAlignSelfType);
        };
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_vertical_align(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {
            gui.commands.set_style(node_id, ResetVerticalAlignType);
            gui.commands.set_style(node_id, ResetAlignSelfType);
        };
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_stroke(
        gui: &mut Gui,
        node_id: f64,
        width: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextStrokeType(Stroke {
                    width: NotNan::new(width).expect("stroke width is nan"),
                    color: CgColor::new(r, g, b, a),
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_stroke(
        gui: &mut Gui,
        node_id: f64,
        width: f32,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextStrokeType(Stroke {
                    width: NotNan::new(width).expect("stroke width is nan"),
                    color: CgColor::new(r, g, b, a),
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_stroke(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextStrokeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_stroke(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextStrokeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_white_space(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WhiteSpaceType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_white_space(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, WhiteSpaceType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_white_space(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWhiteSpaceType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_white_space(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetWhiteSpaceType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_style(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontStyleType(unsafe { transmute(v as u8) }));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_style(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontStyleType(unsafe { transmute(v as u8) }));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_style(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontStyleType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_style(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontStyleType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_weight(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontWeightType(v as usize));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_weight(gui: &mut Gui, node_id: f64, v: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontWeightType(v as usize));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_weight(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontWeightType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_weight(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontWeightType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::None));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::None));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_size_none(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_size(gui: &mut Gui, node_id: f64, value: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::Length(value as usize)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_size(gui: &mut Gui, node_id: f64, value: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::Length(value as usize)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_size(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_size(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_size_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::Percent(value)));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_size_percent(gui: &mut Gui, node_id: f64, value: f32) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontSizeType(FontSize::Percent(value)));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_size_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_size_percent(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontSizeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_content_utf8(gui: &mut Gui, node_id: f64, content: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextContentType({
                    let content = unsafe { String::from_utf8_unchecked(content) };
                    TextContent(content, pi_atom::Atom::from(""))
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_content_utf8(gui: &mut Gui, node_id: f64, content: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextContentType({
                    let content = unsafe { String::from_utf8_unchecked(content) };
                    TextContent(content, pi_atom::Atom::from(""))
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_content_utf8(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_content_utf8(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_clip_path_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ClipPathType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    match BaseShape::parse(&mut parse) {
                        Ok(r) => r,
                        Err(e) => {
                            ();
                            return;
                        }
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_clip_path_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                ClipPathType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    match BaseShape::parse(&mut parse) {
                        Ok(r) => r,
                        Err(e) => {
                            ();
                            return;
                        }
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_clip_path_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetClipPathType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_clip_path_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetClipPathType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_duration(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDurationType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_duration(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDurationType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_duration(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDurationType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_duration(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDurationType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_delay(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDelayType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_delay(gui: &mut Gui, node_id: f64, name: Vec<usize>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDelayType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[usize; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_delay(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDelayType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_delay(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDelayType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_iteration_count(gui: &mut Gui, node_id: f64, name: Vec<f32>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationIterationCountType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_iteration_count(gui: &mut Gui, node_id: f64, name: Vec<f32>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationIterationCountType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[f32; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_iteration_count(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationIterationCountType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_iteration_count(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationIterationCountType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_direction(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDirectionType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_direction(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationDirectionType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDirectionType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_direction(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationDirectionType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_fill_mode(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationFillModeType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_fill_mode(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationFillModeType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_fill_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationFillModeType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_fill_mode(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationFillModeType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_play_state(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationPlayStateType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_play_state(gui: &mut Gui, node_id: f64, name: Vec<u8>) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationPlayStateType(unsafe {
                    transmute(name.into_iter().collect::<SmallVec<[u8; 1]>>())
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_play_state(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationPlayStateType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_play_state(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationPlayStateType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_name_str(
        gui: &mut Gui,
        node_id: f64,
        value: &str,
        scope_hash: u32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationNameType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let value = if let Ok(value)
                        = parse_comma_separated::<
                            _,
                            _,
                            cssparser::ParseError<pi_style::style_parse::TokenParseError>,
                        >(
                            &mut parse,
                            |input| Ok(
                                pi_atom::Atom::from(input.expect_ident()?.as_ref()),
                            ),
                        ) {
                        value
                    } else {
                        Default::default()
                    };
                    AnimationName {
                        scope_hash: scope_hash as usize,
                        value,
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_name_str(
        gui: &mut Gui,
        node_id: f64,
        value: &str,
        scope_hash: u32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationNameType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let value = if let Ok(value)
                        = parse_comma_separated::<
                            _,
                            _,
                            cssparser::ParseError<pi_style::style_parse::TokenParseError>,
                        >(
                            &mut parse,
                            |input| Ok(
                                pi_atom::Atom::from(input.expect_ident()?.as_ref()),
                            ),
                        ) {
                        value
                    } else {
                        Default::default()
                    };
                    AnimationName {
                        scope_hash: scope_hash as usize,
                        value,
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_name_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationNameType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_name_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationNameType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_runtime_animation(
        gui: &mut Gui,
        node_id: f64,
        animation: &str,
        key_frames: &str,
        scope_hash: u32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .add_runtime_animation(node_id, animation, key_frames, scope_hash as usize);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_runtime_animation(
        gui: &mut Gui,
        node_id: f64,
        animation: &str,
        key_frames: &str,
        scope_hash: u32,
    ) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .add_runtime_animation(node_id, animation, key_frames, scope_hash as usize);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_runtime_animation(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {};
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_runtime_animation(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        {};
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        set_animation_str_inner(gui, node_id, value, scope_hash);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_animation_str(gui: &mut Gui, node_id: f64, value: &str, scope_hash: u32) {
        set_animation_str_inner(gui, node_id, value, scope_hash);
    }
    #[cfg(feature = "pi_js_export")]
    pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
        reset_animation_str_inner(gui, node_id);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn reset_animation_str(gui: &mut Gui, node_id: f64) {
        reset_animation_str_inner(gui, node_id);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaskImageType(MaskImage::Path((**image_hash).clone())));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_mask_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(node_id, MaskImageType(MaskImage::Path((**image_hash).clone())));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_mask_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_mask_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetMaskImageType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_background_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BackgroundImageType((**image_hash).clone()));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_background_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BackgroundImageType((**image_hash).clone()));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_background_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_background_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBackgroundImageType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_border_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BorderImageType((**image_hash).clone()));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_border_image(gui: &mut Gui, node_id: f64, image_hash: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, BorderImageType((**image_hash).clone()));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_border_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_border_image(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetBorderImageType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_shadow(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextShadowType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let shadows = parse_text_shadow(&mut parse);
                    if let Ok(value) = shadows { value } else { Default::default() }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_shadow(gui: &mut Gui, node_id: f64, s: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextShadowType({
                    let mut input = cssparser::ParserInput::new(s);
                    let mut parse = cssparser::Parser::new(&mut input);
                    let shadows = parse_text_shadow(&mut parse);
                    if let Ok(value) = shadows { value } else { Default::default() }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextShadowType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_shadow(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextShadowType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_font_family(gui: &mut Gui, node_id: f64, name: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontFamilyType((**name).clone()));
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_font_family(gui: &mut Gui, node_id: f64, name: &Atom) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, FontFamilyType((**name).clone()));
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_font_family(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontFamilyType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_font_family(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetFontFamilyType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_text_content(gui: &mut Gui, node_id: f64, content: String) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextContentType(TextContent(content, pi_atom::Atom::from(""))),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_text_content(gui: &mut Gui, node_id: f64, content: String) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                TextContentType(TextContent(content, pi_atom::Atom::from(""))),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_text_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_text_content(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetTextContentType);
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn set_animation_timing_function_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationTimingFunctionType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    if let Ok(value)
                        = parse_comma_separated(
                            &mut parse,
                            <AnimationTimingFunction as StyleParse>::parse,
                        ) {
                        value
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn set_animation_timing_function_str(gui: &mut Gui, node_id: f64, value: &str) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands
            .set_style(
                node_id,
                AnimationTimingFunctionType({
                    let mut input = cssparser::ParserInput::new(value);
                    let mut parse = cssparser::Parser::new(&mut input);
                    if let Ok(value)
                        = parse_comma_separated(
                            &mut parse,
                            <AnimationTimingFunction as StyleParse>::parse,
                        ) {
                        value
                    } else {
                        Default::default()
                    }
                }),
            );
    }
    #[cfg(feature = "pi_js_export")]
    #[allow(unused_attributes)]
    pub fn reset_animation_timing_function_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationTimingFunctionType);
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    #[allow(unused_attributes)]
    pub fn reset_animation_timing_function_str(gui: &mut Gui, node_id: f64) {
        let node_id = unsafe { Entity::from_bits(transmute::<f64, u64>(node_id)) };
        gui.commands.set_style(node_id, ResetAnimationTimingFunctionType);
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_default_style(gui: &mut Gui, value: &str) {
        {
            gui.commands.set_default_style_by_str(value, 0);
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_default_style(gui: &mut Gui, value: &str) {
        {
            gui.commands.set_default_style_by_str(value, 0);
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn create_class_by_str(gui: &mut Gui, css: &str, scope_hash: u32) {
        {
            gui.commands.add_css(css, scope_hash as usize);
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn create_class_by_str(gui: &mut Gui, css: &str, scope_hash: u32) {
        {
            gui.commands.add_css(css, scope_hash as usize);
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_class(gui: &mut Gui, node: f64, class_name: Vec<u32>) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut s = SmallVec::with_capacity(class_name.len());
            for i in class_name.iter() {
                s.push(*i as usize);
            }
            gui.commands.set_class(node, ClassName(s));
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_class(gui: &mut Gui, node: f64, class_name: Vec<u32>) {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut s = SmallVec::with_capacity(class_name.len());
            for i in class_name.iter() {
                s.push(*i as usize);
            }
            gui.commands.set_class(node, ClassName(s));
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_view_port(
        gui: &mut Gui,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        root: f64,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands
                .set_view_port(
                    root,
                    pi_ui_render::components::user::Viewport(
                        Aabb2::new(
                            Point2::new(x as f32, y as f32),
                            Point2::new(width as f32, height as f32),
                        ),
                    ),
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_view_port(
        gui: &mut Gui,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        root: f64,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands
                .set_view_port(
                    root,
                    pi_ui_render::components::user::Viewport(
                        Aabb2::new(
                            Point2::new(x as f32, y as f32),
                            Point2::new(width as f32, height as f32),
                        ),
                    ),
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_brush(gui: &mut Gui, node: f64, brush: f64) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            let brush = unsafe { Entity::from_bits(transmute::<f64, u64>(brush)) };
            gui.commands
                .push_cmd(
                    ComponentCmd(pi_ui_render::components::user::Canvas(brush), node),
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_brush(gui: &mut Gui, node: f64, brush: f64) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            let brush = unsafe { Entity::from_bits(transmute::<f64, u64>(brush)) };
            gui.commands
                .push_cmd(
                    ComponentCmd(pi_ui_render::components::user::Canvas(brush), node),
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_radial_wave(
        gui: &mut Gui,
        node: f64,
        aspect_ratio: bool,
        start: f32,
        end: f32,
        center_x: f32,
        center_y: f32,
        cycle: u8,
        weight: f32,
    ) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            gui.commands
                .push_cmd(
                    NodeCmd(
                        RadialWave(pi_postprocess::prelude::RadialWave {
                            aspect_ratio,
                            start,
                            end,
                            center_x,
                            center_y,
                            cycle,
                            weight,
                        }),
                        node,
                    ),
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_radial_wave(
        gui: &mut Gui,
        node: f64,
        aspect_ratio: bool,
        start: f32,
        end: f32,
        center_x: f32,
        center_y: f32,
        cycle: u8,
        weight: f32,
    ) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            gui.commands
                .push_cmd(
                    NodeCmd(
                        RadialWave(pi_postprocess::prelude::RadialWave {
                            aspect_ratio,
                            start,
                            end,
                            center_x,
                            center_y,
                            cycle,
                            weight,
                        }),
                        node,
                    ),
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_rendertarget_type(gui: &mut Gui, node: f64, target_ty: u8) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            gui.commands
                .set_target_type(
                    node,
                    unsafe {
                        transmute::<
                            _,
                            pi_ui_render::components::user::RenderTargetType,
                        >(target_ty)
                    },
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_rendertarget_type(gui: &mut Gui, node: f64, target_ty: u8) {
        {
            let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
            gui.commands
                .set_target_type(
                    node,
                    unsafe {
                        transmute::<
                            _,
                            pi_ui_render::components::user::RenderTargetType,
                        >(target_ty)
                    },
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_clear_color(
        gui: &mut Gui,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        root: f64,
        is_clear_window: bool,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands
                .set_clear_color(
                    root,
                    pi_ui_render::components::user::ClearColor(
                        CgColor::new(r, g, b, a),
                        is_clear_window,
                    ),
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_clear_color(
        gui: &mut Gui,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        root: f64,
        is_clear_window: bool,
    ) {
        {
            let root = unsafe { Entity::from_bits(transmute::<f64, u64>(root)) };
            gui.commands
                .set_clear_color(
                    root,
                    pi_ui_render::components::user::ClearColor(
                        CgColor::new(r, g, b, a),
                        is_clear_window,
                    ),
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
        match postcard::from_bytes::<Vec<pi_style::style_parse::ClassMap>>(bin) {
            Ok(r) => {
                gui.commands.add_css_bin(pi_ui_render::resource::ExtendCssCmd(r));
            }
            Err(e) => {
                ();
                return;
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn create_class_by_bin(gui: &mut Gui, bin: &[u8]) {
        match postcard::from_bytes::<Vec<pi_style::style_parse::ClassMap>>(bin) {
            Ok(r) => {
                gui.commands.add_css_bin(pi_ui_render::resource::ExtendCssCmd(r));
            }
            Err(e) => {
                ();
                return;
            }
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_render_dirty(gui: &mut Gui, root: f64) {
        {
            let node: Entity = Entity::from_bits(unsafe { transmute(root) });
            gui.commands
                .set_render_dirty(
                    node,
                    pi_ui_render::components::user::RenderDirty(true),
                );
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_render_dirty(gui: &mut Gui, root: f64) {
        {
            let node: Entity = Entity::from_bits(unsafe { transmute(root) });
            gui.commands
                .set_render_dirty(
                    node,
                    pi_ui_render::components::user::RenderDirty(true),
                );
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn bind_render_target(_gui: &mut Gui) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn bind_render_target(_gui: &mut Gui) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_pixel_ratio(_gui: &mut Gui, _pixel_ratio: f32) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_pixel_ratio(_gui: &mut Gui, _pixel_ratio: f32) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn nullify_clear_color(_gui: &mut Gui) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn nullify_clear_color(_gui: &mut Gui) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_scissor(_gui: &mut Gui, _x: i32, _y: i32, _width: i32, _height: i32) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_scissor(_gui: &mut Gui, _x: i32, _y: i32, _width: i32, _height: i32) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_project_transfrom(
        _gui: &mut Gui,
        _scale_x: f32,
        _scale_y: f32,
        _translate_x: f32,
        _translate_y: f32,
        _rotate: f32,
        _width: f64,
        _height: f64,
    ) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_project_transfrom(
        _gui: &mut Gui,
        _scale_x: f32,
        _scale_y: f32,
        _translate_x: f32,
        _translate_y: f32,
        _rotate: f32,
        _width: f64,
        _height: f64,
    ) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn force_update_text(_gui: &mut Gui, _node: f64) {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn force_update_text(_gui: &mut Gui, _node: f64) {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn set_shader(_gui: &mut Gui, _shader_name: &str, _shader_code: &str) {
        {}
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn set_shader(_gui: &mut Gui, _shader_name: &str, _shader_code: &str) {
        {}
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_font_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_font_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_class_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_class_sheet(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn create_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn create_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn destroy_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn destroy_render_target(_gui: &mut Gui, _fbo: f64) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn clone_engine(_gui: &mut Gui) -> Gui {
        ::core::panicking::panic("not yet implemented")
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn clone_engine(_gui: &mut Gui) -> Gui {
        ::core::panicking::panic("not yet implemented")
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_text_texture_width(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_text_texture_width(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_text_texture_height(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_text_texture_height(_gui: &mut Gui) -> u32 {
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn node_is_exist(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn node_is_exist(_gui: &mut Gui, _node: f64) -> bool {
        true
    }
    #[cfg(feature = "pi_js_export")]
    pub fn node_is_visibility(_gui: &mut Gui, _engine: &Engine, _node: f64) -> bool {
        true
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn node_is_visibility(_gui: &mut Gui, _engine: &Engine, _node: f64) -> bool {
        true
    }
    type ReturnNode = Option<f64>;
    #[cfg(feature = "pi_js_export")]
    pub fn first_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn first_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(feature = "pi_js_export")]
    pub fn last_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn last_child(_gui: &Gui, _parent: f64) -> ReturnNode {
        let _parent = unsafe { Entity::from_bits(transmute::<f64, u64>(_parent)) };
        None
    }
    #[cfg(feature = "pi_js_export")]
    pub fn next_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn next_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(feature = "pi_js_export")]
    pub fn previous_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn previous_sibling(_gui: &Gui, _node: f64) -> ReturnNode {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        None
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_layer(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_layer(_gui: &Gui, _node: f64) -> u32 {
        let _node = unsafe { Entity::from_bits(transmute::<f64, u64>(_node)) };
        0
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_enable(gui: &mut Gui, engine: &Engine, node: f64) -> bool {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        if let Ok(is_show) = gui.enable_query.get(&engine.world, node) {
            is_show.get_enable()
        } else {
            false
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_enable(gui: &mut Gui, engine: &Engine, node: f64) -> bool {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        if let Ok(is_show) = gui.enable_query.get(&engine.world, node) {
            is_show.get_enable()
        } else {
            false
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn offset_top(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
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
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn offset_top(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
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
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn offset_left(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
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
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn offset_left(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
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
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn offset_width(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
                layout.rect.right - layout.rect.left
            } else {
                0.0
            };
            r.round() as u32
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn offset_width(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
                layout.rect.right - layout.rect.left
            } else {
                0.0
            };
            r.round() as u32
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn offset_height(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
                layout.rect.bottom - layout.rect.top
            } else {
                0.0
            };
            r.round() as u32
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn offset_height(gui: &mut Gui, engine: &Engine, node: f64) -> u32 {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let r = if let Ok(layout) = gui.layout_query.get(&engine.world, node) {
                layout.rect.bottom - layout.rect.top
            } else {
                0.0
            };
            r.round() as u32
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_class_name(_gui: &mut Gui, engine: &mut Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let value = match engine.world.get::<ClassName>(node) {
                Some(r) => Some(&r.0),
                _ => None,
            };
            serde_json::to_string(&value).unwrap()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_class_name(_gui: &mut Gui, engine: &mut Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let value = match engine.world.get::<ClassName>(node) {
                Some(r) => Some(&r.0),
                _ => None,
            };
            serde_json::to_string(&value).unwrap()
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn offset_document(gui: &mut Gui, engine: &Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let value = match gui.quad_query.get(&engine.world, node) {
                Ok(quad) => {
                    OffsetDocument {
                        left: quad.mins.x,
                        top: quad.mins.y,
                        width: quad.maxs.x - quad.mins.x,
                        height: quad.maxs.y - quad.mins.y,
                    }
                }
                _ => {
                    OffsetDocument {
                        left: 0.0,
                        top: 0.0,
                        width: 0.0,
                        height: 0.0,
                    }
                }
            };
            serde_json::to_string(&value).unwrap()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn offset_document(gui: &mut Gui, engine: &Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let value = match gui.quad_query.get(&engine.world, node) {
                Ok(quad) => {
                    OffsetDocument {
                        left: quad.mins.x,
                        top: quad.mins.y,
                        width: quad.maxs.x - quad.mins.x,
                        height: quad.maxs.y - quad.mins.y,
                    }
                }
                _ => {
                    OffsetDocument {
                        left: 0.0,
                        top: 0.0,
                        width: 0.0,
                        height: 0.0,
                    }
                }
            };
            serde_json::to_string(&value).unwrap()
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn content_box(gui: &mut Gui, engine: &Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut cur_child = match gui.down_query.get(&engine.world, node) {
                Ok(down) => down.head(),
                _ => {
                    return serde_json::to_string(&Size { width: 0.0, height: 0.0 })
                        .unwrap();
                }
            };
            let (mut left, mut right, mut top, mut bottom) = (
                std::f32::MAX,
                0.0,
                std::f32::MAX,
                0.0,
            );
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
            serde_json::to_string(
                    &Size {
                        width: right - left,
                        height: bottom - top,
                    },
                )
                .unwrap()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn content_box(gui: &mut Gui, engine: &Engine, node: f64) -> String {
        let node = unsafe { Entity::from_bits(transmute::<f64, u64>(node)) };
        {
            let mut cur_child = match gui.down_query.get(&engine.world, node) {
                Ok(down) => down.head(),
                _ => {
                    return serde_json::to_string(&Size { width: 0.0, height: 0.0 })
                        .unwrap();
                }
            };
            let (mut left, mut right, mut top, mut bottom) = (
                std::f32::MAX,
                0.0,
                std::f32::MAX,
                0.0,
            );
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
            serde_json::to_string(
                    &Size {
                        width: right - left,
                        height: bottom - top,
                    },
                )
                .unwrap()
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_animation_events_max_len(engine: &Engine) -> u32 {
        {
            let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
            let events = key_frames.get_animation_events();
            return (events.len() * 5) as u32;
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_animation_events_max_len(engine: &Engine) -> u32 {
        {
            let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
            let events = key_frames.get_animation_events();
            return (events.len() * 5) as u32;
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_animation_events(arr: &mut [u32], engine: &Engine) -> u32 {
        {
            let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
            let events = key_frames.get_animation_events();
            let map = key_frames.get_group_bind();
            let mut i = 0;
            for (group_id, ty, count) in events.iter() {
                match map.get(*group_id) {
                    Some(r) => {
                        ();
                        arr[i] = r.0.index();
                        arr[i + 1] = r.0.generation();
                        arr[i + 2] = r.1.1.get_hash() as u32;
                    }
                    None => continue,
                };
                arr[i + 3] = unsafe { transmute::<_, u8>(*ty) } as u32;
                arr[i + 4] = *count;
                i += 5;
            }
            i as u32
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_animation_events(arr: &mut [u32], engine: &Engine) -> u32 {
        {
            let key_frames = engine.world.get_resource::<KeyFramesSheet>().unwrap();
            let events = key_frames.get_animation_events();
            let map = key_frames.get_group_bind();
            let mut i = 0;
            for (group_id, ty, count) in events.iter() {
                match map.get(*group_id) {
                    Some(r) => {
                        ();
                        arr[i] = r.0.index();
                        arr[i + 1] = r.0.generation();
                        arr[i + 2] = r.1.1.get_hash() as u32;
                    }
                    None => continue,
                };
                arr[i + 3] = unsafe { transmute::<_, u8>(*ty) } as u32;
                arr[i + 4] = *count;
                i += 5;
            }
            i as u32
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_entity_offset(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.index()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_entity_offset(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.index()
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn get_entity_version(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.generation()
        }
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn get_entity_version(value: f64) -> u32 {
        {
            let r = unsafe { Entity::from_bits(transmute::<f64, u64>(value)) };
            r.generation()
        }
    }
    #[cfg(feature = "pi_js_export")]
    pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
        crate::index::query(engine, gui, x, y)
    }
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen]
    pub fn query(engine: &mut Engine, gui: &mut Gui, x: f32, y: f32) -> Option<f64> {
        crate::index::query(engine, gui, x, y)
    }
    pub fn to_linear_gradient_color(
        color_and_positions: &[f32],
        direction: f32,
    ) -> LinearGradientColor {
        let arr = color_and_positions;
        let len = arr.len();
        let count = len / 5;
        let mut list = Vec::with_capacity(count);
        for i in 0..count {
            let start = i * 5;
            let color_pos = ColorAndPosition {
                rgba: CgColor::new(
                    arr[start],
                    arr[start + 1],
                    arr[start + 2],
                    arr[start + 3],
                ),
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
    fn set_animation_str_inner(
        gui: &mut Gui,
        node_id: f64,
        value: &str,
        scope_hash: u32,
    ) {
        use pi_style::style_parse::parse_animation;
        let node_id = Entity::from_bits(unsafe { transmute(node_id) });
        let mut input = cssparser::ParserInput::new(value);
        let mut parse = cssparser::Parser::new(&mut input);
        let mut animations = match parse_animation(&mut parse) {
            Ok(r) => r,
            Err(e) => {
                ();
                return;
            }
        };
        animations.name.scope_hash = scope_hash as usize;
        ();
        if animations.name.value.len() > 0 {
            gui.commands.set_style(node_id, AnimationNameType(animations.name));
            gui.commands.set_style(node_id, AnimationDurationType(animations.duration));
            gui.commands
                .set_style(
                    node_id,
                    AnimationTimingFunctionType(animations.timing_function),
                );
            gui.commands
                .set_style(
                    node_id,
                    AnimationIterationCountType(animations.iteration_count),
                );
            gui.commands.set_style(node_id, AnimationDelayType(animations.delay));
            gui.commands
                .set_style(node_id, AnimationDirectionType(animations.direction));
            gui.commands.set_style(node_id, AnimationFillModeType(animations.fill_mode));
            gui.commands
                .set_style(node_id, AnimationPlayStateType(animations.play_state));
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
    pub mod debug {}
}

pub use self::style_macro::*;