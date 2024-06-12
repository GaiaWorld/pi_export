use std::{mem::transmute, ops::Deref};

// use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_assets::asset::Handle;
use pi_scene_shell::{assets::texture::TextureKeyList, prelude::*};
use pi_scene_shell::prelude::single_res::TickRes;
pub use pi_export_base::export::Engine;
use pi_export_base::export::await_last_frame;
use pi_gltf2_load::{GLTFResLoader, KeyGLTF, PluginGLTF2Res, GLTF};
use pi_mesh_builder::{cube::PluginCubeBuilder, quad::PluginQuadBuilder};
use pi_node_materials::{prelude::*, NodeMaterialBlocks, PluginNodeMaterial, PluginNodeMaterialSimple};
use pi_particle_system::{PluginParticleSystem, prelude::*};
use pi_scene_context::{prelude::*, shadow::PluginShadowGenerator};
use pi_particle_system::prelude::*;
use pi_trail_renderer::{ActionSetTrailRenderer, PluginTrail, ResTrailBuffer, TrailPoints};
pub use pi_export_base::asset::Atom;
use system::TypeInfo;
use world::ComponentIndex;
use pi_slotmap::Key;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ImageRes(Handle<pi_render::renderer::texture::ImageTexture>);

use crate::{as_entity, as_f64};
pub use crate::commands::CommandsExchangeD3;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;
use js_proxy_gen_macro::pi_js_export;
use pi_hal::*;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_init_engine(app: &mut Engine) {
	await_last_frame(app);
    // use pi_scene_shell::frame_time::PluginFrameTime;
    println!("======== p3d_init_engine");

    if app.world.get_resource::<AssetMgrConfigs>().is_none() {
        app.insert_resource(AssetMgrConfigs::default());
    }

    PluginBundleDefault::add(app);
    app
        .add_plugins(PluginNodeMaterialSimple)
        .add_plugins(PluginShadowGenerator)
        .add_plugins(PluginShadowMapping)
        .add_plugins(PluginCubeBuilder)
        .add_plugins(PluginQuadBuilder)
        // .add_plugins(PluginParticleSystem)
        .add_plugins(PluginGLTF2Res)
        // .add_plugins(PluginTrail)
        ;

    app.add_systems(
        Update,
            sys_state_transform.in_set(ERunStageChap::StateCheck)
    );
}

#[derive(SystemParam)]
pub struct GlobalState<'w> {
    pub resource: Res<'w, pi_3d::StateResource>,
    pub performance: Res<'w, pi_scene_shell::prelude::Performance>,
    pub psperformance: Res<'w, pi_particle_system::prelude::ParticleSystemPerformance>,
    // pub statemesh: ResMut<'w, pi_scene_context::prelude::StateMesh>,
    pub statetransform: Res<'w, pi_scene_context::prelude::StateTransform>,
    pub statecamera: Res<'w, pi_scene_context::prelude::StateCamera>,
    pub statelight: Res<'w, pi_scene_context::prelude::StateLight>,
    // pub statecamera: ResMut<'w, pi_scene_context::prelude::StateCamera>,
    // pub statematerial: ResMut<'w, pi_scene_context::prelude::StateMaterial>,
    pub statetrail: Res<'w, pi_trail_renderer::StateTrail>,
}

macro_rules! get_component {
    // 空实现
    ($name: ident, $value_ty: ty) => {
        $crate::paste::item! {
            pub fn [<get_ $name>]<'a>(&self, world: &'a World, e: Entity) -> Result<&'a $value_ty, QueryError> {
                world.get_component_by_index::<$value_ty>(e, self.$name)
            }
        }
    };
}

