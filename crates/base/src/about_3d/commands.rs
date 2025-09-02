use std::{mem::{replace, transmute}, ops::{Deref, Range}};

use pi_3d::{ActionSets, ResourceSets, TActionSet};
use pi_bevy_render_plugin::{PlayState, Records, RECORD_D3_COMMAND};
use pi_gltf2_load::{ResGLTFRecords, GLTF};
use pi_scene_shell::prelude::*;
pub use crate::export::Engine;
use pi_particle_system::prelude::*;
use pi_scene_context::prelude::*;
use pi_trail_renderer::*;
use pi_hash::XHashMap;
use pi_curves::curve::frame::KeyFrameCurveValue;
pub use super::engine::ActionSetScene3D;
use super::{animation::{EAmountMode, EFillMode, ELoopMode}, as_entity, as_f64, cmd_call::_amountcalc, engine::{GLTFRes, GlobalState}, mesh::{GeometryMeta, VInstanceAttributes}, node_materials::{MaterialUniformDefines, NodeMaterialBlock, NodematerialIncludes, P3DShaderMeta, P3DShaderVaryings}, record::{ERecordCMD}};
use super::animation::EAnimePropertyID;
use super::animation::EAnimeCurve;
use super::animation::curve;
use super::constants::EngineConstants;
use crate::{constants::ContextConstants, record::ERecord3D};
use pi_mesh_builder::{quad::QuadBuilder, cube::CubeBuilder};
use pi_node_materials::prelude::NodeMaterialBuilder;
use pi_node_materials::NodeMaterialBlocks;
use pi_slotmap::Key;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Default)]
pub struct CommandsExchangeD3 {
    pub(crate) recordframes: Vec<ERecord3D>,
    pub(crate) replayrendertargetkey: XHashMap<Entity, Entity>,

    pub(crate) scene_create: ActionListSceneCreate,
    pub(crate) scene_options: ActionListSceneOption,
    pub(crate) scene_dispose: ActionListSceneDispose,
    pub(crate) scene_boundingbox: ActionListBoundingBoxDisplay,
    pub(crate) scene_collider: ActionListCollider,

    pub(crate) obj_dispose: ActionListDispose,
    
    pub(crate) transform_create: ActionListTransformNodeCreate,
    pub(crate) transform_localsrt: ActionListTransformNodeLocal,
    pub(crate) transform_localrotq: ActionListTransformNodeLocalRotationQuaternion,
    pub(crate) transform_tree: ActionListTransformNodeParent,
    pub(crate) transform_enable: ActionListNodeEnable,

    pub(crate) camera_create: ActionListCameraCreate,
    pub(crate) camera_param: ActionListCameraModify,
    pub(crate) camera_target: ActionListCameraTarget,
    pub(crate) camera_forceinclude: ActionListViewerForceInclude,

    pub(crate) mesh_create: ActionListMeshCreate,
    // pub(crate) mesh_shadow: ActionListMeshShadow,
    pub(crate) mesh_render_state: ActionListRenderState,
    pub(crate) mesh_pose: ActionListAbstractMeshPose,
    pub(crate) mesh_state: ActionListMeshStateModify,
    pub(crate) mesh_valuestate: ActionListAbstructMeshValueStateModify,
    pub(crate) mesh_bounding: ActionListMeshBounding,
    pub(crate) forcelighting: ActionListMeshForceLighting,

    pub(crate) skin_create: ActionListSkinCreate,
    pub(crate) skin_use: ActionListSkinUse,
    pub(crate) skin_bonecreate: ActionListBoneCreate,
    pub(crate) skin_bonepose: ActionListBonePose,

    pub(crate) mesh_layermask: ActionListLayerMask,
    
    pub(crate) instance_create: ActionListInstanceMeshCreate,
    pub(crate) instance_attr: ActionListInstanceAttr,
    pub(crate) instance_targetanime: ActionListTargetAnimationAttribute,

    pub(crate) geometry_create: ActionListGeometryCreate,
    
    pub(crate) material_usemat: ActionListMaterialUse,
    pub(crate) material_create: ActionListMaterialCreate,
    pub(crate) material_val: ActionListUniformVal,
    pub(crate) material_valb: ActionListUniformValB,
    
    pub(crate) light_create: ActionListLightCreate,
    pub(crate) light_param: ActionListLightParam,
    
    pub(crate) shadow_param: ActionListShadowGeneratorParam,
    pub(crate) shadow_create: ActionListShadowGenerator,
    
    pub(crate) renderer_subgraph: ActionListSubGraphCreate,
    pub(crate) renderer_create: ActionListRendererCreate,
    pub(crate) renderer_connect: ActionListRendererConnect,
    pub(crate) renderer_modify: ActionListRendererModify,
    pub(crate) renderer_target: ActionListRendererTarget,
    
    pub(crate) anime_create: ActionListAnimeGroupCreate,
    pub(crate) anime_action: ActionListAnimationGroupAction,
    pub(crate) anime_dispose: ActionListAnimeGroupDispose,
    pub(crate) anime_reset_while_start: ActionListAnimeGroupStartReset,
    pub(crate) anime_property_targetanime: ActionListPropertyTargetAnimation,
    pub(crate) anime_goto: ActionListAnimationGroupGoto,
    pub(crate) anime_float: ActionListAnimatorableFloat,
    pub(crate) anime_sint: ActionListAnimatorableSint,
    pub(crate) anime_uint: ActionListAnimatorableUint,
    pub(crate) anime_vec2: ActionListAnimatorableVec2,
    pub(crate) anime_vec3: ActionListAnimatorableVec3,
    pub(crate) anime_vec4: ActionListAnimatorableVec4,

    pub(crate) trail_create: ActionListTrail,
    pub(crate) trail_age: ActionListTrailAge,

    pub(crate) parsys_calculator: ActionListCPUParticleCalculator,
    pub(crate) parsys_create: ActionListCPUParticleSystem,
    pub(crate) parsys_state: ActionListCPUParticleSystemState,
    pub(crate) parsys_trailmaterial: ActionListCPUParticleSystemTrailMaterial,
    
    pub(crate) sprite_create: ActionListSpriteCreate,
    pub(crate) sprite_modify: ActionListSpriteModify,

    pub(crate) datatexcmd: DataTextureCmds,
    pub(crate) combinecmds: XHashMap<u32, (Atom, XHashMap<Atom, (u32, u16, bool, u32, u32, u32, u32)>)>,
    
    pub(crate) sprite_frames: (usize, Vec<SpriteFrame>),
    pub(crate) loadtextures: Vec<KeyImageTextureFrame>,
    pub(crate) verticesbuffers: Vec<(KeyVertexBuffer, Vec<u8>)>,
    pub(crate) indicesbuffers: Vec<(KeyVertexBuffer, Vec<u8>)>,
    pub(crate) indicesbuffersu32: Vec<(KeyVertexBuffer, Vec<u8>)>,
    pub(crate) custombuffers: ActionListCustomBuffer,
    
    pub(crate) crossdrawlistinfo: Vec<(Entity, Vec<Entity>)>,
    pub(crate) screenwithpostprocess: bool,
    pub(crate) gltfs: XHashMap<Entity, Handle<GLTF>>,
    pub(crate) disposegltfs: Vec<Entity>,
}

impl TActionSet for CommandsExchangeD3 {
    fn scene_create(&mut self) -> &mut ActionListSceneCreate { &mut self.scene_create }
    fn scene_options(&mut self) -> &mut ActionListSceneOption { &mut self.scene_options }
    fn scene_dispose(&mut self) -> &mut ActionListSceneDispose { &mut self.scene_dispose }
    fn scene_boundingbox(&mut self) -> &mut ActionListBoundingBoxDisplay { &mut self.scene_boundingbox }
    fn scene_collider(&mut self) -> &mut ActionListCollider { &mut self.scene_collider }
    fn obj_dispose(&mut self) -> &mut ActionListDispose { &mut self.obj_dispose }
    fn transform_create(&mut self) -> &mut ActionListTransformNodeCreate { &mut self.transform_create }
    fn transform_localsrt(&mut self) -> &mut ActionListTransformNodeLocal { &mut self.transform_localsrt }
    fn transform_localrotq(&mut self) -> &mut ActionListTransformNodeLocalRotationQuaternion { &mut self.transform_localrotq }
    fn transform_tree(&mut self) -> &mut ActionListTransformNodeParent { &mut self.transform_tree }
    fn transform_enable(&mut self) -> &mut ActionListNodeEnable { &mut self.transform_enable }
    fn camera_create(&mut self) -> &mut ActionListCameraCreate { &mut self.camera_create }
    fn camera_param(&mut self) -> &mut ActionListCameraModify { &mut self.camera_param }
    fn camera_target(&mut self) -> &mut ActionListCameraTarget { &mut self.camera_target }
    fn camera_forceinclude(&mut self) -> &mut ActionListViewerForceInclude { &mut self.camera_forceinclude }
    fn mesh_create(&mut self) -> &mut ActionListMeshCreate { &mut self.mesh_create }
    fn mesh_render_state(&mut self) -> &mut ActionListRenderState { &mut self.mesh_render_state }
    fn mesh_pose(&mut self) -> &mut ActionListAbstractMeshPose { &mut self.mesh_pose }
    fn mesh_state(&mut self) -> &mut ActionListMeshStateModify { &mut self.mesh_state }
    fn mesh_valuestate(&mut self) -> &mut ActionListAbstructMeshValueStateModify { &mut self.mesh_valuestate }
    fn mesh_bounding(&mut self) -> &mut ActionListMeshBounding { &mut self.mesh_bounding }
    fn forcelighting(&mut self) -> &mut ActionListMeshForceLighting { &mut self.forcelighting }
    fn skin_create(&mut self) -> &mut ActionListSkinCreate { &mut self.skin_create }
    fn skin_use(&mut self) -> &mut ActionListSkinUse { &mut self.skin_use }
    fn skin_bonecreate(&mut self) -> &mut ActionListBoneCreate { &mut self.skin_bonecreate }
    fn skin_bonepose(&mut self) -> &mut ActionListBonePose { &mut self.skin_bonepose }
    fn mesh_layermask(&mut self) -> &mut ActionListLayerMask { &mut self.mesh_layermask }
    fn instance_create(&mut self) -> &mut ActionListInstanceMeshCreate { &mut self.instance_create }
    fn instance_attr(&mut self) -> &mut ActionListInstanceAttr { &mut self.instance_attr }
    fn instance_targetanime(&mut self) -> &mut ActionListTargetAnimationAttribute { &mut self.instance_targetanime }
    fn geometry_create(&mut self) -> &mut ActionListGeometryCreate { &mut self.geometry_create }
    fn material_usemat(&mut self) -> &mut ActionListMaterialUse { &mut self.material_usemat }
    fn material_create(&mut self) -> &mut ActionListMaterialCreate { &mut self.material_create }
    fn material_val(&mut self) -> &mut ActionListUniformVal { &mut self.material_val }
    fn material_valb(&mut self) -> &mut ActionListUniformValB { &mut self.material_valb }
    fn light_create(&mut self) -> &mut ActionListLightCreate { &mut self.light_create }
    fn light_param(&mut self) -> &mut ActionListLightParam { &mut self.light_param }
    fn shadow_param(&mut self) -> &mut ActionListShadowGeneratorParam { &mut self.shadow_param }
    fn shadow_create(&mut self) -> &mut ActionListShadowGenerator { &mut self.shadow_create }
    fn renderer_subgraph(&mut self) -> &mut ActionListSubGraphCreate { &mut self.renderer_subgraph }
    fn renderer_create(&mut self) -> &mut ActionListRendererCreate { &mut self.renderer_create }
    fn renderer_connect(&mut self) -> &mut ActionListRendererConnect { &mut self.renderer_connect }
    fn renderer_modify(&mut self) -> &mut ActionListRendererModify { &mut self.renderer_modify }
    fn renderer_target(&mut self) -> &mut ActionListRendererTarget { &mut self.renderer_target }
    fn anime_create(&mut self) -> &mut ActionListAnimeGroupCreate { &mut self.anime_create }
    fn anime_action(&mut self) -> &mut ActionListAnimationGroupAction { &mut self.anime_action }
    fn anime_dispose(&mut self) -> &mut ActionListAnimeGroupDispose { &mut self.anime_dispose }
    fn anime_reset_while_start(&mut self) -> &mut ActionListAnimeGroupStartReset { &mut self.anime_reset_while_start }
    fn anime_property_targetanime(&mut self) -> &mut ActionListPropertyTargetAnimation { &mut self.anime_property_targetanime }
    fn anime_goto(&mut self) -> &mut ActionListAnimationGroupGoto { &mut self.anime_goto }
    fn anime_float(&mut self) -> &mut ActionListAnimatorableFloat { &mut self.anime_float }
    fn anime_sint(&mut self) -> &mut ActionListAnimatorableSint { &mut self.anime_sint }
    fn anime_uint(&mut self) -> &mut ActionListAnimatorableUint { &mut self.anime_uint }
    fn anime_vec2(&mut self) -> &mut ActionListAnimatorableVec2 { &mut self.anime_vec2 }
    fn anime_vec3(&mut self) -> &mut ActionListAnimatorableVec3 { &mut self.anime_vec3 }
    fn anime_vec4(&mut self) -> &mut ActionListAnimatorableVec4 { &mut self.anime_vec4 }
    fn trail_create(&mut self) -> &mut ActionListTrail { &mut self.trail_create }
    fn trail_age(&mut self) -> &mut ActionListTrailAge { &mut self.trail_age }
    fn parsys_calculator(&mut self) -> &mut ActionListCPUParticleCalculator { &mut self.parsys_calculator }
    fn parsys_create(&mut self) -> &mut ActionListCPUParticleSystem { &mut self.parsys_create }
    fn parsys_state(&mut self) -> &mut ActionListCPUParticleSystemState { &mut self.parsys_state }
    fn parsys_trailmaterial(&mut self) -> &mut ActionListCPUParticleSystemTrailMaterial { &mut self.parsys_trailmaterial }
    fn sprite_create(&mut self) -> &mut ActionListSpriteCreate { &mut self.sprite_create }
    fn sprite_modify(&mut self) -> &mut ActionListSpriteModify { &mut self.sprite_modify }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl CommandsExchangeD3 {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        let mut result = Self::default();
        result
    }
}

