use std::{mem::transmute, ops::Deref};
use pi_assets::asset::{Handle, Size};
use pi_bevy_render_plugin::PiRenderDevice;
use pi_export_base::export::DataTextureSubData;
use pi_hash::XHashMap;
use pi_scene_context::pass::{KeyAtlasDesc, KeyImageTextureFrame, TextureCombineCmds, WorldResourceTemp};
use pi_scene_shell::prelude::{ResImageTexture, KeyImageTexture};
pub use pi_export_base::{export::{Engine, Atom}, constants::*};
use pi_scene_shell::prelude::ResTextureCombineAtlas2DMgr;
// use pi_render::asset::TAssetKeyU64;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;
pub use crate::{constants::EngineConstants, mesh::CommandsExchangeD3};
pub use crate::engine::ActionSetScene3D;

pub struct CombineTextureAtlas {
    keys: Vec<Atom>,
    frames: Vec<(u32, u32, u32, u32)>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct DataTextureRes(Handle<ResImageTexture>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_data_texture(param: &mut CommandsExchangeD3, key: &Atom, width: f64, height: f64, format: f64, aspect: Option<f64>) {

    let key = key.deref().clone();
    let format = EngineConstants::texture_format(format);
    let width = width as u32;
    let height = height as u32;
    let dimension = wgpu::TextureViewDimension::D2;
    let is_opacity = true;
    let useage = wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING;
    let texkey = KeyImageTextureFrame { url: key.clone(), file: false, compressed: false, cancombine: false };

    let info = DataTextureSubData {
        data: None,
        dataoffset: 0,
        xoffset: 0,
        yoffset: 0,
        width, height,
        aspect: None,
        depth_or_array_layers: 0
    };
    param.datatexcmd.createdata.insert(key.clone(), (info, format, dimension, texkey));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_update_data_texture(param: &mut CommandsExchangeD3, key: &Atom, data:&[u8], xoffset: f64, yoffset: f64, width: f64, height: f64, aspect: Option<f64>) {

    let key = key.deref().clone();
    if param.datatexcmd.updatedata.contains_key(&key) == false {
        param.datatexcmd.updatedata.insert(key.clone(), vec![]);
    }
    if let Some(list) = param.datatexcmd.updatedata.get_mut(&key) {
        list.push(DataTextureSubData {
            data: Some(data.to_vec()),
            dataoffset: 0,
            xoffset: xoffset as u32,
            yoffset: yoffset as u32,
            width: width as u32, height: height as u32,
            aspect: None,
            depth_or_array_layers: 0
        });
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_remove_data_texture(param: &mut CommandsExchangeD3, key: &Atom) {
    let key = key.deref().clone();
    param.datatexcmd.createdata.remove(&key);
    param.datatexcmd.updatedata.remove(&key);
    param.datatexcmd.record.remove(&key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_texture_loader(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, isfile: bool, url: &Atom, cancombine: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
	pi_export_base::export::await_last_frame(app);

    let mut resource = param.resource.get_mut(&mut app.world);

    let key = KeyImageTextureFrame { url: url.deref().clone(), file: isfile, compressed, cancombine };
    let loader = resource.imgtex_loader.create_load(key);
    unsafe { transmute(loader) }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_texture_loader(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, loader: f64, info: &mut [u32]) {
	pi_export_base::export::await_last_frame(app);

    let mut resource = param.resource.get_mut(&mut app.world);

    let loader = unsafe { transmute(loader) };
    if let Some(success) = resource.imgtex_loader.query_success(loader) {
        info[0] = 1;
        info[1] = success.width();
        info[2] = success.height();
        info[3] = success.size() as u32;
    } else if let Some(fail) = resource.imgtex_loader.query_failed_reason(loader) {
        info[0] = 0;
        info[1] = fail;
    } else {
        info[0] = 2;
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_combine_add_frame(cmd: &mut CommandsExchangeD3, requestid: f64, targetkey: &Atom, file: &Atom, idx: f64, iscompress: bool, xoffset: f64, yoffset: f64, width: f64, height: f64) {
    let requestid = requestid as u32;
    let targetkey = targetkey.deref().clone();
    let file = file.deref().clone();
    if cmd.combinecmds.contains_key(&requestid) == false {
        cmd.combinecmds.insert(requestid, (targetkey.clone(), XHashMap::default()));
    }
    if let Some(cmd) = cmd.combinecmds.get_mut(&requestid) {
        cmd.1.insert(file, (requestid, idx as u16, iscompress, xoffset as u32, yoffset as u32, width as u32, height as u32));
    }
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_combine_remove(app: &mut Engine, cmd: &mut CommandsExchangeD3, requestid: f64, targetkey: &Atom) {
    pi_export_base::export::await_last_frame(app);
    cmd.combinecmds.remove(&(requestid as u32));
    app.world.get_resource_mut::<TextureCombineCmds>().unwrap().remove(requestid as u32, targetkey.deref().clone());
}
#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_combine_query(app: &mut Engine, success: &mut [u32], fail: &mut [u32]) {
    pi_export_base::export::await_last_frame(app);
    
    let cmds = app.world.get_resource_mut::<TextureCombineCmds>().unwrap();
    let mut idx = 0;
    cmds.successed().for_each(|id| {
        success[idx] = id;
        idx += 1;
    });
    let mut idx = 0;
    cmds.failed().for_each(|id| {
        fail[idx] = id;
        idx += 1;
    });
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_combine_param(app: &mut Engine, cmds: &mut CommandsExchangeD3, format: f64, maxlayer: f64, maxsize: f64, maxcount: f64) {
    pi_export_base::export::await_last_frame(app);
    let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
    let format = EngineConstants::texture_format(format);
    let cmds = app.world.get_resource_mut::<ResTextureCombineAtlas2DMgr>().unwrap();
    cmds.append_desc(KeyAtlasDesc { format }, &device, maxlayer as u32, maxsize as u32, maxcount as usize);
    // let loader = app.world.get_resource_mut::<pi_scene_shell::prelude::ImageTextureLoader>().unwrap();
    // loader.test.push(String::from("assets/qian_01.astc.ktx"));
    // loader.test.push(String::from("assets/plant1_0.astc.ktx"));
    // loader.test.push(String::from("assets/player_001.astc.ktx"));
    // loader.test.push(String::from("assets/plant3_0.astc.ktx"));
    // loader.test.push(String::from("assets/plant4_1.astc.ktx"));
    // loader.test.push(String::from("assets/plant4_0.astc.ktx"));
    // loader.test.push(String::from("assets/qian_03.astc.ktx"));
    // loader.test.push(String::from("assets/mutou_02.astc.ktx"));
    // loader.test.push(String::from("assets/meigui_1.astc.ktx"));
    // loader.test.push(String::from("assets/meigui_2.astc.ktx"));
    // loader.test.push(String::from("assets/qiezi_4.astc.ktx"));
    // loader.test.push(String::from("assets/qiezi_3.astc.ktx"));
    // loader.test.push(String::from("assets/meigui_4.astc.ktx"));
    // loader.test.push(String::from("assets/citiehua_2.astc.ktx"));
    // loader.test.push(String::from("assets/citiehua_3.astc.ktx"));
}

