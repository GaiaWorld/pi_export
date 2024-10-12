use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_particle_system::prelude::*;
use pi_scene_context::prelude::*;
use pi_trail_renderer::*;

pub use crate::engine::ActionSetScene3D;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Default)]
pub struct CommandsExchangeD3 {
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
    
    pub(crate) renderer_create: ActionListRendererCreate,
    pub(crate) renderer_connect: ActionListRendererConnect,
    pub(crate) renderer_modify: ActionListRendererModify,
    pub(crate) renderer_target: ActionListRendererTarget,
    
    pub(crate) anime_create: ActionListAnimeGroupCreate,
    pub(crate) anime_action: ActionListAnimationGroupAction,
    pub(crate) anime_dispose: ActionListAnimeGroupDispose,
    pub(crate) anime_reset_while_start: ActionListAnimeGroupStartReset,
    pub(crate) anime_property_targetanime: ActionListPropertyTargetAnimation,

    pub(crate) trail_create: ActionListTrail,
    pub(crate) trail_age: ActionListTrailAge,

    pub(crate) parsys_calculator: ActionListCPUParticleCalculator,
    pub(crate) parsys_create: ActionListCPUParticleSystem,
    pub(crate) parsys_state: ActionListCPUParticleSystemState,
    pub(crate) parsys_trailmaterial: ActionListCPUParticleSystemTrailMaterial,
    
    pub(crate) sprite_create: ActionListSpriteCreate,
    pub(crate) sprite_modify: ActionListSpriteModify,
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
        self.trail_create               .capacity() +
        self.trail_age                  .capacity() +
        self.parsys_create              .capacity() +
        self.parsys_calculator          .capacity() +
        self.parsys_state               .capacity() +
        self.parsys_trailmaterial       .capacity() +
        self.sprite_create              .capacity() +
        self.sprite_modify              .capacity() +
        cmds.scene.create               .capacity() +
        cmds.scene.options              .capacity() +
        cmds.scene_dispose              .capacity() +
        cmds.scene.boundingboxdisplay   .capacity() +
        cmds.scene.collider             .capacity() +
        cmds.obj_dispose                .capacity() +
        cmds.transform.create           .capacity() +
        cmds.transform.localsrt         .capacity() +
        cmds.transform.localrotq        .capacity() +
        cmds.transform.tree             .capacity() +
        cmds.transform.enable           .capacity() +
        cmds.camera.create              .capacity() +
        cmds.camera.param               .capacity() +
        cmds.camera.target              .capacity() +
        cmds.camera.forceinclude        .capacity() +
        cmds.mesh.create                .capacity() +
        cmds.mesh.state                 .capacity() +
        cmds.mesh.value_state           .capacity() +
        cmds.mesh.render_state          .capacity() +
        cmds.mesh.forcelighting         .capacity() +
        cmds.mesh.bounding              .capacity() +
        cmds.mesh.layermask             .capacity() +
        cmds.skin.skin_create           .capacity() +
        cmds.skin.bone_create           .capacity() +
        cmds.skin.skin_use              .capacity() +
        cmds.skin.bone_pose             .capacity() +
        cmds.instance.create            .capacity() +
        cmds.instance.attr              .capacity() +
        cmds.anime_instance             .capacity() +
        cmds.geometry.create            .capacity() +
        cmds.material.usemat            .capacity() +
        cmds.material.create            .capacity() +
        cmds.material.val               .capacity() +
        cmds.material.valb              .capacity() +
        cmds.light.create               .capacity() +
        cmds.light.param                .capacity() +
        cmds.shadow.param               .capacity() +
        cmds.shadow.create              .capacity() +
        cmds.renderer.create            .capacity() +
        cmds.renderer.connect           .capacity() +
        cmds.renderer.modify            .capacity() +
        cmds.renderer.target            .capacity() +
        cmds.anime.create               .capacity() +
        cmds.anime.action               .capacity() +
        cmds.anime.dispose              .capacity() +
        cmds.anime.reset_while_start    .capacity() +
        cmds.property_targetanimation   .capacity() +
        cmds.trail.create               .capacity() +
        cmds.trail.age                  .capacity() +
        cmds.parsys.create              .capacity() +
        cmds.parsys.calculator          .capacity() +
        cmds.parsys.state               .capacity() +
        cmds.parsys.trailmaterial       .capacity() +
        cmds.spritecreate               .capacity() +
        cmds.spritemodify               .capacity() + 0
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
        cmds.anime_instance.append(&mut self.instance_targetanime );
        cmds.geometry.create.append(&mut self.geometry_create );
        cmds.material.usemat.append(&mut self.material_usemat );
        cmds.material.create.append(&mut self.material_create );
        cmds.material.val.append(&mut self.material_val );
        cmds.material.valb.append(&mut self.material_valb );
        cmds.light.create.append(&mut self.light_create );
        cmds.light.param.append(&mut self.light_param );
        cmds.shadow.param.append(&mut self.shadow_param );
        cmds.shadow.create.append(&mut self.shadow_create );
        cmds.renderer.create.append(&mut self.renderer_create );
        cmds.renderer.connect.append(&mut self.renderer_connect );
        cmds.renderer.modify.append(&mut self.renderer_modify );
        cmds.renderer.target.append(&mut self.renderer_target );
        cmds.anime.create.append(&mut self.anime_create );
        cmds.anime.action.append(&mut self.anime_action );
        cmds.anime.dispose.append(&mut self.anime_dispose );
        cmds.anime.reset_while_start.append(&mut self.anime_reset_while_start );
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
}