macro_rules! get_single_res {
    // 空实现
    ($name: ident, $value_ty: ty) => {
        $crate::paste::item! {
            pub fn [<get_ $name>]<'a>(&self, world: &'a World) -> Option<&'a TickRes<$value_ty>> {
                world.index_single_res::<$value_ty>(self.$name)
            }
            pub fn [<get_ $name _mut>]<'a>(&self, world: &'a mut World) -> Option<&'a mut TickRes<$value_ty>> {
                world.index_single_res_mut::<$value_ty>(self.$name)
            }
        }
    };
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D {
    pub(crate) acts: <pi_3d::ActionSets<'static> as pi_scene_shell::prelude::SystemParam>::State,
    pub(crate) resource: <pi_3d::ResourceSets<'static> as pi_scene_shell::prelude::SystemParam>::State,
    pub(crate) state: <GlobalState<'static> as pi_scene_shell::prelude::SystemParam>::State,

    pub(crate) acts_meta: SystemMeta,
    pub(crate) resource_meta: SystemMeta,
    pub(crate) state_meta: SystemMeta,
    
    pub(crate) anime_events: usize, // GlobalAnimationEvents,
    pub(crate) default_mat: usize,
    pub(crate) node_material_blocks: usize,
    pub(crate) imgtex_loader: usize,
    pub(crate) imgtex_loader_state: usize,
    pub(crate) imgtex_asset: usize,
    pub(crate) imgtexview_asset: usize,
    pub(crate) gltf2_asset: usize,
    pub(crate) gltf2_loader: usize,
    pub(crate) device: usize,
    pub(crate) queue: usize,
    pub(crate) render_targets: usize,
    pub(crate) asset_samp: usize,
    pub(crate) asset_atlas: usize,
    pub(crate) scene_lighting_limit: usize,
    pub(crate) model_lighting_limit: usize,
    pub(crate) scene_shadow_limit: usize,
    pub(crate) vb_mgr: usize,
    pub(crate) vb_wait: usize,
    pub(crate) shader_metas: usize,
    pub(crate) anime_global: usize,
    pub(crate) trailbuffer: usize,
    pub(crate) error_record: usize,
    pub(crate) calcultors: usize,
    pub(crate) calculator_queue: usize,


    // pub(crate) tree: SystemState<EntityTree<'static, 'static>>,
    pub(crate) up: ComponentIndex,
    pub(crate) down: ComponentIndex,
    pub(crate) world_transform: ComponentIndex, // QueryState<&'static GlobalMatrix>,
    pub(crate) local_transform: ComponentIndex, // QueryState<&'static LocalMatrix>,
    pub(crate) view_matrix: ComponentIndex, // QueryState<&'static ViewerViewMatrix>,
    pub(crate) project_matrix: ComponentIndex, // QueryState<&'static ViewerProjectionMatrix>,
    pub(crate) vp_matrix: ComponentIndex, // QueryState<&'static ViewerTransformMatrix>,

    // meshes
    pub(crate) scene_id: ComponentIndex, // SceneID
    pub(crate) global_enable: ComponentIndex, // GlobalEnable
    pub(crate) render_geometry_eable: ComponentIndex, // RenderGeometryEable
    pub(crate) instance_mesh: ComponentIndex, // InstanceMesh
    pub(crate) abstruct_mesh: ComponentIndex, // AbstructMesh

    // materials
    pub(crate) asset_res_shader_effect_meta: ComponentIndex, // AssetResShaderEffectMeta
    pub(crate) effect_texture_samplers_comp: ComponentIndex, // EffectTextureSamplersComp
    pub(crate) texture_key_list: ComponentIndex, // Option TextureKeyList

    // transforms
    pub(crate) enable: ComponentIndex, // Enable

    // transforms
    pub(crate) camera: ComponentIndex, // Camera
    pub(crate) model_list: ComponentIndex, // ModelList
    pub(crate) model_list_after_culling: ComponentIndex, // ModelListAfterCulling

    // renderers
    pub(crate) viewer_id: ComponentIndex, // ViewerID
    pub(crate) renderer: ComponentIndex, // Renderer

    // viewers
    pub(crate) viewer_active: ComponentIndex, // ViewerActive

    // particlesystems
    pub(crate) particle_ids: ComponentIndex, // ParticleIDs
    // animectxs
    pub(crate) scene_animation_context: ComponentIndex, // SceneAnimationContext
    // trails
    pub(crate) trails: ComponentIndex, // SceneAnimationContext
    // model
    pub(crate) pass_ids: ComponentIndex, // PassIDs
    // pass
    pub(crate) pass_viewer_id: ComponentIndex, // PassViewerID
    pub(crate) pass_renderer_id: ComponentIndex, // PassRendererID
    pub(crate) pass_material_id: ComponentIndex, // PassMaterialID
    pub(crate) pass_bind_group_scene: ComponentIndex, // Option PassBindGroupScene
    pub(crate) pass_bind_group_model: ComponentIndex, // Option PassBindGroupModel
    pub(crate) pass_bind_group_texture_samplers: ComponentIndex, // Option PassBindGroupTextureSamplers
    pub(crate) pass_bind_group_lighting_shadow: ComponentIndex, // Option PassBindGroupLightingShadow
    pub(crate) pass_bind_group: ComponentIndex, // Option PassBindGroups
    pub(crate) pass_shader: ComponentIndex, // Option PassShader
    pub(crate) pass_draw: ComponentIndex, // Option PassDraw

    // nodes
    pub(crate) layer: ComponentIndex, // Layer
    pub(crate) mesh: ComponentIndex, // Option Mesh
    pub(crate) direct_light: ComponentIndex, // Option DirectLight
    pub(crate) point_light: ComponentIndex, // Option PointLight
    
    // pub(crate) uniforms: QueryState<&'static BindEffect>,
    // pub(crate) animatorablefloat: QueryState<&'static AnimatorableFloat>,
    // pub(crate) animatorablevec2s: QueryState<&'static AnimatorableVec2>,
    // pub(crate) animatorablevec3s: QueryState<&'static AnimatorableVec3>,
    // pub(crate) animatorablevec4s: QueryState<&'static AnimatorableVec4>,
    // pub(crate) animatorableuints: QueryState<&'static AnimatorableUint>,
    // pub(crate) animatorablesints: QueryState<&'static AnimatorableSint>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(app: &mut Engine) -> Self {
		pi_export_base::export::await_last_frame(app);
        let mut acts_meta = SystemMeta::new(TypeInfo::of::<()>());
        let mut resource_meta = SystemMeta::new(TypeInfo::of::<()>());
        let mut state_meta = SystemMeta::new(TypeInfo::of::<()>());
        Self {

            acts: <pi_3d::ActionSets<'static> as pi_scene_shell::prelude::SystemParam>::init_state(&mut app.world, &mut acts_meta),
            resource: <pi_3d::ResourceSets<'static> as pi_scene_shell::prelude::SystemParam>::init_state(&mut app.world, &mut resource_meta),
            state: <GlobalState<'static> as pi_scene_shell::prelude::SystemParam>::init_state(&mut app.world, &mut state_meta),

            acts_meta,
            resource_meta,
            state_meta,

            anime_events: app.world.register_single_res::<GlobalAnimeEvents>(), // GlobalAnimeEvents
            default_mat: app.world.register_single_res::<SingleIDBaseDefaultMaterial>(),
            node_material_blocks: app.world.register_single_res::<NodeMaterialBlocks>(),
            imgtex_loader: app.world.register_single_res::<ImageTextureLoader>(),
            imgtex_loader_state: app.world.register_single_res::<StateTextureLoader>(),
            imgtex_asset: app.world.register_single_res::<ShareAssetMgr<ImageTexture>>(),
            imgtexview_asset: app.world.register_single_res::<ImageTextureView>(),
            gltf2_asset: app.world.register_single_res::<ShareAssetMgr<GLTF>>(),
            gltf2_loader: app.world.register_single_res::<GLTFResLoader>(),
            device: app.world.register_single_res::<PiRenderDevice>(),
            queue: app.world.register_single_res::<PiRenderQueue>(),
            render_targets: app.world.register_single_res::<CustomRenderTargets>(),
            asset_samp: app.world.register_single_res::<ShareAssetMgr<SamplerRes>>(),
            asset_atlas: app.world.register_single_res::<PiSafeAtlasAllocator>(),
            scene_lighting_limit: app.world.register_single_res::<SceneLightLimit>(),
            model_lighting_limit: app.world.register_single_res::<ModelLightLimit>(),
            scene_shadow_limit: app.world.register_single_res::<SceneShadowLimit>(),
            vb_mgr: app.world.register_single_res::<ShareAssetMgr<EVertexBufferRange>>(),
            vb_wait: app.world.register_single_res::<VertexBufferDataMap3D>(),
            shader_metas: app.world.register_single_res::<ShareAssetMgr<ShaderEffectMeta>>(),
            // anime_scene_ctxs:,
            anime_global: app.world.register_single_res::<GlobalAnimeAbout>(),
            trailbuffer: app.world.register_single_res::<ResTrailBuffer>(),
            error_record: app.world.register_single_res::<ErrorRecord>(),

            calcultors: app.world.register_single_res::<ShareAssetMgr<ParticleSystemCalculatorID>>(),
            calculator_queue: app.world.register_single_res::<ResParticleCalculatorUninstallQueue>(),   

            // tree: SystemState<EntityTree<'static, 'static>>,
            up: app.world.init_component::<Up>(),
            down: app.world.init_component::<Down>(),
            world_transform: app.world.init_component::<GlobalMatrix>(), // QueryState<&'static GlobalMatrix>,
            local_transform: app.world.init_component::<LocalMatrix>(), // QueryState<&'static LocalMatrix>,
            view_matrix: app.world.init_component::<ViewerViewMatrix>(), // QueryState<&'static ViewerViewMatrix>,
            project_matrix: app.world.init_component::<ViewerProjectionMatrix>(), // QueryState<&'static ViewerProjectionMatrix>,
            vp_matrix: app.world.init_component::<ViewerTransformMatrix>(), // QueryState<&'static ViewerTransformMatrix>,

            // meshes
            scene_id: app.world.init_component::<SceneID>(), // SceneID
            global_enable: app.world.init_component::<GlobalEnable>(), // GlobalEnable
            render_geometry_eable: app.world.init_component::<RenderGeometryEable>(), // RenderGeometryEable
            instance_mesh: app.world.init_component::<InstanceMesh>(), // InstanceMesh
            abstruct_mesh: app.world.init_component::<AbstructMesh>(), // AbstructMesh

            // materials
            asset_res_shader_effect_meta: app.world.init_component::<AssetResShaderEffectMeta>(), // AssetResShaderEffectMeta
            effect_texture_samplers_comp: app.world.init_component::<EffectTextureSamplersComp>(), // EffectTextureSamplersComp
            texture_key_list: app.world.init_component::<TextureKeyList>(), // Option TextureKeyList

            // transforms
            enable: app.world.init_component::<Enable>(), // Enable

            // transforms
            camera: app.world.init_component::<Camera>(), // Camera
            model_list: app.world.init_component::<ModelList>(), // ModelList
            model_list_after_culling: app.world.init_component::<ModelListAfterCulling>(), // ModelListAfterCulling

            // renderers
            viewer_id: app.world.init_component::<ViewerID>(), // ViewerID
            renderer: app.world.init_component::<Renderer>(), // Renderer

            // viewers
            viewer_active: app.world.init_component::<ViewerActive>(), // ViewerActive

            // particlesystems
            particle_ids: app.world.init_component::<ParticleIDs>(), // ParticleIDs
            // animectxs
            scene_animation_context: app.world.init_component::<SceneAnimationContext>(), // SceneAnimationContext
            // trails
            trails: app.world.init_component::<SceneAnimationContext>(), // SceneAnimationContext
            // model
            pass_ids: app.world.init_component::<PassIDs>(), // PassIDs
            // pass
            pass_viewer_id: app.world.init_component::<PassViewerID>(), // PassViewerID
            pass_renderer_id: app.world.init_component::<PassRendererID>(), // PassRendererID
            pass_material_id: app.world.init_component::<PassMaterialID>(), // PassMaterialID
            pass_bind_group_scene: app.world.init_component::<PassBindGroupScene>(), // Option PassBindGroupScene
            pass_bind_group_model: app.world.init_component::<PassBindGroupModel>(), // Option PassBindGroupModel
            pass_bind_group_texture_samplers: app.world.init_component::<PassBindGroupTextureSamplers>(), // Option PassBindGroupTextureSamplers
            pass_bind_group_lighting_shadow: app.world.init_component::<PassBindGroupLightingShadow>(), // Option PassBindGroupLightingShadow
            pass_bind_group: app.world.init_component::<PassBindGroups>(), // Option PassBindGroups
            pass_shader: app.world.init_component::<PassShader>(), // Option PassShader
            pass_draw: app.world.init_component::<PassDraw>(), // Option PassDraw

            // nodes
            layer: app.world.init_component::<Layer>(), // Layer
            mesh: app.world.init_component::<Mesh>(), // Option Mesh
            direct_light: app.world.init_component::<DirectLight>(), // Option DirectLight
            point_light: app.world.init_component::<PointLight>(), // Option PointLight

        }
    }
}

