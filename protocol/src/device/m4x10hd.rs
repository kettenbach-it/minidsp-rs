//
// This file is generated by `minidsp-devtools codegen`. DO NOT EDIT.
//
use super::*;
pub mod sym {
    #[allow(dead_code)]
    pub const NON_MOD_RAM_ALLOC: u16 = 0;
    pub const DC_INP_ALG_1: u16 = 8;
    pub const DC_INP_ALG_2: u16 = 9;
    pub const DC_INP_ALG_3: u16 = 10;
    pub const GAIN_1940_ALG_NS11: u16 = 11;
    pub const GAIN_1940_ALG_NS12: u16 = 12;
    pub const GAIN_1940_ALG_NS13: u16 = 13;
    pub const GAIN_1940_ALG_NS14: u16 = 14;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_0: u16 = 15;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_1: u16 = 16;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_2: u16 = 17;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_3: u16 = 18;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_4: u16 = 19;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_5: u16 = 20;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_6: u16 = 21;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_7: u16 = 22;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_8: u16 = 23;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_9: u16 = 24;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_10: u16 = 25;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_11: u16 = 26;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_12: u16 = 27;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_13: u16 = 28;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_14: u16 = 29;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_15: u16 = 30;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_16: u16 = 31;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_17: u16 = 32;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_18: u16 = 33;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_19: u16 = 34;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_20: u16 = 35;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_21: u16 = 36;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_22: u16 = 37;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_23: u16 = 38;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_24: u16 = 39;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_25: u16 = 40;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_26: u16 = 41;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_27: u16 = 42;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_28: u16 = 43;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_29: u16 = 44;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_30: u16 = 45;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_31: u16 = 46;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_32: u16 = 47;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_33: u16 = 48;
    pub const MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1RMS: u16 = 49;
    pub const MONOMUX_194_0NS_3_0: u16 = 50;
    pub const MONOMUX_194_0NS_3_1: u16 = 51;
    pub const MONOMUX_194_0NS_3_2: u16 = 52;
    pub const MUTE_NO_SLEW_ALG_7_1MUTE: u16 = 53;
    pub const MUTE_NO_SLEW_ALG_7_2MUTE: u16 = 54;
    pub const MUTE_NO_SLEW_ALG_7_3MUTE: u16 = 55;
    pub const MUTE_NO_SLEW_ALG_7_4MUTE: u16 = 56;
    pub const MONOMUX_194_0NS_2_0: u16 = 57;
    pub const MONOMUX_194_0NS_2_1: u16 = 58;
    pub const PEQ_13_1: u16 = 59;
    pub const PEQ_13_2: u16 = 64;
    pub const PEQ_13_3: u16 = 69;
    pub const PEQ_13_4: u16 = 74;
    pub const PEQ_13_5: u16 = 79;
    pub const PEQ_14_1: u16 = 84;
    pub const PEQ_14_2: u16 = 89;
    pub const PEQ_14_3: u16 = 94;
    pub const PEQ_14_4: u16 = 99;
    pub const PEQ_14_5: u16 = 104;
    pub const PEQ_12_1: u16 = 109;
    pub const PEQ_12_2: u16 = 114;
    pub const PEQ_12_3: u16 = 119;
    pub const PEQ_12_4: u16 = 124;
    pub const PEQ_12_5: u16 = 129;
    pub const PEQ_11_1: u16 = 134;
    pub const PEQ_11_2: u16 = 139;
    pub const PEQ_11_3: u16 = 144;
    pub const PEQ_11_4: u16 = 149;
    pub const PEQ_11_5: u16 = 154;
    pub const MUTE_NO_SLEW_ALG_1_1MUTE: u16 = 159;
    pub const MUTE_NO_SLEW_ALG_1_2MUTE: u16 = 160;
    pub const MUTE_NO_SLEW_ALG_1_3MUTE: u16 = 161;
    pub const MUTE_NO_SLEW_ALG_1_4MUTE: u16 = 162;
    pub const MUTE_NO_SLEW_ALG_1_7MUTE: u16 = 163;
    pub const MUTE_NO_SLEW_ALG_1_8MUTE: u16 = 164;
    pub const MUTE_NO_SLEW_ALG_1_9MUTE: u16 = 165;
    pub const MUTE_NO_SLEW_ALG_2_0MUTE: u16 = 166;
    pub const MUTE_NO_SLEW_ALG_2_3MUTE: u16 = 167;
    pub const MUTE_NO_SLEW_ALG_2_4MUTE: u16 = 168;
    pub const MUTE_NO_SLEW_ALG_2_5MUTE: u16 = 169;
    pub const MUTE_NO_SLEW_ALG_2_6MUTE: u16 = 170;
    pub const MUTE_NO_SLEW_ALG_2_9MUTE: u16 = 171;
    pub const MUTE_NO_SLEW_ALG_3_0MUTE: u16 = 172;
    pub const MUTE_NO_SLEW_ALG_3_1MUTE: u16 = 173;
    pub const MUTE_NO_SLEW_ALG_3_2MUTE: u16 = 174;
    pub const MUTE_NO_SLEW_ALG_3_5MUTE: u16 = 175;
    pub const MUTE_NO_SLEW_ALG_3_6MUTE: u16 = 176;
    pub const MUTE_NO_SLEW_ALG_3_7MUTE: u16 = 177;
    pub const MUTE_NO_SLEW_ALG_3_8MUTE: u16 = 178;
    pub const MUTE_NO_SLEW_ALG_4_1MUTE: u16 = 179;
    pub const MUTE_NO_SLEW_ALG_4_2MUTE: u16 = 180;
    pub const MUTE_NO_SLEW_ALG_4_3MUTE: u16 = 181;
    pub const MUTE_NO_SLEW_ALG_4_4MUTE: u16 = 182;
    pub const MUTE_NO_SLEW_ALG_4_7MUTE: u16 = 183;
    pub const MUTE_NO_SLEW_ALG_4_8MUTE: u16 = 184;
    pub const MUTE_NO_SLEW_ALG_4_9MUTE: u16 = 185;
    pub const MUTE_NO_SLEW_ALG_5_0MUTE: u16 = 186;
    pub const MUTE_NO_SLEW_ALG_5_3MUTE: u16 = 187;
    pub const MUTE_NO_SLEW_ALG_5_4MUTE: u16 = 188;
    pub const MUTE_NO_SLEW_ALG_5_5MUTE: u16 = 189;
    pub const MUTE_NO_SLEW_ALG_5_6MUTE: u16 = 190;
    pub const MUTE_NO_SLEW_ALG_5_9MUTE: u16 = 191;
    pub const MUTE_NO_SLEW_ALG_6_0MUTE: u16 = 192;
    pub const MUTE_NO_SLEW_ALG_6_1MUTE: u16 = 193;
    pub const MUTE_NO_SLEW_ALG_6_2MUTE: u16 = 194;
    pub const MUTE_NO_SLEW_ALG_6_5MUTE: u16 = 195;
    pub const MUTE_NO_SLEW_ALG_6_6MUTE: u16 = 196;
    pub const MUTE_NO_SLEW_ALG_6_7MUTE: u16 = 197;
    pub const MUTE_NO_SLEW_ALG_6_8MUTE: u16 = 198;
    pub const SINGLE_CTRL_MIXER_NEW19401: u16 = 199;
    pub const SINGLE_CTRL_MIXER_NEW19402: u16 = 200;
    pub const SINGLE_CTRL_MIXER_NEW19404: u16 = 201;
    pub const SINGLE_CTRL_MIXER_NEW19403: u16 = 202;
    pub const SINGLE_CTRL_MIXER_NEW19405: u16 = 203;
    pub const SINGLE_CTRL_MIXER_NEW19406: u16 = 204;
    pub const SINGLE_CTRL_MIXER_NEW19407: u16 = 205;
    pub const SINGLE_CTRL_MIXER_NEW19408: u16 = 206;
    pub const SINGLE_CTRL_MIXER_NEW19409: u16 = 207;
    pub const SINGLE_CTRL_MIXER_NEW194010: u16 = 208;
    pub const BPF_3_1: u16 = 209;
    pub const BPF_3_5: u16 = 229;
    pub const BPF_5_1: u16 = 249;
    pub const BPF_5_5: u16 = 269;
    pub const BPF_6_1: u16 = 289;
    pub const BPF_6_5: u16 = 309;
    pub const BPF_7_1: u16 = 329;
    pub const BPF_7_5: u16 = 349;
    pub const BPF_8_1: u16 = 369;
    pub const BPF_8_5: u16 = 389;
    pub const BPF_9_1: u16 = 409;
    pub const BPF_9_5: u16 = 429;
    pub const BPF_10_1: u16 = 449;
    pub const BPF_10_5: u16 = 469;
    pub const BPF_4_1: u16 = 489;
    pub const BPF_4_5: u16 = 509;
    pub const BPF_2_1: u16 = 529;
    pub const BPF_2_5: u16 = 549;
    pub const BPF_1_1: u16 = 569;
    pub const BPF_1_5: u16 = 589;
    pub const PEQ_2_1: u16 = 609;
    pub const PEQ_2_2: u16 = 614;
    pub const PEQ_2_3: u16 = 619;
    pub const PEQ_2_4: u16 = 624;
    pub const PEQ_2_5: u16 = 629;
    pub const PEQ_3_1: u16 = 634;
    pub const PEQ_3_2: u16 = 639;
    pub const PEQ_3_3: u16 = 644;
    pub const PEQ_3_4: u16 = 649;
    pub const PEQ_3_5: u16 = 654;
    pub const PEQ_5_1: u16 = 659;
    pub const PEQ_5_2: u16 = 664;
    pub const PEQ_5_3: u16 = 669;
    pub const PEQ_5_4: u16 = 674;
    pub const PEQ_5_5: u16 = 679;
    pub const PEQ_6_1: u16 = 684;
    pub const PEQ_6_2: u16 = 689;
    pub const PEQ_6_3: u16 = 694;
    pub const PEQ_6_4: u16 = 699;
    pub const PEQ_6_5: u16 = 704;
    pub const PEQ_7_1: u16 = 709;
    pub const PEQ_7_2: u16 = 714;
    pub const PEQ_7_3: u16 = 719;
    pub const PEQ_7_4: u16 = 724;
    pub const PEQ_7_5: u16 = 729;
    pub const PEQ_8_1: u16 = 734;
    pub const PEQ_8_2: u16 = 739;
    pub const PEQ_8_3: u16 = 744;
    pub const PEQ_8_4: u16 = 749;
    pub const PEQ_8_5: u16 = 754;
    pub const PEQ_9_1: u16 = 759;
    pub const PEQ_9_2: u16 = 764;
    pub const PEQ_9_3: u16 = 769;
    pub const PEQ_9_4: u16 = 774;
    pub const PEQ_9_5: u16 = 779;
    pub const PEQ_10_1: u16 = 784;
    pub const PEQ_10_2: u16 = 789;
    pub const PEQ_10_3: u16 = 794;
    pub const PEQ_10_4: u16 = 799;
    pub const PEQ_10_5: u16 = 804;
    pub const PEQ_1_1: u16 = 809;
    pub const PEQ_1_2: u16 = 814;
    pub const PEQ_1_3: u16 = 819;
    pub const PEQ_1_4: u16 = 824;
    pub const PEQ_1_5: u16 = 829;
    pub const PEQ_4_1: u16 = 834;
    pub const PEQ_4_2: u16 = 839;
    pub const PEQ_4_3: u16 = 844;
    pub const PEQ_4_4: u16 = 849;
    pub const PEQ_4_5: u16 = 854;
    pub const MULT_CTRL_DEL_GROW_ALG_1: u16 = 859;
    pub const MULT_CTRL_DEL_GROW_ALG_3: u16 = 860;
    pub const MULT_CTRL_DEL_GROW_ALG_4: u16 = 861;
    pub const MULT_CTRL_DEL_GROW_ALG_5: u16 = 862;
    pub const MULT_CTRL_DEL_GROW_ALG_6: u16 = 863;
    pub const MULT_CTRL_DEL_GROW_ALG_7: u16 = 864;
    pub const MULT_CTRL_DEL_GROW_ALG_8: u16 = 865;
    pub const MULT_CTRL_DEL_GROW_ALG_2: u16 = 866;
    pub const EQ1940_INVERT_9GAIN: u16 = 867;
    pub const EQ1940_INVERT_1_0GAIN: u16 = 868;
    pub const EQ1940_INVERT_1GAIN: u16 = 869;
    pub const MUTE_NO_SLEW_ALG_9MUTE: u16 = 870;
    pub const MUTE_NO_SLEW_ALG_1_0MUTE: u16 = 871;
    pub const EQ1940_INVERT_8GAIN: u16 = 872;
    pub const EQ1940_INVERT_7GAIN: u16 = 873;
    pub const EQ1940_INVERT_6GAIN: u16 = 874;
    pub const EQ1940_INVERT_5GAIN: u16 = 875;
    pub const EQ1940_INVERT_4GAIN: u16 = 876;
    pub const EQ1940_INVERT_2GAIN: u16 = 877;
    pub const EQ1940_INVERT_3GAIN: u16 = 878;
    pub const MUTE_NO_SLEW_ALG_1MUTE: u16 = 879;
    pub const MUTE_NO_SLEW_ALG_2MUTE: u16 = 880;
    pub const MUTE_NO_SLEW_ALG_3MUTE: u16 = 881;
    pub const MUTE_NO_SLEW_ALG_5MUTE: u16 = 882;
    pub const MUTE_NO_SLEW_ALG_6MUTE: u16 = 883;
    pub const MUTE_NO_SLEW_ALG_7MUTE: u16 = 884;
    pub const MUTE_NO_SLEW_ALG_8MUTE: u16 = 885;
    pub const GAIN_1940_ALG_NS9: u16 = 886;
    pub const GAIN_1940_ALG_NS10: u16 = 887;
    pub const MUTE_NO_SLEW_ALG_4MUTE: u16 = 888;
    pub const GAIN_1940_ALG_NS4: u16 = 889;
    pub const GAIN_1940_ALG_NS5: u16 = 890;
    pub const GAIN_1940_ALG_NS6: u16 = 891;
    pub const GAIN_1940_ALG_NS7: u16 = 892;
    pub const GAIN_1940_ALG_NS8: u16 = 893;
    pub const GAIN_1940_ALG_NS3: u16 = 894;
    pub const GAIN_1940_ALG_NS2: u16 = 895;
    pub const GAIN_1940_ALG_NS1: u16 = 896;
    pub const EXT_SW_GAIN_DB_1STEP: u16 = 897;
    pub const MONOMUX_194_0NS_1_0: u16 = 898;
    pub const MONOMUX_194_0NS_1_1: u16 = 899;
    pub const MONOMUX_194_0NS_1_2: u16 = 900;
    pub const MONOMUX_194_0NS_1_3: u16 = 901;
    pub const MONOMUX_194_0NS_1_4: u16 = 902;
    pub const MONOMUX_194_0NS_1_5: u16 = 903;
    pub const MONOMUX_194_0NS_1_6: u16 = 904;
    pub const MONOMUX_194_0NS_1_7: u16 = 905;
    pub const MONOMUX_194_0NS_1_8: u16 = 906;
    pub const MONOMUX_194_0NS_1_9: u16 = 907;
    pub const MONOMUX_194_0NS_1_10: u16 = 908;
    pub const MONOMUX_194_0NS_1_11: u16 = 909;
    pub const MONOMUX_194_0NS_1_12: u16 = 910;
    pub const MONOMUX_194_0NS_1_13: u16 = 911;
    pub const MONO_ENVELOPE_PEAK_ALG_1HOLD: u16 = 912;
    pub const MONO_ENVELOPE_PEAK_ALG_1DECAY: u16 = 913;
    pub const READ_BACK_ALG_SIGMA_2001: u16 = 914;
    #[cfg(feature = "symbols")]
    pub const SYMBOLS: &[(&str, u16)] = &[
        ("NON_MOD_RAM_ALLOC", NON_MOD_RAM_ALLOC),
        ("DC_INP_ALG_1", DC_INP_ALG_1),
        ("DC_INP_ALG_2", DC_INP_ALG_2),
        ("DC_INP_ALG_3", DC_INP_ALG_3),
        ("GAIN_1940_ALG_NS11", GAIN_1940_ALG_NS11),
        ("GAIN_1940_ALG_NS12", GAIN_1940_ALG_NS12),
        ("GAIN_1940_ALG_NS13", GAIN_1940_ALG_NS13),
        ("GAIN_1940_ALG_NS14", GAIN_1940_ALG_NS14),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_0",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_0,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_1",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_1,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_2",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_2,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_3",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_3,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_4",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_4,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_5",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_5,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_6",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_6,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_7",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_7,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_8",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_8,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_9",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_9,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_10",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_10,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_11",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_11,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_12",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_12,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_13",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_13,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_14",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_14,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_15",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_15,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_16",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_16,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_17",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_17,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_18",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_18,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_19",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_19,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_20",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_20,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_21",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_21,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_22",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_22,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_23",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_23,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_24",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_24,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_25",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_25,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_26",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_26,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_27",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_27,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_28",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_28,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_29",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_29,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_30",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_30,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_31",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_31,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_32",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_32,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_33",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1_33,
        ),
        (
            "MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1RMS",
            MONO_NO_POST_GAIN_NO_HOLD_NO_DELAY_1RMS,
        ),
        ("MONOMUX_194_0NS_3_0", MONOMUX_194_0NS_3_0),
        ("MONOMUX_194_0NS_3_1", MONOMUX_194_0NS_3_1),
        ("MONOMUX_194_0NS_3_2", MONOMUX_194_0NS_3_2),
        ("MUTE_NO_SLEW_ALG_7_1MUTE", MUTE_NO_SLEW_ALG_7_1MUTE),
        ("MUTE_NO_SLEW_ALG_7_2MUTE", MUTE_NO_SLEW_ALG_7_2MUTE),
        ("MUTE_NO_SLEW_ALG_7_3MUTE", MUTE_NO_SLEW_ALG_7_3MUTE),
        ("MUTE_NO_SLEW_ALG_7_4MUTE", MUTE_NO_SLEW_ALG_7_4MUTE),
        ("MONOMUX_194_0NS_2_0", MONOMUX_194_0NS_2_0),
        ("MONOMUX_194_0NS_2_1", MONOMUX_194_0NS_2_1),
        ("PEQ_13_1", PEQ_13_1),
        ("PEQ_13_2", PEQ_13_2),
        ("PEQ_13_3", PEQ_13_3),
        ("PEQ_13_4", PEQ_13_4),
        ("PEQ_13_5", PEQ_13_5),
        ("PEQ_14_1", PEQ_14_1),
        ("PEQ_14_2", PEQ_14_2),
        ("PEQ_14_3", PEQ_14_3),
        ("PEQ_14_4", PEQ_14_4),
        ("PEQ_14_5", PEQ_14_5),
        ("PEQ_12_1", PEQ_12_1),
        ("PEQ_12_2", PEQ_12_2),
        ("PEQ_12_3", PEQ_12_3),
        ("PEQ_12_4", PEQ_12_4),
        ("PEQ_12_5", PEQ_12_5),
        ("PEQ_11_1", PEQ_11_1),
        ("PEQ_11_2", PEQ_11_2),
        ("PEQ_11_3", PEQ_11_3),
        ("PEQ_11_4", PEQ_11_4),
        ("PEQ_11_5", PEQ_11_5),
        ("MUTE_NO_SLEW_ALG_1_1MUTE", MUTE_NO_SLEW_ALG_1_1MUTE),
        ("MUTE_NO_SLEW_ALG_1_2MUTE", MUTE_NO_SLEW_ALG_1_2MUTE),
        ("MUTE_NO_SLEW_ALG_1_3MUTE", MUTE_NO_SLEW_ALG_1_3MUTE),
        ("MUTE_NO_SLEW_ALG_1_4MUTE", MUTE_NO_SLEW_ALG_1_4MUTE),
        ("MUTE_NO_SLEW_ALG_1_7MUTE", MUTE_NO_SLEW_ALG_1_7MUTE),
        ("MUTE_NO_SLEW_ALG_1_8MUTE", MUTE_NO_SLEW_ALG_1_8MUTE),
        ("MUTE_NO_SLEW_ALG_1_9MUTE", MUTE_NO_SLEW_ALG_1_9MUTE),
        ("MUTE_NO_SLEW_ALG_2_0MUTE", MUTE_NO_SLEW_ALG_2_0MUTE),
        ("MUTE_NO_SLEW_ALG_2_3MUTE", MUTE_NO_SLEW_ALG_2_3MUTE),
        ("MUTE_NO_SLEW_ALG_2_4MUTE", MUTE_NO_SLEW_ALG_2_4MUTE),
        ("MUTE_NO_SLEW_ALG_2_5MUTE", MUTE_NO_SLEW_ALG_2_5MUTE),
        ("MUTE_NO_SLEW_ALG_2_6MUTE", MUTE_NO_SLEW_ALG_2_6MUTE),
        ("MUTE_NO_SLEW_ALG_2_9MUTE", MUTE_NO_SLEW_ALG_2_9MUTE),
        ("MUTE_NO_SLEW_ALG_3_0MUTE", MUTE_NO_SLEW_ALG_3_0MUTE),
        ("MUTE_NO_SLEW_ALG_3_1MUTE", MUTE_NO_SLEW_ALG_3_1MUTE),
        ("MUTE_NO_SLEW_ALG_3_2MUTE", MUTE_NO_SLEW_ALG_3_2MUTE),
        ("MUTE_NO_SLEW_ALG_3_5MUTE", MUTE_NO_SLEW_ALG_3_5MUTE),
        ("MUTE_NO_SLEW_ALG_3_6MUTE", MUTE_NO_SLEW_ALG_3_6MUTE),
        ("MUTE_NO_SLEW_ALG_3_7MUTE", MUTE_NO_SLEW_ALG_3_7MUTE),
        ("MUTE_NO_SLEW_ALG_3_8MUTE", MUTE_NO_SLEW_ALG_3_8MUTE),
        ("MUTE_NO_SLEW_ALG_4_1MUTE", MUTE_NO_SLEW_ALG_4_1MUTE),
        ("MUTE_NO_SLEW_ALG_4_2MUTE", MUTE_NO_SLEW_ALG_4_2MUTE),
        ("MUTE_NO_SLEW_ALG_4_3MUTE", MUTE_NO_SLEW_ALG_4_3MUTE),
        ("MUTE_NO_SLEW_ALG_4_4MUTE", MUTE_NO_SLEW_ALG_4_4MUTE),
        ("MUTE_NO_SLEW_ALG_4_7MUTE", MUTE_NO_SLEW_ALG_4_7MUTE),
        ("MUTE_NO_SLEW_ALG_4_8MUTE", MUTE_NO_SLEW_ALG_4_8MUTE),
        ("MUTE_NO_SLEW_ALG_4_9MUTE", MUTE_NO_SLEW_ALG_4_9MUTE),
        ("MUTE_NO_SLEW_ALG_5_0MUTE", MUTE_NO_SLEW_ALG_5_0MUTE),
        ("MUTE_NO_SLEW_ALG_5_3MUTE", MUTE_NO_SLEW_ALG_5_3MUTE),
        ("MUTE_NO_SLEW_ALG_5_4MUTE", MUTE_NO_SLEW_ALG_5_4MUTE),
        ("MUTE_NO_SLEW_ALG_5_5MUTE", MUTE_NO_SLEW_ALG_5_5MUTE),
        ("MUTE_NO_SLEW_ALG_5_6MUTE", MUTE_NO_SLEW_ALG_5_6MUTE),
        ("MUTE_NO_SLEW_ALG_5_9MUTE", MUTE_NO_SLEW_ALG_5_9MUTE),
        ("MUTE_NO_SLEW_ALG_6_0MUTE", MUTE_NO_SLEW_ALG_6_0MUTE),
        ("MUTE_NO_SLEW_ALG_6_1MUTE", MUTE_NO_SLEW_ALG_6_1MUTE),
        ("MUTE_NO_SLEW_ALG_6_2MUTE", MUTE_NO_SLEW_ALG_6_2MUTE),
        ("MUTE_NO_SLEW_ALG_6_5MUTE", MUTE_NO_SLEW_ALG_6_5MUTE),
        ("MUTE_NO_SLEW_ALG_6_6MUTE", MUTE_NO_SLEW_ALG_6_6MUTE),
        ("MUTE_NO_SLEW_ALG_6_7MUTE", MUTE_NO_SLEW_ALG_6_7MUTE),
        ("MUTE_NO_SLEW_ALG_6_8MUTE", MUTE_NO_SLEW_ALG_6_8MUTE),
        ("SINGLE_CTRL_MIXER_NEW19401", SINGLE_CTRL_MIXER_NEW19401),
        ("SINGLE_CTRL_MIXER_NEW19402", SINGLE_CTRL_MIXER_NEW19402),
        ("SINGLE_CTRL_MIXER_NEW19404", SINGLE_CTRL_MIXER_NEW19404),
        ("SINGLE_CTRL_MIXER_NEW19403", SINGLE_CTRL_MIXER_NEW19403),
        ("SINGLE_CTRL_MIXER_NEW19405", SINGLE_CTRL_MIXER_NEW19405),
        ("SINGLE_CTRL_MIXER_NEW19406", SINGLE_CTRL_MIXER_NEW19406),
        ("SINGLE_CTRL_MIXER_NEW19407", SINGLE_CTRL_MIXER_NEW19407),
        ("SINGLE_CTRL_MIXER_NEW19408", SINGLE_CTRL_MIXER_NEW19408),
        ("SINGLE_CTRL_MIXER_NEW19409", SINGLE_CTRL_MIXER_NEW19409),
        ("SINGLE_CTRL_MIXER_NEW194010", SINGLE_CTRL_MIXER_NEW194010),
        ("BPF_3_1", BPF_3_1),
        ("BPF_3_5", BPF_3_5),
        ("BPF_5_1", BPF_5_1),
        ("BPF_5_5", BPF_5_5),
        ("BPF_6_1", BPF_6_1),
        ("BPF_6_5", BPF_6_5),
        ("BPF_7_1", BPF_7_1),
        ("BPF_7_5", BPF_7_5),
        ("BPF_8_1", BPF_8_1),
        ("BPF_8_5", BPF_8_5),
        ("BPF_9_1", BPF_9_1),
        ("BPF_9_5", BPF_9_5),
        ("BPF_10_1", BPF_10_1),
        ("BPF_10_5", BPF_10_5),
        ("BPF_4_1", BPF_4_1),
        ("BPF_4_5", BPF_4_5),
        ("BPF_2_1", BPF_2_1),
        ("BPF_2_5", BPF_2_5),
        ("BPF_1_1", BPF_1_1),
        ("BPF_1_5", BPF_1_5),
        ("PEQ_2_1", PEQ_2_1),
        ("PEQ_2_2", PEQ_2_2),
        ("PEQ_2_3", PEQ_2_3),
        ("PEQ_2_4", PEQ_2_4),
        ("PEQ_2_5", PEQ_2_5),
        ("PEQ_3_1", PEQ_3_1),
        ("PEQ_3_2", PEQ_3_2),
        ("PEQ_3_3", PEQ_3_3),
        ("PEQ_3_4", PEQ_3_4),
        ("PEQ_3_5", PEQ_3_5),
        ("PEQ_5_1", PEQ_5_1),
        ("PEQ_5_2", PEQ_5_2),
        ("PEQ_5_3", PEQ_5_3),
        ("PEQ_5_4", PEQ_5_4),
        ("PEQ_5_5", PEQ_5_5),
        ("PEQ_6_1", PEQ_6_1),
        ("PEQ_6_2", PEQ_6_2),
        ("PEQ_6_3", PEQ_6_3),
        ("PEQ_6_4", PEQ_6_4),
        ("PEQ_6_5", PEQ_6_5),
        ("PEQ_7_1", PEQ_7_1),
        ("PEQ_7_2", PEQ_7_2),
        ("PEQ_7_3", PEQ_7_3),
        ("PEQ_7_4", PEQ_7_4),
        ("PEQ_7_5", PEQ_7_5),
        ("PEQ_8_1", PEQ_8_1),
        ("PEQ_8_2", PEQ_8_2),
        ("PEQ_8_3", PEQ_8_3),
        ("PEQ_8_4", PEQ_8_4),
        ("PEQ_8_5", PEQ_8_5),
        ("PEQ_9_1", PEQ_9_1),
        ("PEQ_9_2", PEQ_9_2),
        ("PEQ_9_3", PEQ_9_3),
        ("PEQ_9_4", PEQ_9_4),
        ("PEQ_9_5", PEQ_9_5),
        ("PEQ_10_1", PEQ_10_1),
        ("PEQ_10_2", PEQ_10_2),
        ("PEQ_10_3", PEQ_10_3),
        ("PEQ_10_4", PEQ_10_4),
        ("PEQ_10_5", PEQ_10_5),
        ("PEQ_1_1", PEQ_1_1),
        ("PEQ_1_2", PEQ_1_2),
        ("PEQ_1_3", PEQ_1_3),
        ("PEQ_1_4", PEQ_1_4),
        ("PEQ_1_5", PEQ_1_5),
        ("PEQ_4_1", PEQ_4_1),
        ("PEQ_4_2", PEQ_4_2),
        ("PEQ_4_3", PEQ_4_3),
        ("PEQ_4_4", PEQ_4_4),
        ("PEQ_4_5", PEQ_4_5),
        ("MULT_CTRL_DEL_GROW_ALG_1", MULT_CTRL_DEL_GROW_ALG_1),
        ("MULT_CTRL_DEL_GROW_ALG_3", MULT_CTRL_DEL_GROW_ALG_3),
        ("MULT_CTRL_DEL_GROW_ALG_4", MULT_CTRL_DEL_GROW_ALG_4),
        ("MULT_CTRL_DEL_GROW_ALG_5", MULT_CTRL_DEL_GROW_ALG_5),
        ("MULT_CTRL_DEL_GROW_ALG_6", MULT_CTRL_DEL_GROW_ALG_6),
        ("MULT_CTRL_DEL_GROW_ALG_7", MULT_CTRL_DEL_GROW_ALG_7),
        ("MULT_CTRL_DEL_GROW_ALG_8", MULT_CTRL_DEL_GROW_ALG_8),
        ("MULT_CTRL_DEL_GROW_ALG_2", MULT_CTRL_DEL_GROW_ALG_2),
        ("EQ1940_INVERT_9GAIN", EQ1940_INVERT_9GAIN),
        ("EQ1940_INVERT_1_0GAIN", EQ1940_INVERT_1_0GAIN),
        ("EQ1940_INVERT_1GAIN", EQ1940_INVERT_1GAIN),
        ("MUTE_NO_SLEW_ALG_9MUTE", MUTE_NO_SLEW_ALG_9MUTE),
        ("MUTE_NO_SLEW_ALG_1_0MUTE", MUTE_NO_SLEW_ALG_1_0MUTE),
        ("EQ1940_INVERT_8GAIN", EQ1940_INVERT_8GAIN),
        ("EQ1940_INVERT_7GAIN", EQ1940_INVERT_7GAIN),
        ("EQ1940_INVERT_6GAIN", EQ1940_INVERT_6GAIN),
        ("EQ1940_INVERT_5GAIN", EQ1940_INVERT_5GAIN),
        ("EQ1940_INVERT_4GAIN", EQ1940_INVERT_4GAIN),
        ("EQ1940_INVERT_2GAIN", EQ1940_INVERT_2GAIN),
        ("EQ1940_INVERT_3GAIN", EQ1940_INVERT_3GAIN),
        ("MUTE_NO_SLEW_ALG_1MUTE", MUTE_NO_SLEW_ALG_1MUTE),
        ("MUTE_NO_SLEW_ALG_2MUTE", MUTE_NO_SLEW_ALG_2MUTE),
        ("MUTE_NO_SLEW_ALG_3MUTE", MUTE_NO_SLEW_ALG_3MUTE),
        ("MUTE_NO_SLEW_ALG_5MUTE", MUTE_NO_SLEW_ALG_5MUTE),
        ("MUTE_NO_SLEW_ALG_6MUTE", MUTE_NO_SLEW_ALG_6MUTE),
        ("MUTE_NO_SLEW_ALG_7MUTE", MUTE_NO_SLEW_ALG_7MUTE),
        ("MUTE_NO_SLEW_ALG_8MUTE", MUTE_NO_SLEW_ALG_8MUTE),
        ("GAIN_1940_ALG_NS9", GAIN_1940_ALG_NS9),
        ("GAIN_1940_ALG_NS10", GAIN_1940_ALG_NS10),
        ("MUTE_NO_SLEW_ALG_4MUTE", MUTE_NO_SLEW_ALG_4MUTE),
        ("GAIN_1940_ALG_NS4", GAIN_1940_ALG_NS4),
        ("GAIN_1940_ALG_NS5", GAIN_1940_ALG_NS5),
        ("GAIN_1940_ALG_NS6", GAIN_1940_ALG_NS6),
        ("GAIN_1940_ALG_NS7", GAIN_1940_ALG_NS7),
        ("GAIN_1940_ALG_NS8", GAIN_1940_ALG_NS8),
        ("GAIN_1940_ALG_NS3", GAIN_1940_ALG_NS3),
        ("GAIN_1940_ALG_NS2", GAIN_1940_ALG_NS2),
        ("GAIN_1940_ALG_NS1", GAIN_1940_ALG_NS1),
        ("EXT_SW_GAIN_DB_1STEP", EXT_SW_GAIN_DB_1STEP),
        ("MONOMUX_194_0NS_1_0", MONOMUX_194_0NS_1_0),
        ("MONOMUX_194_0NS_1_1", MONOMUX_194_0NS_1_1),
        ("MONOMUX_194_0NS_1_2", MONOMUX_194_0NS_1_2),
        ("MONOMUX_194_0NS_1_3", MONOMUX_194_0NS_1_3),
        ("MONOMUX_194_0NS_1_4", MONOMUX_194_0NS_1_4),
        ("MONOMUX_194_0NS_1_5", MONOMUX_194_0NS_1_5),
        ("MONOMUX_194_0NS_1_6", MONOMUX_194_0NS_1_6),
        ("MONOMUX_194_0NS_1_7", MONOMUX_194_0NS_1_7),
        ("MONOMUX_194_0NS_1_8", MONOMUX_194_0NS_1_8),
        ("MONOMUX_194_0NS_1_9", MONOMUX_194_0NS_1_9),
        ("MONOMUX_194_0NS_1_10", MONOMUX_194_0NS_1_10),
        ("MONOMUX_194_0NS_1_11", MONOMUX_194_0NS_1_11),
        ("MONOMUX_194_0NS_1_12", MONOMUX_194_0NS_1_12),
        ("MONOMUX_194_0NS_1_13", MONOMUX_194_0NS_1_13),
        ("MONO_ENVELOPE_PEAK_ALG_1HOLD", MONO_ENVELOPE_PEAK_ALG_1HOLD),
        (
            "MONO_ENVELOPE_PEAK_ALG_1DECAY",
            MONO_ENVELOPE_PEAK_ALG_1DECAY,
        ),
        ("READ_BACK_ALG_SIGMA_2001", READ_BACK_ALG_SIGMA_2001),
    ];
}
#[allow(unused_imports)]
use sym::*;
pub const DEVICE: Device = Device {
    product_name: "MiniDSP 4x10HD",
    sources: &[Spdif, Toslink, Aesebu],
    inputs: &[
        Input {
            gate: Some(Gate {
                enable: MUTE_NO_SLEW_ALG_7_1MUTE,
                gain: GAIN_1940_ALG_NS11,
            }),
            meter: None,
            routing: &[],
            peq: &[PEQ_11_5, PEQ_11_4, PEQ_11_3, PEQ_11_2, PEQ_11_1],
        },
        Input {
            gate: Some(Gate {
                enable: MUTE_NO_SLEW_ALG_7_2MUTE,
                gain: GAIN_1940_ALG_NS12,
            }),
            meter: None,
            routing: &[],
            peq: &[PEQ_12_5, PEQ_12_4, PEQ_12_3, PEQ_12_2, PEQ_12_1],
        },
        Input {
            gate: Some(Gate {
                enable: MUTE_NO_SLEW_ALG_7_3MUTE,
                gain: GAIN_1940_ALG_NS13,
            }),
            meter: None,
            routing: &[],
            peq: &[PEQ_13_5, PEQ_13_4, PEQ_13_3, PEQ_13_2, PEQ_13_1],
        },
        Input {
            gate: Some(Gate {
                enable: MUTE_NO_SLEW_ALG_7_4MUTE,
                gain: GAIN_1940_ALG_NS14,
            }),
            meter: None,
            routing: &[],
            peq: &[PEQ_14_5, PEQ_14_4, PEQ_14_3, PEQ_14_2, PEQ_14_1],
        },
    ],
    outputs: &[
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_1MUTE,
                gain: GAIN_1940_ALG_NS1,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_1,
            invert_addr: EQ1940_INVERT_1GAIN,
            peq: &[PEQ_1_5, PEQ_1_4, PEQ_1_3, PEQ_1_2, PEQ_1_1],
            xover: Some(Crossover {
                peqs: &[BPF_1_1, BPF_1_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_2MUTE,
                gain: GAIN_1940_ALG_NS2,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_2,
            invert_addr: EQ1940_INVERT_2GAIN,
            peq: &[PEQ_2_5, PEQ_2_4, PEQ_2_3, PEQ_2_2, PEQ_2_1],
            xover: Some(Crossover {
                peqs: &[BPF_2_1, BPF_2_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_3MUTE,
                gain: GAIN_1940_ALG_NS3,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_3,
            invert_addr: EQ1940_INVERT_3GAIN,
            peq: &[PEQ_3_5, PEQ_3_4, PEQ_3_3, PEQ_3_2, PEQ_3_1],
            xover: Some(Crossover {
                peqs: &[BPF_3_1, BPF_3_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_4MUTE,
                gain: GAIN_1940_ALG_NS4,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_4,
            invert_addr: EQ1940_INVERT_4GAIN,
            peq: &[PEQ_4_5, PEQ_4_4, PEQ_4_3, PEQ_4_2, PEQ_4_1],
            xover: Some(Crossover {
                peqs: &[BPF_4_1, BPF_4_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_5MUTE,
                gain: GAIN_1940_ALG_NS5,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_5,
            invert_addr: EQ1940_INVERT_5GAIN,
            peq: &[PEQ_5_5, PEQ_5_4, PEQ_5_3, PEQ_5_2, PEQ_5_1],
            xover: Some(Crossover {
                peqs: &[BPF_5_1, BPF_5_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_6MUTE,
                gain: GAIN_1940_ALG_NS6,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_6,
            invert_addr: EQ1940_INVERT_6GAIN,
            peq: &[PEQ_6_5, PEQ_6_4, PEQ_6_3, PEQ_6_2, PEQ_6_1],
            xover: Some(Crossover {
                peqs: &[BPF_6_1, BPF_6_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_7MUTE,
                gain: GAIN_1940_ALG_NS7,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_7,
            invert_addr: EQ1940_INVERT_7GAIN,
            peq: &[PEQ_7_5, PEQ_7_4, PEQ_7_3, PEQ_7_2, PEQ_7_1],
            xover: Some(Crossover {
                peqs: &[BPF_7_1, BPF_7_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_8MUTE,
                gain: GAIN_1940_ALG_NS8,
            },
            meter: 0,
            delay_addr: MULT_CTRL_DEL_GROW_ALG_8,
            invert_addr: EQ1940_INVERT_8GAIN,
            peq: &[PEQ_8_5, PEQ_8_4, PEQ_8_3, PEQ_8_2, PEQ_8_1],
            xover: Some(Crossover {
                peqs: &[BPF_8_1, BPF_8_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_9MUTE,
                gain: GAIN_1940_ALG_NS9,
            },
            meter: 0,
            delay_addr: 0,
            invert_addr: EQ1940_INVERT_9GAIN,
            peq: &[PEQ_9_5, PEQ_9_4, PEQ_9_3, PEQ_9_2, PEQ_9_1],
            xover: Some(Crossover {
                peqs: &[BPF_9_1, BPF_9_5],
            }),
            compressor: None,
            fir: None,
        },
        Output {
            gate: Gate {
                enable: MUTE_NO_SLEW_ALG_1_0MUTE,
                gain: GAIN_1940_ALG_NS10,
            },
            meter: 0,
            delay_addr: 0,
            invert_addr: EQ1940_INVERT_1_0GAIN,
            peq: &[PEQ_10_5, PEQ_10_4, PEQ_10_3, PEQ_10_2, PEQ_10_1],
            xover: Some(Crossover {
                peqs: &[BPF_10_1, BPF_10_5],
            }),
            compressor: None,
            fir: None,
        },
    ],
    fir_max_taps: 0,
    internal_sampling_rate: 48000,
    dialect: Dialect {
        addr_encoding: AddrEncoding::AddrLen2,
        float_encoding: FloatEncoding::FixedPoint,
    },
    #[cfg(feature = "symbols")]
    symbols: SYMBOLS,
};
