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

/// 代表一个玩家在作业时可以使用的一个技能的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actions {
    // Reserve 0 so that Option<Actions> can be initialized to 0 as a None value.
    BasicSynthesis = 1,
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
    AdvancedTouch,
    Reflect,
    PreparatoryTouch,
    Groundwork,
    DelicateSynthesis,
    IntensiveSynthesis,
    TrainedEye,
    PrudentSynthesis,
    TrainedFinesse,
    CarefulObservation,
    HeartAndSoul,
    // 7.0
    RefinedTouch,
    DaringTouch,
    QuickInnovation,
    ImmaculateMend,
    TrainedPerfection,
    // fake actions
    RapidSynthesisFail,
    HastyTouchFail,
    DaringTouchFail,
    FocusedSynthesisFail,
    FocusedTouchFail,
}

#[deprecated]
pub type Skills = Actions;

#[cfg(feature = "serde-support")]
impl Serialize for Actions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

#[cfg(feature = "serde-support")]
impl<'de> Deserialize<'de> for Actions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrVisitor;
        impl<'de> Visitor<'de> for StrVisitor {
            type Value = Actions;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Actions::try_from(v).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}

impl Actions {
    fn unlock_level(&self) -> u8 {
        match self {
            Actions::BasicSynthesis => 1,
            Actions::BasicTouch => 5,
            Actions::MastersMend => 7,
            Actions::HastyTouch => 9,
            Actions::RapidSynthesis => 9,
            Actions::Observe => 13,
            Actions::TricksOfTheTrade => 13,
            Actions::WasteNot => 15,
            Actions::Veneration => 15,
            Actions::StandardTouch => 18,
            Actions::GreatStrides => 21,
            Actions::Innovation => 26,
            Actions::FinalAppraisal => 42,
            Actions::WasteNotII => 47,
            Actions::ByregotsBlessing => 50,
            Actions::PreciseTouch => 53,
            Actions::MuscleMemory => 54,
            Actions::CarefulSynthesis => 62,
            Actions::Manipulation => 65,
            Actions::PrudentTouch => 66,
            Actions::AdvancedTouch => 68,
            Actions::Reflect => 69,
            Actions::PreparatoryTouch => 71,
            Actions::Groundwork => 72,
            Actions::DelicateSynthesis => 76,
            Actions::IntensiveSynthesis => 78,
            Actions::TrainedEye => 80,
            Actions::PrudentSynthesis => 88,
            Actions::TrainedFinesse => 90,
            Actions::CarefulObservation => 55,
            Actions::HeartAndSoul => 86,
            // 7.0
            Actions::RefinedTouch => 92,
            Actions::DaringTouch => 96,
            Actions::QuickInnovation => 96,
            Actions::ImmaculateMend => 98,
            Actions::TrainedPerfection => 100,
            // fake actions
            Actions::RapidSynthesisFail => 9,
            Actions::HastyTouchFail => 9,
            Actions::DaringTouchFail => 96,
            Actions::FocusedSynthesisFail => 67,
            Actions::FocusedTouchFail => 68,
        }
    }
}

