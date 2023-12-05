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
        cmds.scene.create.push_some( self.scene_create.exchange(vec![]).drain(..) );
        cmds.scene.time.push_some( self.scene_time.exchange(vec![]).drain(..) );
        cmds.scene.fogcolor.push_some( self.scene_fogcolor.exchange(vec![]).drain(..) );
        cmds.scene.fogparam.push_some( self.scene_fogparam.exchange(vec![]).drain(..) );
        cmds.scene.ambientcolor.push_some( self.scene_ambientcolor.exchange(vec![]).drain(..) );
        cmds.scene.ambientintensity.push_some( self.scene_ambientintensity.exchange(vec![]).drain(..) );
        cmds.scene.animeenable.push_some( self.scene_animeenable.exchange(vec![]).drain(..) );
        cmds.scene.brdf.push_some( self.scene_brdf.exchange(vec![]).drain(..) );
        cmds.scene.env.push_some( self.scene_env.exchange(vec![]).drain(..) );
        cmds.scene.shadowmap.push_some( self.scene_shadowmap.exchange(vec![]).drain(..) );
        cmds.scene_dispose.push_some( self.scene_dispose.exchange(vec![]).drain(..) );
        cmds.obj_dispose.push_some( self.obj_dispose.exchange(vec![]).drain(..) );
        cmds.transform.create.push_some( self.transform_create.exchange(vec![]).drain(..) );
        cmds.transform.localpos.push_some( self.transform_localpos.exchange(vec![]).drain(..) );
        cmds.transform.localscl.push_some( self.transform_localscl.exchange(vec![]).drain(..) );
        cmds.transform.localrot.push_some( self.transform_localrot.exchange(vec![]).drain(..) );
        cmds.transform.localrotq.push_some( self.transform_localrotq.exchange(vec![]).drain(..) );
        cmds.transform.tree.push_some( self.transform_tree.exchange(vec![]).drain(..) );
        cmds.transform.enable.push_some( self.transform_enable.exchange(vec![]).drain(..) );
        cmds.camera.create.push_some( self.camera_create.exchange(vec![]).drain(..) );
        cmds.camera.mode.push_some( self.camera_mode.exchange(vec![]).drain(..) );
        cmds.camera.target.push_some( self.camera_target.exchange(vec![]).drain(..) );
        cmds.camera.active.push_some( self.camera_active.exchange(vec![]).drain(..) );
        cmds.camera.fixmode.push_some( self.camera_fixmode.exchange(vec![]).drain(..) );
        cmds.camera.fov.push_some( self.camera_fov.exchange(vec![]).drain(..) );
        cmds.camera.size.push_some( self.camera_size.exchange(vec![]).drain(..) );
        cmds.camera.nearfar.push_some( self.camera_nearfar.exchange(vec![]).drain(..) );
        cmds.camera.aspect.push_some( self.camera_aspect.exchange(vec![]).drain(..) );
        cmds.camera.forceinclude.push_some( self.camera_forceinclude.exchange(vec![]).drain(..) );
        cmds.mesh.create.push_some( self.mesh_create.exchange(vec![]).drain(..) );
        cmds.mesh.shadow.push_some( self.mesh_shadow.exchange(vec![]).drain(..) );
        cmds.mesh.blend.push_some( self.mesh_blend.exchange(vec![]).drain(..) );
        cmds.mesh.cullmode.push_some( self.mesh_cullmode.exchange(vec![]).drain(..) );
        cmds.mesh.polygonmode.push_some( self.mesh_polygonmode.exchange(vec![]).drain(..) );
        cmds.mesh.frontface.push_some( self.mesh_frontface.exchange(vec![]).drain(..) );
        cmds.mesh.topology.push_some( self.mesh_topology.exchange(vec![]).drain(..) );
        cmds.mesh.unclip_depth.push_some( self.mesh_unclip_depth.exchange(vec![]).drain(..) );
        cmds.mesh.depth_write.push_some( self.mesh_depth_write.exchange(vec![]).drain(..) );
        cmds.mesh.depth_compare.push_some( self.mesh_depth_compare.exchange(vec![]).drain(..) );
        cmds.mesh.depth_bias.push_some( self.mesh_depth_bias.exchange(vec![]).drain(..) );
        cmds.mesh.stencil_front.push_some( self.mesh_stencil_front.exchange(vec![]).drain(..) );
        cmds.mesh.stencil_back.push_some( self.mesh_stencil_back.exchange(vec![]).drain(..) );
        cmds.mesh.stencil_read.push_some( self.mesh_stencil_read.exchange(vec![]).drain(..) );
        cmds.mesh.stencil_write.push_some( self.mesh_stencil_write.exchange(vec![]).drain(..) );
        cmds.mesh.render_queue.push_some( self.mesh_render_queue.exchange(vec![]).drain(..) );
        cmds.mesh.render_alignment.push_some( self.mesh_render_alignment.exchange(vec![]).drain(..) );
        cmds.mesh.indexrange.push_some( self.mesh_indexrange.exchange(vec![]).drain(..) );
        cmds.mesh.vertexrange.push_some( self.mesh_vertexrange.exchange(vec![]).drain(..) );
        cmds.mesh.bounding.push_some( self.mesh_bounding.exchange(vec![]).drain(..) );
        cmds.mesh.boundingculling.push_some( self.mesh_boundingculling.exchange(vec![]).drain(..) );
        cmds.mesh.layermask.push_some( self.mesh_layermask.exchange(vec![]).drain(..) );
        cmds.instance.create.push_some( self.instance_create.exchange(vec![]).drain(..) );
        cmds.instance.color.push_some( self.instance_color.exchange(vec![]).drain(..) );
        cmds.instance.alpha.push_some( self.instance_alpha.exchange(vec![]).drain(..) );
        cmds.instance.tilloff.push_some( self.instance_tilloff.exchange(vec![]).drain(..) );
        cmds.instance.ins_world_matrixs.push_some( self.instance_ins_world_matrixs.exchange(vec![]).drain(..) );
        cmds.instance.ins_colors.push_some( self.instance_ins_colors.exchange(vec![]).drain(..) );
        cmds.instance.ins_tilloffs.push_some( self.instance_ins_tilloffs.exchange(vec![]).drain(..) );
        cmds.instance.floats.push_some( self.instance_floats.exchange(vec![]).drain(..) );
        cmds.abstructmesh.scaling_mode.push_some( self.abstructmesh_scaling_mode.exchange(vec![]).drain(..) );
        cmds.abstructmesh.velocity.push_some( self.abstructmesh_velocity.exchange(vec![]).drain(..) );
        cmds.abstructmesh.boneoffset.push_some( self.abstructmesh_boneoffset.exchange(vec![]).drain(..) );
        cmds.abstructmesh.force_point_light.push_some( self.abstructmesh_force_point_light.exchange(vec![]).drain(..) );
        cmds.abstructmesh.force_spot_light.push_some( self.abstructmesh_force_spot_light.exchange(vec![]).drain(..) );
        cmds.abstructmesh.force_hemi_light.push_some( self.abstructmesh_force_hemi_light.exchange(vec![]).drain(..) );
        cmds.geometry.create.push_some( self.geometry_create.exchange(vec![]).drain(..) );
        cmds.material.usemat.push_some( self.material_usemat.exchange(vec![]).drain(..) );
        cmds.material.create.push_some( self.material_create.exchange(vec![]).drain(..) );
        cmds.material.float.push_some( self.material_float.exchange(vec![]).drain(..) );
        // cmds.material.int.push_some( self.material_int.exchange(vec![]).drain(..) );
        cmds.material.uint.push_some( self.material_uint.exchange(vec![]).drain(..) );
        cmds.material.vec2.push_some( self.material_vec2.exchange(vec![]).drain(..) );
        cmds.material.vec4.push_some( self.material_vec4.exchange(vec![]).drain(..) );
        // cmds.material.mat2.push_some( self.material_mat2.exchange(vec![]).drain(..) );
        cmds.material.mat4.push_some( self.material_mat4.exchange(vec![]).drain(..) );
        cmds.material.texture.push_some( self.material_texture.exchange(vec![]).drain(..) );
        cmds.material.texturefromtarget.push_some( self.material_texturefromtarget.exchange(vec![]).drain(..) );
        cmds.light.create.push_some( self.light_create.exchange(vec![]).drain(..) );
        cmds.light.param.push_some( self.light_param.exchange(vec![]).drain(..) );
        cmds.light.color.push_some( self.light_color.exchange(vec![]).drain(..) );
        cmds.light.strength.push_some( self.light_strength.exchange(vec![]).drain(..) );
        cmds.light.radius.push_some( self.light_radius.exchange(vec![]).drain(..) );
        cmds.light.spotangle.push_some( self.light_spotangle.exchange(vec![]).drain(..) );
        cmds.shadow.param.push_some( self.shadow_param.exchange(vec![]).drain(..) );
        cmds.shadow.create.push_some( self.shadow_create.exchange(vec![]).drain(..) );
        cmds.renderer.create.push_some( self.renderer_create.exchange(vec![]).drain(..) );
        cmds.renderer.connect.push_some( self.renderer_connect.exchange(vec![]).drain(..) );
        cmds.renderer.modify.push_some( self.renderer_modify.exchange(vec![]).drain(..) );
        cmds.renderer.target.push_some( self.renderer_target.exchange(vec![]).drain(..) );
        cmds.anime.attach.push_some( self.anime_attach.exchange(vec![]).drain(..) );
        cmds.anime.reset_while_start.push_some( self.anime_reset_while_start.exchange(vec![]).drain(..) );
        
        cmds.trail.create.push_some( self.trail_create.exchange(vec![]).drain(..) );
        cmds.trail.age.push_some( self.trail_age.exchange(vec![]).drain(..) );
        
        cmds.parsys.create.push_some( self.parsys_create.exchange(vec![]).drain(..) );
        cmds.parsys.calculator.push_some( self.parsys_calculator.exchange(vec![]).drain(..) );
        cmds.parsys.state.push_some( self.parsys_state.exchange(vec![]).drain(..) );
        cmds.parsys.trailmaterial.push_some( self.parsys_trailmaterial.exchange(vec![]).drain(..) );
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_commands_exchange(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3) {
    let mut sets = param.acts.get_mut(&mut app.world);
    cmds.exchange(&mut sets);
}