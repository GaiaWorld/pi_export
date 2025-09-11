use std::ops::DerefMut;
use std::{ops::Deref};

use js_proxy_gen_macro::pi_js_export;
use pi_scene_shell::prelude::*;
use pi_particle_system::{prelude::*};
use pi_scene_context::{prelude::*};
use pi_trail_renderer::TrailBase;
use crate::export::Engine;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(SystemParam)]
pub struct GlobalState<'w> {
    pub resource: Res<'w, pi_3d::StateResource>,
    pub performance: ResMut<'w, pi_scene_shell::prelude::Performance>,
    pub psperformance: ResMut<'w, pi_particle_system::prelude::ParticleSystemPerformance>,
    // pub statemesh: ResMut<'w, pi_scene_context::prelude::StateMesh>,
    pub statetransform: Res<'w, pi_scene_context::prelude::StateTransform>,
    pub statecamera: Res<'w, pi_scene_context::prelude::StateCamera>,
    pub statelight: Res<'w, pi_scene_context::prelude::StateLight>,
    // pub statecamera: ResMut<'w, pi_scene_context::prelude::StateCamera>,
    // pub statematerial: ResMut<'w, pi_scene_context::prelude::StateMaterial>,
    pub statetrail: Res<'w, pi_trail_renderer::StateTrail>,
    pub stateengine: ResMut<'w, pi_scene_shell::run_stage::EngineCustomPlugins>,
}

pub struct _ActionSetScene3D {
    pub acts: SystemState<pi_3d::ActionSets<'static>>,
    pub resource: SystemState<pi_3d::ResourceSets<'static>>,
    pub state: SystemState<GlobalState<'static>>,
    pub tree: SystemState<EntityTree<'static>>,
    pub treedown: QueryState<&'static Down, (With<Enable>)>,
    pub world_transform: QueryState<&'static GlobalMatrix, ()>,
    pub local_transform: QueryState<&'static LocalMatrix, ()>,
    pub view_matrix: QueryState<&'static ViewerViewMatrix, ()>,
    pub project_matrix: QueryState<&'static ViewerProjectionMatrix, ()>,
    pub vp_matrix: QueryState<&'static ViewerTransformMatrix, ()>,
    pub meshes: QueryState<(&'static SceneID, &'static GlobalEnable, Option<&'static RenderGeometryEable>, Option<&'static InstanceMesh>, &'static AbstructMesh), ()>, // StateMeshQuery,
    pub materials: QueryState<(&'static AssetResShaderEffectMeta, &'static EffectTextureSamplersComp, Option<&'static TextureKeyList>), ()>, // StateMaterialQuery,
    pub transforms: QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable), ()>, // StateTransformQuery,
    pub cameras: QueryState<(&'static Camera, &'static ModelList, &'static ModelListAfterCulling), ()>, // StateCameraQuery,
    pub renderers: QueryState<(&'static ViewerID, &'static Renderer), ()>,
    pub viewers: QueryState<(&'static ViewerActive, &'static SceneID), ()>,
    pub particlesystems: QueryState<(&'static ParticleIDs, &'static SceneID), ()>,
    pub animectxs: QueryState<&'static SceneAnimationContext, ()>,
    pub trails: QueryState<(&'static pi_trail_renderer::TrailPoints, &'static SceneID), ()>,
    pub model: QueryState<(&'static RenderGeometryEable, &'static PassIDs), ()>,
    pub pass: QueryState<(&'static PassRendererID, &'static PassMaterialID), ()>,
    pub passactive: QueryState<(&'static PassBindGroups, &'static PassShader, &'static PassDraw), ()>,
    pub nodes: QueryState<(&'static SceneID, &'static Enable, &'static GlobalEnable, &'static Layer), ()>, // StateTransformQuery,
    pub nodesscene: QueryState<(&'static Scene), ()>,
    pub isonode: QueryState<(&'static LocalPosition, &'static LocalEulerAngles, &'static LocalRotationQuaternion), ()>,
    pub scaling: QueryState<(&'static LocalScaling), ()>,
    pub nodestransform: QueryState<(&'static TransformNode), ()>,
    pub nodesinstance: QueryState<(&'static InstanceMesh), ()>,
    pub nodesmesh: QueryState<(&'static Mesh), ()>,
    pub nodescamera: QueryState<(&'static Camera), ()>,
    pub nodesdirectlight: QueryState<(&'static DirectLight), ()>,
    pub nodespointlight: QueryState<(&'static PointLight), ()>,
    pub nodestrail: QueryState<(&'static TrailBase), ()>,
    pub nodesparticlesys: QueryState<(&'static ParticleIDs), ()>,
    pub renders: QueryState<(&'static Renderer, &'static RendererEnable, &'static RendererParam), ()>,
    pub collider: QueryState<(&'static SceneColliderPool, &'static SceneBoundingPool), ()>,
    pub pickitems: QueryState<&'static GlobalEnable, ()>,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub struct ActionSetScene3D(pub(crate) _ActionSetScene3D);
impl Deref for ActionSetScene3D {
    type Target = _ActionSetScene3D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ActionSetScene3D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
impl ActionSetScene3D {
    #[cfg_attr(target_arch="wasm32", wasm_bindgen)]
    #[pi_js_export]
    pub fn create(app: &mut Engine) -> Self {
		crate::export::await_last_frame(app);

        Self(_ActionSetScene3D {
            acts: SystemState::<pi_3d::ActionSets>::new(&mut app.world),
            resource: SystemState::<pi_3d::ResourceSets>::new(&mut app.world),
            state: SystemState::<GlobalState>::new(&mut app.world),
            tree: SystemState::<EntityTree<'static>>::new(&mut app.world),
            treedown: app.world.query(),
            world_transform: app.world.query(),
            local_transform: app.world.query(),
            view_matrix: app.world.query(),
            project_matrix: app.world.query(),
            vp_matrix: app.world.query(),
            meshes: app.world.query(),
            materials: app.world.query(),
            transforms: app.world.query(),
            cameras: app.world.query(),
            renderers: app.world.query(),
            viewers: app.world.query(),
            particlesystems: app.world.query(),
            trails: app.world.query(),
            model: app.world.query(),
            pass: app.world.query(),
            nodes: app.world.query(),
            nodesscene: app.world.query(),
            isonode: app.world.query(),
            scaling: app.world.query(),
            nodestransform: app.world.query(),
            animectxs: app.world.query(),
            passactive: app.world.query(),
            nodesinstance: app.world.query(),
            nodesmesh: app.world.query(),
            nodescamera: app.world.query(),
            nodesdirectlight: app.world.query(),
            nodespointlight: app.world.query(),
            nodestrail: app.world.query(),
            nodesparticlesys: app.world.query(),
            renders: app.world.query(),
            collider: app.world.query(),
            pickitems: app.world.query(),
            
            // uniforms: app.world.query(),
            // animatorablefloat: app.world.query(),
            // animatorablevec2s: app.world.query(),
            // animatorablevec3s: app.world.query(),
            // animatorablevec4s: app.world.query(),
            // animatorableuints: app.world.query(),
            // animatorablesints: app.world.query(),
        })
    }
}