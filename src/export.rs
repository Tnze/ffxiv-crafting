// use crate::Actions;
//
// /// 将技能序列导出为游戏宏
// ///
// /// Examples:
// ///
// /// ```rust
// /// use ffxiv_crafting::export::to_chinese_macro;
// /// use ffxiv_crafting::Actions;
// /// let skill_seq = [
// ///     Actions::Reflect,
// ///     Actions::DelicateSynthesis,
// ///     Actions::DelicateSynthesis,
// ///     Actions::PreparatoryTouch,
// ///     Actions::ByregotsBlessing,
// ///     Actions::Groundwork,
// /// ];
// /// let game_macro = to_chinese_macro(&skill_seq);
// /// assert_eq!(game_macro, concat!(
// ///     "/ac 闲静 <wait.3>\n",
// ///     "/ac 精密制作 <wait.3>\n",
// ///     "/ac 精密制作 <wait.3>\n",
// ///     "/ac 坯料加工 <wait.3>\n",
// ///     "/ac 比尔格的祝福 <wait.3>\n",
// ///     "/ac 坯料制作 <wait.3>\n",
// /// ))
// /// ```
// pub fn to_chinese_macro(seq: &[Actions]) -> String {
//     let mut str = String::new();
//     for i in seq {
//         str.push_str("/ac ");
//         str.push_str(i.into());
//         str.push_str(match i {
//             Actions::MuscleMemory
//             | Actions::Reflect
//             | Actions::TrainedEye
//             | Actions::BasicSynthesis
//             | Actions::BasicTouch
//             | Actions::MastersMend
//             | Actions::HastyTouch
//             | Actions::RapidSynthesis
//             | Actions::Observe
//             | Actions::StandardTouch
//             | Actions::ByregotsBlessing
//             | Actions::PreciseTouch
//             | Actions::CarefulSynthesis
//             | Actions::PrudentTouch
//             | Actions::FocusedSynthesis
//             | Actions::FocusedTouch
//             | Actions::PreparatoryTouch
//             | Actions::Groundwork
//             | Actions::DelicateSynthesis
//             | Actions::PrudentSynthesis
//             | Actions::AdvancedTouch
//             | Actions::TrainedFinesse
//             | Actions::IntensiveSynthesis => " <wait.3>\n",
//             Actions::Innovation
//             | Actions::Veneration
//             | Actions::GreatStrides
//             | Actions::Manipulation
//             | Actions::WasteNot
//             | Actions::WasteNotII
//             | Actions::TricksOfTheTrade
//             | Actions::FinalAppraisal
//             | Actions::CarefulObservation
//             | Actions::HeartAndSoul => " <wait.2>\n",
//         })
//     }
//     str
// }
