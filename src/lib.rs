#![feature(mixed_integer_ops)]

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};

use crate::CastActionError::RequireInnerQuiet;

mod data;
pub mod export;

/// 代表一个玩家在作业时可以使用的一个技能的枚举。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Skills {
    BasicSynthesis,
    BasicTouch,
    MastersMend,
    HastyTouch,
    RapidSynthesis,
    Observe,
    TricksOfTheTrade,
    WasteNot,
    Veneration,
    StandardTouch,
    GreatStrides,
    Innovation,
    FinalAppraisal,
    WasteNotII,
    ByregotsBlessing,
    PreciseTouch,
    MuscleMemory,
    CarefulSynthesis,
    Manipulation,
    PrudentTouch,
    FocusedSynthesis,
    FocusedTouch,
    Reflect,
    PreparatoryTouch,
    Groundwork,
    DelicateSynthesis,
    IntensiveSynthesis,
    TrainedEye,
    AdvancedTouch,
    PrudentSynthesis,
    TrainedFinesse,
    CarefulObservation,
    HeartAndSoul,
}

impl Skills {
    fn craft_point(&self) -> i32 {
        match self {
            Skills::BasicSynthesis => 0,
            Skills::BasicTouch => 18,
            Skills::MastersMend => 88,
            Skills::HastyTouch => 0,
            Skills::RapidSynthesis => 0,
            Skills::Observe => 7,
            Skills::TricksOfTheTrade => 0,
            Skills::WasteNot => 56,
            Skills::Veneration => 18,
            Skills::StandardTouch => 32,
            Skills::GreatStrides => 32,
            Skills::Innovation => 18,
            Skills::FinalAppraisal => 1,
            Skills::WasteNotII => 98,
            Skills::ByregotsBlessing => 24,
            Skills::PreciseTouch => 18,
            Skills::MuscleMemory => 6,
            Skills::CarefulSynthesis => 7,
            Skills::Manipulation => 96,
            Skills::PrudentTouch => 25,
            Skills::FocusedSynthesis => 5,
            Skills::FocusedTouch => 18,
            Skills::Reflect => 6,
            Skills::PreparatoryTouch => 40,
            Skills::Groundwork => 18,
            Skills::DelicateSynthesis => 32,
            Skills::IntensiveSynthesis => 6,
            Skills::TrainedEye => 250,
            Skills::AdvancedTouch => 46,
            Skills::PrudentSynthesis => 18,
            Skills::TrainedFinesse => 32,
            Skills::CarefulObservation => 0,
            Skills::HeartAndSoul => 0,
        }
    }
    fn unlock_level(&self) -> u8 {
        match self {
            Skills::BasicSynthesis => 1,
            Skills::BasicTouch => 5,
            Skills::MastersMend => 7,
            Skills::HastyTouch => 9,
            Skills::RapidSynthesis => 9,
            Skills::Observe => 13,
            Skills::TricksOfTheTrade => 13,
            Skills::WasteNot => 15,
            Skills::Veneration => 15,
            Skills::StandardTouch => 18,
            Skills::GreatStrides => 21,
            Skills::Innovation => 26,
            Skills::FinalAppraisal => 42,
            Skills::WasteNotII => 47,
            Skills::ByregotsBlessing => 50,
            Skills::PreciseTouch => 53,
            Skills::MuscleMemory => 54,
            Skills::CarefulSynthesis => 62,
            Skills::Manipulation => 65,
            Skills::PrudentTouch => 66,
            Skills::FocusedSynthesis => 67,
            Skills::FocusedTouch => 68,
            Skills::Reflect => 69,
            Skills::PreparatoryTouch => 71,
            Skills::Groundwork => 72,
            Skills::DelicateSynthesis => 76,
            Skills::IntensiveSynthesis => 78,
            Skills::TrainedEye => 80,
            Skills::AdvancedTouch => 84,
            Skills::PrudentSynthesis => 88,
            Skills::TrainedFinesse => 90,
            Skills::CarefulObservation => 55,
            Skills::HeartAndSoul => 86,
        }
    }
}

impl From<&Skills> for &str {
    fn from(sk: &Skills) -> Self {
        match sk {
            Skills::BasicSynthesis => "basic_synthesis",
            Skills::BasicTouch => "basic_touch",
            Skills::MastersMend => "masters_mend",
            Skills::HastyTouch => "hasty_touch",
            Skills::RapidSynthesis => "rapid_synthesis",
            Skills::Observe => "observe",
            Skills::TricksOfTheTrade => "tricks_of_the_trade",
            Skills::WasteNot => "waste_not",
            Skills::Veneration => "veneration",
            Skills::StandardTouch => "standard_touch",
            Skills::GreatStrides => "great_strides",
            Skills::Innovation => "innovation",
            Skills::FinalAppraisal => "final_appraisal",
            Skills::WasteNotII => "waste_not_ii",
            Skills::ByregotsBlessing => "byregot_s_blessing",
            Skills::PreciseTouch => "precise_touch",
            Skills::MuscleMemory => "muscle_memory",
            Skills::CarefulSynthesis => "careful_synthesis",
            Skills::Manipulation => "manipulation",
            Skills::PrudentTouch => "prudent_touch",
            Skills::FocusedSynthesis => "focused_synthesis",
            Skills::FocusedTouch => "focused_touch",
            Skills::Reflect => "reflect",
            Skills::PreparatoryTouch => "preparatory_touch",
            Skills::Groundwork => "groundwork",
            Skills::DelicateSynthesis => "delicate_synthesis",
            Skills::IntensiveSynthesis => "intensive_synthesis",
            Skills::TrainedEye => "trained_eye",
            Skills::AdvancedTouch => "advanced_touch",
            Skills::PrudentSynthesis => "prudent_synthesis",
            Skills::TrainedFinesse => "trained_finesse",
            Skills::CarefulObservation => "careful_observation",
            Skills::HeartAndSoul => "heart_and_soul",
        }
    }
}

