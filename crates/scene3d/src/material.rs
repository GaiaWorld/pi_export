
use pi_engine_shell::prelude::*;
use pi_export_base::{export::Engine, constants::{ColorFormat, DepthFormat}};
use pi_scene_context::{
    prelude::*,
};

use crate::{engine::ActionSetScene3D, as_entity, as_f64, geometry::GeometryMeta};
use wasm_bindgen::prelude::wasm_bindgen;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub enum OpsPass {
    ShadowCast,
    Opaque,
    Sky,
    Water,
    AlphaTest,
    Transparent,
    OpaqueExtend,
    TransparentExtend,
}
impl OpsPass {
    pub fn val(&self) -> EPassTag {
        match self {
            OpsPass::ShadowCast => EPassTag::ShadowCast,
            OpsPass::Opaque => EPassTag::Opaque,
            OpsPass::Sky => EPassTag::Sky,
            OpsPass::Water => EPassTag::Water,
            OpsPass::AlphaTest => EPassTag::AlphaTest,
            OpsPass::Transparent => EPassTag::Transparent,
            OpsPass::OpaqueExtend => EPassTag::OpaqueExtend,
            OpsPass::TransparentExtend => EPassTag::TransparentExtend,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_material(app: &mut Engine, param: &mut ActionSetScene3D, shader: &pi_export_base::export::Atom, pass: OpsPass) -> f64 {
    let id: Entity = app.world.spawn_empty().id();
    
    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);
    cmds.matcmd.create.push(OpsMaterialCreate::ops(id, shader.as_str(), pass.val()));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_material_shader(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64, shader: &pi_export_base::export::Atom, pass: OpsPass) {
    let mat: Entity = as_entity(mat);
    
    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);
    cmds.matcmd.create.push(OpsMaterialCreate::ops(mat, shader.as_str(), pass.val()));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn p3d_material_apply(app: &mut Engine, param: &mut ActionSetScene3D, mat: f64, mesh: f64) {
    let mat: Entity = as_entity(mat);
    let mesh: Entity = as_entity(mesh);

    let mut cmds: crate::engine::ActionSets = param.0.get_mut(&mut app.world);

    cmds.matcmd.usemat.push(OpsMaterialUse::ops(mesh, mat));
}