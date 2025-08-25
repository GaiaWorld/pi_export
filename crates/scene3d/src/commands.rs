use std::{mem::transmute, ops::{Deref, Range}};

use pi_3d::ActionSets;
use pi_export_base::{asset::ActionListCustomBuffer, export::{update_data_texture, DataTextureCmds}};
use pi_gltf2_load::GLTF;
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_particle_system::prelude::*;
use pi_scene_context::prelude::*;
use pi_trail_renderer::*;
use pi_hash::XHashMap;
use pi_curves::curve::frame::KeyFrameCurveValue;
pub use crate::engine::ActionSetScene3D;
use crate::{animation::{EAmountMode, EFillMode, ELoopMode}, cmd_call::_amountcalc, engine::{gltf_particle_calculator, GLTFRes}, mesh::{GeometryMeta, VInstanceAttributes}, node_materials::{MaterialUniformDefines, NodeMaterialBlock, NodematerialIncludes, P3DShaderMeta, P3DShaderVaryings}, record::{ERecordCMD, ERecordMode}};
use crate::animation::EAnimePropertyID;
use crate::animation::EAnimeCurve;
use crate::animation::curve;
use crate::constants::EngineConstants;
use pi_export_base::constants::ContextConstants;
use pi_mesh_builder::{quad::QuadBuilder, cube::CubeBuilder};
use pi_node_materials::prelude::NodeMaterialBuilder;
use pi_node_materials::NodeMaterialBlocks;
use pi_slotmap::Key;
use pi_export_base::export::DataTextureSubData;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Default)]
pub struct CommandsExchangeD3 {
    pub(crate) frame: u64,
    pub(crate) recordmode: ERecordMode,
    pub(crate) recordframes: Vec<(u32, Vec<ERecordCMD>)>,
    pub(crate) replayentities: XHashMap<Entity, Entity>,

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
    pub(crate) gltfs: XHashMap<u64, Handle<GLTF>>,
    pub(crate) gltfcounter: u64,
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl CommandsExchangeD3 {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create() -> Self {
        Self::default()
    }
}

impl CommandsExchangeD3 {
    pub(crate) fn capacity(&self, cmds: & pi_3d::ActionSets) -> usize {
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
        self.anime_float .capacity() +
        self.anime_sint .capacity() +
        self.anime_uint .capacity() +
        self.anime_vec2 .capacity() +
        self.anime_vec3 .capacity() +
        self.anime_vec4 .capacity() +
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
    pub(crate) fn exchange(&mut self, cmds: &mut pi_3d::ActionSets) {
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
    pub fn p3d_animation_curve_id_bygltf(
        cmds: &CommandsExchangeD3,
        gltf: &GLTFRes,
        group_index: usize,
        channel_index: usize,
    ) -> f64 {
        if let Some(gltf) = cmds.gltfs.get(&gltf.0) {
            let key = gltf.key_anime_curve(group_index as usize, channel_index as usize);
            unsafe { transmute(key) }
        } else {
            0.
        }
    }
    pub(crate) fn p3d_entity(app: &mut Engine) -> Entity {
        app.world.entities().reserve_entity()
    }
    pub(crate) fn p3d_dispose(&mut self, entity: Entity) {
        self.obj_dispose.push(OpsDispose::ops(entity));
    }
    pub(crate) fn p3d_scene_dispose(&mut self, scene: Entity) {
        self.scene_dispose.push(OpsSceneDispose::ops(scene));
    }
    pub(crate) fn p3d_lighting_shadow_limit(
        app: &mut Engine, param: &mut ActionSetScene3D,
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
        let mut resource = param.resource.get_mut(&mut app.world);

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
    pub fn p3d_engine_state(app: &mut Engine, param: &mut ActionSetScene3D, active: bool) {
        let mut cmds = param.state.get_mut(&mut app.world);
        cmds.stateengine.active = active;
    }
    pub fn p3d_render_graphic(&mut self, before: Entity, after: Entity, isdisconnect: bool) {
        self.renderer_connect.push(OpsRendererConnect::ops(before, after, isdisconnect));
    }
    pub fn p3d_engine_debug(app: &mut Engine, param: &mut ActionSetScene3D, debug: bool) {
        let mut cmds = param.state.get_mut(&mut app.world);
        cmds.performance.debug = debug;
        cmds.psperformance.debug = debug;
    }
    pub fn p3d_create_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, entity: Entity, baseurl: pi_atom::Atom, dyndesc: String) {
        let resource = param.resource.get_mut(&mut app.world);
        let param = baseurl;
        resource.gltf2_loader.create_load(entity, param);
    }
    pub fn p3d_get_gltf(&mut self, app: &mut Engine, param: &mut ActionSetScene3D, entity: Entity) -> Option<GLTFRes> {
        let mut resource = param.resource.get_mut(&mut app.world);
        if let Some(val) = resource.gltf2_loader.get_success(entity) {
            let id = self.gltfcounter;
            self.gltfcounter += 1;
            self.gltfs.insert(id, val);
            Some(GLTFRes(id))
        } else {
            None
        }
    }
    pub fn p3d_dispose_gltf(&mut self, entity: &GLTFRes) {
        self.gltfs.remove(&entity.0);
    }
    pub fn _create_image_load(app: &mut Engine, param: &mut ActionSetScene3D, url: pi_atom::Atom, cancombine: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
        let mut resource = param.resource.get_mut(&mut app.world);
        let id = resource.imgtex_loader.create_load(KeyImageTextureFrame { 
            url,
            file: true,
            compressed,
            cancombine
        });
        unsafe { transmute(id) }
    }
    pub fn p3d_animation_group(&mut self, scene: Entity, id: Entity) {
        self.anime_create.push(OpsAnimationGroupCreation::ops(scene, id));
    }
    pub fn p3d_animation_group_weight(&mut self, group: Entity, weight: f32) {
        self.anime_action.push(OpsAnimationGroupAction::weight(group, weight as f32));
    }
    pub fn p3d_animation_group_target_reset(cmds: &mut CommandsExchangeD3, group: Entity) {
        cmds.anime_reset_while_start.push(OpsAnimationGroupStartReset::ops(group));
    }
    pub fn p3d_anime_group_start(
        &mut self,
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
        self.anime_action.push(OpsAnimationGroupAction::Start(group_key, AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as FramePerSecond, amountcalc), delay_ms as KeyFrameCurveValue, fillmode));
    }
    pub fn p3d_anime_group_pause(
        cmds: &mut CommandsExchangeD3,
        group_key: Entity,
    ) {
        cmds.anime_action.push(OpsAnimationGroupAction::Pause(group_key));
    }
    pub fn p3d_anime_group_stop(
        cmds: &mut CommandsExchangeD3,
        group_key: Entity,
    ) {
        cmds.anime_action.push(OpsAnimationGroupAction::Stop(group_key));
    }
    pub fn p3d_anime_group_goto(
        cmds: &mut CommandsExchangeD3,
        group_key: Entity,
        amount: KeyFrameCurveValue,
    ) {
        cmds.anime_goto.push(AnimationGroupGoto::ops(group_key, amount));
    }
    pub fn p3d_animation_group_delete(
        cmds: &mut CommandsExchangeD3,
        group: Entity,
    ) {
        cmds.anime_dispose.push(OpsAnimationGroupDispose::ops(group));
    }
    pub fn p3d_anime_curve_create(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyID, data: &[f32], mode: EAnimeCurve) -> bool {
        let resource = param.resource.get_mut(&mut app.world);
        let key: u64 = unsafe { transmute(key) };

        let cmds = resource.anime_assets;

        match property {
            EAnimePropertyID::LocalPosition       => {
                let v = curve::<3, LocalPosition>(data,  mode);
                cmds.position.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalScaling        => {
                let v = curve::<3, LocalScaling>(data,  mode);
                cmds.scaling.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalRotation    => {
                let v = curve::<4, LocalRotationQuaternion>(data,  mode);
                cmds.quaternion.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LocalEulerAngles    => {
                let v = curve::<3, LocalEulerAngles>(data,  mode);
                cmds.euler.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Alpha               => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainColor           => {
                let v = curve::<3, AnimatorableVec3>(data,  mode);
                cmds.vec3s.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexUScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexVScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexUOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MainTexVOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexUScale    => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexVScale    => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexUOffset   => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexVOffset   => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::AlphaCutoff         => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::CameraFov           => {
                let v = curve::<1, CameraFov>(data,  mode);
                cmds.camerafov.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::CameraOrthSize      => {
                let v = curve::<1, CameraOrthSize>(data,  mode);
                cmds.camerasize.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::LightDiffuse        => {
                let v = curve::<3, AnimatorableVec3>(data,  mode);
                cmds.vec3s.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexUScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexVScale       => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexUOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexVOffset      => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskCutoff          => {
                let v = curve::<1, AnimatorableFloat>(data,  mode);
                cmds.float.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Enable            => {
                let v = curve::<1, Enable>(data,  mode);
                cmds.enable.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::BoneOffset          => {
                let v = curve::<1, AnimatorableUint>(data,  mode);
                cmds.uints.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::IndicesRange        => {
                let v = curve::<2, IndiceRenderRange>(data,  mode);
                cmds.indicerange_curves.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::Intensity => {
                false
            },
            EAnimePropertyID::CellId => {
                false
            },
            EAnimePropertyID::MainTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::MaskTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s.insert(key, TypeFrameCurve(v)).is_ok()
            },
            EAnimePropertyID::OpacityTexTilloff        => {
                let v = curve::<4, AnimatorableVec4>(data,  mode);
                cmds.vec4s.insert(key, TypeFrameCurve(v)).is_ok()
            },
        }
    }
    pub fn p3d_property_target_animation(
        cmds: &mut CommandsExchangeD3,
        key: u64,
        property: EAnimePropertyID,
        group: Entity,
        curve_target: Entity,
    ) -> bool {
        let info = match property {
            EAnimePropertyID::LocalPosition => {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalPosition, key));
            },
            EAnimePropertyID::LocalScaling =>  {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalScaling, key));
            },
            EAnimePropertyID::LocalRotation =>  {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalQuaternion, key));
            },
            EAnimePropertyID::LocalEulerAngles =>  {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::LocalEuler, key));
            },
            EAnimePropertyID::Enable =>  {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::Enable, key));
            },
            EAnimePropertyID::IndicesRange =>  {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::IndicesRange, key));
            },
            EAnimePropertyID::CameraFov => {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::Fov, key));
            },
            EAnimePropertyID::CameraOrthSize => {
                cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(curve_target, group, EPropertyAnimationValueType::OrthSize, key));
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

