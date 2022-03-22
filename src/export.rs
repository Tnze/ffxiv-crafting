// use crate::Skills;
//
// /// 将技能序列导出为游戏宏
// ///
// /// Examples:
// ///
// /// ```rust
// /// use ffxiv_crafting::export::to_chinese_macro;
// /// use ffxiv_crafting::Skills;
// /// let skill_seq = [
// ///     Skills::Reflect,
// ///     Skills::DelicateSynthesis,
// ///     Skills::DelicateSynthesis,
// ///     Skills::PreparatoryTouch,
// ///     Skills::ByregotsBlessing,
// ///     Skills::Groundwork,
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
// pub fn to_chinese_macro(seq: &[Skills]) -> String {
//     let mut str = String::new();
//     for i in seq {
//         str.push_str("/ac ");
//         str.push_str(i.into());
//         str.push_str(match i {
//             Skills::MuscleMemory
//             | Skills::Reflect
//             | Skills::TrainedEye
//             | Skills::BasicSynthesis
//             | Skills::BasicTouch
//             | Skills::MastersMend
//             | Skills::HastyTouch
//             | Skills::RapidSynthesis
//             | Skills::Observe
//             | Skills::StandardTouch
//             | Skills::ByregotsBlessing
//             | Skills::PreciseTouch
//             | Skills::CarefulSynthesis
//             | Skills::PrudentTouch
//             | Skills::FocusedSynthesis
//             | Skills::FocusedTouch
//             | Skills::PreparatoryTouch
//             | Skills::Groundwork
//             | Skills::DelicateSynthesis
//             | Skills::PrudentSynthesis
//             | Skills::AdvancedTouch
//             | Skills::TrainedFinesse
//             | Skills::IntensiveSynthesis => " <wait.3>\n",
//             Skills::Innovation
//             | Skills::Veneration
//             | Skills::GreatStrides
//             | Skills::Manipulation
//             | Skills::WasteNot
//             | Skills::WasteNotII
//             | Skills::TricksOfTheTrade
//             | Skills::FinalAppraisal
//             | Skills::CarefulObservation
//             | Skills::HeartAndSoul => " <wait.2>\n",
//         })
//     }
//     str
// }
