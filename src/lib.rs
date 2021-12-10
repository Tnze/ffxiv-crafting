#[cfg(feature = "serde-support")]
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Skills 是代表了一个玩家在作业时可以使用的一个技能的枚举。
#[cfg_attr(feature = "serde-support", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Skills {
    BasicSynthesis,
    BasicTouch,
    MastersMend,
    HastyTouch,
    RapidSynthesis,
    InnerQuiet,
    Observe,
    TricksOfTheTrade,
    WasteNot,
    Veneration,
    StandardTouch,
    GreatStrides,
    Innovation,
    NameOfTheElements,
    BrandOfTheElements,
    FinalAppraisal,
    WasteNotII,
    ByregotsBlessing,
    PreciseTouch,
    MuscleMemory,
    CarefulSynthesis,
    PatientTouch,
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
    CarefulObservation,
}

impl Skills {
    fn craft_point(&self) -> i32 {
        match self {
            Skills::InnerQuiet => 18,
            Skills::Veneration => 18,
            Skills::GreatStrides => 32,
            Skills::Innovation => 18,
            Skills::NameOfTheElements => 30,

            Skills::Observe => 7,
            Skills::FinalAppraisal => 1,

            Skills::MuscleMemory => 6,
            Skills::Reflect => 24,
            Skills::TrainedEye => 250,

            Skills::BasicSynthesis => 0,
            Skills::RapidSynthesis => 0,
            Skills::BrandOfTheElements => 6,
            Skills::CarefulSynthesis => 7,
            Skills::FocusedSynthesis => 5,
            Skills::Groundwork => 18,
            Skills::IntensiveSynthesis => 6,

            Skills::DelicateSynthesis => 32,

            Skills::BasicTouch => 18,
            Skills::HastyTouch => 0,
            Skills::StandardTouch => 32,
            Skills::ByregotsBlessing => 24,
            Skills::PreciseTouch => 18,
            Skills::PatientTouch => 6,
            Skills::PrudentTouch => 25,
            Skills::FocusedTouch => 18,
            Skills::PreparatoryTouch => 40,

            Skills::TricksOfTheTrade => 0,

            Skills::MastersMend => 88,
            Skills::WasteNot => 56,
            Skills::WasteNotII => 98,
            Skills::Manipulation => 96,

            Skills::CarefulObservation => 0,
        }
    }
    fn unlock_level(&self) -> i32 {
        match self {
            Skills::BasicSynthesis => 1,
            Skills::BasicTouch => 5,
            Skills::MastersMend => 7,
            Skills::HastyTouch => 9,
            Skills::RapidSynthesis => 63,
            Skills::InnerQuiet => 11,
            Skills::Observe => 13,
            Skills::TricksOfTheTrade => 13,
            Skills::WasteNot => 15,
            Skills::Veneration => 15,
            Skills::StandardTouch => 18,
            Skills::GreatStrides => 21,
            Skills::Innovation => 26,
            Skills::NameOfTheElements => 37,
            Skills::BrandOfTheElements => 37,
            Skills::FinalAppraisal => 42,
            Skills::WasteNotII => 47,
            Skills::ByregotsBlessing => 50,
            Skills::PreciseTouch => 53,
            Skills::MuscleMemory => 54,
            Skills::CarefulSynthesis => 62,
            Skills::PatientTouch => 64,
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
            Skills::CarefulObservation => 55,
        }
    }

    /// 将Skills转换为对应技能中文名字的字符串。
    pub fn as_chinese(&self) -> &str {
        match self {
            Skills::BasicSynthesis => "制作",
            Skills::BasicTouch => "加工",
            Skills::MastersMend => "精修",
            Skills::HastyTouch => "仓促",
            Skills::RapidSynthesis => "高速制作",
            Skills::InnerQuiet => "内静",
            Skills::Observe => "观察",
            Skills::TricksOfTheTrade => "秘诀",
            Skills::WasteNot => "俭约",
            Skills::Veneration => "崇敬",
            Skills::StandardTouch => "中级加工",
            Skills::GreatStrides => "阔步",
            Skills::Innovation => "改革",
            Skills::NameOfTheElements => "元素之美名",
            Skills::BrandOfTheElements => "元素之印记",
            Skills::FinalAppraisal => "最终确认",
            Skills::WasteNotII => "长期俭约",
            Skills::ByregotsBlessing => "比尔格的祝福",
            Skills::PreciseTouch => "集中加工",
            Skills::MuscleMemory => "坚信",
            Skills::CarefulSynthesis => "模范制作",
            Skills::PatientTouch => "专心加工",
            Skills::Manipulation => "掌握",
            Skills::PrudentTouch => "俭约加工",
            Skills::FocusedSynthesis => "注视制作",
            Skills::FocusedTouch => "注视加工",
            Skills::Reflect => "闲静",
            Skills::PreparatoryTouch => "坯料加工",
            Skills::Groundwork => "坯料制作",
            Skills::DelicateSynthesis => "精密制作",
            Skills::IntensiveSynthesis => "集中制作",
            Skills::TrainedEye => "工匠的神速技巧",
            Skills::CarefulObservation => "设计变动",
        }
    }
}

impl From<&Skills> for &str {
    fn from(sk: &Skills) -> Self {
        match sk {
            Skills::BasicSynthesis => "basic_synth",
            Skills::BasicTouch => "basic_touch",
            Skills::MastersMend => "masters_mend",
            Skills::HastyTouch => "hasty_touch",
            Skills::RapidSynthesis => "rapid_synth",
            Skills::InnerQuiet => "inner_quiet",
            Skills::Observe => "observe",
            Skills::TricksOfTheTrade => "tricks_of_the_trade",
            Skills::WasteNot => "waste_not",
            Skills::Veneration => "veneration",
            Skills::StandardTouch => "standard_touch",
            Skills::GreatStrides => "great_strides",
            Skills::Innovation => "innovation",
            Skills::NameOfTheElements => "name_of_the_elements",
            Skills::BrandOfTheElements => "brand_of_the_elements",
            Skills::FinalAppraisal => "final_appraisal",
            Skills::WasteNotII => "waste_not_ii",
            Skills::ByregotsBlessing => "byregot_s_blessing",
            Skills::PreciseTouch => "precise_touch",
            Skills::MuscleMemory => "muscle_memory",
            Skills::CarefulSynthesis => "careful_synth",
            Skills::PatientTouch => "patient_touch",
            Skills::Manipulation => "manipulation",
            Skills::PrudentTouch => "prudent_touch",
            Skills::FocusedSynthesis => "focused_synth",
            Skills::FocusedTouch => "focused_touch",
            Skills::Reflect => "reflect",
            Skills::PreparatoryTouch => "preparatory_touch",
            Skills::Groundwork => "groundwork",
            Skills::DelicateSynthesis => "delicate_synth",
            Skills::IntensiveSynthesis => "intensive_synth",
            Skills::TrainedEye => "trained_eye",
            Skills::CarefulObservation => "careful_observation",
        }
    }
}

