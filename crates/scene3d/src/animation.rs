
use std::{mem::{replace, transmute}, fmt::Debug};
use js_proxy_gen_macro::pi_js_export;
use pi_gltf2_load::TValue;
use pi_node_materials::prelude::*;
pub use pi_export_base::export::Atom;

use pi_animation::amount::AnimationAmountCalc;
use pi_curves::{easing::EEasingMode, steps::EStepMode, curve::{frame_curve::{FrameCurve, frames::interplate_frame_values_step}, frame::{FrameDataValue, KeyFrameCurveValue}, FramePerSecond, FrameIndex}};
use pi_engine_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;
use pi_slotmap::DefaultKey;
pub use crate::engine::ActionSetScene3D;
use crate::{as_entity, as_f64, as_f64_dk, as_dk};
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
    param: &mut ActionSetScene3D,
    scene: f64,
    group_target: f64,
) -> Option<f64> {
    let scene: Entity = as_entity(scene);
    let group_target: Entity = as_entity(group_target);

    let mut resource = param.resource.get_mut(&mut app.world);

    if let Some(id_group) = resource.anime_scene_ctxs.create_group(scene) {
        resource.anime_global.record_group(group_target, id_group.clone());
        Some(as_f64_dk(&id_group))
    } else {
        None
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
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
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    scene: f64,
    group_key: f64,
    weight: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut resource = param.resource.get_mut(&mut app.world);

    resource.anime_scene_ctxs.group_weight(scene, group_key, weight as f32);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 重置动画组各动画作用目标到非动画操作状态
pub fn p3d_animation_group_target_reset(cmds: &mut CommandsExchangeD3, scene: f64, group_key: f64,) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    cmds.anime_reset_while_start.push(OpsAnimationGroupStartReset::ops(scene, group_key));
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
    delay_ms: f64,
    fillmode: EFillMode,
    amount_param0: f64,
    amount_param1: f64,
    amount_param2: f64,
    amount_param3: f64,
) {
    let scene: Entity = as_entity(scene);
    let group_key = as_dk(&group_key);

    let mut resource = param.resource.get_mut(&mut app.world);

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

    resource.anime_scene_ctxs.start_with_progress(scene, group_key, AnimationGroupParam::new(speed as f32, loop_mode, from as f32, to as f32, fps as FramePerSecond, amountcalc), delay_ms as KeyFrameCurveValue, fillmode);
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

    let mut resource = param.resource.get_mut(&mut app.world);

    resource.anime_scene_ctxs.pause(scene, group_key);
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

    let mut resource = param.resource.get_mut(&mut app.world);

    resource.anime_scene_ctxs.stop(scene, group_key);
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

    let mut resource = param.resource.get_mut(&mut app.world);

    match mode {
        EAnimationGroupListen::Start    => {
            resource.anime_global.add_start_listen(id_group);
        },
        EAnimationGroupListen::End      => {
            resource.anime_global.add_end_listen(id_group);
        },
        EAnimationGroupListen::Loop     => {
            resource.anime_global.add_loop_listen(id_group);
        },
        EAnimationGroupListen::Frame    => {
            resource.anime_global.add_frame_event_listen(id_group);
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

    let mut resource = param.resource.get_mut(&mut app.world);
    resource.anime_global.add_frame_event(id_group, percent as f32, data as AnimeFrameEventData);
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

    let mut resource = param.resource.get_mut(&mut app.world);
    resource.anime_global.remove(&id_group);
    resource.anime_scene_ctxs.delete_group(&id_scene, id_group);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_anime_events(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    receive: &mut [f64],
    receive_len: f64,
) -> f64 {

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
        receive[i + 1] = as_f64_dk(&name);
        receive[i + 2] = tag as f64;
        receive[i + 3] = count as f64;
        index += 1;
    });

    index as f64
}


fn curve<const N: usize, T: TValue<N> + FrameDataValue + Debug>(
    data: &[f32],
    mode: EAnimeCurve,
) -> FrameCurve<T> {
    let vs = N; let vs2 = N * 2; let vs3 = N * 3;
    let design_frame_per_second = data[0] as FrameIndex;
    let curve = match mode {
        EAnimeCurve::FrameValues => {
            let mut curve = FrameCurve::<T>::curve_frame_values(design_frame_per_second);
            let head = 1;
            let step = 1 + vs;
            let frames = (data.len() - head) / step;
            for i in 0..frames {
                let index = head + i * step;
                let frame = data[index + 0] as FrameIndex;
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
                let frame = data[index + 0] as FrameIndex;
                curve.curve_frame_values_frame(frame, T::newn(data, index + 1));
            }
            curve.call = interplate_frame_values_step;
            curve
        },
        EAnimeCurve::EasingCurve => {
            let design_frame_per_second = data[0] as FramePerSecond;
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
            let design_frame_per_second = data[0] as FramePerSecond;
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
                curve.curve_minmax_curve_frame(frame, value, intangent, outtangent);
            }
            curve
        },
        EAnimeCurve::CubicBezierCurve => {
            let design_frame_per_second = data[0] as FramePerSecond;
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
            let design_frame_per_second = data[0] as FramePerSecond;
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
pub fn p3d_anime_curve_query(app: &mut Engine, param: &mut ActionSetScene3D, key: f64, property: EAnimePropertyID) -> bool {
    let resource = param.resource.get_mut(&mut app.world);

    let key = unsafe { transmute(key) };
    let property = unsafe { transmute(property) };

    pi_gltf2_load::p3d_anime_curve_query(&resource.anime_assets, key, property)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
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
            let v = curve::<1, Alpha>(data,  mode);
            cmds.alpha.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MainColor           => {
            let v = curve::<3, MainColor>(data,  mode);
            cmds.maincolor_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MainTexUScale       => {
            let v = curve::<1, MainTexUScale>(data,  mode);
            cmds.mainuscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MainTexVScale       => {
            let v = curve::<1, MainTexVScale>(data,  mode);
            cmds.mainvscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MainTexUOffset      => {
            let v = curve::<1, MainTexUOffset>(data,  mode);
            cmds.mainuoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MainTexVOffset      => {
            let v = curve::<1, MainTexVOffset>(data,  mode);
            cmds.mainvoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::OpacityTexUScale    => {
            let v = curve::<1, OpacityTexUScale>(data,  mode);
            cmds.opacityuscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::OpacityTexVScale    => {
            let v = curve::<1, OpacityTexVScale>(data,  mode);
            cmds.opacityvscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::OpacityTexUOffset   => {
            let v = curve::<1, OpacityTexUOffset>(data,  mode);
            cmds.opacityuoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::OpacityTexVOffset   => {
            let v = curve::<1, OpacityTexVOffset>(data,  mode);
            cmds.opacityvoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::AlphaCutoff         => {
            let v = curve::<1, Cutoff>(data,  mode);
            cmds.alphacutoff.insert(key, TypeFrameCurve(v)).is_ok()
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
            let v = curve::<3, LightDiffuse>(data,  mode);
            cmds.lightdiffuse_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MaskTexUScale       => {
            let v = curve::<1, MaskTexUScale>(data,  mode);
            cmds.maskuscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MaskTexVScale       => {
            let v = curve::<1, MaskTexVScale>(data,  mode);
            cmds.maskvscl_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MaskTexUOffset      => {
            let v = curve::<1, MaskTexUOffset>(data,  mode);
            cmds.maskuoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MaskTexVOffset      => {
            let v = curve::<1, MaskTexVOffset>(data,  mode);
            cmds.maskvoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::MaskCutoff          => {
            let v = curve::<1, MaskCutoff>(data,  mode);
            cmds.maskcutoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::Enable            => {
            let v = curve::<1, Enable>(data,  mode);
            cmds.enable.insert(key, TypeFrameCurve(v)).is_ok()
        },
        EAnimePropertyID::BoneOffset          => {
            let v = curve::<1, InstanceBoneoffset>(data,  mode);
            cmds.boneoff_curves.insert(key, TypeFrameCurve(v)).is_ok()
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
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_target_animation(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    curve_key: f64,
    property: EAnimePropertyID,
    id_scene: f64,
    group: f64,
    curve_target: f64,
) -> bool {
    let id_scene = as_entity(id_scene);
    let group = as_dk(&group);
    let anime_target = as_entity(curve_target);

    let mut resource = param.resource.get_mut(&mut app.world);
    let key: u64 = unsafe { transmute(curve_key) };

    let info = match property {
        EAnimePropertyID::LocalPosition => {
            if let Some(curve) = resource.anime_assets.position.get(&key) {
                // log::warn!("Curve Ok!");
                resource.anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::LocalScaling =>  {
            if let Some(curve) = resource.anime_assets.scaling.get(&key) {
                resource.anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::LocalRotation =>  {
            if let Some(curve) = resource.anime_assets.quaternion.get(&key) {
                resource.anime_contexts.quaternion.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::LocalEulerAngles =>  {
            if let Some(curve) = resource.anime_assets.euler.get(&key) {
                resource.anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::Alpha =>  {
            if let Some(curve) = resource.anime_assets.alpha.get(&key) {
                resource.anime_contexts.alpha.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MainColor =>  {
            if let Some(curve) = resource.anime_assets.maincolor_curves.get(&key) {
                resource.anime_contexts.maincolor.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MainTexUScale =>  {
            if let Some(curve) = resource.anime_assets.mainuscl_curves.get(&key) {
                resource.anime_contexts.maintex_uscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MainTexVScale =>  {
            if let Some(curve) = resource.anime_assets.mainvscl_curves.get(&key) {
                resource.anime_contexts.maintex_vscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MainTexUOffset =>  {
            if let Some(curve) = resource.anime_assets.mainuoff_curves.get(&key) {
                resource.anime_contexts.maintex_uoffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MainTexVOffset =>  {
            if let Some(curve) = resource.anime_assets.mainvoff_curves.get(&key) {
                resource.anime_contexts.maintex_voffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::OpacityTexUScale =>  {
            if let Some(curve) = resource.anime_assets.opacityuscl_curves.get(&key) {
                resource.anime_contexts.opacitytex_uscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::OpacityTexVScale =>  {
            if let Some(curve) = resource.anime_assets.opacityvscl_curves.get(&key) {
                resource.anime_contexts.opacitytex_vscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::OpacityTexUOffset =>  {
            if let Some(curve) = resource.anime_assets.opacityuoff_curves.get(&key) {
                resource.anime_contexts.opacitytex_uoffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::OpacityTexVOffset =>  {
            if let Some(curve) = resource.anime_assets.opacityvoff_curves.get(&key) {
                resource.anime_contexts.opacitytex_voffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::AlphaCutoff =>  {
            if let Some(curve) = resource.anime_assets.alphacutoff.get(&key) {
                resource.anime_contexts.alphacutoff.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::LightDiffuse =>  {
            if let Some(curve) = resource.anime_assets.lightdiffuse_curves.get(&key) {
                resource.anime_contexts.lightdiffuse.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MaskTexUScale =>  {
            if let Some(curve) = resource.anime_assets.maskuscl_curves.get(&key) {
                resource.anime_contexts.masktex_uscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MaskTexVScale =>  {
            if let Some(curve) = resource.anime_assets.maskvscl_curves.get(&key) {
                resource.anime_contexts.masktex_vscale.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MaskTexUOffset =>  {
            if let Some(curve) = resource.anime_assets.maskuoff_curves.get(&key) {
                resource.anime_contexts.masktex_uoffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MaskTexVOffset =>  {
            if let Some(curve) = resource.anime_assets.maskvoff_curves.get(&key) {
                resource.anime_contexts.masktex_voffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::MaskCutoff =>  {
            if let Some(curve) = resource.anime_assets.maskcutoff_curves.get(&key) {
                resource.anime_contexts.maskcutoff.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::Enable =>  {
            if let Some(curve) = resource.anime_assets.enable.get(&key) {
                resource.anime_contexts.isactive.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::BoneOffset =>  {
            if let Some(curve) = resource.anime_assets.boneoff_curves.get(&key) {
                resource.anime_contexts.boneoffset.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::IndicesRange =>  {
            if let Some(curve) = resource.anime_assets.indicerange_curves.get(&key) {
                resource.anime_contexts.indices_range.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::CameraFov => {
            if let Some(curve) = resource.anime_assets.camerafov.get(&key) {
                resource.anime_contexts.camerafov.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::CameraOrthSize => {
            if let Some(curve) = resource.anime_assets.camerasize.get(&key) {
                resource.anime_contexts.camerasize.ctx.create_animation(0, AssetTypeFrameCurve::from(curve))
            } else { return false; }
        },
        EAnimePropertyID::CellId => {
            return false;
        },
        EAnimePropertyID::Intensity => {
            return false;
        },
    };

    resource.anime_scene_ctxs.add_target_anime(id_scene, anime_target, group, info);
    return true;
}