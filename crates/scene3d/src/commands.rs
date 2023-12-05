use pi_engine_shell::prelude::*;
use pi_export_base::export::Engine;
use pi_particle_system::prelude::*;
use pi_scene_context::prelude::*;
use pi_trail_renderer::*;

use crate::engine::ActionSetScene3D;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Default)]
pub struct CommandsExchangeD3 {
    pub(crate) scene_create: ActionListSceneCreate,
    pub(crate) scene_time: ActionListSceneTime,
    pub(crate) scene_fogcolor: ActionListSceneFogColor,
    pub(crate) scene_fogparam: ActionListSceneFogParam,
    pub(crate) scene_ambientcolor: ActionListSceneAmbientColor,
    pub(crate) scene_ambientintensity: ActionListSceneAmbientIntensity,
    pub(crate) scene_animeenable: ActionListSceneAnimationEnable,
    pub(crate) scene_brdf: ActionListSceneBRDF,
    pub(crate) scene_env: ActionListSceneEnvTexture,
    pub(crate) scene_shadowmap: ActionListSceneShadowMap,
    pub(crate) scene_dispose: ActionListSceneDispose,

    pub(crate) obj_dispose: ActionListDispose,
    
    pub(crate) transform_create: ActionListTransformNodeCreate,
    pub(crate) transform_localpos: ActionListTransformNodeLocalPosition,
    pub(crate) transform_localscl: ActionListTransformNodeLocalScaling,
    pub(crate) transform_localrot: ActionListTransformNodeLocalEuler,
    pub(crate) transform_localrotq: ActionListTransformNodeLocalRotationQuaternion,
    pub(crate) transform_tree: ActionListTransformNodeParent,
    pub(crate) transform_enable: ActionListNodeEnable,

    pub(crate) camera_create: ActionListCameraCreate,
    pub(crate) camera_mode: ActionListCameraMode,
    pub(crate) camera_target: ActionListCameraTarget,
    pub(crate) camera_active: ActionListCameraActive,
    pub(crate) camera_fixmode: ActionListCameraFixedMode,
    pub(crate) camera_fov: ActionListCameraFov,
    pub(crate) camera_size: ActionListCameraOrthSize,
    pub(crate) camera_nearfar: ActionListCameraNearFar,
    pub(crate) camera_aspect: ActionListCameraAspect,
    pub(crate) camera_forceinclude: ActionListViewerForceInclude,
    
    pub(crate) mesh_create: ActionListMeshCreate,
    pub(crate) mesh_shadow: ActionListMeshShadow,
    pub(crate) mesh_blend: ActionListBlend,
    pub(crate) mesh_cullmode: ActionListCullMode,
    pub(crate) mesh_polygonmode: ActionListPolyginMode,
    pub(crate) mesh_frontface: ActionListFrontFace,
    pub(crate) mesh_topology: ActionListTopology,
    pub(crate) mesh_unclip_depth: ActionListUnClipDepth,
    pub(crate) mesh_depth_write: ActionListDepthWrite,
    pub(crate) mesh_depth_compare: ActionListDepthCompare,
    pub(crate) mesh_depth_bias: ActionListDepthBias,
    pub(crate) mesh_stencil_front: ActionListStencilFront,
    pub(crate) mesh_stencil_back: ActionListStencilBack,
    pub(crate) mesh_stencil_read: ActionListStencilRead,
    pub(crate) mesh_stencil_write: ActionListStencilWrite,
    pub(crate) mesh_render_queue: ActionListRenderQueue,
    pub(crate) mesh_render_alignment: ActionListMeshRenderAlignment,
    pub(crate) mesh_indexrange: ActionListMeshRenderIndiceRange,
    pub(crate) mesh_vertexrange: ActionListMeshRenderVertexRange,
    pub(crate) mesh_bounding: ActionListMeshBounding,
    pub(crate) mesh_boundingculling: ActionListMeshBoundingCullingMode,

    pub(crate) mesh_layermask: ActionListLayerMask,
    
