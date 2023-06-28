
use std::mem::replace;
use js_proxy_gen_macro::pi_js_export;
use derive_deref::{Deref, DerefMut};
use std::ops::Deref;

use pi_animation::amount::AnimationAmountCalc;
use pi_curves::{easing::EEasingMode, steps::EStepMode};
use pi_engine_shell::prelude::*;
use pi_export_base::{export::{Engine, Atom}, constants::{RenderFormat, DepthStencilFormat}};
use pi_scene_context::prelude::*;
use pi_slotmap::DefaultKey;

use crate::{engine::ActionSetScene3D, as_entity, as_f64, as_f64_dk, as_dk};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


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
        cmds.animegroupcmd.global.record_group(group_target, key_group, id_group.clone());
        Some(as_f64_dk(&id_group))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_start(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    target: f64,
    entity: f64,
    name: &Atom,
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
    let target: Entity = as_entity(target);

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

    cmds.animegroupcmd.start.push(
        OpsAnimationGroupStart::ops(
            target,
            name.deref().clone(),
            AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as u16, amountcalc)
        )
    )
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
        receive[i + 1] = name as f64;
        receive[i + 2] = tag as f64;
        receive[i + 3] = count as f64;
        index += 1;
    });

    index as f64
}