impl TryFrom<&str> for Skills {
    type Error = UnknownSkillErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "basic_synthesis" | "制作" => Skills::BasicSynthesis,
            "basic_touch" | "加工" => Skills::BasicTouch,
            "masters_mend" | "精修" => Skills::MastersMend,
            "hasty_touch" | "仓促" => Skills::HastyTouch,
            "rapid_synthesis" | "高速制作" => Skills::RapidSynthesis,
            "observe" | "观察" => Skills::Observe,
            "tricks_of_the_trade" | "秘诀" => Skills::TricksOfTheTrade,
            "waste_not" | "俭约" => Skills::WasteNot,
            "veneration" | "崇敬" => Skills::Veneration,
            "standard_touch" | "中级加工" => Skills::StandardTouch,
            "great_strides" | "阔步" => Skills::GreatStrides,
            "innovation" | "改革" => Skills::Innovation,
            "final_appraisal" | "最终确认" => Skills::FinalAppraisal,
            "waste_not_ii" | "长期俭约" => Skills::WasteNotII,
            "byregot_s_blessing" | "比尔格的祝福" => Skills::ByregotsBlessing,
            "precise_touch" | "集中加工" => Skills::PreciseTouch,
            "muscle_memory" | "坚信" => Skills::MuscleMemory,
            "careful_synthesis" | "模范制作" => Skills::CarefulSynthesis,
            "manipulation" | "掌握" => Skills::Manipulation,
            "prudent_touch" | "俭约加工" => Skills::PrudentTouch,
            "focused_synthesis" | "注视制作" => Skills::FocusedSynthesis,
            "focused_touch" | "注视加工" => Skills::FocusedTouch,
            "reflect" | "闲静" => Skills::Reflect,
            "preparatory_touch" | "坯料加工" => Skills::PreparatoryTouch,
            "groundwork" | "坯料制作" => Skills::Groundwork,
            "delicate_synthesis" | "精密制作" => Skills::DelicateSynthesis,
            "intensive_synthesis" | "集中制作" => Skills::IntensiveSynthesis,
            "trained_eye" | "工匠的神速技巧" => Skills::TrainedEye,
            "advanced_touch" | "上级加工" => Skills::AdvancedTouch,
            "prudent_synthesis" | "俭约制作" => Skills::PrudentSynthesis,
            "trained_finesse" | "工匠的神技" => Skills::TrainedFinesse,
            "careful_observation" | "设计变动" => Skills::CarefulObservation,
            "heart_and_soul" | "专心致志" => Skills::HeartAndSoul,
            _ => return Err(UnknownSkillErr()),
        })
    }
}

#[derive(Debug)]
pub struct UnknownSkillErr();

impl Display for UnknownSkillErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown skill name")
    }
}

impl Error for UnknownSkillErr {}

/// 代表了当前的“制作状态”，也就是俗称的球色。
#[derive(Copy, Clone, Debug)]
pub enum Condition {
    // 白：通常
    Normal,
    // 红：高品质，加工效率1.5倍
    Good,
    // 彩：最高品质
    Excellent,
    // 黑：低品质
    Poor,

    // 黄：成功率增加 25%
    Centered,
    // 蓝：耐久消耗降低 50%, 效果可与俭约叠加
    Sturdy,
    // 绿：CP 消耗减少 50%
    Pliant,
    // 深蓝：作业效率1.5倍
    Malleable,
    // 紫：技能效果持续增加两回合
    Primed,
}

impl From<&Condition> for &str {
    fn from(c: &Condition) -> Self {
        match c {
            Condition::Normal => "normal",
            Condition::Good => "good",
            Condition::Excellent => "excellent",
            Condition::Poor => "poor",
            Condition::Centered => "centered",
            Condition::Sturdy => "sturdy",
            Condition::Pliant => "pliant",
            Condition::Malleable => "malleable",
            Condition::Primed => "primed",
        }
    }
}

impl TryFrom<&str> for Condition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "normal" => Condition::Normal,
            "good" => Condition::Good,
            "excellent" => Condition::Excellent,
            "poor" => Condition::Poor,
            "centered" => Condition::Centered,
            "sturdy" => Condition::Sturdy,
            "pliant" => Condition::Pliant,
            "malleable" => Condition::Malleable,
            "primed" => Condition::Primed,
            _ => return Err(()),
        })
    }
}

impl Condition {
    fn touch_ratio(&self) -> f32 {
        match self {
            Condition::Good => 1.5,
            Condition::Excellent => 4.0,
            Condition::Poor => 0.5,
            _ => 1.0,
        }
    }

    fn synth_ratio(&self) -> f32 {
        match self {
            Condition::Malleable => 1.5,
            _ => 1.0,
        }
    }
}

/// 玩家装备属性
#[derive(Copy, Clone)]
pub struct Attributes {
    /// 玩家等级
    pub level: u8,
    /// 制作精度
    pub craftsmanship: i32,
    /// 加工精度
    pub control: i32,
    /// 制作力
    pub craft_points: i32,
}

impl Attributes {
    fn level_value(&self) -> i32 { data::level_table(self.level) }
}

/// 储存了一次制作中配方的信息。
#[derive(Copy, Clone)]
pub struct Recipe {
    /// 配方等级
    pub rlv: i32,