    pub(crate) instance_create: ActionListInstanceMeshCreate,
    pub(crate) instance_color: ActionListInstanceColor,
    pub(crate) instance_alpha: ActionListInstanceAlpha,
    pub(crate) instance_tilloff: ActionListInstanceTillOff,
    pub(crate) instance_ins_world_matrixs: ActionListInstanceWorldMatrixs,
    pub(crate) instance_ins_colors: ActionListInstanceColors,
    pub(crate) instance_ins_tilloffs: ActionListInstanceTilloffs,
    pub(crate) instance_floats: ActionListInstanceFloat,
    
    pub(crate) abstructmesh_scaling_mode: ActionListAbstructMeshScalingMode,
    pub(crate) abstructmesh_velocity: ActionListAbstructMeshVelocity,
    pub(crate) abstructmesh_boneoffset: ActionListBoneOffset,
    pub(crate) abstructmesh_force_point_light: ActionListMeshForcePointLighting,
    pub(crate) abstructmesh_force_spot_light: ActionListMeshForceSpotLighting,
    pub(crate) abstructmesh_force_hemi_light: ActionListMeshForceHemiLighting,

    pub(crate) geometry_create: ActionListGeometryCreate,
    
    pub(crate) material_usemat: ActionListMaterialUse,
    pub(crate) material_create: ActionListMaterialCreate,
    pub(crate) material_float: ActionListUniformFloat,
    // pub(crate) material_int: ActionListUniformInt,
    pub(crate) material_uint: ActionListUniformUint,
    pub(crate) material_vec2: ActionListUniformVec2,
    pub(crate) material_vec4: ActionListUniformVec4,
    // pub(crate) material_mat2: ActionListUniformMat2,
    pub(crate) material_mat4: ActionListUniformMat4,
    pub(crate) material_texture: ActionListUniformTexture,
    pub(crate) material_texturefromtarget: ActionListUniformTextureFromRenderTarget,
    
    pub(crate) light_create: ActionListLightCreate,
    pub(crate) light_param: ActionListLightParam,
    pub(crate) light_color: ActionListLightColor,
    pub(crate) light_strength: ActionListLightStrength,
    pub(crate) light_radius: ActionListLightRadius,
    pub(crate) light_spotangle: ActionListSpotLightAngle,
    
    pub(crate) shadow_param: ActionListShadowGeneratorParam,
    pub(crate) shadow_create: ActionListShadowGenerator,
    
    pub(crate) renderer_create: ActionListRendererCreate,
    pub(crate) renderer_connect: ActionListRendererConnect,
    pub(crate) renderer_modify: ActionListRendererModify,
    pub(crate) renderer_target: ActionListRendererTarget,
    
    pub(crate) anime_attach: ActionListAnimeGroupAttach,
    pub(crate) anime_reset_while_start: ActionListAnimeGroupStartReset,

    pub(crate) trail_create: ActionListTrail,
    pub(crate) trail_age: ActionListTrailAge,