impl From<&Actions> for &str {
    fn from(sk: &Actions) -> Self {
        match sk {
            Actions::BasicSynthesis => "basic_synthesis",
            Actions::BasicTouch => "basic_touch",
            Actions::MastersMend => "masters_mend",
            Actions::HastyTouch => "hasty_touch",
            Actions::RapidSynthesis => "rapid_synthesis",
            Actions::Observe => "observe",
            Actions::TricksOfTheTrade => "tricks_of_the_trade",
            Actions::WasteNot => "waste_not",
            Actions::Veneration => "veneration",
            Actions::StandardTouch => "standard_touch",
            Actions::GreatStrides => "great_strides",
            Actions::Innovation => "innovation",
            Actions::FinalAppraisal => "final_appraisal",
            Actions::WasteNotII => "waste_not_ii",
            Actions::ByregotsBlessing => "byregot_s_blessing",
            Actions::PreciseTouch => "precise_touch",
            Actions::MuscleMemory => "muscle_memory",
            Actions::CarefulSynthesis => "careful_synthesis",
            Actions::Manipulation => "manipulation",
            Actions::PrudentTouch => "prudent_touch",
            Actions::AdvancedTouch => "advanced_touch",
            Actions::Reflect => "reflect",
            Actions::PreparatoryTouch => "preparatory_touch",
            Actions::Groundwork => "groundwork",
            Actions::DelicateSynthesis => "delicate_synthesis",
            Actions::IntensiveSynthesis => "intensive_synthesis",
            Actions::TrainedEye => "trained_eye",
            Actions::PrudentSynthesis => "prudent_synthesis",
            Actions::TrainedFinesse => "trained_finesse",
            Actions::CarefulObservation => "careful_observation",
            Actions::HeartAndSoul => "heart_and_soul",
            // 7.0
            Actions::RefinedTouch => "refined_touch",
            Actions::DaringTouch => "daring_touch",
            Actions::QuickInnovation => "quick_innovation",
            Actions::ImmaculateMend => "immaculate_mend",
            Actions::TrainedPerfection => "trained_perfection",
            // fake actions
            Actions::RapidSynthesisFail => "rapid_synthsis_fail",
            Actions::HastyTouchFail => "hasty_touch_fail",
            Actions::DaringTouchFail => "daring_touch_fail",
            Actions::FocusedSynthesisFail => "focused_synthesis_fail",
            Actions::FocusedTouchFail => "focused_touch_fail",
        }
    }
}

impl TryFrom<&str> for Actions {
    type Error = UnknownSkillErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "basic_synthesis" | "制作" => Actions::BasicSynthesis,
            "basic_touch" | "加工" => Actions::BasicTouch,
            "masters_mend" | "精修" => Actions::MastersMend,
            "hasty_touch" | "仓促" => Actions::HastyTouch,
            "rapid_synthesis" | "高速制作" => Actions::RapidSynthesis,
            "observe" | "观察" => Actions::Observe,
            "tricks_of_the_trade" | "秘诀" => Actions::TricksOfTheTrade,
            "waste_not" | "俭约" => Actions::WasteNot,
            "veneration" | "崇敬" => Actions::Veneration,
            "standard_touch" | "中级加工" => Actions::StandardTouch,
            "great_strides" | "阔步" => Actions::GreatStrides,
            "innovation" | "改革" => Actions::Innovation,
            "final_appraisal" | "最终确认" => Actions::FinalAppraisal,
            "waste_not_ii" | "长期俭约" => Actions::WasteNotII,
            "byregot_s_blessing" | "比尔格的祝福" => Actions::ByregotsBlessing,
            "precise_touch" | "集中加工" => Actions::PreciseTouch,
            "muscle_memory" | "坚信" => Actions::MuscleMemory,
            "careful_synthesis" | "模范制作" => Actions::CarefulSynthesis,
            "manipulation" | "掌握" => Actions::Manipulation,
            "prudent_touch" | "俭约加工" => Actions::PrudentTouch,
            "advanced_touch" | "上级加工" => Actions::AdvancedTouch,
            "reflect" | "闲静" => Actions::Reflect,
            "preparatory_touch" | "坯料加工" => Actions::PreparatoryTouch,
            "groundwork" | "坯料制作" => Actions::Groundwork,
            "delicate_synthesis" | "精密制作" => Actions::DelicateSynthesis,
            "intensive_synthesis" | "集中制作" => Actions::IntensiveSynthesis,
            "trained_eye" | "工匠的神速技巧" => Actions::TrainedEye,
            "prudent_synthesis" | "俭约制作" => Actions::PrudentSynthesis,
            "trained_finesse" | "工匠的神技" => Actions::TrainedFinesse,
            "careful_observation" | "设计变动" => Actions::CarefulObservation,
            "heart_and_soul" | "专心致志" => Actions::HeartAndSoul,
            // 7.0
            "refined_touch" => Actions::RefinedTouch,
            "daring_touch" => Actions::DaringTouch,
            "quick_innovation" => Actions::QuickInnovation,
            "immaculate_mend" => Actions::ImmaculateMend,
            "trained_perfection" => Actions::TrainedPerfection,
            // fake actions
            "rapid_synthesis_fail" => Actions::RapidSynthesisFail,
            "hasty_touch_fail" => Actions::HastyTouchFail,
            "daring_touch_fail" => Actions::DaringTouchFail,
            "focused_synthesis_fail" => Actions::FocusedSynthesisFail,
            "focused_touch_fail" => Actions::FocusedTouchFail,
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
    /// 白：通常
    Normal,
    /// 红：高品质，加工效率1.5倍
    Good,
    /// 彩：最高品质
    Excellent,
    /// 黑：低品质
    Poor,

