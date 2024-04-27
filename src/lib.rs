#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use pyo3::exceptions::PyValueError;
#[cfg(feature = "serde-support")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use pyo3::prelude::*;
use rand::{rngs::SmallRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};

pub mod data;
pub mod export;

/// 代表一个玩家在作业时可以使用的一个技能的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
#[pyclass]
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
    // fake actions
    RapidSynthesisFail,
    HastyTouchFail,
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
            Actions::FocusedSynthesis => 67,
            Actions::FocusedTouch => 68,
            Actions::Reflect => 69,
            Actions::PreparatoryTouch => 71,
            Actions::Groundwork => 72,
            Actions::DelicateSynthesis => 76,
            Actions::IntensiveSynthesis => 78,
            Actions::TrainedEye => 80,
            Actions::AdvancedTouch => 84,
            Actions::PrudentSynthesis => 88,
            Actions::TrainedFinesse => 90,
            Actions::CarefulObservation => 55,
            Actions::HeartAndSoul => 86,
            // fake actions
            Actions::RapidSynthesisFail => 9,
            Actions::HastyTouchFail => 9,
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
            Actions::FocusedSynthesis => "focused_synthesis",
            Actions::FocusedTouch => "focused_touch",
            Actions::Reflect => "reflect",
            Actions::PreparatoryTouch => "preparatory_touch",
            Actions::Groundwork => "groundwork",
            Actions::DelicateSynthesis => "delicate_synthesis",
            Actions::IntensiveSynthesis => "intensive_synthesis",
            Actions::TrainedEye => "trained_eye",
            Actions::AdvancedTouch => "advanced_touch",
            Actions::PrudentSynthesis => "prudent_synthesis",
            Actions::TrainedFinesse => "trained_finesse",
            Actions::CarefulObservation => "careful_observation",
            Actions::HeartAndSoul => "heart_and_soul",
            // fake actions
            Actions::RapidSynthesisFail => "rapid_synthsis_fail",
            Actions::HastyTouchFail => "hasty_touch_fail",
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
            "focused_synthesis" | "注视制作" => Actions::FocusedSynthesis,
            "focused_touch" | "注视加工" => Actions::FocusedTouch,
            "reflect" | "闲静" => Actions::Reflect,
            "preparatory_touch" | "坯料加工" => Actions::PreparatoryTouch,
            "groundwork" | "坯料制作" => Actions::Groundwork,
            "delicate_synthesis" | "精密制作" => Actions::DelicateSynthesis,
            "intensive_synthesis" | "集中制作" => Actions::IntensiveSynthesis,
            "trained_eye" | "工匠的神速技巧" => Actions::TrainedEye,
            "advanced_touch" | "上级加工" => Actions::AdvancedTouch,
            "prudent_synthesis" | "俭约制作" => Actions::PrudentSynthesis,
            "trained_finesse" | "工匠的神技" => Actions::TrainedFinesse,
            "careful_observation" | "设计变动" => Actions::CarefulObservation,
            "heart_and_soul" | "专心致志" => Actions::HeartAndSoul,
            // fake actions
            "rapid_synthesis_fail" => Actions::RapidSynthesisFail,
            "hasty_touch_fail" => Actions::HastyTouchFail,
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
#[pyclass]
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
#[pyclass]
pub struct Attributes {
    /// 玩家等级
    #[pyo3(get)]
    pub level: u8,
    /// 制作精度
    #[pyo3(get)]
    pub craftsmanship: i32,
    /// 加工精度
    #[pyo3(get)]
    pub control: i32,
    /// 制作力
    #[pyo3(get)]
    pub craft_points: i32,
}

#[pymethods]
impl Attributes {
    #[new]
    fn new(level: u8, craftsmanship: i32, control: i32, craft_points: i32) -> Self {
        Self {
            level,
            craftsmanship,
            control,
            craft_points,
        }
    }
}

/// 储存了一次制作中配方的信息。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[pyclass]
pub struct Recipe {
    /// 配方等级
    pub rlv: i32,

    /// 制作配方所需的玩家等级
    pub job_level: u8,

    /// 难度（最大进展）
    #[pyo3(get)]
    pub difficulty: u16,