    /// 制作配方所需的玩家等级
    pub job_level: u8,

    /// 难度（最大进展）
    pub difficulty: u16,

    /// 最高品质
    pub quality: i32,

    /// 耐久
    pub durability: i32,

    /// 制作状态标志位，用于表示本次制作有可能出现哪些球色。
    /// 该字段从低到高每个bit依次表示对应Condition中的状态是否会出现
    ///
    /// Example:
    /// ```rust
    /// use ffxiv_crafting::Condition;
    ///
    /// fn belong(cond_flag: i32, cond: Condition) -> bool {
    ///     cond_flag & (1 << cond as usize) != 0
    /// }
    ///
    /// let cond_flag = 15; // 15即0b00001111，表示只有可能出现白球、红球、彩球和黑球
    /// assert_eq!(belong(cond_flag, Condition::Normal), true);
    /// assert_eq!(belong(cond_flag, Condition::Good), true);
    /// assert_eq!(belong(cond_flag, Condition::Excellent), true);
    /// assert_eq!(belong(cond_flag, Condition::Poor), true);
    ///
    /// assert_eq!(belong(cond_flag, Condition::Centered), false);
    /// assert_eq!(belong(cond_flag, Condition::Sturdy), false);
    /// assert_eq!(belong(cond_flag, Condition::Pliant), false);
    /// assert_eq!(belong(cond_flag, Condition::Malleable), false);
    /// assert_eq!(belong(cond_flag, Condition::Primed), false);
    /// ```
    pub conditions_flag: u16,

    progress_divider: u8,
    quality_divider: u8,
    progress_modifier: u8,
    quality_modifier: u8,
}

impl Recipe {
    pub fn new(rlv: i32, difficulty_factor: u16, quality_factor: u16, durability_factor: u16) -> Self {
        let rt = data::recipe_level_table(rlv);
        Self {
            rlv,
            job_level: rt.class_job_level,
            difficulty: (rt.difficulty as u32 * difficulty_factor as u32 / 100) as u16,
            quality: rt.quality as i32 * quality_factor as i32 / 100,
            durability: rt.durability as i32 * durability_factor as i32 / 100,
            conditions_flag: rt.conditions_flag,
            progress_divider: rt.progress_divider,
            quality_divider: rt.quality_divider,
            progress_modifier: rt.progress_modifier,
            quality_modifier: rt.quality_modifier,
        }
    }
}

/// Buffs 储存了一次制作中玩家全部buff状态信息
#[derive(Copy, Clone, Default)]
pub struct Buffs {
    /// 坚信
    pub muscle_memory: Option<DurationBuff>,
    /// 阔步
    pub great_strides: Option<DurationBuff>,
    /// 崇敬
    pub veneration: Option<DurationBuff>,
    /// 改革
    pub innovation: Option<DurationBuff>,
    /// 内静
    pub inner_quiet: u8,
    /// 最终确认
    pub final_appraisal: Option<DurationBuff>,
    /// 掌握
    pub manipulation: Option<DurationBuff>,
    /// 俭约 OR 长期俭约
    pub wast_not: Option<DurationBuff>,
    /// 中级加工预备
    /// 假想buff，用于处理 加工-中级加工 的连击。
    pub standard_touch_prepared: Option<DurationBuff>,
    /// 观察（注视预备）
    /// 假想buff，用于处理 观察-注释制作 OR 观察-注视加工 的连击。
    pub observed: Option<DurationBuff>,
}

/// StacksBuff 代表拥有层数的buff，且层数不会随着制作回合衰减，如内静。
#[derive(Copy, Clone, Debug)]
pub struct StacksBuff(pub usize);

/// DurationBuff 代表拥有剩余时长的buff，该时长会随着制作回合而递减，例如改革、崇敬等等。
#[derive(Copy, Clone, Debug)]
pub struct DurationBuff(pub usize);

fn fade(b: &mut Option<DurationBuff>) {
    if let Some(DurationBuff(d)) = b {
        if *d > 1 {
            *d -= 1
        } else {
            *b = None
        }
    }
}

fn round_down(v: f32, scale: f32) -> f32 {
    (v * scale).floor() / scale
}

impl Buffs {
    pub(crate) fn synthesis(&mut self, skill_e: f32) -> f32 {
        let mut e = 0.0;
        if self.muscle_memory.take().is_some() {
            e += 1.0;
        }
        if self.veneration.is_some() {
            e += 0.5;
        }
        skill_e * round_down(1.0 + e, 100.0)
    }

    pub(crate) fn touch(&mut self, skill_e: f32) -> f32 {
        let mut bm = 1.0;
        if self.great_strides.take().is_some() {
            bm += 1.0;
        }
        if self.innovation.is_some() {
            bm += 0.5;
        }
        let iq = 1.0 + self.inner_quiet as f32 * 0.1;
        skill_e * round_down(iq * bm, 100.0)
    }

    pub(crate) fn fade(&mut self) {
        fade(&mut self.muscle_memory);
        fade(&mut self.great_strides);
        fade(&mut self.veneration);
        fade(&mut self.innovation);
        fade(&mut self.final_appraisal);
        fade(&mut self.manipulation);
        fade(&mut self.wast_not);
        fade(&mut self.standard_touch_prepared);
        fade(&mut self.observed);
    }
}

/// Status 储存一次制作模拟所需的全部状态信息
#[derive(Copy, Clone)]
pub struct Status {
    /// 玩家当前身上的buff
    pub buffs: Buffs,
    /// 玩家的装备属性
    pub attributes: Attributes,
    /// 本次制作配方
    pub recipe: Recipe,

