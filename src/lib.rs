#![feature(mixed_integer_ops)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[cfg(feature = "serde-support")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub mod data;
pub mod export;

/// 代表一个玩家在作业时可以使用的一个技能的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    // fake skills
    RapidSynthesisFail,
    HastyTouchFail,
    FocusedSynthesisFail,
    FocusedTouchFail,
}

#[cfg(feature = "serde-support")]
impl Serialize for Skills {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "serde-support")]
impl<'de> Deserialize<'de> for Skills {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct StrVisitor;
        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Skills;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Skills::try_from(v).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}

impl Skills {
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
            // fake skills
            Skills::RapidSynthesisFail => 9,
            Skills::HastyTouchFail => 9,
            Skills::FocusedSynthesisFail => 67,
            Skills::FocusedTouchFail => 68,
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
            // fake skills
            Skills::RapidSynthesisFail => "rapid_synthsis_fail",
            Skills::HastyTouchFail => "hasty_touch_fail",
            Skills::FocusedSynthesisFail => "focused_synthesis_fail",
            Skills::FocusedTouchFail => "focused_touch_fail",
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
            // fake skills
            "rapid_synthesis_fail" => Skills::RapidSynthesisFail,
            "hasty_touch_fail" => Skills::HastyTouchFail,
            "focused_synthesis_fail" => Skills::FocusedSynthesisFail,
            "focused_touch_fail" => Skills::FocusedTouchFail,
            _ => return Err(UnknownSkillErr),
        })
    }
}

#[derive(Debug)]
pub struct UnknownSkillErr;

impl Error for UnknownSkillErr {}

impl Display for UnknownSkillErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown skill name")
    }
}

#[cfg(feature = "serde-support")]
impl de::Error for UnknownSkillErr {
    fn custom<T: Display>(_msg: T) -> Self {
        Self
    }
}

/// 代表了当前的“制作状态”，也就是俗称的球色。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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
    fn level_value(&self) -> i32 {
        data::level_table(self.level)
    }
}

/// 储存了一次制作中配方的信息。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Recipe {
    /// 配方等级
    pub rlv: i32,

    /// 制作配方所需的玩家等级
    pub job_level: u8,

    /// 难度（最大进展）
    pub difficulty: u16,

    /// 最高品质
    pub quality: u32,

    /// 耐久
    pub durability: u16,

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
}

impl Recipe {
    pub fn new(
        rlv: i32,
        difficulty_factor: u16,
        quality_factor: u16,
        durability_factor: u16,
    ) -> Self {
        let rt = data::recipe_level_table(rlv);
        Self {
            rlv,
            job_level: rt.class_job_level,
            difficulty: (rt.difficulty as u32 * difficulty_factor as u32 / 100) as u16,
            quality: rt.quality * quality_factor as u32 / 100,
            durability: rt.durability * durability_factor / 100,
            conditions_flag: rt.conditions_flag,
        }
    }
}

/// Buffs 储存了一次制作中玩家全部buff状态信息
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Default, Debug)]
pub struct Buffs {
    /// 坚信
    pub muscle_memory: u8,
    /// 阔步
    pub great_strides: u8,
    /// 崇敬
    pub veneration: u8,
    /// 改革
    pub innovation: u8,
    /// 内静
    pub inner_quiet: u8,
    /// 最终确认
    pub final_appraisal: u8,
    /// 掌握
    pub manipulation: u8,
    /// 俭约 OR 长期俭约
    pub wast_not: u8,
    /// 专心致志
    pub heart_and_soul: u8,
    /// 设计变动使用次数
    /// 假想buff，用于记录设计变动使用的次数
    pub careful_observation_used: u8,
    /// 禁止使用专心致志
    /// 假想buff，用于禁止使用专心致志
    pub heart_and_soul_used: u8,
    /// 加工连击状态：0无，1中级加工预备，2上级加工预备
    pub touch_combo_stage: u8,
    /// 观察（注视预备）
    /// 假想buff，用于处理 观察-注释制作 OR 观察-注视加工 的连击。
    pub observed: u8,
}