impl TryFrom<&str> for Skills {
    type Error = UnknownSkillErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "basic_synth" | "制作" => Skills::BasicSynthesis,
            "basic_touch" | "加工" => Skills::BasicTouch,
            "masters_mend" | "精修" => Skills::MastersMend,
            "hasty_touch" | "仓促" => Skills::HastyTouch,
            "rapid_synth" | "高速制作" => Skills::RapidSynthesis,
            "inner_quiet" | "内静" => Skills::InnerQuiet,
            "observe" | "观察" => Skills::Observe,
            "tricks_of_the_trade" | "秘诀" => Skills::TricksOfTheTrade,
            "waste_not" | "俭约" => Skills::WasteNot,
            "veneration" | "崇敬" => Skills::Veneration,
            "standard_touch" | "中级加工" => Skills::StandardTouch,
            "great_strides" | "阔步" => Skills::GreatStrides,
            "innovation" | "改革" => Skills::Innovation,
            "name_of_the_elements" | "元素之美名" => Skills::NameOfTheElements,
            "brand_of_the_elements" | "元素之印记" => Skills::BrandOfTheElements,
            "final_appraisal" | "最终确认" => Skills::FinalAppraisal,
            "waste_not_ii" | "长期俭约" => Skills::WasteNotII,
            "byregot_s_blessing" | "比尔格的祝福" => Skills::ByregotsBlessing,
            "precise_touch" | "集中加工" => Skills::PreciseTouch,
            "muscle_memory" | "坚信" => Skills::MuscleMemory,
            "careful_synth" | "模范制作" => Skills::CarefulSynthesis,
            "patient_touch" | "专心加工" => Skills::PatientTouch,
            "manipulation" | "掌握" => Skills::Manipulation,
            "prudent_touch" | "俭约加工" => Skills::PrudentTouch,
            "focused_synth" | "注视制作" => Skills::FocusedSynthesis,
            "focused_touch" | "注视加工" => Skills::FocusedTouch,
            "reflect" | "闲静" => Skills::Reflect,
            "preparatory_touch" | "坯料加工" => Skills::PreparatoryTouch,
            "groundwork" | "坯料制作" => Skills::Groundwork,
            "delicate_synth" | "精密制作" => Skills::DelicateSynthesis,
            "intensive_synth" | "集中制作" => Skills::IntensiveSynthesis,
            "trained_eye" | "工匠的神速技巧" => Skills::TrainedEye,
            "careful_observation" | "设计变动" => Skills::CarefulObservation,
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

/// Condition 代表了当前的“制作状态”，也就是俗称的球色。
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

#[derive(Copy, Clone)]
pub struct Attributes {
    pub level: i32,
    pub craftsmanship: i32,
    pub control: i32,
    pub craft_points: i32,
}

/// Recipe 储存了一次制作中配方的信息。
#[derive(Copy, Clone)]
pub struct Recipe {
    /// 配方品级
    pub rlv: i32,

    /// 制作配方所需的玩家等级
    pub level: i32,

    /// 制作精度
    pub craftsmanship: i32,

    /// 加工精度
    pub control: i32,