    /// 黄：成功率增加 25%
    Centered,
    /// 蓝：耐久消耗降低 50%, 效果可与俭约叠加
    Sturdy,
    /// 绿：CP 消耗减少 50%
    Pliant,
    /// 深蓝：作业效率1.5倍
    Malleable,
    /// 紫：技能效果持续增加两回合
    Primed,
    /// 粉：下一回合必定是红球
    GoodOmen,
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
            Condition::GoodOmen => "good_omen",
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
            "good_omen" => Condition::GoodOmen,
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

/// 储存了一次制作中配方的信息。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Recipe {
    /// 配方等级
    pub rlv: RecipeLevel,

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

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct RecipeLevel {
    pub id: i32,
    pub class_job_level: u8,
    pub stars: u8,
    pub suggested_craftsmanship: u16,
    pub difficulty: u16,
    pub quality: u32,
    pub progress_divider: u8,
    pub quality_divider: u8,
    pub progress_modifier: u8,
    pub quality_modifier: u8,
    pub durability: u16,
    pub conditions_flag: u16,
}

impl Recipe {
    pub fn new(
        rlv: RecipeLevel,
        difficulty_factor: u16,
        quality_factor: u16,
        durability_factor: u16,
    ) -> Self {
        Self {
            rlv,
            job_level: rlv.class_job_level,
            difficulty: (rlv.difficulty as u32 * difficulty_factor as u32 / 100) as u16,
            quality: rlv.quality * quality_factor as u32 / 100,
            durability: rlv.durability * durability_factor / 100,
            conditions_flag: rlv.conditions_flag,
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
    /// 匠の好機
    pub expedience: u8,
    /// 专心致志
    pub heart_and_soul: LimitedActionState,
    /// 工匠的绝技
    pub trained_perfection: LimitedActionState,
    /// 设计变动使用次数
    /// 假想buff，用于记录设计变动使用的次数
    pub careful_observation_used: u8,
    /// 禁止使用Quick改革
    pub quick_innovation_used: u8,
    /// 加工连击状态：0无，1中级加工预备，2上级加工预备
    pub touch_combo_stage: u8,
    /// 观察（注视预备）
    /// 假想buff，用于处理 观察-注释制作 OR 观察-注视加工 的连击。
    pub observed: u8,
}

#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
pub enum LimitedActionState {
    Unused,
    Active,
    Used,
}

impl Default for LimitedActionState {
    fn default() -> Self {
        LimitedActionState::Unused
    }
}

fn round_down(v: f64, scale: f64) -> f64 {
    (v * scale).floor() / scale
}

impl Buffs {
    pub(crate) fn synthesis(&self, skill_e: f64) -> f64 {
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

    pub(crate) fn touch(&self, skill_e: f64) -> f64 {
        let mut bm = 1.0;
        if self.great_strides > 0 {
            bm += 1.0;
        }
        if self.innovation > 0 {
            bm += 0.5;
        }
        let iq = 1.0 + self.inner_quiet as f64 * 0.1;
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
        self.expedience = self.expedience.saturating_sub(1);
        self.next_combo();
    }

    pub(crate) fn next_combo(&mut self) {
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
    pub caches: Caches,

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
pub struct Caches {
    /// 无buff效果下100效率的作业技能能推动的进展数值
    pub base_synth: f32,
    /// 无buff效果下100效率的加工技能能推动的品质数值
    pub base_touch: f32,
}

impl Caches {
    pub fn new(attributes: &Attributes, recipe: &Recipe) -> Self {
        let rlv = &recipe.rlv;
        Self {
            base_synth: {
                let mut base =
                    attributes.craftsmanship as f32 * 10.0 / rlv.progress_divider as f32 + 2.0;
                if attributes.level <= recipe.job_level {
                    base *= rlv.progress_modifier as f32 * 0.01
                }
                base.floor()
            },
            base_touch: {
                let mut base = attributes.control as f32 * 10.0 / rlv.quality_divider as f32 + 35.0;
                if attributes.level <= recipe.job_level {
                    base *= rlv.quality_modifier as f32 * 0.01
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
    /// 必须在仓促成功后使用
    RequireHastyTouchSuccessed,
    /// 快速改革一次制作只能使用一次
    QuickInnovationUsed,
    /// 改革状态下无法发动快速改革
    NotAllowedInInnovationBuff,
    /// 工匠的绝技一次制作只能使用一次
    TrainedPerfectionUsed,
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
            CastActionError::RequireHastyTouchSuccessed => "require hasty touch successed first",
            CastActionError::QuickInnovationUsed => "quick innovation can be only used once",
            CastActionError::NotAllowedInInnovationBuff => "not allowed in innovation buff",
            CastActionError::TrainedPerfectionUsed => "trained perfection can be only used once",
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

    pub fn calc_durability(&self, durability: u16) -> u16 {
        let mut reduce = durability;
        if let Condition::Sturdy = self.condition {
            reduce -= reduce / 2;
        }
        if self.buffs.wast_not > 0 {
            reduce -= reduce / 2;
        }
        if let LimitedActionState::Active = self.buffs.trained_perfection {
            reduce = 0;
        }
        reduce
    }

    fn consume_durability(&mut self, durability: u16) {
        if durability == 0 {
            return;
        }
        if let LimitedActionState::Active = self.buffs.trained_perfection {
            self.buffs.trained_perfection = LimitedActionState::Used;
            return;
        }
        self.durability = self
            .durability
            .saturating_sub(self.calc_durability(durability));
    }

    pub fn calc_synthesis(&self, efficiency: f64) -> u16 {
        (self.caches.base_synth as f64
            * self.buffs.synthesis(efficiency)
            * self.condition.synth_ratio() as f64) as f32 as u16
    }

    pub fn calc_touch(&self, efficiency: f64) -> u32 {
        (self.caches.base_touch as f64
            * self.buffs.touch(efficiency)
            * self.condition.touch_ratio() as f64) as f32 as u32
    }

    fn cast_synthesis(&mut self, durability: u16, efficiency: f64) {
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

    fn cast_touch(&mut self, durability: u16, efficiency: f64, inner_quiet_addon: i8) {
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
    pub fn craft_point(&self, skill: Actions) -> i32 {
        let cp: i32 = match skill {
            Actions::BasicSynthesis => 0,
            Actions::BasicTouch => 18,
            Actions::MastersMend => 88,
            Actions::HastyTouch => 0,
            Actions::RapidSynthesis => 0,
            Actions::Observe => 7,
            Actions::TricksOfTheTrade => 0,
            Actions::WasteNot => 56,
            Actions::Veneration => 18,
            Actions::StandardTouch => {
                if self.buffs.touch_combo_stage == 1 {
                    18
                } else {
                    32
                }
            }
            Actions::GreatStrides => 32,
            Actions::Innovation => 18,
            Actions::FinalAppraisal => 1,
            Actions::WasteNotII => 98,
            Actions::ByregotsBlessing => 24,
            Actions::PreciseTouch => 18,
            Actions::MuscleMemory => 6,
            Actions::CarefulSynthesis => 7,
            Actions::Manipulation => 96,
            Actions::PrudentTouch => 25,
            Actions::AdvancedTouch => {
                if self.buffs.touch_combo_stage == 2 || self.buffs.observed > 0 {
                    18
                } else {
                    46
                }
            }
            Actions::Reflect => 6,
            Actions::PreparatoryTouch => 40,
            Actions::Groundwork => 18,
            Actions::DelicateSynthesis => 32,
            Actions::IntensiveSynthesis => 6,
            Actions::TrainedEye => 250,
            Actions::PrudentSynthesis => 18,
            Actions::TrainedFinesse => 32,
            Actions::CarefulObservation => 0,
            Actions::HeartAndSoul => 0,
            // 7.0
            Actions::RefinedTouch => 24,
            Actions::DaringTouch => 0,
            Actions::ImmaculateMend => 112,
            Actions::QuickInnovation => 0,
            Actions::TrainedPerfection => 0,
            // fake actions
            Actions::RapidSynthesisFail => 0,
            Actions::HastyTouchFail => 0,
            Actions::DaringTouchFail => 0,
            Actions::FocusedSynthesisFail => 5,
            Actions::FocusedTouchFail => 18,
        };
        if let Condition::Pliant = self.condition {
            cp - cp / 2
        } else {
            cp
        }
    }

    /// 发动一次技能。
    pub fn cast_action(&mut self, action: Actions) {
        self.craft_points -= self.craft_point(action);
        match action {
            Actions::BasicSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 31 { 1.0 } else { 1.2 })
            }
            Actions::RapidSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 63 { 2.5 } else { 5.0 })
            }
            Actions::CarefulSynthesis => {
                self.cast_synthesis(10, if self.attributes.level < 82 { 1.5 } else { 1.8 })
            }
            Actions::Groundwork => {
                let mut e = if self.attributes.level < 86 { 3.0 } else { 3.6 };
                let d = self.calc_durability(20);
                if self.durability < d {
                    e *= 0.5
                };
                self.cast_synthesis(20, e)
            }
            Actions::IntensiveSynthesis => {
                self.cast_synthesis(10, 4.0);
                if !matches!(self.condition, Condition::Good | Condition::Excellent) {
                    self.buffs.heart_and_soul = LimitedActionState::Used;
                }
            }
            Actions::PrudentSynthesis => self.cast_synthesis(5, 1.8),

            Actions::DelicateSynthesis => {
                self.cast_synthesis(0, if self.attributes.level < 94 { 1.0 } else { 1.5 });
                self.cast_touch(10, 1.0, 1);
            }

            Actions::BasicTouch => {
                self.cast_touch(10, 1.0, 1);
                self.buffs.touch_combo_stage = 1 + 2;
            }
            Actions::HastyTouch => {
                self.cast_touch(10, 1.0, 1);
                self.buffs.expedience = 2;
            }
            Actions::StandardTouch => {
                if self.buffs.touch_combo_stage == 1 {
                    self.buffs.touch_combo_stage = 2 + 2;
                };
                self.cast_touch(10, 1.25, 1)
            }
            Actions::AdvancedTouch => self.cast_touch(10, 1.5, 1),
            Actions::ByregotsBlessing => {
                let e = (1.0 + self.buffs.inner_quiet as f64 * 0.2).min(3.0);
                self.cast_touch(10, e, -(self.buffs.inner_quiet as i8));
            }
            Actions::PreciseTouch => {
                self.cast_touch(10, 1.5, 2);
                if !matches!(self.condition, Condition::Good | Condition::Excellent)
                    && matches!(self.buffs.heart_and_soul, LimitedActionState::Active)
                {
                    self.buffs.heart_and_soul = LimitedActionState::Used;
                }
            }
            Actions::PrudentTouch => self.cast_touch(5, 1.0, 1),
            Actions::PreparatoryTouch => self.cast_touch(20, 2.0, 2),
            Actions::TrainedFinesse => self.cast_touch(0, 1.0, 0),
            Actions::TricksOfTheTrade => {
                self.craft_points = (self.craft_points + 20).min(self.attributes.craft_points);
                if !matches!(self.condition, Condition::Good | Condition::Excellent)
                    && matches!(self.buffs.heart_and_soul, LimitedActionState::Active)
                {
                    self.buffs.heart_and_soul = LimitedActionState::Used;
                }
            }

            Actions::MastersMend => {
                self.durability = self.recipe.durability.min(self.durability + 30);
            }
            Actions::WasteNot => {
                self.buffs.wast_not = self.new_duration_buff(4);
            }
            Actions::WasteNotII => {
                self.buffs.wast_not = self.new_duration_buff(8);
            }
            Actions::Manipulation => {
                self.buffs.manipulation = self.new_duration_buff(8);
                self.buffs.next();
                self.step += 1;
                return;
            }
            Actions::MuscleMemory => {
                self.cast_synthesis(10, 3.0);
                self.buffs.muscle_memory = self.new_duration_buff(5);
            }
            Actions::Reflect => {
                self.cast_touch(10, 3.0, 2);
            }
            Actions::TrainedEye => {
                self.quality += self.recipe.quality;
                self.buffs.inner_quiet = self.buffs.inner_quiet.saturating_add_signed(1).min(10);
            }
            Actions::Veneration => {
                self.buffs.veneration = self.new_duration_buff(4);
            }
            Actions::GreatStrides => {
                self.buffs.great_strides = self.new_duration_buff(3);
            }
            Actions::Innovation => {
                self.buffs.innovation = self.new_duration_buff(4);
            }
            Actions::Observe => {
                self.buffs.observed = 2;
            }
            Actions::FinalAppraisal => {
                self.buffs.final_appraisal = self.new_duration_buff(5);
                self.buffs.next_combo();
                return;
            }
            Actions::CarefulObservation => {
                self.buffs.careful_observation_used += 1;
                self.buffs.next_combo();
                return;
            }
            Actions::HeartAndSoul => {
                self.buffs.heart_and_soul = LimitedActionState::Active;
                self.buffs.next_combo();
                return;
            }
            // 7.0
            Actions::RefinedTouch => {
                self.cast_touch(
                    10,
                    1.0,
                    if self.buffs.touch_combo_stage == 1 {
                        2
                    } else {
                        1
                    },
                );
            }
            Actions::DaringTouch => self.cast_touch(10, 1.5, 1),
            Actions::QuickInnovation => {
                self.buffs.innovation = self.new_duration_buff(1);
                self.buffs.quick_innovation_used += 1;
                self.buffs.next_combo();
                return;
            }
            Actions::ImmaculateMend => {
                self.durability = self.recipe.durability;
            }
            Actions::TrainedPerfection => {
                self.buffs.trained_perfection = LimitedActionState::Active;
            }
            // fake actions
            Actions::RapidSynthesisFail => self.consume_durability(10),
            Actions::HastyTouchFail => self.consume_durability(10),
            Actions::DaringTouchFail => self.consume_durability(10),
            Actions::FocusedSynthesisFail => self.consume_durability(10),
            Actions::FocusedTouchFail => self.consume_durability(10),
        }
        if self.buffs.manipulation > 0 && self.durability > 0 {
            self.durability += 5;
            self.durability = self.durability.min(self.recipe.durability);
        }
        self.buffs.next();
        self.step += 1;
    }

    /// 计算当前状态下某技能的成功概率，返回结果介于[0..=100]之间。
    pub fn success_rate(&self, action: Actions) -> u8 {
        let addon = match self.condition {
            Condition::Centered => 25,
            _ => 0,
        };
        addon
            + match action {
                Actions::HastyTouch | Actions::DaringTouch => 60,
                Actions::RapidSynthesis => 50,
                _ => return 100,
            }
    }

    /// 当前状态是否允许发动某技能。
    pub fn is_action_allowed(&self, action: Actions) -> Result<(), CastActionError> {
        use CastActionError::{
            CarefulObservationUsed3, CraftPointNotEnough, CraftingAlreadyFinished,
            DurabilityNotEnough, FocusNeverFailsAfterObserved, HeartAndSoulUsed,
            LevelGapMustGreaterThanTen, NotAllowedInInnovationBuff, NotAllowedInWastNotBuff,
            OnlyAllowedInFirstStep, PlayerLevelTooLow, QuickInnovationUsed, RequireGoodOrExcellent,
            RequireHastyTouchSuccessed, RequireInnerQuiet1, RequireInnerQuiet10,
            TrainedPerfectionUsed,
        };

        match action {
            _ if action.unlock_level() > self.attributes.level => Err(PlayerLevelTooLow),

            Actions::TricksOfTheTrade | Actions::IntensiveSynthesis | Actions::PreciseTouch
                if !matches!(self.condition, Condition::Good | Condition::Excellent)
                    && !matches!(self.buffs.heart_and_soul, LimitedActionState::Active) =>
            {
                Err(RequireGoodOrExcellent)
            }

            Actions::PrudentTouch | Actions::PrudentSynthesis if self.buffs.wast_not > 0 => {
                Err(NotAllowedInWastNotBuff)
            }

            Actions::MuscleMemory | Actions::Reflect | Actions::TrainedEye if self.step != 0 => {
                Err(OnlyAllowedInFirstStep)
            }

            Actions::TrainedEye if self.attributes.level < 10 + self.recipe.job_level => {
                Err(LevelGapMustGreaterThanTen)
            }

            Actions::ByregotsBlessing if self.buffs.inner_quiet < 1 => Err(RequireInnerQuiet1),
            Actions::TrainedFinesse if self.buffs.inner_quiet != 10 => Err(RequireInnerQuiet10),

            Actions::CarefulObservation if self.buffs.careful_observation_used >= 3 => {
                Err(CarefulObservationUsed3)
            }
            Actions::HeartAndSoul
                if !matches!(self.buffs.heart_and_soul, LimitedActionState::Unused) =>
            {
                Err(HeartAndSoulUsed)
            }

            Actions::FocusedSynthesisFail | Actions::FocusedTouchFail
                if self.buffs.observed > 0 =>
            {
                Err(FocusNeverFailsAfterObserved)
            }

            Actions::DaringTouch if self.buffs.expedience < 1 => Err(RequireHastyTouchSuccessed),
            Actions::QuickInnovation if self.buffs.quick_innovation_used >= 1 => {
                Err(QuickInnovationUsed)
            }
            Actions::QuickInnovation if self.buffs.innovation > 0 => {
                Err(NotAllowedInInnovationBuff)
            }
            Actions::TrainedPerfection
                if !matches!(self.buffs.trained_perfection, LimitedActionState::Unused) =>
            {
                Err(TrainedPerfectionUsed)
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
            step: Some(Condition::Normal),
        }
    }

    fn next_cond(cond: Condition) -> Option<Condition> {
        match cond {
            Condition::Normal => Some(Condition::Good),
            Condition::Good => Some(Condition::Excellent),
            Condition::Excellent => Some(Condition::Poor),
            Condition::Poor => Some(Condition::Centered),
            Condition::Centered => Some(Condition::Sturdy),
            Condition::Sturdy => Some(Condition::Pliant),
            Condition::Pliant => Some(Condition::Malleable),
            Condition::Malleable => Some(Condition::Primed),
            Condition::Primed => Some(Condition::GoodOmen),
            Condition::GoodOmen => None,
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
            Condition::GoodOmen => 0.12,
            Condition::Normal => 1.0 - self.rate,
        };
        self.rate += rate;
        Some((cond, rate))
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::{data, Actions, Attributes, Condition, Recipe, Status};

    #[test]
    fn option_actions() {
        use std::mem::{size_of, transmute};
        // For Option<Actions>, the None value has to be represented by 0,
        // to achieve rapid initialization of large Actions arrays.
        assert_eq!(size_of::<Option<Actions>>(), 1);
        unsafe {
            assert_eq!(transmute::<Option<Actions>, u8>(None), 0);
        }
    }

    #[test]
    fn basic_synth() {
        let attr = Attributes {
            level: 80,
            craftsmanship: 2806,
            control: 2784,
            craft_points: 548,
        };
        let rlv = data::recipe_level_table(517);
        let recipe = Recipe::new(rlv, 50, 100, 50);
        let mut s = Status::new(attr, recipe);

        let result = [279, 558, 837, 1000];
        for &pg in &result {
            s.cast_action(Actions::BasicSynthesis);
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
        let rlv = data::recipe_level_table(517);
        let recipe = Recipe::new(rlv, 100, 100, 100);
        let mut s = Status::new(attr, recipe);

        struct Step {
            a: Actions,
            pg: u16,
            qu: u32,
            du: u16,
            co: u8,
        }
        for (i, step) in [
            Step {
                a: Actions::DelicateSynthesis,
                pg: 230,
                qu: 301,
                du: 70,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 460,
                qu: 632,
                du: 60,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 690,
                qu: 993,
                du: 50,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 920,
                qu: 1384,
                du: 40,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 1150,
                qu: 1805,
                du: 30,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 1380,
                qu: 2256,
                du: 20,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
                pg: 1610,
                qu: 2737,
                du: 10,
                co: 1,
            },
            Step {
                a: Actions::DelicateSynthesis,
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
        let rlv = data::recipe_level_table(580);
        let recipe = Recipe::new(rlv, 100, 140, 100);
        let player = Attributes {
            level: 90,
            craftsmanship: 3293,
            control: 3524,
            craft_points: 626,
        };
        let s = Status::new(player, recipe);
        let actions = [
            Actions::MuscleMemory,
            Actions::Manipulation,
            Actions::Veneration,
            Actions::WasteNotII,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::BasicTouch,
            Actions::Innovation,
            Actions::PreparatoryTouch,
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::AdvancedTouch,
            Actions::Innovation,
            Actions::PrudentTouch,
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::AdvancedTouch,
            Actions::Innovation,
            Actions::TrainedFinesse,
            Actions::TrainedFinesse,
            Actions::GreatStrides,
            Actions::ByregotsBlessing,
            Actions::CarefulSynthesis,
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
            rlv: data::recipe_level_table(590),
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
            Actions::MuscleMemory,
            Actions::Manipulation,
            Actions::Veneration,
            Actions::WasteNot,
            Actions::Groundwork,
            Actions::Groundwork,
            Actions::CarefulSynthesis,
            Actions::BasicTouch,
            Actions::StandardTouch,
            Actions::AdvancedTouch,
            Actions::Innovation,
            Actions::PrudentTouch,
            Actions::PrudentTouch,
        ];
        for &sk in &actions {
            s.cast_action(sk);
        }
        assert_eq!(s.quality, 1865);
    }

    #[test]
    fn test2() {
        let recipe = Recipe {
            rlv: data::recipe_level_table(640),
            job_level: 90,
            difficulty: 6600,
            quality: 14040,
            durability: 70,
            conditions_flag: 15,
        };
        let player = Attributes {
            level: 90,
            craftsmanship: 4048,
            control: 4005,
            craft_points: 594,
        };
        let mut s = Status::new(player, recipe);
        s.cast_action(Actions::Veneration);
        s.cast_action(Actions::Groundwork);
        assert_eq!(s.progress, 1350);
    }
}
