use crate::Skills;

/// 将技能序列导出供[FFXIV Craft Opt](http://ffxiv.tk/crafterbeta/#/simulator)导入
///
/// Examples:
///
/// ```rust
/// use ffxiv_crafting::export::to_ffxiv_craft_opt_web;
/// use ffxiv_crafting::Skills;
/// let skill_seq = [
///     Skills::Reflect,
///     Skills::DelicateSynthesis,
///     Skills::DelicateSynthesis,
///     Skills::PreparatoryTouch,
///     Skills::ByregotsBlessing,
///     Skills::Groundwork,
/// ];
/// let seq = to_ffxiv_craft_opt_web(&skill_seq, 80);
/// assert_eq!(seq, r#"["reflect", "delicateSynthesis", "delicateSynthesis", "preparatoryTouch", "byregotsBlessing", "groundwork"]"#);
/// ```
pub fn to_ffxiv_craft_opt_web(seq: &[Skills], level: i32) -> String {
    let mut str_seq = String::from("[");
    for i in seq.iter().map(|&x| match x {
        Skills::BasicSynthesis => match level {
            x if x < 32 => "basicSynth",
            _ => "basicSynth2",
        },
        Skills::BasicTouch => "basicTouch",
        Skills::MastersMend => "mastersMend",
        Skills::HastyTouch => "hastyTouch",
        Skills::RapidSynthesis => match level {
            x if x < 63 => "rapidSynthesis",
            _ => "rapidSynthesis2",
        },
        Skills::InnerQuiet => "innerQuiet",
        Skills::Observe => "observe",
        Skills::TricksOfTheTrade => "tricksOfTheTrade",
        Skills::WasteNot => "wasteNot",
        Skills::Veneration => "veneration",
        Skills::StandardTouch => "standardTouch",
        Skills::GreatStrides => "greatStrides",
        Skills::Innovation => "innovation",
        Skills::NameOfTheElements => "nameOfTheElements",
        Skills::BrandOfTheElements => "brandOfTheElements",
        Skills::FinalAppraisal => "finalAppraisal",
        Skills::WasteNotII => "wasteNot2",
        Skills::ByregotsBlessing => "byregotsBlessing",
        Skills::PreciseTouch => "preciseTouch",
        Skills::MuscleMemory => "muscleMemory",
        Skills::CarefulSynthesis => "carefulSynthesis",
        Skills::PatientTouch => "patientTouch",
        Skills::Manipulation => "manipulation",
        Skills::PrudentTouch => "prudentTouch",
        Skills::FocusedSynthesis => "focusedSynthesis",
        Skills::FocusedTouch => "focusedTouch",
        Skills::Reflect => "reflect",
        Skills::PreparatoryTouch => "preparatoryTouch",
        Skills::Groundwork => "groundwork",
        Skills::DelicateSynthesis => "delicateSynthesis",
        Skills::IntensiveSynthesis => "intensiveSynthesis",
        Skills::TrainedEye => "trainedEye",
        Skills::CarefulObservation => "carefulObservation",
    }) {
        str_seq.push('\"');
        str_seq.push_str(i);
        str_seq.push('\"');
        str_seq.push_str(", ");
    }
    if str_seq.len() > 2 {
        str_seq.replace_range((str_seq.len() - 2).., "]");
    } else {
        str_seq.push(']');
    }
    str_seq
}

/// 将技能序列导出为游戏宏
///
/// Examples:
///
/// ```rust
/// use ffxiv_crafting::export::to_chinese_macro;
/// use ffxiv_crafting::Skills;
/// let skill_seq = [
///     Skills::Reflect,
///     Skills::DelicateSynthesis,
///     Skills::DelicateSynthesis,
///     Skills::PreparatoryTouch,
///     Skills::ByregotsBlessing,
///     Skills::Groundwork,
/// ];
/// let game_macro = to_chinese_macro(&skill_seq);
/// assert_eq!(game_macro, concat!(
///     "/ac 闲静 <wait.3>\n",
///     "/ac 精密制作 <wait.3>\n",
///     "/ac 精密制作 <wait.3>\n",
///     "/ac 坯料加工 <wait.3>\n",
///     "/ac 比尔格的祝福 <wait.3>\n",
///     "/ac 坯料制作 <wait.3>\n",
/// ))
/// ```
pub fn to_chinese_macro(seq: &[Skills]) -> String {
    let mut str = String::new();
    for i in seq {
        str.push_str("/ac ");
        str.push_str(i.as_chinese());
        str.push_str(match i {
            Skills::MuscleMemory
            | Skills::Reflect
            | Skills::TrainedEye
            | Skills::BasicSynthesis
            | Skills::BasicTouch
            | Skills::MastersMend
            | Skills::HastyTouch
            | Skills::RapidSynthesis
            | Skills::Observe
            | Skills::StandardTouch
            | Skills::BrandOfTheElements
            | Skills::ByregotsBlessing
            | Skills::PreciseTouch
            | Skills::CarefulSynthesis
            | Skills::PatientTouch
            | Skills::PrudentTouch
            | Skills::FocusedSynthesis
            | Skills::FocusedTouch
            | Skills::PreparatoryTouch
            | Skills::Groundwork
            | Skills::DelicateSynthesis
            | Skills::IntensiveSynthesis => " <wait.3>\n",
            Skills::InnerQuiet
            | Skills::Innovation
            | Skills::Veneration
            | Skills::GreatStrides
            | Skills::Manipulation
            | Skills::WasteNot
            | Skills::WasteNotII
            | Skills::TricksOfTheTrade
            | Skills::NameOfTheElements
            | Skills::FinalAppraisal
            | Skills::CarefulObservation => " <wait.2>\n",
        })
    }
    str
}