impl ActionSetScene3D {
    get_single_res!(anime_events, GlobalAnimeEvents);
    get_single_res!(default_mat, SingleIDBaseDefaultMaterial);
    get_single_res!(node_material_blocks, NodeMaterialBlocks);
    get_single_res!(imgtex_loader, ImageTextureLoader);
    get_single_res!(imgtex_loader_state, StateTextureLoader);
    get_single_res!(imgtex_asset, ShareAssetMgr<ImageTexture>);
    get_single_res!(imgtexview_asset, ImageTextureView);
    get_single_res!(gltf2_asset, ShareAssetMgr<GLTF>);
    get_single_res!(gltf2_loader, GLTFResLoader);
    get_single_res!(device, PiRenderDevice);
    get_single_res!(queue, PiRenderQueue);
    get_single_res!(render_targets, CustomRenderTargets);
    get_single_res!(asset_samp, ShareAssetMgr<SamplerRes>);
    get_single_res!(asset_atlas, PiSafeAtlasAllocator);
    get_single_res!(scene_lighting_limit, SceneLightLimit);
    get_single_res!(model_lighting_limit, ModelLightLimit);
    get_single_res!(scene_shadow_limit, SceneShadowLimit);
    get_single_res!(vb_mgr, ShareAssetMgr<EVertexBufferRange>);
    get_single_res!(vb_wait, VertexBufferDataMap3D);
    get_single_res!(shader_metas, ShareAssetMgr<ShaderEffectMeta>);
    get_single_res!(anime_global, GlobalAnimeEvents);
    get_single_res!(trailbuffer, ResTrailBuffer);
    get_single_res!(error_record, ErrorRecord);

