use std::{mem::transmute, ops::DerefMut};

use super::{animation::{EAmountMode, EAnimationGroupListen, EAnimeCurve, EAnimePropertyID, EFillMode, ELoopMode}, as_entity, as_f64, engine::GLTFRes, mesh::{GeometryMeta, VInstanceAttributes}, node_materials::{MaterialUniformDefines, NodeMaterialBlock, NodematerialIncludes, P3DShaderVaryings}};
use pi_3d::TActionSet;
use pi_atom::Atom;
use pi_bevy_render_plugin::{PlayState, Records};
use pi_gltf2_load::{GLTFResLoader, ResGLTFRecords};
use pi_node_materials::NodeMaterialBlocks;
use pi_particle_system::prelude::{ECPUParticleSystemState, ParticleSystemPerformance};
use pi_scene_context::prelude::*;
use pi_scene_shell::prelude::*;
use pi_world::world::{Entity, World};
use serde::{Serialize, Deserialize};
use crate::{commands::CommandsExchangeD3, export::Engine};
use pi_hash::XHashMap;
use pi_bevy_render_plugin::ShareFontSheet;
use pi_render::font::FontId;

#[cfg(any(feature = "record", feature = "replay"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERecordCMD {
    Dispose(f64)                       ,
    SceneDispose(f64)                 ,
    SCENE(f64, f64, f64, [i32; 9])                         ,
    SceneOption(f64, ESceneOps),
    LAYERMASK(f64, f64)                     ,
    SceneBoundingbox(f64, bool, f64)             ,
    COLLIDER(f64,f64,f64,f64,f64,f64,f64,f64, Option<f64>)                      ,
    RenderGraphic(f64,f64,bool)                ,
    AnimationGroup(f64, f64)               ,
    AnimationGroupWeight(f64, f64)        ,
    AnimationGroupTargetReset(f64)  ,
    AnimationGroupStart(f64, f64, ELoopMode, Option<f64>, f64, f64, f64, EAmountMode, f64, EFillMode, f64, f64, f64, f64)         ,
    AnimationGroupPause(f64)         ,
    AnimationGroupStop(f64)          ,
    AnimationGroupGoto(f64, f64)          ,
    AnimationGroupRestart(f64)       ,
    AnimationGroupDelete(f64)        ,
    PropertyTargetAnimation(String, EAnimePropertyID, f64, f64)     ,
    TRANSFORMNODE(f64, f64)                 ,
    TransformnodeParent(f64, f64)          ,
    TransformnodeSRT(f64, ETransformSRT)         ,
    TransformnodeEnable(f64, bool)          ,
    TransformnodeQuaternion(f64, f64, f64, f64, f64)      ,
    TRAIL(f64, f64, f64)                         ,
    TrailAge(f64, f64)                     ,
    CAMERA(f64, f64, Option<f64>)                        ,
    CameraParam(f64, ECameraModify)                   ,
    CameraTarget(f64, f64, f64, f64)                 ,
    ViewerForceinclude(f64, f64, bool)           ,
    SPRITE(f64, f64, Atom)                        ,
    SpriteFrame(f64, Atom, f64)                  ,
    SpriteFrameData(f64, Atom, Vec<u16>)             ,
    InstanceMesh(f64, f64)                 ,
    InstanceAttr(f64, EInstanceAttr, Atom)                 ,
    MeshBoneOffset(f64, f64)              ,
    MESH(f64, f64, VInstanceAttributes, bool)                          ,
    MeshGeometry(f64, GeometryMeta, f64)                 ,
    MeshValueState(f64, EMeshValueStateModify)               ,
    MeshRenderState(f64, ERenderState)                    ,
    MeshState(f64, EMeshStateModify)              ,
    MeshBoundingBox(f64, f64, f64, f64, f64, f64, f64)             ,
    MeshAttributeTargetAnim(f64, f64, Atom, String)    ,
    MeshPoseMatrix(f64, Vec<f32>)              ,
    LIGHT(f64, f64, f64)                         ,
    LightParam(f64, ELightModify)               ,
    MeshForceIndludInLight(f64, f64, EMeshForceLighting)        ,
    MaterialShader(f64, Atom, bool)               ,
    MaterialApply(f64, f64, f64)                ,
    MaterialUniformMat4(f64, Atom, [f32;16])         ,
    MaterialUniformV0(f64, EUniformVal)         ,
    MaterialUniformTex(f64, Atom, Atom, bool, bool, bool, f64, f64, f64, f64, f64, f64, f64, f64, bool, bool, Option<f64>)          ,
    LoadTexture(Atom, bool, bool, bool)                  ,
    MaterialTexFromRendertarget(f64, Atom, Atom, f64, bool, f64, f64, f64, f64, f64, f64, f64, f64, Option<f64>),
    MaterialTexFromRenderer(f64, Atom, Atom, f64, bool, f64, f64, f64, f64, f64, f64, f64, f64, Option<f64>)    ,
    MaterialTargetAnimation(f64, f64, Atom, String)     ,
    PARTICLESYS(f64, f64, f64, f64, u64, Atom, Atom, Option<f64>)                   ,
    ParticlesysState(f64, ECPUParticleSystemState)             ,
    RenderSubgraph(f64, String)               ,
    RENDER(f64, f64, String, f64, bool, Option<bool>, Option<bool>)                        ,
    RenderModify(f64, ERendererCommand)                 ,
    RenderTarget(f64, ERendererTarget)             ,
    ShadowGenerator(f64, f64, f64, f64, Option<f64>)              ,
    ShadowParam(f64, EShadowGeneratorParam)             ,
    SKELETON(f64, f64, f64, Vec<f64>, f64, f64)                      ,
    BONE(f64, f64)                          ,
    BoneLink(f64, f64)                     ,
    BonePose(f64, Vec<f32>)                     ,
    SkinUse(f64, f64)                      ,
}

