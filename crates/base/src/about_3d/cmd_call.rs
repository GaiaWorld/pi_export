use super::animation::EAmountMode;
use pi_curves::amount::AnimationAmountCalc;
use pi_curves::easing::EEasingMode;
use pi_curves::steps::EStepMode;
use pi_curves::curve::FrameIndex;

pub fn _amountcalc(
    amount_mode: EAmountMode,
    amount_param0: f64,
    amount_param1: f64,
    amount_param2: f64,
    amount_param3: f64,
) -> AnimationAmountCalc {
    match amount_mode {
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
    }
}