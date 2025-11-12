
use std::ops::Deref;

use pi_bevy_render_plugin::ShareFontSheet;
use pi_export_base::{as_dk, as_f64_dk};
use pi_render::font::FontId;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

#[cfg(any(feature = "record", feature = "replay"))]
use crate::record::ERecordCMD;
use crate::{constants::EngineConstants};
pub use crate::commands::CommandsExchangeD3;
pub use crate::{as_entity, as_f64};
pub use pi_export_base::{export::{Engine, Atom}, constants::*};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh(app: &mut Engine, cmds: &mut CommandsExchangeD3, source: f64) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());

    let id: Entity = CommandsExchangeD3::p3d_entity(app);

    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceMesh(source, as_f64(&id)));

    let source: Entity = as_entity(source);
    CommandsExchangeD3::p3d_instance_mesh(cmds, source, id);

    // cmds.instance_create.push(OpsInstanceMeshCreation::ops(source, id));

    as_f64(&id)
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_vec4(cmds: &mut CommandsExchangeD3, instance: f64, uscale: f64, vscale: f64, uoffset: f64, voffset: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::Vec4([uscale as f32, vscale as f32, uoffset as f32, voffset as f32]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));
    
    let instance: Entity = as_entity(instance);


    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_vec3(cmds: &mut CommandsExchangeD3, instance: f64, r: f64, g: f64, b: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;


    let attr = EInstanceAttr::Vec3([r as f32, g as f32, b as f32]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_vec2(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, y: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;


    let attr = EInstanceAttr::Vec2([x as f32, y as f32]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);

    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_float(cmds: &mut CommandsExchangeD3, instance: f64, val: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;


    let attr = EInstanceAttr::Float(val as f32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);

    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_sint(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::Int(x as i32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_uint(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::Uint(x as u32);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_ivec4(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, y: f64, z: f64, w: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::IVec4([x as i32, y as i32, z as i32, w as i32]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_u16x2(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, y: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::U16x2([x as u16, y as u16]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_u16x4(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, y: f64, z: f64, w: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::U16x4([x as u16, y as u16, z as u16, w as u16]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_instance_mesh_u8x4(cmds: &mut CommandsExchangeD3, instance: f64, x: f64, y: f64, z: f64, w: f64, key: &Atom) {
    #[cfg(feature = "replay")]
    return ;

    
    let attr = EInstanceAttr::U8x4([x as u8, y as u8, z as u8, w as u8]);

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::InstanceAttr(instance, attr, key.deref().clone()));

    let instance: Entity = as_entity(instance);
    
    CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key.deref().clone());
    // cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key.deref().clone() ));
    // cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));
}
// #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
// #[pi_js_export]
// pub fn p3d_instance_mesh_alpha_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
//     // let instance: Entity = as_entity(instance);    // cmds.instancemeshcmds_alpha.push(OpsInstanceAlpha::ops(instance, val as f32));

//     let len = len as usize;
//     let size = 2;
//     let count = len / size;
//     for i in 0..count {
//         let instance: Entity = as_entity(data[i * size + 0]);
//         let alpha = data[i * size + 1];
//         cmds.instance_alpha.push(OpsInstanceAlpha::ops(instance, alpha as f32));
//     }
// }

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bone_offset(cmds: &mut CommandsExchangeD3, instance: f64, val: f64) {
    #[cfg(feature = "replay")]
    return ;


    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::MeshBoneOffset(instance, val));

    let instance: Entity = as_entity(instance);
    CommandsExchangeD3::p3d_mesh_bone_offset(cmds, instance, val as u32);
    // cmds.mesh_valuestate.push(OpsAbstructMeshValueStateModify::ops(instance, EMeshValueStateModify::BoneOffset(val as u32)));
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_bone_offset_arr(cmds: &mut CommandsExchangeD3, data: &[f64], len: f64) {
    #[cfg(feature = "replay")]
    return ;

    // let instance: Entity = as_entity(instance);    // cmds.abstructmeshcmds_boneoffset.push(OpsBoneOffset::ops(instance, val as u32));

    let len = len as usize;
    let size = 2;
    let count = len / size;
    for i in 0..count {
        let instance = data[i * size + 0];
        let val = data[i * size + 1];

        #[cfg(feature = "record")]
        cmds.record(ERecordCMD::MeshBoneOffset(instance, val));
        
        let instance: Entity = as_entity(data[i * size + 0]);
        CommandsExchangeD3::p3d_mesh_bone_offset(cmds, instance, val as u32);
        // cmds.mesh_valuestate.push(OpsAbstructMeshValueStateModify::ops(instance, EMeshValueStateModify::BoneOffset(val as u32)));
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_sdf_font_id(app: &mut Engine, cmds: &mut CommandsExchangeD3, fontname: &str, fontsize: f64, fontweight: f64) -> f64 {
    #[cfg(feature = "replay")]
    return ;


    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::Font(fontname.to_string(), fontsize as usize, fontweight as usize));

    let _fontname = pi_atom::Atom::from(fontname);
    let _fontsize = fontsize as usize;
    let _fontweight = fontweight as usize;

    let fontsheet = app.world.get_resource_mut::<ShareFontSheet>().unwrap();
    let fontid = fontsheet.borrow_mut().font_id(pi_render::font::Font::new(_fontname, _fontsize, _fontweight));
    as_f64_dk(&fontid.0)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_sdf_char_glyphid(app: &mut Engine, cmds: &mut CommandsExchangeD3, font: f64, char: &str, fontsize: f64, line_height: f64, scaleoffset: &mut [f32], uvtilloff: &mut [f32]) -> Option<f64> {
    #[cfg(feature = "replay")]
    return ;


    let fontsheet = app.world.get_resource_mut::<ShareFontSheet>().unwrap();
    let mut fsheet = fontsheet.borrow_mut();
    let char = char.chars().next();
    if let Some(char) = char {
    let f = FontId(as_dk(&font));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::FontChar(f.clone(), char.clone(), fontsize as f32, line_height as f32));

    if let Some(id) = fsheet.glyph_id(f, char) {
        fsheet.measure_width(f, char);
        ShareFontSheet::char_calc(&mut fsheet, f, char, scaleoffset, uvtilloff, line_height as f32, fontsize as f32, 1., 32., 32., 1.);
        Some(as_f64_dk(&id.0))
    } else {
        None
    }
    } else {
        None
    }
}