    /// 最高品质
    #[pyo3(get)]
    pub quality: u32,

    /// 耐久
    #[pyo3(get)]
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
#[pyclass]
pub struct RecipeLevel {
    pub class_job_level: u8,
    pub stars: u8,
    pub suggested_craftsmanship: u16,
    pub suggested_control: u16,
    pub difficulty: u16,
    pub quality: u32,
    pub progress_divider: u8,
    pub quality_divider: u8,
    pub progress_modifier: u8,
    pub quality_modifier: u8,
    pub durability: u16,
    pub conditions_flag: u16,
}

#[pymethods]
impl RecipeLevel {
    #[new]
    fn new(rlv: i32) -> Self {
        data::recipe_level_table(rlv)
    }
}

#[pymethods]
impl Recipe {
    #[new]
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
#[pyclass]
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
#[pyclass]
pub struct Status {
    /// 玩家当前身上的buff
    pub buffs: Buffs,
    /// 玩家的装备属性
    pub attributes: Attributes,
    /// 本次制作配方
    pub recipe: Recipe,
    /// 预计算数据
    pub caches: Caches,

    rng: SmallRng,

    /// 剩余耐久
    #[pyo3(get)]
    pub durability: u16,
    /// 剩余制作力
    #[pyo3(get)]
    pub craft_points: i32,
    /// 进展
    #[pyo3(get)]
    pub progress: u16,
    /// 品质
    #[pyo3(get)]
    pub quality: u32,