impl CommandsExchangeD3 {
    pub fn loadtextures(&mut self) -> &mut Vec<KeyImageTextureFrame> {
        &mut self.loadtextures
    }
    pub fn gltfs(& self) -> & XHashMap<Entity, Handle<GLTF>> {
        & self.gltfs
    }
    pub fn gltfs_mut(&mut self) -> &mut XHashMap<Entity, Handle<GLTF>> {
        &mut self.gltfs
    }
    pub fn datatexcmd(&mut self) -> &mut DataTextureCmds {
        &mut self.datatexcmd
    }
    pub fn combinecmds(&mut self) -> &mut XHashMap<u32, (Atom, XHashMap<Atom, (u32, u16, bool, u32, u32, u32, u32)>)> {
        &mut self.combinecmds
    }
    pub fn capacity(&self, cmds: & pi_3d::ActionSets) -> usize {
        self.scene_create               .capacity() +
        self.scene_options              .capacity() +
        self.scene_dispose              .capacity() +
        self.scene_boundingbox          .capacity() +
        self.scene_collider             .capacity() +
        self.obj_dispose                .capacity() +
        self.transform_create           .capacity() +
        self.transform_localsrt         .capacity() +
        self.transform_localrotq        .capacity() +
        self.transform_tree             .capacity() +
        self.transform_enable           .capacity() +
        self.camera_create              .capacity() +
        self.camera_param               .capacity() +
        self.camera_target              .capacity() +
        self.camera_forceinclude        .capacity() +
        self.mesh_create                .capacity() +
        self.mesh_state                 .capacity() +
        self.mesh_valuestate            .capacity() +
        self.mesh_render_state          .capacity() +
        self.mesh_pose                  .capacity() +
        self.forcelighting              .capacity() +
        self.mesh_bounding              .capacity() +
        self.mesh_layermask             .capacity() +
        self.skin_create                .capacity() +
        self.skin_bonecreate            .capacity() +
        self.skin_use                   .capacity() +
        self.skin_bonepose              .capacity() +
        self.instance_create            .capacity() +
        self.instance_attr              .capacity() +
        self.instance_targetanime       .capacity() +
        self.geometry_create            .capacity() +
        self.material_usemat            .capacity() +
        self.material_create            .capacity() +
        self.light_create               .capacity() +
        self.light_param                .capacity() +
        self.shadow_param               .capacity() +
        self.shadow_create              .capacity() +
        self.renderer_create            .capacity() +
        self.renderer_connect           .capacity() +
        self.renderer_modify            .capacity() +
        self.renderer_target            .capacity() +
        self.anime_create               .capacity() +
        self.anime_action               .capacity() +
        self.anime_dispose              .capacity() +
        self.anime_reset_while_start    .capacity() +
        self.anime_property_targetanime .capacity() +
        self.anime_float                .capacity() +
        self.anime_sint                 .capacity() +
        self.anime_uint                 .capacity() +
        self.anime_vec2                 .capacity() +
        self.anime_vec3                 .capacity() +
        self.anime_vec4                 .capacity() +
        self.trail_create               .capacity() +
        self.trail_age                  .capacity() +
        self.parsys_create              .capacity() +
        self.parsys_calculator          .capacity() +
        self.parsys_state               .capacity() +
        self.parsys_trailmaterial       .capacity() +
        self.sprite_create              .capacity() +
        self.sprite_modify              .capacity() +
        cmds.memsize()
    }
    pub fn exchange(&mut self, cmds: &mut pi_3d::ActionSets) {
        cmds.scene.create.append(&mut self.scene_create );
        cmds.scene.options.append(&mut self.scene_options );
        cmds.scene_dispose.append(&mut self.scene_dispose );
        cmds.scene.boundingboxdisplay.append(&mut self.scene_boundingbox );
        cmds.scene.collider.append(&mut self.scene_collider );
        cmds.obj_dispose.append(&mut self.obj_dispose );
        cmds.transform.create.append(&mut self.transform_create );
        cmds.transform.localsrt.append(&mut self.transform_localsrt );
        cmds.transform.localrotq.append(&mut self.transform_localrotq );
        cmds.transform.tree.append(&mut self.transform_tree );
        cmds.transform.enable.append(&mut self.transform_enable );
        cmds.camera.create.append(&mut self.camera_create );
        cmds.camera.param.append(&mut self.camera_param );
        cmds.camera.target.append(&mut self.camera_target );
        cmds.camera.forceinclude.append(&mut self.camera_forceinclude );
        cmds.mesh.create.append(&mut self.mesh_create );
        cmds.mesh.state.append(&mut self.mesh_state );
        cmds.mesh.value_state.append(&mut self.mesh_valuestate );
        cmds.mesh.render_state.append(&mut self.mesh_render_state );
        cmds.mesh.pose.append(&mut self.mesh_pose );
        cmds.mesh.forcelighting.append(&mut self.forcelighting );
        cmds.mesh.bounding.append(&mut self.mesh_bounding );
        cmds.mesh.layermask.append(&mut self.mesh_layermask );
        cmds.skin.skin_create.append(&mut self.skin_create );
        cmds.skin.bone_create.append(&mut self.skin_bonecreate );
        cmds.skin.skin_use.append(&mut self.skin_use );
        cmds.skin.bone_pose.append(&mut self.skin_bonepose );
        cmds.instance.create.append(&mut self.instance_create );
        cmds.instance.attr.append(&mut self.instance_attr );
        cmds.animation.anime_instance.append(&mut self.instance_targetanime );
        cmds.animation.anime_float.append(&mut self.anime_float );
        cmds.animation.anime_sint.append(&mut self.anime_sint );
        cmds.animation.anime_uint.append(&mut self.anime_uint );
        cmds.animation.anime_vec2.append(&mut self.anime_vec2 );
        cmds.animation.anime_vec3.append(&mut self.anime_vec3 );
        cmds.animation.anime_vec4.append(&mut self.anime_vec4 );
        cmds.geometry.create.append(&mut self.geometry_create );
        cmds.material.usemat.append(&mut self.material_usemat );
        cmds.material.create.append(&mut self.material_create );
        cmds.material.val.append(&mut self.material_val );
        cmds.material.valb.append(&mut self.material_valb );
        cmds.light.create.append(&mut self.light_create );
        cmds.light.param.append(&mut self.light_param );
        cmds.shadow.param.append(&mut self.shadow_param );
        cmds.shadow.create.append(&mut self.shadow_create );
        cmds.renderer.subgraph.append(&mut self.renderer_subgraph );
        cmds.renderer.create.append(&mut self.renderer_create );
        cmds.renderer.connect.append(&mut self.renderer_connect );
        cmds.renderer.modify.append(&mut self.renderer_modify );
        cmds.renderer.target.append(&mut self.renderer_target );
        cmds.anime.create.append(&mut self.anime_create );
        cmds.anime.action.append(&mut self.anime_action );
        cmds.anime.dispose.append(&mut self.anime_dispose );
        cmds.anime.reset_while_start.append(&mut self.anime_reset_while_start );
        cmds.anime.goto.append(&mut self.anime_goto );
        cmds.property_targetanimation.append(&mut self.anime_property_targetanime );
        cmds.trail.create.append(&mut self.trail_create );
        cmds.trail.age.append(&mut self.trail_age );
        cmds.parsys.create.append(&mut self.parsys_create );
        cmds.parsys.calculator.append(&mut self.parsys_calculator );
        cmds.parsys.state.append(&mut self.parsys_state );
        cmds.parsys.trailmaterial.append(&mut self.parsys_trailmaterial );
        cmds.spritecreate.append(&mut self.sprite_create );
        cmds.spritemodify.append(&mut self.sprite_modify );
        
        // cmds.scene.create.exchange( self.scene_create.exchange(vec![]) );
        // cmds.scene.options.exchange( self.scene_options.exchange(vec![]) );
        // cmds.scene_dispose.exchange( self.scene_dispose.exchange(vec![]) );
        // cmds.scene.boundingboxdisplay.exchange( self.scene_boundingbox.exchange(vec![]) );
        // cmds.scene.collider.exchange( self.scene_collider.exchange(vec![]) );
        // cmds.obj_dispose.exchange( self.obj_dispose.exchange(vec![]) );
        // cmds.transform.create.exchange( self.transform_create.exchange(vec![]) );
        // cmds.transform.localsrt.exchange( self.transform_localsrt.exchange(vec![]) );
        // cmds.transform.localrotq.exchange( self.transform_localrotq.exchange(vec![]) );
        // cmds.transform.tree.exchange( self.transform_tree.exchange(vec![]) );
        // cmds.transform.enable.exchange( self.transform_enable.exchange(vec![]) );
        // cmds.camera.create.exchange( self.camera_create.exchange(vec![]) );
        // cmds.camera.param.exchange( self.camera_param.exchange(vec![]) );
        // cmds.camera.target.exchange( self.camera_target.exchange(vec![]) );
        // cmds.camera.forceinclude.exchange( self.camera_forceinclude.exchange(vec![]) );
        // cmds.mesh.create.exchange( self.mesh_create.exchange(vec![]) );
        // cmds.mesh.state.exchange( self.mesh_state.exchange(vec![]) );
        // cmds.mesh.value_state.exchange( self.mesh_valuestate.exchange(vec![]) );
        // cmds.mesh.blend.exchange( self.mesh_blend.exchange(vec![]) );
        // cmds.mesh.forcelighting.exchange( self.forcelighting.exchange(vec![]) );
        // cmds.mesh.primitive_state.exchange( self.mesh_primitivestate.exchange(vec![]) );
        // cmds.mesh.stencil_state.exchange( self.mesh_stencilstate.exchange(vec![]) );
        // cmds.mesh.depth_state.exchange( self.mesh_depthstate.exchange(vec![]) );
        // cmds.mesh.render_queue.exchange( self.mesh_render_queue.exchange(vec![]) );
        // cmds.mesh.bounding.exchange( self.mesh_bounding.exchange(vec![]) );
        // cmds.mesh.layermask.exchange( self.mesh_layermask.exchange(vec![]) );
        // cmds.skin.skin_create.exchange( self.skin_create.exchange(vec![]) );
        // cmds.skin.bone_create.exchange( self.skin_bonecreate.exchange(vec![]) );
        // cmds.skin.skin_use.exchange( self.skin_use.exchange(vec![]) );
        // cmds.skin.bone_pose.exchange( self.skin_bonepose.exchange(vec![]) );
        // cmds.instance.create.exchange( self.instance_create.exchange(vec![]) );
        // cmds.instance.attr.exchange( self.instance_attr.exchange(vec![]) );
        // cmds.anime_instance.exchange( self.instance_targetanime.exchange(vec![]) );
        // cmds.geometry.create.exchange( self.geometry_create.exchange(vec![]) );
        // cmds.material.usemat.exchange( self.material_usemat.exchange(vec![]) );
        // cmds.material.create.exchange( self.material_create.exchange(vec![]) );
        // cmds.material.float.exchange( self.material_float.exchange(vec![]) );
        // cmds.material.uint.exchange( self.material_uint.exchange(vec![]) );
        // cmds.material.vec2.exchange( self.material_vec2.exchange(vec![]) );
        // cmds.material.vec4.exchange( self.material_vec4.exchange(vec![]) );
        // cmds.material.mat4.exchange( self.material_mat4.exchange(vec![]) );
        // cmds.material.texture.exchange( self.material_texture.exchange(vec![]) );
        // cmds.material.texturefromtarget.exchange( self.material_texturefromtarget.exchange(vec![]) );
        // cmds.anime_uniform.exchange( self.uniform_targetanime.exchange(vec![]) );
        // cmds.light.create.exchange( self.light_create.exchange(vec![]) );
        // cmds.light.param.exchange( self.light_param.exchange(vec![]) );
        // cmds.shadow.param.exchange( self.shadow_param.exchange(vec![]) );
        // cmds.shadow.create.exchange( self.shadow_create.exchange(vec![]) );
        // cmds.renderer.create.exchange( self.renderer_create.exchange(vec![]) );
        // cmds.renderer.connect.exchange( self.renderer_connect.exchange(vec![]) );
        // cmds.renderer.modify.exchange( self.renderer_modify.exchange(vec![]) );
        // cmds.renderer.target.exchange( self.renderer_target.exchange(vec![]) );
        // cmds.anime.create.exchange( self.anime_create.exchange(vec![]) );
        // cmds.anime.action.exchange( self.anime_action.exchange(vec![]) );
        // cmds.anime.dispose.exchange( self.anime_dispose.exchange(vec![]) );
        // cmds.anime.reset_while_start.exchange( self.anime_reset_while_start.exchange(vec![]) );
        // cmds.anime.listens.exchange( self.anime_listen.exchange(vec![]) );
        // cmds.anime.frameevents.exchange( self.anime_frameevent.exchange(vec![]) );
        // cmds.anime.weight.exchange( self.anime_weight.exchange(vec![]) );
        // cmds.property_targetanimation.exchange( self.anime_property_targetanime.exchange(vec![]) );
        // cmds.trail.create.exchange( self.trail_create.exchange(vec![]) );
        // cmds.trail.age.exchange( self.trail_age.exchange(vec![]) );
        // cmds.parsys.create.exchange( self.parsys_create.exchange(vec![]) );
        // cmds.parsys.calculator.exchange( self.parsys_calculator.exchange(vec![]) );
        // cmds.parsys.state.exchange( self.parsys_state.exchange(vec![]) );
        // cmds.parsys.trailmaterial.exchange( self.parsys_trailmaterial.exchange(vec![]) );
        // cmds.spritecreate.exchange( self.sprite_create.exchange(vec![]) );
        // cmds.spritemodify.exchange( self.sprite_modify.exchange(vec![]) );
    }
    