    /// 剩余耐久
    pub durability: i32,
    /// 剩余制作力
    pub craft_points: i32,
    /// 进展
    pub progress: u16,
    /// 品质
    pub quality: i32,

    /// 步数
    pub step: i32,
    /// 制作状态
    pub condition: Condition,
}

/// 技能释放错误
#[derive(Debug)]
pub enum CastActionError {
    /// 耐久不足
    DurabilityNotEnough,
    /// 制作力不足
    CraftPointNotEnough,
    /// 制作已完成无法发动技能
    CraftingAlreadyFinished,
    /// 该技能尚未学会，玩家等级不足
    PlayerLevelTooLow,
    /// 该技能只有在“高品质”及以上的状态下才能使用
    RequireGoodOrExcellent,
    /// 该技能在俭约及长期简约状态下无法使用
    NotAllowedInWastNotBuff,
    /// 该技能仅可在首次作业时发动
    OnlyAllowedInFirstStep,
    /// 该技能仅可在首次作业且用于等级低了10级及以上的配方时发动
    LevelGapMustGreaterThanTen,
    /// 该技能只有在内静的档数大于1时才可以使用
    RequireInnerQuiet,
}

impl Display for CastActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CastActionError::DurabilityNotEnough => "durability not enough",
            CastActionError::CraftPointNotEnough => "craft point not enough",
            CastActionError::CraftingAlreadyFinished => "crafting already finished",
            CastActionError::PlayerLevelTooLow => "player level too low",
            CastActionError::RequireGoodOrExcellent => "require good or excellent",
            CastActionError::NotAllowedInWastNotBuff => "not allowed in wast not buff",
            CastActionError::OnlyAllowedInFirstStep => "only allowed in first step",
            CastActionError::LevelGapMustGreaterThanTen => "level gap must greater than 10",
            CastActionError::RequireInnerQuiet => "require at least 1 stack of inner quiet",
        })
    }
}

impl Error for CastActionError {}

impl Status {
    pub fn new(attributes: Attributes, recipe: Recipe) -> Self {
        Status {
            buffs: Buffs::default(),
            attributes,
            recipe,
            durability: recipe.durability,
            craft_points: attributes.craft_points,
            condition: Condition::Normal,
            progress: 0,
            quality: 0,
            step: 0,
        }
    }

    fn calc_durability(&self, durability: i32) -> i32 {
        let mut reduce = durability;
        if let Condition::Sturdy = self.condition {
            reduce -= reduce / 2;
        }
        if self.buffs.wast_not.is_some() {
            reduce -= reduce / 2;
        }
        reduce
    }

    fn consume_durability(&mut self, durability: i32) {
        self.durability -= self.calc_durability(durability);
    }

    fn consume_craft_point(&mut self, cp: i32) {
        let mut reduce = cp;
        if let Condition::Pliant = self.condition {
            reduce -= reduce / 2;
        }
        self.craft_points -= reduce;
    }

    pub fn calc_synthesis(&self, efficiency: f32) -> u16 {
        let mut base = self.attributes.craftsmanship as f32 * 10.0
            / self.recipe.progress_divider as f32 + 2.0;
        if self.attributes.level_value() <= self.recipe.rlv {
            base *= self.recipe.progress_modifier as f32 * 0.01
        }
        (base.floor() * self.condition.synth_ratio() * efficiency) as u16
    }

    pub fn calc_touch(&self, efficiency: f32) -> i32 {
        let mut base = self.attributes.control as f32 * 10.0
            / self.recipe.quality_divider as f32 + 35.0;
        if self.attributes.level_value() <= self.recipe.rlv {
            base *= self.recipe.quality_modifier as f32 * 0.01
        }
        (base.floor() * self.condition.touch_ratio() * efficiency) as i32
    }

    fn cast_synthesis(&mut self, cp: i32, durability: i32, efficiency: f32) {
        let e = self.buffs.synthesis(efficiency);
        self.progress += self.calc_synthesis(e);
        self.consume_durability(durability);
        self.consume_craft_point(cp);
        if self.progress >= self.recipe.difficulty {
            self.progress = if self.buffs.final_appraisal.take().is_some() {
                self.recipe.difficulty - 1
            } else {
                self.recipe.difficulty
            }
        }
    }

    fn cast_touch(&mut self, cp: i32, durability: i32, efficiency: f32, inner_quiet_addon: i8) {
        let e = self.buffs.touch(efficiency);
        let quality_addon = self.calc_touch(e);
        self.quality = (self.quality + quality_addon).min(self.recipe.quality);
        self.consume_durability(durability);
        self.consume_craft_point(cp);
        self.buffs.inner_quiet = (self.buffs.inner_quiet.saturating_add_signed(inner_quiet_addon)).min(10);
    }

    pub fn new_duration_buff(&self, dt: usize) -> Option<DurationBuff> {
        let mut a = 1;
        if let Condition::Primed = self.condition {
            a += 2
        }
        Some(DurationBuff(dt + a))
    }