fn round_down(v: f32, scale: f32) -> f32 {
    (v * scale).floor() / scale
}

impl Buffs {
    pub(crate) fn synthesis(&self, skill_e: f32) -> f32 {
        let mut e = 0.0;
        if self.muscle_memory > 0 {
            e += 1.0;
        }
        if self.veneration > 0 {
            e += 0.5;
        }
        skill_e * round_down(1.0 + e, 100.0)
    }

    pub(crate) fn apply_synthesis(&mut self) {
        if self.muscle_memory > 0 {
            self.muscle_memory = 0;
        }
    }

    pub(crate) fn touch(&self, skill_e: f32) -> f32 {
        let mut bm = 1.0;
        if self.great_strides > 0 {
            bm += 1.0;
        }
        if self.innovation > 0 {
            bm += 0.5;
        }
        let iq = 1.0 + self.inner_quiet as f32 * 0.1;
        skill_e * bm * iq
    }

    pub(crate) fn apply_touch(&mut self) {
        if self.great_strides > 0 {
            self.great_strides = 0;
        }
    }

    pub(crate) fn next(&mut self) {
        self.muscle_memory = self.muscle_memory.saturating_sub(1);
        self.great_strides = self.great_strides.saturating_sub(1);
        self.veneration = self.veneration.saturating_sub(1);
        self.innovation = self.innovation.saturating_sub(1);
        self.final_appraisal = self.final_appraisal.saturating_sub(1);
        self.manipulation = self.manipulation.saturating_sub(1);
        self.wast_not = self.wast_not.saturating_sub(1);
        self.touch_combo_stage = self.touch_combo_stage.saturating_sub(2);
        self.observed = self.observed.saturating_sub(1);
    }
}

/// Status 储存一次制作模拟所需的全部状态信息
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Status {
    /// 玩家当前身上的buff
    pub buffs: Buffs,
    /// 玩家的装备属性
    pub attributes: Attributes,
    /// 本次制作配方
    pub recipe: Recipe,
    /// 预计算数据
    caches: Caches,

    /// 剩余耐久
    pub durability: u16,
    /// 剩余制作力
    pub craft_points: i32,
    /// 进展
    pub progress: u16,
    /// 品质
    pub quality: u32,

    /// 步数
    pub step: i32,
    /// 制作状态
    pub condition: Condition,
}

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
struct Caches {
    base_synth: f32,
    base_touch: f32,
}

impl Caches {
    fn new(attributes: &Attributes, recipe: &Recipe) -> Self {
        let rt = data::recipe_level_table(recipe.rlv);
        Self {
            base_synth: {
                let mut base =
                    attributes.craftsmanship as f32 * 10.0 / rt.progress_divider as f32 + 2.0;
                if attributes.level_value() <= recipe.rlv {
                    base *= rt.progress_modifier as f32 * 0.01
                }
                base.floor()
            },
            base_touch: {
                let mut base = attributes.control as f32 * 10.0 / rt.quality_divider as f32 + 35.0;
                if attributes.level_value() <= recipe.rlv {
                    base *= rt.quality_modifier as f32 * 0.01
                }
                base.floor()
            },
        }
    }
}

/// 技能释放错误
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
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
    RequireInnerQuiet1,
    /// 该技能只有在内静的档数为10时才可以使用
    RequireInnerQuiet10,
    /// 设计变动最多使用三次
    CarefulObservationUsed3,
    /// 专心致志一次制作只能使用一次
    HeartAndSoulUsed,
    /// 注视在观察之后无法失败
    FocusNeverFailsAfterObserved,
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
            CastActionError::RequireInnerQuiet1 => "require at least 1 stack of inner quiet",
            CastActionError::RequireInnerQuiet10 => "require 10 stack of inner quiet",
            CastActionError::CarefulObservationUsed3 => "careful observation can only use 3 times",
            CastActionError::HeartAndSoulUsed => "heart and soul can be only used once",
            CastActionError::FocusNeverFailsAfterObserved => "focus never fails after observed",
        })
    }
}

impl Error for CastActionError {}