    /// 步数
    #[pyo3(get)]
    pub step: i32,
    /// 制作状态
    #[pyo3(get)]
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
    pub fn new(attributes: &Attributes, recipe: &Recipe, rlv: &RecipeLevel) -> Self {
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
#[pyclass]
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

impl From<CastActionError> for PyErr {
    fn from(value: CastActionError) -> Self {
        PyValueError::new_err(format!("{value}"))
    }
}

#[pymethods]
impl Status {
    #[new]
    pub fn new(
        attributes: Attributes,
        recipe: Recipe,
        rlv: RecipeLevel,
        seed: Option<u64>,
    ) -> Self {
        Status {
            buffs: Buffs::default(),
            caches: Caches::new(&attributes, &recipe, &rlv),
            attributes,
            recipe,
            rng: match seed {
                Some(seed) => SmallRng::seed_from_u64(seed),
                None => SmallRng::from_rng(thread_rng()).unwrap(),
            },
            durability: recipe.durability,
            craft_points: attributes.craft_points,
            condition: Condition::Normal,
            progress: 0,
            quality: 0,
            step: 0,
        }
    }

    #[getter]
    pub fn buffs(&self) -> Vec<u8> {
        vec![
            self.buffs.muscle_memory,
            self.buffs.great_strides,
            self.buffs.veneration,
            self.buffs.innovation,
            self.buffs.inner_quiet,
            self.buffs.final_appraisal,
            self.buffs.manipulation,
            self.buffs.wast_not,
            if self.buffs.heart_and_soul_used > 0 {
                self.buffs.heart_and_soul + 1
            } else {
                0
            },
            self.buffs.touch_combo_stage,
            self.buffs.observed,
        ]
    }

    pub fn calc_durability(&self, durability: u16) -> u16 {
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
    pub fn craft_point(&self, skill: Actions) -> i32 {
        let cp = match skill {
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
            Actions::FocusedSynthesis => 5,
            Actions::FocusedTouch => 18,
            Actions::Reflect => 6,
            Actions::PreparatoryTouch => 40,
            Actions::Groundwork => 18,
            Actions::DelicateSynthesis => 32,
            Actions::IntensiveSynthesis => 6,
            Actions::TrainedEye => 250,
            Actions::AdvancedTouch => {
                if self.buffs.touch_combo_stage == 2 {
                    18
                } else {
                    46
                }
            }
            Actions::PrudentSynthesis => 18,
            Actions::TrainedFinesse => 32,
            Actions::CarefulObservation => 0,
            Actions::HeartAndSoul => 0,
            // fake actions
            Actions::RapidSynthesisFail => 0,
            Actions::HastyTouchFail => 0,
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
    pub fn cast_action(&mut self, action: Actions) -> Result<(), CastActionError> {
        self.is_action_allowed(action)?;
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
            Actions::FocusedSynthesis => self.cast_synthesis(10, 2.0),
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
                    self.buffs.heart_and_soul = 0;
                }
            }
            Actions::PrudentSynthesis => self.cast_synthesis(5, 1.8),

            Actions::DelicateSynthesis => {
                self.cast_synthesis(0, 1.0);
                self.cast_touch(10, 1.0, 1);
            }

            Actions::BasicTouch => {
                self.cast_touch(10, 1.0, 1);
                self.buffs.touch_combo_stage = 1 + 2;
            }
            Actions::HastyTouch => self.cast_touch(10, 1.0, 1),
            Actions::StandardTouch => {
                if self.buffs.touch_combo_stage == 1 {
                    self.buffs.touch_combo_stage = 2 + 2;
                };
                self.cast_touch(10, 1.25, 1)
            }
            Actions::AdvancedTouch => self.cast_touch(10, 1.5, 1),
            Actions::ByregotsBlessing => {
                let e = (1.0 + self.buffs.inner_quiet as f32 * 0.2).min(3.0);
                self.cast_touch(10, e, -(self.buffs.inner_quiet as i8));
            }
            Actions::PreciseTouch => {
                self.cast_touch(10, 1.5, 2);
                if !matches!(self.condition, Condition::Good | Condition::Excellent) {
                    self.buffs.heart_and_soul = 0;
                }
            }
            Actions::PrudentTouch => self.cast_touch(5, 1.0, 1),
            Actions::FocusedTouch => self.cast_touch(10, 1.5, 1),
            Actions::PreparatoryTouch => self.cast_touch(20, 2.0, 2),
            Actions::TrainedFinesse => self.cast_touch(0, 1.0, 0),

            Actions::TricksOfTheTrade => {
                self.craft_points = (self.craft_points + 20).min(self.attributes.craft_points);
                if !matches!(self.condition, Condition::Good | Condition::Excellent) {
                    self.buffs.heart_and_soul = 0;
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
                return Ok(());
            }
            Actions::MuscleMemory => {
                self.cast_synthesis(10, 3.0);
                self.buffs.muscle_memory = self.new_duration_buff(5);
            }
            Actions::Reflect => {
                self.cast_touch(10, 1.0, 2);
            }
            Actions::TrainedEye => {
                self.quality += self.recipe.quality;
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
                self.buffs.final_appraisal = 5;
                return Ok(());
            }
            Actions::CarefulObservation => {
                self.buffs.careful_observation_used += 1;
                return Ok(());
            }
            Actions::HeartAndSoul => {
                self.buffs.heart_and_soul = 1;
                self.buffs.heart_and_soul_used += 1;
                return Ok(());
            }
            // fake actions
            Actions::RapidSynthesisFail => self.consume_durability(10),
            Actions::HastyTouchFail => self.consume_durability(10),
            Actions::FocusedSynthesisFail => self.consume_durability(10),
            Actions::FocusedTouchFail => self.consume_durability(10),
        }
        if self.buffs.manipulation > 0 && self.durability > 0 {
            self.durability += 5;
            self.durability = self.durability.min(self.recipe.durability);
        }
        self.buffs.next();
        self.step += 1;
        Ok(())
    }

    /// 计算当前状态下某技能的成功概率，返回结果介于[0..=100]之间。
    pub fn success_rate(&self, action: Actions) -> u8 {
        let addon = match self.condition {
            Condition::Centered => 25,
            _ => 0,
        };
        addon
            + match action {
                Actions::HastyTouch => 60,
                Actions::RapidSynthesis => 50,
                Actions::FocusedSynthesis | Actions::FocusedTouch => {
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
    fn is_action_allowed(&self, action: Actions) -> Result<(), CastActionError> {
        use CastActionError::{
            CarefulObservationUsed3, CraftPointNotEnough, CraftingAlreadyFinished,
            DurabilityNotEnough, FocusNeverFailsAfterObserved, HeartAndSoulUsed,
            LevelGapMustGreaterThanTen, NotAllowedInWastNotBuff, OnlyAllowedInFirstStep,
            PlayerLevelTooLow, RequireGoodOrExcellent, RequireInnerQuiet1, RequireInnerQuiet10,
        };

        match action {
            _ if action.unlock_level() > self.attributes.level => Err(PlayerLevelTooLow),

            Actions::TricksOfTheTrade | Actions::IntensiveSynthesis | Actions::PreciseTouch
                if !matches!(self.condition, Condition::Good | Condition::Excellent)
                    && self.buffs.heart_and_soul == 0 =>
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
            Actions::HeartAndSoul if self.buffs.heart_and_soul_used >= 1 => Err(HeartAndSoulUsed),

            Actions::FocusedSynthesisFail | Actions::FocusedTouchFail
                if self.buffs.observed > 0 =>
            {
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

    pub fn simulate_one_step(
        &mut self,
        action: Actions,
        force_success: bool,
        random_condition: bool,
    ) -> Result<(), CastActionError> {
        let is_success = force_success || self.success_rate(action) as f32 / 100.0 > self.rng.gen();
        if is_success {
            self.cast_action(action)?;
        } else {
            self.cast_action(match action {
                Actions::RapidSynthesis => Actions::RapidSynthesisFail,
                Actions::HastyTouch => Actions::HastyTouchFail,
                Actions::FocusedSynthesis => Actions::FocusedSynthesisFail,
                Actions::FocusedTouch => Actions::FocusedTouchFail,
                _ => unreachable!(),
            })?;
        }
        if !matches!(action, Actions::FinalAppraisal | Actions::HeartAndSoul) {
            self.condition = match self.condition {
                Condition::Good if !random_condition => Condition::Normal,
                Condition::Poor if !random_condition => Condition::Excellent,
                Condition::GoodOmen => Condition::Good,
                _ if random_condition => {
                    ConditionIterator::new(
                        self.recipe.conditions_flag as i32,
                        self.attributes.level as i32,
                    )
                    .collect::<Vec<_>>()
                    .choose_weighted(&mut self.rng, |c| c.1)
                    .unwrap()
                    .0
                }
                _ => Condition::Normal,
            };
        }
        Ok(())
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
        let recipe = Recipe::new(517, 50, 100, 50);
        let mut s = Status::new(attr, recipe, data::recipe_level_table(recipe.rlv), None);

        let result = [279, 558, 837, 1000];
        for &pg in &result {
            s.cast_action(Actions::BasicSynthesis).unwrap();
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
        let mut s = Status::new(attr, recipe, data::recipe_level_table(recipe.rlv), None);

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
            s.cast_action(step.a).unwrap();
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
        let mut s = Status::new(attr, recipe, data::recipe_level_table(recipe.rlv), None);

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
            s.cast_action(skill).unwrap();
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
        let mut s = Status::new(attr, recipe, data::recipe_level_table(recipe.rlv), None);

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
                s.cast_action(skill).unwrap();
            } else {
                s.cast_action(match skill {
                    Actions::RapidSynthesis => Actions::RapidSynthesisFail,
                    Actions::HastyTouch => Actions::HastyTouchFail,
                    Actions::FocusedSynthesis => Actions::FocusedSynthesisFail,
                    Actions::FocusedTouch => Actions::FocusedTouchFail,
                    _ => unreachable!(),
                })
                .unwrap()
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
        let s = Status::new(player, recipe, data::recipe_level_table(recipe.rlv), None);
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
                s.cast_action(*sk).unwrap();
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
        let mut s = Status::new(player, recipe, data::recipe_level_table(recipe.rlv), None);
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
            s.cast_action(sk).unwrap();
        }
        assert_eq!(s.quality, 1865);
    }
}

#[pymodule]
fn ffxiv_crafting(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Actions>()?;
    m.add_class::<Buffs>()?;
    m.add_class::<Condition>()?;
    m.add_class::<Attributes>()?;
    m.add_class::<Recipe>()?;
    m.add_class::<RecipeLevel>()?;
    m.add_class::<CastActionError>()?;
    m.add_class::<Status>()?;
    Ok(())
}