    /// 发动一次技能。
    pub fn cast_action(&mut self, action: Skills) {
        match action {
            Skills::BasicSynthesis => {
                self.cast_synthesis(0, 10, if self.attributes.level < 31 { 1.0 } else { 1.2 })
            }
            Skills::RapidSynthesis => {
                self.cast_synthesis(0, 10, if self.attributes.level < 63 { 2.5 } else { 5.0 })
            }
            Skills::CarefulSynthesis => {
                self.cast_synthesis(7, 10, if self.attributes.level < 82 { 1.5 } else { 1.8 })
            }
            Skills::FocusedSynthesis => self.cast_synthesis(5, 10, 2.0),
            Skills::Groundwork => {
                let mut e = if self.attributes.level < 86 { 3.0 } else { 3.6 };
                let d = self.calc_durability(20);
                if self.durability < d { e *= 0.5 };
                self.cast_synthesis(18, 20, e)
            }
            Skills::IntensiveSynthesis => self.cast_synthesis(6, 10, 4.0),

            Skills::DelicateSynthesis => {
                self.cast_synthesis(16, 0, 1.0);
                self.cast_touch(16, 10, 1.0, 1);
            }

            Skills::BasicTouch => {
                self.buffs.standard_touch_prepared = Some(DurationBuff(1 + 1));
                self.cast_touch(18, 10, 1.0, 1)
            }
            Skills::HastyTouch => self.cast_touch(0, 10, 1.0, 1),
            Skills::StandardTouch => {
                let cp = match self.buffs.standard_touch_prepared {
                    None => 32,
                    Some(_) => 18,
                };
                self.cast_touch(cp, 10, 1.25, 1)
            }
            Skills::ByregotsBlessing => {
                let e = (1.0 + self.buffs.inner_quiet as f32 * 0.2).max(3.0);
                self.cast_touch(24, 10, e, -(self.buffs.inner_quiet as i8));
            }
            Skills::PreciseTouch => self.cast_touch(18, 10, 1.5, 2),
            Skills::PrudentTouch => self.cast_touch(25, 5, 1.0, 1),
            Skills::FocusedTouch => self.cast_touch(18, 10, 1.5, 1),
            Skills::PreparatoryTouch => self.cast_touch(40, 20, 2.0, 2),

            Skills::TricksOfTheTrade => {
                self.craft_points = (self.craft_points + 20).min(self.attributes.craft_points);
            }

            Skills::MastersMend => {
                self.consume_craft_point(88);
                self.durability = self.recipe.durability.min(self.durability + 30);
            }
            Skills::WasteNot => {
                self.consume_craft_point(56);
                self.buffs.wast_not = self.new_duration_buff(4);
            }
            Skills::WasteNotII => {
                self.consume_craft_point(98);
                self.buffs.wast_not = self.new_duration_buff(8);
            }
            Skills::Manipulation => {
                self.consume_craft_point(96);
                self.buffs.manipulation = self.new_duration_buff(8);
            }
            Skills::MuscleMemory => {
                self.cast_synthesis(6, 10, 3.0);
                self.buffs.muscle_memory = self.new_duration_buff(5);
            }
            Skills::Reflect => {
                self.cast_touch(24, 10, 1.0, 2);
            }
            Skills::TrainedEye => {
                self.consume_craft_point(250);
                self.quality += self.recipe.quality;
            }
            Skills::Veneration => {
                self.consume_craft_point(18);
                self.buffs.veneration = self.new_duration_buff(4);
            }
            Skills::GreatStrides => {
                self.consume_craft_point(32);
                self.buffs.great_strides = self.new_duration_buff(3);
            }
            Skills::Innovation => {
                self.consume_craft_point(18);
                self.buffs.innovation = self.new_duration_buff(4);
            }
            Skills::Observe => {
                self.consume_craft_point(7);
                self.buffs.observed = Some(DurationBuff(1 + 1));
            }
            Skills::FinalAppraisal => {
                self.consume_craft_point(1);
                self.buffs.final_appraisal = self.new_duration_buff(5);
                return;
            }
            Skills::AdvancedTouch => todo!(),
            Skills::PrudentSynthesis => todo!(),
            Skills::TrainedFinesse => todo!(),
            Skills::CarefulObservation => {
                self.condition = Condition::Normal;
                return;
            }
            Skills::HeartAndSoul => todo!(),
        }
        if !matches!(action, Skills::Manipulation)
            && self.buffs.manipulation.is_some()
            && self.durability > 0
        {
            self.durability = self.recipe.durability.min(self.durability + 5);
        }
        self.buffs.fade();
        self.condition = Condition::Normal;
        self.step += 1;
    }

    /// 发动一次技能，并且失败。
    pub fn fail_action(&mut self, action: Skills) {
        match action {
            Skills::HastyTouch => self.consume_durability(10),
            Skills::RapidSynthesis => self.consume_durability(10),
            Skills::FocusedSynthesis => {
                self.consume_durability(10);
                self.consume_craft_point(5);
            }
            Skills::FocusedTouch => {
                self.consume_durability(10);
                self.consume_craft_point(18);
            }
            _ => panic!("action {:?} never fail", action),
        }
        if self.durability > 0 && self.buffs.manipulation.is_some() {
            self.durability = self.recipe.durability.min(self.durability + 5);
        }
        self.buffs.fade();
        self.condition = Condition::Normal;
        self.step += 1;
    }

    /// 计算当前状态下某技能的成功概率，返回结果介于[0_f32..=1_f32]之间。
    pub fn success_rate(&self, action: Skills) -> f32 {
        let addon = match self.condition {
            Condition::Centered => 0.25,
            _ => 0.0,
        };
        addon
            + match action {
            Skills::HastyTouch => 0.6,
            Skills::RapidSynthesis => 0.5,
            Skills::FocusedSynthesis | Skills::FocusedTouch => {
                if self.buffs.observed.is_some() {
                    1.0
                } else {
                    0.5
                }
            }
            _ => return 1.0,
        }
    }

