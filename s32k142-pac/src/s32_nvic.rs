#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Set Enable Register n"]
    pub nviciser0: crate::Reg<nviciser0::NVICISER0_SPEC>,
    #[doc = "0x04 - Interrupt Set Enable Register n"]
    pub nviciser1: crate::Reg<nviciser1::NVICISER1_SPEC>,
    #[doc = "0x08 - Interrupt Set Enable Register n"]
    pub nviciser2: crate::Reg<nviciser2::NVICISER2_SPEC>,
    #[doc = "0x0c - Interrupt Set Enable Register n"]
    pub nviciser3: crate::Reg<nviciser3::NVICISER3_SPEC>,
    #[doc = "0x10 - Interrupt Set Enable Register n"]
    pub nviciser4: crate::Reg<nviciser4::NVICISER4_SPEC>,
    #[doc = "0x14 - Interrupt Set Enable Register n"]
    pub nviciser5: crate::Reg<nviciser5::NVICISER5_SPEC>,
    #[doc = "0x18 - Interrupt Set Enable Register n"]
    pub nviciser6: crate::Reg<nviciser6::NVICISER6_SPEC>,
    #[doc = "0x1c - Interrupt Set Enable Register n"]
    pub nviciser7: crate::Reg<nviciser7::NVICISER7_SPEC>,
    _reserved8: [u8; 0x60],
    #[doc = "0x80 - Interrupt Clear Enable Register n"]
    pub nvicicer0: crate::Reg<nvicicer0::NVICICER0_SPEC>,
    #[doc = "0x84 - Interrupt Clear Enable Register n"]
    pub nvicicer1: crate::Reg<nvicicer1::NVICICER1_SPEC>,
    #[doc = "0x88 - Interrupt Clear Enable Register n"]
    pub nvicicer2: crate::Reg<nvicicer2::NVICICER2_SPEC>,
    #[doc = "0x8c - Interrupt Clear Enable Register n"]
    pub nvicicer3: crate::Reg<nvicicer3::NVICICER3_SPEC>,
    #[doc = "0x90 - Interrupt Clear Enable Register n"]
    pub nvicicer4: crate::Reg<nvicicer4::NVICICER4_SPEC>,
    #[doc = "0x94 - Interrupt Clear Enable Register n"]
    pub nvicicer5: crate::Reg<nvicicer5::NVICICER5_SPEC>,
    #[doc = "0x98 - Interrupt Clear Enable Register n"]
    pub nvicicer6: crate::Reg<nvicicer6::NVICICER6_SPEC>,
    #[doc = "0x9c - Interrupt Clear Enable Register n"]
    pub nvicicer7: crate::Reg<nvicicer7::NVICICER7_SPEC>,
    _reserved16: [u8; 0x60],
    #[doc = "0x100 - Interrupt Set Pending Register n"]
    pub nvicispr0: crate::Reg<nvicispr0::NVICISPR0_SPEC>,
    #[doc = "0x104 - Interrupt Set Pending Register n"]
    pub nvicispr1: crate::Reg<nvicispr1::NVICISPR1_SPEC>,
    #[doc = "0x108 - Interrupt Set Pending Register n"]
    pub nvicispr2: crate::Reg<nvicispr2::NVICISPR2_SPEC>,
    #[doc = "0x10c - Interrupt Set Pending Register n"]
    pub nvicispr3: crate::Reg<nvicispr3::NVICISPR3_SPEC>,
    #[doc = "0x110 - Interrupt Set Pending Register n"]
    pub nvicispr4: crate::Reg<nvicispr4::NVICISPR4_SPEC>,
    #[doc = "0x114 - Interrupt Set Pending Register n"]
    pub nvicispr5: crate::Reg<nvicispr5::NVICISPR5_SPEC>,
    #[doc = "0x118 - Interrupt Set Pending Register n"]
    pub nvicispr6: crate::Reg<nvicispr6::NVICISPR6_SPEC>,
    #[doc = "0x11c - Interrupt Set Pending Register n"]
    pub nvicispr7: crate::Reg<nvicispr7::NVICISPR7_SPEC>,
    _reserved24: [u8; 0x60],
    #[doc = "0x180 - Interrupt Clear Pending Register n"]
    pub nvicicpr0: crate::Reg<nvicicpr0::NVICICPR0_SPEC>,
    #[doc = "0x184 - Interrupt Clear Pending Register n"]
    pub nvicicpr1: crate::Reg<nvicicpr1::NVICICPR1_SPEC>,
    #[doc = "0x188 - Interrupt Clear Pending Register n"]
    pub nvicicpr2: crate::Reg<nvicicpr2::NVICICPR2_SPEC>,
    #[doc = "0x18c - Interrupt Clear Pending Register n"]
    pub nvicicpr3: crate::Reg<nvicicpr3::NVICICPR3_SPEC>,
    #[doc = "0x190 - Interrupt Clear Pending Register n"]
    pub nvicicpr4: crate::Reg<nvicicpr4::NVICICPR4_SPEC>,
    #[doc = "0x194 - Interrupt Clear Pending Register n"]
    pub nvicicpr5: crate::Reg<nvicicpr5::NVICICPR5_SPEC>,
    #[doc = "0x198 - Interrupt Clear Pending Register n"]
    pub nvicicpr6: crate::Reg<nvicicpr6::NVICICPR6_SPEC>,
    #[doc = "0x19c - Interrupt Clear Pending Register n"]
    pub nvicicpr7: crate::Reg<nvicicpr7::NVICICPR7_SPEC>,
    _reserved32: [u8; 0x60],
    #[doc = "0x200 - Interrupt Active bit Register n"]
    pub nviciabr0: crate::Reg<nviciabr0::NVICIABR0_SPEC>,
    #[doc = "0x204 - Interrupt Active bit Register n"]
    pub nviciabr1: crate::Reg<nviciabr1::NVICIABR1_SPEC>,
    #[doc = "0x208 - Interrupt Active bit Register n"]
    pub nviciabr2: crate::Reg<nviciabr2::NVICIABR2_SPEC>,
    #[doc = "0x20c - Interrupt Active bit Register n"]
    pub nviciabr3: crate::Reg<nviciabr3::NVICIABR3_SPEC>,
    #[doc = "0x210 - Interrupt Active bit Register n"]
    pub nviciabr4: crate::Reg<nviciabr4::NVICIABR4_SPEC>,
    #[doc = "0x214 - Interrupt Active bit Register n"]
    pub nviciabr5: crate::Reg<nviciabr5::NVICIABR5_SPEC>,
    #[doc = "0x218 - Interrupt Active bit Register n"]
    pub nviciabr6: crate::Reg<nviciabr6::NVICIABR6_SPEC>,
    #[doc = "0x21c - Interrupt Active bit Register n"]
    pub nviciabr7: crate::Reg<nviciabr7::NVICIABR7_SPEC>,
    _reserved40: [u8; 0xe0],
    #[doc = "0x300 - Interrupt Priority Register n"]
    pub nvicip0: crate::Reg<nvicip0::NVICIP0_SPEC>,
    #[doc = "0x301 - Interrupt Priority Register n"]
    pub nvicip1: crate::Reg<nvicip1::NVICIP1_SPEC>,
    #[doc = "0x302 - Interrupt Priority Register n"]
    pub nvicip2: crate::Reg<nvicip2::NVICIP2_SPEC>,
    #[doc = "0x303 - Interrupt Priority Register n"]
    pub nvicip3: crate::Reg<nvicip3::NVICIP3_SPEC>,
    #[doc = "0x304 - Interrupt Priority Register n"]
    pub nvicip4: crate::Reg<nvicip4::NVICIP4_SPEC>,
    #[doc = "0x305 - Interrupt Priority Register n"]
    pub nvicip5: crate::Reg<nvicip5::NVICIP5_SPEC>,
    #[doc = "0x306 - Interrupt Priority Register n"]
    pub nvicip6: crate::Reg<nvicip6::NVICIP6_SPEC>,
    #[doc = "0x307 - Interrupt Priority Register n"]
    pub nvicip7: crate::Reg<nvicip7::NVICIP7_SPEC>,
    #[doc = "0x308 - Interrupt Priority Register n"]
    pub nvicip8: crate::Reg<nvicip8::NVICIP8_SPEC>,
    #[doc = "0x309 - Interrupt Priority Register n"]
    pub nvicip9: crate::Reg<nvicip9::NVICIP9_SPEC>,
    #[doc = "0x30a - Interrupt Priority Register n"]
    pub nvicip10: crate::Reg<nvicip10::NVICIP10_SPEC>,
    #[doc = "0x30b - Interrupt Priority Register n"]
    pub nvicip11: crate::Reg<nvicip11::NVICIP11_SPEC>,
    #[doc = "0x30c - Interrupt Priority Register n"]
    pub nvicip12: crate::Reg<nvicip12::NVICIP12_SPEC>,
    #[doc = "0x30d - Interrupt Priority Register n"]
    pub nvicip13: crate::Reg<nvicip13::NVICIP13_SPEC>,
    #[doc = "0x30e - Interrupt Priority Register n"]
    pub nvicip14: crate::Reg<nvicip14::NVICIP14_SPEC>,
    #[doc = "0x30f - Interrupt Priority Register n"]
    pub nvicip15: crate::Reg<nvicip15::NVICIP15_SPEC>,
    #[doc = "0x310 - Interrupt Priority Register n"]
    pub nvicip16: crate::Reg<nvicip16::NVICIP16_SPEC>,
    #[doc = "0x311 - Interrupt Priority Register n"]
    pub nvicip17: crate::Reg<nvicip17::NVICIP17_SPEC>,
    #[doc = "0x312 - Interrupt Priority Register n"]
    pub nvicip18: crate::Reg<nvicip18::NVICIP18_SPEC>,
    #[doc = "0x313 - Interrupt Priority Register n"]
    pub nvicip19: crate::Reg<nvicip19::NVICIP19_SPEC>,
    #[doc = "0x314 - Interrupt Priority Register n"]
    pub nvicip20: crate::Reg<nvicip20::NVICIP20_SPEC>,
    #[doc = "0x315 - Interrupt Priority Register n"]
    pub nvicip21: crate::Reg<nvicip21::NVICIP21_SPEC>,
    #[doc = "0x316 - Interrupt Priority Register n"]
    pub nvicip22: crate::Reg<nvicip22::NVICIP22_SPEC>,
    #[doc = "0x317 - Interrupt Priority Register n"]
    pub nvicip23: crate::Reg<nvicip23::NVICIP23_SPEC>,
    #[doc = "0x318 - Interrupt Priority Register n"]
    pub nvicip24: crate::Reg<nvicip24::NVICIP24_SPEC>,
    #[doc = "0x319 - Interrupt Priority Register n"]
    pub nvicip25: crate::Reg<nvicip25::NVICIP25_SPEC>,
    #[doc = "0x31a - Interrupt Priority Register n"]
    pub nvicip26: crate::Reg<nvicip26::NVICIP26_SPEC>,
    #[doc = "0x31b - Interrupt Priority Register n"]
    pub nvicip27: crate::Reg<nvicip27::NVICIP27_SPEC>,
    #[doc = "0x31c - Interrupt Priority Register n"]
    pub nvicip28: crate::Reg<nvicip28::NVICIP28_SPEC>,
    #[doc = "0x31d - Interrupt Priority Register n"]
    pub nvicip29: crate::Reg<nvicip29::NVICIP29_SPEC>,
    #[doc = "0x31e - Interrupt Priority Register n"]
    pub nvicip30: crate::Reg<nvicip30::NVICIP30_SPEC>,
    #[doc = "0x31f - Interrupt Priority Register n"]
    pub nvicip31: crate::Reg<nvicip31::NVICIP31_SPEC>,
    #[doc = "0x320 - Interrupt Priority Register n"]
    pub nvicip32: crate::Reg<nvicip32::NVICIP32_SPEC>,
    #[doc = "0x321 - Interrupt Priority Register n"]
    pub nvicip33: crate::Reg<nvicip33::NVICIP33_SPEC>,
    #[doc = "0x322 - Interrupt Priority Register n"]
    pub nvicip34: crate::Reg<nvicip34::NVICIP34_SPEC>,
    #[doc = "0x323 - Interrupt Priority Register n"]
    pub nvicip35: crate::Reg<nvicip35::NVICIP35_SPEC>,
    #[doc = "0x324 - Interrupt Priority Register n"]
    pub nvicip36: crate::Reg<nvicip36::NVICIP36_SPEC>,
    #[doc = "0x325 - Interrupt Priority Register n"]
    pub nvicip37: crate::Reg<nvicip37::NVICIP37_SPEC>,
    #[doc = "0x326 - Interrupt Priority Register n"]
    pub nvicip38: crate::Reg<nvicip38::NVICIP38_SPEC>,
    #[doc = "0x327 - Interrupt Priority Register n"]
    pub nvicip39: crate::Reg<nvicip39::NVICIP39_SPEC>,
    #[doc = "0x328 - Interrupt Priority Register n"]
    pub nvicip40: crate::Reg<nvicip40::NVICIP40_SPEC>,
    #[doc = "0x329 - Interrupt Priority Register n"]
    pub nvicip41: crate::Reg<nvicip41::NVICIP41_SPEC>,
    #[doc = "0x32a - Interrupt Priority Register n"]
    pub nvicip42: crate::Reg<nvicip42::NVICIP42_SPEC>,
    #[doc = "0x32b - Interrupt Priority Register n"]
    pub nvicip43: crate::Reg<nvicip43::NVICIP43_SPEC>,
    #[doc = "0x32c - Interrupt Priority Register n"]
    pub nvicip44: crate::Reg<nvicip44::NVICIP44_SPEC>,
    #[doc = "0x32d - Interrupt Priority Register n"]
    pub nvicip45: crate::Reg<nvicip45::NVICIP45_SPEC>,
    #[doc = "0x32e - Interrupt Priority Register n"]
    pub nvicip46: crate::Reg<nvicip46::NVICIP46_SPEC>,
    #[doc = "0x32f - Interrupt Priority Register n"]
    pub nvicip47: crate::Reg<nvicip47::NVICIP47_SPEC>,
    #[doc = "0x330 - Interrupt Priority Register n"]
    pub nvicip48: crate::Reg<nvicip48::NVICIP48_SPEC>,
    #[doc = "0x331 - Interrupt Priority Register n"]
    pub nvicip49: crate::Reg<nvicip49::NVICIP49_SPEC>,
    #[doc = "0x332 - Interrupt Priority Register n"]
    pub nvicip50: crate::Reg<nvicip50::NVICIP50_SPEC>,
    #[doc = "0x333 - Interrupt Priority Register n"]
    pub nvicip51: crate::Reg<nvicip51::NVICIP51_SPEC>,
    #[doc = "0x334 - Interrupt Priority Register n"]
    pub nvicip52: crate::Reg<nvicip52::NVICIP52_SPEC>,
    #[doc = "0x335 - Interrupt Priority Register n"]
    pub nvicip53: crate::Reg<nvicip53::NVICIP53_SPEC>,
    #[doc = "0x336 - Interrupt Priority Register n"]
    pub nvicip54: crate::Reg<nvicip54::NVICIP54_SPEC>,
    #[doc = "0x337 - Interrupt Priority Register n"]
    pub nvicip55: crate::Reg<nvicip55::NVICIP55_SPEC>,
    #[doc = "0x338 - Interrupt Priority Register n"]
    pub nvicip56: crate::Reg<nvicip56::NVICIP56_SPEC>,
    #[doc = "0x339 - Interrupt Priority Register n"]
    pub nvicip57: crate::Reg<nvicip57::NVICIP57_SPEC>,
    #[doc = "0x33a - Interrupt Priority Register n"]
    pub nvicip58: crate::Reg<nvicip58::NVICIP58_SPEC>,
    #[doc = "0x33b - Interrupt Priority Register n"]
    pub nvicip59: crate::Reg<nvicip59::NVICIP59_SPEC>,
    #[doc = "0x33c - Interrupt Priority Register n"]
    pub nvicip60: crate::Reg<nvicip60::NVICIP60_SPEC>,
    #[doc = "0x33d - Interrupt Priority Register n"]
    pub nvicip61: crate::Reg<nvicip61::NVICIP61_SPEC>,
    #[doc = "0x33e - Interrupt Priority Register n"]
    pub nvicip62: crate::Reg<nvicip62::NVICIP62_SPEC>,
    #[doc = "0x33f - Interrupt Priority Register n"]
    pub nvicip63: crate::Reg<nvicip63::NVICIP63_SPEC>,
    #[doc = "0x340 - Interrupt Priority Register n"]
    pub nvicip64: crate::Reg<nvicip64::NVICIP64_SPEC>,
    #[doc = "0x341 - Interrupt Priority Register n"]
    pub nvicip65: crate::Reg<nvicip65::NVICIP65_SPEC>,
    #[doc = "0x342 - Interrupt Priority Register n"]
    pub nvicip66: crate::Reg<nvicip66::NVICIP66_SPEC>,
    #[doc = "0x343 - Interrupt Priority Register n"]
    pub nvicip67: crate::Reg<nvicip67::NVICIP67_SPEC>,
    #[doc = "0x344 - Interrupt Priority Register n"]
    pub nvicip68: crate::Reg<nvicip68::NVICIP68_SPEC>,
    #[doc = "0x345 - Interrupt Priority Register n"]
    pub nvicip69: crate::Reg<nvicip69::NVICIP69_SPEC>,
    #[doc = "0x346 - Interrupt Priority Register n"]
    pub nvicip70: crate::Reg<nvicip70::NVICIP70_SPEC>,
    #[doc = "0x347 - Interrupt Priority Register n"]
    pub nvicip71: crate::Reg<nvicip71::NVICIP71_SPEC>,
    #[doc = "0x348 - Interrupt Priority Register n"]
    pub nvicip72: crate::Reg<nvicip72::NVICIP72_SPEC>,
    #[doc = "0x349 - Interrupt Priority Register n"]
    pub nvicip73: crate::Reg<nvicip73::NVICIP73_SPEC>,
    #[doc = "0x34a - Interrupt Priority Register n"]
    pub nvicip74: crate::Reg<nvicip74::NVICIP74_SPEC>,
    #[doc = "0x34b - Interrupt Priority Register n"]
    pub nvicip75: crate::Reg<nvicip75::NVICIP75_SPEC>,
    #[doc = "0x34c - Interrupt Priority Register n"]
    pub nvicip76: crate::Reg<nvicip76::NVICIP76_SPEC>,
    #[doc = "0x34d - Interrupt Priority Register n"]
    pub nvicip77: crate::Reg<nvicip77::NVICIP77_SPEC>,
    #[doc = "0x34e - Interrupt Priority Register n"]
    pub nvicip78: crate::Reg<nvicip78::NVICIP78_SPEC>,
    #[doc = "0x34f - Interrupt Priority Register n"]
    pub nvicip79: crate::Reg<nvicip79::NVICIP79_SPEC>,
    #[doc = "0x350 - Interrupt Priority Register n"]
    pub nvicip80: crate::Reg<nvicip80::NVICIP80_SPEC>,
    #[doc = "0x351 - Interrupt Priority Register n"]
    pub nvicip81: crate::Reg<nvicip81::NVICIP81_SPEC>,
    #[doc = "0x352 - Interrupt Priority Register n"]
    pub nvicip82: crate::Reg<nvicip82::NVICIP82_SPEC>,
    #[doc = "0x353 - Interrupt Priority Register n"]
    pub nvicip83: crate::Reg<nvicip83::NVICIP83_SPEC>,
    #[doc = "0x354 - Interrupt Priority Register n"]
    pub nvicip84: crate::Reg<nvicip84::NVICIP84_SPEC>,
    #[doc = "0x355 - Interrupt Priority Register n"]
    pub nvicip85: crate::Reg<nvicip85::NVICIP85_SPEC>,
    #[doc = "0x356 - Interrupt Priority Register n"]
    pub nvicip86: crate::Reg<nvicip86::NVICIP86_SPEC>,
    #[doc = "0x357 - Interrupt Priority Register n"]
    pub nvicip87: crate::Reg<nvicip87::NVICIP87_SPEC>,
    #[doc = "0x358 - Interrupt Priority Register n"]
    pub nvicip88: crate::Reg<nvicip88::NVICIP88_SPEC>,
    #[doc = "0x359 - Interrupt Priority Register n"]
    pub nvicip89: crate::Reg<nvicip89::NVICIP89_SPEC>,
    #[doc = "0x35a - Interrupt Priority Register n"]
    pub nvicip90: crate::Reg<nvicip90::NVICIP90_SPEC>,
    #[doc = "0x35b - Interrupt Priority Register n"]
    pub nvicip91: crate::Reg<nvicip91::NVICIP91_SPEC>,
    #[doc = "0x35c - Interrupt Priority Register n"]
    pub nvicip92: crate::Reg<nvicip92::NVICIP92_SPEC>,
    #[doc = "0x35d - Interrupt Priority Register n"]
    pub nvicip93: crate::Reg<nvicip93::NVICIP93_SPEC>,
    #[doc = "0x35e - Interrupt Priority Register n"]
    pub nvicip94: crate::Reg<nvicip94::NVICIP94_SPEC>,
    #[doc = "0x35f - Interrupt Priority Register n"]
    pub nvicip95: crate::Reg<nvicip95::NVICIP95_SPEC>,
    #[doc = "0x360 - Interrupt Priority Register n"]
    pub nvicip96: crate::Reg<nvicip96::NVICIP96_SPEC>,
    #[doc = "0x361 - Interrupt Priority Register n"]
    pub nvicip97: crate::Reg<nvicip97::NVICIP97_SPEC>,
    #[doc = "0x362 - Interrupt Priority Register n"]
    pub nvicip98: crate::Reg<nvicip98::NVICIP98_SPEC>,
    #[doc = "0x363 - Interrupt Priority Register n"]
    pub nvicip99: crate::Reg<nvicip99::NVICIP99_SPEC>,
    #[doc = "0x364 - Interrupt Priority Register n"]
    pub nvicip100: crate::Reg<nvicip100::NVICIP100_SPEC>,
    #[doc = "0x365 - Interrupt Priority Register n"]
    pub nvicip101: crate::Reg<nvicip101::NVICIP101_SPEC>,
    #[doc = "0x366 - Interrupt Priority Register n"]
    pub nvicip102: crate::Reg<nvicip102::NVICIP102_SPEC>,
    #[doc = "0x367 - Interrupt Priority Register n"]
    pub nvicip103: crate::Reg<nvicip103::NVICIP103_SPEC>,
    #[doc = "0x368 - Interrupt Priority Register n"]
    pub nvicip104: crate::Reg<nvicip104::NVICIP104_SPEC>,
    #[doc = "0x369 - Interrupt Priority Register n"]
    pub nvicip105: crate::Reg<nvicip105::NVICIP105_SPEC>,
    #[doc = "0x36a - Interrupt Priority Register n"]
    pub nvicip106: crate::Reg<nvicip106::NVICIP106_SPEC>,
    #[doc = "0x36b - Interrupt Priority Register n"]
    pub nvicip107: crate::Reg<nvicip107::NVICIP107_SPEC>,
    #[doc = "0x36c - Interrupt Priority Register n"]
    pub nvicip108: crate::Reg<nvicip108::NVICIP108_SPEC>,
    #[doc = "0x36d - Interrupt Priority Register n"]
    pub nvicip109: crate::Reg<nvicip109::NVICIP109_SPEC>,
    #[doc = "0x36e - Interrupt Priority Register n"]
    pub nvicip110: crate::Reg<nvicip110::NVICIP110_SPEC>,
    #[doc = "0x36f - Interrupt Priority Register n"]
    pub nvicip111: crate::Reg<nvicip111::NVICIP111_SPEC>,
    #[doc = "0x370 - Interrupt Priority Register n"]
    pub nvicip112: crate::Reg<nvicip112::NVICIP112_SPEC>,
    #[doc = "0x371 - Interrupt Priority Register n"]
    pub nvicip113: crate::Reg<nvicip113::NVICIP113_SPEC>,
    #[doc = "0x372 - Interrupt Priority Register n"]
    pub nvicip114: crate::Reg<nvicip114::NVICIP114_SPEC>,
    #[doc = "0x373 - Interrupt Priority Register n"]
    pub nvicip115: crate::Reg<nvicip115::NVICIP115_SPEC>,
    #[doc = "0x374 - Interrupt Priority Register n"]
    pub nvicip116: crate::Reg<nvicip116::NVICIP116_SPEC>,
    #[doc = "0x375 - Interrupt Priority Register n"]
    pub nvicip117: crate::Reg<nvicip117::NVICIP117_SPEC>,
    #[doc = "0x376 - Interrupt Priority Register n"]
    pub nvicip118: crate::Reg<nvicip118::NVICIP118_SPEC>,
    #[doc = "0x377 - Interrupt Priority Register n"]
    pub nvicip119: crate::Reg<nvicip119::NVICIP119_SPEC>,
    #[doc = "0x378 - Interrupt Priority Register n"]
    pub nvicip120: crate::Reg<nvicip120::NVICIP120_SPEC>,
    #[doc = "0x379 - Interrupt Priority Register n"]
    pub nvicip121: crate::Reg<nvicip121::NVICIP121_SPEC>,
    #[doc = "0x37a - Interrupt Priority Register n"]
    pub nvicip122: crate::Reg<nvicip122::NVICIP122_SPEC>,
    #[doc = "0x37b - Interrupt Priority Register n"]
    pub nvicip123: crate::Reg<nvicip123::NVICIP123_SPEC>,
    #[doc = "0x37c - Interrupt Priority Register n"]
    pub nvicip124: crate::Reg<nvicip124::NVICIP124_SPEC>,
    #[doc = "0x37d - Interrupt Priority Register n"]
    pub nvicip125: crate::Reg<nvicip125::NVICIP125_SPEC>,
    #[doc = "0x37e - Interrupt Priority Register n"]
    pub nvicip126: crate::Reg<nvicip126::NVICIP126_SPEC>,
    #[doc = "0x37f - Interrupt Priority Register n"]
    pub nvicip127: crate::Reg<nvicip127::NVICIP127_SPEC>,
    #[doc = "0x380 - Interrupt Priority Register n"]
    pub nvicip128: crate::Reg<nvicip128::NVICIP128_SPEC>,
    #[doc = "0x381 - Interrupt Priority Register n"]
    pub nvicip129: crate::Reg<nvicip129::NVICIP129_SPEC>,
    #[doc = "0x382 - Interrupt Priority Register n"]
    pub nvicip130: crate::Reg<nvicip130::NVICIP130_SPEC>,
    #[doc = "0x383 - Interrupt Priority Register n"]
    pub nvicip131: crate::Reg<nvicip131::NVICIP131_SPEC>,
    #[doc = "0x384 - Interrupt Priority Register n"]
    pub nvicip132: crate::Reg<nvicip132::NVICIP132_SPEC>,
    #[doc = "0x385 - Interrupt Priority Register n"]
    pub nvicip133: crate::Reg<nvicip133::NVICIP133_SPEC>,
    #[doc = "0x386 - Interrupt Priority Register n"]
    pub nvicip134: crate::Reg<nvicip134::NVICIP134_SPEC>,
    #[doc = "0x387 - Interrupt Priority Register n"]
    pub nvicip135: crate::Reg<nvicip135::NVICIP135_SPEC>,
    #[doc = "0x388 - Interrupt Priority Register n"]
    pub nvicip136: crate::Reg<nvicip136::NVICIP136_SPEC>,
    #[doc = "0x389 - Interrupt Priority Register n"]
    pub nvicip137: crate::Reg<nvicip137::NVICIP137_SPEC>,
    #[doc = "0x38a - Interrupt Priority Register n"]
    pub nvicip138: crate::Reg<nvicip138::NVICIP138_SPEC>,
    #[doc = "0x38b - Interrupt Priority Register n"]
    pub nvicip139: crate::Reg<nvicip139::NVICIP139_SPEC>,
    #[doc = "0x38c - Interrupt Priority Register n"]
    pub nvicip140: crate::Reg<nvicip140::NVICIP140_SPEC>,
    #[doc = "0x38d - Interrupt Priority Register n"]
    pub nvicip141: crate::Reg<nvicip141::NVICIP141_SPEC>,
    #[doc = "0x38e - Interrupt Priority Register n"]
    pub nvicip142: crate::Reg<nvicip142::NVICIP142_SPEC>,
    #[doc = "0x38f - Interrupt Priority Register n"]
    pub nvicip143: crate::Reg<nvicip143::NVICIP143_SPEC>,
    #[doc = "0x390 - Interrupt Priority Register n"]
    pub nvicip144: crate::Reg<nvicip144::NVICIP144_SPEC>,
    #[doc = "0x391 - Interrupt Priority Register n"]
    pub nvicip145: crate::Reg<nvicip145::NVICIP145_SPEC>,
    #[doc = "0x392 - Interrupt Priority Register n"]
    pub nvicip146: crate::Reg<nvicip146::NVICIP146_SPEC>,
    #[doc = "0x393 - Interrupt Priority Register n"]
    pub nvicip147: crate::Reg<nvicip147::NVICIP147_SPEC>,
    #[doc = "0x394 - Interrupt Priority Register n"]
    pub nvicip148: crate::Reg<nvicip148::NVICIP148_SPEC>,
    #[doc = "0x395 - Interrupt Priority Register n"]
    pub nvicip149: crate::Reg<nvicip149::NVICIP149_SPEC>,
    #[doc = "0x396 - Interrupt Priority Register n"]
    pub nvicip150: crate::Reg<nvicip150::NVICIP150_SPEC>,
    #[doc = "0x397 - Interrupt Priority Register n"]
    pub nvicip151: crate::Reg<nvicip151::NVICIP151_SPEC>,
    #[doc = "0x398 - Interrupt Priority Register n"]
    pub nvicip152: crate::Reg<nvicip152::NVICIP152_SPEC>,
    #[doc = "0x399 - Interrupt Priority Register n"]
    pub nvicip153: crate::Reg<nvicip153::NVICIP153_SPEC>,
    #[doc = "0x39a - Interrupt Priority Register n"]
    pub nvicip154: crate::Reg<nvicip154::NVICIP154_SPEC>,
    #[doc = "0x39b - Interrupt Priority Register n"]
    pub nvicip155: crate::Reg<nvicip155::NVICIP155_SPEC>,
    #[doc = "0x39c - Interrupt Priority Register n"]
    pub nvicip156: crate::Reg<nvicip156::NVICIP156_SPEC>,
    #[doc = "0x39d - Interrupt Priority Register n"]
    pub nvicip157: crate::Reg<nvicip157::NVICIP157_SPEC>,
    #[doc = "0x39e - Interrupt Priority Register n"]
    pub nvicip158: crate::Reg<nvicip158::NVICIP158_SPEC>,
    #[doc = "0x39f - Interrupt Priority Register n"]
    pub nvicip159: crate::Reg<nvicip159::NVICIP159_SPEC>,
    #[doc = "0x3a0 - Interrupt Priority Register n"]
    pub nvicip160: crate::Reg<nvicip160::NVICIP160_SPEC>,
    #[doc = "0x3a1 - Interrupt Priority Register n"]
    pub nvicip161: crate::Reg<nvicip161::NVICIP161_SPEC>,
    #[doc = "0x3a2 - Interrupt Priority Register n"]
    pub nvicip162: crate::Reg<nvicip162::NVICIP162_SPEC>,
    #[doc = "0x3a3 - Interrupt Priority Register n"]
    pub nvicip163: crate::Reg<nvicip163::NVICIP163_SPEC>,
    #[doc = "0x3a4 - Interrupt Priority Register n"]
    pub nvicip164: crate::Reg<nvicip164::NVICIP164_SPEC>,
    #[doc = "0x3a5 - Interrupt Priority Register n"]
    pub nvicip165: crate::Reg<nvicip165::NVICIP165_SPEC>,
    #[doc = "0x3a6 - Interrupt Priority Register n"]
    pub nvicip166: crate::Reg<nvicip166::NVICIP166_SPEC>,
    #[doc = "0x3a7 - Interrupt Priority Register n"]
    pub nvicip167: crate::Reg<nvicip167::NVICIP167_SPEC>,
    #[doc = "0x3a8 - Interrupt Priority Register n"]
    pub nvicip168: crate::Reg<nvicip168::NVICIP168_SPEC>,
    #[doc = "0x3a9 - Interrupt Priority Register n"]
    pub nvicip169: crate::Reg<nvicip169::NVICIP169_SPEC>,
    #[doc = "0x3aa - Interrupt Priority Register n"]
    pub nvicip170: crate::Reg<nvicip170::NVICIP170_SPEC>,
    #[doc = "0x3ab - Interrupt Priority Register n"]
    pub nvicip171: crate::Reg<nvicip171::NVICIP171_SPEC>,
    #[doc = "0x3ac - Interrupt Priority Register n"]
    pub nvicip172: crate::Reg<nvicip172::NVICIP172_SPEC>,
    #[doc = "0x3ad - Interrupt Priority Register n"]
    pub nvicip173: crate::Reg<nvicip173::NVICIP173_SPEC>,
    #[doc = "0x3ae - Interrupt Priority Register n"]
    pub nvicip174: crate::Reg<nvicip174::NVICIP174_SPEC>,
    #[doc = "0x3af - Interrupt Priority Register n"]
    pub nvicip175: crate::Reg<nvicip175::NVICIP175_SPEC>,
    #[doc = "0x3b0 - Interrupt Priority Register n"]
    pub nvicip176: crate::Reg<nvicip176::NVICIP176_SPEC>,
    #[doc = "0x3b1 - Interrupt Priority Register n"]
    pub nvicip177: crate::Reg<nvicip177::NVICIP177_SPEC>,
    #[doc = "0x3b2 - Interrupt Priority Register n"]
    pub nvicip178: crate::Reg<nvicip178::NVICIP178_SPEC>,
    #[doc = "0x3b3 - Interrupt Priority Register n"]
    pub nvicip179: crate::Reg<nvicip179::NVICIP179_SPEC>,
    #[doc = "0x3b4 - Interrupt Priority Register n"]
    pub nvicip180: crate::Reg<nvicip180::NVICIP180_SPEC>,
    #[doc = "0x3b5 - Interrupt Priority Register n"]
    pub nvicip181: crate::Reg<nvicip181::NVICIP181_SPEC>,
    #[doc = "0x3b6 - Interrupt Priority Register n"]
    pub nvicip182: crate::Reg<nvicip182::NVICIP182_SPEC>,
    #[doc = "0x3b7 - Interrupt Priority Register n"]
    pub nvicip183: crate::Reg<nvicip183::NVICIP183_SPEC>,
    #[doc = "0x3b8 - Interrupt Priority Register n"]
    pub nvicip184: crate::Reg<nvicip184::NVICIP184_SPEC>,
    #[doc = "0x3b9 - Interrupt Priority Register n"]
    pub nvicip185: crate::Reg<nvicip185::NVICIP185_SPEC>,
    #[doc = "0x3ba - Interrupt Priority Register n"]
    pub nvicip186: crate::Reg<nvicip186::NVICIP186_SPEC>,
    #[doc = "0x3bb - Interrupt Priority Register n"]
    pub nvicip187: crate::Reg<nvicip187::NVICIP187_SPEC>,
    #[doc = "0x3bc - Interrupt Priority Register n"]
    pub nvicip188: crate::Reg<nvicip188::NVICIP188_SPEC>,
    #[doc = "0x3bd - Interrupt Priority Register n"]
    pub nvicip189: crate::Reg<nvicip189::NVICIP189_SPEC>,
    #[doc = "0x3be - Interrupt Priority Register n"]
    pub nvicip190: crate::Reg<nvicip190::NVICIP190_SPEC>,
    #[doc = "0x3bf - Interrupt Priority Register n"]
    pub nvicip191: crate::Reg<nvicip191::NVICIP191_SPEC>,
    #[doc = "0x3c0 - Interrupt Priority Register n"]
    pub nvicip192: crate::Reg<nvicip192::NVICIP192_SPEC>,
    #[doc = "0x3c1 - Interrupt Priority Register n"]
    pub nvicip193: crate::Reg<nvicip193::NVICIP193_SPEC>,
    #[doc = "0x3c2 - Interrupt Priority Register n"]
    pub nvicip194: crate::Reg<nvicip194::NVICIP194_SPEC>,
    #[doc = "0x3c3 - Interrupt Priority Register n"]
    pub nvicip195: crate::Reg<nvicip195::NVICIP195_SPEC>,
    #[doc = "0x3c4 - Interrupt Priority Register n"]
    pub nvicip196: crate::Reg<nvicip196::NVICIP196_SPEC>,
    #[doc = "0x3c5 - Interrupt Priority Register n"]
    pub nvicip197: crate::Reg<nvicip197::NVICIP197_SPEC>,
    #[doc = "0x3c6 - Interrupt Priority Register n"]
    pub nvicip198: crate::Reg<nvicip198::NVICIP198_SPEC>,
    #[doc = "0x3c7 - Interrupt Priority Register n"]
    pub nvicip199: crate::Reg<nvicip199::NVICIP199_SPEC>,
    #[doc = "0x3c8 - Interrupt Priority Register n"]
    pub nvicip200: crate::Reg<nvicip200::NVICIP200_SPEC>,
    #[doc = "0x3c9 - Interrupt Priority Register n"]
    pub nvicip201: crate::Reg<nvicip201::NVICIP201_SPEC>,
    #[doc = "0x3ca - Interrupt Priority Register n"]
    pub nvicip202: crate::Reg<nvicip202::NVICIP202_SPEC>,
    #[doc = "0x3cb - Interrupt Priority Register n"]
    pub nvicip203: crate::Reg<nvicip203::NVICIP203_SPEC>,
    #[doc = "0x3cc - Interrupt Priority Register n"]
    pub nvicip204: crate::Reg<nvicip204::NVICIP204_SPEC>,
    #[doc = "0x3cd - Interrupt Priority Register n"]
    pub nvicip205: crate::Reg<nvicip205::NVICIP205_SPEC>,
    #[doc = "0x3ce - Interrupt Priority Register n"]
    pub nvicip206: crate::Reg<nvicip206::NVICIP206_SPEC>,
    #[doc = "0x3cf - Interrupt Priority Register n"]
    pub nvicip207: crate::Reg<nvicip207::NVICIP207_SPEC>,
    #[doc = "0x3d0 - Interrupt Priority Register n"]
    pub nvicip208: crate::Reg<nvicip208::NVICIP208_SPEC>,
    #[doc = "0x3d1 - Interrupt Priority Register n"]
    pub nvicip209: crate::Reg<nvicip209::NVICIP209_SPEC>,
    #[doc = "0x3d2 - Interrupt Priority Register n"]
    pub nvicip210: crate::Reg<nvicip210::NVICIP210_SPEC>,
    #[doc = "0x3d3 - Interrupt Priority Register n"]
    pub nvicip211: crate::Reg<nvicip211::NVICIP211_SPEC>,
    #[doc = "0x3d4 - Interrupt Priority Register n"]
    pub nvicip212: crate::Reg<nvicip212::NVICIP212_SPEC>,
    #[doc = "0x3d5 - Interrupt Priority Register n"]
    pub nvicip213: crate::Reg<nvicip213::NVICIP213_SPEC>,
    #[doc = "0x3d6 - Interrupt Priority Register n"]
    pub nvicip214: crate::Reg<nvicip214::NVICIP214_SPEC>,
    #[doc = "0x3d7 - Interrupt Priority Register n"]
    pub nvicip215: crate::Reg<nvicip215::NVICIP215_SPEC>,
    #[doc = "0x3d8 - Interrupt Priority Register n"]
    pub nvicip216: crate::Reg<nvicip216::NVICIP216_SPEC>,
    #[doc = "0x3d9 - Interrupt Priority Register n"]
    pub nvicip217: crate::Reg<nvicip217::NVICIP217_SPEC>,
    #[doc = "0x3da - Interrupt Priority Register n"]
    pub nvicip218: crate::Reg<nvicip218::NVICIP218_SPEC>,
    #[doc = "0x3db - Interrupt Priority Register n"]
    pub nvicip219: crate::Reg<nvicip219::NVICIP219_SPEC>,
    #[doc = "0x3dc - Interrupt Priority Register n"]
    pub nvicip220: crate::Reg<nvicip220::NVICIP220_SPEC>,
    #[doc = "0x3dd - Interrupt Priority Register n"]
    pub nvicip221: crate::Reg<nvicip221::NVICIP221_SPEC>,
    #[doc = "0x3de - Interrupt Priority Register n"]
    pub nvicip222: crate::Reg<nvicip222::NVICIP222_SPEC>,
    #[doc = "0x3df - Interrupt Priority Register n"]
    pub nvicip223: crate::Reg<nvicip223::NVICIP223_SPEC>,
    #[doc = "0x3e0 - Interrupt Priority Register n"]
    pub nvicip224: crate::Reg<nvicip224::NVICIP224_SPEC>,
    #[doc = "0x3e1 - Interrupt Priority Register n"]
    pub nvicip225: crate::Reg<nvicip225::NVICIP225_SPEC>,
    #[doc = "0x3e2 - Interrupt Priority Register n"]
    pub nvicip226: crate::Reg<nvicip226::NVICIP226_SPEC>,
    #[doc = "0x3e3 - Interrupt Priority Register n"]
    pub nvicip227: crate::Reg<nvicip227::NVICIP227_SPEC>,
    #[doc = "0x3e4 - Interrupt Priority Register n"]
    pub nvicip228: crate::Reg<nvicip228::NVICIP228_SPEC>,
    #[doc = "0x3e5 - Interrupt Priority Register n"]
    pub nvicip229: crate::Reg<nvicip229::NVICIP229_SPEC>,
    #[doc = "0x3e6 - Interrupt Priority Register n"]
    pub nvicip230: crate::Reg<nvicip230::NVICIP230_SPEC>,
    #[doc = "0x3e7 - Interrupt Priority Register n"]
    pub nvicip231: crate::Reg<nvicip231::NVICIP231_SPEC>,
    #[doc = "0x3e8 - Interrupt Priority Register n"]
    pub nvicip232: crate::Reg<nvicip232::NVICIP232_SPEC>,
    #[doc = "0x3e9 - Interrupt Priority Register n"]
    pub nvicip233: crate::Reg<nvicip233::NVICIP233_SPEC>,
    #[doc = "0x3ea - Interrupt Priority Register n"]
    pub nvicip234: crate::Reg<nvicip234::NVICIP234_SPEC>,
    #[doc = "0x3eb - Interrupt Priority Register n"]
    pub nvicip235: crate::Reg<nvicip235::NVICIP235_SPEC>,
    #[doc = "0x3ec - Interrupt Priority Register n"]
    pub nvicip236: crate::Reg<nvicip236::NVICIP236_SPEC>,
    #[doc = "0x3ed - Interrupt Priority Register n"]
    pub nvicip237: crate::Reg<nvicip237::NVICIP237_SPEC>,
    #[doc = "0x3ee - Interrupt Priority Register n"]
    pub nvicip238: crate::Reg<nvicip238::NVICIP238_SPEC>,
    #[doc = "0x3ef - Interrupt Priority Register n"]
    pub nvicip239: crate::Reg<nvicip239::NVICIP239_SPEC>,
    _reserved280: [u8; 0x0a10],
    #[doc = "0xe00 - Software Trigger Interrupt Register"]
    pub nvicstir: crate::Reg<nvicstir::NVICSTIR_SPEC>,
}
#[doc = "NVICISER0 register accessor: an alias for `Reg<NVICISER0_SPEC>`"]
pub type NVICISER0 = crate::Reg<nviciser0::NVICISER0_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser0;
#[doc = "NVICISER1 register accessor: an alias for `Reg<NVICISER1_SPEC>`"]
pub type NVICISER1 = crate::Reg<nviciser1::NVICISER1_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser1;
#[doc = "NVICISER2 register accessor: an alias for `Reg<NVICISER2_SPEC>`"]
pub type NVICISER2 = crate::Reg<nviciser2::NVICISER2_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser2;
#[doc = "NVICISER3 register accessor: an alias for `Reg<NVICISER3_SPEC>`"]
pub type NVICISER3 = crate::Reg<nviciser3::NVICISER3_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser3;
#[doc = "NVICISER4 register accessor: an alias for `Reg<NVICISER4_SPEC>`"]
pub type NVICISER4 = crate::Reg<nviciser4::NVICISER4_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser4;
#[doc = "NVICISER5 register accessor: an alias for `Reg<NVICISER5_SPEC>`"]
pub type NVICISER5 = crate::Reg<nviciser5::NVICISER5_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser5;
#[doc = "NVICISER6 register accessor: an alias for `Reg<NVICISER6_SPEC>`"]
pub type NVICISER6 = crate::Reg<nviciser6::NVICISER6_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser6;
#[doc = "NVICISER7 register accessor: an alias for `Reg<NVICISER7_SPEC>`"]
pub type NVICISER7 = crate::Reg<nviciser7::NVICISER7_SPEC>;
#[doc = "Interrupt Set Enable Register n"]
pub mod nviciser7;
#[doc = "NVICICER0 register accessor: an alias for `Reg<NVICICER0_SPEC>`"]
pub type NVICICER0 = crate::Reg<nvicicer0::NVICICER0_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer0;
#[doc = "NVICICER1 register accessor: an alias for `Reg<NVICICER1_SPEC>`"]
pub type NVICICER1 = crate::Reg<nvicicer1::NVICICER1_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer1;
#[doc = "NVICICER2 register accessor: an alias for `Reg<NVICICER2_SPEC>`"]
pub type NVICICER2 = crate::Reg<nvicicer2::NVICICER2_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer2;
#[doc = "NVICICER3 register accessor: an alias for `Reg<NVICICER3_SPEC>`"]
pub type NVICICER3 = crate::Reg<nvicicer3::NVICICER3_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer3;
#[doc = "NVICICER4 register accessor: an alias for `Reg<NVICICER4_SPEC>`"]
pub type NVICICER4 = crate::Reg<nvicicer4::NVICICER4_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer4;
#[doc = "NVICICER5 register accessor: an alias for `Reg<NVICICER5_SPEC>`"]
pub type NVICICER5 = crate::Reg<nvicicer5::NVICICER5_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer5;
#[doc = "NVICICER6 register accessor: an alias for `Reg<NVICICER6_SPEC>`"]
pub type NVICICER6 = crate::Reg<nvicicer6::NVICICER6_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer6;
#[doc = "NVICICER7 register accessor: an alias for `Reg<NVICICER7_SPEC>`"]
pub type NVICICER7 = crate::Reg<nvicicer7::NVICICER7_SPEC>;
#[doc = "Interrupt Clear Enable Register n"]
pub mod nvicicer7;
#[doc = "NVICISPR0 register accessor: an alias for `Reg<NVICISPR0_SPEC>`"]
pub type NVICISPR0 = crate::Reg<nvicispr0::NVICISPR0_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr0;
#[doc = "NVICISPR1 register accessor: an alias for `Reg<NVICISPR1_SPEC>`"]
pub type NVICISPR1 = crate::Reg<nvicispr1::NVICISPR1_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr1;
#[doc = "NVICISPR2 register accessor: an alias for `Reg<NVICISPR2_SPEC>`"]
pub type NVICISPR2 = crate::Reg<nvicispr2::NVICISPR2_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr2;
#[doc = "NVICISPR3 register accessor: an alias for `Reg<NVICISPR3_SPEC>`"]
pub type NVICISPR3 = crate::Reg<nvicispr3::NVICISPR3_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr3;
#[doc = "NVICISPR4 register accessor: an alias for `Reg<NVICISPR4_SPEC>`"]
pub type NVICISPR4 = crate::Reg<nvicispr4::NVICISPR4_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr4;
#[doc = "NVICISPR5 register accessor: an alias for `Reg<NVICISPR5_SPEC>`"]
pub type NVICISPR5 = crate::Reg<nvicispr5::NVICISPR5_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr5;
#[doc = "NVICISPR6 register accessor: an alias for `Reg<NVICISPR6_SPEC>`"]
pub type NVICISPR6 = crate::Reg<nvicispr6::NVICISPR6_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr6;
#[doc = "NVICISPR7 register accessor: an alias for `Reg<NVICISPR7_SPEC>`"]
pub type NVICISPR7 = crate::Reg<nvicispr7::NVICISPR7_SPEC>;
#[doc = "Interrupt Set Pending Register n"]
pub mod nvicispr7;
#[doc = "NVICICPR0 register accessor: an alias for `Reg<NVICICPR0_SPEC>`"]
pub type NVICICPR0 = crate::Reg<nvicicpr0::NVICICPR0_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr0;
#[doc = "NVICICPR1 register accessor: an alias for `Reg<NVICICPR1_SPEC>`"]
pub type NVICICPR1 = crate::Reg<nvicicpr1::NVICICPR1_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr1;
#[doc = "NVICICPR2 register accessor: an alias for `Reg<NVICICPR2_SPEC>`"]
pub type NVICICPR2 = crate::Reg<nvicicpr2::NVICICPR2_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr2;
#[doc = "NVICICPR3 register accessor: an alias for `Reg<NVICICPR3_SPEC>`"]
pub type NVICICPR3 = crate::Reg<nvicicpr3::NVICICPR3_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr3;
#[doc = "NVICICPR4 register accessor: an alias for `Reg<NVICICPR4_SPEC>`"]
pub type NVICICPR4 = crate::Reg<nvicicpr4::NVICICPR4_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr4;
#[doc = "NVICICPR5 register accessor: an alias for `Reg<NVICICPR5_SPEC>`"]
pub type NVICICPR5 = crate::Reg<nvicicpr5::NVICICPR5_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr5;
#[doc = "NVICICPR6 register accessor: an alias for `Reg<NVICICPR6_SPEC>`"]
pub type NVICICPR6 = crate::Reg<nvicicpr6::NVICICPR6_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr6;
#[doc = "NVICICPR7 register accessor: an alias for `Reg<NVICICPR7_SPEC>`"]
pub type NVICICPR7 = crate::Reg<nvicicpr7::NVICICPR7_SPEC>;
#[doc = "Interrupt Clear Pending Register n"]
pub mod nvicicpr7;
#[doc = "NVICIABR0 register accessor: an alias for `Reg<NVICIABR0_SPEC>`"]
pub type NVICIABR0 = crate::Reg<nviciabr0::NVICIABR0_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr0;
#[doc = "NVICIABR1 register accessor: an alias for `Reg<NVICIABR1_SPEC>`"]
pub type NVICIABR1 = crate::Reg<nviciabr1::NVICIABR1_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr1;
#[doc = "NVICIABR2 register accessor: an alias for `Reg<NVICIABR2_SPEC>`"]
pub type NVICIABR2 = crate::Reg<nviciabr2::NVICIABR2_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr2;
#[doc = "NVICIABR3 register accessor: an alias for `Reg<NVICIABR3_SPEC>`"]
pub type NVICIABR3 = crate::Reg<nviciabr3::NVICIABR3_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr3;
#[doc = "NVICIABR4 register accessor: an alias for `Reg<NVICIABR4_SPEC>`"]
pub type NVICIABR4 = crate::Reg<nviciabr4::NVICIABR4_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr4;
#[doc = "NVICIABR5 register accessor: an alias for `Reg<NVICIABR5_SPEC>`"]
pub type NVICIABR5 = crate::Reg<nviciabr5::NVICIABR5_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr5;
#[doc = "NVICIABR6 register accessor: an alias for `Reg<NVICIABR6_SPEC>`"]
pub type NVICIABR6 = crate::Reg<nviciabr6::NVICIABR6_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr6;
#[doc = "NVICIABR7 register accessor: an alias for `Reg<NVICIABR7_SPEC>`"]
pub type NVICIABR7 = crate::Reg<nviciabr7::NVICIABR7_SPEC>;
#[doc = "Interrupt Active bit Register n"]
pub mod nviciabr7;
#[doc = "NVICIP0 register accessor: an alias for `Reg<NVICIP0_SPEC>`"]
pub type NVICIP0 = crate::Reg<nvicip0::NVICIP0_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip0;
#[doc = "NVICIP1 register accessor: an alias for `Reg<NVICIP1_SPEC>`"]
pub type NVICIP1 = crate::Reg<nvicip1::NVICIP1_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip1;
#[doc = "NVICIP2 register accessor: an alias for `Reg<NVICIP2_SPEC>`"]
pub type NVICIP2 = crate::Reg<nvicip2::NVICIP2_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip2;
#[doc = "NVICIP3 register accessor: an alias for `Reg<NVICIP3_SPEC>`"]
pub type NVICIP3 = crate::Reg<nvicip3::NVICIP3_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip3;
#[doc = "NVICIP4 register accessor: an alias for `Reg<NVICIP4_SPEC>`"]
pub type NVICIP4 = crate::Reg<nvicip4::NVICIP4_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip4;
#[doc = "NVICIP5 register accessor: an alias for `Reg<NVICIP5_SPEC>`"]
pub type NVICIP5 = crate::Reg<nvicip5::NVICIP5_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip5;
#[doc = "NVICIP6 register accessor: an alias for `Reg<NVICIP6_SPEC>`"]
pub type NVICIP6 = crate::Reg<nvicip6::NVICIP6_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip6;
#[doc = "NVICIP7 register accessor: an alias for `Reg<NVICIP7_SPEC>`"]
pub type NVICIP7 = crate::Reg<nvicip7::NVICIP7_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip7;
#[doc = "NVICIP8 register accessor: an alias for `Reg<NVICIP8_SPEC>`"]
pub type NVICIP8 = crate::Reg<nvicip8::NVICIP8_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip8;
#[doc = "NVICIP9 register accessor: an alias for `Reg<NVICIP9_SPEC>`"]
pub type NVICIP9 = crate::Reg<nvicip9::NVICIP9_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip9;
#[doc = "NVICIP10 register accessor: an alias for `Reg<NVICIP10_SPEC>`"]
pub type NVICIP10 = crate::Reg<nvicip10::NVICIP10_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip10;
#[doc = "NVICIP11 register accessor: an alias for `Reg<NVICIP11_SPEC>`"]
pub type NVICIP11 = crate::Reg<nvicip11::NVICIP11_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip11;
#[doc = "NVICIP12 register accessor: an alias for `Reg<NVICIP12_SPEC>`"]
pub type NVICIP12 = crate::Reg<nvicip12::NVICIP12_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip12;
#[doc = "NVICIP13 register accessor: an alias for `Reg<NVICIP13_SPEC>`"]
pub type NVICIP13 = crate::Reg<nvicip13::NVICIP13_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip13;
#[doc = "NVICIP14 register accessor: an alias for `Reg<NVICIP14_SPEC>`"]
pub type NVICIP14 = crate::Reg<nvicip14::NVICIP14_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip14;
#[doc = "NVICIP15 register accessor: an alias for `Reg<NVICIP15_SPEC>`"]
pub type NVICIP15 = crate::Reg<nvicip15::NVICIP15_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip15;
#[doc = "NVICIP16 register accessor: an alias for `Reg<NVICIP16_SPEC>`"]
pub type NVICIP16 = crate::Reg<nvicip16::NVICIP16_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip16;
#[doc = "NVICIP17 register accessor: an alias for `Reg<NVICIP17_SPEC>`"]
pub type NVICIP17 = crate::Reg<nvicip17::NVICIP17_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip17;
#[doc = "NVICIP18 register accessor: an alias for `Reg<NVICIP18_SPEC>`"]
pub type NVICIP18 = crate::Reg<nvicip18::NVICIP18_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip18;
#[doc = "NVICIP19 register accessor: an alias for `Reg<NVICIP19_SPEC>`"]
pub type NVICIP19 = crate::Reg<nvicip19::NVICIP19_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip19;
#[doc = "NVICIP20 register accessor: an alias for `Reg<NVICIP20_SPEC>`"]
pub type NVICIP20 = crate::Reg<nvicip20::NVICIP20_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip20;
#[doc = "NVICIP21 register accessor: an alias for `Reg<NVICIP21_SPEC>`"]
pub type NVICIP21 = crate::Reg<nvicip21::NVICIP21_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip21;
#[doc = "NVICIP22 register accessor: an alias for `Reg<NVICIP22_SPEC>`"]
pub type NVICIP22 = crate::Reg<nvicip22::NVICIP22_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip22;
#[doc = "NVICIP23 register accessor: an alias for `Reg<NVICIP23_SPEC>`"]
pub type NVICIP23 = crate::Reg<nvicip23::NVICIP23_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip23;
#[doc = "NVICIP24 register accessor: an alias for `Reg<NVICIP24_SPEC>`"]
pub type NVICIP24 = crate::Reg<nvicip24::NVICIP24_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip24;
#[doc = "NVICIP25 register accessor: an alias for `Reg<NVICIP25_SPEC>`"]
pub type NVICIP25 = crate::Reg<nvicip25::NVICIP25_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip25;
#[doc = "NVICIP26 register accessor: an alias for `Reg<NVICIP26_SPEC>`"]
pub type NVICIP26 = crate::Reg<nvicip26::NVICIP26_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip26;
#[doc = "NVICIP27 register accessor: an alias for `Reg<NVICIP27_SPEC>`"]
pub type NVICIP27 = crate::Reg<nvicip27::NVICIP27_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip27;
#[doc = "NVICIP28 register accessor: an alias for `Reg<NVICIP28_SPEC>`"]
pub type NVICIP28 = crate::Reg<nvicip28::NVICIP28_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip28;
#[doc = "NVICIP29 register accessor: an alias for `Reg<NVICIP29_SPEC>`"]
pub type NVICIP29 = crate::Reg<nvicip29::NVICIP29_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip29;
#[doc = "NVICIP30 register accessor: an alias for `Reg<NVICIP30_SPEC>`"]
pub type NVICIP30 = crate::Reg<nvicip30::NVICIP30_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip30;
#[doc = "NVICIP31 register accessor: an alias for `Reg<NVICIP31_SPEC>`"]
pub type NVICIP31 = crate::Reg<nvicip31::NVICIP31_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip31;
#[doc = "NVICIP32 register accessor: an alias for `Reg<NVICIP32_SPEC>`"]
pub type NVICIP32 = crate::Reg<nvicip32::NVICIP32_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip32;
#[doc = "NVICIP33 register accessor: an alias for `Reg<NVICIP33_SPEC>`"]
pub type NVICIP33 = crate::Reg<nvicip33::NVICIP33_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip33;
#[doc = "NVICIP34 register accessor: an alias for `Reg<NVICIP34_SPEC>`"]
pub type NVICIP34 = crate::Reg<nvicip34::NVICIP34_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip34;
#[doc = "NVICIP35 register accessor: an alias for `Reg<NVICIP35_SPEC>`"]
pub type NVICIP35 = crate::Reg<nvicip35::NVICIP35_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip35;
#[doc = "NVICIP36 register accessor: an alias for `Reg<NVICIP36_SPEC>`"]
pub type NVICIP36 = crate::Reg<nvicip36::NVICIP36_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip36;
#[doc = "NVICIP37 register accessor: an alias for `Reg<NVICIP37_SPEC>`"]
pub type NVICIP37 = crate::Reg<nvicip37::NVICIP37_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip37;
#[doc = "NVICIP38 register accessor: an alias for `Reg<NVICIP38_SPEC>`"]
pub type NVICIP38 = crate::Reg<nvicip38::NVICIP38_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip38;
#[doc = "NVICIP39 register accessor: an alias for `Reg<NVICIP39_SPEC>`"]
pub type NVICIP39 = crate::Reg<nvicip39::NVICIP39_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip39;
#[doc = "NVICIP40 register accessor: an alias for `Reg<NVICIP40_SPEC>`"]
pub type NVICIP40 = crate::Reg<nvicip40::NVICIP40_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip40;
#[doc = "NVICIP41 register accessor: an alias for `Reg<NVICIP41_SPEC>`"]
pub type NVICIP41 = crate::Reg<nvicip41::NVICIP41_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip41;
#[doc = "NVICIP42 register accessor: an alias for `Reg<NVICIP42_SPEC>`"]
pub type NVICIP42 = crate::Reg<nvicip42::NVICIP42_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip42;
#[doc = "NVICIP43 register accessor: an alias for `Reg<NVICIP43_SPEC>`"]
pub type NVICIP43 = crate::Reg<nvicip43::NVICIP43_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip43;
#[doc = "NVICIP44 register accessor: an alias for `Reg<NVICIP44_SPEC>`"]
pub type NVICIP44 = crate::Reg<nvicip44::NVICIP44_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip44;
#[doc = "NVICIP45 register accessor: an alias for `Reg<NVICIP45_SPEC>`"]
pub type NVICIP45 = crate::Reg<nvicip45::NVICIP45_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip45;
#[doc = "NVICIP46 register accessor: an alias for `Reg<NVICIP46_SPEC>`"]
pub type NVICIP46 = crate::Reg<nvicip46::NVICIP46_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip46;
#[doc = "NVICIP47 register accessor: an alias for `Reg<NVICIP47_SPEC>`"]
pub type NVICIP47 = crate::Reg<nvicip47::NVICIP47_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip47;
#[doc = "NVICIP48 register accessor: an alias for `Reg<NVICIP48_SPEC>`"]
pub type NVICIP48 = crate::Reg<nvicip48::NVICIP48_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip48;
#[doc = "NVICIP49 register accessor: an alias for `Reg<NVICIP49_SPEC>`"]
pub type NVICIP49 = crate::Reg<nvicip49::NVICIP49_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip49;
#[doc = "NVICIP50 register accessor: an alias for `Reg<NVICIP50_SPEC>`"]
pub type NVICIP50 = crate::Reg<nvicip50::NVICIP50_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip50;
#[doc = "NVICIP51 register accessor: an alias for `Reg<NVICIP51_SPEC>`"]
pub type NVICIP51 = crate::Reg<nvicip51::NVICIP51_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip51;
#[doc = "NVICIP52 register accessor: an alias for `Reg<NVICIP52_SPEC>`"]
pub type NVICIP52 = crate::Reg<nvicip52::NVICIP52_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip52;
#[doc = "NVICIP53 register accessor: an alias for `Reg<NVICIP53_SPEC>`"]
pub type NVICIP53 = crate::Reg<nvicip53::NVICIP53_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip53;
#[doc = "NVICIP54 register accessor: an alias for `Reg<NVICIP54_SPEC>`"]
pub type NVICIP54 = crate::Reg<nvicip54::NVICIP54_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip54;
#[doc = "NVICIP55 register accessor: an alias for `Reg<NVICIP55_SPEC>`"]
pub type NVICIP55 = crate::Reg<nvicip55::NVICIP55_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip55;
#[doc = "NVICIP56 register accessor: an alias for `Reg<NVICIP56_SPEC>`"]
pub type NVICIP56 = crate::Reg<nvicip56::NVICIP56_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip56;
#[doc = "NVICIP57 register accessor: an alias for `Reg<NVICIP57_SPEC>`"]
pub type NVICIP57 = crate::Reg<nvicip57::NVICIP57_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip57;
#[doc = "NVICIP58 register accessor: an alias for `Reg<NVICIP58_SPEC>`"]
pub type NVICIP58 = crate::Reg<nvicip58::NVICIP58_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip58;
#[doc = "NVICIP59 register accessor: an alias for `Reg<NVICIP59_SPEC>`"]
pub type NVICIP59 = crate::Reg<nvicip59::NVICIP59_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip59;
#[doc = "NVICIP60 register accessor: an alias for `Reg<NVICIP60_SPEC>`"]
pub type NVICIP60 = crate::Reg<nvicip60::NVICIP60_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip60;
#[doc = "NVICIP61 register accessor: an alias for `Reg<NVICIP61_SPEC>`"]
pub type NVICIP61 = crate::Reg<nvicip61::NVICIP61_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip61;
#[doc = "NVICIP62 register accessor: an alias for `Reg<NVICIP62_SPEC>`"]
pub type NVICIP62 = crate::Reg<nvicip62::NVICIP62_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip62;
#[doc = "NVICIP63 register accessor: an alias for `Reg<NVICIP63_SPEC>`"]
pub type NVICIP63 = crate::Reg<nvicip63::NVICIP63_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip63;
#[doc = "NVICIP64 register accessor: an alias for `Reg<NVICIP64_SPEC>`"]
pub type NVICIP64 = crate::Reg<nvicip64::NVICIP64_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip64;
#[doc = "NVICIP65 register accessor: an alias for `Reg<NVICIP65_SPEC>`"]
pub type NVICIP65 = crate::Reg<nvicip65::NVICIP65_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip65;
#[doc = "NVICIP66 register accessor: an alias for `Reg<NVICIP66_SPEC>`"]
pub type NVICIP66 = crate::Reg<nvicip66::NVICIP66_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip66;
#[doc = "NVICIP67 register accessor: an alias for `Reg<NVICIP67_SPEC>`"]
pub type NVICIP67 = crate::Reg<nvicip67::NVICIP67_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip67;
#[doc = "NVICIP68 register accessor: an alias for `Reg<NVICIP68_SPEC>`"]
pub type NVICIP68 = crate::Reg<nvicip68::NVICIP68_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip68;
#[doc = "NVICIP69 register accessor: an alias for `Reg<NVICIP69_SPEC>`"]
pub type NVICIP69 = crate::Reg<nvicip69::NVICIP69_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip69;
#[doc = "NVICIP70 register accessor: an alias for `Reg<NVICIP70_SPEC>`"]
pub type NVICIP70 = crate::Reg<nvicip70::NVICIP70_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip70;
#[doc = "NVICIP71 register accessor: an alias for `Reg<NVICIP71_SPEC>`"]
pub type NVICIP71 = crate::Reg<nvicip71::NVICIP71_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip71;
#[doc = "NVICIP72 register accessor: an alias for `Reg<NVICIP72_SPEC>`"]
pub type NVICIP72 = crate::Reg<nvicip72::NVICIP72_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip72;
#[doc = "NVICIP73 register accessor: an alias for `Reg<NVICIP73_SPEC>`"]
pub type NVICIP73 = crate::Reg<nvicip73::NVICIP73_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip73;
#[doc = "NVICIP74 register accessor: an alias for `Reg<NVICIP74_SPEC>`"]
pub type NVICIP74 = crate::Reg<nvicip74::NVICIP74_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip74;
#[doc = "NVICIP75 register accessor: an alias for `Reg<NVICIP75_SPEC>`"]
pub type NVICIP75 = crate::Reg<nvicip75::NVICIP75_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip75;
#[doc = "NVICIP76 register accessor: an alias for `Reg<NVICIP76_SPEC>`"]
pub type NVICIP76 = crate::Reg<nvicip76::NVICIP76_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip76;
#[doc = "NVICIP77 register accessor: an alias for `Reg<NVICIP77_SPEC>`"]
pub type NVICIP77 = crate::Reg<nvicip77::NVICIP77_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip77;
#[doc = "NVICIP78 register accessor: an alias for `Reg<NVICIP78_SPEC>`"]
pub type NVICIP78 = crate::Reg<nvicip78::NVICIP78_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip78;
#[doc = "NVICIP79 register accessor: an alias for `Reg<NVICIP79_SPEC>`"]
pub type NVICIP79 = crate::Reg<nvicip79::NVICIP79_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip79;
#[doc = "NVICIP80 register accessor: an alias for `Reg<NVICIP80_SPEC>`"]
pub type NVICIP80 = crate::Reg<nvicip80::NVICIP80_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip80;
#[doc = "NVICIP81 register accessor: an alias for `Reg<NVICIP81_SPEC>`"]
pub type NVICIP81 = crate::Reg<nvicip81::NVICIP81_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip81;
#[doc = "NVICIP82 register accessor: an alias for `Reg<NVICIP82_SPEC>`"]
pub type NVICIP82 = crate::Reg<nvicip82::NVICIP82_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip82;
#[doc = "NVICIP83 register accessor: an alias for `Reg<NVICIP83_SPEC>`"]
pub type NVICIP83 = crate::Reg<nvicip83::NVICIP83_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip83;
#[doc = "NVICIP84 register accessor: an alias for `Reg<NVICIP84_SPEC>`"]
pub type NVICIP84 = crate::Reg<nvicip84::NVICIP84_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip84;
#[doc = "NVICIP85 register accessor: an alias for `Reg<NVICIP85_SPEC>`"]
pub type NVICIP85 = crate::Reg<nvicip85::NVICIP85_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip85;
#[doc = "NVICIP86 register accessor: an alias for `Reg<NVICIP86_SPEC>`"]
pub type NVICIP86 = crate::Reg<nvicip86::NVICIP86_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip86;
#[doc = "NVICIP87 register accessor: an alias for `Reg<NVICIP87_SPEC>`"]
pub type NVICIP87 = crate::Reg<nvicip87::NVICIP87_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip87;
#[doc = "NVICIP88 register accessor: an alias for `Reg<NVICIP88_SPEC>`"]
pub type NVICIP88 = crate::Reg<nvicip88::NVICIP88_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip88;
#[doc = "NVICIP89 register accessor: an alias for `Reg<NVICIP89_SPEC>`"]
pub type NVICIP89 = crate::Reg<nvicip89::NVICIP89_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip89;
#[doc = "NVICIP90 register accessor: an alias for `Reg<NVICIP90_SPEC>`"]
pub type NVICIP90 = crate::Reg<nvicip90::NVICIP90_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip90;
#[doc = "NVICIP91 register accessor: an alias for `Reg<NVICIP91_SPEC>`"]
pub type NVICIP91 = crate::Reg<nvicip91::NVICIP91_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip91;
#[doc = "NVICIP92 register accessor: an alias for `Reg<NVICIP92_SPEC>`"]
pub type NVICIP92 = crate::Reg<nvicip92::NVICIP92_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip92;
#[doc = "NVICIP93 register accessor: an alias for `Reg<NVICIP93_SPEC>`"]
pub type NVICIP93 = crate::Reg<nvicip93::NVICIP93_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip93;
#[doc = "NVICIP94 register accessor: an alias for `Reg<NVICIP94_SPEC>`"]
pub type NVICIP94 = crate::Reg<nvicip94::NVICIP94_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip94;
#[doc = "NVICIP95 register accessor: an alias for `Reg<NVICIP95_SPEC>`"]
pub type NVICIP95 = crate::Reg<nvicip95::NVICIP95_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip95;
#[doc = "NVICIP96 register accessor: an alias for `Reg<NVICIP96_SPEC>`"]
pub type NVICIP96 = crate::Reg<nvicip96::NVICIP96_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip96;
#[doc = "NVICIP97 register accessor: an alias for `Reg<NVICIP97_SPEC>`"]
pub type NVICIP97 = crate::Reg<nvicip97::NVICIP97_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip97;
#[doc = "NVICIP98 register accessor: an alias for `Reg<NVICIP98_SPEC>`"]
pub type NVICIP98 = crate::Reg<nvicip98::NVICIP98_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip98;
#[doc = "NVICIP99 register accessor: an alias for `Reg<NVICIP99_SPEC>`"]
pub type NVICIP99 = crate::Reg<nvicip99::NVICIP99_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip99;
#[doc = "NVICIP100 register accessor: an alias for `Reg<NVICIP100_SPEC>`"]
pub type NVICIP100 = crate::Reg<nvicip100::NVICIP100_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip100;
#[doc = "NVICIP101 register accessor: an alias for `Reg<NVICIP101_SPEC>`"]
pub type NVICIP101 = crate::Reg<nvicip101::NVICIP101_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip101;
#[doc = "NVICIP102 register accessor: an alias for `Reg<NVICIP102_SPEC>`"]
pub type NVICIP102 = crate::Reg<nvicip102::NVICIP102_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip102;
#[doc = "NVICIP103 register accessor: an alias for `Reg<NVICIP103_SPEC>`"]
pub type NVICIP103 = crate::Reg<nvicip103::NVICIP103_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip103;
#[doc = "NVICIP104 register accessor: an alias for `Reg<NVICIP104_SPEC>`"]
pub type NVICIP104 = crate::Reg<nvicip104::NVICIP104_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip104;
#[doc = "NVICIP105 register accessor: an alias for `Reg<NVICIP105_SPEC>`"]
pub type NVICIP105 = crate::Reg<nvicip105::NVICIP105_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip105;
#[doc = "NVICIP106 register accessor: an alias for `Reg<NVICIP106_SPEC>`"]
pub type NVICIP106 = crate::Reg<nvicip106::NVICIP106_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip106;
#[doc = "NVICIP107 register accessor: an alias for `Reg<NVICIP107_SPEC>`"]
pub type NVICIP107 = crate::Reg<nvicip107::NVICIP107_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip107;
#[doc = "NVICIP108 register accessor: an alias for `Reg<NVICIP108_SPEC>`"]
pub type NVICIP108 = crate::Reg<nvicip108::NVICIP108_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip108;
#[doc = "NVICIP109 register accessor: an alias for `Reg<NVICIP109_SPEC>`"]
pub type NVICIP109 = crate::Reg<nvicip109::NVICIP109_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip109;
#[doc = "NVICIP110 register accessor: an alias for `Reg<NVICIP110_SPEC>`"]
pub type NVICIP110 = crate::Reg<nvicip110::NVICIP110_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip110;
#[doc = "NVICIP111 register accessor: an alias for `Reg<NVICIP111_SPEC>`"]
pub type NVICIP111 = crate::Reg<nvicip111::NVICIP111_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip111;
#[doc = "NVICIP112 register accessor: an alias for `Reg<NVICIP112_SPEC>`"]
pub type NVICIP112 = crate::Reg<nvicip112::NVICIP112_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip112;
#[doc = "NVICIP113 register accessor: an alias for `Reg<NVICIP113_SPEC>`"]
pub type NVICIP113 = crate::Reg<nvicip113::NVICIP113_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip113;
#[doc = "NVICIP114 register accessor: an alias for `Reg<NVICIP114_SPEC>`"]
pub type NVICIP114 = crate::Reg<nvicip114::NVICIP114_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip114;
#[doc = "NVICIP115 register accessor: an alias for `Reg<NVICIP115_SPEC>`"]
pub type NVICIP115 = crate::Reg<nvicip115::NVICIP115_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip115;
#[doc = "NVICIP116 register accessor: an alias for `Reg<NVICIP116_SPEC>`"]
pub type NVICIP116 = crate::Reg<nvicip116::NVICIP116_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip116;
#[doc = "NVICIP117 register accessor: an alias for `Reg<NVICIP117_SPEC>`"]
pub type NVICIP117 = crate::Reg<nvicip117::NVICIP117_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip117;
#[doc = "NVICIP118 register accessor: an alias for `Reg<NVICIP118_SPEC>`"]
pub type NVICIP118 = crate::Reg<nvicip118::NVICIP118_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip118;
#[doc = "NVICIP119 register accessor: an alias for `Reg<NVICIP119_SPEC>`"]
pub type NVICIP119 = crate::Reg<nvicip119::NVICIP119_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip119;
#[doc = "NVICIP120 register accessor: an alias for `Reg<NVICIP120_SPEC>`"]
pub type NVICIP120 = crate::Reg<nvicip120::NVICIP120_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip120;
#[doc = "NVICIP121 register accessor: an alias for `Reg<NVICIP121_SPEC>`"]
pub type NVICIP121 = crate::Reg<nvicip121::NVICIP121_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip121;
#[doc = "NVICIP122 register accessor: an alias for `Reg<NVICIP122_SPEC>`"]
pub type NVICIP122 = crate::Reg<nvicip122::NVICIP122_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip122;
#[doc = "NVICIP123 register accessor: an alias for `Reg<NVICIP123_SPEC>`"]
pub type NVICIP123 = crate::Reg<nvicip123::NVICIP123_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip123;
#[doc = "NVICIP124 register accessor: an alias for `Reg<NVICIP124_SPEC>`"]
pub type NVICIP124 = crate::Reg<nvicip124::NVICIP124_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip124;
#[doc = "NVICIP125 register accessor: an alias for `Reg<NVICIP125_SPEC>`"]
pub type NVICIP125 = crate::Reg<nvicip125::NVICIP125_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip125;
#[doc = "NVICIP126 register accessor: an alias for `Reg<NVICIP126_SPEC>`"]
pub type NVICIP126 = crate::Reg<nvicip126::NVICIP126_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip126;
#[doc = "NVICIP127 register accessor: an alias for `Reg<NVICIP127_SPEC>`"]
pub type NVICIP127 = crate::Reg<nvicip127::NVICIP127_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip127;
#[doc = "NVICIP128 register accessor: an alias for `Reg<NVICIP128_SPEC>`"]
pub type NVICIP128 = crate::Reg<nvicip128::NVICIP128_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip128;
#[doc = "NVICIP129 register accessor: an alias for `Reg<NVICIP129_SPEC>`"]
pub type NVICIP129 = crate::Reg<nvicip129::NVICIP129_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip129;
#[doc = "NVICIP130 register accessor: an alias for `Reg<NVICIP130_SPEC>`"]
pub type NVICIP130 = crate::Reg<nvicip130::NVICIP130_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip130;
#[doc = "NVICIP131 register accessor: an alias for `Reg<NVICIP131_SPEC>`"]
pub type NVICIP131 = crate::Reg<nvicip131::NVICIP131_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip131;
#[doc = "NVICIP132 register accessor: an alias for `Reg<NVICIP132_SPEC>`"]
pub type NVICIP132 = crate::Reg<nvicip132::NVICIP132_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip132;
#[doc = "NVICIP133 register accessor: an alias for `Reg<NVICIP133_SPEC>`"]
pub type NVICIP133 = crate::Reg<nvicip133::NVICIP133_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip133;
#[doc = "NVICIP134 register accessor: an alias for `Reg<NVICIP134_SPEC>`"]
pub type NVICIP134 = crate::Reg<nvicip134::NVICIP134_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip134;
#[doc = "NVICIP135 register accessor: an alias for `Reg<NVICIP135_SPEC>`"]
pub type NVICIP135 = crate::Reg<nvicip135::NVICIP135_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip135;
#[doc = "NVICIP136 register accessor: an alias for `Reg<NVICIP136_SPEC>`"]
pub type NVICIP136 = crate::Reg<nvicip136::NVICIP136_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip136;
#[doc = "NVICIP137 register accessor: an alias for `Reg<NVICIP137_SPEC>`"]
pub type NVICIP137 = crate::Reg<nvicip137::NVICIP137_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip137;
#[doc = "NVICIP138 register accessor: an alias for `Reg<NVICIP138_SPEC>`"]
pub type NVICIP138 = crate::Reg<nvicip138::NVICIP138_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip138;
#[doc = "NVICIP139 register accessor: an alias for `Reg<NVICIP139_SPEC>`"]
pub type NVICIP139 = crate::Reg<nvicip139::NVICIP139_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip139;
#[doc = "NVICIP140 register accessor: an alias for `Reg<NVICIP140_SPEC>`"]
pub type NVICIP140 = crate::Reg<nvicip140::NVICIP140_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip140;
#[doc = "NVICIP141 register accessor: an alias for `Reg<NVICIP141_SPEC>`"]
pub type NVICIP141 = crate::Reg<nvicip141::NVICIP141_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip141;
#[doc = "NVICIP142 register accessor: an alias for `Reg<NVICIP142_SPEC>`"]
pub type NVICIP142 = crate::Reg<nvicip142::NVICIP142_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip142;
#[doc = "NVICIP143 register accessor: an alias for `Reg<NVICIP143_SPEC>`"]
pub type NVICIP143 = crate::Reg<nvicip143::NVICIP143_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip143;
#[doc = "NVICIP144 register accessor: an alias for `Reg<NVICIP144_SPEC>`"]
pub type NVICIP144 = crate::Reg<nvicip144::NVICIP144_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip144;
#[doc = "NVICIP145 register accessor: an alias for `Reg<NVICIP145_SPEC>`"]
pub type NVICIP145 = crate::Reg<nvicip145::NVICIP145_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip145;
#[doc = "NVICIP146 register accessor: an alias for `Reg<NVICIP146_SPEC>`"]
pub type NVICIP146 = crate::Reg<nvicip146::NVICIP146_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip146;
#[doc = "NVICIP147 register accessor: an alias for `Reg<NVICIP147_SPEC>`"]
pub type NVICIP147 = crate::Reg<nvicip147::NVICIP147_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip147;
#[doc = "NVICIP148 register accessor: an alias for `Reg<NVICIP148_SPEC>`"]
pub type NVICIP148 = crate::Reg<nvicip148::NVICIP148_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip148;
#[doc = "NVICIP149 register accessor: an alias for `Reg<NVICIP149_SPEC>`"]
pub type NVICIP149 = crate::Reg<nvicip149::NVICIP149_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip149;
#[doc = "NVICIP150 register accessor: an alias for `Reg<NVICIP150_SPEC>`"]
pub type NVICIP150 = crate::Reg<nvicip150::NVICIP150_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip150;
#[doc = "NVICIP151 register accessor: an alias for `Reg<NVICIP151_SPEC>`"]
pub type NVICIP151 = crate::Reg<nvicip151::NVICIP151_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip151;
#[doc = "NVICIP152 register accessor: an alias for `Reg<NVICIP152_SPEC>`"]
pub type NVICIP152 = crate::Reg<nvicip152::NVICIP152_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip152;
#[doc = "NVICIP153 register accessor: an alias for `Reg<NVICIP153_SPEC>`"]
pub type NVICIP153 = crate::Reg<nvicip153::NVICIP153_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip153;
#[doc = "NVICIP154 register accessor: an alias for `Reg<NVICIP154_SPEC>`"]
pub type NVICIP154 = crate::Reg<nvicip154::NVICIP154_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip154;
#[doc = "NVICIP155 register accessor: an alias for `Reg<NVICIP155_SPEC>`"]
pub type NVICIP155 = crate::Reg<nvicip155::NVICIP155_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip155;
#[doc = "NVICIP156 register accessor: an alias for `Reg<NVICIP156_SPEC>`"]
pub type NVICIP156 = crate::Reg<nvicip156::NVICIP156_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip156;
#[doc = "NVICIP157 register accessor: an alias for `Reg<NVICIP157_SPEC>`"]
pub type NVICIP157 = crate::Reg<nvicip157::NVICIP157_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip157;
#[doc = "NVICIP158 register accessor: an alias for `Reg<NVICIP158_SPEC>`"]
pub type NVICIP158 = crate::Reg<nvicip158::NVICIP158_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip158;
#[doc = "NVICIP159 register accessor: an alias for `Reg<NVICIP159_SPEC>`"]
pub type NVICIP159 = crate::Reg<nvicip159::NVICIP159_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip159;
#[doc = "NVICIP160 register accessor: an alias for `Reg<NVICIP160_SPEC>`"]
pub type NVICIP160 = crate::Reg<nvicip160::NVICIP160_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip160;
#[doc = "NVICIP161 register accessor: an alias for `Reg<NVICIP161_SPEC>`"]
pub type NVICIP161 = crate::Reg<nvicip161::NVICIP161_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip161;
#[doc = "NVICIP162 register accessor: an alias for `Reg<NVICIP162_SPEC>`"]
pub type NVICIP162 = crate::Reg<nvicip162::NVICIP162_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip162;
#[doc = "NVICIP163 register accessor: an alias for `Reg<NVICIP163_SPEC>`"]
pub type NVICIP163 = crate::Reg<nvicip163::NVICIP163_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip163;
#[doc = "NVICIP164 register accessor: an alias for `Reg<NVICIP164_SPEC>`"]
pub type NVICIP164 = crate::Reg<nvicip164::NVICIP164_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip164;
#[doc = "NVICIP165 register accessor: an alias for `Reg<NVICIP165_SPEC>`"]
pub type NVICIP165 = crate::Reg<nvicip165::NVICIP165_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip165;
#[doc = "NVICIP166 register accessor: an alias for `Reg<NVICIP166_SPEC>`"]
pub type NVICIP166 = crate::Reg<nvicip166::NVICIP166_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip166;
#[doc = "NVICIP167 register accessor: an alias for `Reg<NVICIP167_SPEC>`"]
pub type NVICIP167 = crate::Reg<nvicip167::NVICIP167_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip167;
#[doc = "NVICIP168 register accessor: an alias for `Reg<NVICIP168_SPEC>`"]
pub type NVICIP168 = crate::Reg<nvicip168::NVICIP168_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip168;
#[doc = "NVICIP169 register accessor: an alias for `Reg<NVICIP169_SPEC>`"]
pub type NVICIP169 = crate::Reg<nvicip169::NVICIP169_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip169;
#[doc = "NVICIP170 register accessor: an alias for `Reg<NVICIP170_SPEC>`"]
pub type NVICIP170 = crate::Reg<nvicip170::NVICIP170_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip170;
#[doc = "NVICIP171 register accessor: an alias for `Reg<NVICIP171_SPEC>`"]
pub type NVICIP171 = crate::Reg<nvicip171::NVICIP171_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip171;
#[doc = "NVICIP172 register accessor: an alias for `Reg<NVICIP172_SPEC>`"]
pub type NVICIP172 = crate::Reg<nvicip172::NVICIP172_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip172;
#[doc = "NVICIP173 register accessor: an alias for `Reg<NVICIP173_SPEC>`"]
pub type NVICIP173 = crate::Reg<nvicip173::NVICIP173_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip173;
#[doc = "NVICIP174 register accessor: an alias for `Reg<NVICIP174_SPEC>`"]
pub type NVICIP174 = crate::Reg<nvicip174::NVICIP174_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip174;
#[doc = "NVICIP175 register accessor: an alias for `Reg<NVICIP175_SPEC>`"]
pub type NVICIP175 = crate::Reg<nvicip175::NVICIP175_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip175;
#[doc = "NVICIP176 register accessor: an alias for `Reg<NVICIP176_SPEC>`"]
pub type NVICIP176 = crate::Reg<nvicip176::NVICIP176_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip176;
#[doc = "NVICIP177 register accessor: an alias for `Reg<NVICIP177_SPEC>`"]
pub type NVICIP177 = crate::Reg<nvicip177::NVICIP177_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip177;
#[doc = "NVICIP178 register accessor: an alias for `Reg<NVICIP178_SPEC>`"]
pub type NVICIP178 = crate::Reg<nvicip178::NVICIP178_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip178;
#[doc = "NVICIP179 register accessor: an alias for `Reg<NVICIP179_SPEC>`"]
pub type NVICIP179 = crate::Reg<nvicip179::NVICIP179_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip179;
#[doc = "NVICIP180 register accessor: an alias for `Reg<NVICIP180_SPEC>`"]
pub type NVICIP180 = crate::Reg<nvicip180::NVICIP180_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip180;
#[doc = "NVICIP181 register accessor: an alias for `Reg<NVICIP181_SPEC>`"]
pub type NVICIP181 = crate::Reg<nvicip181::NVICIP181_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip181;
#[doc = "NVICIP182 register accessor: an alias for `Reg<NVICIP182_SPEC>`"]
pub type NVICIP182 = crate::Reg<nvicip182::NVICIP182_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip182;
#[doc = "NVICIP183 register accessor: an alias for `Reg<NVICIP183_SPEC>`"]
pub type NVICIP183 = crate::Reg<nvicip183::NVICIP183_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip183;
#[doc = "NVICIP184 register accessor: an alias for `Reg<NVICIP184_SPEC>`"]
pub type NVICIP184 = crate::Reg<nvicip184::NVICIP184_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip184;
#[doc = "NVICIP185 register accessor: an alias for `Reg<NVICIP185_SPEC>`"]
pub type NVICIP185 = crate::Reg<nvicip185::NVICIP185_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip185;
#[doc = "NVICIP186 register accessor: an alias for `Reg<NVICIP186_SPEC>`"]
pub type NVICIP186 = crate::Reg<nvicip186::NVICIP186_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip186;
#[doc = "NVICIP187 register accessor: an alias for `Reg<NVICIP187_SPEC>`"]
pub type NVICIP187 = crate::Reg<nvicip187::NVICIP187_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip187;
#[doc = "NVICIP188 register accessor: an alias for `Reg<NVICIP188_SPEC>`"]
pub type NVICIP188 = crate::Reg<nvicip188::NVICIP188_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip188;
#[doc = "NVICIP189 register accessor: an alias for `Reg<NVICIP189_SPEC>`"]
pub type NVICIP189 = crate::Reg<nvicip189::NVICIP189_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip189;
#[doc = "NVICIP190 register accessor: an alias for `Reg<NVICIP190_SPEC>`"]
pub type NVICIP190 = crate::Reg<nvicip190::NVICIP190_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip190;
#[doc = "NVICIP191 register accessor: an alias for `Reg<NVICIP191_SPEC>`"]
pub type NVICIP191 = crate::Reg<nvicip191::NVICIP191_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip191;
#[doc = "NVICIP192 register accessor: an alias for `Reg<NVICIP192_SPEC>`"]
pub type NVICIP192 = crate::Reg<nvicip192::NVICIP192_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip192;
#[doc = "NVICIP193 register accessor: an alias for `Reg<NVICIP193_SPEC>`"]
pub type NVICIP193 = crate::Reg<nvicip193::NVICIP193_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip193;
#[doc = "NVICIP194 register accessor: an alias for `Reg<NVICIP194_SPEC>`"]
pub type NVICIP194 = crate::Reg<nvicip194::NVICIP194_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip194;
#[doc = "NVICIP195 register accessor: an alias for `Reg<NVICIP195_SPEC>`"]
pub type NVICIP195 = crate::Reg<nvicip195::NVICIP195_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip195;
#[doc = "NVICIP196 register accessor: an alias for `Reg<NVICIP196_SPEC>`"]
pub type NVICIP196 = crate::Reg<nvicip196::NVICIP196_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip196;
#[doc = "NVICIP197 register accessor: an alias for `Reg<NVICIP197_SPEC>`"]
pub type NVICIP197 = crate::Reg<nvicip197::NVICIP197_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip197;
#[doc = "NVICIP198 register accessor: an alias for `Reg<NVICIP198_SPEC>`"]
pub type NVICIP198 = crate::Reg<nvicip198::NVICIP198_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip198;
#[doc = "NVICIP199 register accessor: an alias for `Reg<NVICIP199_SPEC>`"]
pub type NVICIP199 = crate::Reg<nvicip199::NVICIP199_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip199;
#[doc = "NVICIP200 register accessor: an alias for `Reg<NVICIP200_SPEC>`"]
pub type NVICIP200 = crate::Reg<nvicip200::NVICIP200_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip200;
#[doc = "NVICIP201 register accessor: an alias for `Reg<NVICIP201_SPEC>`"]
pub type NVICIP201 = crate::Reg<nvicip201::NVICIP201_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip201;
#[doc = "NVICIP202 register accessor: an alias for `Reg<NVICIP202_SPEC>`"]
pub type NVICIP202 = crate::Reg<nvicip202::NVICIP202_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip202;
#[doc = "NVICIP203 register accessor: an alias for `Reg<NVICIP203_SPEC>`"]
pub type NVICIP203 = crate::Reg<nvicip203::NVICIP203_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip203;
#[doc = "NVICIP204 register accessor: an alias for `Reg<NVICIP204_SPEC>`"]
pub type NVICIP204 = crate::Reg<nvicip204::NVICIP204_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip204;
#[doc = "NVICIP205 register accessor: an alias for `Reg<NVICIP205_SPEC>`"]
pub type NVICIP205 = crate::Reg<nvicip205::NVICIP205_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip205;
#[doc = "NVICIP206 register accessor: an alias for `Reg<NVICIP206_SPEC>`"]
pub type NVICIP206 = crate::Reg<nvicip206::NVICIP206_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip206;
#[doc = "NVICIP207 register accessor: an alias for `Reg<NVICIP207_SPEC>`"]
pub type NVICIP207 = crate::Reg<nvicip207::NVICIP207_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip207;
#[doc = "NVICIP208 register accessor: an alias for `Reg<NVICIP208_SPEC>`"]
pub type NVICIP208 = crate::Reg<nvicip208::NVICIP208_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip208;
#[doc = "NVICIP209 register accessor: an alias for `Reg<NVICIP209_SPEC>`"]
pub type NVICIP209 = crate::Reg<nvicip209::NVICIP209_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip209;
#[doc = "NVICIP210 register accessor: an alias for `Reg<NVICIP210_SPEC>`"]
pub type NVICIP210 = crate::Reg<nvicip210::NVICIP210_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip210;
#[doc = "NVICIP211 register accessor: an alias for `Reg<NVICIP211_SPEC>`"]
pub type NVICIP211 = crate::Reg<nvicip211::NVICIP211_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip211;
#[doc = "NVICIP212 register accessor: an alias for `Reg<NVICIP212_SPEC>`"]
pub type NVICIP212 = crate::Reg<nvicip212::NVICIP212_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip212;
#[doc = "NVICIP213 register accessor: an alias for `Reg<NVICIP213_SPEC>`"]
pub type NVICIP213 = crate::Reg<nvicip213::NVICIP213_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip213;
#[doc = "NVICIP214 register accessor: an alias for `Reg<NVICIP214_SPEC>`"]
pub type NVICIP214 = crate::Reg<nvicip214::NVICIP214_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip214;
#[doc = "NVICIP215 register accessor: an alias for `Reg<NVICIP215_SPEC>`"]
pub type NVICIP215 = crate::Reg<nvicip215::NVICIP215_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip215;
#[doc = "NVICIP216 register accessor: an alias for `Reg<NVICIP216_SPEC>`"]
pub type NVICIP216 = crate::Reg<nvicip216::NVICIP216_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip216;
#[doc = "NVICIP217 register accessor: an alias for `Reg<NVICIP217_SPEC>`"]
pub type NVICIP217 = crate::Reg<nvicip217::NVICIP217_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip217;
#[doc = "NVICIP218 register accessor: an alias for `Reg<NVICIP218_SPEC>`"]
pub type NVICIP218 = crate::Reg<nvicip218::NVICIP218_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip218;
#[doc = "NVICIP219 register accessor: an alias for `Reg<NVICIP219_SPEC>`"]
pub type NVICIP219 = crate::Reg<nvicip219::NVICIP219_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip219;
#[doc = "NVICIP220 register accessor: an alias for `Reg<NVICIP220_SPEC>`"]
pub type NVICIP220 = crate::Reg<nvicip220::NVICIP220_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip220;
#[doc = "NVICIP221 register accessor: an alias for `Reg<NVICIP221_SPEC>`"]
pub type NVICIP221 = crate::Reg<nvicip221::NVICIP221_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip221;
#[doc = "NVICIP222 register accessor: an alias for `Reg<NVICIP222_SPEC>`"]
pub type NVICIP222 = crate::Reg<nvicip222::NVICIP222_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip222;
#[doc = "NVICIP223 register accessor: an alias for `Reg<NVICIP223_SPEC>`"]
pub type NVICIP223 = crate::Reg<nvicip223::NVICIP223_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip223;
#[doc = "NVICIP224 register accessor: an alias for `Reg<NVICIP224_SPEC>`"]
pub type NVICIP224 = crate::Reg<nvicip224::NVICIP224_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip224;
#[doc = "NVICIP225 register accessor: an alias for `Reg<NVICIP225_SPEC>`"]
pub type NVICIP225 = crate::Reg<nvicip225::NVICIP225_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip225;
#[doc = "NVICIP226 register accessor: an alias for `Reg<NVICIP226_SPEC>`"]
pub type NVICIP226 = crate::Reg<nvicip226::NVICIP226_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip226;
#[doc = "NVICIP227 register accessor: an alias for `Reg<NVICIP227_SPEC>`"]
pub type NVICIP227 = crate::Reg<nvicip227::NVICIP227_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip227;
#[doc = "NVICIP228 register accessor: an alias for `Reg<NVICIP228_SPEC>`"]
pub type NVICIP228 = crate::Reg<nvicip228::NVICIP228_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip228;
#[doc = "NVICIP229 register accessor: an alias for `Reg<NVICIP229_SPEC>`"]
pub type NVICIP229 = crate::Reg<nvicip229::NVICIP229_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip229;
#[doc = "NVICIP230 register accessor: an alias for `Reg<NVICIP230_SPEC>`"]
pub type NVICIP230 = crate::Reg<nvicip230::NVICIP230_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip230;
#[doc = "NVICIP231 register accessor: an alias for `Reg<NVICIP231_SPEC>`"]
pub type NVICIP231 = crate::Reg<nvicip231::NVICIP231_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip231;
#[doc = "NVICIP232 register accessor: an alias for `Reg<NVICIP232_SPEC>`"]
pub type NVICIP232 = crate::Reg<nvicip232::NVICIP232_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip232;
#[doc = "NVICIP233 register accessor: an alias for `Reg<NVICIP233_SPEC>`"]
pub type NVICIP233 = crate::Reg<nvicip233::NVICIP233_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip233;
#[doc = "NVICIP234 register accessor: an alias for `Reg<NVICIP234_SPEC>`"]
pub type NVICIP234 = crate::Reg<nvicip234::NVICIP234_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip234;
#[doc = "NVICIP235 register accessor: an alias for `Reg<NVICIP235_SPEC>`"]
pub type NVICIP235 = crate::Reg<nvicip235::NVICIP235_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip235;
#[doc = "NVICIP236 register accessor: an alias for `Reg<NVICIP236_SPEC>`"]
pub type NVICIP236 = crate::Reg<nvicip236::NVICIP236_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip236;
#[doc = "NVICIP237 register accessor: an alias for `Reg<NVICIP237_SPEC>`"]
pub type NVICIP237 = crate::Reg<nvicip237::NVICIP237_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip237;
#[doc = "NVICIP238 register accessor: an alias for `Reg<NVICIP238_SPEC>`"]
pub type NVICIP238 = crate::Reg<nvicip238::NVICIP238_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip238;
#[doc = "NVICIP239 register accessor: an alias for `Reg<NVICIP239_SPEC>`"]
pub type NVICIP239 = crate::Reg<nvicip239::NVICIP239_SPEC>;
#[doc = "Interrupt Priority Register n"]
pub mod nvicip239;
#[doc = "NVICSTIR register accessor: an alias for `Reg<NVICSTIR_SPEC>`"]
pub type NVICSTIR = crate::Reg<nvicstir::NVICSTIR_SPEC>;
#[doc = "Software Trigger Interrupt Register"]
pub mod nvicstir;