    /// 难度（最大进展）
    pub progress: i32,

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
    pub cond_flag: i32,
}

impl Recipe {
    pub fn new(
        rlv: i32,
        level: i32,
        progress: i32,
        quality: i32,
        durability: i32,
        cond_flag: i32,
    ) -> Self {
        let suggest = match rlv {
            1 => Some((22, 11)),
            2 => Some((22, 11)),
            3 => Some((22, 11)),
            4 => Some((22, 11)),
            5 => Some((50, 25)),
            6 => Some((50, 25)),
            7 => Some((50, 25)),
            8 => Some((59, 29)),
            9 => Some((59, 29)),
            10 => Some((59, 29)),
            11 => Some((67, 33)),
            12 => Some((67, 33)),
            13 => Some((67, 33)),
            14 => Some((67, 33)),
            15 => Some((67, 33)),
            16 => Some((78, 39)),
            17 => Some((78, 39)),
            18 => Some((78, 39)),
            19 => Some((82, 41)),
            20 => Some((94, 47)),
            21 => Some((94, 47)),
            22 => Some((94, 47)),
            23 => Some((99, 49)),
            24 => Some((99, 49)),
            25 => Some((99, 49)),
            26 => Some((99, 49)),
            27 => Some((99, 49)),
            28 => Some((106, 53)),
            29 => Some((106, 53)),
            30 => Some((106, 53)),
            31 => Some((121, 60)),
            32 => Some((121, 60)),
            33 => Some((121, 60)),
            34 => Some((129, 64)),
            35 => Some((129, 64)),
            36 => Some((129, 64)),
            37 => Some((129, 64)),
            38 => Some((129, 64)),
            39 => Some((136, 68)),
            40 => Some((136, 68)),
            41 => Some((136, 68)),
            42 => Some((150, 75)),
            43 => Some((150, 75)),
            44 => Some((150, 75)),
            45 => Some((150, 75)),
            46 => Some((150, 75)),
            47 => Some((161, 80)),
            48 => Some((161, 80)),
            49 => Some((161, 80)),
            50 => Some((176, 88)),
            51 => Some((281, 281)),
            52 => Some((291, 291)),
            53 => Some((302, 302)),
            54 => Some((314, 314)),
            55 => Some((325, 325)),
            56 => Some((325, 325)),
            57 => Some((325, 325)),
            58 => Some((325, 325)),
            59 => Some((325, 325)),
            60 => Some((325, 325)),
            61 => Some((325, 325)),
            62 => Some((325, 325)),
            63 => Some((325, 325)),
            64 => Some((325, 325)),
            65 => Some((325, 325)),
            66 => Some((325, 325)),
            67 => Some((325, 325)),
            68 => Some((325, 325)),
            69 => Some((325, 325)),
            70 => Some((325, 325)),
            71 => Some((329, 328)),
            72 => Some((332, 330)),
            73 => Some((335, 333)),
            74 => Some((339, 335)),
            75 => Some((342, 338)),
            76 => Some((345, 340)),
            77 => Some((349, 343)),
            78 => Some((352, 345)),
            79 => Some((355, 348)),
            80 => Some((358, 350)),
            81 => Some((362, 352)),
            82 => Some((365, 355)),
            83 => Some((368, 357)),
            84 => Some((372, 360)),
            85 => Some((375, 362)),
            86 => Some((378, 365)),
            87 => Some((382, 367)),
            88 => Some((385, 370)),
            89 => Some((388, 372)),
            90 => Some((391, 374)),
            91 => Some((394, 376)),
            92 => Some((397, 378)),
            93 => Some((400, 379)),
            94 => Some((403, 381)),
            95 => Some((406, 383)),
            96 => Some((409, 384)),
            97 => Some((412, 386)),
            98 => Some((415, 388)),
            99 => Some((418, 389)),
            100 => Some((421, 391)),
            101 => Some((424, 393)),
            102 => Some((427, 394)),
            103 => Some((430, 396)),
            104 => Some((433, 398)),
            105 => Some((436, 399)),
            106 => Some((439, 401)),
            107 => Some((442, 403)),
            108 => Some((445, 404)),
            109 => Some((448, 406)),
            110 => Some((451, 407)),
            111 => Some((455, 411)),
            112 => Some((458, 415)),
            113 => Some((462, 418)),
            114 => Some((465, 422)),
            115 => Some((468, 426)),
            116 => Some((472, 429)),
            117 => Some((475, 433)),
            118 => Some((479, 437)),
            119 => Some((482, 440)),
            120 => Some((485, 444)),
            121 => Some((489, 448)),
            122 => Some((492, 451)),
            123 => Some((495, 455)),
            124 => Some((499, 458)),
            125 => Some((502, 462)),
            126 => Some((506, 466)),
            127 => Some((509, 469)),
            128 => Some((512, 473)),
            129 => Some((516, 477)),
            130 => Some((519, 480)),
            131 => Some((522, 484)),
            132 => Some((526, 488)),
            133 => Some((529, 491)),
            134 => Some((533, 495)),
            135 => Some((536, 498)),
            136 => Some((539, 502)),
            137 => Some((543, 506)),
            138 => Some((546, 509)),
            139 => Some((550, 513)),
            140 => Some((553, 517)),
            141 => Some((556, 520)),
            142 => Some((560, 524)),
            143 => Some((563, 528)),
            144 => Some((566, 531)),
            145 => Some((570, 535)),
            146 => Some((573, 539)),
            147 => Some((577, 542)),
            148 => Some((580, 546)),
            149 => Some((583, 549)),
            150 => Some((587, 553)),
            151 => Some((590, 557)),
            152 => Some((593, 560)),
            153 => Some((597, 564)),
            154 => Some((600, 568)),
            155 => Some((604, 571)),
            156 => Some((607, 575)),
            157 => Some((610, 579)),
            158 => Some((614, 582)),
            159 => Some((617, 586)),
            160 => Some((620, 589)),
            161 => Some((625, 595)),
            162 => Some((630, 600)),
            163 => Some((635, 605)),
            164 => Some((640, 611)),
            165 => Some((645, 616)),
            166 => Some((650, 621)),
            167 => Some((655, 627)),
            168 => Some((660, 632)),
            169 => Some((665, 637)),
            170 => Some((669, 642)),
            171 => Some((674, 648)),
            172 => Some((679, 653)),
            173 => Some((684, 658)),
            174 => Some((689, 664)),
            175 => Some((694, 669)),
            176 => Some((699, 674)),
            177 => Some((704, 680)),
            178 => Some((709, 685)),
            179 => Some((714, 690)),
            180 => Some((718, 695)),
            181 => Some((723, 700)),
            182 => Some((727, 704)),
            183 => Some((732, 708)),
            184 => Some((736, 712)),
            185 => Some((740, 716)),
            186 => Some((745, 720)),
            187 => Some((749, 725)),
            188 => Some((754, 729)),
            189 => Some((758, 733)),
            190 => Some((762, 737)),
            191 => Some((767, 741)),
            192 => Some((771, 745)),
            193 => Some((776, 750)),
            194 => Some((780, 754)),
            195 => Some((784, 758)),
            196 => Some((789, 762)),
            197 => Some((793, 766)),
            198 => Some((798, 770)),
            199 => Some((802, 775)),
            200 => Some((806, 779)),
            201 => Some((811, 783)),
            202 => Some((815, 787)),
            203 => Some((820, 791)),
            204 => Some((824, 795)),
            205 => Some((828, 800)),
            206 => Some((833, 804)),
            207 => Some((837, 808)),
            208 => Some((842, 812)),
            209 => Some((846, 816)),
            210 => Some((850, 820)),
            211 => Some((852, 822)),
            212 => Some((854, 823)),
            213 => Some((856, 825)),
            214 => Some((858, 826)),
            215 => Some((860, 828)),
            216 => Some((862, 829)),
            217 => Some((864, 831)),
            218 => Some((866, 832)),
            219 => Some((868, 834)),
            220 => Some((870, 835)),
            221 => Some((875, 839)),
            222 => Some((879, 843)),
            223 => Some((883, 847)),
            224 => Some((887, 851)),
            225 => Some((891, 855)),
            226 => Some((895, 859)),
            227 => Some((900, 863)),
            228 => Some((904, 867)),
            229 => Some((908, 871)),
            230 => Some((912, 875)),
            231 => Some((916, 879)),
            232 => Some((920, 883)),
            233 => Some((925, 887)),
            234 => Some((929, 891)),
            235 => Some((933, 895)),
            236 => Some((937, 899)),
            237 => Some((941, 903)),
            238 => Some((945, 907)),
            239 => Some((950, 911)),
            240 => Some((954, 915)),
            241 => Some((958, 919)),
            242 => Some((962, 923)),
            243 => Some((966, 927)),
            244 => Some((970, 931)),
            245 => Some((975, 935)),
            246 => Some((979, 939)),
            247 => Some((983, 943)),
            248 => Some((987, 947)),
            249 => Some((991, 951)),
            250 => Some((995, 955)),
            251 => Some((998, 958)),
            252 => Some((1000, 960)),
            253 => Some((1002, 963)),
            254 => Some((1004, 965)),
            255 => Some((1006, 968)),
            256 => Some((1008, 970)),
            257 => Some((1010, 973)),
            258 => Some((1012, 975)),
            259 => Some((1014, 978)),
            260 => Some((1016, 980)),
            261 => Some((1019, 983)),
            262 => Some((1021, 985)),
            263 => Some((1023, 988)),
            264 => Some((1025, 990)),
            265 => Some((1027, 993)),
            266 => Some((1029, 995)),
            267 => Some((1031, 998)),
            268 => Some((1033, 1000)),
            269 => Some((1035, 1003)),
            270 => Some((1037, 1005)),
            271 => Some((1040, 1008)),
            272 => Some((1042, 1010)),
            273 => Some((1044, 1013)),
            274 => Some((1046, 1015)),
            275 => Some((1048, 1018)),
            276 => Some((1050, 1020)),
            277 => Some((1052, 1023)),
            278 => Some((1054, 1025)),
            279 => Some((1056, 1028)),
            280 => Some((1058, 1030)),
            281 => Some((1061, 1033)),
            282 => Some((1063, 1035)),
            283 => Some((1065, 1038)),
            284 => Some((1067, 1040)),
            285 => Some((1069, 1043)),
            286 => Some((1071, 1045)),
            287 => Some((1073, 1048)),
            288 => Some((1075, 1050)),
            289 => Some((1077, 1053)),
            290 => Some((1079, 1055)),
            291 => Some((1082, 1058)),
            292 => Some((1084, 1060)),
            293 => Some((1086, 1063)),
            294 => Some((1088, 1065)),
            295 => Some((1090, 1068)),
            296 => Some((1092, 1070)),
            297 => Some((1094, 1073)),
            298 => Some((1096, 1075)),
            299 => Some((1098, 1078)),
            300 => Some((1100, 1080)),
            301 => Some((1111, 1087)),
            302 => Some((1122, 1094)),
            303 => Some((1133, 1101)),
            304 => Some((1144, 1108)),
            305 => Some((1155, 1115)),
            306 => Some((1166, 1122)),
            307 => Some((1177, 1129)),
            308 => Some((1188, 1136)),
            309 => Some((1199, 1143)),
            310 => Some((1210, 1150)),
            311 => Some((1221, 1157)),
            312 => Some((1232, 1164)),
            313 => Some((1243, 1171)),
            314 => Some((1254, 1178)),
            315 => Some((1265, 1185)),
            316 => Some((1276, 1192)),
            317 => Some((1287, 1199)),
            318 => Some((1298, 1206)),
            319 => Some((1309, 1213)),
            320 => Some((1320, 1220)),
            321 => Some((1326, 1224)),
            322 => Some((1332, 1229)),
            323 => Some((1338, 1233)),
            324 => Some((1344, 1237)),
            325 => Some((1350, 1242)),
            326 => Some((1356, 1246)),
            327 => Some((1362, 1250)),
            328 => Some((1368, 1255)),
            329 => Some((1374, 1259)),
            330 => Some((1380, 1263)),
            331 => Some((1386, 1268)),
            332 => Some((1392, 1272)),
            333 => Some((1398, 1276)),
            334 => Some((1404, 1281)),
            335 => Some((1410, 1285)),
            336 => Some((1416, 1289)),
            337 => Some((1422, 1294)),
            338 => Some((1428, 1298)),
            339 => Some((1434, 1302)),
            340 => Some((1440, 1307)),
            341 => Some((1446, 1311)),
            342 => Some((1452, 1315)),
            343 => Some((1458, 1320)),
            344 => Some((1464, 1324)),
            345 => Some((1470, 1328)),
            346 => Some((1476, 1333)),
            347 => Some((1482, 1337)),
            348 => Some((1488, 1341)),
            349 => Some((1494, 1346)),
            350 => Some((1500, 1350)),
            351 => Some((1505, 1358)),
            352 => Some((1510, 1366)),
            353 => Some((1515, 1374)),
            354 => Some((1520, 1383)),
            355 => Some((1525, 1391)),
            356 => Some((1530, 1399)),
            357 => Some((1535, 1408)),
            358 => Some((1540, 1416)),
            359 => Some((1545, 1424)),
            360 => Some((1550, 1433)),
            361 => Some((1555, 1441)),
            362 => Some((1560, 1449)),
            363 => Some((1565, 1458)),
            364 => Some((1570, 1466)),
            365 => Some((1575, 1474)),
            366 => Some((1580, 1483)),
            367 => Some((1585, 1491)),
            368 => Some((1590, 1499)),
            369 => Some((1595, 1508)),
            370 => Some((1600, 1516)),
            371 => Some((1605, 1524)),
            372 => Some((1610, 1533)),
            373 => Some((1615, 1541)),
            374 => Some((1620, 1549)),
            375 => Some((1625, 1558)),
            376 => Some((1630, 1566)),
            377 => Some((1635, 1574)),
            378 => Some((1640, 1583)),
            379 => Some((1645, 1592)),
            380 => Some((1650, 1600)),
            381 => Some((1320, 1220)),
            382 => Some((1320, 1220)),
            383 => Some((1320, 1220)),
            384 => Some((1320, 1220)),
            385 => Some((1320, 1220)),
            386 => Some((1320, 1220)),
            387 => Some((1320, 1220)),
            388 => Some((1320, 1220)),
            389 => Some((1320, 1220)),
            390 => Some((1320, 1220)),
            391 => Some((1334, 1233)),
            392 => Some((1347, 1246)),
            393 => Some((1361, 1258)),
            394 => Some((1375, 1271)),
            395 => Some((1388, 1284)),
            396 => Some((1402, 1297)),
            397 => Some((1416, 1310)),
            398 => Some((1429, 1323)),
            399 => Some((1443, 1335)),
            400 => Some((1457, 1348)),
            401 => Some((1470, 1361)),
            402 => Some((1484, 1374)),
            403 => Some((1498, 1387)),
            404 => Some((1511, 1400)),
            405 => Some((1525, 1412)),
            406 => Some((1539, 1425)),
            407 => Some((1552, 1438)),
            408 => Some((1566, 1451)),
            409 => Some((1580, 1464)),
            410 => Some((1593, 1477)),
            411 => Some((1607, 1489)),
            412 => Some((1621, 1502)),
            413 => Some((1634, 1515)),
            414 => Some((1648, 1528)),
            415 => Some((1662, 1541)),
            416 => Some((1675, 1554)),
            417 => Some((1689, 1566)),
            418 => Some((1702, 1579)),
            419 => Some((1716, 1592)),
            420 => Some((1730, 1605)),
            421 => Some((1743, 1618)),
            422 => Some((1757, 1631)),
            423 => Some((1771, 1643)),
            424 => Some((1784, 1656)),
            425 => Some((1798, 1669)),
            426 => Some((1812, 1682)),
            427 => Some((1825, 1695)),
            428 => Some((1839, 1708)),
            429 => Some((1853, 1720)),
            430 => Some((1866, 1733)),
            431 => Some((1880, 1746)),
            432 => Some((1894, 1759)),
            433 => Some((1907, 1772)),
            434 => Some((1921, 1785)),
            435 => Some((1935, 1797)),
            436 => Some((1948, 1810)),
            437 => Some((1962, 1823)),
            438 => Some((1976, 1836)),
            439 => Some((1989, 1849)),
            440 => Some((2000, 1860)),
            441 => Some((2017, 1874)),
            442 => Some((2030, 1887)),
            443 => Some((2044, 1900)),
            444 => Some((2058, 1913)),
            445 => Some((2071, 1926)),
            446 => Some((2085, 1938)),
            447 => Some((2099, 1951)),
            448 => Some((2112, 1964)),
            449 => Some((2126, 1977)),
            450 => Some((2140, 1990)),
            451 => Some((2151, 1996)),
            452 => Some((2162, 2002)),
            453 => Some((2173, 2008)),
            454 => Some((2184, 2014)),
            455 => Some((2195, 2020)),
            456 => Some((2206, 2027)),
            457 => Some((2217, 2034)),
            458 => Some((2228, 2041)),
            459 => Some((2239, 2048)),
            460 => Some((2250, 2055)),
            461 => Some((2261, 2062)),
            462 => Some((2272, 2069)),
            463 => Some((2283, 2076)),
            464 => Some((2294, 2083)),
            465 => Some((2305, 2090)),
            466 => Some((2316, 2097)),
            467 => Some((2327, 2104)),
            468 => Some((2338, 2111)),
            469 => Some((2349, 2118)),
            470 => Some((2360, 2125)),
            471 => Some((2372, 2132)),
            472 => Some((2384, 2139)),
            473 => Some((2396, 2146)),
            474 => Some((2408, 2153)),
            475 => Some((2420, 2160)),
            476 => Some((2432, 2167)),
            477 => Some((2444, 2174)),
            478 => Some((2456, 2181)),
            479 => Some((2468, 2188)),
            480 => Some((2480, 2195)),
            481 => Some((2484, 2206)),
            482 => Some((2488, 2217)),
            483 => Some((2492, 2228)),
            484 => Some((2496, 2239)),
            485 => Some((2500, 2250)),
            486 => Some((2504, 2261)),
            487 => Some((2508, 2272)),
            488 => Some((2512, 2283)),
            489 => Some((2516, 2294)),
            490 => Some((2520, 2305)),
            491 => Some((2525, 2316)),
            492 => Some((2530, 2327)),
            493 => Some((2535, 2338)),
            494 => Some((2540, 2349)),
            495 => Some((2545, 2360)),
            496 => Some((2550, 2372)),
            497 => Some((2555, 2384)),
            498 => Some((2560, 2396)),
            499 => Some((2565, 2408)),
            500 => Some((2570, 2420)),
            501 => Some((2575, 2432)),
            502 => Some((2580, 2444)),
            503 => Some((2585, 2456)),
            504 => Some((2590, 2468)),
            505 => Some((2595, 2480)),
            506 => Some((2600, 2492)),
            507 => Some((2605, 2504)),
            508 => Some((2610, 2516)),
            509 => Some((2615, 2528)),
            510 => Some((2620, 2540)),
            511 => Some((2620, 2540)),
            512 => Some((2620, 2540)),
            513 => Some((2620, 2540)),
            514 => Some((2620, 2540)),
            515 => Some((2620, 2540)),
            516 => Some((2620, 2540)),
            517 => Some((2620, 2540)),
            518 => Some((2620, 2540)),
            519 => Some((2620, 2540)),
            520 => Some((2620, 2540)),
            _ => None,
        }
            .unwrap();

        Recipe {
            rlv,
            level,
            craftsmanship: suggest.0,
            control: suggest.1,
            progress,
            quality,
            durability,
            cond_flag,
        }
    }
}

/// Buffs 储存了一次制作中玩家全部buff状态信息
#[derive(Copy, Clone, Default)]
pub struct Buffs {
    pub name_of_the_elements: Option<DurationBuff>,
    pub muscle_memory: Option<DurationBuff>,
    pub great_strides: Option<DurationBuff>,
    pub veneration: Option<DurationBuff>,
    pub innovation: Option<DurationBuff>,
    pub inner_quiet: Option<LayerBuff>,
    pub final_appraisal: Option<DurationBuff>,
    pub manipulation: Option<DurationBuff>,
    pub wast_not: Option<DurationBuff>,
    pub standard_touch_prepared: Option<DurationBuff>,
    pub observed: Option<DurationBuff>,
}

/// LayerBuff 代表拥有层数的buff，且层数不会随着制作回合衰减，唯一的例子就是内静。
#[derive(Copy, Clone, Debug)]
pub struct LayerBuff(pub usize);

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