impl Status {
    pub fn new(attributes: Attributes, recipe: Recipe) -> Self {
        Status {
            buffs: Buffs::default(),
            caches: Caches::new(&attributes, &recipe),
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

    fn calc_durability(&self, durability: u16) -> u16 {
        let mut reduce = durability;
        if let Condition::Sturdy = self.condition {
            reduce -= reduce / 2;
        }
        if self.buffs.wast_not > 0 {
            reduce -= reduce / 2;
        }
        reduce
    }

    fn consume_durability(&mut self, durability: u16) {
        self.durability = self
            .durability
            .saturating_sub(self.calc_durability(durability));
    }

    pub fn calc_synthesis(&self, efficiency: f32) -> u16 {
        (self.caches.base_synth * self.condition.synth_ratio() * self.buffs.synthesis(efficiency))
            as u16
    }

    pub fn calc_touch(&self, efficiency: f32) -> u32 {
        (self.caches.base_touch * self.buffs.touch(efficiency) * self.condition.touch_ratio())
            as u32
    }

    fn cast_synthesis(&mut self, durability: u16, efficiency: f32) {
        self.progress += self.calc_synthesis(efficiency);
        self.consume_durability(durability);
        self.buffs.apply_synthesis();
        if self.progress >= self.recipe.difficulty {
            self.progress = self.recipe.difficulty;
            if self.buffs.final_appraisal > 0 {
                self.progress -= 1;
                self.buffs.final_appraisal = 0;
            }
        }
    }

    fn cast_touch(&mut self, durability: u16, efficiency: f32, inner_quiet_addon: i8) {
        let quality_addon = self.calc_touch(efficiency);
        self.quality = (self.quality + quality_addon).min(self.recipe.quality);
        self.consume_durability(durability);
        self.buffs.apply_touch();
        self.buffs.inner_quiet = self
            .buffs
            .inner_quiet
            .saturating_add_signed(inner_quiet_addon)
            .min(10);
    }

    pub fn new_duration_buff(&self, dt: u8) -> u8 {
        if let Condition::Primed = self.condition {
            dt + 2 + 1
        } else {
            dt + 1
        }
    }

    /// 计算当前状态指定技能消耗的CP。
    /// 考虑连击与球色
    pub fn craft_point(&self, skill: Skills) -> i32 {
        let cp = match skill {
            Skills::BasicSynthesis => 0,
            Skills::BasicTouch => 18,
            Skills::MastersMend => 88,
            Skills::HastyTouch => 0,
            Skills::RapidSynthesis => 0,
            Skills::Observe => 7,
            Skills::TricksOfTheTrade => 0,
            Skills::WasteNot => 56,
            Skills::Veneration => 18,
            Skills::StandardTouch => {
                if self.buffs.touch_combo_stage == 1 {
                    18
                } else {
                    32
                }
            }
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
            Skills::AdvancedTouch => {
                if self.buffs.touch_combo_stage == 2 {
                    18
                } else {
                    46
                }
            }
            Skills::PrudentSynthesis => 18,
            Skills::TrainedFinesse => 32,
            Skills::CarefulObservation => 0,
            Skills::HeartAndSoul => 0,
            // fake skills
            Skills::RapidSynthesisFail => 0,
            Skills::HastyTouchFail => 0,
            Skills::FocusedSynthesisFail => 5,
            Skills::FocusedTouchFail => 18,
        };
        if let Condition::Pliant = self.condition {
            cp - cp / 2
        } else {
            cp
        }
    }

    /// 发动一次技能。
    pub fn cast_action(&mut self, action: Skills) {
        self.craft_points -= self.craft_point(action);
        match action {
            Skills::BasicSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 31 { 1.0 } else { 1.2 })
            }
            Skills::RapidSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 63 { 2.5 } else { 5.0 })
            }
            Skills::CarefulSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 82 { 1.5 } else { 1.8 })
            }
            Skills::FocusedSynthesis => self.cast_synthesis(10, 2.0),
            Skills::Groundwork => {
                let mut e = if self.attributes.level < 86 { 3.0 } else { 3.6 };
                let d = self.calc_durability(20);
                if self.durability < d {
                    e *= 0.5
                };
                self.cast_synthesis(20, e)
            }
            Skills::IntensiveSynthesis => {
                self.cast_synthesis(10, 4.0);
                self.buffs.heart_and_soul = 0;
            }
            Skills::PrudentSynthesis => self.cast_synthesis(5, 1.8),

            Skills::DelicateSynthesis => {
                self.cast_synthesis(0, 1.0);
                self.cast_touch(10, 1.0, 1);
            }

            Skills::BasicTouch => {
                self.cast_touch(10, 1.0, 1);
                self.buffs.touch_combo_stage = 1 + 2;
            }
            Skills::HastyTouch => self.cast_touch(10, 1.0, 1),
            Skills::StandardTouch => {
                if self.buffs.touch_combo_stage == 1 {
                    self.buffs.touch_combo_stage = 2 + 2;
                };
                self.cast_touch(10, 1.25, 1)
            }
            Skills::AdvancedTouch => self.cast_touch(10, 1.5, 1),
            Skills::ByregotsBlessing => {
                let e = (1.0 + self.buffs.inner_quiet as f32 * 0.2).min(3.0);
                self.cast_touch(10, e, -(self.buffs.inner_quiet as i8));
            }
            Skills::PreciseTouch => {
                self.cast_touch(10, 1.5, 2);
                self.buffs.heart_and_soul = 0;
            }
            Skills::PrudentTouch => self.cast_touch(5, 1.0, 1),
            Skills::FocusedTouch => self.cast_touch(10, 1.5, 1),
            Skills::PreparatoryTouch => self.cast_touch(20, 2.0, 2),
            Skills::TrainedFinesse => self.cast_touch(0, 1.0, 0),

            Skills::TricksOfTheTrade => {
                self.craft_points = (self.craft_points + 20).min(self.attributes.craft_points);
                self.buffs.heart_and_soul = 0;
            }

            Skills::MastersMend => {
                self.durability = self.recipe.durability.min(self.durability + 30);
            }
            Skills::WasteNot => {
                self.buffs.wast_not = self.new_duration_buff(4);
            }
            Skills::WasteNotII => {
                self.buffs.wast_not = self.new_duration_buff(8);
            }
            Skills::Manipulation => {
                self.buffs.manipulation = self.new_duration_buff(8);
                self.buffs.next();
                self.step += 1;
                return;
            }
            Skills::MuscleMemory => {
                self.cast_synthesis(10, 3.0);
                self.buffs.muscle_memory = self.new_duration_buff(5);
            }
            Skills::Reflect => {
                self.cast_touch(10, 1.0, 2);
            }
            Skills::TrainedEye => {
                self.quality += self.recipe.quality;
            }
            Skills::Veneration => {
                self.buffs.veneration = self.new_duration_buff(4);
            }
            Skills::GreatStrides => {
                self.buffs.great_strides = self.new_duration_buff(3);
            }
            Skills::Innovation => {
                self.buffs.innovation = self.new_duration_buff(4);
            }
            Skills::Observe => {
                self.buffs.observed = 2;
            }
            Skills::FinalAppraisal => {
                self.buffs.final_appraisal = 5;
                return;
            }
            Skills::CarefulObservation => {
                self.buffs.careful_observation_used += 1;
                return;
            }
            Skills::HeartAndSoul => {
                self.buffs.heart_and_soul = 1;
                self.buffs.heart_and_soul_used += 1;
                return;
            }
            // fake skills
            Skills::RapidSynthesisFail => self.consume_durability(10),
            Skills::HastyTouchFail => self.consume_durability(10),
            Skills::FocusedSynthesisFail => self.consume_durability(10),
            Skills::FocusedTouchFail => self.consume_durability(10),
        }
        if self.buffs.manipulation > 0 && self.durability > 0 {
            self.durability += 5;
            self.durability = self.durability.min(self.recipe.durability);
        }
        self.buffs.next();
        self.step += 1;
    }

    /// 计算当前状态下某技能的成功概率，返回结果介于[0..=100]之间。
    pub fn success_rate(&self, action: Skills) -> u8 {
        let addon = match self.condition {
            Condition::Centered => 25,
            _ => 0,
        };
        addon
            + match action {
            Skills::HastyTouch => 60,
            Skills::RapidSynthesis => 50,
            Skills::FocusedSynthesis | Skills::FocusedTouch => {
                if self.buffs.observed > 0 {
                    100
                } else {
                    50
                }
            }
            _ => return 100,
        }
    }

    /// 当前状态是否允许发动某技能。
    pub fn is_action_allowed(&self, action: Skills) -> Result<(), CastActionError> {
        use CastActionError::{
            CarefulObservationUsed3, CraftPointNotEnough, CraftingAlreadyFinished,
            DurabilityNotEnough, FocusNeverFailsAfterObserved, HeartAndSoulUsed,
            LevelGapMustGreaterThanTen, NotAllowedInWastNotBuff, OnlyAllowedInFirstStep,
            PlayerLevelTooLow, RequireGoodOrExcellent, RequireInnerQuiet1, RequireInnerQuiet10,
        };

        match action {
            _ if action.unlock_level() > self.attributes.level => Err(PlayerLevelTooLow),

            Skills::TricksOfTheTrade | Skills::IntensiveSynthesis | Skills::PreciseTouch
            if !matches!(self.condition, Condition::Good | Condition::Excellent)
                && self.buffs.heart_and_soul == 0 =>
                {
                    Err(RequireGoodOrExcellent)
                }

            Skills::PrudentTouch | Skills::PrudentSynthesis if self.buffs.wast_not > 0 => {
                Err(NotAllowedInWastNotBuff)
            }

            Skills::MuscleMemory | Skills::Reflect | Skills::TrainedEye if self.step != 0 => {
                Err(OnlyAllowedInFirstStep)
            }

            Skills::TrainedEye if self.attributes.level < 10 + self.recipe.job_level => {
                Err(LevelGapMustGreaterThanTen)
            }

            Skills::ByregotsBlessing if self.buffs.inner_quiet < 1 => Err(RequireInnerQuiet1),
            Skills::TrainedFinesse if self.buffs.inner_quiet != 10 => Err(RequireInnerQuiet10),

            Skills::CarefulObservation if self.buffs.careful_observation_used >= 3 => {
                Err(CarefulObservationUsed3)
            }
            Skills::HeartAndSoul if self.buffs.heart_and_soul_used >= 1 => Err(HeartAndSoulUsed),

            Skills::FocusedSynthesisFail | Skills::FocusedTouchFail if self.buffs.observed > 0 => {
                Err(FocusNeverFailsAfterObserved)
            }

            _ if self.durability <= 0 => Err(DurabilityNotEnough),
            _ if self.craft_point(action) > self.craft_points => Err(CraftPointNotEnough),
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
        data::high_quality_table(percent)
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
    use test::Bencher;

    use crate::{Attributes, Condition, data, Recipe, Skills, Status};

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
            qu: u32,
            du: u16,
            co: u8,
        }
        for (i, step) in [
            Step {
                a: Skills::DelicateSynthesis,
                pg: 230,
                qu: 301,
                du: 70,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 460,
                qu: 632,
                du: 60,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 690,
                qu: 993,
                du: 50,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 920,
                qu: 1384,
                du: 40,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 1150,
                qu: 1805,
                du: 30,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 1380,
                qu: 2256,
                du: 20,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 1610,
                qu: 2737,
                du: 10,
                co: 1,
            },
            Step {
                a: Skills::DelicateSynthesis,
                pg: 1840,
                qu: 3248,
                du: 0,
                co: 1,
            },
        ]
            .iter()
            .enumerate()
        {
            s.cast_action(step.a);
            assert_eq!(
                s.progress, step.pg,
                "step [{}] progress simulation fail: want {}, get {}",
                i, step.pg, s.progress
            );
            assert_eq!(
                s.quality, step.qu,
                "step [{}] quality simulation fail: want {}, get {}",
                i, step.qu, s.quality
            );
            assert_eq!(
                s.durability, step.du,
                "step [{}] durability simulation fail: want {}, get {}",
                i, step.du, s.durability
            );
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
            qu: u32,
            du: u16,
            co: u8,
        }
        for step in [
            Step {
                a: 100390,
                pg: 0,
                qu: 288,
                du: 70,
                co: 2,
            },
            Step {
                a: 100131,
                pg: 0,
                qu: 1065,
                du: 60,
                co: 1,
            },
            Step {
                a: 4577,
                pg: 0,
                qu: 1065,
                du: 60,
                co: 1,
            },
            Step {
                a: 100230,
                pg: 0,
                qu: 1468,
                du: 60,
                co: 3,
            },
            Step {
                a: 100302,
                pg: 0,
                qu: 4924,
                du: 45,
                co: 4,
            },
            Step {
                a: 100082,
                pg: 0,
                qu: 4924,
                du: 50,
                co: 1,
            },
            Step {
                a: 100246,
                pg: 0,
                qu: 5658,
                du: 45,
                co: 1,
            },
            Step {
                a: 100342,
                pg: 0,
                qu: 6700,
                du: 40,
                co: 1,
            },
            Step {
                a: 19300,
                pg: 0,
                qu: 6700,
                du: 45,
                co: 1,
            },
            Step {
                a: 100075,
                pg: 403,
                qu: 6700,
                du: 40,
                co: 1,
            },
            Step {
                a: 100075,
                pg: 806,
                qu: 6700,
                du: 35,
                co: 1,
            },
            Step {
                a: 100075,
                pg: 1209,
                qu: 6700,
                du: 25,
                co: 1,
            },
            Step {
                a: 100075,
                pg: 1612,
                qu: 6700,
                du: 15,
                co: 1,
            },
            Step {
                a: 100077,
                pg: 1612,
                qu: 6700,
                du: 45,
                co: 1,
            },
            Step {
                a: 19300,
                pg: 1612,
                qu: 6700,
                du: 45,
                co: 1,
            },
            Step {
                a: 100082,
                pg: 1612,
                qu: 6700,
                du: 45,
                co: 1,
            },
            Step {
                a: 100246,
                pg: 1612,
                qu: 6700,
                du: 35,
                co: 1,
            },
            Step {
                a: 100082,
                pg: 1612,
                qu: 6700,
                du: 35,
                co: 1,
            },
            Step {
                a: 100238,
                pg: 2284,
                qu: 6700,
                du: 25,
                co: 1,
            },
            Step {
                a: 100082,
                pg: 2284,
                qu: 6700,
                du: 25,
                co: 1,
            },
            Step {
                a: 100238,
                pg: 2732,
                qu: 6700,
                du: 15,
                co: 1,
            },
            Step {
                a: 100082,
                pg: 2732,
                qu: 6700,
                du: 15,
                co: 1,
            },
            Step {
                a: 100238,
                pg: 3000,
                qu: 6700,
                du: 5,
                co: 1,
            },
        ] {
            let skill = data::action_table(step.a).unwrap();
            s.cast_action(skill);
            // println!("casting: {:?}", skill);
            assert_eq!(
                s.progress, step.pg,
                "step [{:?}] progress simulation fail: want {}, get {}",
                skill, step.pg, s.progress
            );
            assert_eq!(
                s.quality, step.qu,
                "step [{:?}] quality simulation fail: want {}, get {}",
                skill, step.qu, s.quality
            );
            assert_eq!(
                s.durability, step.du,
                "step [{:?}] durability simulation fail: want {}, get {}",
                skill, step.du, s.durability
            );
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
            qu: u32,
            du: u16,
            co: u8,
            su: bool,
        }
        for step in [
            Step {
                a: 100390,
                pg: 0,
                qu: 247,
                du: 50,
                co: 9,
                su: true,
            },
            Step {
                a: 19300,
                pg: 0,
                qu: 247,
                du: 50,
                co: 1,
                su: true,
            },
            Step {
                a: 100366,
                pg: 0,
                qu: 247,
                du: 40,
                co: 2,
                su: false,
            },
            Step {
                a: 100318,
                pg: 1206,
                qu: 247,
                du: 30,
                co: 6,
                su: true,
            },
            Step {
                a: 100366,
                pg: 1206,
                qu: 247,
                du: 25,
                co: 7,
                su: false,
            },
            Step {
                a: 4577,
                pg: 1206,
                qu: 247,
                du: 25,
                co: 5,
                su: true,
            },
            Step {
                a: 100366,
                pg: 2713,
                qu: 247,
                du: 20,
                co: 6,
                su: true,
            },
            Step {
                a: 100366,
                pg: 2713,
                qu: 247,
                du: 20,
                co: 2,
                su: false,
            },
            Step {
                a: 100131,
                pg: 2713,
                qu: 913,
                du: 15,
                co: 5,
                su: true,
            },
            Step {
                a: 100358,
                pg: 2713,
                qu: 913,
                du: 10,
                co: 6,
                su: false,
            },
            Step {
                a: 100358,
                pg: 2713,
                qu: 1258,
                du: 10,
                co: 6,
                su: true,
            },
            Step {
                a: 100358,
                pg: 2713,
                qu: 1628,
                du: 10,
                co: 1,
                su: true,
            },
            Step {
                a: 100082,
                pg: 2713,
                qu: 1628,
                du: 15,
                co: 1,
                su: true,
            },
            Step {
                a: 100246,
                pg: 2713,
                qu: 2220,
                du: 10,
                co: 2,
                su: true,
            },
            Step {
                a: 100374,
                pg: 2713,
                qu: 2220,
                du: 10,
                co: 8,
                su: true,
            },
            Step {
                a: 100077,
                pg: 2713,
                qu: 2220,
                du: 40,
                co: 6,
                su: true,
            },
            Step {
                a: 100366,
                pg: 3718,
                qu: 2220,
                du: 35,
                co: 2,
                su: true,
            },
            Step {
                a: 100131,
                pg: 3718,
                qu: 3164,
                du: 25,
                co: 2,
                su: true,
            },
            Step {
                a: 100131,
                pg: 3718,
                qu: 4219,
                du: 15,
                co: 7,
                su: true,
            },
            Step {
                a: 100077,
                pg: 3718,
                qu: 4219,
                du: 45,
                co: 1,
                su: true,
            },
            Step {
                a: 100366,
                pg: 3718,
                qu: 4219,
                du: 35,
                co: 1,
                su: false,
            },
            Step {
                a: 100366,
                pg: 4723,
                qu: 4219,
                du: 25,
                co: 2,
                su: true,
            },
            Step {
                a: 100374,
                pg: 4723,
                qu: 4219,
                du: 25,
                co: 6,
                su: true,
            },
            Step {
                a: 100358,
                pg: 4723,
                qu: 4219,
                du: 20,
                co: 1,
                su: false,
            },
            Step {
                a: 19007,
                pg: 4723,
                qu: 4219,
                du: 20,
                co: 6,
                su: true,
            },
            Step {
                a: 100082,
                pg: 4723,
                qu: 4219,
                du: 20,
                co: 7,
                su: true,
            },
            Step {
                a: 100246,
                pg: 4723,
                qu: 5330,
                du: 10,
                co: 9,
                su: true,
            },
            Step {
                a: 100077,
                pg: 4723,
                qu: 5330,
                du: 40,
                co: 5,
                su: true,
            },
            Step {
                a: 100082,
                pg: 4723,
                qu: 5330,
                du: 40,
                co: 6,
                su: true,
            },
            Step {
                a: 100246,
                pg: 4723,
                qu: 6071,
                du: 35,
                co: 8,
                su: true,
            },
            Step {
                a: 100075,
                pg: 5084,
                qu: 6071,
                du: 25,
                co: 8,
                su: true,
            },
            Step {
                a: 100075,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 2,
                su: true,
            },
            Step {
                a: 100374,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 2,
                su: true,
            },
            Step {
                a: 100374,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 9,
                su: true,
            },
            Step {
                a: 263,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 9,
                su: true,
            },
            Step {
                a: 19007,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 5,
                su: true,
            },
            Step {
                a: 100082,
                pg: 5445,
                qu: 6071,
                du: 15,
                co: 1,
                su: true,
            },
            Step {
                a: 100246,
                pg: 5445,
                qu: 7923,
                du: 5,
                co: 8,
                su: true,
            },
            Step {
                a: 100077,
                pg: 5445,
                qu: 7923,
                du: 35,
                co: 1,
                su: true,
            },
            Step {
                a: 100082,
                pg: 5445,
                qu: 7923,
                du: 35,
                co: 1,
                su: true,
            },
            Step {
                a: 100082,
                pg: 5445,
                qu: 7923,
                du: 35,
                co: 1,
                su: true,
            },
            Step {
                a: 100075,
                pg: 5470,
                qu: 7923,
                du: 25,
                co: 1,
                su: true,
            },
        ] {
            let skill = data::action_table(step.a).unwrap();
            // println!("casting: {:?} {}", skill,  s.craft_points);
            if step.su {
                s.cast_action(skill);
            } else {
                s.cast_action(match skill {
                    Skills::RapidSynthesis => Skills::RapidSynthesisFail,
                    Skills::HastyTouch => Skills::HastyTouchFail,
                    Skills::FocusedSynthesis => Skills::FocusedSynthesisFail,
                    Skills::FocusedTouch => Skills::FocusedTouchFail,
                    _ => unreachable!(),
                })
            }
            assert_eq!(
                s.progress, step.pg,
                "step [{:?}] progress simulation fail: want {}, get {}",
                skill, step.pg, s.progress
            );
            assert_eq!(
                s.quality, step.qu,
                "step [{:?}] quality simulation fail: want {}, get {}",
                skill, step.qu, s.quality
            );
            assert_eq!(
                s.durability, step.du,
                "step [{:?}] durability simulation fail: want {}, get {}",
                skill, step.du, s.durability
            );
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
            _ => unreachable!(),
        }
    }

    #[allow(unused_must_use)]
    #[bench]
    fn simple_benchmark(b: &mut Bencher) {
        let recipe = Recipe::new(580, 100, 140, 100);
        let player = Attributes {
            level: 90,
            craftsmanship: 3293,
            control: 3524,
            craft_points: 626,
        };
        let s = Status::new(player, recipe);
        let actions = [
            Skills::MuscleMemory,
            Skills::Manipulation,
            Skills::Veneration,
            Skills::WasteNotII,
            Skills::Groundwork,
            Skills::Groundwork,
            Skills::BasicTouch,
            Skills::Innovation,
            Skills::PreparatoryTouch,
            Skills::BasicTouch,
            Skills::StandardTouch,
            Skills::AdvancedTouch,
            Skills::Innovation,
            Skills::PrudentTouch,
            Skills::BasicTouch,
            Skills::StandardTouch,
            Skills::AdvancedTouch,
            Skills::Innovation,
            Skills::TrainedFinesse,
            Skills::TrainedFinesse,
            Skills::GreatStrides,
            Skills::ByregotsBlessing,
            Skills::CarefulSynthesis,
        ];
        b.iter(|| {
            let mut s = s.clone();
            for sk in &actions {
                s.cast_action(*sk);
            }
        })
    }

    #[test]
    fn test1() {
        let recipe = Recipe {
            rlv: 590,
            job_level: 90,
            difficulty: 4300,
            quality: 12800,
            durability: 70,
            conditions_flag: 15,
        };
        let player = Attributes {
            level: 90,
            craftsmanship: 3258,
            control: 3340,
            craft_points: 654,
        };
        let mut s = Status::new(player, recipe);
        let actions = vec![
            Skills::MuscleMemory,
            Skills::Manipulation,
            Skills::Veneration,
            Skills::WasteNot,
            Skills::Groundwork,
            Skills::Groundwork,
            Skills::CarefulSynthesis,
            Skills::BasicTouch,
            Skills::StandardTouch,
            Skills::AdvancedTouch,
            Skills::Innovation,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
        ];
        for &sk in &actions {
            s.cast_action(sk);
        }
        assert_eq!(s.quality, 1865);
    }
}