    pub fn gltfres(replayentities: &XHashMap<Entity, Entity>, v: &GLTFRes) -> GLTFRes {
        GLTFRes::new(*replayentities.get(v.val()).unwrap())
    }
    pub fn entity(replayentities: &XHashMap<Entity, Entity>, entity: f64) -> Entity {
        *replayentities.get(&as_entity(entity)).unwrap()
    }
    pub fn entity_f64(replayentities: &XHashMap<Entity, Entity>, entity: f64) -> f64 {
        as_f64(replayentities.get(&as_entity(entity)).unwrap())
    }
    pub fn rendertarget(replayrendertargetkey: &XHashMap<Entity, Entity>, entity: f64) -> Entity {
        *replayrendertargetkey.get(&as_entity(entity)).unwrap()
    }
    pub fn rendertarget_f64(replayrendertargetkey: &XHashMap<Entity, Entity>, entity: f64) -> f64 {
        as_f64(&replayrendertargetkey.get(&as_entity(entity)).unwrap())
    }
    pub fn record_create(world: &mut World, entity: f64) {
        world.get_resource_mut::<Records>().unwrap().record_create(as_entity(entity));
    }
    pub fn record2(&mut self, cmd: ERecord3D) {
        self.recordframes.push(cmd);
    }
    pub fn record(&mut self, cmd: ERecordCMD) {
        self.recordframes.push(ERecord3D::CMD(cmd));
    }
    pub fn read_record(&mut self) -> Vec<ERecord3D> {
        replace(&mut self.recordframes, vec![]) 
    }
    pub fn replay<T: TActionSet>(cmds: &mut T, cmd: ERecordCMD, replayentities: & XHashMap<Entity, Entity>) {
        match cmd {
            ERecordCMD::PARTICLESYS(scene, entity, trailmesh, trailgeo, key, color_attr_key, tilloff_attr_key, update_buffer_interval_frame) => {
                let scene = Self::entity(replayentities, scene);
                let entity = Self::entity(replayentities, entity);
                let trailmesh = Self::entity(replayentities, trailmesh);
                let trailgeo = Self::entity(replayentities, trailgeo);
                CommandsExchangeD3::p3d_particle_system(cmds, scene, entity, trailmesh, trailgeo, key, &color_attr_key, &tilloff_attr_key, update_buffer_interval_frame);
            },

            ERecordCMD::Dispose(entity) => {
                let entity = Self::entity(replayentities, entity);
                CommandsExchangeD3::p3d_dispose(cmds, entity);
            },
            ERecordCMD::SceneDispose(entity) => {
                let entity = Self::entity(replayentities, entity);
                CommandsExchangeD3::p3d_scene_dispose(cmds, entity);
            },
            ERecordCMD::RenderGraphic(before, after, isdisconnect) => {
                let before: Entity = Self::entity(replayentities, before);
                let after: Entity = Self::entity(replayentities, after);
                CommandsExchangeD3::p3d_render_graphic(cmds, before, after, isdisconnect);
            },
            ERecordCMD::SCENE(scene, cullingmode, collidermode, values) => {
                let scene = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_scene(cmds, scene, cullingmode, collidermode, values);
            },
            ERecordCMD::SceneOption(scene, val) => {
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_scene_option(cmds, scene, val);
            },
            ERecordCMD::LAYERMASK(node, val) => {
                let node: Entity = Self::entity(replayentities, node);
                cmds.mesh_layermask().push(OpsLayerMask::ops(node, val as u32));
            },
            ERecordCMD::SceneBoundingbox(scene, display, pass) => {
                let pass = EngineConstants::passtag(pass);
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_scene_boundingbox(cmds, scene, display, pass);
            },
            ERecordCMD::COLLIDER(node, minx, miny, minz, maxx, maxy, maxz, intersection_treshold, alphaindex) => {
                let node: Entity = Self::entity(replayentities, node);
                let alphaindex = if let Some(alphaindex) = alphaindex { alphaindex as i32 } else { i32::MIN };
                CommandsExchangeD3::p3d_collider(cmds, node, minx as f32, miny as f32, minz as f32, maxx as f32, maxy as f32, maxz as f32, intersection_treshold as f32, alphaindex as i32);
            },
            ERecordCMD::AnimationGroup(scene, id) => {
                let id: Entity = Self::entity(replayentities, id);
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_animation_group(cmds, scene, id);
            },
            ERecordCMD::AnimationGroupWeight(group, weight) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_animation_group_weight(cmds, group, weight as f32);
            },
            ERecordCMD::AnimationGroupTargetReset(group) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_animation_group_target_reset(cmds, group);
            },
            ERecordCMD::AnimationGroupStart(group, speed, loop_mode, loop_count, from, to, fps, amount_mode, delay_ms, fillmode, amount_param0, amount_param1, amount_param2, amount_param3) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_anime_group_start(cmds, group, speed, loop_mode, loop_count, from, to, fps, amount_mode, delay_ms, fillmode, amount_param0, amount_param1, amount_param2, amount_param3);
            },
            ERecordCMD::AnimationGroupPause(group) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_anime_group_pause(cmds, group);
            },
            ERecordCMD::AnimationGroupStop(group) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_anime_group_stop(cmds, group);
            },
            ERecordCMD::AnimationGroupGoto(group, amount) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_anime_group_goto(cmds, group, amount as KeyFrameCurveValue);
            },
            ERecordCMD::AnimationGroupRestart(group) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_animation_group_restart(cmds, group);
            },
            ERecordCMD::AnimationGroupDelete(group) => {
                let group = Self::entity(replayentities, group);
                CommandsExchangeD3::p3d_animation_group_delete(cmds, group);
            },
            ERecordCMD::PropertyTargetAnimation(curve_key, property, group, curve_target) => {
                let group = Self::entity(replayentities, group);
                let curve_target = Self::entity(replayentities, curve_target);
                let curve_key: u64 = pi_atom::Atom::from(&curve_key).asset_u64();
                CommandsExchangeD3::p3d_property_target_animation(cmds, curve_key, property, group, curve_target);
            },
            ERecordCMD::TRANSFORMNODE(scene, id) => {
                let id: Entity = Self::entity(replayentities, id);
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_transform_node( cmds, scene, id);
            },
            ERecordCMD::TransformnodeParent(node, parent) => {
                let node: Entity = Self::entity(replayentities, node);
                let parent: Entity = Self::entity(replayentities, parent);
                CommandsExchangeD3::p3d_transform_node_parent(cmds, node, parent);
            },
            ERecordCMD::TransformnodeSRT(node, val) => {
                let node: Entity = Self::entity(replayentities, node);
                CommandsExchangeD3::p3d_local_srt(cmds, node, val);
            },
            ERecordCMD::TransformnodeEnable(node, val) => {
                let node: Entity = Self::entity(replayentities, node);
                CommandsExchangeD3::p3d_node_enable(cmds, node, val);
            },
            ERecordCMD::TransformnodeQuaternion(node, x, y, z, w) => {
                let node: Entity = Self::entity(replayentities, node);
                CommandsExchangeD3::p3d_local_quaternion(cmds, node, x as f32, y as f32, z as f32, w as f32);
            },
            ERecordCMD::TRAIL(scene, entity, linked) => {
                let entity = Self::entity(replayentities, entity);
                let scene = Self::entity(replayentities, scene);
                let id_linked_transform = Self::entity(replayentities, linked);
                CommandsExchangeD3::p3d_trail(cmds, scene, entity, id_linked_transform);
            },
            ERecordCMD::TrailAge(entity, age_ms) => {
                let entity = Self::entity(replayentities, entity);
                let age_ms = age_ms as u32;
                CommandsExchangeD3::p3d_trail_age(cmds, entity, age_ms);
            },
            ERecordCMD::CAMERA(scene, id, graph) => {
                let scene: Entity = Self::entity(replayentities, scene);
                let id: Entity = Self::entity(replayentities, id);
                let graph = if let Some(graph) = graph { Self::entity(replayentities, graph) } else { Entity::null() };
                CommandsExchangeD3::p3d_camera(cmds, scene, id, graph);
            },
            ERecordCMD::CameraParam(camera, val) => {
                let camera: Entity = Self::entity(replayentities, camera);
                CommandsExchangeD3::p3d_camera_param(cmds, camera, val);
            },
            ERecordCMD::CameraTarget(camera, x, y, z) => {
                let camera: Entity = Self::entity(replayentities, camera);
                CommandsExchangeD3::p3d_camera_target(cmds, camera,  x as f32, y as f32, z as f32);
            },
            ERecordCMD::ViewerForceinclude(viewer, entity, add) => {
                let viewer: Entity = Self::entity(replayentities, viewer);
                let entity: Entity = Self::entity(replayentities, entity);
                CommandsExchangeD3::p3d_viewer_force_include(cmds, viewer, entity, add);
            },
            ERecordCMD::SPRITE(source, id, atlas) => {
                let id: Entity = Self::entity(replayentities, id);
                let source = Self::entity(replayentities, source);
                CommandsExchangeD3::p3d_sprite(cmds, source, id, &atlas);
            },
            ERecordCMD::SpriteFrame(sprite, tilloffkey, idxframe) => {
                let sprite = Self::entity(replayentities, sprite);
                CommandsExchangeD3::p3d_sprite_frame(cmds, sprite, &tilloffkey, idxframe);
            },
            ERecordCMD::SpriteFrameData(sprite, tilloffkey, data) => {
                let sprite = Self::entity(replayentities, sprite);
                CommandsExchangeD3::p3d_sprite_frame_data(cmds, sprite, &tilloffkey, &data);
            },
            ERecordCMD::InstanceMesh(source, id) => {
                let id: Entity = Self::entity(replayentities, id);
                let source: Entity = Self::entity(replayentities, source);
                CommandsExchangeD3::p3d_instance_mesh(cmds, source, id);
            },
            ERecordCMD::InstanceAttr(instance, attr, key) => {
                let instance: Entity = Self::entity(replayentities, instance);
                CommandsExchangeD3::p3d_instance_attr(cmds, instance, attr, key);
            },
            ERecordCMD::MeshBoneOffset(instance, val) => {
                let instance: Entity = Self::entity(replayentities, instance);
                CommandsExchangeD3::p3d_mesh_bone_offset(cmds, instance, val as u32);
            },
            ERecordCMD::MESH(scene, id, instancestate, instance_use_single_buffer) => {
                let id: Entity = Self::entity(replayentities, id);
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_mesh(cmds, scene, id, &instancestate, instance_use_single_buffer);
            },
            ERecordCMD::MeshGeometry(mesh, geometa, geoid) => {
                let geoid: Entity = Self::entity(replayentities, geoid);
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_mesh_geometry(cmds, mesh, &geometa, geoid);
            },
            ERecordCMD::MeshValueState(mesh, val) => {
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_mesh_valuestate(cmds, mesh, val);
            },
            ERecordCMD::MeshRenderState(mesh, val) => {
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_mesh_render_state(cmds, mesh, val);
            },
            ERecordCMD::MeshState(mesh, val) => {
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_mesh_state(cmds, mesh, val);
            },
            ERecordCMD::MeshBoundingBox(mesh, minx, miny, minz, maxx, maxy, maxz) => {
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_mesh_bounding_box(cmds, mesh, minx, miny, minz, maxx, maxy, maxz);
            },
            ERecordCMD::MeshAttributeTargetAnim(target, group, key, curve_key) => {
                let target = Self::entity(replayentities, target);
                let group = Self::entity(replayentities, group);
                let curve_key = pi_atom::Atom::from(&curve_key).asset_u64();
                CommandsExchangeD3::p3d_attribute_target_animation(cmds, target, group, &key, curve_key);
            },
            ERecordCMD::MeshPoseMatrix(mesh, val) => {
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_abstruct_pose_matrix(cmds, mesh, val);
            },
            ERecordCMD::LIGHT(scene, id, ltype) => {
                let id: Entity = Self::entity(replayentities, id);
                let scene: Entity = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_light(cmds, scene, id, ltype);
            },
            ERecordCMD::LightParam(light, val) => {
                let light: Entity = Self::entity(replayentities, light);
                CommandsExchangeD3::p3d_light_param(cmds, light, val);
            },
            ERecordCMD::MeshForceIndludInLight(light, mesh_or_instance, val) => {
                let mesh_or_instance: Entity = Self::entity(replayentities, mesh_or_instance);
                let light: Entity = Self::entity(replayentities, light);
                CommandsExchangeD3::p3d_light_forceinclude(cmds, light, mesh_or_instance, val);
            },
            ERecordCMD::MaterialShader(mat, shader, usematarray) => {
                let mat: Entity = Self::entity(replayentities, mat);
                CommandsExchangeD3::p3d_material_shader(cmds, mat, &shader, usematarray);
            },
            ERecordCMD::MaterialApply(mat, mesh, pass) => {
                let mat: Entity = Self::entity(replayentities, mat);
                let mesh: Entity = Self::entity(replayentities, mesh);
                CommandsExchangeD3::p3d_material_apply(cmds, mat, mesh, pass);
            },
            ERecordCMD::MaterialUniformMat4(mat, key, val) => {
                let mat: Entity = Self::entity(replayentities, mat);
                CommandsExchangeD3::p3d_material_uniform_mat(cmds, mat, &key, val);
            },
            ERecordCMD::MaterialUniformV0(mat, val) => {
                let mat: Entity = Self::entity(replayentities, mat);
                CommandsExchangeD3::p3d_material_uniform_value(cmds, mat, val);
            },
            ERecordCMD::MaterialUniformTex(mat, key, url,
                srgb, compressed, filter, address_mode_u, address_mode_v, address_mode_w,
                mag_filter, min_filter, mipmap_filter, anisotropy_clamp, border_color,
                isfile, cancombine, compare
            ) => {
                let mat: Entity = Self::entity(replayentities, mat);  
                CommandsExchangeD3::p3d_material_uniform_tex(cmds, mat, &key, &url,
                    srgb, compressed, filter, address_mode_u, address_mode_v, address_mode_w,
                    mag_filter, min_filter, mipmap_filter,
                    anisotropy_clamp, border_color,
                    isfile, cancombine, compare
                );
            },
            ERecordCMD::LoadTexture(atom, _, _, _) => todo!(),
            ERecordCMD::MaterialTexFromRendertarget(mat, key, key_tilloff, url,
                filter, address_mode_u, address_mode_v, address_mode_w,
                mag_filter, min_filter, mipmap_filter,
                anisotropy_clamp, border_color, compare
            ) => {
                let mat: Entity = Self::entity(replayentities, mat);  
                CommandsExchangeD3::p3d_material_uniform_tex_from_render_target(cmds, mat, &key, &key_tilloff, url,
                    filter, address_mode_u, address_mode_v, address_mode_w,
                    mag_filter, min_filter, mipmap_filter,
                    anisotropy_clamp, border_color, compare
                );
            },
            ERecordCMD::MaterialTexFromRenderer(mat, key, key_tilloff, url,
                filter, address_mode_u, address_mode_v, address_mode_w,
                mag_filter, min_filter, mipmap_filter,
                anisotropy_clamp, border_color, compare
            ) => {
                let mat: Entity = Self::entity(replayentities, mat);  
                let url = Self::entity(replayentities, url);
                CommandsExchangeD3::p3d_material_uniform_tex_from_renderer(cmds, mat, &key, &key_tilloff, url,
                    filter, address_mode_u, address_mode_v, address_mode_w,
                    mag_filter, min_filter, mipmap_filter,
                    anisotropy_clamp, border_color, compare
                );
            },
            ERecordCMD::MaterialTargetAnimation(mat0, group0, key, curve_key) => {
                let mat = Self::entity(replayentities, mat0);
                let group = Self::entity(replayentities, group0);
                let curve_key = pi_atom::Atom::from(&curve_key).asset_u64();
                CommandsExchangeD3::p3d_uniform_target_animation(cmds, mat, group, &key, curve_key);
            },
            ERecordCMD::ParticlesysState(entity, val) => {
                let entity = Self::entity(replayentities, entity);
                CommandsExchangeD3::p3d_particle_system_state(cmds, entity, val);
            },
            ERecordCMD::RenderSubgraph(id_renderer, name) => {
                let id_renderer = Self::entity(replayentities, id_renderer);
                cmds.renderer_subgraph().push(OpsSubGraphCreate::ops(id_renderer, name));
            },
            ERecordCMD::RENDER(viewer, id_renderer, name, pass_tag, transparent, recordinput, crossrender) => {
                let viewer: Entity = Self::entity(replayentities, viewer);
                let id_renderer: Entity = Self::entity(replayentities, id_renderer);
                CommandsExchangeD3::p3d_create_render(cmds, viewer, id_renderer, name, pass_tag, transparent, recordinput, crossrender);
            },
            ERecordCMD::RenderModify(renderer, val) => {
                let renderer: Entity = Self::entity(replayentities, renderer);
                CommandsExchangeD3::p3d_renderer_modify(cmds, renderer, val);
            },
            ERecordCMD::RenderTarget(renderer, val) => {
                let renderer: Entity = Self::entity(replayentities, renderer);
                CommandsExchangeD3::p3d_render_target(cmds, renderer, val);
            },
            ERecordCMD::ShadowGenerator(scene, light, id, pass_tag, graph) => {
                let id: Entity = Self::entity(replayentities, id);
                let scene: Entity = Self::entity(replayentities, scene);
                let light: Entity = Self::entity(replayentities, light);
                let graph = if let Some(graph) = graph { Self::entity(replayentities, graph) } else { Entity::null() };
                CommandsExchangeD3::p3d_shadow_generator(cmds, scene, light, id, pass_tag as u16, graph);
            },
            ERecordCMD::ShadowParam(shadow, val) => {
                let shadow: Entity = Self::entity(replayentities, shadow);
                CommandsExchangeD3::p3d_shadow_base_param(cmds, shadow, val);
            },
            ERecordCMD::SKELETON(id, bonespervertex, root, bones, bonecount, cacheframe) => {
                let id = Self::entity(replayentities, id);
                let root = Self::entity(replayentities, root);
                let mut boneentities = vec![];
                for idx in 0..(bonecount as usize) {
                    let bone = Self::entity(replayentities, bones[idx]);
                    boneentities.push(bone);
                }
                CommandsExchangeD3::p3d_skeleton(cmds, id, bonespervertex as u8, root, &boneentities, bonecount, cacheframe);
            },
            ERecordCMD::BONE(bone, scene) => {
                let bone = Self::entity(replayentities, bone);
                let scene = Self::entity(replayentities, scene);
                CommandsExchangeD3::p3d_bone(cmds, bone, scene);
            },
            ERecordCMD::BoneLink(bone, link) => {
                let bone = Self::entity(replayentities, bone);
                let link = Self::entity(replayentities, link);
                CommandsExchangeD3::p3d_bone_link(cmds, bone, link);
            },
            ERecordCMD::BonePose(bone, data) => {
                let bone = Self::entity(replayentities, bone);
                CommandsExchangeD3::p3d_bone_pose(cmds, bone, &data);
            },
            ERecordCMD::SkinUse(id_mesh, skin) => {
                let id_mesh = Self::entity(replayentities, id_mesh);
                let skin = Self::entity(replayentities, skin);
                CommandsExchangeD3::p3d_skin_use(cmds, id_mesh, skin);
            },
        }

    }

    pub fn p3d_animation_curve_id_bygltf(
        cmds: &CommandsExchangeD3,
        gltf: &GLTFRes,
        group_index: usize,
        channel_index: usize,
    ) -> f64 {
        if let Some(gltf) = cmds.gltfs.get(gltf.val()) {
            let key = gltf.key_anime_curve(group_index as usize, channel_index as usize);
            unsafe { transmute(key) }
        } else {
            0.
        }
    }
    pub fn p3d_entity(app: &mut Engine) -> Entity {
        app.world.entities().reserve_entity()
    }
    pub fn p3d_dispose<T: TActionSet>(cmds: &mut T, entity: Entity) {
        cmds.obj_dispose().push(OpsDispose::ops(entity));
    }
    pub fn p3d_scene_dispose<T: TActionSet>(cmds: &mut T, scene: Entity) {
        cmds.scene_dispose().push(OpsSceneDispose::ops(scene));
    }
    pub fn p3d_lighting_shadow_limit<'w>(resource: &mut ResourceSets<'w>,
        scene_max_direct_light_count: u16,
        scene_max_point_light_count: u16,
        scene_max_spot_light_count: u16,
        scene_max_hemi_light_count: u16,
        scene_max_shadow_count: u16,
        model_max_direct_light_count: u16,
        model_max_point_light_count: u16,
        model_max_spot_light_count: u16,
        model_max_hemi_light_count: u16,
    ) {
        resource.scene_lighting_limit.0.max_direct_light_count = scene_max_direct_light_count as u16;
        resource.scene_lighting_limit.0.max_point_light_count = scene_max_point_light_count as u16;
        resource.scene_lighting_limit.0.max_spot_light_count = scene_max_spot_light_count as u16;
        resource.scene_lighting_limit.0.max_hemi_light_count = scene_max_hemi_light_count as u16;
        
        resource.scene_shadow_limit.0.max_count = scene_max_shadow_count as u16;

        resource.model_lighting_limit.0.max_direct_light_count = model_max_direct_light_count as u16;
        resource.model_lighting_limit.0.max_point_light_count = model_max_point_light_count as u16;
        resource.model_lighting_limit.0.max_spot_light_count = model_max_spot_light_count as u16;
        resource.model_lighting_limit.0.max_hemi_light_count = model_max_hemi_light_count as u16;
    }
    pub fn p3d_engine_state<'w>(cmds: &mut GlobalState<'w>, active: bool) {
        cmds.stateengine.active = active;
    }
    pub fn p3d_render_graphic<T: TActionSet>(cmds: &mut T, before: Entity, after: Entity, isdisconnect: bool) {
        cmds.renderer_connect().push(OpsRendererConnect::ops(before, after, isdisconnect));
    }
    pub fn p3d_engine_debug<'w>(cmds: &mut GlobalState<'w>, debug: bool) {
        cmds.performance.debug = debug;
        cmds.psperformance.debug = debug;
    }
    pub fn p3d_create_gltf_load<'w>(resource: &mut ResourceSets<'w>, entity: Entity, baseurl: pi_atom::Atom, dyndesc: String) {
        let param = baseurl;
        resource.gltf2_loader.create_load(entity, param);
    }
    pub fn p3d_get_gltf<'w>(resource: &mut ResourceSets<'w>, entity: Entity) -> Option<GLTFRes> {
        if let Some(val) = resource.gltf2_loader.get_success(entity) {
            resource.gltf2_records.0.insert(entity, val);
            Some(GLTFRes::new(entity))
        } else {
            None
        }
    }
    pub fn p3d_dispose_gltf(gltfs: &mut CommandsExchangeD3, entity: &GLTFRes) {
        gltfs.disposegltfs.push(*entity.val());
    }
    pub fn _create_image_load<'w>(resource: &mut ResourceSets<'w>, url: pi_atom::Atom, cancombine: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
        let id = resource.imgtex_loader.create_load(KeyImageTextureFrame { 
            url,
            file: true,
            compressed,
            cancombine
        });
        unsafe { transmute(id) }
    }
    pub fn p3d_animation_group<T: TActionSet>(cmds: &mut T, scene: Entity, id: Entity) {
        cmds.anime_create().push(OpsAnimationGroupCreation::ops(scene, id));
    }
    pub fn p3d_animation_group_weight<T: TActionSet>(cmds: &mut T, group: Entity, weight: f32) {
        cmds.anime_action().push(OpsAnimationGroupAction::weight(group, weight as f32));
    }
    pub fn p3d_animation_group_target_reset<T: TActionSet>(cmds: &mut T, group: Entity) {
        cmds.anime_reset_while_start().push(OpsAnimationGroupStartReset::ops(group));
    }
    pub fn p3d_anime_group_start<T: TActionSet>(cmds: &mut T,
        group_key: Entity,
        speed: f64,
        loop_mode: ELoopMode,
        loop_count: Option<f64>,
        from: f64,
        to: f64,
        fps: f64,
        amount_mode: EAmountMode,
        delay_ms: f64,
        fillmode: EFillMode,
        amount_param0: f64,
        amount_param1: f64,
        amount_param2: f64,
        amount_param3: f64,
    ) {
        let amountcalc = _amountcalc(amount_mode, amount_param0, amount_param1, amount_param2, amount_param3);
    
        let loop_count = if let Some(loop_count) = loop_count {
            Some(loop_count as u32)
        } else { None };
        let loop_mode = loop_mode.val(loop_count);
        let fillmode = unsafe { transmute(fillmode) };
        cmds.anime_action().push(OpsAnimationGroupAction::Start(group_key, AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as FramePerSecond, amountcalc), delay_ms as KeyFrameCurveValue, fillmode));
    }
    pub fn p3d_anime_group_pause<T: TActionSet>(cmds: &mut T,
        group_key: Entity,
    ) {
        cmds.anime_action().push(OpsAnimationGroupAction::Pause(group_key));
    }
    pub fn p3d_anime_group_stop<T: TActionSet>(cmds: &mut T,
        group_key: Entity,
    ) {
        cmds.anime_action().push(OpsAnimationGroupAction::Stop(group_key));
    }
    pub fn p3d_anime_group_goto<T: TActionSet>(cmds: &mut T,
        group_key: Entity,
        amount: KeyFrameCurveValue,
    ) {
        cmds.anime_goto().push(AnimationGroupGoto::ops(group_key, amount));
    }
    pub fn p3d_animation_group_restart<T: TActionSet>(cmds: &mut T,
        group: Entity,
    ) {
        cmds.anime_action().push(OpsAnimationGroupAction::restart(group));
    }
    pub fn p3d_animation_group_delete<T: TActionSet>(cmds: &mut T,
        group: Entity,
    ) {
        cmds.anime_dispose().push(OpsAnimationGroupDispose::ops(group));
    }
    pub fn p3d_anime_curve_create<T: TTypeAnimeAssetMgr>(cmds: &mut T, key: u64, property: EAnimePropertyID, data: &[f32], mode: EAnimeCurve) -> bool {

        match property {
            EAnimePropertyID::LocalPosition       => {
                let v = curve::<3, LocalPosition>(data,  mode);
                cmds.position().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalScaling        => {
                let v = curve::<3, LocalScaling>(data,  mode);
                cmds.scaling().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalRotation    => {
                let v = curve::<4, LocalRotationQuaternion>(data,  mode);
                cmds.quaternion().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalEulerAngles    => {
                let v = curve::<3, LocalEulerAngles>(data,  mode);
                cmds.euler().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Alpha               => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainColor           => {
                let v = curve::<3, AnimatorableVec3>(data,  mode);
                let result = cmds.vec3s().insert(key, TypeFrameCurve(v)).is_ok();
                log::error!("Curve Result: {:?}", (result, key));
                result
            },
            EAnimePropertyID::MainTexUScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexVScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexUOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexVOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexUScale    => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexVScale    => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexUOffset   => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexVOffset   => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::AlphaCutoff         => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::CameraFov           => {
                let v = curve::<1, CameraFov>(data,  mode);
                cmds.camerafov().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::CameraOrthSize      => {
                let v = curve::<1, CameraOrthSize>(data,  mode);
                cmds.camerasize().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LightDiffuse        => {
                let v = curve::<3, AnimatorableVec3>(data,  mode);
                cmds.vec3s().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexUScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexVScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexUOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexVOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskCutoff          => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Enable            => {
                let v = curve::<1, Enable>(data,  mode);
                cmds.enable().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::BoneOffset          => {
                let v = curve::<1, AnimatorableUint>(data,  mode);
                cmds.uints().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::IndicesRange        => {
                let v = curve::<2, IndiceRenderRange>(data,  mode);
                cmds.indicerange_curves().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Intensity => {
                false
            },
            EAnimePropertyID::CellId => {
                false
            },
            EAnimePropertyID::MainTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s().insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s().insert(key, TypeFrameCurve(v)).is_ok()
            },
        }
    }
    pub fn p3d_property_target_animation<T: TActionSet>(cmds: &mut T,
        key: u64,
        property: EAnimePropertyID,
        group: Entity,
        curve_target: Entity,
    ) -> bool {
        let info = match property {
            EAnimePropertyID::LocalPosition => {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalPosition, key));
            },
            EAnimePropertyID::LocalScaling =>  {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalScaling, key));
            },
            EAnimePropertyID::LocalRotation =>  {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalQuaternion, key));
            },
            EAnimePropertyID::LocalEulerAngles =>  {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalEuler, key));
            },
            EAnimePropertyID::Enable =>  {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::Enable, key));
            },
            EAnimePropertyID::IndicesRange =>  {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::IndicesRange, key));
            },
            EAnimePropertyID::CameraFov => {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::Fov, key));
            },
            EAnimePropertyID::CameraOrthSize => {
                cmds.anime_property_targetanime().push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::OrthSize, key));
            },
            EAnimePropertyID::CellId => {
                return false;
            },
            EAnimePropertyID::Intensity => {
                return false;
            },
            EAnimePropertyID::Alpha =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MainColor =>  {
                // if let Some(curve) = resource.anime_assets.vec3s.get(&key) {
                //     resource.anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MainTexUScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MainTexVScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MainTexUOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MainTexVOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::OpacityTexUScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::OpacityTexVScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::OpacityTexUOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::OpacityTexVOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::AlphaCutoff =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::LightDiffuse =>  {
                // if let Some(curve) = resource.anime_assets.vec3s.get(&key) {
                //     resource.anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MaskTexUScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MaskTexVScale =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MaskTexUOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MaskTexVOffset =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::MaskCutoff =>  {
                // if let Some(curve) = resource.anime_assets.float.get(&key) {
                //     resource.anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            EAnimePropertyID::BoneOffset =>  {
                // if let Some(curve) = resource.anime_assets.uints.get(&key) {
                //     resource.anime_contexts.uints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
                // } else { return false; }
            },
            _ => {}
        };

        return true;
    }

    pub fn p3d_camera<T: TActionSet>(cmds: &mut T, scene: Entity, id: Entity, graph: Entity) {
        cmds.transform_tree().push(OpsTransformNodeParent::ops(id, scene));
        cmds.camera_create().push(OpsCameraCreation::ops(scene, id, graph));
    }
    pub fn p3d_camera_param<T: TActionSet>(cmds: &mut T, camera: Entity, val: ECameraModify) {
        cmds.camera_param().push(OpsCameraModify::ops(camera, val));
    }
    pub fn p3d_camera_target<T: TActionSet>(cmds: &mut T, camera: Entity, x: f32, y: f32, z: f32) {
        cmds.camera_target().push(OpsCameraTarget::ops(camera, x as f32, y as f32, z as f32));
    }
    pub fn p3d_viewer_force_include<T: TActionSet>(cmds: &mut T, viewer: Entity, entity: Entity, add: bool) {
        cmds.camera_forceinclude().push(OpsViewerForceInclude::ops(viewer, entity, add));
    }
    pub fn p3d_create_vertex_buffer(cmds: &mut CommandsExchangeD3, key: String, data: Vec<u8> ) {
        let key = KeyVertexBuffer::from(key.as_str());
        cmds.verticesbuffers.push((key, data));
    }
    pub fn p3d_create_indices_buffer(cmds: &mut CommandsExchangeD3, key: String, data: Vec<u8>) {
        let key = KeyVertexBuffer::from(key.as_str());
        cmds.indicesbuffers.push((key, data));
    }
    pub fn p3d_instance_mesh<T: TActionSet>(cmds: &mut T, source: Entity, id: Entity) {
        cmds.instance_create().push(OpsInstanceMeshCreation::ops(source, id));
    }
    pub fn p3d_instance_attr<T: TActionSet>(cmds: &mut T, instance: Entity, attr: EInstanceAttr, key: pi_atom::Atom) {
        cmds.instance_attr().push(OpsInstanceAttr::ops(instance, attr, key ));
    }
    pub fn p3d_mesh_bone_offset<T: TActionSet>(cmds: &mut T, instance: Entity, val: u32) {
        cmds.mesh_valuestate().push(OpsAbstructMeshValueStateModify::ops(instance, EMeshValueStateModify::BoneOffset(val as u32)));
    }
    pub fn p3d_light<T: TActionSet>(cmds: &mut T, scene: Entity, id: Entity, ltype: f64) {
        cmds.transform_tree().push(OpsTransformNodeParent::ops(id, scene));
        cmds.light_create().push(OpsLightCreate::ops(scene, id, EngineConstants::light(ltype)));
    }
    pub fn p3d_light_param<T: TActionSet>(cmds: &mut T, light: Entity, val: ELightModify) {
        cmds.light_param().push(OpsLightParam::ops(light, val ));
    }
    pub fn p3d_light_forceinclude<T: TActionSet>(cmds: &mut T, light: Entity, mesh_or_instance: Entity, val: EMeshForceLighting) {
        cmds.forcelighting().push(OpsMeshForceLighting::ops(mesh_or_instance, light, val));
    }
    pub fn p3d_material_shader<T: TActionSet>(cmds: &mut T, mat: Entity, shader: &pi_atom::Atom, usematarray: bool) {
        cmds.material_create().push(OpsMaterialCreate::ops(mat, shader.as_str(), usematarray));
    }
    pub fn p3d_material_apply<T: TActionSet>(cmds: &mut T, mat: Entity, mesh: Entity, pass: f64) {
        let pass = EngineConstants::passtag(pass);
        cmds.material_usemat().push(OpsMaterialUse::ops(mesh, mat, pass));
    }
    pub fn p3d_material_uniform_value<T: TActionSet>(cmds: &mut T, entity: Entity, val: EUniformVal) {
        cmds.material_val().push( OpsUniformVal::ops( entity, val ) );
    }
    pub fn p3d_material_uniform_mat<T: TActionSet>(cmds: &mut T, mat: Entity,  key: &pi_atom::Atom, val: [f32;16]) {
        cmds.material_valb().push( OpsUniformValB::mat4(mat, key.clone(), val) );
    }
    pub fn p3d_material_uniform_tex<T: TActionSet>(cmds: &mut T, mat: Entity,  key: &pi_atom::Atom,
        url: &pi_atom::Atom,
        srgb: bool,
        compressed: bool,
        filter: bool,
        address_mode_u: f64,
        address_mode_v: f64,
        address_mode_w: f64,
        mag_filter: f64,
        min_filter: f64,
        mipmap_filter: f64,
        anisotropy_clamp: f64,
        border_color: f64,
        isfile: bool,
        cancombine: bool,
        compare: Option<f64>,
    ) {

        let address_mode_u = EngineConstants::address_mode(address_mode_u);
        let address_mode_v = EngineConstants::address_mode(address_mode_v);
        let address_mode_w = EngineConstants::address_mode(address_mode_w);
        let mag_filter = ContextConstants::filter_mode(mag_filter as u32);
        let min_filter = ContextConstants::filter_mode(min_filter as u32);
        let mipmap_filter = ContextConstants::filter_mode(mipmap_filter as u32);
        let compare = if let Some(compare) = compare { Some(ContextConstants::compare_function(compare as u32).val2()) } else { None };
        let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
        let border_color = EngineConstants::border_color(border_color);
        cmds.material_valb().push(
            OpsUniformValB::texture(
                mat,
                UniformTextureWithSamplerParam {
                    slotname: key.clone(),
                    wrapu: address_mode_u,
                    wrapv: address_mode_v,
                    wrapw: address_mode_w,
                    sample: crate::constants::sampler_desc(
                        EAddressMode::ClampToEdge,
                        EAddressMode::ClampToEdge,
                        EAddressMode::ClampToEdge,
                        mag_filter,
                        min_filter,
                        mipmap_filter,
                        compare,
                        anisotropy_clamp,
                        border_color,
                    ),
                    url: EKeyTexture::ImageFrame(KeyImageTextureViewFrame::new(
                        KeyImageTextureFrame { url: pi_atom::Atom::from(url.to_string()), cancombine, file: isfile, compressed },
                        TextureViewDesc {
                            // aspect: wgpu::TextureAspect::All,
                            base_mip_level: 0,
                            mip_level_count: None,
                            base_array_layer: 0,
                            array_layer_count: None,
                        }
                    )),
                    texture_sample: wgpu::TextureSampleType::Float { filterable: true },
                    sampler_bind_type: if filter { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering },
                }
            )
        );
    }
    
    pub fn p3d_material_uniform_tex_from_render_target<T: TActionSet>(cmds: &mut T, mat: Entity, key: &pi_atom::Atom, key_tilloff: &pi_atom::Atom, url: f64,
        filter: bool,
        address_mode_u: f64,
        address_mode_v: f64,
        address_mode_w: f64,
        mag_filter: f64,
        min_filter: f64,
        mipmap_filter: f64,
        anisotropy_clamp: f64,
        border_color: f64,
        compare: Option<f64>,
    ) {
        let address_mode_u = EngineConstants::address_mode(address_mode_u);
        let address_mode_v = EngineConstants::address_mode(address_mode_v);
        let address_mode_w = EngineConstants::address_mode(address_mode_w);
        let mag_filter = ContextConstants::filter_mode(mag_filter as u32);
        let min_filter = ContextConstants::filter_mode(min_filter as u32);
        let mipmap_filter = ContextConstants::filter_mode(mipmap_filter as u32);
        let compare = if let Some(compare) = compare { Some(ContextConstants::compare_function(compare as u32).val2()) } else { None };
        let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
        let border_color = EngineConstants::border_color(border_color);
        let texparam = UniformTextureWithSamplerParam { 
            slotname: key.clone(),
            wrapu: address_mode_u,
            wrapv: address_mode_v,
            wrapw: address_mode_w,
            sample: crate::constants::sampler_desc(
                EAddressMode::ClampToEdge,
                EAddressMode::ClampToEdge,
                EAddressMode::ClampToEdge,
                mag_filter,
                min_filter,
                mipmap_filter,
                compare,
                anisotropy_clamp,
                border_color,
            ),
            texture_sample: wgpu::TextureSampleType::Float { filterable: true },
            sampler_bind_type: if filter { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering },
            ..Default::default()
        };
        let key = unsafe { transmute(url) };
        cmds.material_valb().push(OpsUniformValB::texture_from_target(mat, texparam, key, key_tilloff.clone()));
    }
    pub fn p3d_node_material_block_regist<'w>(resource: &mut ResourceSets<'w>, block: &NodeMaterialBlock) {
        resource.node_material_blocks.0.insert(block.v0().clone(), block.v1().clone());
    }
    pub fn p3d_material_uniform_tex_from_renderer<T: TActionSet>(cmds: &mut T, mat: Entity, key: &pi_atom::Atom, key_tilloff: &pi_atom::Atom, url: Entity,
        filter: bool,
        address_mode_u: f64,
        address_mode_v: f64,
        address_mode_w: f64,
        mag_filter: f64,
        min_filter: f64,
        mipmap_filter: f64,
        anisotropy_clamp: f64,
        border_color: f64,
        compare: Option<f64>,
    ) {
        let address_mode_u = EngineConstants::address_mode(address_mode_u);
        let address_mode_v = EngineConstants::address_mode(address_mode_v);
        let address_mode_w = EngineConstants::address_mode(address_mode_w);
        let mag_filter = ContextConstants::filter_mode(mag_filter as u32);
        let min_filter = ContextConstants::filter_mode(min_filter as u32);
        let mipmap_filter = ContextConstants::filter_mode(mipmap_filter as u32);
        let compare = if let Some(compare) = compare { Some(ContextConstants::compare_function(compare as u32).val2()) } else { None };
        let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp);
        let border_color = EngineConstants::border_color(border_color);
        let texparam = UniformTextureWithSamplerParam { 
            slotname: key.clone(),
            wrapu: address_mode_u,
            wrapv: address_mode_v,
            wrapw: address_mode_w,
            sample: crate::constants::sampler_desc(
                EAddressMode::ClampToEdge,
                EAddressMode::ClampToEdge,
                EAddressMode::ClampToEdge,
                mag_filter,
                min_filter,
                mipmap_filter,
                compare,
                anisotropy_clamp,
                border_color,
            ),
            texture_sample: wgpu::TextureSampleType::Float { filterable: true },
            sampler_bind_type: if filter { wgpu::SamplerBindingType::Filtering } else { wgpu::SamplerBindingType::NonFiltering },
            ..Default::default()
        };
        cmds.material_valb().push(OpsUniformValB::texture_from_renderer(mat, texparam, url, key_tilloff.clone()));
    }
    
    pub fn p3d_uniform_target_animation<T: TActionSet>(cmds: &mut T,
        target: Entity,
        group: Entity,
        key: &pi_atom::Atom,
        curve_key: u64,
    ) {
        cmds.material_valb().push(OpsUniformValB::targetanim(target, key.clone(), group, curve_key));
    }
    pub fn p3d_load_texture(cmds: &mut CommandsExchangeD3, url: &pi_atom::Atom, compressed: bool, isfile: bool, cancombine: bool) {
        let key = KeyImageTextureFrame { url: pi_atom::Atom::from(url.to_string()), cancombine, file: isfile, compressed };
        cmds.loadtextures.push(key);
    }
    pub fn p3d_mesh<T: TActionSet>(cmds: &mut T, scene: Entity, id: Entity, instancestate: &VInstanceAttributes, instance_use_single_buffer: bool) {
        cmds.transform_tree().push(OpsTransformNodeParent::ops(id, scene));
        let state = MeshInstanceState { instances: instancestate.v1().clone(), instance_matrix: instancestate.v0(), use_single_instancebuffer: instance_use_single_buffer };
        cmds.mesh_create().push(OpsMeshCreation::ops(scene, id, state));
    }
    pub fn p3d_mesh_geometry<T: TActionSet>(cmds: &mut T, mesh: Entity, geometa: &GeometryMeta, geoid: Entity) {

        let geo: Entity = geoid;
        // log::error!("MeshGeo: {:?}", geometa.0);
        let mut vertices = vec![];
        let mut indices = None;
        match geometa.val1() {
            crate::geometry::EGeometry::Vec(vbmetas) => {
                vbmetas.iter().for_each(|vb| {
                    vertices.push( vb.desc() );
                });
                
                indices = if let Some(indice) = geometa.val2() {
                    let range = if let (Some(start), Some(end)) = (indice.1, indice.2) {
                        Some(Range { start: start as u32, end: end as u32 })
                    } else {
                        None
                    };

                    let ib = IndicesBufferDesc {
                        format: if indice.3 { wgpu::IndexFormat::Uint16 } else { wgpu::IndexFormat::Uint32 },
                        buffer_range: range,
                        buffer: KeyVertexBuffer::from(indice.0.as_str()),
                    };
                    Some(ib)
                } else { None };
            },
            crate::geometry::EGeometry::Quad => {
                vertices = QuadBuilder::attrs_meta();
                indices = None;
            },
            crate::geometry::EGeometry::Cube => {
                vertices = CubeBuilder::attrs_meta();
                indices = CubeBuilder::indices_meta();
            },
        }

        cmds.geometry_create().push(OpsGeomeryCreate::ops(mesh, geo, vertices, indices));
    }
    
    pub fn p3d_mesh_valuestate<T: TActionSet>(cmds: &mut T, mesh: Entity, val: EMeshValueStateModify) {
        cmds.mesh_valuestate().push(OpsAbstructMeshValueStateModify::ops(mesh, val));
    }
    pub fn p3d_mesh_render_state<T: TActionSet>(cmds: &mut T, mesh: Entity, val: ERenderState) {
        cmds.mesh_render_state().push(OpsRenderState::ops(mesh, val));
    }
    pub fn p3d_mesh_state<T: TActionSet>(cmds: &mut T, mesh: Entity, val: EMeshStateModify) {
        cmds.mesh_state().push(OpsMeshStateModify::ops(mesh, val));
    }
    pub fn p3d_mesh_bounding_box<T: TActionSet>(cmds: &mut T, mesh: Entity,
        minx: f64, miny: f64, minz: f64,
        maxx: f64, maxy: f64, maxz: f64
    ) {
        cmds.mesh_bounding().push(OpsMeshBounding::ops(mesh, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32)));
    }
    pub fn p3d_attribute_target_animation<T: TActionSet>(cmds: &mut T,
        target: Entity,
        group: Entity,
        key: &pi_atom::Atom,
        curve_key: u64,
    ) {
        let curve: u64 = unsafe { transmute(curve_key) };
        cmds.instance_targetanime().push(OpsTargetAnimationAttribute::ops(target, key.clone(), group, curve));
    }
    pub fn p3d_abstruct_pose_matrix<T: TActionSet>(cmds: &mut T, mesh: Entity, data: Vec<Number>) {
        cmds.mesh_pose().push(OpsAbstractMeshPose::ops(mesh, Matrix::from_column_slice(&data)));
    }
    pub fn p3d_regist_material(
        world: &World,
        key: &str,
        uniforms: &MaterialUniformDefines,
        vs_define_code: &str,
        fs_define_code: &str,
        vs_code: &str,
        fs_code: &str,
        includes: &NodematerialIncludes,
        instance_code: &str,
        varyings: &P3DShaderVaryings,
        binds_defines_base: Option<f64>,
    ) -> Option<P3DShaderMeta> {
        let mut nodemat = NodeMaterialBuilder::new();
        varyings.v0().iter().for_each(|v| { nodemat.varyings.0.push(v.clone()) });

        if let Some(binds_defines_base) = binds_defines_base {
            nodemat.binddefines = binds_defines_base as BindDefine;
        }
        nodemat.material_instance_code = String::from(instance_code);

        nodemat.values = uniforms.v0().clone();
        nodemat.textures = uniforms.v1().clone();

        let node_material_blocks = world.get_resource::<NodeMaterialBlocks>().unwrap();
        let shader_metas = world.get_resource::<ShareAssetMgr::<ShaderEffectMeta>>().unwrap();
        let enginopt = world.get_resource::<EngineCustomPlugins>().unwrap();
        
        includes.v0().iter().for_each(|val| {
            nodemat.include(val, node_material_blocks);
        });

        // log::warn!("Material {:?}", key);

        nodemat.vs_define += vs_define_code;
        nodemat.fs_define += fs_define_code;
        nodemat.vs = String::from(vs_code);
        nodemat.fs = String::from(fs_code);

        // log::error!("Material {:?} {:?}", key, &nodemat.fs);
        ActionMaterial::regist_material_meta(shader_metas, KeyShaderMeta::from(key), nodemat.meta(enginopt));

        if let Some(data) = shader_metas.get(&KeyShaderMeta::from(key)) {
            Some(P3DShaderMeta::new(data))
        } else { None }
    }
    pub fn p3d_particle_system<'w, T: TActionSet>(cmds: &mut T,
        scene: Entity,
        entity: Entity,
        trailmesh: Entity,
        trailgeo: Entity,
        calculator: u64,
        color_attr_key: &pi_atom::Atom,
        tilloff_attr_key: &pi_atom::Atom,
        update_buffer_interval_frame: Option<f64>,
    ) {
        let attrs = vec![
            ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
            ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.clone() },
            ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.clone() },
        ];
        let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
            update_buffer_interval_frame as u8
        } else { 0 };
        cmds.parsys_create().push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator, attrs, update_buffer_interval_frame));
    }
    // pub fn p3d_particle_system_with_gltf(cmds: &mut CommandsExchangeD3,
    //     scene: Entity,
    //     entity: Entity,
    //     trailmesh: Entity,
    //     trailgeo: Entity,
    //     gltf: &GLTFRes,
    //     index_calculator: f64,
    //     color_attr_key: &pi_atom::Atom,
    //     tilloff_attr_key: &pi_atom::Atom,
    //     update_buffer_interval_frame: Option<f64>,
    // ) {
    //     if let Some(calculator) = gltf_particle_calculator(&cmds, gltf, index_calculator) {
    //         let attrs = vec![
    //             ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
    //             ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.clone() },
    //             ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.clone() },
    //         ];
    //         let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
    //             update_buffer_interval_frame as u8
    //         } else { 0 };

    //         let calculator = *calculator.key();
    //         cmds.parsys_create().push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator, attrs, update_buffer_interval_frame));
    //     }
    // }
    pub fn p3d_particle_system_state<T: TActionSet>(cmds: &mut T, entity: Entity, val: ECPUParticleSystemState) {
        cmds.parsys_state().push( OpsCPUParticleSystemState::ops(entity, val) );
    }
    pub fn p3d_create_render_target(render_targets: &mut CustomRenderTargets, color_format: f64, depth_stencil_format: f64, width: f64, height: f64, filter: f64, address: f64, anisotropy_clamp: f64) -> Option<f64> {

        let filter = ContextConstants::filter_mode(filter as u32);
        let address = EngineConstants::address_mode(address);
        let anisotropy_clamp = EngineConstants::anisotropy_clamp(anisotropy_clamp );
        let sampler = KeySampler {
            address_mode_u: address, address_mode_v: address, address_mode_w: address, 
            mag_filter: filter, min_filter: filter, mipmap_filter: filter,
            border_color: None, anisotropy_clamp, compare: None
        };
        let color_format = EngineConstants::render_color_format(color_format);
        let depth_stencil_format = EngineConstants::render_depth_format(depth_stencil_format);

        if let Some(key) = render_targets.create(
            sampler, color_format, depth_stencil_format, width as u32, height as u32
        ) {
            Some(unsafe { transmute(key) })
        } else {
            None
        }
    }
    pub fn p3d_dispose_render_target<'w>(resource: &'w mut ResourceSets<'w>, key: f64) {
        resource.render_targets.delete(unsafe { transmute(key) });
    }
    pub fn p3d_create_render_subgraph<T: TActionSet>(cmds: &mut T, id_renderer: Entity, name: String) {
        cmds.renderer_subgraph().push(OpsSubGraphCreate::ops(id_renderer, name.clone()));
    }
    pub fn p3d_create_render<T: TActionSet>(cmds: &mut T, viewer: Entity, id_renderer: Entity, name: String, pass_tag: f64, transparent: bool, recordinput: Option<bool>, crossrender: Option<bool>) {
        let recordinput = if let Some(recordinput) = recordinput { recordinput } else { true };
        let crossrender = if let Some(crossrender) = crossrender { crossrender } else { false };
        cmds.renderer_create().push(OpsRendererCreate::ops(id_renderer, name.clone(), viewer, PassTag::new(pass_tag as u16), transparent, recordinput, crossrender));
    }
    pub fn p3d_renderer_modify<T: TActionSet>(cmds: &mut T, renderer: Entity, val: ERendererCommand) {
        cmds.renderer_modify().push(OpsRendererCommand::ops(renderer, val));
    }
    pub fn p3d_render_target<T: TActionSet>(cmds: &mut T, renderer: Entity, val: ERendererTarget) {
        cmds.renderer_target().push(OpsRendererTarget::new(renderer, val));
    }
    pub fn p3d_crossrender_link_drawlists(cmds: &mut CommandsExchangeD3, linkentity: Entity, drawlistrenderers: Vec<Entity>) {
        cmds.crossdrawlistinfo.push((linkentity, drawlistrenderers));
    }
    pub fn p3d_render_screenwithpostprocess(cmds: &mut CommandsExchangeD3, flag: bool) {
        cmds.screenwithpostprocess = flag;
    }
    pub fn p3d_scene<T: TActionSet>(cmds: &mut T, scene: Entity, cullingmode: f64, collidermode: f64, values: [i32; 9]) {
        cmds.scene_create().push(OpsSceneCreation::ops(scene, cullingmode as u8, collidermode as u8, values));
    }
    pub fn p3d_scene_option<T: TActionSet>(cmds: &mut T, scene: Entity, val: ESceneOps) {
        cmds.scene_options().push(OpsSceneOption::ops(scene, val));
    }
    pub fn p3d_layermask<T: TActionSet>(cmds: &mut T, node: Entity, val: u32) {
        cmds.mesh_layermask().push(OpsLayerMask::ops(node, val));
    }
    pub fn p3d_scene_boundingbox<T: TActionSet>(cmds: &mut T, scene: Entity, display: bool, pass: PassTag) {
        cmds.scene_boundingbox().push(OpsBoundingBoxDisplay::ops(scene, display, pass));
    }
    pub fn p3d_collider<T: TActionSet>(cmds: &mut T, node: Entity,
        minx: f32, miny: f32, minz: f32,
        maxx: f32, maxy: f32, maxz: f32,
        intersection_treshold: f32, alphaindex: i32
    ) {
        cmds.scene_collider().push(OpsCollider::new(node, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32), intersection_treshold as f32, alphaindex));
    }
    pub fn p3d_shadow_generator<T: TActionSet>(cmds: &mut T, scene: Entity, light: Entity, id: Entity, pass_tag: u16, graph: Entity) {
        cmds.shadow_create().push(OpsShadowGenerator::ops(id, scene, light, PassTag::new(pass_tag as u16), graph));
        cmds.renderer_create().push(OpsRendererCreate::ops(id, String::from("Shadow") + id.index().to_string().as_str(), id, PassTag::new(pass_tag as u16), false, false, false));
    }
    pub fn p3d_shadow_base_param<T: TActionSet>(cmds: &mut T, shadow: Entity, val: EShadowGeneratorParam) {
        cmds.shadow_param().push(OpsShadowGeneratorParam(shadow, val));
    }
    pub fn p3d_skeleton<T: TActionSet>(cmds: &mut T, id: Entity, bonespervertex: u8, root: Entity, bones: &[Entity], bonecount: f64, cacheframe: f64) {
        let state = match (bonespervertex as u8) {
            1 => ESkinBonesPerVertex::One,
            2 => ESkinBonesPerVertex::Two,
            3 => ESkinBonesPerVertex::Three,
            _ => ESkinBonesPerVertex::Four
        };

        cmds.skin_create().push(OpsSkinCreation::ops(id, state, root, &bones, cacheframe as u16, None));
    }
    pub fn p3d_bone<T: TActionSet>(cmds: &mut T, id: Entity, scene: Entity) {
        cmds.skin_bonecreate().push(OpsBoneCreation::ops(id, scene));
    }
    pub fn p3d_bone_link<T: TActionSet>(cmds: &mut T, bone: Entity, link: Entity) {
        cmds.skin_use().push(OpsSkinUse::bone_link(bone, link));
    }
    pub fn p3d_bone_pose<T: TActionSet>(cmds: &mut T, bone: Entity, data: &[f32]) {
        let matrix = Matrix::new(
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]
        );
        cmds.skin_bonepose().push(OpsBonePose::ops(bone, matrix));
    }
    pub fn p3d_skin_use<T: TActionSet>(cmds: &mut T, id_mesh: Entity, skin: Entity) {
        cmds.skin_use().push(OpsSkinUse::ops(id_mesh, skin));
    }
    pub fn p3d_sprite<T: TActionSet>(cmds: &mut T, source: Entity, id: Entity, atlas: &pi_atom::Atom) {
        cmds.instance_create().push(OpsInstanceMeshCreation::ops(source, id));
        cmds.sprite_create().push(OpsSpriteCreate::ops(source, id, atlas.to_string().asset_u64()));
    }
    pub fn p3d_sprite_frame<T: TActionSet>(cmds: &mut T, sprite: Entity, tilloffkey: &pi_atom::Atom, idxframe: f64) {
        cmds.sprite_modify().push(OpsSpriteModify::ops(sprite, SpriteModify::Idx(idxframe as IdxTextureFrame), tilloffkey.clone()));
    }
    pub fn p3d_sprite_frame_data<T: TActionSet>(cmds: &mut T, sprite: Entity, tilloffkey: &pi_atom::Atom, data: &[u16]) {
        let data = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
            data[10], data[11], data[12], data[13],
        ];
        cmds.sprite_modify().push(OpsSpriteModify::ops(sprite, SpriteModify::Data(data), tilloffkey.clone()));
    }
    pub fn p3d_record_sprite_frame_data(cmds: &mut CommandsExchangeD3, sprite: SpriteFrame) -> usize {
        cmds.sprite_frames.1.push(sprite);
        let result = cmds.sprite_frames.0;
        cmds.sprite_frames.0 = cmds.sprite_frames.0 + 1;
        result
    }
    pub fn p3d_create_data_texture(datatexcmd: &mut DataTextureCmds, key: &pi_atom::Atom, width: u32, height: u32, format: f64, aspect: Option<f64>) {
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
        datatexcmd.createdata.insert(key.clone(), (info, format, dimension, texkey));
    }
    pub fn p3d_update_data_texture(datatexcmd: &mut DataTextureCmds, key: &pi_atom::Atom, data:&[u8], xoffset: u32, yoffset: u32, width: u32, height: u32, aspect: Option<f64>) {

        let key = key.clone();
        if datatexcmd.updatedata.contains_key(&key) == false {
            datatexcmd.updatedata.insert(key.clone(), vec![]);
        }
        if let Some(list) = datatexcmd.updatedata.get_mut(&key) {
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
    pub fn p3d_remove_data_texture(datatexcmd: &mut DataTextureCmds, key: &pi_atom::Atom) {
        datatexcmd.createdata.remove(key);
        datatexcmd.updatedata.remove(key);
        datatexcmd.record.remove(key);
    }
    pub fn p3d_texture_combine_param(device: &PiRenderDevice, cmds: &mut ResTextureCombineAtlas2DMgr, format: f64, maxlayer: f64, maxsize: f64, maxcount: f64) {
        let format = EngineConstants::texture_format(format);
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
    
    pub fn p3d_trail<T: TActionSet>(cmds: &mut T, scene: Entity, entity: Entity, linked: Entity) {
        cmds.trail_create().push(OpsTrail::ops(scene, linked, entity));
    }
    pub fn p3d_trail_age<T: TActionSet>(cmds: &mut T, entity: Entity, age_ms: u32) {
        cmds.trail_age().push(OpsTrailAgeControl::ops(entity, age_ms));
    }
    pub fn p3d_transform_node<T: TActionSet>(cmds: &mut T, scene: Entity, id: Entity) {
        cmds.transform_tree().push(OpsTransformNodeParent::ops(id, scene));
        cmds.transform_create().push(OpsTransformNode::ops(scene, id));
    }
    pub fn p3d_transform_node_parent<T: TActionSet>(cmds: &mut T, node: Entity, parent: Entity) {
        cmds.transform_tree().push(OpsTransformNodeParent::ops(node, parent));
    }
    pub fn p3d_node_enable<T: TActionSet>(cmds: &mut T, node: Entity, val: bool) {
        cmds.transform_enable().push(OpsNodeEnable::ops(node, val));
    }
    pub fn p3d_local_srt<T: TActionSet>(cmds: &mut T, node: Entity, val: ETransformSRT) {
        cmds.transform_localsrt().push(OpsTransformNodeLocal::ops(node, val));
    }
    pub fn p3d_local_quaternion<T: TActionSet>(cmds: &mut T, node: Entity, x: f32, y: f32, z: f32, w: f32) {
        cmds.transform_localrotq().push(OpsTransformNodeLocalRotationQuaternion::ops(node, x as f32, y as f32, z as f32, w as f32));
    }
}

pub fn commands_exchange_call(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3) {

    let playstate = app.world.get_resource::<PlayState>().unwrap();
    match playstate.option {
        pi_bevy_render_plugin::TraceOption::Play => {
            
        },
        pi_bevy_render_plugin::TraceOption::None => {},
        pi_bevy_render_plugin::TraceOption::Record => {
            let records = app.world.get_resource_mut::<Records>().unwrap();
            match postcard::to_stdvec::<Vec<ERecord3D>>(&cmds.recordframes) {
                Ok(data) => {
                    records.record(RECORD_D3_COMMAND, data);
                }
                Err(_) => {}
            };
            cmds.recordframes.clear();
        },
    }

    let state = param.state.get_mut(&mut app.world);

    if state.stateengine.active {
        let mut sets = param.acts.get_mut(&mut app.world);
    
        cmds.exchange(&mut sets);
    }

    let spriteframes = app.world.get_resource_mut::<ResSpriteFrames>().unwrap();
    cmds.sprite_frames.1.drain(..).for_each(|v| {
        spriteframes.push(v);
    });

    let crossrenderinfos = app.world.get_resource_mut::<pi_bevy_render_plugin::render_cross::CrossRenderDrawListEntities>().unwrap();
    cmds.crossdrawlistinfo.drain(..).for_each(|(link, list)| {
        if list.len() > 0 {
            crossrenderinfos.0.insert(link, list);
        } else {
            crossrenderinfos.0.remove(&link);
        }
    });

    let datatexcmd = app.world.get_resource_mut::<DataTextureCmds>().unwrap();
    cmds.datatexcmd.createdata.drain().for_each(|(k, v)| {
        datatexcmd.createdata.insert(k, v);
    });
    cmds.datatexcmd.updatedata.drain().for_each(|(k, v)| {
        datatexcmd.updatedata.insert(k, v);
    });
    cmds.datatexcmd.record.drain().for_each(|(k, v)| {
        datatexcmd.record.insert(k, v);
    });

    cmds.disposegltfs.drain(..).for_each(|key| {
        app.world.get_resource_mut::<ResGLTFRecords>().unwrap().0.remove(&key);
    });
    *cmds.gltfs_mut() = app.world.get_resource::<ResGLTFRecords>().unwrap().0.clone();

    let imgtex_asset = app.world.get_resource::<ShareAssetMgr<ImageTextureFrame>>().unwrap().clone();
    let requests = app.world.get_resource_mut::<TextureCombineCmds>().unwrap();
    cmds.combinecmds.drain().for_each(|(requestid, (key, atlas))| {
        let keytex = KeyImageTextureFrame { url: key, file: false, compressed: false, cancombine: false };
        requests.request(requestid, keytex, atlas, &imgtex_asset);
    });

    let texloader = app.world.get_resource_mut::<ResImageTextureLoader>().unwrap();
    while let Some(key) = cmds.loadtextures.pop() {
        texloader.create_load(key);
    }

    let actions = app.world.get_resource_mut::<ActionListCustomBuffer>().unwrap();

    while let Some((key, data)) = cmds.verticesbuffers.pop() {
        actions.push((key, data, false));
    }
    while let Some((key, data)) = cmds.indicesbuffers.pop() {
        actions.push((key, data, true));
    }
    while let Some((key, data)) = cmds.indicesbuffersu32.pop() {
        actions.push((key, data, true));
    }

    let screenwithpostprocess = app.world.get_resource_mut::<pi_bevy_render_plugin::ScreenWithPostprocess>().unwrap();
    screenwithpostprocess.0 = cmds.screenwithpostprocess;

}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_commands_exchange(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3) {
	crate::export::await_last_frame(app);

    commands_exchange_call(app, param, cmds);
}