#[cfg(any(feature = "record", feature = "replay"))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERecord3D {
    EngineState(bool)                  ,
    EngineDebug(bool)                  ,
    CreateGltfLoad(f64, Atom, String)  ,
    SpriteFrameDataRecord(u32, SpriteFrame)      ,
    LightingShadowLimit(f64,f64,f64,f64,f64,f64,f64,f64,f64)         ,
    DisposeGltf(GLTFRes)              ,
    CreateImageLoad(KeyImageTextureFrame)             ,
    CreateRenderTarget(Option<f64>,f64,f64,f64,f64,f64,f64,f64)          ,
    DisposeRenderTarget(f64)         ,
    CreateAnimationCurve(String, EAnimePropertyID, Vec<f32>, EAnimeCurve)        ,
    MaterialRegist(String, MaterialUniformDefines, String, String, String, String, NodematerialIncludes, String, P3DShaderVaryings, Option<f64>)               ,
    NodeMaterialBlockRegist(NodeMaterialBlock)    ,
    RenderLinkDrawlist(f64, Vec<f64>)          ,
    RenderScreenWithPostprocess(bool),
    CreateVertexBuffer(String, Vec<u8>)          ,
    CreateIndiceBuffer(String, Vec<u8>)          ,
    FontId(String, usize, usize)          ,
    FontChar(FontId, char, f32, f32)          ,
    CMD(ERecordCMD),
}

#[derive(Default, Clone, Resource)]
pub struct Records3D {
    a: FontId,
    pub(crate) replayrendertargetkey: XHashMap<Entity, Entity>,
}