    /// 当前状态是否允许发动某技能。
    pub fn is_action_allowed(&self, action: Skills) -> Result<(), CastActionError> {
        use CastActionError::{
            CraftPointNotEnough, CraftingAlreadyFinished, DurabilityNotEnough,
            LevelGapMustGreaterThanTen, NotAllowedInWastNotBuff,
            OnlyAllowedInFirstStep, PlayerLevelTooLow, RequireGoodOrExcellent,
        };

        let cp = {
            let mut reduce = 0;
            let cp = action.craft_point();
            if let Condition::Pliant = self.condition {
                reduce += cp / 2;
            }
            cp - reduce
        };

        match action {
            _ if action.unlock_level() > self.attributes.level => Err(PlayerLevelTooLow),

            Skills::TricksOfTheTrade | Skills::IntensiveSynthesis | Skills::PreciseTouch
            if !matches!(self.condition, Condition::Good | Condition::Excellent) => Err(RequireGoodOrExcellent),

            Skills::PrudentTouch if self.buffs.wast_not.is_some() => Err(NotAllowedInWastNotBuff),

            Skills::MuscleMemory | Skills::Reflect | Skills::TrainedEye if self.step != 0 => {
                Err(OnlyAllowedInFirstStep)
            }

            Skills::TrainedEye if self.attributes.level - self.recipe.job_level < 10 => {
                Err(LevelGapMustGreaterThanTen)
            }

            Skills::ByregotsBlessing if self.buffs.inner_quiet < 1 => Err(RequireInnerQuiet),

            _ if self.durability <= 0 => Err(DurabilityNotEnough),
            _ if cp > self.craft_points => Err(CraftPointNotEnough),
            _ if self.progress >= self.recipe.difficulty => Err(CraftingAlreadyFinished),
            _ => Ok(()),
        }
    }

    /// 本次制作是否已经结束。
    pub fn is_finished(&self) -> bool {
        self.progress >= self.recipe.difficulty || self.durability <= 0
    }

    /// 计算当前状态的HQ概率。
    /// 返回一个百分数，即：
    /// 如果返回89，则代表概率为89%。
    ///
    /// Calculate the HQ probability of current status.
    /// The return value is a percentage, that is,
    /// if 89 is returned, it means that the probability is 89%.
    pub fn high_quality_probability(&self) -> Option<i32> {
        let percent = self.quality * 100 / self.recipe.quality;
        match percent {
            0 => Some(1),
            5 => Some(2),
            9 => Some(3),
            13 => Some(4),
            17 => Some(5),
            21 => Some(6),
            25 => Some(7),
            29 => Some(8),
            32 => Some(9),
            35 => Some(10),
            38 => Some(11),
            41 => Some(12),
            44 => Some(13),
            47 => Some(14),
            50 => Some(15),
            53 => Some(16),
            55 => Some(17),
            58 => Some(18),
            61 => Some(19),
            63 => Some(20),
            65 => Some(21),
            66 => Some(22),
            67 => Some(23),
            68 => Some(24),
            69 => Some(26),
            70 => Some(28),
            71 => Some(31),
            72 => Some(34),
            73 => Some(38),
            74 => Some(42),
            75 => Some(47),
            76 => Some(52),
            77 => Some(58),
            78 => Some(64),
            79 => Some(68),
            80 => Some(71),
            81 => Some(74),
            82 => Some(76),
            83 => Some(78),
            84 => Some(80),
            85 => Some(81),
            86 => Some(82),
            87 => Some(83),
            88 => Some(84),
            89 => Some(85),
            90 => Some(86),
            91 => Some(87),
            92 => Some(88),
            93 => Some(89),
            94 => Some(90),
            95 => Some(91),
            96 => Some(92),
            97 => Some(94),
            98 => Some(96),
            99 => Some(98),
            100 => Some(100),
            _ => None,
        }
    }
}

/// 用于根据cond_flag和玩家等级计算各个球色出现概率的迭代器
///
/// Examples:
///
/// ```rust
/// use ffxiv_crafting::{Condition, ConditionIterator};
/// // 该配方的cond_flag为15，玩家等级为80。
/// for (c, p) in ConditionIterator::new(15, 80) {
///     println!("出现 {:?} 的概率为: {}", c, p);
/// }
/// ```
///
pub struct ConditionIterator {
    flag: i32,
    rate: f32,
    good_chance: f32,
    step: Option<Condition>,
}

impl ConditionIterator {
    pub fn new(flag: i32, level: i32) -> Self {
        Self {
            flag,
            rate: 0.0,
            good_chance: [0.2, 0.25][(level >= 63) as usize],
            step: Some(Condition::Good),
        }
    }

    fn next_cond(cond: Condition) -> Option<Condition> {
        match cond {
            Condition::Good => Some(Condition::Excellent),
            Condition::Excellent => Some(Condition::Poor),
            Condition::Poor => Some(Condition::Centered),
            Condition::Centered => Some(Condition::Sturdy),
            Condition::Sturdy => Some(Condition::Pliant),
            Condition::Pliant => Some(Condition::Malleable),
            Condition::Malleable => Some(Condition::Primed),
            Condition::Primed => Some(Condition::Normal),
            Condition::Normal => None,
        }
    }
}

impl Iterator for ConditionIterator {
    type Item = (Condition, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let cond = match self.step {
            Some(cond) => {
                let mut cond = cond;
                while self.flag & (1 << cond as i32) == 0 {
                    cond = match Self::next_cond(cond) {
                        Some(cond) => cond,
                        None => return None,
                    };
                }
                cond
            }
            None => return None,
        };
        self.step = Self::next_cond(cond);
        let expert = self.flag & (1 << Condition::Excellent as i32) == 0;
        let rate = match cond {
            Condition::Good => [self.good_chance, 0.12][expert as usize],
            Condition::Excellent => [0.04, 0.0][expert as usize],
            Condition::Poor => 0.0,
            Condition::Centered => 0.15,
            Condition::Sturdy => 0.15,
            Condition::Pliant => 0.12,
            Condition::Malleable => 0.12,
            Condition::Primed => 0.12,
            Condition::Normal => 1.0 - self.rate,
        };
        self.rate += rate;
        Some((cond, rate))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Condition, data};