    pub(crate) fn brand_of_the_elements(&mut self, left_progress: f32) -> f32 {
        const SKILL_E: f32 = 1.0;
        let mut e = 0.0;
        if self.muscle_memory.take().is_some() {
            e += 1.0;
        }
        if self.veneration.is_some() {
            e += 0.5;
        }
        if self.name_of_the_elements.is_some() {
            e += 2.0 * (left_progress * 1e2).ceil() / 1e2;
        }
        SKILL_E * round_down(1.0 + e, 100.0)
    }

    pub(crate) fn touch(&mut self, skill_e: f32) -> f32 {
        let mut e = 0.0;
        if self.great_strides.take().is_some() {
            e += 1.0;
        }
        if self.innovation.is_some() {
            e += 0.5;
        }
        skill_e * round_down(1.0 + e, 100.0)
    }

    pub(crate) fn fade(&mut self) {
        fade(&mut self.name_of_the_elements);
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
    pub progress: i32,
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
    /// 内静状态下无法使用内静
    InnerQuietWhenAlreadyExist,
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
            CastActionError::InnerQuietWhenAlreadyExist => "inner quiet when exist",
        })
    }
}

impl Error for CastActionError {}

impl Status {
    pub fn new(attributes: Attributes, recipe: Recipe) -> Self {
        let durability = recipe.durability;
        let craft_points = attributes.craft_points;
        Status {
            buffs: Buffs::default(),
            attributes,
            recipe,
            durability,
            craft_points,
            condition: Condition::Normal,
            progress: 0,
            quality: 0,
            step: 0,
        }
    }

