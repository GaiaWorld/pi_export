
use std::{mem::{replace, transmute}, fmt::Debug};
use js_proxy_gen_macro::pi_js_export;
use pi_gltf2_load::TValue;
pub use pi_export_base::export::Atom;

use pi_animation::amount::AnimationAmountCalc;
use pi_curves::{curve::{curves::curve_frame_index, frame::{CurveFrameValue, FrameDataValue, KeyFrameCurveValue}, frame_curve::{frames::interplate_frame_values_step, FrameCurve}, FrameIndex, FramePerSecond}, easing::EEasingMode, steps::EStepMode};
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;
use pi_slotmap::DefaultKey;
pub use crate::engine::ActionSetScene3D;
use crate::{as_entity, as_f64, as_f64_dk};
pub use crate::commands::CommandsExchangeD3;


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
pub enum EAnimePropertyID {
    LocalPosition       =  0,
    LocalRotation       =  1,
    LocalScaling        =  2, 
    MainTexUScale       =  3, 
    MainTexVScale       =  4,
    MainTexUOffset      =  5, 
    MainTexVOffset      =  6,
    Alpha               =  7, 
    MainColor           =  8, 
    CameraOrthSize      =  9, 
    CameraFov           = 10,
    Enable              = 11,
    LocalEulerAngles    = 12,
    Intensity           = 13,
    LightDiffuse        = 14,
    AlphaCutoff         = 15,
    CellId              = 16,
    OpacityTexUScale    = 17,
    OpacityTexVScale    = 18,
    OpacityTexUOffset   = 19,
    OpacityTexVOffset   = 20,
    MaskCutoff          = 21,
    MaskTexUScale       = 22,
    MaskTexVScale       = 23,
    MaskTexUOffset      = 24,
    MaskTexVOffset      = 25,
    
