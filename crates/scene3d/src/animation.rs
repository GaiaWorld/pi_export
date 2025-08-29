
use std::{mem::{replace, transmute}, fmt::Debug};
use js_proxy_gen_macro::pi_js_export;
#[cfg(feature = "record")]
use pi_export_base::record::ERecord3D;
use pi_gltf2_load::TValue;
pub use pi_export_base::export::Atom;

use pi_animation::amount::AnimationAmountCalc;
use pi_curves::{curve::{curves::curve_frame_index, frame::{CurveFrameValue, FrameDataValue, KeyFrameCurveValue}, frame_curve::{frames::interplate_frame_values_step, FrameCurve}, FrameIndex, FramePerSecond}, easing::EEasingMode, steps::EStepMode};
use pi_scene_shell::prelude::*;
pub use pi_export_base::export::Engine;
use pi_scene_context::prelude::*;
use pi_slotmap::DefaultKey;
use serde::{Serialize, Deserialize};
pub use crate::engine::ActionSetScene3D;
use crate::{as_entity, as_f64, as_f64_dk, record::{ERecordCMD, ERecordMode}};
pub use crate::commands::CommandsExchangeD3;
pub use pi_export_base::about_3d::animation::*;
use pi_3d::TActionSet;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;


#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 创建动画组
pub fn p3d_animation_group(
    app: &mut Engine,
    cmds: &mut CommandsExchangeD3,
    scene: f64,
) -> f64 {
    #[cfg(feature = "replay")]
    return as_f64(&Entity::null());

	pi_export_base::export::await_last_frame(app);

    let id: Entity = app.world.spawn_empty_id();
    
    #[cfg(feature = "record")]
    CommandsExchangeD3::record_create(&mut app.world, as_f64(&id));

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroup(scene, as_f64(&id)));

    let scene: Entity = as_entity(scene);


    CommandsExchangeD3::p3d_animation_group(cmds, scene, id);

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
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupWeight(group, weight));

    let group = as_entity(group);
    CommandsExchangeD3::p3d_animation_group_weight(cmds, group, weight as f32);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
/// 重置动画组各动画作用目标到非动画操作状态
pub fn p3d_animation_group_target_reset(cmds: &mut CommandsExchangeD3, group: f64,) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupTargetReset(group));

    let group = as_entity(group);
    CommandsExchangeD3::p3d_animation_group_target_reset(cmds, group);
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
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupStart(group_key, speed, loop_mode, loop_count, from, to, fps, amount_mode, delay_ms, fillmode, amount_param0, amount_param1, amount_param2, amount_param3));

    let group_key = as_entity(group_key);
    CommandsExchangeD3::p3d_anime_group_start(cmds, group_key, speed, loop_mode, loop_count, from, to, fps, amount_mode, delay_ms, fillmode, amount_param0, amount_param1, amount_param2, amount_param3);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_pause(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupPause(group_key));

    let group_key = as_entity(group_key);
    CommandsExchangeD3::p3d_anime_group_pause(cmds, group_key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_stop(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupStop(group_key));

    let group_key = as_entity(group_key);
    CommandsExchangeD3::p3d_anime_group_stop(cmds, group_key);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_group_goto(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
    amount: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupGoto(group_key, amount));

    let group_key = as_entity(group_key);
    CommandsExchangeD3::p3d_anime_group_goto(cmds, group_key, amount as KeyFrameCurveValue);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_listen(
    cmds: &mut CommandsExchangeD3,
    group_key: f64,
    mode: EAnimationGroupListen,
) {
    #[cfg(feature = "replay")]
    return ;

    let group_key = as_entity(group_key);

    match mode {
        EAnimationGroupListen::Start    => {
            cmds.anime_action().push(OpsAnimationGroupAction::listen_start(group_key));
        },
        EAnimationGroupListen::End      => {
            cmds.anime_action().push(OpsAnimationGroupAction::listen_end(group_key));
        },
        EAnimationGroupListen::Loop     => {
            cmds.anime_action().push(OpsAnimationGroupAction::listen_loop(group_key));
        },
        EAnimationGroupListen::Frame    => {
            cmds.anime_action().push(OpsAnimationGroupAction::listen_frame(group_key));
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
    #[cfg(feature = "replay")]
    return ;

    let group = as_entity(group);

    cmds.anime_action().push(OpsAnimationGroupAction::frameevent(group, percent as f32, data as AnimeFrameEventData));
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_restart(
    cmds: &mut CommandsExchangeD3,
    group: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupRestart(group));

    let id_group = as_entity(group);
    CommandsExchangeD3::p3d_animation_group_restart(cmds, id_group);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_animation_group_delete(
    cmds: &mut CommandsExchangeD3,
    group: f64,
) {
    #[cfg(feature = "replay")]
    return ;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::AnimationGroupDelete(group));

    let id_group = as_entity(group);
    CommandsExchangeD3::p3d_animation_group_delete(cmds, id_group);
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_query_anime_events(
    app: &mut Engine,
    param: &mut ActionSetScene3D,
    receive: &mut [f64],
    receive_len: f64,
) -> f64 {
    #[cfg(feature = "replay")]
    return ;


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
    #[cfg(feature = "replay")]
    return false;

	pi_export_base::export::await_last_frame(app);
    let resource = param.resource.get_mut(&mut app.world);

    let key = unsafe { transmute(key) };
    let property = unsafe { transmute(property) };

    pi_gltf2_load::p3d_anime_curve_query(&resource.anime_assets, key, property)
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
pub fn p3d_anime_curve_create(app: &mut Engine, param: &mut ActionSetScene3D, cmds: &mut CommandsExchangeD3, key: f64, property: EAnimePropertyID, data: &[f32], mode: EAnimeCurve) -> bool {
    
    #[cfg(feature = "replay")]
    return false;

    pi_export_base::export::await_last_frame(app);

    #[cfg(feature = "record")]
    cmds.record2(ERecord3D::CreateAnimationCurve(key, property, data.to_vec(), mode));

    let mut resource = param.resource.get_mut(&mut app.world);
    let result = CommandsExchangeD3::p3d_anime_curve_create(&mut resource.anime_assets, key, property, data, mode);
    log::error!("Anim Curve: {:?}", result);
result
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
    #[cfg(feature = "replay")]
    return false;

    #[cfg(feature = "record")]
    cmds.record(ERecordCMD::PropertyTargetAnimation(curve_key, property, group, curve_target));

    let group = as_entity(group);
    let curve_target = as_entity(curve_target);
    let key: u64 = unsafe { transmute(curve_key) };
    return CommandsExchangeD3::p3d_property_target_animation(cmds, key, property, group, curve_target);
}