    pub fn p3d_camera(cmds: &mut CommandsExchangeD3, scene: Entity, id: Entity, graph: Entity) {
        cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
        cmds.camera_create.push(OpsCameraCreation::ops(scene, id, graph));
    }
    pub fn p3d_camera_param(cmds: &mut CommandsExchangeD3, camera: Entity, val: ECameraModify) {
        cmds.camera_param.push(OpsCameraModify::ops(camera, val));
    }
    pub fn p3d_camera_target(cmds: &mut CommandsExchangeD3, camera: Entity, x: f32, y: f32, z: f32) {
        cmds.camera_target.push(OpsCameraTarget::ops(camera, x as f32, y as f32, z as f32));
    }
    pub fn p3d_viewer_force_include(cmds: &mut CommandsExchangeD3, viewer: Entity, entity: Entity, add: bool) {
        cmds.camera_forceinclude.push(OpsViewerForceInclude::ops(viewer, entity, add));
    }
    pub fn p3d_create_vertex_buffer(cmds: &mut CommandsExchangeD3, key: String, data: Vec<u8> ) {
        let key = KeyVertexBuffer::from(key.as_str());
        cmds.verticesbuffers.push((key, data));
    }
    pub fn p3d_create_indices_buffer(cmds: &mut CommandsExchangeD3, key: String, data: Vec<u8>) {
        let key = KeyVertexBuffer::from(key.as_str());
        cmds.indicesbuffers.push((key, data));
    }
    pub fn p3d_instance_mesh(cmds: &mut CommandsExchangeD3, source: Entity, id: Entity) {
        cmds.instance_create.push(OpsInstanceMeshCreation::ops(source, id));
    }
    pub fn p3d_instance_attr(cmds: &mut CommandsExchangeD3, instance: Entity, attr: EInstanceAttr, key: pi_atom::Atom) {
        cmds.instance_attr.push(OpsInstanceAttr::ops(instance, attr, key ));
    }
    pub fn p3d_mesh_bone_offset(cmds: &mut CommandsExchangeD3, instance: Entity, val: u32) {
        cmds.mesh_valuestate.push(OpsAbstructMeshValueStateModify::ops(instance, EMeshValueStateModify::BoneOffset(val as u32)));
    }
    pub fn p3d_light(app: &mut Engine, cmds: &mut CommandsExchangeD3, scene: Entity, id: Entity, ltype: f64) {
        cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
        cmds.light_create.push(OpsLightCreate::ops(scene, id, EngineConstants::light(ltype)));
    }
    pub fn p3d_light_param(cmds: &mut CommandsExchangeD3, light: Entity, val: ELightModify) {
        cmds.light_param.push(OpsLightParam::ops(light, val ));
    }
    pub fn p3d_light_forceinclude(cmds: &mut CommandsExchangeD3, light: Entity, mesh_or_instance: Entity, val: EMeshForceLighting) {
        cmds.forcelighting.push(OpsMeshForceLighting::ops(mesh_or_instance, light, val));
    }
    pub fn p3d_material_shader(cmds: &mut CommandsExchangeD3, mat: Entity, shader: &pi_atom::Atom, usematarray: bool) {
        cmds.material_create.push(OpsMaterialCreate::ops(mat, shader.as_str(), usematarray));
    }
    pub fn p3d_material_apply(cmds: &mut CommandsExchangeD3, mat: Entity, mesh: Entity, pass: f64) {
        let pass = EngineConstants::passtag(pass);
        cmds.material_usemat.push(OpsMaterialUse::ops(mesh, mat, pass));
    }
    pub fn p3d_material_uniform_value(cmds: &mut CommandsExchangeD3, entity: Entity, val: EUniformVal) {
        cmds.material_val.push( OpsUniformVal::ops( entity, val ) );
    }
    pub fn p3d_material_uniform_mat(cmds: &mut CommandsExchangeD3, mat: Entity,  key: &pi_atom::Atom, val: [f32;16]) {
        cmds.material_valb.push( OpsUniformValB::mat4(mat, key.clone(), val) );
    }
    pub fn p3d_material_uniform_tex(
        cmds: &mut CommandsExchangeD3, mat: Entity,  key: &pi_atom::Atom,
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
        cmds.material_valb.push(
            OpsUniformValB::texture(
                mat,
                UniformTextureWithSamplerParam {
                    slotname: key.clone(),
                    wrapu: address_mode_u,
                    wrapv: address_mode_v,
                    wrapw: address_mode_w,
                    sample: pi_export_base::constants::sampler_desc(
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
    
    pub fn p3d_material_uniform_tex_from_render_target(
        cmds: &mut CommandsExchangeD3, mat: Entity, key: &pi_atom::Atom, key_tilloff: &pi_atom::Atom, url: f64,
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
            sample: pi_export_base::constants::sampler_desc(
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
        cmds.material_valb.push(OpsUniformValB::texture_from_target(mat, texparam, key, key_tilloff.clone()));
    }
    pub fn p3d_node_material_block_regist(app: &mut Engine, param: &mut ActionSetScene3D, block: &NodeMaterialBlock) {

        let mut resource = param.resource.get_mut(&mut app.world);
        resource.node_material_blocks.0.insert(block.0.clone(), block.1.clone());
    }
    pub fn p3d_material_uniform_tex_from_renderer(
        cmds: &mut CommandsExchangeD3, mat: Entity, key: &pi_atom::Atom, key_tilloff: &pi_atom::Atom, url: Entity,
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
            sample: pi_export_base::constants::sampler_desc(
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
        cmds.material_valb.push(OpsUniformValB::texture_from_renderer(mat, texparam, url, key_tilloff.clone()));
    }
    
    pub fn p3d_uniform_target_animation(
        cmds: &mut CommandsExchangeD3,
        target: Entity,
        group: Entity,
        key: &pi_atom::Atom,
        curve_key: f64,
    ) {
        let curve: u64 = unsafe { transmute(curve_key) };
        cmds.material_valb.push(OpsUniformValB::targetanim(target, key.clone(), group, curve));
    }
    pub fn p3d_load_texture(cmds: &mut CommandsExchangeD3, url: &pi_atom::Atom, compressed: bool, isfile: bool, cancombine: bool) {
        let key = KeyImageTextureFrame { url: pi_atom::Atom::from(url.to_string()), cancombine, file: isfile, compressed };
        cmds.loadtextures.push(key);
    }
    pub fn p3d_mesh(cmds: &mut CommandsExchangeD3, scene: Entity, id: Entity, instancestate: &VInstanceAttributes, instance_use_single_buffer: bool) {
        cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
        let state = MeshInstanceState { instances: instancestate.1.clone(), instance_matrix: instancestate.0, use_single_instancebuffer: instance_use_single_buffer };
        cmds.mesh_create.push(OpsMeshCreation::ops(scene, id, state));
    }
    pub fn p3d_mesh_geometry(cmds: &mut CommandsExchangeD3, mesh: Entity, geometa: &GeometryMeta, geoid: Entity) {

        let geo: Entity = geoid;
        // log::error!("MeshGeo: {:?}", geometa.0);
        let mut vertices = vec![];
        let mut indices = None;
        match &geometa.0 {
            crate::geometry::EGeometry::Vec(vbmetas) => {
                vbmetas.iter().for_each(|vb| {
                    vertices.push( VertexBufferDesc::new(vb.key.clone(), vb.range.clone(), vb.attrs(), vb.instance) );
                });
                
                indices = if let Some(indice) = &geometa.1 {
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

        cmds.geometry_create.push(OpsGeomeryCreate::ops(mesh, geo, vertices, indices));
    }
    
    pub fn p3d_mesh_valuestate(cmds: &mut CommandsExchangeD3, mesh: Entity, val: EMeshValueStateModify) {
        cmds.mesh_valuestate.push(OpsAbstructMeshValueStateModify::ops(mesh, val));
    }
    pub fn p3d_mesh_render_state(cmds: &mut CommandsExchangeD3, mesh: Entity, val: ERenderState) {
        cmds.mesh_render_state.push(OpsRenderState::ops(mesh, val));
    }
    pub fn p3d_mesh_state(cmds: &mut CommandsExchangeD3, mesh: Entity, val: EMeshStateModify) {
        cmds.mesh_state.push(OpsMeshStateModify::ops(mesh, val));
    }
    pub fn p3d_mesh_bounding_box(
        cmds: &mut CommandsExchangeD3, mesh: Entity,
        minx: f64, miny: f64, minz: f64,
        maxx: f64, maxy: f64, maxz: f64
    ) {
        cmds.mesh_bounding.push(OpsMeshBounding::ops(mesh, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32)));
    }
    pub fn p3d_attribute_target_animation(
        cmds: &mut CommandsExchangeD3,
        target: Entity,
        group: Entity,
        key: &pi_atom::Atom,
        curve_key: u64,
    ) {
        let curve: u64 = unsafe { transmute(curve_key) };
        cmds.instance_targetanime.push(OpsTargetAnimationAttribute::ops(target, key.clone(), group, curve));
    }
    pub fn p3d_abstruct_pose_matrix(
        cmds: &mut CommandsExchangeD3, mesh: Entity, data: Vec<Number>) {
        cmds.mesh_pose.push(OpsAbstractMeshPose::ops(mesh, Matrix::from_column_slice(&data)));
    }
    pub fn p3d_regist_material(
        app: &Engine,
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
        varyings.0.iter().for_each(|v| { nodemat.varyings.0.push(v.clone()) });

        if let Some(binds_defines_base) = binds_defines_base {
            nodemat.binddefines = binds_defines_base as BindDefine;
        }
        nodemat.material_instance_code = String::from(instance_code);

        nodemat.values = uniforms.0.clone();
        nodemat.textures = uniforms.1.clone();

        let node_material_blocks = app.world.get_resource::<NodeMaterialBlocks>().unwrap();
        let shader_metas = app.world.get_resource::<ShareAssetMgr::<ShaderEffectMeta>>().unwrap();
        let enginopt = app.world.get_resource::<EngineCustomPlugins>().unwrap();
        
        includes.0.iter().for_each(|val| {
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
            Some(P3DShaderMeta(data))
        } else { None }
    }
    pub fn p3d_particle_system(
        app: &mut Engine,
        param: &mut ActionSetScene3D,
        cmds: &mut CommandsExchangeD3,
        scene: Entity,
        entity: Entity,
        trailmesh: Entity,
        trailgeo: Entity,
        key: &pi_atom::Atom,
        color_attr_key: &pi_atom::Atom,
        tilloff_attr_key: &pi_atom::Atom,
        update_buffer_interval_frame: Option<f64>,
    ) {
        let reosurce = param.resource.get_mut(&mut app.world);
        if let Some(calculator) = reosurce.particlesys.calcultors.get(&key.asset_u64()) {
            let attrs = vec![
                ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
                ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.clone() },
                ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.clone() },
            ];
            let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
                update_buffer_interval_frame as u8
            } else { 0 };
            cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator, attrs, update_buffer_interval_frame));
        }
    }
    pub fn p3d_particle_system_with_gltf(
        cmds: &mut CommandsExchangeD3,
        scene: Entity,
        entity: Entity,
        trailmesh: Entity,
        trailgeo: Entity,
        gltf: &GLTFRes,
        index_calculator: f64,
        color_attr_key: &pi_atom::Atom,
        tilloff_attr_key: &pi_atom::Atom,
        update_buffer_interval_frame: Option<f64>,
    ) {
        if let Some(calculator) = gltf_particle_calculator(&cmds, gltf, index_calculator) {
            let attrs = vec![
                ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: pi_atom::Atom::from("") },
                ParticleAttribute { vtype: EParticleAttributeType::Color, attr: color_attr_key.clone() },
                ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: tilloff_attr_key.clone() },
            ];
            let update_buffer_interval_frame = if let Some(update_buffer_interval_frame) = update_buffer_interval_frame {
                update_buffer_interval_frame as u8
            } else { 0 };
            cmds.parsys_create.push(OpsCPUParticleSystem::ops(scene, entity, trailmesh, trailgeo, calculator.clone(), attrs, update_buffer_interval_frame));
        }
    }
    pub fn p3d_particle_system_state(cmds: &mut CommandsExchangeD3, entity: Entity, val: ECPUParticleSystemState) {
        cmds.parsys_state.push( OpsCPUParticleSystemState::ops(entity, val) );
    }
    pub fn p3d_create_render_target(app: &mut Engine, param: &mut ActionSetScene3D, color_format: f64, depth_stencil_format: f64, width: f64, height: f64, filter: f64, address: f64, anisotropy_clamp: f64) -> Option<f64> {
        let mut resource = param.resource.get_mut(&mut app.world);
        
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

        if let Some(key) = resource.render_targets.create(
            sampler, color_format, depth_stencil_format, width as u32, height as u32
        ) {
            Some(unsafe { transmute(key) })
        } else {
            None
        }
    }
    pub fn p3d_dispose_render_target(app: &mut Engine, param: &mut ActionSetScene3D, key: f64) {
        let mut resource = param.resource.get_mut(&mut app.world);
        resource.render_targets.delete(unsafe { transmute(key) });
    }
    pub fn p3d_create_render_subgraph(app: &mut Engine, cmds: &mut CommandsExchangeD3, id_renderer: Entity, name: String) {
        cmds.renderer_subgraph.push(OpsSubGraphCreate::ops(id_renderer, name.clone()));
    }
    pub fn p3d_create_render(app: &mut Engine, cmds: &mut CommandsExchangeD3, viewer: Entity, id_renderer: Entity, name: String, pass_tag: f64, transparent: bool, recordinput: Option<bool>, crossrender: Option<bool>) {
        let recordinput = if let Some(recordinput) = recordinput { recordinput } else { true };
        let crossrender = if let Some(crossrender) = crossrender { crossrender } else { false };
        cmds.renderer_create.push(OpsRendererCreate::ops(id_renderer, name.clone(), viewer, PassTag::new(pass_tag as u16), transparent, recordinput, crossrender));
    }
    pub fn p3d_renderer_modify(cmds: &mut CommandsExchangeD3, renderer: Entity, val: ERendererCommand) {
        cmds.renderer_modify.push(OpsRendererCommand::ops(renderer, val));
    }
    pub fn p3d_render_target(cmds: &mut CommandsExchangeD3, renderer: Entity, val: ERendererTarget) {
        cmds.renderer_target.push(OpsRendererTarget::new(renderer, val));
    }
    pub fn p3d_crossrender_link_drawlists(cmds: &mut CommandsExchangeD3, linkentity: Entity, drawlistrenderers: Vec<Entity>) {
        cmds.crossdrawlistinfo.push((linkentity, drawlistrenderers));
    }
    pub fn p3d_render_screenwithpostprocess(cmds: &mut CommandsExchangeD3, flag: bool) {
        cmds.screenwithpostprocess = flag;
    }
    pub fn p3d_scene(cmds: &mut CommandsExchangeD3, scene: Entity, cullingmode: f64, collidermode: f64, values: [i32; 9]) {
        cmds.scene_create.push(OpsSceneCreation::ops(scene, cullingmode as u8, collidermode as u8, values));
    }
    pub fn p3d_scene_option(cmds: &mut CommandsExchangeD3, scene: Entity, val: ESceneOps) {
        cmds.scene_options.push(OpsSceneOption::ops(scene, val));
    }
    pub fn p3d_layermask(cmds: &mut CommandsExchangeD3, node: Entity, val: u32) {
        cmds.mesh_layermask.push(OpsLayerMask::ops(node, val));
    }
    pub fn p3d_scene_boundingbox(cmds: &mut CommandsExchangeD3, scene: Entity, display: bool, pass: PassTag) {
        cmds.scene_boundingbox.push(OpsBoundingBoxDisplay::ops(scene, display, pass));
    }
    pub fn p3d_collider(cmds: &mut CommandsExchangeD3, node: Entity,
        minx: f32, miny: f32, minz: f32,
        maxx: f32, maxy: f32, maxz: f32,
        intersection_treshold: f32, alphaindex: i32
    ) {
        cmds.scene_collider.push(OpsCollider::new(node, (minx as f32, miny as f32, minz as f32), (maxx as f32, maxy as f32, maxz as f32), intersection_treshold as f32, alphaindex));
    }
    pub fn p3d_shadow_generator(cmds: &mut CommandsExchangeD3, scene: Entity, light: Entity, id: Entity, pass_tag: u16, graph: Entity) {
        cmds.shadow_create.push(OpsShadowGenerator::ops(id, scene, light, PassTag::new(pass_tag as u16), graph));
        cmds.renderer_create.push(OpsRendererCreate::ops(id, String::from("Shadow") + id.index().to_string().as_str(), id, PassTag::new(pass_tag as u16), false, false, false));
    }
    pub fn p3d_shadow_base_param(cmds: &mut CommandsExchangeD3, shadow: Entity, val: EShadowGeneratorParam) {
        cmds.shadow_param.push(OpsShadowGeneratorParam(shadow, val));
    }
    pub fn p3d_skeleton(cmds: &mut CommandsExchangeD3, id: Entity, bonespervertex: u8, root: Entity, bones: &[Entity], bonecount: f64, cacheframe: f64) {
        let state = match (bonespervertex as u8) {
            1 => ESkinBonesPerVertex::One,
            2 => ESkinBonesPerVertex::Two,
            3 => ESkinBonesPerVertex::Three,
            _ => ESkinBonesPerVertex::Four
        };

        cmds.skin_create.push(OpsSkinCreation::ops(id, state, root, &bones, cacheframe as u16, None));
    }
    pub fn p3d_bone(cmds: &mut CommandsExchangeD3, id: Entity, scene: Entity) {
        cmds.skin_bonecreate.push(OpsBoneCreation::ops(id, scene));
    }
    pub fn p3d_bone_link(cmds: &mut CommandsExchangeD3, bone: Entity, link: Entity) {
        cmds.skin_use.push(OpsSkinUse::bone_link(bone, link));
    }
    pub fn p3d_bone_pose(cmds: &mut CommandsExchangeD3, bone: Entity, data: &[f32]) {
        let matrix = Matrix::new(
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15]
        );
        cmds.skin_bonepose.push(OpsBonePose::ops(bone, matrix));
    }
    pub fn p3d_skin_use(cmds: &mut CommandsExchangeD3, id_mesh: Entity, skin: Entity) {
        cmds.skin_use.push(OpsSkinUse::ops(id_mesh, skin));
    }
    pub fn p3d_sprite(cmds: &mut CommandsExchangeD3, source: Entity, id: Entity, atlas: &pi_atom::Atom) {
        cmds.instance_create.push(OpsInstanceMeshCreation::ops(source, id));
        cmds.sprite_create.push(OpsSpriteCreate::ops(source, id, atlas.to_string().asset_u64()));
    }
    pub fn p3d_sprite_frame(cmds: &mut CommandsExchangeD3, sprite: Entity, tilloffkey: &pi_atom::Atom, idxframe: f64) {
        cmds.sprite_modify.push(OpsSpriteModify::ops(sprite, SpriteModify::Idx(idxframe as IdxTextureFrame), tilloffkey.clone()));
    }
    pub fn p3d_sprite_frame_data(cmds: &mut CommandsExchangeD3, sprite: Entity, tilloffkey: &pi_atom::Atom, data: &[u16]) {
        let data = [data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9],
            data[10], data[11], data[12], data[13],
        ];
        cmds.sprite_modify.push(OpsSpriteModify::ops(sprite, SpriteModify::Data(data), tilloffkey.clone()));
    }
    pub fn p3d_create_data_texture(param: &mut CommandsExchangeD3, key: &pi_atom::Atom, width: u32, height: u32, format: f64, aspect: Option<f64>) {
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
    pub fn p3d_update_data_texture(param: &mut CommandsExchangeD3, key: &pi_atom::Atom, data:&[u8], xoffset: u32, yoffset: u32, width: u32, height: u32, aspect: Option<f64>) {

        let key = key.clone();
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
    pub fn p3d_remove_data_texture(param: &mut CommandsExchangeD3, key: &pi_atom::Atom) {
        param.datatexcmd.createdata.remove(key);
        param.datatexcmd.updatedata.remove(key);
        param.datatexcmd.record.remove(key);
    }
    pub fn p3d_texture_combine_param(app: &mut Engine, format: f64, maxlayer: f64, maxsize: f64, maxcount: f64) {
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
    
    pub fn p3d_trail(cmds: &mut CommandsExchangeD3, scene: Entity, entity: Entity, linked: Entity) {
        cmds.trail_create.push(OpsTrail::ops(scene, linked, entity));
    }
    pub fn p3d_trail_age(cmds: &mut CommandsExchangeD3, entity: Entity, age_ms: u32) {
        cmds.trail_age.push(OpsTrailAgeControl::ops(entity, age_ms));
    }
    pub fn p3d_transform_node(cmds: &mut CommandsExchangeD3, scene: Entity, id: Entity) {
        cmds.transform_tree.push(OpsTransformNodeParent::ops(id, scene));
        cmds.transform_create.push(OpsTransformNode::ops(scene, id));
    }
    pub fn p3d_transform_node_parent(cmds: &mut CommandsExchangeD3, node: Entity, parent: Entity) {
        cmds.transform_tree.push(OpsTransformNodeParent::ops(node, parent));
    }
    pub fn p3d_node_enable(cmds: &mut CommandsExchangeD3, node: Entity, val: bool) {
        cmds.transform_enable.push(OpsNodeEnable::ops(node, val));
    }
    pub fn p3d_local_srt(cmds: &mut CommandsExchangeD3, node: Entity, val: ETransformSRT) {
        cmds.transform_localsrt.push(OpsTransformNodeLocal::ops(node, val));
    }
    pub fn p3d_local_quaternion(cmds: &mut CommandsExchangeD3, node: Entity, x: f32, y: f32, z: f32, w: f32) {
        cmds.transform_localrotq.push(OpsTransformNodeLocalRotationQuaternion::ops(node, x as f32, y as f32, z as f32, w as f32));
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_commands_exchange(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3) {
	pi_export_base::export::await_last_frame(app);

    // log::error!(">>>>> p3d_commands_exchange");
    // log::error!("World Memory {:?}", app.world.mem_size());
    let state = param.state.get_mut(&mut app.world);
    
    // log::error!("p3d_commands_exchange {:?}", state.stateengine.active);
    if state.stateengine.active {
        let mut sets = param.acts.get_mut(&mut app.world);
        // log::error!(">>>>> p3d_commands_exchange 01");
    
        cmds.exchange(&mut sets);
        // log::error!(">>>>> p3d_commands_exchange 02");
    }

    let spriteframes = app.world.get_resource_mut::<ResSpriteFrames>().unwrap();
    spriteframes.0.append(&mut cmds.sprite_frames.1);

    let crossrenderinfos = app.world.get_resource_mut::<pi_bevy_render_plugin::render_cross::CrossRenderDrawListEntities>().unwrap();
    cmds.crossdrawlistinfo.drain(..).for_each(|(link, list)| {
        if list.len() > 0 {
            crossrenderinfos.0.insert(link, list);
        } else {
            crossrenderinfos.0.remove(&link);
        }
    });

    let imgtex_asset = app.world.get_resource::<ShareAssetMgr<pi_scene_shell::prelude::ImageTextureFrame>>().unwrap().clone();
    let device = app.world.get_resource::<PiRenderDevice>().unwrap();
    let queue = app.world.get_resource::<PiRenderQueue>().unwrap();
    update_data_texture(&mut cmds.datatexcmd, device, queue, &imgtex_asset);
    let requests = app.world.get_resource_mut::<TextureCombineCmds>().unwrap();
    cmds.combinecmds.drain().for_each(|(requestid, (key, atlas))| {
        let keytex = KeyImageTextureFrame { url: key, file: false, compressed: false, cancombine: false, 
        };
        requests.request(requestid, keytex, atlas, &imgtex_asset);
    });

    let texloader = app.world.get_resource_mut::<ResImageTextureLoader>().unwrap();
    while let Some(key) = cmds.loadtextures.pop() {
        texloader.create_load(key);
    }

    // let queue = app.world.get_resource::<pi_scene_shell::prelude::PiRenderQueue>().unwrap().deref().clone();
    // let vb_mgr = app.world.get_resource::<pi_scene_shell::prelude::ShareAssetMgr<pi_scene_shell::prelude::EVertexBufferRange>>().unwrap().deref().clone();
    // let vb_wait = app.world.get_resource_mut::<pi_scene_shell::prelude::VertexBufferDataMap3D>().unwrap();
    let actions = app.world.get_resource_mut::<ActionListCustomBuffer>().unwrap();

    while let Some((key, data)) = cmds.verticesbuffers.pop() {
        actions.push((key, data, false, false));
		// let key_u64 = key.asset_u64();
		// if let Some(buffer) = vb_mgr.get(&key_u64) {
		// 	queue.write_buffer(buffer.buffer(), 0, &data);
		// } else {
		// 	pi_scene_context::prelude::ActionVertexBuffer::create(vb_wait, key, data);
		// }
    }
    while let Some((key, data)) = cmds.indicesbuffers.pop() {
        actions.push((key, data, true, false));
		// let key_u64 = key.asset_u64();
		// if let Some(buffer) = vb_mgr.get(&key_u64) {
		// 	queue.write_buffer(buffer.buffer(), 0, &data);
		// } else {
		// 	pi_scene_context::prelude::ActionVertexBuffer::create_indices(vb_wait, key, data);
		// }
    }
    while let Some((key, data)) = cmds.indicesbuffersu32.pop() {
        actions.push((key, data, true, false));
		// let key_u64 = key.asset_u64();
		// if let Some(buffer) = vb_mgr.get(&key_u64) {
		// 	queue.write_buffer(buffer.buffer(), 0, &data);
		// } else {
		// 	pi_scene_context::prelude::ActionVertexBuffer::create_indices(vb_wait, key, data);
		// }
    }

    let screenwithpostprocess = app.world.get_resource_mut::<pi_bevy_render_plugin::ScreenWithPostprocess>().unwrap();
    screenwithpostprocess.0 = cmds.screenwithpostprocess;
}