#[cfg(feature = "replay")]
pub fn cmd_play_call_3d(world: &mut World, data: &Vec<u8>, replayentities: &XHashMap<Entity, Entity>) {
    // return;
    match postcard::from_bytes::<Vec<ERecord3D>>(data) {
        Ok(mut val) => {
            // log::error!("cmd_play_call_3d {:?}", &val);
            val.drain(..).for_each(|e| {
                match e {
                    ERecord3D::EngineState(val) => world.get_resource_mut::<EngineCustomPlugins>().unwrap().active = val,
                    ERecord3D::EngineDebug(val) => {
                        world.get_resource_mut::<Performance>().unwrap().debug = val;
                        world.get_resource_mut::<ParticleSystemPerformance>().unwrap().debug = val;
                    },
                    ERecord3D::CreateGltfLoad(entity, baseurl, dyndesc) => {
                        if let Some(entity) = world.get_resource::<PlayState>().unwrap().node_map.get(&as_entity(entity)) {
                            let entity = *entity;
                            world.get_resource_mut::<GLTFResLoader>().unwrap().create_load(entity, baseurl);
                        }
                    },
                    ERecord3D::DisposeGltf(gltfres) => {
                        if let Some(entity) = world.get_resource::<PlayState>().unwrap().node_map.get(&gltfres.0) {
                            let entity = *entity;
                            world.get_resource_mut::<ResGLTFRecords>().unwrap().0.remove(&entity);
                        }
                    },
                    ERecord3D::SpriteFrameDataRecord(idx, sprite_frame) => {
                        let sprites = world.get_resource_mut::<ResSpriteFrames>().unwrap();
                        sprites.insert(idx as usize, sprite_frame);
                    },
                    ERecord3D::LightingShadowLimit(v0, v1, v2, v3, v4, v5, v6, v7, v8) => {
                        let limit = world.get_resource_mut::<SceneLightLimit>().unwrap();
                        limit.0.max_direct_light_count = v0 as u16;
                        limit.0.max_point_light_count = v1 as u16;
                        limit.0.max_spot_light_count = v2 as u16;
                        limit.0.max_hemi_light_count = v3 as u16;
                        let limit = world.get_resource_mut::<SceneShadowLimit>().unwrap();
                        limit.0.max_count = v4 as u16;
                        let limit = world.get_resource_mut::<ModelLightLimit>().unwrap();
                        limit.0.max_direct_light_count = v5 as u16;
                        limit.0.max_point_light_count = v6 as u16;
                        limit.0.max_spot_light_count = v7 as u16;
                        limit.0.max_hemi_light_count = v8 as u16;

                    },
                    ERecord3D::CreateImageLoad(key) => {
                        let loader = world.get_resource_mut::<ResImageTextureLoader>().unwrap();
                        loader.create_load(key);
                    },
                    ERecord3D::CreateRenderTarget(key, color_format, depth_stencil_format, width, height, filter, address, anisotropy_clamp) => {
                        let render_targets = world.get_resource_mut::<CustomRenderTargets>().unwrap();
                        let newkey = CommandsExchangeD3::p3d_create_render_target(render_targets, color_format, depth_stencil_format, width, height, filter, address, anisotropy_clamp);
                        if let (Some(newkey), Some(key)) = (newkey, key) {
                            let cmds = world.get_resource_mut::<Records3D>().unwrap();
                            cmds.replayrendertargetkey.insert(as_entity(key), as_entity(newkey));
                        }
                    },
                    ERecord3D::DisposeRenderTarget(key) => {
                        let cmds = world.get_resource_mut::<Records3D>().unwrap();
                        if let Some(newkey) = cmds.replayrendertargetkey.remove(&as_entity(key)) {
                            let gltfs = world.get_resource_mut::<ResGLTFRecords>().unwrap();
                            gltfs.0.remove(&newkey);
                        }
                    },
                    ERecord3D::CreateAnimationCurve(key, property, data, mode) => {
                        let key = pi_atom::Atom::from(&key).asset_u64();
                        CommandsExchangeD3::p3d_anime_curve_create2(world, key, property, &data, mode);
                    },
                    ERecord3D::MaterialRegist(key, uniforms, vs_define_code, fs_define_code, vs_code, fs_code, includes, instance_code, varyings, binds_defines_base) => {
                        CommandsExchangeD3::p3d_regist_material(world, key.as_ref(), &uniforms, vs_define_code.as_ref(), fs_define_code.as_ref(), vs_code.as_ref(), fs_code.as_ref(), &includes, instance_code.as_ref(), &varyings, binds_defines_base);
                    },
                    ERecord3D::NodeMaterialBlockRegist(block) => {
                        let node_material_blocks = world.get_resource_mut::<NodeMaterialBlocks>().unwrap();
                        node_material_blocks.0.insert(block.v0().clone(), block.v1().clone());
                    },
                    ERecord3D::RenderLinkDrawlist(linkentity, drawlistrenderers) => {
                        let replayentities = &world.get_resource::<PlayState>().unwrap().node_map;
                        let mut list = vec![];
                        drawlistrenderers.iter().for_each(|v| {
                            let entity: Entity = CommandsExchangeD3::entity(replayentities, *v);
                            list.push(entity);
                        });
                        let link: Entity = CommandsExchangeD3::entity(replayentities, linkentity);

                        let crossrenderinfos = world.get_resource_mut::<pi_bevy_render_plugin::render_cross::CrossRenderDrawListEntities>().unwrap();
                        if list.len() > 0 {
                            crossrenderinfos.0.insert(link, list);
                        } else {
                            crossrenderinfos.0.remove(&link);
                        }
                    },
                    ERecord3D::RenderScreenWithPostprocess(val) => {
                        let screenwithpostprocess = world.get_resource_mut::<pi_bevy_render_plugin::ScreenWithPostprocess>().unwrap();
                        screenwithpostprocess.0 = val;
                    },
                    ERecord3D::CreateVertexBuffer(key, data) => {
                        let actions = world.get_resource_mut::<ActionListCustomBuffer>().unwrap();
                        let key = KeyVertexBuffer::from(key.as_str());
                        actions.push((key, data, false));
                    }
                    ERecord3D::CreateIndiceBuffer(key, data) => {
                        let actions = world.get_resource_mut::<ActionListCustomBuffer>().unwrap();
                        let key = KeyVertexBuffer::from(key.as_str());
                        actions.push((key, data, true));
                    },
                    ERecord3D::FontId(key, fontsize, fontweight) => {
                        let fontsheet = world.get_resource_mut::<ShareFontSheet>().unwrap();
                        let fontid = fontsheet.borrow_mut().font_id(pi_render::font::Font::new(pi_atom::Atom::from(key), fontsize, fontweight));
                    },
                    ERecord3D::FontChar(f, char, fontsize, line_height) => {
                        let mut scaleoffset = [f32;4];
                        let mut uvtilloff = [f32;4];
                        let fontsheet = world.get_resource_mut::<ShareFontSheet>().unwrap();
                        let mut fsheet = fontsheet.borrow_mut();
                        if let Some(id) = fsheet.glyph_id(f, char) {
                            fsheet.measure_width(f, char);
                            ShareFontSheet::char_calc(&mut fsheet, f, char, &mut scaleoffset, &mut uvtilloff, line_height, fontsize, 1., 32., 32., 1.);
                        }
                    },
                    ERecord3D::CMD(cmd) => {
                        CommandsExchangeD3::replay(world, cmd, replayentities);
                    },
                }
            });
        },
        Err(_) => {},
    }
}
