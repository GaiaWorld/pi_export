
use std::{num::NonZeroU32, mem::transmute};

use bevy::{prelude::{Deref, DerefMut, App, Commands}, ecs::system::CommandQueue};
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_bevy_render_plugin::{PiRenderGraph, PiRenderDevice, PiRenderQueue, GraphError, component::GraphId};
use pi_export_base::{export::Engine, constants::SamplerDescriptor};
use pi_final_render_target::FinalRenderTarget;
use pi_hal::image::{load_from_url, DynamicImage};
use pi_render::{rhi::{asset::TextureRes, device::RenderDevice, RenderQueue}, renderer::sampler::SamplerRes, asset::TAssetKeyU64};

use pi_export_base::constants::{TextureFormat, BlendFactor};
use pi_spine_rs::{shaders::{KeySpineShader}, KeySpineRenderer, SpineRenderContext, SingleSpineCommands, SAMPLER_DESC, SpineRenderNode, ActionSpine};
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub struct SpineTextureSize(pub u32, pub u32);
impl SpineTextureSize {
    pub fn val(val: Option<&Self>) -> Option<(u32, u32)> {
        match val {
            Some(val) => Some((val.0, val.1)),
            None => None,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_renderer_create(app: &mut Engine, name: String, rendersize: Option<SpineTextureSize>) -> f64 {
    let id_renderer = {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);
        let mut entitycmd = commands.spawn_empty();
        let id_renderer = KeySpineRenderer(entitycmd.id());
        queue.apply(&mut app.world);
    
        let final_render_format = app.world.get_resource::<FinalRenderTarget>().unwrap().format();
        let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
        ActionSpine::create_spine_renderer(id_renderer, SpineTextureSize::val(rendersize.as_ref()), &mut ctx, final_render_format);
        
        id_renderer
    };

    let f = {

        let mut render_graph = app.world.get_resource_mut::<PiRenderGraph>().unwrap();
        match ActionSpine::spine_renderer_apply(id_renderer, pi_atom::Atom::from(name), rendersize.is_none(), &mut render_graph) {
            Ok(nodeid) => {
                let mut queue = CommandQueue::default();
                let mut commands = Commands::new(&mut queue, &app.world);
                commands.entity(id_renderer.0).insert(GraphId(nodeid));
                queue.apply(&mut app.world);
            },
            Err(_) => {},
        }
    
    
        unsafe { transmute(id_renderer.0.to_bits()) }
    
    };

    f
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_renderer_dispose(app: &mut Engine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);

    let mut queue = CommandQueue::default();
    let mut commands = Commands::new(&mut queue, &app.world);
    commands.entity(id_renderer.0).despawn();
    queue.apply(&mut app.world);

    let mut ctx = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    ActionSpine::dispose_spine_renderer(id_renderer, &mut ctx);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub async fn spine_texture_load(app: &mut Engine, id_renderer: f64, key: String) -> Result<SpineTextureSize, String> {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let device = app.world.get_resource::<PiRenderDevice>().unwrap().clone();
    let queue = app.world.get_resource::<PiRenderQueue>().unwrap().clone();
    let asset_textures = app.world.get_resource::<ShareAssetMgr<TextureRes>>().unwrap();

    let key_atom = Atom::from(key.clone());

    let key_u64 = key.asset_u64();
    let texture = if let Some(textureres) = asset_textures.get(&key_u64) {
        textureres
    } else {
        let image = load_from_url(&key_atom).await;
        let image = match image {
            Ok(r) => r,
            Err(_e) =>  {
                return Err(String::from("spine_texture_load Fail: ") + key.as_str());
            },
        };

        let texture = create_texture_from_image(&image, device, queue, &key_atom);

        if let Some(texture) = asset_textures.insert(key_u64, texture) {
            texture
        } else {
            return Err(String::from("spine_texture_load Fail While Insert: ") + key.as_str());
        }
    };

    let width = texture.width;
    let height = texture.height;

    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().record_texture(key_u64, texture);
    }

    Ok(SpineTextureSize(width, height))
}


#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_remove_texture(app: &mut Engine, id_renderer: f64, key: String) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().remove_texture(key.asset_u64());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_use_texture(app: &mut Engine, id_renderer: f64, key: String, samplerdesc: SamplerDescriptor) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let device = app.world.get_resource::<PiRenderDevice>().unwrap().clone();
    let queue = app.world.get_resource::<PiRenderQueue>().unwrap().clone();
    let asset_samplers = app.world.get_resource::<ShareAssetMgr<SamplerRes>>().unwrap();

    let samplerdesc = samplerdesc.val();
    let sampler = if let Some(sampler) = asset_samplers.get(&samplerdesc) {
        sampler
    } else {
        if let Some(sampler) = asset_samplers.insert(samplerdesc.clone(), SamplerRes::new(&device, &samplerdesc)) {
            sampler
        } else {
            return;
        }
    };
    let mut renderers = app.world.get_resource_mut::<SpineRenderContext>().unwrap();
    if let Some(renderer) = renderers.get_mut(id_renderer) {
        // log::warn!("Cmd: Texture");
        renderer.render_mut().record_sampler(samplerdesc, sampler);
    }

    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_use_texture(&mut cmds.0, id_renderer, key.asset_u64(), samplerdesc)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_shader(app: &mut Engine, id_renderer: f64, shader: KeySpineShader) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_shader(&mut cmds.0, id_renderer, shader);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_blend(app: &mut Engine, id_renderer: f64, val: bool) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_blend(&mut cmds.0, id_renderer, val);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_blend_mode(app: &mut Engine, id_renderer: f64, src: BlendFactor, dst: BlendFactor) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_blend_mode(&mut cmds.0, id_renderer, src.val(), dst.val());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_uniform(app: &mut Engine, id_renderer: f64, val: &[f32]) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_uniform(&mut cmds.0, id_renderer, val);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_draw(app: &mut Engine, id_renderer: f64, vertices: &[f32], indices: &[u16], vlen: u32, ilen: u32) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_draw(&mut cmds.0, id_renderer, vertices, indices, vlen, ilen);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(feature = "pi_js_export")]
pub fn spine_reset(app: &mut Engine, id_renderer: f64) {
    let id_renderer = KeySpineRenderer::from_f64(id_renderer);
    let mut cmds = app.world.get_resource_mut::<SingleSpineCommands>().unwrap();
    ActionSpine::spine_reset(&mut cmds.0, id_renderer);
}

pub fn create_texture_from_image(
	image: &DynamicImage, 
	device: &RenderDevice, 
	queue: &RenderQueue,
	key: &Atom,
) -> TextureRes {
	let buffer_temp;
	// let buffer_temp1;
	let (width, height, buffer, ty, pre_pixel_size, is_opacity) = match image {
		DynamicImage::ImageLuma8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::R8Unorm, 1, true),
		DynamicImage::ImageRgb8(r) => {
			buffer_temp =  image.to_rgba8();
			(r.width(), r.height(), buffer_temp.as_raw(), wgpu::TextureFormat::Rgba8Unorm, 4, true)
		},
		DynamicImage::ImageRgba8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Rgba8Unorm, 4, false),
		// DynamicImage::ImageBgr8(r) => {
		// 	buffer_temp1 =  image.to_bgra8();
		// 	(r.width(), r.height(), buffer_temp1.as_raw(), wgpu::TextureFormat::Bgra8Unorm, 4, true)
		// },
		// DynamicImage::ImageBgra8(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8Unorm, 4, false),

		_ => panic!("不支持的图片格式"),

		// DynamicImage::ImageLumaA8(image) => panic!("不支持的图片格式: DynamicImage::ImageLumaA8"),
		// DynamicImage::ImageLuma16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8Unorm),
		// DynamicImage::ImageLumaA16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8Unorm),

		// DynamicImage::ImageRgb16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8Unorm),
		// DynamicImage::ImageRgba16(image) => (image.width(), image.height(), image.as_raw(), wgpu::TextureFormat::Bgra8Unorm),
	};
	let texture_extent = wgpu::Extent3d {
		width,
		height,
		depth_or_array_layers: 1,
	};
	let byte_size = buffer.len();

	let texture = (**device).create_texture(&wgpu::TextureDescriptor {
		label: Some("first depth buffer"),
		size: texture_extent,
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: ty,
		usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
		view_formats: &[],
	});
	let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

	queue.write_texture(
		texture.as_image_copy(),
		buffer,
		wgpu::ImageDataLayout {
			offset: 0,
			bytes_per_row: Some(std::num::NonZeroU32::new(width * pre_pixel_size).unwrap()),
			rows_per_image: None,
		},
		texture_extent,
	);

	TextureRes::new(width, height, byte_size, texture_view, is_opacity)
}