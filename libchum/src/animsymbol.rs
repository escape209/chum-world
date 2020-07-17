//! Check out https://github.com/Jellonator/chum-world/wiki/Animation-Symbol-Enum for more information.
//! Don't worry, I didn't type all of this out by hand, I'm not that crazy.
//! I used a python script to generate most of this.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimSymbol {
    AnimDefault,
    AnimWait,
    AnimAnim1,
    AnimAnim2,
    AnimAnim3,
    AnimAction,
    AnimWait01,
    AnimWait02,
    AnimWait03,
    AnimWait04,
    AnimWait05,
    AnimWalk,
    AnimWalk01,
    AnimWalk02,
    AnimJog,
    AnimRun,
    AnimRun01,
    AnimRun02,
    AnimSkidIn,
    AnimSkid,
    AnimSkidOut,
    AnimJump,
    AnimJump01,
    AnimJump02,
    AnimJumpPwr,
    AnimJumpToFall,
    AnimJumpFall,
    AnimJumpFall01,
    AnimJumpFall02,
    AnimJumpLand,
    AnimHookHang,
    AnimAttack,
    AnimAttack01,
    AnimAttack02,
    AnimAttack03,
    AnimAttack04,
    AnimAttack05,
    AnimAttack06,
    AnimAttack07,
    AnimAttack08,
    AnimAttack09,
    AnimJumpAttack,
    AnimMovingAttack,
    AnimPwrAttack,
    AnimPwrAttack01,
    AnimPwrAttack02,
    AnimPwrAttack03,
    AnimAttackJump,
    AnimAttackJump01,
    AnimAttackJump02,
    AnimAttackJump03,
    AnimAttackJumpKick,
    AnimAttackJumpKick01,
    AnimAttackJumpKick02,
    AnimAttackJumpKick03,
    AnimAttackJumpPunch,
    AnimAttackJumpPunch01,
    AnimAttackJumpPunch02,
    AnimAttackJumpPunch03,
    AnimChopWndup,
    AnimDuckAttack,
    AnimDuckIn,
    AnimDuckIn01,
    AnimDuckIn02,
    AnimDuckHld,
    AnimDuckOut,
    AnimBellyBump,
    AnimBellySlam,
    AnimBellySlamLand,
    AnimBellySlamFlip,
    AnimGetHit,
    AnimGetHit01,
    AnimGetHit02,
    AnimDefeated,
    AnimDefeated01,
    AnimDefeated02,
    AnimLaugh,
    AnimLaugh01,
    AnimLaugh02,
    AnimTalkStd,
    AnimTalkStd01,
    AnimTalkStd02,
    AnimTalkStd03,
    AnimTalkStd04,
    AnimTalkStd05,
    AnimTalkStd06,
    AnimTalkStd07,
    AnimTalkStd08,
    AnimTalkStd09,
    AnimTalkStd10,
    AnimTalkStd11,
    AnimTalkStd12,
    AnimTalkStd13,
    AnimTalkStd14,
    AnimTalkStd15,
    AnimTalkStd16,
    AnimTalkStd17,
    AnimTalkStd18,
    AnimTalkStd19,
    AnimTalkStd20,
    AnimTalkStd21,
    AnimTalkStd22,
    AnimTalkStd23,
    AnimTalkStd24,
    AnimTalkStd25,
    AnimTalkStd26,
    AnimTalkStd27,
    AnimTalkStd28,
    AnimTalkStd29,
    AnimGlide,
    AnimGlideLand,
    AnimRoll,
    AnimSlide,
    AnimSlide01,
    AnimSlide02,
    AnimPickup,
    AnimPickup01,
    AnimPickup02,
    AnimCarry,
    AnimJumpCarry,
    AnimCarryFall,
    AnimDrop,
    AnimHold,
    AnimThrow,
    AnimThrow01,
    AnimThrow02,
    AnimJfCompress,
    AnimJfPump,
    AnimJfGlide,
    AnimOpen,
    AnimOpening,
    AnimClosed,
    AnimClosing,
    AnimTucked,
    AnimTuckOut,
    AnimSpring,
    AnimAlerted,
    AnimAlerted01,
    AnimAlerted02,
    AnimAlertedIdle,
    AnimAlertedIdle01,
    AnimAlertedIdle02,
    AnimAttackCharge,
    AnimAttackCharge01,
    AnimAttackCharge02,
    AnimJumpMoving,
    AnimJumpMoving01,
    AnimJumpMoving02,
    AnimFly,
    AnimFly01,
    AnimFly02,
    AnimRunAttack,
    AnimMisc,
    AnimMisc01,
    AnimMisc02,
    AnimMisc03,
    AnimMisc04,
    AnimMisc05,
    AnimMisc06,
    AnimMisc07,
    AnimMisc08,
    AnimMisc09,
    AnimMisc10,
    AnimMisc11,
    AnimMisc12,
    AnimMisc13,
    AnimMisc14,
    AnimMisc15,
    AnimMisc16,
    AnimMisc17,
    AnimMisc18,
    AnimMisc19,
    AnimMisc20,
    AnimMisc21,
    AnimMisc22,
    AnimMisc23,
    AnimMisc24,
    AnimMisc25,
    AnimMisc26,
    AnimMisc27,
    AnimMisc28,
    AnimMisc29,
    AnimJfMoveAll,
    AnimTurn,
    AnimTalkHold,
    AnimTalkHold01,
    AnimTalkHold02,
}