    use super::{Attributes, Recipe, Skills, Status};

    #[test]
    fn basic_synth() {
        let attr = Attributes {
            level: 80,
            craftsmanship: 2806,
            control: 2784,
            craft_points: 548,
        };
        let recipe = Recipe::new(517, 50, 100, 50);
        let mut s = Status::new(attr, recipe);

        let result = [279, 558, 837, 1000];
        for &pg in &result {
            s.cast_action(Skills::BasicSynthesis);
            assert_eq!(s.progress, pg);
        }
    }


    #[test]
    fn delicate_synth() {
        let attr = Attributes {
            level: 80,
            craftsmanship: 2762,
            control: 2794,
            craft_points: 539,
        };
        let recipe = Recipe::new(517, 100, 100, 100);
        let mut s = Status::new(attr, recipe);

        struct Step {
            a: Skills,
            pg: u16,
            qu: i32,
            du: i32,
            co: u8,
        }
        for (i, step) in [
            Step { a: Skills::DelicateSynthesis, pg: 230, qu: 301, du: 70, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 460, qu: 632, du: 60, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 690, qu: 993, du: 50, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 920, qu: 1384, du: 40, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 1150, qu: 1805, du: 30, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 1380, qu: 2256, du: 20, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 1610, qu: 2737, du: 10, co: 1 },
            Step { a: Skills::DelicateSynthesis, pg: 1840, qu: 3248, du: 0, co: 1 },
        ].iter().enumerate() {
            s.cast_action(step.a);
            assert_eq!(s.progress, step.pg, "step [{}] progress simulation fail: want {}, get {}", i, step.pg, s.progress);
            assert_eq!(s.quality, step.qu, "step [{}] quality simulation fail: want {}, get {}", i, step.qu, s.quality);
            assert_eq!(s.durability, step.du, "step [{}] durability simulation fail: want {}, get {}", i, step.du, s.durability);
            s.condition = map_cond(step.co);
        }
    }

    #[test]
    fn phrygian_ear_cuffs_of_healing() {
        let attr = Attributes {
            level: 82,
            craftsmanship: 2786,
            control: 2764,
            craft_points: 533,
        };
        let recipe = Recipe::new(535, 100, 100, 100);
        let mut s = Status::new(attr, recipe);

        struct Step {
            a: i32,
            pg: u16,
            qu: i32,
            du: i32,
            co: u8,
        }
        for (i, step) in [
            Step { a: 100390, pg: 0, qu: 288, du: 70, co: 2 },
            Step { a: 100131, pg: 0, qu: 1065, du: 60, co: 1 },
            Step { a: 4577, pg: 0, qu: 1065, du: 60, co: 1 },
            Step { a: 100230, pg: 0, qu: 1468, du: 60, co: 3 },
            Step { a: 100302, pg: 0, qu: 4924, du: 45, co: 4 },
            Step { a: 100082, pg: 0, qu: 4924, du: 50, co: 1 },
            Step { a: 100246, pg: 0, qu: 5658, du: 45, co: 1 },
            Step { a: 100342, pg: 0, qu: 6700, du: 40, co: 1 },
            Step { a: 19300, pg: 0, qu: 6700, du: 45, co: 1 },
            Step { a: 100075, pg: 403, qu: 6700, du: 40, co: 1 },
            Step { a: 100075, pg: 806, qu: 6700, du: 35, co: 1 },
            Step { a: 100075, pg: 1209, qu: 6700, du: 25, co: 1 },
            Step { a: 100075, pg: 1612, qu: 6700, du: 15, co: 1 },
            Step { a: 100077, pg: 1612, qu: 6700, du: 45, co: 1 },
            Step { a: 19300, pg: 1612, qu: 6700, du: 45, co: 1 },
            Step { a: 100082, pg: 1612, qu: 6700, du: 45, co: 1 },
            Step { a: 100246, pg: 1612, qu: 6700, du: 35, co: 1 },
            Step { a: 100082, pg: 1612, qu: 6700, du: 35, co: 1 },
            Step { a: 100238, pg: 2284, qu: 6700, du: 25, co: 1 },
            Step { a: 100082, pg: 2284, qu: 6700, du: 25, co: 1 },
            Step { a: 100238, pg: 2732, qu: 6700, du: 15, co: 1 },
            Step { a: 100082, pg: 2732, qu: 6700, du: 15, co: 1 },
            Step { a: 100238, pg: 3000, qu: 6700, du: 5, co: 1 },
        ].iter().enumerate() {
            let skill = data::action_table(step.a).unwrap();
            println!("casting: {:?}", skill);
            s.cast_action(skill);
            assert_eq!(s.progress, step.pg, "step [{:?}] progress simulation fail: want {}, get {}", skill, step.pg, s.progress);
            assert_eq!(s.quality, step.qu, "step [{:?}] quality simulation fail: want {}, get {}", skill, step.qu, s.quality);
            assert_eq!(s.durability, step.du, "step [{:?}] durability simulation fail: want {}, get {}", skill, step.du, s.durability);
            s.condition = map_cond(step.co);
        }
    }

