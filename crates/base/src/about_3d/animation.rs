
use js_proxy_gen_macro::pi_js_export;
use pi_gltf2_load::TValue;
use pi_curves::{curve::{curves::curve_frame_index, frame::{CurveFrameValue, FrameDataValue}, frame_curve::{frames::interplate_frame_values_step, FrameCurve}, FrameIndex, FramePerSecond}, easing::EEasingMode, steps::EStepMode};
use pi_scene_context::prelude::*;
use pi_scene_shell::prelude::*;
use serde::{Serialize, Deserialize};


#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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


pub enum EAnimeCurveTemp {
    LocalPosition       (Result<Handle<TypeFrameCurve<LocalPosition                >>, TypeFrameCurve<LocalPosition          >>),
    LocalRotation       (Result<Handle<TypeFrameCurve<LocalRotationQuaternion      >>, TypeFrameCurve<LocalRotationQuaternion>>),
    LocalScaling        (Result<Handle<TypeFrameCurve<LocalScaling                 >>, TypeFrameCurve<LocalScaling           >>), 
    Float               (Result<Handle<TypeFrameCurve<AnimatorableFloat            >>, TypeFrameCurve<AnimatorableFloat      >>), 
    CameraOrthSize      (Result<Handle<TypeFrameCurve<CameraOrthSize               >>, TypeFrameCurve<CameraOrthSize         >>), 
    CameraFov           (Result<Handle<TypeFrameCurve<CameraFov                    >>, TypeFrameCurve<CameraFov              >>),
    Enable              (Result<Handle<TypeFrameCurve<Enable                       >>, TypeFrameCurve<Enable                 >>),
    LocalEulerAngles    (Result<Handle<TypeFrameCurve<LocalEulerAngles             >>, TypeFrameCurve<LocalEulerAngles       >>),
    Vec3                (Result<Handle<TypeFrameCurve<AnimatorableVec3             >>, TypeFrameCurve<AnimatorableVec3       >>), 
    Vec4                (Result<Handle<TypeFrameCurve<AnimatorableVec4             >>, TypeFrameCurve<AnimatorableVec4       >>),
    Uint                (Result<Handle<TypeFrameCurve<AnimatorableUint             >>, TypeFrameCurve<AnimatorableUint       >>),
    IndicesRange        (Result<Handle<TypeFrameCurve<IndiceRenderRange            >>, TypeFrameCurve<IndiceRenderRange      >>),
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen)]
#[pi_js_export]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[inline(never)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EFillMode {
    None = 0,
    Forwards = 1,
    Backwards = 2,
    Both = 3,
}

#[inline]
fn _curve_frame(
    data: &[f32],
    index: usize,
    frames: &mut Vec<u16>,
) -> (usize, u16, u16) {
    let frame = data[index + 0] as FrameIndex;
    curve_frame_index(frames, frame)
}
fn _curve_minmax(
    data: &[f32],
    index: usize,
    frames: &mut Vec<u16>,
) -> ((usize, u16, u16), CurveFrameValue<f32>) {
    let frame = data[index + 0] as FrameIndex;
    let intangent  = data[index + 1] as f32;
    let value = data[index + 2] as f32;
    let outtangent = data[index + 3] as f32;

    let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
    (curve_frame_index(frames, frame), keyframe)
}
pub fn curve<const N: usize, T: TValue<N> + FrameDataValue>(
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
                let (idx, min, max) = _curve_frame(data, index, &mut curve.frames);
                curve.values.insert(idx, T::newn(data, index + 1));
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
                let (idx, min, max) = _curve_frame(data, index, &mut curve.frames);
                curve.values.insert(idx, T::newn(data, index + 1));
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
                let ((idx, min, max), keyframe) = _curve_minmax(data, index, &mut curve.frames);
                curve.minmax_curve_values.insert(idx, keyframe);
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
                let (idx, min, max) = curve_frame_index(&mut curve.frames, frame);
                let keyframe = CurveFrameValue::new(value, [intangent, outtangent]);
                curve.cubic_spline_values.insert(idx, keyframe);
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