    fn control(&self) -> i32 {
        let mut c = self.attributes.control;
        if let Some(LayerBuff(layer)) = &self.buffs.inner_quiet {
            c += (0.2 * (layer - 1) as f32 * c as f32) as i32
        }
        c
    }

    fn consume_durability(&mut self, durability: i32) {
        let mut reduce = durability;
        if let Condition::Sturdy = self.condition {
            reduce -= reduce / 2;
        }
        if self.buffs.wast_not.is_some() {
            reduce -= reduce / 2;
        }
        self.durability -= reduce;
    }

    fn consume_craft_point(&mut self, cp: i32) {
        let mut reduce = cp;
        if let Condition::Pliant = self.condition {
            reduce -= reduce / 2;
        }
        self.craft_points -= reduce;
    }

    pub fn calc_synthesis(&self, efficiency: f32) -> i32 {
        let factor = self.level_diff_factor();
        let craftsmanship = self.attributes.craftsmanship;
        let standard_progress = factor.0
            * (0.21 * craftsmanship as f32 + 2.0)
            * (10000 + craftsmanship) as f32
            / (10000 + self.recipe.craftsmanship) as f32;
        (standard_progress.floor() * self.condition.synth_ratio() * efficiency) as i32
    }

    pub fn calc_touch(&self, efficiency: f32) -> i32 {
        let factor = self.level_diff_factor();
        let control = self.control();
        let standard_quality = factor.1
            * self.condition.touch_ratio()
            * (0.35 * control as f32 + 35.0)
            * (10000 + control) as f32
            / (10000 + self.recipe.control) as f32;
        (standard_quality.floor() * efficiency) as i32
    }