    pub(crate) parsys_calculator: ActionListCPUParticleCalculator,
    pub(crate) parsys_create: ActionListCPUParticleSystem,
    pub(crate) parsys_state: ActionListCPUParticleSystemState,
    pub(crate) parsys_trailmaterial: ActionListCPUParticleSystemTrailMaterial,
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
    pub(crate) fn exchange(&mut self, cmds: &mut pi_3d::ActionSets) {
        cmds.scene.create.exchange( self.scene_create.exchange(vec![]) );
        cmds.scene.time.exchange( self.scene_time.exchange(vec![]) );
        cmds.scene.fogcolor.exchange( self.scene_fogcolor.exchange(vec![]) );
        cmds.scene.fogparam.exchange( self.scene_fogparam.exchange(vec![]) );
        cmds.scene.ambientcolor.exchange( self.scene_ambientcolor.exchange(vec![]) );
        cmds.scene.ambientintensity.exchange( self.scene_ambientintensity.exchange(vec![]) );
        cmds.scene.animeenable.exchange( self.scene_animeenable.exchange(vec![]) );
        cmds.scene.brdf.exchange( self.scene_brdf.exchange(vec![]) );
        cmds.scene.env.exchange( self.scene_env.exchange(vec![]) );
        cmds.scene.shadowmap.exchange( self.scene_shadowmap.exchange(vec![]) );
        cmds.scene_dispose.exchange( self.scene_dispose.exchange(vec![]) );
        cmds.obj_dispose.exchange( self.obj_dispose.exchange(vec![]) );
        cmds.transform.create.exchange( self.transform_create.exchange(vec![]) );
        cmds.transform.localpos.exchange( self.transform_localpos.exchange(vec![]) );
        cmds.transform.localscl.exchange( self.transform_localscl.exchange(vec![]) );
        cmds.transform.localrot.exchange( self.transform_localrot.exchange(vec![]) );
        cmds.transform.localrotq.exchange( self.transform_localrotq.exchange(vec![]) );
        cmds.transform.tree.exchange( self.transform_tree.exchange(vec![]) );
        cmds.transform.enable.exchange( self.transform_enable.exchange(vec![]) );
        cmds.camera.create.exchange( self.camera_create.exchange(vec![]) );
        cmds.camera.mode.exchange( self.camera_mode.exchange(vec![]) );
        cmds.camera.target.exchange( self.camera_target.exchange(vec![]) );
        cmds.camera.active.exchange( self.camera_active.exchange(vec![]) );
        cmds.camera.fixmode.exchange( self.camera_fixmode.exchange(vec![]) );
        cmds.camera.fov.exchange( self.camera_fov.exchange(vec![]) );
        cmds.camera.size.exchange( self.camera_size.exchange(vec![]) );
        cmds.camera.nearfar.exchange( self.camera_nearfar.exchange(vec![]) );
        cmds.camera.aspect.exchange( self.camera_aspect.exchange(vec![]) );
        cmds.camera.forceinclude.exchange( self.camera_forceinclude.exchange(vec![]) );
        cmds.mesh.create.exchange( self.mesh_create.exchange(vec![]) );
        cmds.mesh.shadow.exchange( self.mesh_shadow.exchange(vec![]) );
        cmds.mesh.blend.exchange( self.mesh_blend.exchange(vec![]) );
        cmds.mesh.cullmode.exchange( self.mesh_cullmode.exchange(vec![]) );
        cmds.mesh.polygonmode.exchange( self.mesh_polygonmode.exchange(vec![]) );
        cmds.mesh.frontface.exchange( self.mesh_frontface.exchange(vec![]) );
        cmds.mesh.topology.exchange( self.mesh_topology.exchange(vec![]) );
        cmds.mesh.unclip_depth.exchange( self.mesh_unclip_depth.exchange(vec![]) );
        cmds.mesh.depth_write.exchange( self.mesh_depth_write.exchange(vec![]) );
        cmds.mesh.depth_compare.exchange( self.mesh_depth_compare.exchange(vec![]) );
        cmds.mesh.depth_bias.exchange( self.mesh_depth_bias.exchange(vec![]) );
        cmds.mesh.stencil_front.exchange( self.mesh_stencil_front.exchange(vec![]) );
        cmds.mesh.stencil_back.exchange( self.mesh_stencil_back.exchange(vec![]) );
        cmds.mesh.stencil_read.exchange( self.mesh_stencil_read.exchange(vec![]) );
        cmds.mesh.stencil_write.exchange( self.mesh_stencil_write.exchange(vec![]) );
        cmds.mesh.render_queue.exchange( self.mesh_render_queue.exchange(vec![]) );
        cmds.mesh.render_alignment.exchange( self.mesh_render_alignment.exchange(vec![]) );
        cmds.mesh.indexrange.exchange( self.mesh_indexrange.exchange(vec![]) );
        cmds.mesh.vertexrange.exchange( self.mesh_vertexrange.exchange(vec![]) );
        cmds.mesh.bounding.exchange( self.mesh_bounding.exchange(vec![]) );
        cmds.mesh.boundingculling.exchange( self.mesh_boundingculling.exchange(vec![]) );
        cmds.mesh.layermask.exchange( self.mesh_layermask.exchange(vec![]) );
        cmds.instance.create.exchange( self.instance_create.exchange(vec![]) );
        cmds.instance.color.exchange( self.instance_color.exchange(vec![]) );
        cmds.instance.alpha.exchange( self.instance_alpha.exchange(vec![]) );
        cmds.instance.tilloff.exchange( self.instance_tilloff.exchange(vec![]) );
        cmds.instance.ins_world_matrixs.exchange( self.instance_ins_world_matrixs.exchange(vec![]) );
        cmds.instance.ins_colors.exchange( self.instance_ins_colors.exchange(vec![]) );
        cmds.instance.ins_tilloffs.exchange( self.instance_ins_tilloffs.exchange(vec![]) );
        cmds.instance.floats.exchange( self.instance_floats.exchange(vec![]) );
        cmds.abstructmesh.scaling_mode.exchange( self.abstructmesh_scaling_mode.exchange(vec![]) );
        cmds.abstructmesh.velocity.exchange( self.abstructmesh_velocity.exchange(vec![]) );
        cmds.abstructmesh.boneoffset.exchange( self.abstructmesh_boneoffset.exchange(vec![]) );
        cmds.abstructmesh.force_point_light.exchange( self.abstructmesh_force_point_light.exchange(vec![]) );
        cmds.abstructmesh.force_spot_light.exchange( self.abstructmesh_force_spot_light.exchange(vec![]) );
        cmds.abstructmesh.force_hemi_light.exchange( self.abstructmesh_force_hemi_light.exchange(vec![]) );
        cmds.geometry.create.exchange( self.geometry_create.exchange(vec![]) );
        cmds.material.usemat.exchange( self.material_usemat.exchange(vec![]) );
        cmds.material.create.exchange( self.material_create.exchange(vec![]) );
        cmds.material.float.exchange( self.material_float.exchange(vec![]) );
        // cmds.material.int.exchange( self.material_int.exchange(vec![]) );
        cmds.material.uint.exchange( self.material_uint.exchange(vec![]) );
        cmds.material.vec2.exchange( self.material_vec2.exchange(vec![]) );
        cmds.material.vec4.exchange( self.material_vec4.exchange(vec![]) );
        // cmds.material.mat2.exchange( self.material_mat2.exchange(vec![]) );
        cmds.material.mat4.exchange( self.material_mat4.exchange(vec![]) );
        cmds.material.texture.exchange( self.material_texture.exchange(vec![]) );
        cmds.material.texturefromtarget.exchange( self.material_texturefromtarget.exchange(vec![]) );
        cmds.light.create.exchange( self.light_create.exchange(vec![]) );
        cmds.light.param.exchange( self.light_param.exchange(vec![]) );
        cmds.light.color.exchange( self.light_color.exchange(vec![]) );
        cmds.light.strength.exchange( self.light_strength.exchange(vec![]) );
        cmds.light.radius.exchange( self.light_radius.exchange(vec![]) );
        cmds.light.spotangle.exchange( self.light_spotangle.exchange(vec![]) );
        cmds.shadow.param.exchange( self.shadow_param.exchange(vec![]) );
        cmds.shadow.create.exchange( self.shadow_create.exchange(vec![]) );
        cmds.renderer.create.exchange( self.renderer_create.exchange(vec![]) );
        cmds.renderer.connect.exchange( self.renderer_connect.exchange(vec![]) );
        cmds.renderer.modify.exchange( self.renderer_modify.exchange(vec![]) );
        cmds.renderer.target.exchange( self.renderer_target.exchange(vec![]) );
        cmds.anime.attach.exchange( self.anime_attach.exchange(vec![]) );
        cmds.anime.reset_while_start.exchange( self.anime_reset_while_start.exchange(vec![]) );
        
        cmds.trail.create.exchange( self.trail_create.exchange(vec![]) );
        cmds.trail.age.exchange( self.trail_age.exchange(vec![]) );
        
        cmds.parsys.create.exchange( self.parsys_create.exchange(vec![]) );
        cmds.parsys.calculator.exchange( self.parsys_calculator.exchange(vec![]) );
        cmds.parsys.state.exchange( self.parsys_state.exchange(vec![]) );
        cmds.parsys.trailmaterial.exchange( self.parsys_trailmaterial.exchange(vec![]) );
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_commands_exchange(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3) {
    let mut sets = param.acts.get_mut(&mut app.world);
    cmds.exchange(&mut sets);
}