    #[test]
    fn resplendent() {
        let attr = Attributes {
            level: 82,
            craftsmanship: 2786,
            control: 2764,
            craft_points: 533,
        };
        let recipe = Recipe::new(516, 100, 100, 86);
        let mut s = Status::new(attr, recipe);

        struct Step {
            a: i32,
            pg: u16,
            qu: i32,
            du: i32,
            co: u8,
            su: bool,
        }
        for (i, step) in [
            Step { a: 100390, pg: 0, qu: 247, du: 50, co: 9, su: true },
            Step { a: 19300, pg: 0, qu: 247, du: 50, co: 1, su: true },
            Step { a: 100366, pg: 0, qu: 247, du: 40, co: 2, su: false },
            Step { a: 100318, pg: 1206, qu: 247, du: 30, co: 6, su: true },
            Step { a: 100366, pg: 1206, qu: 247, du: 25, co: 7, su: false },
            Step { a: 4577, pg: 1206, qu: 247, du: 25, co: 5, su: true },
            Step { a: 100366, pg: 2713, qu: 247, du: 20, co: 6, su: true },
            Step { a: 100366, pg: 2713, qu: 247, du: 20, co: 2, su: false },
            Step { a: 100131, pg: 2713, qu: 913, du: 15, co: 5, su: true },
            Step { a: 100358, pg: 2713, qu: 913, du: 10, co: 6, su: false },
            Step { a: 100358, pg: 2713, qu: 1258, du: 10, co: 6, su: true },
            Step { a: 100358, pg: 2713, qu: 1628, du: 10, co: 1, su: true },
            Step { a: 100082, pg: 2713, qu: 1628, du: 15, co: 1, su: true },
            Step { a: 100246, pg: 2713, qu: 2220, du: 10, co: 2, su: true },
            Step { a: 100374, pg: 2713, qu: 2220, du: 10, co: 8, su: true },
            Step { a: 100077, pg: 2713, qu: 2220, du: 40, co: 6, su: true },
            Step { a: 100366, pg: 3718, qu: 2220, du: 35, co: 2, su: true },
            Step { a: 100131, pg: 3718, qu: 3164, du: 25, co: 2, su: true },
            Step { a: 100131, pg: 3718, qu: 4219, du: 15, co: 7, su: true },
            Step { a: 100077, pg: 3718, qu: 4219, du: 45, co: 1, su: true },
            Step { a: 100366, pg: 3718, qu: 4219, du: 35, co: 1, su: false },
            Step { a: 100366, pg: 4723, qu: 4219, du: 25, co: 2, su: true },
            Step { a: 100374, pg: 4723, qu: 4219, du: 25, co: 6, su: true },
            Step { a: 100358, pg: 4723, qu: 4219, du: 20, co: 1, su: false },
            Step { a: 19007, pg: 4723, qu: 4219, du: 20, co: 6, su: true },
            Step { a: 100082, pg: 4723, qu: 4219, du: 20, co: 7, su: true },
            Step { a: 100246, pg: 4723, qu: 5330, du: 10, co: 9, su: true },
            Step { a: 100077, pg: 4723, qu: 5330, du: 40, co: 5, su: true },
            Step { a: 100082, pg: 4723, qu: 5330, du: 40, co: 6, su: true },
            Step { a: 100246, pg: 4723, qu: 6071, du: 35, co: 8, su: true },
            Step { a: 100075, pg: 5084, qu: 6071, du: 25, co: 8, su: true },
            Step { a: 100075, pg: 5445, qu: 6071, du: 15, co: 2, su: true },
            Step { a: 100374, pg: 5445, qu: 6071, du: 15, co: 2, su: true },
            Step { a: 100374, pg: 5445, qu: 6071, du: 15, co: 9, su: true },
            Step { a: 263, pg: 5445, qu: 6071, du: 15, co: 9, su: true },
            Step { a: 19007, pg: 5445, qu: 6071, du: 15, co: 5, su: true },
            Step { a: 100082, pg: 5445, qu: 6071, du: 15, co: 1, su: true },
            Step { a: 100246, pg: 5445, qu: 7923, du: 5, co: 8, su: true },
            Step { a: 100077, pg: 5445, qu: 7923, du: 35, co: 1, su: true },
            Step { a: 100082, pg: 5445, qu: 7923, du: 35, co: 1, su: true },
            Step { a: 100082, pg: 5445, qu: 7923, du: 35, co: 1, su: true },
            Step { a: 100075, pg: 5470, qu: 7923, du: 25, co: 1, su: true },
        ].iter().enumerate() {
            let skill = data::action_table(step.a).unwrap();
            println!("casting: {:?}", skill);
            if step.su {
                s.cast_action(skill);
            } else {
                s.fail_action(skill);
            }
            assert_eq!(s.progress, step.pg, "step [{:?}] progress simulation fail: want {}, get {}", skill, step.pg, s.progress);
            assert_eq!(s.quality, step.qu, "step [{:?}] quality simulation fail: want {}, get {}", skill, step.qu, s.quality);
            assert_eq!(s.durability, step.du, "step [{:?}] durability simulation fail: want {}, get {}", skill, step.du, s.durability);
            s.condition = map_cond(step.co);
        }
    }

    fn map_cond(c: u8) -> Condition {
        match c {
            1 => Condition::Normal,
            2 => Condition::Good,
            3 => Condition::Excellent,
            4 => Condition::Poor,
            5 => Condition::Centered,
            6 => Condition::Sturdy,
            7 => Condition::Pliant,
            8 => Condition::Malleable,
            9 => Condition::Primed,
            _ => unreachable!()
        }
    }
}