    MainTexTilloff      = 50,
    MaskTexTilloff      = 51,
    OpacityTexTilloff   = 52,

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
pub enum EFillMode {
    None = 0,
    Forwards = 1,
    Backwards = 2,
    Both = 3,
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_group(
    app: &mut Engine,
    cmds: &mut CommandsExchangeD3,
    scene: f64,
) -> f64 {
	pi_export_base::export::await_last_frame(app);
    let id: Entity = app.world.spawn_empty();
    let scene: Entity = as_entity(scene);

    cmds.anime_create.push(OpsAnimationGroupCreation::ops(scene, id));

    as_f64(&id)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 动画曲线ID
pub fn p3d_animation_curve_id(
    key: &Atom,
) -> f64 {
    let key = pi_atom::Atom::from(key.as_str());
    let key = key.asset_u64();
    unsafe { transmute(key) }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 设置 动画组 混合权重
pub fn p3d_animation_group_weight(
    cmds: &mut CommandsExchangeD3,
    group: f64,
    weight: f64,
) {
    let group = as_entity(group);

    cmds.anime_weight.push(OpsAnimationWeight::ops(group, weight as f32));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 重置动画组各动画作用目标到非动画操作状态
pub fn p3d_animation_group_target_reset(cmds: &mut CommandsExchangeD3, group: f64,) {
    let group = as_entity(group);

    cmds.anime_reset_while_start.push(OpsAnimationGroupStartReset::ops(group));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_start(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
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
    let group_key = as_entity(group_key);

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
        EAmountMode::JumpStart      => AnimationAmountCalc::from_steps(amount_param0 as FrameIndex, EStepMode::JumpStart    ),
        EAmountMode::JumpEnd        => AnimationAmountCalc::from_steps(amount_param0 as FrameIndex, EStepMode::JumpEnd    ),
        EAmountMode::JumpNone       => AnimationAmountCalc::from_steps(amount_param0 as FrameIndex, EStepMode::JumpNone    ),
        EAmountMode::JumpBoth       => AnimationAmountCalc::from_steps(amount_param0 as FrameIndex, EStepMode::JumpBoth    ),
        EAmountMode::CubicBezier    => AnimationAmountCalc::from_cubic_bezier(amount_param0 as f32, amount_param1 as f32, amount_param2 as f32, amount_param3 as f32),
    };

    let loop_count = if let Some(loop_count) = loop_count {
        Some(loop_count as u32)
    } else {
        None
    };

    let loop_mode = loop_mode.val(loop_count);

    let fillmode = unsafe { transmute(fillmode) };

    cmds.anime_action.push(OpsAnimationGroupAction::Start(group_key, AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as FramePerSecond, amountcalc), delay_ms as KeyFrameCurveValue, fillmode));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_pause(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
) {
    let group_key = as_entity(group_key);

    cmds.anime_action.push(OpsAnimationGroupAction::Pause(group_key));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_stop(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
) {
    let group_key = as_entity(group_key);

    cmds.anime_action.push(OpsAnimationGroupAction::Stop(group_key));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_listen(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
    mode: EAnimationGroupListen,
) {
    let group_key = as_entity(group_key);

    match mode {
        EAnimationGroupListen::Start    => {
            cmds.anime_listen.push(OpsAddAnimationListen::Start(group_key));
        },
        EAnimationGroupListen::End      => {
            cmds.anime_listen.push(OpsAddAnimationListen::End(group_key));
        },
        EAnimationGroupListen::Loop     => {
            cmds.anime_listen.push(OpsAddAnimationListen::Loop(group_key));
        },
        EAnimationGroupListen::Frame    => {
            cmds.anime_listen.push(OpsAddAnimationListen::End(group_key));
        },
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_add_frame_event(
    cmds: &mut CommandsExchangeD3,
    group: f64,
    percent: f64,
    data: f64
) {
    let group = as_entity(group);

    cmds.anime_frameevent.push(OpsAddAnimationFrameEvent::ops(group, percent as f32, data as AnimeFrameEventData));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_delete(
    cmds: &mut CommandsExchangeD3,
    group: f64,
) {
    let id_group = as_entity(group);
    
    cmds.anime_dispose.push(OpsAnimationGroupDispose::ops(id_group));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_anime_events(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    receive: &mut [f64],
    receive_len: f64,
) -> f64 {

	pi_export_base::export::await_last_frame(app);
    let mut resource = param.resource.get_mut(&mut app.world);

    let size = receive_len as usize / 4;

    let count = resource.anime_events.len();
    let mut oldlist = replace(&mut resource.anime_events.0, vec![]);
    if count > size {
        let mut keeplist = vec![];
        let keepcount = count - size;
        for _ in 0..keepcount {
            keeplist.push(oldlist.pop().unwrap());
        }

        while let Some(item) = keeplist.pop() {
            resource.anime_events.push(item);
        };
    };

    let mut index = 0;
    oldlist.drain(..).for_each(|(target, name, tag, count)| {
        let i = index * 4;
        receive[i + 0] = as_f64(&target);
        receive[i + 1] = as_f64(&name);
        receive[i + 2] = tag as f64;
        receive[i + 3] = count as f64;
        index += 1;
    });

    index as f64
}

// fn curve_create(
//     data: &[f32],
//     mode: EAnimeCurve,
//     n: usize,
//     create: &Fn(&[f32], usize) -> FrameDataValue,
// ) -> FrameDataValue {
//     let vs = N; let vs2 = N * 2; let vs3 = N * 3;
//     let design_frame_per_second = data[0] as FramePerSecond;

//     let mut minidx = 0;
//     let mut maxidx = 0;

//     let mut curve = match mode {
//         EAnimeCurve::FrameValues => {
//             let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
//             let head = 1;
//             let step = 1 + vs;
//             let frames = (data.len() - head) / step;
//             for i in 0..frames {
//                 let index = head + i * step;
//                 let frame = data[index + 0] as FrameIndex;
//                 // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));

//                 // curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
//                 let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
//                 curve.values.insert(index, T::newn(data, index + 1));
//                 minidx = min; maxidx = max;
//             }
//             curve
//         },
//         EAnimeCurve::FrameValuesStep => {
//             let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
//             let head = 1;
//             let step = 1 + vs;
//             let frames = (data.len() - head) / step;
//             for i in 0..frames {
//                 let index = head + i * step;
//                 let frame = data[index + 0] as FrameIndex;

//                 // curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
//                 let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
//                 curve.values.insert(index, T::newn(data, index + 1));
//                 minidx = min; maxidx = max;
//             }
//             curve.call = interplate_frame_values_step;
//             curve
//         },
//         EAnimeCurve::EasingCurve => {
//             let frame_count = data[1] as FrameIndex;
//             let mode = number_to_easingmode(data[2] as u8);
//             let head = 3;
//             let from = T::newn(data, head + 0);
//             let scalar = T::newn(data, head + vs);
//             let curve = FrameCurve::<T>::curve_easing(
//                 from,
//                 scalar,
//                 frame_count,
//                 design_frame_per_second, mode
//             );
//             curve
//         },
//         EAnimeCurve::MinMaxCurve => {
//             let from = T::newn(data, 1);
//             let to = T::newn(data, 1 + vs);
//             let head = 1 + vs2;
//             let mut curve = FrameCurve::<T>::curve_minmax_curve(from, to, design_frame_per_second);
//             let step = 4;
//             let frames = (data.len() - head) / step;
//             for i in 0..frames {
//                 let index = head + i * step;
//                 let frame = data[index + 0] as FrameIndex;
//                 let intangent  = data[index + 1] as f32;
//                 let value = data[index + 2] as f32;
//                 let outtangent = data[index + 3] as f32;

//                 // curve.curve_minmax_curve_frame(frame, value, intangent, outtangent);
//                 let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
//                 let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
//                 curve.minmax_curve_values.insert(index, keyframe);
//                 minidx = min; maxidx = max;
//             }
//             curve
//         },
//         EAnimeCurve::CubicBezierCurve => {
//             let frame_count = data[1] as FrameIndex;
//             let mut head = 2;
//             let from = T::newn(data, head);
//             let scalar = T::newn(data, head + vs);
//             head = head + vs2;
//             let x1 = data[head] as f32; let y1 = data[head + 1] as f32; let x2 = data[head + 2] as f32; let y2 = data[head + 3] as f32; 
//             let curve = FrameCurve::<T>::curve_cubic_bezier(
//                 from,
//                 scalar,
//                 frame_count,
//                 design_frame_per_second,
//                 x1 as f32, y1 as f32, x2 as f32, y2 as f32
//             );
//             curve
//         },
//         EAnimeCurve::GLTFCubicSpline => {
//             let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
//             let head = 1;
//             let step = 1 + vs3;
//             let frames = (data.len() - head) / step;
//             for i in 0..frames {
//                 let index = head + i * step;
//                 let frame = data[index + 0] as FrameIndex;
//                 let intangent = T::newn(data, index + 1);
//                 let value = T::newn(data, index + 1 + vs);
//                 let outtangent = T::newn(data, index + 1 + vs2);

//                 // curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
//                 let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
//                 let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
//                 curve.cubic_spline_values.insert(index, keyframe);
//                 minidx = min; maxidx = max;
//             }
//             curve
//         },
//     };

//     curve.min_frame = minidx;
//     curve.max_frame = maxidx;
//     curve.frame_number = maxidx - minidx;
//     curve
// }

fn curve<const N: usize, T: TValue<N> + FrameDataValue>(
    data: &[f32],
    mode: EAnimeCurve,
) -> FrameCurve<T> {
    let vs = N; let vs2 = N * 2; let vs3 = N * 3;
    let design_frame_per_second = data[0] as FramePerSecond;

    let mut minidx = 0;
    let mut maxidx = 0;

    let mut curve = match mode {
        EAnimeCurve::FrameValues => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = 1 + vs;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as FrameIndex;
                // log::warn!("Frame {:?}, data: {:?}", frame, T::newn(data, index + 1));

                // curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
                let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
                curve.values.insert(index, T::newn(data, index + 1));
                minidx = min; maxidx = max;
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
                let frame = data[index + 0] as FrameIndex;

                // curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
                let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
                curve.values.insert(index, T::newn(data, index + 1));
                minidx = min; maxidx = max;
            }
            curve.call = interplate_frame_values_step;
            curve
        },
        EAnimeCurve::EasingCurve => {
            let frame_count = data[1] as FrameIndex;
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
            let from = T::newn(data, 1);
            let to = T::newn(data, 1 + vs);
            let head = 1 + vs2;
            let mut curve = FrameCurve::<T>::curve_minmax_curve(from, to, design_frame_per_second);
            let step = 4;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as FrameIndex;
                let intangent  = data[index + 1] as f32;
                let value = data[index + 2] as f32;
                let outtangent = data[index + 3] as f32;

                // curve.curve_minmax_curve_frame(frame, value, intangent, outtangent);
                let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
                let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
                curve.minmax_curve_values.insert(index, keyframe);
                minidx = min; maxidx = max;
            }
            curve
        },
        EAnimeCurve::CubicBezierCurve => {
            let frame_count = data[1] as FrameIndex;
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
            let mut curve = FrameCurve::<T>::curve_cubic_spline(design_frame_per_second);
            let head = 1;
            let step = 1 + vs3;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as FrameIndex;
                let intangent = T::newn(data, index + 1);
                let value = T::newn(data, index + 1 + vs);
                let outtangent = T::newn(data, index + 1 + vs2);

                // curve.curve_cubic_splice_frame(frame, value, intangent, outtangent);
                let (index, min, max) = curve_frame_index(&mut curve.frames, frame);
                let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
                curve.cubic_spline_values.insert(index, keyframe);
                minidx = min; maxidx = max;
            }
            curve
        },
    };

    curve.min_frame = minidx;
    curve.max_frame = maxidx;
    curve.frame_number = maxidx - minidx;
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
pub fn p3d_anime_curve_query(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyID) -> bool {
	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    let key = unsafe { transmute(key) };
    let property = unsafe { transmute(property) };

    pi_gltf2_load::p3d_anime_curve_query(&resource.anime_assets, key, property)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_curve_create(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyID, data: &[f32], mode: EAnimeCurve) -> bool {
    pi_export_base::export::await_last_frame(app);
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


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_property_target_animation(
    cmds: &mut CommandsExchangeD3,
    curve_key: f64,
    property: EAnimePropertyID,
    group: f64,
    curve_target: f64,
) -> bool {
    let group = as_entity(group);
    let anime_target = as_entity(curve_target);

    let key: u64 = unsafe { transmute(curve_key) };

    let info = match property {
        EAnimePropertyID::LocalPosition => {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::LocalPosition, key));
        },
        EAnimePropertyID::LocalScaling =>  {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::LocalScaling, key));
        },
        EAnimePropertyID::LocalRotation =>  {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::LocalQuaternion, key));
        },
        EAnimePropertyID::LocalEulerAngles =>  {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::LocalEuler, key));
        },
        EAnimePropertyID::Enable =>  {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::Enable, key));
        },
        EAnimePropertyID::IndicesRange =>  {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::IndicesRange, key));
        },
        EAnimePropertyID::CameraFov => {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::Fov, key));
        },
        EAnimePropertyID::CameraOrthSize => {
            cmds.anime_property_targetanime.push(OpsPropertyTargetAnimation::ops(anime_target, group, EPropertyAnimationValueType::OrthSize, key));
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
