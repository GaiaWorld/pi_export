
use std::{mem::replace, fmt::Debug, ops::Range};
use js_proxy_gen_macro::pi_js_export;
use derive_deref::{Deref, DerefMut};
use pi_node_materials::prelude::*;
use pi_scene_math::{Vector3, Quaternion};
use std::ops::Deref;

use pi_animation::amount::AnimationAmountCalc;
use pi_curves::{easing::EEasingMode, steps::EStepMode, curve::{frame_curve::{FrameCurve, frames::interplate_frame_values_step}, frame::FrameDataValue}};
use pi_engine_shell::prelude::*;
use pi_export_base::{export::{Engine, Atom}, constants::{RenderFormat, DepthStencilFormat}};
use pi_scene_context::prelude::*;
use pi_slotmap::DefaultKey;

use crate::{engine::ActionSetScene3D, as_entity, as_f64, as_f64_dk, as_dk};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAnimeCurve {
    FrameValues = 0x00,
    FrameValuesStep = 0x01,
    EasingCurve = 0x02,
    MinMaxCurve = 0x03,
    CubicBezierCurve = 0x04,
    GLTFCubicSpline = 0x05,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAnimePropertyType {
    LocalPosition       = 00,
    LocalScaling        = 01,
    LocalRotation       = 02,
    LocalEulerAngles    = 03,
    Alpha               = 04,
    MainColor           = 05,
    MainTexUScale       = 06,
    MainTexVScale       = 07,
    MainTexUOffset      = 08,
    MainTexVOffset      = 09,
    OpacityTexUScale    = 10,
    OpacityTexVScale    = 11,
    OpacityTexUOffset   = 12,
    OpacityTexVOffset   = 13,
    AlphaCutoff         = 14,
    Fov                 = 15,
    OrthSize            = 16,
    LightDiffuse        = 17,
    MaskTexUScale       = 18,
    MaskTexVScale       = 19,
    MaskTexUOffset      = 20,
    MaskTexVOffset      = 21,
    MaskCutoff          = 22,
    IsActive            = 23,
    
    BoneOffset          = 100,
    IndicesRange        = 101,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum ELoopMode {
    /// 不循环
    Not,
    /// 正向循环
    /// * 指定循环次数 - None 无限循环
    Positive,
    /// 反向循环
    /// * 指定循环次数 - None 无限循环
    Opposite,
    /// 正向反复循环
    /// * 指定循环次数 - None 无限循环
    PositivePly,
    /// 反向反复循环
    /// * 指定循环次数 - None 无限循环
    OppositePly,
}
impl ELoopMode {
    pub fn val(&self, val: Option<u32>) -> pi_animation::loop_mode::ELoopMode {
        match self {
            ELoopMode::Not              => pi_animation::loop_mode::ELoopMode::Not              ,
            ELoopMode::Positive         => pi_animation::loop_mode::ELoopMode::Positive(val)    ,
            ELoopMode::Opposite         => pi_animation::loop_mode::ELoopMode::Opposite(val)    ,
            ELoopMode::PositivePly      => pi_animation::loop_mode::ELoopMode::PositivePly(val) ,
            ELoopMode::OppositePly      => pi_animation::loop_mode::ELoopMode::OppositePly(val) ,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAmountMode {
    None            ,

    BackIn          ,
    BackOut         ,
    BackInOut       ,

    CircleIn        ,
    CircleOut       ,
    CircleInOut     ,

    CubicIn         ,
    CubicOut        ,
    CubicInOut      ,

    SineIn          ,
    SineOut         ,
    SineInOut       ,

    QuadIn          ,
    QuadOut         ,
    QuadInOut       ,

    QuartIn         ,
    QuartOut        ,
    QuartInOut      ,

    QuintIn         ,
    QuintOut        ,
    QuintInOut      ,

    ExpoIn          ,
    ExpoOut         ,
    ExpoInOut       ,

    ElasticIn       ,
    ElasticOut      ,
    ElasticInOut    ,

    BounceIn        ,
    BounceOut       ,
    BounceInOut     ,

    JumpStart       ,
    JumpEnd         ,
    JumpNone        ,
    JumpBoth        ,

    CubicBezier     ,
}

fn number_to_easingmode(val: u8) -> pi_curves::easing::EEasingMode {
    match val {
        /*BackIn          = */ 0x01 => {
            pi_curves::easing::EEasingMode::BackIn
        },
        /*BackOut         = */ 0x02 => {
            pi_curves::easing::EEasingMode::BackOut
        },
        /*BackInOut       = */ 0x03 => {
            pi_curves::easing::EEasingMode::BackInOut
        },
        /*CircleIn        = */ 0x04 => {
            pi_curves::easing::EEasingMode::CircleIn
        },
        /*CircleOut       = */ 0x05 => {
            pi_curves::easing::EEasingMode::CircleOut
        },
        /*CircleInOut     = */ 0x06 => {
            pi_curves::easing::EEasingMode::CircleInOut
        },
        /*CubicIn         = */ 0x07 => {
            pi_curves::easing::EEasingMode::CubicIn
        },
        /*CubicOut        = */ 0x08 => {
            pi_curves::easing::EEasingMode::CubicOut
        },
        /*CubicInOut      = */ 0x09 => {
            pi_curves::easing::EEasingMode::CubicInOut
        },
        /*SineIn          = */ 0x11 => {
            pi_curves::easing::EEasingMode::SineIn
        },
        /*SineOut         = */ 0x12 => {
            pi_curves::easing::EEasingMode::SineOut
        },
        /*SineInOut       = */ 0x13 => {
            pi_curves::easing::EEasingMode::SineInOut
        },
        /*QuadIn          = */ 0x14 => {
            pi_curves::easing::EEasingMode::QuadIn
        },
        /*QuadOut         = */ 0x15 => {
            pi_curves::easing::EEasingMode::QuadOut
        },
        /*QuadInOut       = */ 0x16 => {
            pi_curves::easing::EEasingMode::QuadInOut
        },
        /*QuartIn         = */ 0x17 => {
            pi_curves::easing::EEasingMode::QuartIn
        },
        /*QuartOut        = */ 0x18 => {
            pi_curves::easing::EEasingMode::QuartOut
        },
        /*QuartInOut      = */ 0x19 => {
            pi_curves::easing::EEasingMode::QuartInOut
        },
        /*QuintIn         = */ 0x21 => {
            pi_curves::easing::EEasingMode::QuintIn
        },
        /*QuintOut        = */ 0x22 => {
            pi_curves::easing::EEasingMode::QuintOut
        },
        /*QuintInOut      = */ 0x23 => {
            pi_curves::easing::EEasingMode::QuintInOut
        },
        /*ExpoIn          = */ 0x24 => {
            pi_curves::easing::EEasingMode::ExpoIn
        },
        /*ExpoOut         = */ 0x25 => {
            pi_curves::easing::EEasingMode::ExpoOut
        },
        /*ExpoInOut       = */ 0x26 => {
            pi_curves::easing::EEasingMode::ExpoInOut
        },
        /*ElasticIn       = */ 0x27 => {
            pi_curves::easing::EEasingMode::ElasticIn
        },
        /*ElasticOut      = */ 0x28 => {
            pi_curves::easing::EEasingMode::ElasticOut
        },
        /*ElasticInOut    = */ 0x29 => {
            pi_curves::easing::EEasingMode::ElasticInOut
        },
        /*BounceIn        = */ 0x31 => {
            pi_curves::easing::EEasingMode::BounceIn
        },
        /*BounceOut       = */ 0x32 => {
            pi_curves::easing::EEasingMode::BounceOut
        },
        /*BounceInOut     = */ 0x33 => {
            pi_curves::easing::EEasingMode::BounceInOut
        },
        /*None            = */ _ => {
            pi_curves::easing::EEasingMode::None
        },
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub enum EAnimationGroupListen {
    Start,
    End,
    Loop,
    Frame,
}
impl EAnimationGroupListen {
    pub fn val(&self) -> u8 {
        match self {
            EAnimationGroupListen::Start    => TagGroupListen::START,
            EAnimationGroupListen::End      => TagGroupListen::END  ,
            EAnimationGroupListen::Loop     => TagGroupListen::LOOP ,
            EAnimationGroupListen::Frame    => TagGroupListen::FRAME,
        }
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_group(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_target: f64,
    key_group: &Atom,
) -> Option<f64> {
    let scene: Entity = as_entity(scene);
    let group_target: Entity = as_entity(group_target);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    if let Some(id_group) = cmds.animegroupcmd.scene_ctxs.create_group(scene) {
        cmds.animegroupcmd.global.record_group(group_target, id_group.clone());
        Some(as_f64_dk(&id_group))
    } else {
        None
    }
}


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 设置 动画组 混合权重
pub fn p3d_animation_group_weight(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
    weight: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.animegroupcmd.scene_ctxs.group_weight(scene, group_key, weight as f32);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 重置动画组各动画作用目标到非动画操作状态
pub fn p3d_animation_group_target_reset(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.animegroupcmd.reset_while_start.push(OpsAnimationGroupStartReset::ops(scene, group_key));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_start(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
    speed: f64,
    loop_mode: ELoopMode,
    loop_count: Option<f64>,
    from: f64,
    to: f64,
    fps: f64,
    amount_mode: EAmountMode,
    amount_param0: f64,
    amount_param1: f64,
    amount_param2: f64,
    amount_param3: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let amountcalc = match amount_mode {
        EAmountMode::None           => AnimationAmountCalc::default(),
        EAmountMode::BackIn         => AnimationAmountCalc::from_easing(EEasingMode::BackIn)    ,
        EAmountMode::BackOut        => AnimationAmountCalc::from_easing(EEasingMode::BackOut)    ,
        EAmountMode::BackInOut      => AnimationAmountCalc::from_easing(EEasingMode::BackInOut)    ,
        EAmountMode::CircleIn       => AnimationAmountCalc::from_easing(EEasingMode::CircleIn)    ,
        EAmountMode::CircleOut      => AnimationAmountCalc::from_easing(EEasingMode::CircleOut)    ,
        EAmountMode::CircleInOut    => AnimationAmountCalc::from_easing(EEasingMode::CircleInOut)    ,
        EAmountMode::CubicIn        => AnimationAmountCalc::from_easing(EEasingMode::CubicIn)    ,
        EAmountMode::CubicOut       => AnimationAmountCalc::from_easing(EEasingMode::CubicOut)    ,
        EAmountMode::CubicInOut     => AnimationAmountCalc::from_easing(EEasingMode::CubicInOut)    ,
        EAmountMode::SineIn         => AnimationAmountCalc::from_easing(EEasingMode::SineIn)    ,
        EAmountMode::SineOut        => AnimationAmountCalc::from_easing(EEasingMode::SineOut)    ,
        EAmountMode::SineInOut      => AnimationAmountCalc::from_easing(EEasingMode::SineInOut)    ,
        EAmountMode::QuadIn         => AnimationAmountCalc::from_easing(EEasingMode::QuadIn)    ,
        EAmountMode::QuadOut        => AnimationAmountCalc::from_easing(EEasingMode::QuadOut)    ,
        EAmountMode::QuadInOut      => AnimationAmountCalc::from_easing(EEasingMode::QuadInOut)    ,
        EAmountMode::QuartIn        => AnimationAmountCalc::from_easing(EEasingMode::QuartIn)    ,
        EAmountMode::QuartOut       => AnimationAmountCalc::from_easing(EEasingMode::QuartOut)    ,
        EAmountMode::QuartInOut     => AnimationAmountCalc::from_easing(EEasingMode::QuartInOut)    ,
        EAmountMode::QuintIn        => AnimationAmountCalc::from_easing(EEasingMode::QuintIn)    ,
        EAmountMode::QuintOut       => AnimationAmountCalc::from_easing(EEasingMode::QuintOut)    ,
        EAmountMode::QuintInOut     => AnimationAmountCalc::from_easing(EEasingMode::QuintInOut)    ,
        EAmountMode::ExpoIn         => AnimationAmountCalc::from_easing(EEasingMode::ExpoIn)    ,
        EAmountMode::ExpoOut        => AnimationAmountCalc::from_easing(EEasingMode::ExpoOut)    ,
        EAmountMode::ExpoInOut      => AnimationAmountCalc::from_easing(EEasingMode::ExpoInOut)    ,
        EAmountMode::ElasticIn      => AnimationAmountCalc::from_easing(EEasingMode::ElasticIn)    ,
        EAmountMode::ElasticOut     => AnimationAmountCalc::from_easing(EEasingMode::ElasticOut)    ,
        EAmountMode::ElasticInOut   => AnimationAmountCalc::from_easing(EEasingMode::ElasticInOut)    ,
        EAmountMode::BounceIn       => AnimationAmountCalc::from_easing(EEasingMode::BounceIn)    ,
        EAmountMode::BounceOut      => AnimationAmountCalc::from_easing(EEasingMode::BounceOut)    ,
        EAmountMode::BounceInOut    => AnimationAmountCalc::from_easing(EEasingMode::BounceInOut)    ,
        EAmountMode::JumpStart      => AnimationAmountCalc::from_steps(amount_param0 as u16, EStepMode::JumpStart    ),
        EAmountMode::JumpEnd        => AnimationAmountCalc::from_steps(amount_param0 as u16, EStepMode::JumpEnd    ),
        EAmountMode::JumpNone       => AnimationAmountCalc::from_steps(amount_param0 as u16, EStepMode::JumpNone    ),
        EAmountMode::JumpBoth       => AnimationAmountCalc::from_steps(amount_param0 as u16, EStepMode::JumpBoth    ),
        EAmountMode::CubicBezier    => AnimationAmountCalc::from_cubic_bezier(amount_param0 as f32, amount_param1 as f32, amount_param2 as f32, amount_param3 as f32),
    };

    let loop_count = if let Some(loop_count) = loop_count {
        Some(loop_count as u32)
    } else {
        None
    };

    let loop_mode = loop_mode.val(loop_count);

    cmds.animegroupcmd.scene_ctxs.start_with_progress(scene, group_key, AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as u16, amountcalc));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_pause(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.animegroupcmd.scene_ctxs.pause(scene, group_key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_stop(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    cmds.animegroupcmd.scene_ctxs.stop(scene, group_key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_listen(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    group: f64,
    mode: EAnimationGroupListen,
) {
    let id_group: DefaultKey = as_dk(&group);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    match mode {
        EAnimationGroupListen::Start    => {
            cmds.animegroupcmd.global.add_start_listen(id_group);
        },
        EAnimationGroupListen::End      => {
            cmds.animegroupcmd.global.add_end_listen(id_group);
        },
        EAnimationGroupListen::Loop     => {
            cmds.animegroupcmd.global.add_loop_listen(id_group);
        },
        EAnimationGroupListen::Frame    => {
            cmds.animegroupcmd.global.add_frame_event_listen(id_group);
        },
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_add_frame_event(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    group: f64,
    percent: f64,
    data: f64
) {
    let id_group: DefaultKey = as_dk(&group);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.animegroupcmd.global.add_frame_event(id_group, percent as f32, data as AnimeFrameEventData);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_delete(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group: f64,
) {
    let id_scene: Entity = as_entity(scene);
    let id_group: DefaultKey = as_dk(&group);

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);
    cmds.animegroupcmd.global.remove(&id_group);
    cmds.animegroupcmd.scene_ctxs.delete_group(&id_scene, id_group);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_anime_events(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    receive: &mut [f64],
    receive_len: f64,
) -> f64 {

    let mut cmds: crate::engine::ActionSets = param.acts.get_mut(&mut app.world);

    let size = receive_len as usize / 4;

    let count = cmds.animegroupcmd.events.len();
    let mut oldlist = replace(&mut cmds.animegroupcmd.events.0, vec![]);
    if count > size {
        let mut keeplist = vec![];
        let keepcount = count - size;
        for _ in 0..keepcount {
            keeplist.push(oldlist.pop().unwrap());
        }

        while let Some(item) = keeplist.pop() {
            cmds.animegroupcmd.events.push(item);
        };
    };

    let mut index = 0;
    oldlist.drain(..).for_each(|(target, name, tag, count)| {
        let i = index * 4;
        receive[i + 0] = as_f64(&target);
        receive[i + 1] = as_f64_dk(&name);
        receive[i + 2] = tag as f64;
        receive[i + 3] = count as f64;
        index += 1;
    });

    index as f64
}


pub trait TValue<const N: usize> {
    fn newn(data: &[f32], offset: usize) -> Self;
}
impl TValue<3> for LocalScaling {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<3> for LocalEulerAngles {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<3> for LocalPosition {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<4> for LocalRotationQuaternion {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32; let w = data[offset + 3] as f32;
        Self::create(x, y, z, w)
    }
}
impl TValue<1> for Alpha {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for Cutoff {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<3> for MainColor {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}
impl TValue<1> for MainTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MainTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for MaskTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for MaskTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for OpacityTexUScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexVScale {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexUOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}
impl TValue<1> for OpacityTexVOffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for Enable {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for CameraFov {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for CameraOrthSize {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<1> for MaskCutoff {
    fn newn(data: &[f32], offset: usize) -> Self {
        Self(data[offset])
    }
}

impl TValue<3> for LightDiffuse {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as f32; let y = data[offset + 1] as f32; let z = data[offset + 2] as f32;
        Self(Vector3::new(x, y, z))
    }
}

impl TValue<1> for InstanceBoneoffset {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as u32;
        Self(x)
    }
}

impl TValue<2> for IndiceRenderRange {
    fn newn(data: &[f32], offset: usize) -> Self {
        let x = data[offset + 0] as u32; let y = data[offset + 1] as u32;
        Self(Some(Range { start: x, end: y, }))
    }
}

fn curve<const N: usize, T: TValue<N> + FrameDataValue + Debug>(
    data: &[f32],
    mode: EAnimeCurve,
) -> FrameCurve<T> {
    let vs = N; let vs2 = N * 2; let vs3 = N * 3;
    let design_frame_per_second = data[0] as u16;
    let curve = match mode {
        EAnimeCurve::FrameValues => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = 1 + vs;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as u16;
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));
                curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
            }
            curve
        },
        EAnimeCurve::FrameValuesStep => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = 1 + vs;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as u16;
                curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
            }
            curve.call = interplate_frame_values_step;
            curve
        },
        EAnimeCurve::EasingCurve => {
            let design_frame_per_second = data[0] as u16;
            let frame_count = data[1] as u16;
            let mode = number_to_easingmode(data[2] as u8);
            let head = 3;
            let from = T::newn(data, head + 0);
            let scalar = T::newn(data, head + vs);
            let curve = FrameCurve::<T>::curve_easing(
                from,
                scalar,
                frame_count,
                design_frame_per_second, mode
            );
            curve
        },
        EAnimeCurve::MinMaxCurve => {
            let design_frame_per_second = data[0] as u16;
            let from = T::newn(data, 1);
            let to = T::newn(data, 1 + vs);
            let head = 1 + vs2;
            let mut curve = FrameCurve::<T>::curve_minmax_curve(from, to, design_frame_per_second);
            let step = 4;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as u16;
                let intangent  = data[index + 1] as f32;
                let value = data[index + 2] as f32;
                let outtangent = data[index + 3] as f32;
                curve.curve_minmax_curve_frame(frame, value, intangent, outtangent);
            }
            curve
        },
        EAnimeCurve::CubicBezierCurve => {
            let design_frame_per_second = data[0] as u16;
            let frame_count = data[1] as u16;
            let mut head = 2;
            let from = T::newn(data, head);
            let scalar = T::newn(data, head + vs);
            head = head + vs2;
            let x1 = data[head] as f32; let y1 = data[head + 1] as f32; let x2 = data[head + 2] as f32; let y2 = data[head + 3] as f32; 
            let curve = FrameCurve::<T>::curve_cubic_bezier(
                from,
                scalar,
                frame_count,
                design_frame_per_second,
                x1 as f32, y1 as f32, x2 as f32, y2 as f32
            );
            curve
        },
        EAnimeCurve::GLTFCubicSpline => {
            let design_frame_per_second = data[0] as u16;
            let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
            let head = 1;
            let step = 1 + vs3;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as u16;
                let intangent = T::newn(data, index + 1);
                let value = T::newn(data, index + 1 + vs);
                let outtangent = T::newn(data, index + 1 + vs2);
                curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
            }
            curve
        },
    };
    curve
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// FrameCurve
/// * `FrameValues` data: [design_frame_per_second, (frame, x, y, ..), (frame, x, y, ..) ...]
/// * `FrameValuesStep` data: [design_frame_per_second, (frame, x, y, ..), (frame, x, y, ..) ...]
/// * `EasingCurve` data: [design_frame_per_second, total_frame, mode, (x, y, ..), (x, y, ..)]
/// * `MinMaxCurve` data: [design_frame_per_second, (x, y, ..), (x, y, ..), (frame, f32, it, ot), (frame, f32, it, ot) ...]
/// * `CubicBezierCurve` data: [design_frame_per_second, total_frame, (x, y, ..), (x, y, ..), (x1, y1, x2, y2)]
/// * `GLTFCubicSpline` data: [design_frame_per_second, (frame, (x, y, ..), (x, y, ..), (x, y, ..)), ...]
pub fn p3d_anime_curve_query(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyType) -> bool {
    let mut cmds = param.acts.get_mut(&mut app.world);
    let key = key as IDAssetTypeFrameCurve;
    match property {
        EAnimePropertyType::LocalPosition       => cmds.transformanime.position.curves.get(&key).is_some(),
        EAnimePropertyType::LocalScaling        => cmds.transformanime.scaling.curves.get(&key).is_some(),
        EAnimePropertyType::LocalRotation       => cmds.transformanime.quaternion.curves.get(&key).is_some(),
        EAnimePropertyType::LocalEulerAngles    => cmds.transformanime.euler.curves.get(&key).is_some(),
        EAnimePropertyType::Alpha               => cmds.anime_alpha.1.get(&key).is_some(),
        EAnimePropertyType::MainColor           => cmds.anime_maincolor.1.get(&key).is_some(),
        EAnimePropertyType::MainTexUScale       => cmds.anime_maintex_uscale.1.get(&key).is_some(),
        EAnimePropertyType::MainTexVScale       => cmds.anime_maintex_vscale.1.get(&key).is_some(),
        EAnimePropertyType::MainTexUOffset      => cmds.anime_maintex_uoffset.1.get(&key).is_some(),
        EAnimePropertyType::MainTexVOffset      => cmds.anime_maintex_voffset.1.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUScale    => cmds.anime_opacitytex_uscale.1.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVScale    => cmds.anime_opacitytex_vscale.1.get(&key).is_some(),
        EAnimePropertyType::OpacityTexUOffset   => cmds.anime_opacitytex_uoffset.1.get(&key).is_some(),
        EAnimePropertyType::OpacityTexVOffset   => cmds.anime_opacitytex_voffset.1.get(&key).is_some(),
        EAnimePropertyType::AlphaCutoff         => cmds.anime_alphacutoff.1.get(&key).is_some(),
        EAnimePropertyType::Fov                 => cmds.anime_camerafov.1.get(&key).is_some(),
        EAnimePropertyType::OrthSize            => cmds.anime_camerasize.1.get(&key).is_some(),
        EAnimePropertyType::LightDiffuse        => cmds.anime_lightdiffuse.1.get(&key).is_some(),
        EAnimePropertyType::MaskTexUScale       => cmds.anime_masktex_uscale.1.get(&key).is_some(),
        EAnimePropertyType::MaskTexVScale       => cmds.anime_masktex_vscale.1.get(&key).is_some(),
        EAnimePropertyType::MaskTexUOffset      => cmds.anime_masktex_uoffset.1.get(&key).is_some(),
        EAnimePropertyType::MaskTexVOffset      => cmds.anime_masktex_voffset.1.get(&key).is_some(),
        EAnimePropertyType::MaskCutoff          => cmds.anime_maskcutoff.1.get(&key).is_some(),
        EAnimePropertyType::IsActive            => cmds.anime_isactive.1.get(&key).is_some(),
        EAnimePropertyType::BoneOffset          => cmds.anime_boneoffset.1.get(&key).is_some(),
        EAnimePropertyType::IndicesRange        => cmds.anime_indices_range.1.get(&key).is_some(),
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_curve_create(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyType, data: &[f32], mode: EAnimeCurve) {
    let mut cmds = param.acts.get_mut(&mut app.world);
    let key = key as IDAssetTypeFrameCurve;
    match property {
        EAnimePropertyType::LocalPosition       => {
            let v = curve::<3, LocalPosition>(data, mode);
            cmds.transformanime.position.curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalScaling        => {
            let v = curve::<3, LocalScaling>(data, mode);
            cmds.transformanime.scaling.curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalRotation    => {
            let v = curve::<4, LocalRotationQuaternion>(data, mode);
            cmds.transformanime.quaternion.curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LocalEulerAngles    => {
            let v = curve::<3, LocalEulerAngles>(data, mode);
            cmds.transformanime.euler.curves.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::Alpha               => {
            let v = curve::<1, Alpha>(data, mode);
            cmds.anime_alpha.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainColor           => {
            let v = curve::<3, MainColor>(data, mode);
            cmds.anime_maincolor.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexUScale       => {
            let v = curve::<1, MainTexUScale>(data, mode);
            cmds.anime_maintex_uscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexVScale       => {
            let v = curve::<1, MainTexVScale>(data, mode);
            cmds.anime_maintex_vscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexUOffset      => {
            let v = curve::<1, MainTexUOffset>(data, mode);
            cmds.anime_maintex_uoffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MainTexVOffset      => {
            let v = curve::<1, MainTexVOffset>(data, mode);
            cmds.anime_maintex_voffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexUScale    => {
            let v = curve::<1, OpacityTexUScale>(data, mode);
            cmds.anime_opacitytex_uscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexVScale    => {
            let v = curve::<1, OpacityTexVScale>(data, mode);
            cmds.anime_opacitytex_vscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexUOffset   => {
            let v = curve::<1, OpacityTexUOffset>(data, mode);
            cmds.anime_opacitytex_uoffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OpacityTexVOffset   => {
            let v = curve::<1, OpacityTexVOffset>(data, mode);
            cmds.anime_opacitytex_voffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::AlphaCutoff         => {
            let v = curve::<1, Cutoff>(data, mode);
            cmds.anime_alphacutoff.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::Fov                 => {
            let v = curve::<1, CameraFov>(data, mode);
            cmds.anime_camerafov.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::OrthSize            => {
            let v = curve::<1, CameraOrthSize>(data, mode);
            cmds.anime_camerasize.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::LightDiffuse        => {
            let v = curve::<3, LightDiffuse>(data, mode);
            cmds.anime_lightdiffuse.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexUScale       => {
            let v = curve::<1, MaskTexUScale>(data, mode);
            cmds.anime_masktex_uscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexVScale       => {
            let v = curve::<1, MaskTexVScale>(data, mode);
            cmds.anime_masktex_vscale.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexUOffset      => {
            let v = curve::<1, MaskTexUOffset>(data, mode);
            cmds.anime_masktex_uoffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskTexVOffset      => {
            let v = curve::<1, MaskTexVOffset>(data, mode);
            cmds.anime_masktex_voffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::MaskCutoff          => {
            let v = curve::<1, MaskCutoff>(data, mode);
            cmds.anime_maskcutoff.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::IsActive            => {
            let v = curve::<1, Enable>(data, mode);
            cmds.anime_isactive.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::BoneOffset          => {
            let v = curve::<1, InstanceBoneoffset>(data, mode);
            cmds.anime_boneoffset.1.insert(key, TypeFrameCurve(v));
        },
        EAnimePropertyType::IndicesRange        => {
            let v = curve::<2, IndiceRenderRange>(data, mode);
            cmds.anime_indices_range.1.insert(key, TypeFrameCurve(v));
        },
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_target_animation(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    curve_key: f64,
    property: EAnimePropertyType,
    id_scene: f64,
    group: f64,
    curve_target: f64,
) {
    let id_scene = as_entity(id_scene);
    let group = as_dk(&group);
    let anime_target = as_entity(curve_target);

    let mut cmds = param.acts.get_mut(&mut app.world);
    let key = curve_key as IDAssetTypeFrameCurve;

    let info = match property {
        EAnimePropertyType::LocalPosition => {
            if let Some(curve) = cmds.transformanime.position.curves.get(&key) {
                // log::warn!("Curve Ok!");
                cmds.transformanime.position.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::LocalScaling =>  {
            if let Some(curve) = cmds.transformanime.scaling.curves.get(&key) {
                cmds.transformanime.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::LocalRotation =>  {
            if let Some(curve) = cmds.transformanime.quaternion.curves.get(&key) {
                cmds.transformanime.quaternion.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::LocalEulerAngles =>  {
            if let Some(curve) = cmds.transformanime.euler.curves.get(&key) {
                cmds.transformanime.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::Alpha =>  {
            if let Some(curve) = cmds.anime_alpha.1.get(&key) {
                cmds.anime_alpha.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MainColor =>  {
            if let Some(curve) = cmds.anime_maincolor.1.get(&key) {
                cmds.anime_maincolor.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MainTexUScale =>  {
            if let Some(curve) = cmds.anime_maintex_uscale.1.get(&key) {
                cmds.anime_maintex_uscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MainTexVScale =>  {
            if let Some(curve) = cmds.anime_maintex_vscale.1.get(&key) {
                cmds.anime_maintex_vscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MainTexUOffset =>  {
            if let Some(curve) = cmds.anime_maintex_uoffset.1.get(&key) {
                cmds.anime_maintex_uoffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MainTexVOffset =>  {
            if let Some(curve) = cmds.anime_maintex_voffset.1.get(&key) {
                cmds.anime_maintex_voffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::OpacityTexUScale =>  {
            if let Some(curve) = cmds.anime_opacitytex_uscale.1.get(&key) {
                cmds.anime_opacitytex_uscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::OpacityTexVScale =>  {
            if let Some(curve) = cmds.anime_opacitytex_vscale.1.get(&key) {
                cmds.anime_opacitytex_vscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::OpacityTexUOffset =>  {
            if let Some(curve) = cmds.anime_opacitytex_uoffset.1.get(&key) {
                cmds.anime_opacitytex_uoffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::OpacityTexVOffset =>  {
            if let Some(curve) = cmds.anime_opacitytex_voffset.1.get(&key) {
                cmds.anime_opacitytex_voffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::AlphaCutoff =>  {
            if let Some(curve) = cmds.anime_alphacutoff.1.get(&key) {
                cmds.anime_alphacutoff.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::Fov =>  {
            if let Some(curve) = cmds.anime_alpha.1.get(&key) {
                cmds.anime_alpha.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::OrthSize =>  {
            if let Some(curve) = cmds.anime_alpha.1.get(&key) {
                cmds.anime_alpha.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::LightDiffuse =>  {
            if let Some(curve) = cmds.anime_lightdiffuse.1.get(&key) {
                cmds.anime_lightdiffuse.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MaskTexUScale =>  {
            if let Some(curve) = cmds.anime_masktex_uscale.1.get(&key) {
                cmds.anime_masktex_uscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MaskTexVScale =>  {
            if let Some(curve) = cmds.anime_masktex_vscale.1.get(&key) {
                cmds.anime_masktex_vscale.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MaskTexUOffset =>  {
            if let Some(curve) = cmds.anime_masktex_uoffset.1.get(&key) {
                cmds.anime_masktex_uoffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MaskTexVOffset =>  {
            if let Some(curve) = cmds.anime_masktex_voffset.1.get(&key) {
                cmds.anime_masktex_voffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::MaskCutoff =>  {
            if let Some(curve) = cmds.anime_maskcutoff.1.get(&key) {
                cmds.anime_maskcutoff.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::IsActive =>  {
            if let Some(curve) = cmds.anime_isactive.1.get(&key) {
                cmds.anime_isactive.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::BoneOffset =>  {
            if let Some(curve) = cmds.anime_boneoffset.1.get(&key) {
                cmds.anime_boneoffset.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
        EAnimePropertyType::IndicesRange =>  {
            if let Some(curve) = cmds.anime_indices_range.1.get(&key) {
                cmds.anime_indices_range.0.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return; }
        },
    };

    cmds.animegroupcmd.scene_ctxs.add_target_anime(id_scene, anime_target, group, info);
}