impl AnimSymbol {
    pub fn from_u32(value: u32) -> Option<AnimSymbol> {
        match value {
            0 => Some(AnimSymbol::AnimDefault),
            1 => Some(AnimSymbol::AnimWait),
            2 => Some(AnimSymbol::AnimAnim1),
            3 => Some(AnimSymbol::AnimAnim2),
            4 => Some(AnimSymbol::AnimAnim3),
            5 => Some(AnimSymbol::AnimAction),
            6 => Some(AnimSymbol::AnimWait01),
            7 => Some(AnimSymbol::AnimWait02),
            8 => Some(AnimSymbol::AnimWait03),
            9 => Some(AnimSymbol::AnimWait04),
            10 => Some(AnimSymbol::AnimWait05),
            11 => Some(AnimSymbol::AnimWalk),
            12 => Some(AnimSymbol::AnimWalk01),
            13 => Some(AnimSymbol::AnimWalk02),
            14 => Some(AnimSymbol::AnimJog),
            15 => Some(AnimSymbol::AnimRun),
            16 => Some(AnimSymbol::AnimRun01),
            17 => Some(AnimSymbol::AnimRun02),
            18 => Some(AnimSymbol::AnimSkidIn),
            19 => Some(AnimSymbol::AnimSkid),
            20 => Some(AnimSymbol::AnimSkidOut),
            21 => Some(AnimSymbol::AnimJump),
            22 => Some(AnimSymbol::AnimJump01),
            23 => Some(AnimSymbol::AnimJump02),
            24 => Some(AnimSymbol::AnimJumpPwr),
            25 => Some(AnimSymbol::AnimJumpToFall),
            26 => Some(AnimSymbol::AnimJumpFall),
            27 => Some(AnimSymbol::AnimJumpFall01),
            28 => Some(AnimSymbol::AnimJumpFall02),
            29 => Some(AnimSymbol::AnimJumpLand),
            30 => Some(AnimSymbol::AnimHookHang),
            31 => Some(AnimSymbol::AnimAttack),
            32 => Some(AnimSymbol::AnimAttack01),
            33 => Some(AnimSymbol::AnimAttack02),
            34 => Some(AnimSymbol::AnimAttack03),
            35 => Some(AnimSymbol::AnimAttack04),
            36 => Some(AnimSymbol::AnimAttack05),
            37 => Some(AnimSymbol::AnimAttack06),
            38 => Some(AnimSymbol::AnimAttack07),
            39 => Some(AnimSymbol::AnimAttack08),
            40 => Some(AnimSymbol::AnimAttack09),
            41 => Some(AnimSymbol::AnimJumpAttack),
            42 => Some(AnimSymbol::AnimMovingAttack),
            43 => Some(AnimSymbol::AnimPwrAttack),
            44 => Some(AnimSymbol::AnimPwrAttack01),
            45 => Some(AnimSymbol::AnimPwrAttack02),
            46 => Some(AnimSymbol::AnimPwrAttack03),
            47 => Some(AnimSymbol::AnimAttackJump),
            48 => Some(AnimSymbol::AnimAttackJump01),
            49 => Some(AnimSymbol::AnimAttackJump02),
            50 => Some(AnimSymbol::AnimAttackJump03),
            51 => Some(AnimSymbol::AnimAttackJumpKick),
            52 => Some(AnimSymbol::AnimAttackJumpKick01),
            53 => Some(AnimSymbol::AnimAttackJumpKick02),
            54 => Some(AnimSymbol::AnimAttackJumpKick03),
            55 => Some(AnimSymbol::AnimAttackJumpPunch),
            56 => Some(AnimSymbol::AnimAttackJumpPunch01),
            57 => Some(AnimSymbol::AnimAttackJumpPunch02),
            58 => Some(AnimSymbol::AnimAttackJumpPunch03),
            59 => Some(AnimSymbol::AnimChopWndup),
            60 => Some(AnimSymbol::AnimDuckAttack),
            61 => Some(AnimSymbol::AnimDuckIn),
            62 => Some(AnimSymbol::AnimDuckIn01),
            63 => Some(AnimSymbol::AnimDuckIn02),
            64 => Some(AnimSymbol::AnimDuckHld),
            65 => Some(AnimSymbol::AnimDuckOut),
            66 => Some(AnimSymbol::AnimBellyBump),
            67 => Some(AnimSymbol::AnimBellySlam),
            68 => Some(AnimSymbol::AnimBellySlamLand),
            69 => Some(AnimSymbol::AnimBellySlamFlip),
            70 => Some(AnimSymbol::AnimGetHit),
            71 => Some(AnimSymbol::AnimGetHit01),
            72 => Some(AnimSymbol::AnimGetHit02),
            73 => Some(AnimSymbol::AnimDefeated),
            74 => Some(AnimSymbol::AnimDefeated01),
            75 => Some(AnimSymbol::AnimDefeated02),
            76 => Some(AnimSymbol::AnimLaugh),
            77 => Some(AnimSymbol::AnimLaugh01),
            78 => Some(AnimSymbol::AnimLaugh02),
            79 => Some(AnimSymbol::AnimTalkStd),
            80 => Some(AnimSymbol::AnimTalkStd01),
            81 => Some(AnimSymbol::AnimTalkStd02),
            82 => Some(AnimSymbol::AnimTalkStd03),
            83 => Some(AnimSymbol::AnimTalkStd04),
            84 => Some(AnimSymbol::AnimTalkStd05),
            85 => Some(AnimSymbol::AnimTalkStd06),
            86 => Some(AnimSymbol::AnimTalkStd07),
            87 => Some(AnimSymbol::AnimTalkStd08),
            88 => Some(AnimSymbol::AnimTalkStd09),
            89 => Some(AnimSymbol::AnimTalkStd10),
            90 => Some(AnimSymbol::AnimTalkStd11),
            91 => Some(AnimSymbol::AnimTalkStd12),
            92 => Some(AnimSymbol::AnimTalkStd13),
            93 => Some(AnimSymbol::AnimTalkStd14),
            94 => Some(AnimSymbol::AnimTalkStd15),
            95 => Some(AnimSymbol::AnimTalkStd16),
            96 => Some(AnimSymbol::AnimTalkStd17),
            97 => Some(AnimSymbol::AnimTalkStd18),
            98 => Some(AnimSymbol::AnimTalkStd19),
            99 => Some(AnimSymbol::AnimTalkStd20),
            100 => Some(AnimSymbol::AnimTalkStd21),
            101 => Some(AnimSymbol::AnimTalkStd22),
            102 => Some(AnimSymbol::AnimTalkStd23),
            103 => Some(AnimSymbol::AnimTalkStd24),
            104 => Some(AnimSymbol::AnimTalkStd25),
            105 => Some(AnimSymbol::AnimTalkStd26),
            106 => Some(AnimSymbol::AnimTalkStd27),
            107 => Some(AnimSymbol::AnimTalkStd28),
            108 => Some(AnimSymbol::AnimTalkStd29),
            109 => Some(AnimSymbol::AnimGlide),
            110 => Some(AnimSymbol::AnimGlideLand),
            111 => Some(AnimSymbol::AnimRoll),
            112 => Some(AnimSymbol::AnimSlide),
            113 => Some(AnimSymbol::AnimSlide01),
            114 => Some(AnimSymbol::AnimSlide02),
            115 => Some(AnimSymbol::AnimPickup),
            116 => Some(AnimSymbol::AnimPickup01),
            117 => Some(AnimSymbol::AnimPickup02),
            118 => Some(AnimSymbol::AnimCarry),
            119 => Some(AnimSymbol::AnimJumpCarry),
            120 => Some(AnimSymbol::AnimCarryFall),
            121 => Some(AnimSymbol::AnimDrop),
            122 => Some(AnimSymbol::AnimHold),
            123 => Some(AnimSymbol::AnimThrow),
            124 => Some(AnimSymbol::AnimThrow01),
            125 => Some(AnimSymbol::AnimThrow02),
            126 => Some(AnimSymbol::AnimJfCompress),
            127 => Some(AnimSymbol::AnimJfPump),
            128 => Some(AnimSymbol::AnimJfGlide),
            129 => Some(AnimSymbol::AnimOpen),
            130 => Some(AnimSymbol::AnimOpening),
            131 => Some(AnimSymbol::AnimClosed),
            132 => Some(AnimSymbol::AnimClosing),
            133 => Some(AnimSymbol::AnimTucked),
            134 => Some(AnimSymbol::AnimTuckOut),
            135 => Some(AnimSymbol::AnimSpring),
            136 => Some(AnimSymbol::AnimAlerted),
            137 => Some(AnimSymbol::AnimAlerted01),
            138 => Some(AnimSymbol::AnimAlerted02),
            139 => Some(AnimSymbol::AnimAlertedIdle),
            140 => Some(AnimSymbol::AnimAlertedIdle01),
            141 => Some(AnimSymbol::AnimAlertedIdle02),
            142 => Some(AnimSymbol::AnimAttackCharge),
            143 => Some(AnimSymbol::AnimAttackCharge01),
            144 => Some(AnimSymbol::AnimAttackCharge02),
            145 => Some(AnimSymbol::AnimJumpMoving),
            146 => Some(AnimSymbol::AnimJumpMoving01),
            147 => Some(AnimSymbol::AnimJumpMoving02),
            148 => Some(AnimSymbol::AnimFly),
            149 => Some(AnimSymbol::AnimFly01),
            150 => Some(AnimSymbol::AnimFly02),
            151 => Some(AnimSymbol::AnimRunAttack),
            152 => Some(AnimSymbol::AnimMisc),
            153 => Some(AnimSymbol::AnimMisc01),
            154 => Some(AnimSymbol::AnimMisc02),
            155 => Some(AnimSymbol::AnimMisc03),
            156 => Some(AnimSymbol::AnimMisc04),
            157 => Some(AnimSymbol::AnimMisc05),
            158 => Some(AnimSymbol::AnimMisc06),
            159 => Some(AnimSymbol::AnimMisc07),
            160 => Some(AnimSymbol::AnimMisc08),
            161 => Some(AnimSymbol::AnimMisc09),
            162 => Some(AnimSymbol::AnimMisc10),
            163 => Some(AnimSymbol::AnimMisc11),
            164 => Some(AnimSymbol::AnimMisc12),
            165 => Some(AnimSymbol::AnimMisc13),
            166 => Some(AnimSymbol::AnimMisc14),
            167 => Some(AnimSymbol::AnimMisc15),
            168 => Some(AnimSymbol::AnimMisc16),
            169 => Some(AnimSymbol::AnimMisc17),
            170 => Some(AnimSymbol::AnimMisc18),
            171 => Some(AnimSymbol::AnimMisc19),
            172 => Some(AnimSymbol::AnimMisc20),
            173 => Some(AnimSymbol::AnimMisc21),
            174 => Some(AnimSymbol::AnimMisc22),
            175 => Some(AnimSymbol::AnimMisc23),
            176 => Some(AnimSymbol::AnimMisc24),
            177 => Some(AnimSymbol::AnimMisc25),
            178 => Some(AnimSymbol::AnimMisc26),
            179 => Some(AnimSymbol::AnimMisc27),
            180 => Some(AnimSymbol::AnimMisc28),
            181 => Some(AnimSymbol::AnimMisc29),
            182 => Some(AnimSymbol::AnimJfMoveAll),
            183 => Some(AnimSymbol::AnimTurn),
            184 => Some(AnimSymbol::AnimTalkHold),
            185 => Some(AnimSymbol::AnimTalkHold01),
            186 => Some(AnimSymbol::AnimTalkHold02),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            AnimSymbol::AnimDefault => 0,
            AnimSymbol::AnimWait => 1,
            AnimSymbol::AnimAnim1 => 2,
            AnimSymbol::AnimAnim2 => 3,
            AnimSymbol::AnimAnim3 => 4,
            AnimSymbol::AnimAction => 5,
            AnimSymbol::AnimWait01 => 6,
            AnimSymbol::AnimWait02 => 7,
            AnimSymbol::AnimWait03 => 8,
            AnimSymbol::AnimWait04 => 9,
            AnimSymbol::AnimWait05 => 10,
            AnimSymbol::AnimWalk => 11,
            AnimSymbol::AnimWalk01 => 12,
            AnimSymbol::AnimWalk02 => 13,
            AnimSymbol::AnimJog => 14,
            AnimSymbol::AnimRun => 15,
            AnimSymbol::AnimRun01 => 16,
            AnimSymbol::AnimRun02 => 17,
            AnimSymbol::AnimSkidIn => 18,
            AnimSymbol::AnimSkid => 19,
            AnimSymbol::AnimSkidOut => 20,
            AnimSymbol::AnimJump => 21,
            AnimSymbol::AnimJump01 => 22,
            AnimSymbol::AnimJump02 => 23,
            AnimSymbol::AnimJumpPwr => 24,
            AnimSymbol::AnimJumpToFall => 25,
            AnimSymbol::AnimJumpFall => 26,
            AnimSymbol::AnimJumpFall01 => 27,
            AnimSymbol::AnimJumpFall02 => 28,
            AnimSymbol::AnimJumpLand => 29,
            AnimSymbol::AnimHookHang => 30,
            AnimSymbol::AnimAttack => 31,
            AnimSymbol::AnimAttack01 => 32,
            AnimSymbol::AnimAttack02 => 33,
            AnimSymbol::AnimAttack03 => 34,
            AnimSymbol::AnimAttack04 => 35,
            AnimSymbol::AnimAttack05 => 36,
            AnimSymbol::AnimAttack06 => 37,
            AnimSymbol::AnimAttack07 => 38,
            AnimSymbol::AnimAttack08 => 39,
            AnimSymbol::AnimAttack09 => 40,
            AnimSymbol::AnimJumpAttack => 41,
            AnimSymbol::AnimMovingAttack => 42,
            AnimSymbol::AnimPwrAttack => 43,
            AnimSymbol::AnimPwrAttack01 => 44,
            AnimSymbol::AnimPwrAttack02 => 45,
            AnimSymbol::AnimPwrAttack03 => 46,
            AnimSymbol::AnimAttackJump => 47,
            AnimSymbol::AnimAttackJump01 => 48,
            AnimSymbol::AnimAttackJump02 => 49,
            AnimSymbol::AnimAttackJump03 => 50,
            AnimSymbol::AnimAttackJumpKick => 51,
            AnimSymbol::AnimAttackJumpKick01 => 52,
            AnimSymbol::AnimAttackJumpKick02 => 53,
            AnimSymbol::AnimAttackJumpKick03 => 54,
            AnimSymbol::AnimAttackJumpPunch => 55,
            AnimSymbol::AnimAttackJumpPunch01 => 56,
            AnimSymbol::AnimAttackJumpPunch02 => 57,
            AnimSymbol::AnimAttackJumpPunch03 => 58,
            AnimSymbol::AnimChopWndup => 59,
            AnimSymbol::AnimDuckAttack => 60,
            AnimSymbol::AnimDuckIn => 61,
            AnimSymbol::AnimDuckIn01 => 62,
            AnimSymbol::AnimDuckIn02 => 63,
            AnimSymbol::AnimDuckHld => 64,
            AnimSymbol::AnimDuckOut => 65,
            AnimSymbol::AnimBellyBump => 66,
            AnimSymbol::AnimBellySlam => 67,
            AnimSymbol::AnimBellySlamLand => 68,
            AnimSymbol::AnimBellySlamFlip => 69,
            AnimSymbol::AnimGetHit => 70,
            AnimSymbol::AnimGetHit01 => 71,
            AnimSymbol::AnimGetHit02 => 72,
            AnimSymbol::AnimDefeated => 73,
            AnimSymbol::AnimDefeated01 => 74,
            AnimSymbol::AnimDefeated02 => 75,
            AnimSymbol::AnimLaugh => 76,
            AnimSymbol::AnimLaugh01 => 77,
            AnimSymbol::AnimLaugh02 => 78,
            AnimSymbol::AnimTalkStd => 79,
            AnimSymbol::AnimTalkStd01 => 80,
            AnimSymbol::AnimTalkStd02 => 81,
            AnimSymbol::AnimTalkStd03 => 82,
            AnimSymbol::AnimTalkStd04 => 83,
            AnimSymbol::AnimTalkStd05 => 84,
            AnimSymbol::AnimTalkStd06 => 85,
            AnimSymbol::AnimTalkStd07 => 86,
            AnimSymbol::AnimTalkStd08 => 87,
            AnimSymbol::AnimTalkStd09 => 88,
            AnimSymbol::AnimTalkStd10 => 89,
            AnimSymbol::AnimTalkStd11 => 90,
            AnimSymbol::AnimTalkStd12 => 91,
            AnimSymbol::AnimTalkStd13 => 92,
            AnimSymbol::AnimTalkStd14 => 93,
            AnimSymbol::AnimTalkStd15 => 94,
            AnimSymbol::AnimTalkStd16 => 95,
            AnimSymbol::AnimTalkStd17 => 96,
            AnimSymbol::AnimTalkStd18 => 97,
            AnimSymbol::AnimTalkStd19 => 98,
            AnimSymbol::AnimTalkStd20 => 99,
            AnimSymbol::AnimTalkStd21 => 100,
            AnimSymbol::AnimTalkStd22 => 101,
            AnimSymbol::AnimTalkStd23 => 102,
            AnimSymbol::AnimTalkStd24 => 103,
            AnimSymbol::AnimTalkStd25 => 104,
            AnimSymbol::AnimTalkStd26 => 105,
            AnimSymbol::AnimTalkStd27 => 106,
            AnimSymbol::AnimTalkStd28 => 107,
            AnimSymbol::AnimTalkStd29 => 108,
            AnimSymbol::AnimGlide => 109,
            AnimSymbol::AnimGlideLand => 110,
            AnimSymbol::AnimRoll => 111,
            AnimSymbol::AnimSlide => 112,
            AnimSymbol::AnimSlide01 => 113,
            AnimSymbol::AnimSlide02 => 114,
            AnimSymbol::AnimPickup => 115,
            AnimSymbol::AnimPickup01 => 116,
            AnimSymbol::AnimPickup02 => 117,
            AnimSymbol::AnimCarry => 118,
            AnimSymbol::AnimJumpCarry => 119,
            AnimSymbol::AnimCarryFall => 120,
            AnimSymbol::AnimDrop => 121,
            AnimSymbol::AnimHold => 122,
            AnimSymbol::AnimThrow => 123,
            AnimSymbol::AnimThrow01 => 124,
            AnimSymbol::AnimThrow02 => 125,
            AnimSymbol::AnimJfCompress => 126,
            AnimSymbol::AnimJfPump => 127,
            AnimSymbol::AnimJfGlide => 128,
            AnimSymbol::AnimOpen => 129,
            AnimSymbol::AnimOpening => 130,
            AnimSymbol::AnimClosed => 131,
            AnimSymbol::AnimClosing => 132,
            AnimSymbol::AnimTucked => 133,
            AnimSymbol::AnimTuckOut => 134,
            AnimSymbol::AnimSpring => 135,
            AnimSymbol::AnimAlerted => 136,
            AnimSymbol::AnimAlerted01 => 137,
            AnimSymbol::AnimAlerted02 => 138,
            AnimSymbol::AnimAlertedIdle => 139,
            AnimSymbol::AnimAlertedIdle01 => 140,
            AnimSymbol::AnimAlertedIdle02 => 141,
            AnimSymbol::AnimAttackCharge => 142,
            AnimSymbol::AnimAttackCharge01 => 143,
            AnimSymbol::AnimAttackCharge02 => 144,
            AnimSymbol::AnimJumpMoving => 145,
            AnimSymbol::AnimJumpMoving01 => 146,
            AnimSymbol::AnimJumpMoving02 => 147,
            AnimSymbol::AnimFly => 148,
            AnimSymbol::AnimFly01 => 149,
            AnimSymbol::AnimFly02 => 150,
            AnimSymbol::AnimRunAttack => 151,
            AnimSymbol::AnimMisc => 152,
            AnimSymbol::AnimMisc01 => 153,
            AnimSymbol::AnimMisc02 => 154,
            AnimSymbol::AnimMisc03 => 155,
            AnimSymbol::AnimMisc04 => 156,
            AnimSymbol::AnimMisc05 => 157,
            AnimSymbol::AnimMisc06 => 158,
            AnimSymbol::AnimMisc07 => 159,
            AnimSymbol::AnimMisc08 => 160,
            AnimSymbol::AnimMisc09 => 161,
            AnimSymbol::AnimMisc10 => 162,
            AnimSymbol::AnimMisc11 => 163,
            AnimSymbol::AnimMisc12 => 164,
            AnimSymbol::AnimMisc13 => 165,
            AnimSymbol::AnimMisc14 => 166,
            AnimSymbol::AnimMisc15 => 167,
            AnimSymbol::AnimMisc16 => 168,
            AnimSymbol::AnimMisc17 => 169,
            AnimSymbol::AnimMisc18 => 170,
            AnimSymbol::AnimMisc19 => 171,
            AnimSymbol::AnimMisc20 => 172,
            AnimSymbol::AnimMisc21 => 173,
            AnimSymbol::AnimMisc22 => 174,
            AnimSymbol::AnimMisc23 => 175,
            AnimSymbol::AnimMisc24 => 176,
            AnimSymbol::AnimMisc25 => 177,
            AnimSymbol::AnimMisc26 => 178,
            AnimSymbol::AnimMisc27 => 179,
            AnimSymbol::AnimMisc28 => 180,
            AnimSymbol::AnimMisc29 => 181,
            AnimSymbol::AnimJfMoveAll => 182,
            AnimSymbol::AnimTurn => 183,
            AnimSymbol::AnimTalkHold => 184,
            AnimSymbol::AnimTalkHold01 => 185,
            AnimSymbol::AnimTalkHold02 => 186,
        }
    }
}