    get_single_res!(calcultors, ShareAssetMgr<ParticleSystemCalculatorID>);
    get_single_res!(calculator_queue, ResParticleCalculatorUninstallQueue);


    get_component!(up, Up);
    get_component!(down, Down);
    get_component!(world_transform, GlobalMatrix);
    get_component!(local_transform, LocalMatrix);
    get_component!(view_matrix, ViewerViewMatrix);
    get_component!(project_matrix, ViewerProjectionMatrix);
    get_component!(vp_matrix, ViewerTransformMatrix);
    get_component!(scene_id, SceneID);
    get_component!(global_enable, GlobalEnable);
    get_component!(render_geometry_eable, RenderGeometryEable);
    get_component!(instance_mesh, InstanceMesh);
    get_component!(abstruct_mesh, AbstructMesh);
    get_component!(asset_res_shader_effect_meta, AssetResShaderEffectMeta);
    get_component!(effect_texture_samplers_comp, EffectTextureSamplersComp);
    get_component!(texture_key_list, TextureKeyList);
    get_component!(enable, Enable);
    get_component!(camera, Camera);
    get_component!(model_list, ModelList);
    get_component!(model_list_after_culling, ModelListAfterCulling);
    get_component!(viewer_id, ViewerID);
    get_component!(renderer, Renderer);
    get_component!(viewer_active, ViewerActive);
    get_component!(particle_ids, ParticleIDs);
    get_component!(scene_animation_context, SceneAnimationContext);
    get_component!(pass_ids, PassIDs);
    get_component!(pass_viewer_id, PassViewerID);
    get_component!(pass_renderer_id, PassRendererID);
    get_component!(pass_material_id, PassMaterialID);
    get_component!(pass_bind_group_scene, PassBindGroupScene);
    get_component!(pass_bind_group_model, PassBindGroupModel);
    get_component!(pass_bind_group_texture_samplers, PassBindGroupTextureSamplers);
    get_component!(pass_bind_group_lighting_shadow, PassBindGroupLightingShadow);
    get_component!(pass_bind_group, PassBindGroups);
    get_component!(pass_shader, PassShader);
    get_component!(pass_draw, PassDraw);
    get_component!(layer, Layer);
    get_component!(mesh, Mesh);
    get_component!(direct_light, DirectLight);
    get_component!(point_light, PointLight);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_entity(app: &mut Engine) -> f64 {
    let id: Entity = app.world.make_entity_editor().alloc_entity();
    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_dispose(cmds: &mut CommandsExchangeD3, entity: f64) {
    let entity: Entity = as_entity(entity);

    cmds.obj_dispose.push(OpsDispose::ops(entity));
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_scene_dispose(cmds: &mut CommandsExchangeD3, scene: f64) {
    let entity: Entity = as_entity(scene);

    cmds.scene_dispose.push(OpsSceneDispose::ops(entity));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_lighting_shadow_limit(app: &mut Engine, param: &mut ActionSetScene3D, 
    scene_max_direct_light_count: f64,
    scene_max_point_light_count: f64,
    scene_max_spot_light_count: f64,
    scene_max_hemi_light_count: f64,
    scene_max_shadow_count: f64,
    model_max_direct_light_count: f64,
    model_max_point_light_count: f64,
    model_max_spot_light_count: f64,
    model_max_hemi_light_count: f64,
) {

	pi_export_base::export::await_last_frame(app);
    
    let tick = app.world.tick();
    let mut resource = <pi_3d::ResourceSets<'static> as pi_scene_shell::prelude::SystemParam>::get_self(&mut app.world, &param.resource_meta, &mut param.resource, tick);

    resource.scene_lighting_limit.0.max_direct_light_count = scene_max_direct_light_count as u32;
    resource.scene_lighting_limit.0.max_point_light_count = scene_max_point_light_count as u32;
    resource.scene_lighting_limit.0.max_spot_light_count = scene_max_spot_light_count as u32;
    resource.scene_lighting_limit.0.max_hemi_light_count = scene_max_hemi_light_count as u32;
    
    resource.scene_shadow_limit.0.max_count = scene_max_shadow_count as u32;

    resource.model_lighting_limit.0.max_direct_light_count = model_max_direct_light_count as u32;
    resource.model_lighting_limit.0.max_point_light_count = model_max_point_light_count as u32;
    resource.model_lighting_limit.0.max_spot_light_count = model_max_spot_light_count as u32;
    resource.model_lighting_limit.0.max_hemi_light_count = model_max_hemi_light_count as u32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_render_graphic(cmds: &mut CommandsExchangeD3, before: f64, after: f64, isdisconnect: bool) {
    let before: Entity = as_entity(before);
    let after: Entity = as_entity(after);

    cmds.renderer_connect.push(OpsRendererConnect::ops(before, after, isdisconnect));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_world_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	// pi_export_base::export::await_last_frame(app);
    // let entity: Entity = as_entity(entity);

    // if let Ok(trans) = app.world.get_component_by_index::<GlobalMatrix>(entity, param.world_transform) {
    //     let mut i = 0;
    //     trans.matrix.as_slice().iter().for_each(|val| {
    //         matrix[i] = *val;
    //         i += 1;
    //     });
    //     true
    // } else {
    //     false
    // }
    false
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_scene_state(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, result: &mut [f32]) -> bool {
	// pi_export_base::export::await_last_frame(app);
    // let entity: Entity = as_entity(entity);

    // let mut drawcalls = 0;
    // let mut count_vertex = 0;
    // param.renderers.iter(&app.world).for_each(|(idviewer, renderer)| {
    //     if let Ok((_, idscene)) = param.viewers.get(&app.world, idviewer.0) {
    //         if idscene.0 == entity {
    //             drawcalls += renderer.draws.list.len();
    //             count_vertex += renderer.vertexs;
    //         }
    //     }
    // });

    // let mut count_particlesys = 0;
    // let mut count_particle = 0;
    // param.particlesystems.iter(&app.world).for_each(|(particles, idscene)| {
    //     if idscene.0 == entity {
    //         count_particlesys += 1;
    //         count_particle += particles.count();
    //     }
    // });
    
    // let mut count_trail = 0;
    // let mut count_trail_point = 0;
    // param.trails.iter(&app.world).for_each(|(trail, idscene)| {
    //     if idscene.0 == entity {
    //         count_trail += 1;
    //         count_trail_point += trail.0.len();
    //     }
    // });

    // let mut count_animegroup = 0;
    // if let Ok(ctx) = app.world.get_component_by_index::<SceneAnimationContext>(entity, param.scene_animation_context) {
    //     count_animegroup += ctx.0.group_mgr.groups.len();
    // }
    
    // result[0] = drawcalls as f32;
    // result[1] = count_vertex as f32;
    // result[2] = count_particlesys as f32;
    // result[3] = count_particle as f32;
    // result[4] = count_animegroup as f32;
    // result[5] = count_trail as f32;
    // result[6] = count_trail_point as f32;

    true
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_performance_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	// pi_export_base::export::await_last_frame(app);
    
    // let cmds = param.state.get_mut(&mut app.world);

    // result[0] = cmds.performance.animation as f32;
    // result[1] = cmds.performance.animationgroup as f32;
    // result[2] = cmds.psperformance.total() as f32;
    // result[3] = cmds.statetransform.calc_world_time as f32;
    // result[4] = cmds.statecamera.culling_time as f32 + cmds.statelight.culling_time as f32;
    // result[5] = cmds.performance.gltfanaly as f32;
    // result[6] = cmds.performance.drawobjs as f32;
    // result[7] = cmds.performance.uniformupdate as f32;
    // result[8] = cmds.performance.uniformbufferupdate as f32;
    // result[9] = cmds.statetrail.calc_time as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_resource_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	// pi_export_base::export::await_last_frame(app);
    
    // let cmds = param.state.get_mut(&mut app.world);

    // result[ 0] = cmds.resource.count_bindbuffer as f32;
    // result[ 1] = cmds.resource.count_bindgroup as f32;
    // result[ 2] = cmds.resource.count_gltf as f32;
    // result[ 3] = cmds.resource.count_imgtexture as f32;
    // result[ 4] = cmds.resource.count_shader as f32;
    // result[ 5] = cmds.resource.count_pipeline as f32;
    // result[ 6] = (cmds.resource.size_geometrybuffer / 1024) as f32;
    // result[ 7] = cmds.resource.count_geometrybuffer as f32;
    // result[ 8] = cmds.resource.count_shadermeta as f32;
    // result[ 9] = cmds.resource.mem_shadermeta as f32;
    // result[10] = cmds.resource.mem_shader as f32;
    // result[11] = cmds.resource.mem_bindbuffer as f32;
    // result[12] = cmds.resource.mem_imgtexture as f32;

    // result[13] = cmds.resource.count_material as f32;
    // result[14] = cmds.resource.count_passset0 as f32;
    // result[15] = cmds.resource.count_passset1 as f32;
    // result[16] = cmds.resource.count_passset2 as f32;
    // result[17] = cmds.resource.count_passbindgroups as f32;
    // result[18] = cmds.resource.count_passshader as f32;
    // result[19] = cmds.resource.count_passpipeline as f32;
    // result[20] = cmds.resource.count_passdraw as f32;

    // result[21] = cmds.resource.count_passmat as f32;
    // result[22] = cmds.resource.count_passtexs as f32;
    // result[23] = cmds.resource.count_vertex as f32;

}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_material_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
    // let mut state = StateMaterial::default();
    // param.materials.iter(&app.world).for_each(|(meta, texs, _)| {
    //     state.count += 1;
    //     let texcount = meta.0.as_ref().unwrap().textures.len();
    //     let mut isready = false;
    //     if let Some(texs) = &texs.0 {
    //         if texcount == texs.textures.len() {
    //             isready = true;
    //         }
    //     } else if texcount == 0 {
    //         isready = false;
    //     }
    //     if isready { state.count_ready += 1; }

    //     if texcount == 0 {
    //         state.count_tex0 += 1;
    //         if isready { state.count_tex0_ready += 1; }
    //     }
    //     else if texcount == 1 {
    //         state.count_tex1 += 1;
    //         if isready { state.count_tex1_ready += 1; }
    //     }
    //     else if texcount == 2 {
    //         state.count_tex2 += 1;
    //         if isready { state.count_tex2_ready += 1; }
    //     }
    //     else if texcount == 3 {
    //         state.count_tex3 += 1;
    //         if isready { state.count_tex3_ready += 1; }
    //     }
    //     else if texcount == 4 {
    //         state.count_tex4 += 1;
    //         if isready { state.count_tex4_ready += 1; }
    //     }
    //     else if texcount == 5 {
    //         state.count_tex5 += 1;
    //         if isready { state.count_tex5_ready += 1; }
    //     }
    //     else if texcount == 6 {
    //         state.count_tex6 += 1;
    //         if isready { state.count_tex6_ready += 1; }
    //     }
    // });

    // result[ 0] = state.count as f32;
    // result[ 1] = state.count_ready as f32;
    // result[ 2] = state.count_tex0 as f32;
    // result[ 3] = state.count_tex0_ready as f32;
    // result[ 4] = state.count_tex1 as f32;
    // result[ 5] = state.count_tex1_ready as f32;
    // result[ 6] = state.count_tex2 as f32;
    // result[ 7] = state.count_tex2_ready as f32;
    // result[ 8] = state.count_tex3 as f32;
    // result[ 9] = state.count_tex3_ready as f32;
    // result[10] = state.count_tex4 as f32;
    // result[11] = state.count_tex4_ready as f32;
    // result[12] = state.count_tex5 as f32;
    // result[13] = state.count_tex5_ready as f32;
    // result[14] = state.count_tex6 as f32;
    // result[15] = state.count_tex6_ready as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_mesh_state(app: &mut Engine, param: &mut ActionSetScene3D, scene: Option<f64>, result: &mut [f32]) {

    // let mut state = StateMesh::default();
    // if let Some(scene) = scene {
    //     let scene = as_entity(scene);
    //     param.meshes.iter(&app.world).for_each(|(idscene, enable, geoenable, instance, _)| {
    //         if idscene.0 == scene {
    //             state.abstructmesh += 1;
    //             if enable.0 { state.abstructenable_count += 1; }
    //             if let Some(geoenable) = geoenable { 
    //                 state.meshes += 1;
    //                 if geoenable.0 { state.geometry_enable += 1; }
    //             }
    //             if instance.is_some() { state.instances += 1; }
    //         }
    //     });
    // }
    // result[0] = state.abstructmesh as f32;
    // result[1] = state.abstructenable_count as f32;
    // result[2] = state.meshes as f32;
    // result[3] = state.geometry_enable as f32;
    // result[4] = state.instances as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_transform_state(app: &mut Engine, param: &mut ActionSetScene3D, scene: Option<f64>, result: &mut [f32]) {
	// pi_export_base::export::await_last_frame(app);
    // let mut state = StateTransform::default();
    // if let Some(scene) = scene {
    //     let scene = as_entity(scene);
    //     app.world.make_entity_editor().make_queryer::<(&SceneID, &Enable, &GlobalEnable), ()>().iter().for_each(|(idscene, enable, globalenable)| {
    //         if idscene.0 == scene {
    //             state.count += 1;
    //             if enable.bool() { state.enable += 1; }
    //             if globalenable.0 { state.global_enable += 1; }
    //         }
    //     });

    //     let statetransform = app.world.get_single_res::<StateTransform>().unwrap();
    //     state.calc_local_time   = statetransform.calc_local_time;
    //     state.calc_world_time   = statetransform.calc_world_time;
    //     state.max_level         = statetransform.max_level;
    // }
    // result[0] = state.count as f32;
    // result[1] = state.enable as f32;
    // result[2] = state.global_enable as f32;
    // result[3] = state.calc_local_time as f32;
    // result[4] = state.calc_world_time as f32;
    // result[5] = state.max_level as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_transform_state(app: &mut Engine, param: &mut ActionSetScene3D, transform: Option<f64>, result: &mut [f32]) -> bool {
	// pi_export_base::export::await_last_frame(app);
    // let mut state = StateTransform::default();
    // if let Some(transform) = transform {
    //     let transform = as_entity(transform);
    //     if let Ok((idscene, enable, globalenable)) = app.world.get_component_by_index::<Scene>(transform, param.scene_id) {
    //         result[0] = enable.0;
    //         result[1] = if globalenable.0 { 1. } else { 0. };
    //         true
    //     } else {
    //         false
    //     }
    // } else {
    //     false
    // }
    false
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_camera_state(app: &mut Engine, param: &mut ActionSetScene3D, camera: Option<f64>, result: &mut [f32]) {
    
    // let mut cmds = param.state.get_mut(&mut app.world);

    // let mut state = StateCamera::default();
    // if let Some(camera) = camera {
    //     let camera = as_entity(camera);
    //     if let Ok((_camera, includes, cullings)) = param.cameras.get(&app.world, camera) {
    //         state.includes  = includes.0.len() as u32;
    //         state.culling   = cullings.0.len() as u32;
    //     }
    //     let cmds = param.state.get(&mut app.world);
    //     state.culling_time = cmds.statecamera.culling_time;
    // }

    // result[0] = state.includes as f32;
    // result[1] = state.culling as f32;
    // result[2] = state.culling_time as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_texture_loader_state(app: &mut Engine, param: &mut ActionSetScene3D, result: &mut [f32]) {
	// pi_export_base::export::await_last_frame(app);
    // let resource = param.resource.get_mut(&mut app.world);

    // result[ 0] = resource.imgtex_loader_state.image_count as f32;
    // result[ 1] = resource.imgtex_loader_state.image_fail as f32;
    // result[ 2] = resource.imgtex_loader_state.image_success as f32;
    // result[ 3] = resource.imgtex_loader_state.image_waiting as f32;
    // result[ 4] = resource.imgtex_loader_state.texview_count as f32;
    // result[ 5] = resource.imgtex_loader_state.texview_fail as f32;
    // result[ 6] = resource.imgtex_loader_state.texview_success as f32;
    // result[ 7] = resource.imgtex_loader_state.texview_waiting as f32;
    // result[ 8] = resource.imgtex_asset.len() as f32;
    // result[ 9] = resource.imgtex_asset.size() as f32;
    // result[10] = resource.imgtexview_asset.len() as f32;
    // result[11] = resource.imgtexview_asset.size() as f32;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_errors(app: &mut Engine, param: &mut ActionSetScene3D, info: &mut [u32], flag: bool) -> f64 {
    pi_export_base::export::await_last_frame(app);
    let count = info.len();
    let mut error_record = app.world.get_single_res_mut::<ErrorRecord>().unwrap();
    error_record.1 = flag;
    let mut idx = 0;
    error_record.drain(count).for_each(|v| {
        info[idx] = v;
        idx += 1;
    });

    idx as f64
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_global_state(app: &mut Engine, param: &mut ActionSetScene3D, val: bool) {

    // let cmds = param.state.get(&mut app.world);
    // cmds.resource.debug = val;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_local_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    if let Ok(trans) = (*param).get_local_transform(&app.world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_view_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    if let Ok(trans) = param.get_view_matrix(&app.world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_project_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    if let Ok(trans) = param.get_project_matrix(&app.world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_viewproject_matrix(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, matrix: &mut [f32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let entity: Entity = as_entity(entity);

    if let Ok(trans) = param.get_vp_matrix(&app.world, entity) {
        let mut i = 0;
        trans.0.as_slice().iter().for_each(|val| {
            matrix[i] = *val;
            i += 1;
        });
        true
    } else {
        false
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct GLTFRes(Handle<GLTF>);

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_gltf_val(item: &GLTFRes) -> String {
    item.0.output.clone()
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_curve_id_bygltf(
    gltf: &GLTFRes,
    group_index: f64,
    channel_index: f64,
) -> f64 {
    let key = gltf.0.key_anime_curve(group_index as usize, channel_index as usize);
    unsafe { transmute(key) }
}

pub fn gltf_particle_calculator(item: &GLTFRes, index: f64) -> Option<&Handle<ParticleSystemCalculatorID>> {
    let index = index as usize;
    item.0.particlesys_calculators.get(&index)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64, baseurl: &Atom, dyndesc: String) {
    let gltf2_loader = app.world.get_single_res_mut::<GLTFResLoader>().unwrap();

    let entity: Entity = as_entity(entity);

    let param = KeyGLTF { base_url: baseurl.deref().clone(), dyn_desc: pi_atom::Atom::from(dyndesc) };

    gltf2_loader.create_load(entity, param);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_gltf_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
	pi_export_base::export::await_last_frame(app);
    let gltf2_loader = app.world.get_single_res_mut::<GLTFResLoader>().unwrap();

    let max = success.len();
    let mut item = gltf2_loader.success.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = gltf2_loader.success.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = gltf2_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = as_f64(&entity);

        idx += 1;
        if idx >= max {
            break;
        }
        item = gltf2_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<GLTFRes> {
	pi_export_base::export::await_last_frame(app);
    let gltf2_loader = app.world.get_single_res_mut::<GLTFResLoader>().unwrap();
    let entity: Entity = as_entity(entity);
    if let Some(val) = gltf2_loader.get_success(entity) {
        Some(GLTFRes(val))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_gltf_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, entity: f64) -> Option<String> {
	pi_export_base::export::await_last_frame(app);
    let gltf2_loader = app.world.get_single_res_mut::<GLTFResLoader>().unwrap();
    let entity: Entity = as_entity(entity);
    gltf2_loader.get_fail_reason(entity)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_create_image_load(app: &mut Engine, param: &mut ActionSetScene3D, url: &Atom, srgb: bool, compressed: bool, depth_or_array_layers: f64) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let imgtex_loader = app.world.get_single_res_mut::<ImageTextureLoader>().unwrap();

    let id = imgtex_loader.create_load(KeyImageTexture { 
        url: url.deref().clone(),
        srgb,
        file: true,
        compressed,
        depth_or_array_layers: depth_or_array_layers as u8,
        useage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING
    });

    unsafe { transmute(id) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_image_load(app: &mut Engine, param: &mut ActionSetScene3D, success: &mut [f64], failed: &mut [f64]) {
	pi_export_base::export::await_last_frame(app);
    let imgtex_loader = app.world.get_single_res_mut::<ImageTextureLoader>().unwrap();

    let max = success.len();
    let mut item = imgtex_loader.success_load.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        success[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = imgtex_loader.success_load.pop();
    }
    success[idx] = 0.;
    
    let max = success.len();
    let mut item = imgtex_loader.fails.pop();
    let mut idx = 0;
    while let Some(entity) = item {
        failed[idx] = unsafe { transmute(entity) };

        idx += 1;
        if idx >= max {
            break;
        }

        item = imgtex_loader.fails.pop();
    }
    failed[idx] = 0.;
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<ImageRes> {
	pi_export_base::export::await_last_frame(app);
    let imgtex_loader = app.world.get_single_res_mut::<ImageTextureLoader>().unwrap();
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    if let Some(img) = imgtex_loader.query_success(id) {
        Some(ImageRes(img))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_get_image_fail_reason(app: &mut Engine, param: &mut ActionSetScene3D, id: f64) -> Option<String> {
	pi_export_base::export::await_last_frame(app);
    let imgtex_loader = app.world.get_single_res_mut::<ImageTextureLoader>().unwrap();
    let id: IDImageTextureLoad = unsafe { transmute(id) };
    imgtex_loader.query_failed_reason(id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_children(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [f64]) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let id = as_entity(id);
    let mut idx = 0;
    if let Ok(down) = param.get_down(&app.world, id) {
        let mut child = down.head();
        while !child.is_null() {
            if let (Ok(idscene), Ok(enable), Ok(genable), Ok(layer), instance, mesh, camera, directlight, pointlight) = (
                param.get_scene_id(&app.world, child),
                param.get_enable(&app.world, child),
                param.get_global_enable(&app.world, child),
                param.get_layer(&app.world, child),
                param.get_instance_mesh(&app.world, child),
                param.get_mesh(&app.world, child),
                param.get_camera(&app.world, child),
                param.get_direct_light(&app.world, child),
                param.get_point_light(&app.world, child),
            ) {
                let mut ntype = 1;
                ntype |= if enable.bool()   { 2 } else { 0 };
                ntype |= if genable.0       { 4 } else { 0 };
                if instance.is_ok()       { ntype |= 8 };
                if mesh.is_ok()           { ntype |= 16 };
                if camera.is_ok()         { ntype |= 32 };
                if directlight.is_ok()    { ntype |= 64 };
                if pointlight.is_ok()     { ntype |= 128 };
                let id = as_f64(&child);

                info[idx] = id; idx += 1;
                info[idx] = ntype as f64; idx += 1;
            }

            let up = param.get_up(&app.world, child);
        }
    }

    idx as f64
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_mesh_info(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [u32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let id = as_entity(id);
    if let (Ok(geoenable), Ok(passids)) = (param.get_global_enable(&app.world, id), param.get_pass_ids(&app.world, id)) {
        let temp = passids.0;
        for i in 0..8 {
            if let (Ok(idviewer), Ok(idrenderer), Ok(idrmaterial), set0, set1, set2, set3, bindgroups, shader, draw) = (
                param.get_viewer_id(&app.world, temp[i]),
                param.get_pass_renderer_id(&app.world, temp[i]),
                param.get_pass_material_id(&app.world, temp[i]),
                param.get_pass_bind_group_scene(&app.world, temp[i]),
                param.get_pass_bind_group_model(&app.world, temp[i]),
                param.get_pass_bind_group_texture_samplers(&app.world, temp[i]),
                param.get_pass_bind_group_lighting_shadow(&app.world, temp[i]),
                param.get_pass_bind_group(&app.world, temp[i]),
                param.get_pass_shader(&app.world, temp[i]),
                param.get_pass_draw(&app.world, temp[i]),
            ) {
                info[i * 3 + 0] = idviewer.0.index() as u32;
                info[i * 3 + 1] = idrenderer.0.index() as u32;
                let mut state: u32 = 0;
                if let Ok(set0) = set0 {
                    if set0.val().is_some() { state |= 1 << 0; }
                }
                if let Ok(set0) = set1 {
                    if set0.val().is_some() { state |= 1 << 1; }
                }
                if let Ok(set0) = set2 {
                    if set0.val().is_some() { state |= 1 << 2; }
                }
                if let Ok(set0) = bindgroups {
                    if set0.val().is_some() { state |= 1 << 3; }
                }
                if let Ok(set0) = shader {
                    if set0.val().is_some() { state |= 1 << 4; }
                }
                if let Ok(set0) = draw {
                    if set0.val().is_some() { state |= 1 << 5; }
                }
                if let Ok(set0) = set3 {
                    if set0.val().is_some() { state |= 1 << 6; }
                }
                info[i * 3 + 2] = state;
            }
        }
        info[8 * 3 + 0] = if geoenable.0 { 1 } else { 0 };

        true
    } else {
        false
    }
    
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_material_info(app: &mut Engine, param: &mut ActionSetScene3D, id: f64, info: &mut [u32]) -> bool {
	pi_export_base::export::await_last_frame(app);
    let id = as_entity(id);
    if let (
        Ok(meta), Ok(textures)
        , slots
    ) = (
        param.get_asset_res_shader_effect_meta(&app.world, id),
        param.get_effect_texture_samplers_comp(&app.world, id),
        param.get_texture_key_list(&app.world, id) 
    ){

        let mut idx = 0;
        if let Ok(slots) = slots {
            slots.0.iter().for_each(|slot| {
                match &slot.url {
                    EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
                    EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
                    EKeyTexture::SRT(key) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = *key as u32; },
                }
                idx += 1;
            })
        } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot02 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot03 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;
        // if let Some(slot) = slot04 {
        //     match &slot.0.url {
        //         EKeyTexture::Tex(key) => { info[idx * 2 + 0] = 1; info[idx * 2 + 1] = key.str_hash() as u32; },
        //         EKeyTexture::Image(key) => { info[idx * 2 + 0] = 2; info[idx * 2 + 1] = key.url().url.str_hash() as u32; },
        //         EKeyTexture::SRT(_) => { info[idx * 2 + 0] = 4; info[idx * 2 + 1] = 0 as u32; },
        //     }
        // } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // idx += 1;

        // if let Some(texs) = &textures.0 {
        //     if let Some(slot) = &texs.textures.0 {
        //         match &slot.0.0.key() {
        //             KeyTextureViewUsage::Tex(_, _) => todo!(),
        //             KeyTextureViewUsage::Image(_, _) => todo!(),
        //             KeyTextureViewUsage::Render(_, _) => todo!(),
        //             KeyTextureViewUsage::SRT(_, _, _) => todo!(),
        //             KeyTextureViewUsage::Temp(_, _) => todo!(),
        //         }
        //     } else { info[idx * 2 + 0] = 0; info[idx * 2 + 1] = 0 as u32; }
        // }

        true
    } else {
        false
    }
}

// pub fn p3d_query_uniforms(app: &mut Engine, param: &mut ActionSetScene3D, material: f64, key: &Atom, info: &mut [f64]) -> bool {
// 	pi_export_base::export::await_last_frame(app);
//     let material = as_entity(material);

//     if let Ok(bindeffect) = param.uniforms.get(&app.world, material) {
//         if let Some(info) = &bindeffect.0 {
//             if let Some(offset) = info.offset(key.deref()) {
//                 if let Some(entity) = offset.entity() {
//                     info[0] = 1.; info[1] = as_f64(&entity);
//                 } else {
//                     info[0] = 0.;
//                 }
//                 true
//             } else {
//                 false
//             }
//         } else {
//             false
//         }
//     } else {
//         false
//     }
// }

// pub fn p3d_query_animator_value(app: &mut Engine, param: &mut ActionSetScene3D, target: f64, info: &mut [f64]) -> bool {
// 	pi_export_base::export::await_last_frame(app);
//     let target = as_entity(target);

//     if let Ok(value) = param.animatorablefloat.get(&app.world, target) {
//        true
//     } else {
//         false
//     }
// }

fn texture_view_usage_info(key: &KeyTextureViewUsage) {
    match key {
        KeyTextureViewUsage::Tex(_, _) => todo!(),
        KeyTextureViewUsage::Image(_, _) => todo!(),
        KeyTextureViewUsage::Render(_, _) => todo!(),
        KeyTextureViewUsage::SRT(_, _, _) => todo!(),
        KeyTextureViewUsage::Temp(_, _) => todo!(),
    }
}