    fn cast_synthesis(&mut self, cp: i32, durability: i32, efficiency: f32) {
        let e = self.buffs.synthesis(efficiency);
        self.progress += self.calc_synthesis(e);
        self.consume_durability(durability);
        self.consume_craft_point(cp);
        if self.progress >= self.recipe.progress && self.buffs.final_appraisal.take().is_some() {
            self.progress = self.recipe.progress - 1;
        }
    }

    fn cast_touch(&mut self, cp: i32, durability: i32, efficiency: f32, inner_quiet_addon: usize) {
        let e = self.buffs.touch(efficiency);
        self.quality += self.calc_touch(e);
        self.consume_durability(durability);
        self.consume_craft_point(cp);
        if let Some(LayerBuff(inner_quiet)) = &mut self.buffs.inner_quiet {
            *inner_quiet = (*inner_quiet + inner_quiet_addon).min(11);
        }
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
                let e = if self.attributes.level < 31 { 1.0 } else { 1.2 };
                self.cast_synthesis(0, 10, e);
            }
            Skills::RapidSynthesis => {
                let e = if self.attributes.level < 63 { 2.5 } else { 5.0 };
                self.cast_synthesis(0, 10, e);
            }
            Skills::BrandOfTheElements => {
                let r = 1.0 - self.progress as f32 / self.recipe.progress as f32;
                let e = self.buffs.brand_of_the_elements(r);
                self.progress += self.calc_synthesis(e);
                self.consume_durability(10);
                self.consume_craft_point(6);
            }
            Skills::CarefulSynthesis => self.cast_synthesis(7, 10, 1.5),
            Skills::FocusedSynthesis => self.cast_synthesis(5, 10, 2.0),
            Skills::Groundwork => {
                let e = if self.durability < 20 { 1.5 } else { 3.0 };
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
                let mut e = 1.0;
                if let Some(LayerBuff(b)) = self.buffs.inner_quiet {
                    e += 0.2 * (b - 1) as f32;
                }
                self.cast_touch(24, 10, e, 1);
                self.buffs.inner_quiet = None;
            }
            Skills::PreciseTouch => self.cast_touch(18, 10, 1.5, 2),
            Skills::PatientTouch => {
                let inner_quiet_addon = match self.buffs.inner_quiet {
                    Some(LayerBuff(iq)) => iq,
                    _ => 0,
                };
                self.cast_touch(6, 10, 1.0, inner_quiet_addon)
            }
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
                self.cast_touch(24, 10, 1.0, 1);
                self.buffs.inner_quiet = Some(LayerBuff(3))
            }
            Skills::TrainedEye => {
                self.consume_craft_point(250);
                self.quality += self.recipe.quality;
            }

            Skills::InnerQuiet => {
                self.consume_craft_point(18);
                self.buffs.inner_quiet = Some(LayerBuff(1));
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
            Skills::NameOfTheElements => {
                self.consume_craft_point(30);
                self.buffs.name_of_the_elements = self.new_duration_buff(3);
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
            Skills::CarefulObservation => {
                self.condition = Condition::Normal;
                return;
            }
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
            Skills::PatientTouch => {
                self.consume_durability(10);
                self.consume_craft_point(6);
                if let Some(LayerBuff(layer)) = &mut self.buffs.inner_quiet {
                    *layer -= *layer / 2;
                }
            }
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
            Skills::PatientTouch => 0.5,
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
            InnerQuietWhenAlreadyExist, LevelGapMustGreaterThanTen, NotAllowedInWastNotBuff,
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
            if !matches!(self.condition, Condition::Good | Condition::Excellent) =>
                {
                    Err(RequireGoodOrExcellent)
                }
            Skills::PrudentTouch if self.buffs.wast_not.is_some() => Err(NotAllowedInWastNotBuff),
            Skills::MuscleMemory | Skills::Reflect | Skills::TrainedEye if self.step != 0 => {
                Err(OnlyAllowedInFirstStep)
            }
            Skills::TrainedEye if self.attributes.level - self.recipe.level < 10 => {
                Err(LevelGapMustGreaterThanTen)
            }
            Skills::InnerQuiet if self.buffs.inner_quiet.is_some() => {
                Err(InnerQuietWhenAlreadyExist)
            }
            _ if self.durability <= 0 => Err(DurabilityNotEnough),
            _ if cp > self.craft_points => Err(CraftPointNotEnough),
            _ if self.progress >= self.recipe.progress => Err(CraftingAlreadyFinished),
            _ => Ok(()),
        }
    }

    /// 本次制作是否已经结束。
    pub fn is_finished(&self) -> bool {
        self.progress >= self.recipe.progress || self.durability <= 0
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

    fn level_diff_factor(&self) -> (f32, f32) {
        let crafter_level = match self.attributes.level {
            x if x <= 50 => x,
            51 => 120,
            52 => 125,
            53 => 130,
            54 => 133,
            55 => 136,
            56 => 139,
            57 => 142,
            58 => 145,
            59 => 148,
            60 => 150,
            61 => 260,
            62 => 265,
            63 => 270,
            64 => 273,
            65 => 276,
            66 => 279,
            67 => 282,
            68 => 285,
            69 => 288,
            70 => 290,
            71 => 390,
            72 => 395,
            73 => 400,
            74 => 403,
            75 => 406,
            76 => 409,
            77 => 412,
            78 => 415,
            79 => 418,
            80 => 420,
            x => unreachable!("Player level cannot be {}", x),
        };
        match crafter_level - self.recipe.rlv {
            x if x <= -30 => (0.80, 0.60),
            -29 => (0.82, 0.64),
            -28 => (0.84, 0.68),
            -27 => (0.86, 0.72),
            -26 => (0.88, 0.76),
            -25 => (0.90, 0.80),
            -24 => (0.92, 0.84),
            -23 => (0.94, 0.88),
            -22 => (0.96, 0.92),
            -21 => (0.98, 0.96),
            x if x <= 0 => (1.00, 1.00),
            1 => (1.05, 1.00),
            2 => (1.10, 1.00),
            3 => (1.15, 1.00),
            4 => (1.20, 1.00),
            5 => (1.25, 1.00),
            6 => (1.27, 1.00),
            7 => (1.29, 1.00),
            8 => (1.31, 1.00),
            9 => (1.33, 1.00),
            10 => (1.35, 1.00),
            11 => (1.37, 1.00),
            12 => (1.39, 1.00),
            13 => (1.41, 1.00),
            14 => (1.43, 1.00),
            15 => (1.45, 1.00),
            16 => (1.46, 1.00),
            17 => (1.47, 1.00),
            18 => (1.48, 1.00),
            19 => (1.49, 1.00),
            x if x >= 20 => (1.50, 1.00),
            x => unreachable!(
                "Level difference cannot be {} - {} = {}",
                crafter_level, self.recipe.rlv, x
            ),
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
///     println!("出现 {} 的概率为: {}", c, p);
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
    use super::{Attributes, Condition, ConditionIterator, Recipe, Skills, Status};

    #[test]
    fn hq_probability() {
        let attributes = Attributes {
            level: 80,
            craftsmanship: 1069,
            control: 981,
            craft_points: 357,
        };
        let recipe = Recipe::new(480, 80, 0, 20287, 35, 15);
        let mut s = Status::new(attributes, recipe);

        s.quality = 15620;
        assert_eq!(s.high_quality_probability().unwrap(), 52);
        s.quality = 15621;
        assert_eq!(s.high_quality_probability().unwrap(), 58);
    }

    #[test]
    fn craft_zefir() {
        let attributes = Attributes {
            level: 80,
            craftsmanship: 2549,
            control: 2565,
            craft_points: 357,
        };

        let recipe = Recipe::new(510, 80, 8591, 56662, 70, 15);
        assert_eq!(recipe.craftsmanship, 2620);
        assert_eq!(recipe.control, 2540);
        assert_eq!(recipe.progress, 8591);
        assert_eq!(recipe.quality, 56662);
        assert_eq!(recipe.durability, 70);

        let mut s = Status::new(attributes, recipe);
        for step in [
            (Skills::BasicSynthesis, 60, 357, 512, 0),
            (Skills::DelicateSynthesis, 50, 325, 939, 560),
            (Skills::InnerQuiet, 50, 307, 939, 560),
            (Skills::Groundwork, 30, 289, 2220, 560),
            (Skills::DelicateSynthesis, 20, 257, 2647, 1120),
            (Skills::NameOfTheElements, 20, 227, 2647, 1120),
            (Skills::BrandOfTheElements, 10, 221, 3671, 1120),
            (Skills::MastersMend, 40, 133, 3671, 1120),
            (Skills::Innovation, 40, 115, 3671, 1120),
            (Skills::StandardTouch, 30, 83, 3671, 2425),
            (Skills::ByregotsBlessing, 20, 59, 3671, 4188),
        ]
            .iter()
        {
            s.cast_action(step.0);
            assert_eq!(s.durability, step.1);
            assert_eq!(s.craft_points, step.2);
            assert_eq!(s.progress, step.3);
            assert_eq!(s.quality, step.4);
        }
    }

    // Grade 4 Skybuilders' All-purpose Infusion
    fn craft_infusion() -> (Attributes, Recipe) {
        (
            Attributes {
                level: 76,
                craftsmanship: 1069,
                control: 981,
                craft_points: 357,
            },
            Recipe::new(290, 70, 2214, 32860, 60, 15),
        )
    }

    #[test]
    fn craft_infusion1() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for step in [
            (Skills::InnerQuiet, 60, 339, 0, 0),
            (Skills::DelicateSynthesis, 50, 307, 339, 375),
            (Skills::Innovation, 50, 289, 339, 375),
            (Skills::DelicateSynthesis, 40, 257, 678, 1051),
            (Skills::DelicateSynthesis, 30, 225, 1017, 1846),
            (Skills::ByregotsBlessing, 20, 201, 1017, 3312),
            (Skills::StandardTouch, 10, 169, 1017, 4015),
        ]
            .iter()
        {
            s.cast_action(step.0);
            assert_eq!(s.durability, step.1);
            assert_eq!(s.craft_points, step.2);
            assert_eq!(s.progress, step.3);
            assert_eq!(s.quality, step.4);
        }
    }

    #[test]
    fn craft_infusion2() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for step in [
            (Skills::MuscleMemory, 50, 351, 1017, 0),
            (Skills::Groundwork, 30, 333, 3051, 0),
        ]
            .iter()
        {
            s.cast_action(step.0);
            assert_eq!(s.durability, step.1);
            assert_eq!(s.craft_points, step.2);
            assert_eq!(s.progress, step.3);
            assert_eq!(s.quality, step.4);
        }
    }

    #[test]
    fn craft_infusion3() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for step in [
            (Skills::Reflect, 50, 333, 0, 375),
            (Skills::MastersMend, 60, 245, 0, 375),
            (Skills::MastersMend, 60, 157, 0, 375),
            (Skills::MastersMend, 60, 69, 0, 375),
        ]
            .iter()
        {
            s.cast_action(step.0);
            assert_eq!(s.durability, step.1);
            assert_eq!(s.craft_points, step.2);
            assert_eq!(s.progress, step.3);
            assert_eq!(s.quality, step.4);
        }
    }

    #[test]
    fn craft_infusion4() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for step in [
            (Skills::MuscleMemory, 50, 351, 1017, 0),
            (Skills::CarefulSynthesis, 40, 344, 2034, 0),
            (Skills::GreatStrides, 40, 312, 2034, 0),
            (Skills::MastersMend, 60, 224, 2034, 0),
            (Skills::InnerQuiet, 60, 206, 2034, 0),
            (Skills::PreparatoryTouch, 40, 166, 2034, 1500),
            (Skills::Innovation, 40, 148, 2034, 1500),
            (Skills::StandardTouch, 30, 116, 2034, 2493),
            (Skills::PreparatoryTouch, 10, 76, 2034, 4326),
            (Skills::DelicateSynthesis, 0, 44, 2373, 5496),
        ]
            .iter()
        {
            s.cast_action(step.0);
            assert_eq!(s.durability, step.1);
            assert_eq!(s.craft_points, step.2);
            assert_eq!(s.progress, step.3);
            assert_eq!(s.quality, step.4);
        }
    }

    #[test]
    fn test_infusion() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for &step in [
            Skills::Reflect,
            Skills::Innovation,
            Skills::WasteNotII,
            Skills::BasicTouch,
            Skills::BasicTouch,
            Skills::BasicTouch,
            Skills::Innovation,
            Skills::Observe,
            Skills::FocusedTouch,
            Skills::GreatStrides,
            Skills::ByregotsBlessing,
            Skills::Veneration,
            Skills::Groundwork,
            Skills::CarefulSynthesis,
        ]
            .iter()
        {
            s.cast_action(step);
        }
        assert_eq!(s.progress, 2287);
        assert_eq!(s.quality, 9661);
        assert_eq!(s.craft_points, 21);
    }

    #[test]
    fn test_manipulation() {
        let (attributes, recipe) = craft_infusion();
        let mut s = Status::new(attributes, recipe);
        for &step in [
            Skills::Manipulation,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::PrudentTouch,
            Skills::BasicTouch,
        ]
            .iter()
        {
            s.cast_action(step);
        }
        assert_eq!(s.durability, 50);
    }

    #[test]
    fn test_condition1() {
        let a = Attributes {
            level: 80,
            craftsmanship: 2782,
            control: 2848,
            craft_points: 563,
        };
        let r = Recipe::new(515, 80, 12132, 78180, 55, 483);
        let mut s = Status::new(a, r);
        for step in [
            (Skills::Reflect, true, 0, 634, Condition::Good),
            (
                Skills::IntensiveSynthesis,
                true,
                1896,
                634,
                Condition::Pliant,
            ),
            (Skills::Manipulation, true, 1896, 634, Condition::Malleable),
            (
                Skills::RapidSynthesis,
                true,
                5451,
                634,
                Condition::Malleable,
            ),
            (Skills::RapidSynthesis, true, 9006, 634, Condition::Normal),
            (Skills::RapidSynthesis, true, 11376, 634, Condition::Normal),
        ]
            .iter()
        {
            if step.1 {
                s.cast_action(step.0);
            } else {
                s.fail_action(step.0);
            }
            s.condition = step.4;
            assert_eq!(s.progress, step.2);
            assert_eq!(s.quality, step.3);
        }
        /*
        ,失败,0,0,0,122,0,0:
        中级加工,成功,24,11376,27656,122,0,1:
        加工,成功,24,11376,24301,140,10,1:
        中级加工,成功,23,11376,21617,158,20,1:
        加工,成功,22,11376,18262,176,30,9:
        精修,成功,21,11376,15578,194,40,8:
        集中加工,成功,20,11376,15578,282,10,1:
        仓促,成功,19,11376,10658,300,20,2:
        集中加工,成功,18,11376,8703,300,25,1:
        仓促,失败,17,11376,5276,318,30,2:
        精修,成功,16,11376,5276,318,35,1:
        秘诀,成功,15,11376,5276,386,5,9:
        仓促,成功,14,11376,5276,386,5,2:
        俭约,成功,13,11376,3292,386,10,1:
        仓促,成功,12,11376,3292,442,10,9:
        改革,成功,11,11376,1591,442,15,8:
        仓促,失败,10,11376,1591,460,10,9:
        仓促,失败,9,11376,1591,460,15,1:
        俭约加工,成功,8,11376,1591,460,20,1:
        高速制作,成功,7,11376,634,485,20,1:
        高速制作,成功,6,9006,634,485,25,1:
        高速制作,成功,5,5451,634,485,30,8:
        掌握,成功,4,1896,634,485,35,8:
        集中制作,成功,3,1896,634,533,35,7:
        闲静,成功,2,0,634,539,45,2:
                */
    }

    #[test]
    fn test_condition2() {
        let a = Attributes {
            level: 80,
            craftsmanship: 2782,
            control: 2848,
            craft_points: 563,
        };
        let r = Recipe::new(516, 80, 13024, 85033, 60, 499);
        let mut s = Status::new(a, r);
        for (step, (skill, success, quality, condition)) in [
            (Skills::Reflect, true, 634, Condition::Primed),
            (Skills::RapidSynthesis, true, 634, Condition::Normal),
            (Skills::RapidSynthesis, false, 634, Condition::Sturdy),
            (Skills::RapidSynthesis, true, 634, Condition::Malleable),
            (Skills::RapidSynthesis, true, 634, Condition::Sturdy),
            (Skills::PrudentTouch, true, 1591, Condition::Normal),
            (Skills::MastersMend, true, 1591, Condition::Primed),
            (Skills::Innovation, true, 1591, Condition::Malleable),
            (Skills::RapidSynthesis, false, 1591, Condition::Normal),
            (Skills::RapidSynthesis, true, 1591, Condition::Normal),
            (Skills::Manipulation, true, 1591, Condition::Sturdy),
            (Skills::HastyTouch, false, 1591, Condition::Normal),
            (Skills::PrudentTouch, true, 3292, Condition::Pliant),
            (Skills::WasteNotII, true, 3292, Condition::Centered),
            (Skills::FinalAppraisal, true, 3292, Condition::Centered),
            (Skills::RapidSynthesis, true, 3292, Condition::Normal),
            (Skills::Innovation, true, 3292, Condition::Sturdy),
            (Skills::PreparatoryTouch, true, 7261, Condition::Pliant),
            (Skills::PreparatoryTouch, true, 12460, Condition::Good),
            (Skills::PreciseTouch, true, 19840, Condition::Centered),
        ]
            .iter()
            .enumerate()
        {
            if *success {
                s.cast_action(*skill);
            } else {
                s.fail_action(*skill);
            }
            assert_eq!(
                s.quality, *quality,
                "quality lose sync in [{}]{:?}",
                step, skill
            );
            s.condition = *condition;
        }
        assert_eq!(s.quality, 19840)
    }

    #[test]
    fn test_condition_iter() {
        for flag in [499, 483, 115, 15] {
            let mut rate_all = 0.0;
            for (_cond, rate) in ConditionIterator::new(flag, 80) {
                rate_all += rate;
            }
            assert_eq!(rate_all, 1.0);
        }
    }

    #[test]
    fn calculate_the_minimum_attribute() {
        for control in 0..2818 {
            let attributes = Attributes {
                level: 80,
                craftsmanship: 2553,
                control,
                craft_points: 467,
            };
            let recipe = Recipe::new(412, 77, 3586, 15742, 80, 15);
            let mut s = Status::new(attributes, recipe);
            s.cast_action(Skills::Reflect);
            s.cast_action(Skills::DelicateSynthesis);
            s.cast_action(Skills::DelicateSynthesis);
            s.cast_action(Skills::PreparatoryTouch);
            s.cast_action(Skills::ByregotsBlessing);
            s.cast_action(Skills::Groundwork);
            if let Some(100) = s.high_quality_probability() {
                println!("Minimum control: {}", control);
                break;
            }
        }
    }
}
