
use pi_scene_context::pass::{ESkinBonesPerVertex, WorldResourceTemp};
use pi_scene_context::prelude::*;
use pi_scene_shell::prelude::*;
pub use pi_export_base::{export::{Engine, Atom}, constants::* };

use crate::as_f64;
pub use crate::commands::CommandsExchangeD3;
pub use crate::{engine::ActionSetScene3D, as_entity};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_sprite(app: &mut Engine, cmds: &mut CommandsExchangeD3, source: f64, atlas: &Atom) -> f64 {
    let id: Entity = app.world.entities().reserve_entity();

    let source = as_entity(source);

    cmds.instance_create.push(OpsInstanceMeshCreation::ops(source, id));
    cmds.sprite_create.push(OpsSpriteCreate::ops(source, id, atlas.to_string().asset_u64()));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_sprite_frame(cmds: &mut CommandsExchangeD3, sprite: f64, idxframe: f64) {
    cmds.sprite_modify.push(OpsSpriteModify::ops(as_entity(sprite), SpriteModify::Idx(idxframe as IdxTextureFrame)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_sprite_frame_data(cmds: &mut CommandsExchangeD3, sprite: f64, data: &[u16]) {
    let data = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
        data[10], data[11], data[12], data[13],
    ];
    cmds.sprite_modify.push(OpsSpriteModify::ops(as_entity(sprite), SpriteModify::Data(data)));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct PTextureFrameAtlas(TextureFrameAtlas);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ResTextureFrameAtlas(Handle<TextureFrameAtlas>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_frame_atlas(atlas: &Atom, width: f64, height: f64) -> PTextureFrameAtlas {
    let mut result = PTextureFrameAtlas(TextureFrameAtlas::new(String::from(atlas.to_string())));

    result.0.width = width as u16;
    result.0.height = height as u16;

    return result;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_frame_atlas_append_frame(atlas: &mut PTextureFrameAtlas, frame_name: &Atom, framedata: &[u16]) -> f64 {
    let frame: TextureFrame = TextureFrame::from_data(framedata);
    let idxframe = atlas.0.append_frame(frame_name.to_string(), frame);
    return idxframe as f64;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_frame_atlas_append_animation(atlas: &mut PTextureFrameAtlas, anim_name: &Atom, animframeidxs: &[u16]) -> f64 {
    let idxanime = atlas.0.append_animation(anim_name.to_string(), animframeidxs.to_vec());

    return idxanime as f64;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_frame_atlas_cache(app: &mut Engine, atlas: &PTextureFrameAtlas) -> Option<ResTextureFrameAtlas> {
    if let Some(atlasmgr) = app.world.get_resource_mut::<TextureFrameAtlasManager>() {
        let key = atlas.0.image.asset_u64();
        if let Ok(result) = atlasmgr.insert(key, atlas.0.clone()) {
            return Some(ResTextureFrameAtlas(result));
        }
    }

    return None;
}
