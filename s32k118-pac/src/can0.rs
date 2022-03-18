#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Rx Mailboxes Global Mask Register"]
    pub rxmgmask: crate::Reg<rxmgmask::RXMGMASK_SPEC>,
    #[doc = "0x14 - Rx 14 Mask register"]
    pub rx14mask: crate::Reg<rx14mask::RX14MASK_SPEC>,
    #[doc = "0x18 - Rx 15 Mask register"]
    pub rx15mask: crate::Reg<rx15mask::RX15MASK_SPEC>,
    #[doc = "0x1c - Error Counter"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x20 - Error and Status 1 register"]
    pub esr1: crate::Reg<esr1::ESR1_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: crate::Reg<imask1::IMASK1_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: crate::Reg<iflag1::IFLAG1_SPEC>,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: crate::Reg<esr2::ESR2_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x44 - CRC Register"]
    pub crcr: crate::Reg<crcr::CRCR_SPEC>,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: crate::Reg<rxfgmask::RXFGMASK_SPEC>,
    #[doc = "0x4c - Rx FIFO Information Register"]
    pub rxfir: crate::Reg<rxfir::RXFIR_SPEC>,
    #[doc = "0x50 - CAN Bit Timing Register"]
    pub cbt: crate::Reg<cbt::CBT_SPEC>,
    _reserved16: [u8; 0x2c],
    #[doc = "0x80 - Embedded RAM"]
    pub ramn0: crate::Reg<ramn0::RAMN0_SPEC>,
    #[doc = "0x84 - Embedded RAM"]
    pub ramn1: crate::Reg<ramn1::RAMN1_SPEC>,
    #[doc = "0x88 - Embedded RAM"]
    pub ramn2: crate::Reg<ramn2::RAMN2_SPEC>,
    #[doc = "0x8c - Embedded RAM"]
    pub ramn3: crate::Reg<ramn3::RAMN3_SPEC>,
    #[doc = "0x90 - Embedded RAM"]
    pub ramn4: crate::Reg<ramn4::RAMN4_SPEC>,
    #[doc = "0x94 - Embedded RAM"]
    pub ramn5: crate::Reg<ramn5::RAMN5_SPEC>,
    #[doc = "0x98 - Embedded RAM"]
    pub ramn6: crate::Reg<ramn6::RAMN6_SPEC>,
    #[doc = "0x9c - Embedded RAM"]
    pub ramn7: crate::Reg<ramn7::RAMN7_SPEC>,
    #[doc = "0xa0 - Embedded RAM"]
    pub ramn8: crate::Reg<ramn8::RAMN8_SPEC>,
    #[doc = "0xa4 - Embedded RAM"]
    pub ramn9: crate::Reg<ramn9::RAMN9_SPEC>,
    #[doc = "0xa8 - Embedded RAM"]
    pub ramn10: crate::Reg<ramn10::RAMN10_SPEC>,
    #[doc = "0xac - Embedded RAM"]
    pub ramn11: crate::Reg<ramn11::RAMN11_SPEC>,
    #[doc = "0xb0 - Embedded RAM"]
    pub ramn12: crate::Reg<ramn12::RAMN12_SPEC>,
    #[doc = "0xb4 - Embedded RAM"]
    pub ramn13: crate::Reg<ramn13::RAMN13_SPEC>,
    #[doc = "0xb8 - Embedded RAM"]
    pub ramn14: crate::Reg<ramn14::RAMN14_SPEC>,
    #[doc = "0xbc - Embedded RAM"]
    pub ramn15: crate::Reg<ramn15::RAMN15_SPEC>,
    #[doc = "0xc0 - Embedded RAM"]
    pub ramn16: crate::Reg<ramn16::RAMN16_SPEC>,
    #[doc = "0xc4 - Embedded RAM"]
    pub ramn17: crate::Reg<ramn17::RAMN17_SPEC>,
    #[doc = "0xc8 - Embedded RAM"]
    pub ramn18: crate::Reg<ramn18::RAMN18_SPEC>,
    #[doc = "0xcc - Embedded RAM"]
    pub ramn19: crate::Reg<ramn19::RAMN19_SPEC>,
    #[doc = "0xd0 - Embedded RAM"]
    pub ramn20: crate::Reg<ramn20::RAMN20_SPEC>,
    #[doc = "0xd4 - Embedded RAM"]
    pub ramn21: crate::Reg<ramn21::RAMN21_SPEC>,
    #[doc = "0xd8 - Embedded RAM"]
    pub ramn22: crate::Reg<ramn22::RAMN22_SPEC>,
    #[doc = "0xdc - Embedded RAM"]
    pub ramn23: crate::Reg<ramn23::RAMN23_SPEC>,
    #[doc = "0xe0 - Embedded RAM"]
    pub ramn24: crate::Reg<ramn24::RAMN24_SPEC>,
    #[doc = "0xe4 - Embedded RAM"]
    pub ramn25: crate::Reg<ramn25::RAMN25_SPEC>,
    #[doc = "0xe8 - Embedded RAM"]
    pub ramn26: crate::Reg<ramn26::RAMN26_SPEC>,
    #[doc = "0xec - Embedded RAM"]
    pub ramn27: crate::Reg<ramn27::RAMN27_SPEC>,
    #[doc = "0xf0 - Embedded RAM"]
    pub ramn28: crate::Reg<ramn28::RAMN28_SPEC>,
    #[doc = "0xf4 - Embedded RAM"]
    pub ramn29: crate::Reg<ramn29::RAMN29_SPEC>,
    #[doc = "0xf8 - Embedded RAM"]
    pub ramn30: crate::Reg<ramn30::RAMN30_SPEC>,
    #[doc = "0xfc - Embedded RAM"]
    pub ramn31: crate::Reg<ramn31::RAMN31_SPEC>,
    #[doc = "0x100 - Embedded RAM"]
    pub ramn32: crate::Reg<ramn32::RAMN32_SPEC>,
    #[doc = "0x104 - Embedded RAM"]
    pub ramn33: crate::Reg<ramn33::RAMN33_SPEC>,
    #[doc = "0x108 - Embedded RAM"]
    pub ramn34: crate::Reg<ramn34::RAMN34_SPEC>,
    #[doc = "0x10c - Embedded RAM"]
    pub ramn35: crate::Reg<ramn35::RAMN35_SPEC>,
    #[doc = "0x110 - Embedded RAM"]
    pub ramn36: crate::Reg<ramn36::RAMN36_SPEC>,
    #[doc = "0x114 - Embedded RAM"]
    pub ramn37: crate::Reg<ramn37::RAMN37_SPEC>,
    #[doc = "0x118 - Embedded RAM"]
    pub ramn38: crate::Reg<ramn38::RAMN38_SPEC>,
    #[doc = "0x11c - Embedded RAM"]
    pub ramn39: crate::Reg<ramn39::RAMN39_SPEC>,
    #[doc = "0x120 - Embedded RAM"]
    pub ramn40: crate::Reg<ramn40::RAMN40_SPEC>,
    #[doc = "0x124 - Embedded RAM"]
    pub ramn41: crate::Reg<ramn41::RAMN41_SPEC>,
    #[doc = "0x128 - Embedded RAM"]
    pub ramn42: crate::Reg<ramn42::RAMN42_SPEC>,
    #[doc = "0x12c - Embedded RAM"]
    pub ramn43: crate::Reg<ramn43::RAMN43_SPEC>,
    #[doc = "0x130 - Embedded RAM"]
    pub ramn44: crate::Reg<ramn44::RAMN44_SPEC>,
    #[doc = "0x134 - Embedded RAM"]
    pub ramn45: crate::Reg<ramn45::RAMN45_SPEC>,
    #[doc = "0x138 - Embedded RAM"]
    pub ramn46: crate::Reg<ramn46::RAMN46_SPEC>,
    #[doc = "0x13c - Embedded RAM"]
    pub ramn47: crate::Reg<ramn47::RAMN47_SPEC>,
    #[doc = "0x140 - Embedded RAM"]
    pub ramn48: crate::Reg<ramn48::RAMN48_SPEC>,
    #[doc = "0x144 - Embedded RAM"]
    pub ramn49: crate::Reg<ramn49::RAMN49_SPEC>,
    #[doc = "0x148 - Embedded RAM"]
    pub ramn50: crate::Reg<ramn50::RAMN50_SPEC>,
    #[doc = "0x14c - Embedded RAM"]
    pub ramn51: crate::Reg<ramn51::RAMN51_SPEC>,
    #[doc = "0x150 - Embedded RAM"]
    pub ramn52: crate::Reg<ramn52::RAMN52_SPEC>,
    #[doc = "0x154 - Embedded RAM"]
    pub ramn53: crate::Reg<ramn53::RAMN53_SPEC>,
    #[doc = "0x158 - Embedded RAM"]
    pub ramn54: crate::Reg<ramn54::RAMN54_SPEC>,
    #[doc = "0x15c - Embedded RAM"]
    pub ramn55: crate::Reg<ramn55::RAMN55_SPEC>,
    #[doc = "0x160 - Embedded RAM"]
    pub ramn56: crate::Reg<ramn56::RAMN56_SPEC>,
    #[doc = "0x164 - Embedded RAM"]
    pub ramn57: crate::Reg<ramn57::RAMN57_SPEC>,
    #[doc = "0x168 - Embedded RAM"]
    pub ramn58: crate::Reg<ramn58::RAMN58_SPEC>,
    #[doc = "0x16c - Embedded RAM"]
    pub ramn59: crate::Reg<ramn59::RAMN59_SPEC>,
    #[doc = "0x170 - Embedded RAM"]
    pub ramn60: crate::Reg<ramn60::RAMN60_SPEC>,
    #[doc = "0x174 - Embedded RAM"]
    pub ramn61: crate::Reg<ramn61::RAMN61_SPEC>,
    #[doc = "0x178 - Embedded RAM"]
    pub ramn62: crate::Reg<ramn62::RAMN62_SPEC>,
    #[doc = "0x17c - Embedded RAM"]
    pub ramn63: crate::Reg<ramn63::RAMN63_SPEC>,
    #[doc = "0x180 - Embedded RAM"]
    pub ramn64: crate::Reg<ramn64::RAMN64_SPEC>,
    #[doc = "0x184 - Embedded RAM"]
    pub ramn65: crate::Reg<ramn65::RAMN65_SPEC>,
    #[doc = "0x188 - Embedded RAM"]
    pub ramn66: crate::Reg<ramn66::RAMN66_SPEC>,
    #[doc = "0x18c - Embedded RAM"]
    pub ramn67: crate::Reg<ramn67::RAMN67_SPEC>,
    #[doc = "0x190 - Embedded RAM"]
    pub ramn68: crate::Reg<ramn68::RAMN68_SPEC>,
    #[doc = "0x194 - Embedded RAM"]
    pub ramn69: crate::Reg<ramn69::RAMN69_SPEC>,
    #[doc = "0x198 - Embedded RAM"]
    pub ramn70: crate::Reg<ramn70::RAMN70_SPEC>,
    #[doc = "0x19c - Embedded RAM"]
    pub ramn71: crate::Reg<ramn71::RAMN71_SPEC>,
    #[doc = "0x1a0 - Embedded RAM"]
    pub ramn72: crate::Reg<ramn72::RAMN72_SPEC>,
    #[doc = "0x1a4 - Embedded RAM"]
    pub ramn73: crate::Reg<ramn73::RAMN73_SPEC>,
    #[doc = "0x1a8 - Embedded RAM"]
    pub ramn74: crate::Reg<ramn74::RAMN74_SPEC>,
    #[doc = "0x1ac - Embedded RAM"]
    pub ramn75: crate::Reg<ramn75::RAMN75_SPEC>,
    #[doc = "0x1b0 - Embedded RAM"]
    pub ramn76: crate::Reg<ramn76::RAMN76_SPEC>,
    #[doc = "0x1b4 - Embedded RAM"]
    pub ramn77: crate::Reg<ramn77::RAMN77_SPEC>,
    #[doc = "0x1b8 - Embedded RAM"]
    pub ramn78: crate::Reg<ramn78::RAMN78_SPEC>,
    #[doc = "0x1bc - Embedded RAM"]
    pub ramn79: crate::Reg<ramn79::RAMN79_SPEC>,
    #[doc = "0x1c0 - Embedded RAM"]
    pub ramn80: crate::Reg<ramn80::RAMN80_SPEC>,
    #[doc = "0x1c4 - Embedded RAM"]
    pub ramn81: crate::Reg<ramn81::RAMN81_SPEC>,
    #[doc = "0x1c8 - Embedded RAM"]
    pub ramn82: crate::Reg<ramn82::RAMN82_SPEC>,
    #[doc = "0x1cc - Embedded RAM"]
    pub ramn83: crate::Reg<ramn83::RAMN83_SPEC>,
    #[doc = "0x1d0 - Embedded RAM"]
    pub ramn84: crate::Reg<ramn84::RAMN84_SPEC>,
    #[doc = "0x1d4 - Embedded RAM"]
    pub ramn85: crate::Reg<ramn85::RAMN85_SPEC>,
    #[doc = "0x1d8 - Embedded RAM"]
    pub ramn86: crate::Reg<ramn86::RAMN86_SPEC>,
    #[doc = "0x1dc - Embedded RAM"]
    pub ramn87: crate::Reg<ramn87::RAMN87_SPEC>,
    #[doc = "0x1e0 - Embedded RAM"]
    pub ramn88: crate::Reg<ramn88::RAMN88_SPEC>,
    #[doc = "0x1e4 - Embedded RAM"]
    pub ramn89: crate::Reg<ramn89::RAMN89_SPEC>,
    #[doc = "0x1e8 - Embedded RAM"]
    pub ramn90: crate::Reg<ramn90::RAMN90_SPEC>,
    #[doc = "0x1ec - Embedded RAM"]
    pub ramn91: crate::Reg<ramn91::RAMN91_SPEC>,
    #[doc = "0x1f0 - Embedded RAM"]
    pub ramn92: crate::Reg<ramn92::RAMN92_SPEC>,
    #[doc = "0x1f4 - Embedded RAM"]
    pub ramn93: crate::Reg<ramn93::RAMN93_SPEC>,
    #[doc = "0x1f8 - Embedded RAM"]
    pub ramn94: crate::Reg<ramn94::RAMN94_SPEC>,
    #[doc = "0x1fc - Embedded RAM"]
    pub ramn95: crate::Reg<ramn95::RAMN95_SPEC>,
    #[doc = "0x200 - Embedded RAM"]
    pub ramn96: crate::Reg<ramn96::RAMN96_SPEC>,
    #[doc = "0x204 - Embedded RAM"]
    pub ramn97: crate::Reg<ramn97::RAMN97_SPEC>,
    #[doc = "0x208 - Embedded RAM"]
    pub ramn98: crate::Reg<ramn98::RAMN98_SPEC>,
    #[doc = "0x20c - Embedded RAM"]
    pub ramn99: crate::Reg<ramn99::RAMN99_SPEC>,
    #[doc = "0x210 - Embedded RAM"]
    pub ramn100: crate::Reg<ramn100::RAMN100_SPEC>,
    #[doc = "0x214 - Embedded RAM"]
    pub ramn101: crate::Reg<ramn101::RAMN101_SPEC>,
    #[doc = "0x218 - Embedded RAM"]
    pub ramn102: crate::Reg<ramn102::RAMN102_SPEC>,
    #[doc = "0x21c - Embedded RAM"]
    pub ramn103: crate::Reg<ramn103::RAMN103_SPEC>,
    #[doc = "0x220 - Embedded RAM"]
    pub ramn104: crate::Reg<ramn104::RAMN104_SPEC>,
    #[doc = "0x224 - Embedded RAM"]
    pub ramn105: crate::Reg<ramn105::RAMN105_SPEC>,
    #[doc = "0x228 - Embedded RAM"]
    pub ramn106: crate::Reg<ramn106::RAMN106_SPEC>,
    #[doc = "0x22c - Embedded RAM"]
    pub ramn107: crate::Reg<ramn107::RAMN107_SPEC>,
    #[doc = "0x230 - Embedded RAM"]
    pub ramn108: crate::Reg<ramn108::RAMN108_SPEC>,
    #[doc = "0x234 - Embedded RAM"]
    pub ramn109: crate::Reg<ramn109::RAMN109_SPEC>,
    #[doc = "0x238 - Embedded RAM"]
    pub ramn110: crate::Reg<ramn110::RAMN110_SPEC>,
    #[doc = "0x23c - Embedded RAM"]
    pub ramn111: crate::Reg<ramn111::RAMN111_SPEC>,
    #[doc = "0x240 - Embedded RAM"]
    pub ramn112: crate::Reg<ramn112::RAMN112_SPEC>,
    #[doc = "0x244 - Embedded RAM"]
    pub ramn113: crate::Reg<ramn113::RAMN113_SPEC>,
    #[doc = "0x248 - Embedded RAM"]
    pub ramn114: crate::Reg<ramn114::RAMN114_SPEC>,
    #[doc = "0x24c - Embedded RAM"]
    pub ramn115: crate::Reg<ramn115::RAMN115_SPEC>,
    #[doc = "0x250 - Embedded RAM"]
    pub ramn116: crate::Reg<ramn116::RAMN116_SPEC>,
    #[doc = "0x254 - Embedded RAM"]
    pub ramn117: crate::Reg<ramn117::RAMN117_SPEC>,
    #[doc = "0x258 - Embedded RAM"]
    pub ramn118: crate::Reg<ramn118::RAMN118_SPEC>,
    #[doc = "0x25c - Embedded RAM"]
    pub ramn119: crate::Reg<ramn119::RAMN119_SPEC>,
    #[doc = "0x260 - Embedded RAM"]
    pub ramn120: crate::Reg<ramn120::RAMN120_SPEC>,
    #[doc = "0x264 - Embedded RAM"]
    pub ramn121: crate::Reg<ramn121::RAMN121_SPEC>,
    #[doc = "0x268 - Embedded RAM"]
    pub ramn122: crate::Reg<ramn122::RAMN122_SPEC>,
    #[doc = "0x26c - Embedded RAM"]
    pub ramn123: crate::Reg<ramn123::RAMN123_SPEC>,
    #[doc = "0x270 - Embedded RAM"]
    pub ramn124: crate::Reg<ramn124::RAMN124_SPEC>,
    #[doc = "0x274 - Embedded RAM"]
    pub ramn125: crate::Reg<ramn125::RAMN125_SPEC>,
    #[doc = "0x278 - Embedded RAM"]
    pub ramn126: crate::Reg<ramn126::RAMN126_SPEC>,
    #[doc = "0x27c - Embedded RAM"]
    pub ramn127: crate::Reg<ramn127::RAMN127_SPEC>,
    _reserved144: [u8; 0x0600],
    #[doc = "0x880 - Rx Individual Mask Registers"]
    pub rximr0: crate::Reg<rximr0::RXIMR0_SPEC>,
    #[doc = "0x884 - Rx Individual Mask Registers"]
    pub rximr1: crate::Reg<rximr1::RXIMR1_SPEC>,
    #[doc = "0x888 - Rx Individual Mask Registers"]
    pub rximr2: crate::Reg<rximr2::RXIMR2_SPEC>,
    #[doc = "0x88c - Rx Individual Mask Registers"]
    pub rximr3: crate::Reg<rximr3::RXIMR3_SPEC>,
    #[doc = "0x890 - Rx Individual Mask Registers"]
    pub rximr4: crate::Reg<rximr4::RXIMR4_SPEC>,
    #[doc = "0x894 - Rx Individual Mask Registers"]
    pub rximr5: crate::Reg<rximr5::RXIMR5_SPEC>,
    #[doc = "0x898 - Rx Individual Mask Registers"]
    pub rximr6: crate::Reg<rximr6::RXIMR6_SPEC>,
    #[doc = "0x89c - Rx Individual Mask Registers"]
    pub rximr7: crate::Reg<rximr7::RXIMR7_SPEC>,
    #[doc = "0x8a0 - Rx Individual Mask Registers"]
    pub rximr8: crate::Reg<rximr8::RXIMR8_SPEC>,
    #[doc = "0x8a4 - Rx Individual Mask Registers"]
    pub rximr9: crate::Reg<rximr9::RXIMR9_SPEC>,
    #[doc = "0x8a8 - Rx Individual Mask Registers"]
    pub rximr10: crate::Reg<rximr10::RXIMR10_SPEC>,
    #[doc = "0x8ac - Rx Individual Mask Registers"]
    pub rximr11: crate::Reg<rximr11::RXIMR11_SPEC>,
    #[doc = "0x8b0 - Rx Individual Mask Registers"]
    pub rximr12: crate::Reg<rximr12::RXIMR12_SPEC>,
    #[doc = "0x8b4 - Rx Individual Mask Registers"]
    pub rximr13: crate::Reg<rximr13::RXIMR13_SPEC>,
    #[doc = "0x8b8 - Rx Individual Mask Registers"]
    pub rximr14: crate::Reg<rximr14::RXIMR14_SPEC>,
    #[doc = "0x8bc - Rx Individual Mask Registers"]
    pub rximr15: crate::Reg<rximr15::RXIMR15_SPEC>,
    #[doc = "0x8c0 - Rx Individual Mask Registers"]
    pub rximr16: crate::Reg<rximr16::RXIMR16_SPEC>,
    #[doc = "0x8c4 - Rx Individual Mask Registers"]
    pub rximr17: crate::Reg<rximr17::RXIMR17_SPEC>,
    #[doc = "0x8c8 - Rx Individual Mask Registers"]
    pub rximr18: crate::Reg<rximr18::RXIMR18_SPEC>,
    #[doc = "0x8cc - Rx Individual Mask Registers"]
    pub rximr19: crate::Reg<rximr19::RXIMR19_SPEC>,
    #[doc = "0x8d0 - Rx Individual Mask Registers"]
    pub rximr20: crate::Reg<rximr20::RXIMR20_SPEC>,
    #[doc = "0x8d4 - Rx Individual Mask Registers"]
    pub rximr21: crate::Reg<rximr21::RXIMR21_SPEC>,
    #[doc = "0x8d8 - Rx Individual Mask Registers"]
    pub rximr22: crate::Reg<rximr22::RXIMR22_SPEC>,
    #[doc = "0x8dc - Rx Individual Mask Registers"]
    pub rximr23: crate::Reg<rximr23::RXIMR23_SPEC>,
    #[doc = "0x8e0 - Rx Individual Mask Registers"]
    pub rximr24: crate::Reg<rximr24::RXIMR24_SPEC>,
    #[doc = "0x8e4 - Rx Individual Mask Registers"]
    pub rximr25: crate::Reg<rximr25::RXIMR25_SPEC>,
    #[doc = "0x8e8 - Rx Individual Mask Registers"]
    pub rximr26: crate::Reg<rximr26::RXIMR26_SPEC>,
    #[doc = "0x8ec - Rx Individual Mask Registers"]
    pub rximr27: crate::Reg<rximr27::RXIMR27_SPEC>,
    #[doc = "0x8f0 - Rx Individual Mask Registers"]
    pub rximr28: crate::Reg<rximr28::RXIMR28_SPEC>,
    #[doc = "0x8f4 - Rx Individual Mask Registers"]
    pub rximr29: crate::Reg<rximr29::RXIMR29_SPEC>,
    #[doc = "0x8f8 - Rx Individual Mask Registers"]
    pub rximr30: crate::Reg<rximr30::RXIMR30_SPEC>,
    #[doc = "0x8fc - Rx Individual Mask Registers"]
    pub rximr31: crate::Reg<rximr31::RXIMR31_SPEC>,
    _reserved176: [u8; 0x0200],
    #[doc = "0xb00 - Pretended Networking Control 1 Register"]
    pub ctrl1_pn: crate::Reg<ctrl1_pn::CTRL1_PN_SPEC>,
    #[doc = "0xb04 - Pretended Networking Control 2 Register"]
    pub ctrl2_pn: crate::Reg<ctrl2_pn::CTRL2_PN_SPEC>,
    #[doc = "0xb08 - Pretended Networking Wake Up Match Register"]
    pub wu_mtc: crate::Reg<wu_mtc::WU_MTC_SPEC>,
    #[doc = "0xb0c - Pretended Networking ID Filter 1 Register"]
    pub flt_id1: crate::Reg<flt_id1::FLT_ID1_SPEC>,
    #[doc = "0xb10 - Pretended Networking DLC Filter Register"]
    pub flt_dlc: crate::Reg<flt_dlc::FLT_DLC_SPEC>,
    #[doc = "0xb14 - Pretended Networking Payload Low Filter 1 Register"]
    pub pl1_lo: crate::Reg<pl1_lo::PL1_LO_SPEC>,
    #[doc = "0xb18 - Pretended Networking Payload High Filter 1 Register"]
    pub pl1_hi: crate::Reg<pl1_hi::PL1_HI_SPEC>,
    #[doc = "0xb1c - Pretended Networking ID Filter 2 Register / ID Mask Register"]
    pub flt_id2_idmask: crate::Reg<flt_id2_idmask::FLT_ID2_IDMASK_SPEC>,
    #[doc = "0xb20 - Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
    pub pl2_plmask_lo: crate::Reg<pl2_plmask_lo::PL2_PLMASK_LO_SPEC>,
    #[doc = "0xb24 - Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
    pub pl2_plmask_hi: crate::Reg<pl2_plmask_hi::PL2_PLMASK_HI_SPEC>,
    _reserved186: [u8; 0x18],
    #[doc = "0xb40 - Wake Up Message Buffer Register for C/S"]
    pub wmb0_cs: crate::Reg<wmb0_cs::WMB0_CS_SPEC>,
    #[doc = "0xb44 - Wake Up Message Buffer Register for ID"]
    pub wmb0_id: crate::Reg<wmb0_id::WMB0_ID_SPEC>,
    #[doc = "0xb48 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb0_d03: crate::Reg<wmb0_d03::WMB0_D03_SPEC>,
    #[doc = "0xb4c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb0_d47: crate::Reg<wmb0_d47::WMB0_D47_SPEC>,
    #[doc = "0xb50 - Wake Up Message Buffer Register for C/S"]
    pub wmb1_cs: crate::Reg<wmb1_cs::WMB1_CS_SPEC>,
    #[doc = "0xb54 - Wake Up Message Buffer Register for ID"]
    pub wmb1_id: crate::Reg<wmb1_id::WMB1_ID_SPEC>,
    #[doc = "0xb58 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb1_d03: crate::Reg<wmb1_d03::WMB1_D03_SPEC>,
    #[doc = "0xb5c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb1_d47: crate::Reg<wmb1_d47::WMB1_D47_SPEC>,
    #[doc = "0xb60 - Wake Up Message Buffer Register for C/S"]
    pub wmb2_cs: crate::Reg<wmb2_cs::WMB2_CS_SPEC>,
    #[doc = "0xb64 - Wake Up Message Buffer Register for ID"]
    pub wmb2_id: crate::Reg<wmb2_id::WMB2_ID_SPEC>,
    #[doc = "0xb68 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb2_d03: crate::Reg<wmb2_d03::WMB2_D03_SPEC>,
    #[doc = "0xb6c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb2_d47: crate::Reg<wmb2_d47::WMB2_D47_SPEC>,
    #[doc = "0xb70 - Wake Up Message Buffer Register for C/S"]
    pub wmb3_cs: crate::Reg<wmb3_cs::WMB3_CS_SPEC>,
    #[doc = "0xb74 - Wake Up Message Buffer Register for ID"]
    pub wmb3_id: crate::Reg<wmb3_id::WMB3_ID_SPEC>,
    #[doc = "0xb78 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmb3_d03: crate::Reg<wmb3_d03::WMB3_D03_SPEC>,
    #[doc = "0xb7c - Wake Up Message Buffer Register Data 4-7"]
    pub wmb3_d47: crate::Reg<wmb3_d47::WMB3_D47_SPEC>,
    _reserved202: [u8; 0x80],
    #[doc = "0xc00 - CAN FD Control Register"]
    pub fdctrl: crate::Reg<fdctrl::FDCTRL_SPEC>,
    #[doc = "0xc04 - CAN FD Bit Timing Register"]
    pub fdcbt: crate::Reg<fdcbt::FDCBT_SPEC>,
    #[doc = "0xc08 - CAN FD CRC Register"]
    pub fdcrc: crate::Reg<fdcrc::FDCRC_SPEC>,
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "RXMGMASK register accessor: an alias for `Reg<RXMGMASK_SPEC>`"]
pub type RXMGMASK = crate::Reg<rxmgmask::RXMGMASK_SPEC>;
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod rxmgmask;
#[doc = "RX14MASK register accessor: an alias for `Reg<RX14MASK_SPEC>`"]
pub type RX14MASK = crate::Reg<rx14mask::RX14MASK_SPEC>;
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "RX15MASK register accessor: an alias for `Reg<RX15MASK_SPEC>`"]
pub type RX15MASK = crate::Reg<rx15mask::RX15MASK_SPEC>;
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "ESR1 register accessor: an alias for `Reg<ESR1_SPEC>`"]
pub type ESR1 = crate::Reg<esr1::ESR1_SPEC>;
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "IMASK1 register accessor: an alias for `Reg<IMASK1_SPEC>`"]
pub type IMASK1 = crate::Reg<imask1::IMASK1_SPEC>;
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "IFLAG1 register accessor: an alias for `Reg<IFLAG1_SPEC>`"]
pub type IFLAG1 = crate::Reg<iflag1::IFLAG1_SPEC>;
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ESR2 register accessor: an alias for `Reg<ESR2_SPEC>`"]
pub type ESR2 = crate::Reg<esr2::ESR2_SPEC>;
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRCR register accessor: an alias for `Reg<CRCR_SPEC>`"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRC Register"]
pub mod crcr;
#[doc = "RXFGMASK register accessor: an alias for `Reg<RXFGMASK_SPEC>`"]
pub type RXFGMASK = crate::Reg<rxfgmask::RXFGMASK_SPEC>;
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "RXFIR register accessor: an alias for `Reg<RXFIR_SPEC>`"]
pub type RXFIR = crate::Reg<rxfir::RXFIR_SPEC>;
#[doc = "Rx FIFO Information Register"]
pub mod rxfir;
#[doc = "CBT register accessor: an alias for `Reg<CBT_SPEC>`"]
pub type CBT = crate::Reg<cbt::CBT_SPEC>;
#[doc = "CAN Bit Timing Register"]
pub mod cbt;
#[doc = "RAMn0 register accessor: an alias for `Reg<RAMN0_SPEC>`"]
pub type RAMN0 = crate::Reg<ramn0::RAMN0_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn0;
#[doc = "RAMn1 register accessor: an alias for `Reg<RAMN1_SPEC>`"]
pub type RAMN1 = crate::Reg<ramn1::RAMN1_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn1;
#[doc = "RAMn2 register accessor: an alias for `Reg<RAMN2_SPEC>`"]
pub type RAMN2 = crate::Reg<ramn2::RAMN2_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn2;
#[doc = "RAMn3 register accessor: an alias for `Reg<RAMN3_SPEC>`"]
pub type RAMN3 = crate::Reg<ramn3::RAMN3_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn3;
#[doc = "RAMn4 register accessor: an alias for `Reg<RAMN4_SPEC>`"]
pub type RAMN4 = crate::Reg<ramn4::RAMN4_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn4;
#[doc = "RAMn5 register accessor: an alias for `Reg<RAMN5_SPEC>`"]
pub type RAMN5 = crate::Reg<ramn5::RAMN5_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn5;
#[doc = "RAMn6 register accessor: an alias for `Reg<RAMN6_SPEC>`"]
pub type RAMN6 = crate::Reg<ramn6::RAMN6_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn6;
#[doc = "RAMn7 register accessor: an alias for `Reg<RAMN7_SPEC>`"]
pub type RAMN7 = crate::Reg<ramn7::RAMN7_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn7;
#[doc = "RAMn8 register accessor: an alias for `Reg<RAMN8_SPEC>`"]
pub type RAMN8 = crate::Reg<ramn8::RAMN8_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn8;
#[doc = "RAMn9 register accessor: an alias for `Reg<RAMN9_SPEC>`"]
pub type RAMN9 = crate::Reg<ramn9::RAMN9_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn9;
#[doc = "RAMn10 register accessor: an alias for `Reg<RAMN10_SPEC>`"]
pub type RAMN10 = crate::Reg<ramn10::RAMN10_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn10;
#[doc = "RAMn11 register accessor: an alias for `Reg<RAMN11_SPEC>`"]
pub type RAMN11 = crate::Reg<ramn11::RAMN11_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn11;
#[doc = "RAMn12 register accessor: an alias for `Reg<RAMN12_SPEC>`"]
pub type RAMN12 = crate::Reg<ramn12::RAMN12_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn12;
#[doc = "RAMn13 register accessor: an alias for `Reg<RAMN13_SPEC>`"]
pub type RAMN13 = crate::Reg<ramn13::RAMN13_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn13;
#[doc = "RAMn14 register accessor: an alias for `Reg<RAMN14_SPEC>`"]
pub type RAMN14 = crate::Reg<ramn14::RAMN14_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn14;
#[doc = "RAMn15 register accessor: an alias for `Reg<RAMN15_SPEC>`"]
pub type RAMN15 = crate::Reg<ramn15::RAMN15_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn15;
#[doc = "RAMn16 register accessor: an alias for `Reg<RAMN16_SPEC>`"]
pub type RAMN16 = crate::Reg<ramn16::RAMN16_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn16;
#[doc = "RAMn17 register accessor: an alias for `Reg<RAMN17_SPEC>`"]
pub type RAMN17 = crate::Reg<ramn17::RAMN17_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn17;
#[doc = "RAMn18 register accessor: an alias for `Reg<RAMN18_SPEC>`"]
pub type RAMN18 = crate::Reg<ramn18::RAMN18_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn18;
#[doc = "RAMn19 register accessor: an alias for `Reg<RAMN19_SPEC>`"]
pub type RAMN19 = crate::Reg<ramn19::RAMN19_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn19;
#[doc = "RAMn20 register accessor: an alias for `Reg<RAMN20_SPEC>`"]
pub type RAMN20 = crate::Reg<ramn20::RAMN20_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn20;
#[doc = "RAMn21 register accessor: an alias for `Reg<RAMN21_SPEC>`"]
pub type RAMN21 = crate::Reg<ramn21::RAMN21_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn21;
#[doc = "RAMn22 register accessor: an alias for `Reg<RAMN22_SPEC>`"]
pub type RAMN22 = crate::Reg<ramn22::RAMN22_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn22;
#[doc = "RAMn23 register accessor: an alias for `Reg<RAMN23_SPEC>`"]
pub type RAMN23 = crate::Reg<ramn23::RAMN23_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn23;
#[doc = "RAMn24 register accessor: an alias for `Reg<RAMN24_SPEC>`"]
pub type RAMN24 = crate::Reg<ramn24::RAMN24_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn24;
#[doc = "RAMn25 register accessor: an alias for `Reg<RAMN25_SPEC>`"]
pub type RAMN25 = crate::Reg<ramn25::RAMN25_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn25;
#[doc = "RAMn26 register accessor: an alias for `Reg<RAMN26_SPEC>`"]
pub type RAMN26 = crate::Reg<ramn26::RAMN26_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn26;
#[doc = "RAMn27 register accessor: an alias for `Reg<RAMN27_SPEC>`"]
pub type RAMN27 = crate::Reg<ramn27::RAMN27_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn27;
#[doc = "RAMn28 register accessor: an alias for `Reg<RAMN28_SPEC>`"]
pub type RAMN28 = crate::Reg<ramn28::RAMN28_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn28;
#[doc = "RAMn29 register accessor: an alias for `Reg<RAMN29_SPEC>`"]
pub type RAMN29 = crate::Reg<ramn29::RAMN29_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn29;
#[doc = "RAMn30 register accessor: an alias for `Reg<RAMN30_SPEC>`"]
pub type RAMN30 = crate::Reg<ramn30::RAMN30_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn30;
#[doc = "RAMn31 register accessor: an alias for `Reg<RAMN31_SPEC>`"]
pub type RAMN31 = crate::Reg<ramn31::RAMN31_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn31;
#[doc = "RAMn32 register accessor: an alias for `Reg<RAMN32_SPEC>`"]
pub type RAMN32 = crate::Reg<ramn32::RAMN32_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn32;
#[doc = "RAMn33 register accessor: an alias for `Reg<RAMN33_SPEC>`"]
pub type RAMN33 = crate::Reg<ramn33::RAMN33_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn33;
#[doc = "RAMn34 register accessor: an alias for `Reg<RAMN34_SPEC>`"]
pub type RAMN34 = crate::Reg<ramn34::RAMN34_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn34;
#[doc = "RAMn35 register accessor: an alias for `Reg<RAMN35_SPEC>`"]
pub type RAMN35 = crate::Reg<ramn35::RAMN35_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn35;
#[doc = "RAMn36 register accessor: an alias for `Reg<RAMN36_SPEC>`"]
pub type RAMN36 = crate::Reg<ramn36::RAMN36_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn36;
#[doc = "RAMn37 register accessor: an alias for `Reg<RAMN37_SPEC>`"]
pub type RAMN37 = crate::Reg<ramn37::RAMN37_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn37;
#[doc = "RAMn38 register accessor: an alias for `Reg<RAMN38_SPEC>`"]
pub type RAMN38 = crate::Reg<ramn38::RAMN38_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn38;
#[doc = "RAMn39 register accessor: an alias for `Reg<RAMN39_SPEC>`"]
pub type RAMN39 = crate::Reg<ramn39::RAMN39_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn39;
#[doc = "RAMn40 register accessor: an alias for `Reg<RAMN40_SPEC>`"]
pub type RAMN40 = crate::Reg<ramn40::RAMN40_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn40;
#[doc = "RAMn41 register accessor: an alias for `Reg<RAMN41_SPEC>`"]
pub type RAMN41 = crate::Reg<ramn41::RAMN41_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn41;
#[doc = "RAMn42 register accessor: an alias for `Reg<RAMN42_SPEC>`"]
pub type RAMN42 = crate::Reg<ramn42::RAMN42_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn42;
#[doc = "RAMn43 register accessor: an alias for `Reg<RAMN43_SPEC>`"]
pub type RAMN43 = crate::Reg<ramn43::RAMN43_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn43;
#[doc = "RAMn44 register accessor: an alias for `Reg<RAMN44_SPEC>`"]
pub type RAMN44 = crate::Reg<ramn44::RAMN44_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn44;
#[doc = "RAMn45 register accessor: an alias for `Reg<RAMN45_SPEC>`"]
pub type RAMN45 = crate::Reg<ramn45::RAMN45_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn45;
#[doc = "RAMn46 register accessor: an alias for `Reg<RAMN46_SPEC>`"]
pub type RAMN46 = crate::Reg<ramn46::RAMN46_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn46;
#[doc = "RAMn47 register accessor: an alias for `Reg<RAMN47_SPEC>`"]
pub type RAMN47 = crate::Reg<ramn47::RAMN47_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn47;
#[doc = "RAMn48 register accessor: an alias for `Reg<RAMN48_SPEC>`"]
pub type RAMN48 = crate::Reg<ramn48::RAMN48_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn48;
#[doc = "RAMn49 register accessor: an alias for `Reg<RAMN49_SPEC>`"]
pub type RAMN49 = crate::Reg<ramn49::RAMN49_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn49;
#[doc = "RAMn50 register accessor: an alias for `Reg<RAMN50_SPEC>`"]
pub type RAMN50 = crate::Reg<ramn50::RAMN50_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn50;
#[doc = "RAMn51 register accessor: an alias for `Reg<RAMN51_SPEC>`"]
pub type RAMN51 = crate::Reg<ramn51::RAMN51_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn51;
#[doc = "RAMn52 register accessor: an alias for `Reg<RAMN52_SPEC>`"]
pub type RAMN52 = crate::Reg<ramn52::RAMN52_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn52;
#[doc = "RAMn53 register accessor: an alias for `Reg<RAMN53_SPEC>`"]
pub type RAMN53 = crate::Reg<ramn53::RAMN53_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn53;
#[doc = "RAMn54 register accessor: an alias for `Reg<RAMN54_SPEC>`"]
pub type RAMN54 = crate::Reg<ramn54::RAMN54_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn54;
#[doc = "RAMn55 register accessor: an alias for `Reg<RAMN55_SPEC>`"]
pub type RAMN55 = crate::Reg<ramn55::RAMN55_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn55;
#[doc = "RAMn56 register accessor: an alias for `Reg<RAMN56_SPEC>`"]
pub type RAMN56 = crate::Reg<ramn56::RAMN56_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn56;
#[doc = "RAMn57 register accessor: an alias for `Reg<RAMN57_SPEC>`"]
pub type RAMN57 = crate::Reg<ramn57::RAMN57_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn57;
#[doc = "RAMn58 register accessor: an alias for `Reg<RAMN58_SPEC>`"]
pub type RAMN58 = crate::Reg<ramn58::RAMN58_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn58;
#[doc = "RAMn59 register accessor: an alias for `Reg<RAMN59_SPEC>`"]
pub type RAMN59 = crate::Reg<ramn59::RAMN59_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn59;
#[doc = "RAMn60 register accessor: an alias for `Reg<RAMN60_SPEC>`"]
pub type RAMN60 = crate::Reg<ramn60::RAMN60_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn60;
#[doc = "RAMn61 register accessor: an alias for `Reg<RAMN61_SPEC>`"]
pub type RAMN61 = crate::Reg<ramn61::RAMN61_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn61;
#[doc = "RAMn62 register accessor: an alias for `Reg<RAMN62_SPEC>`"]
pub type RAMN62 = crate::Reg<ramn62::RAMN62_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn62;
#[doc = "RAMn63 register accessor: an alias for `Reg<RAMN63_SPEC>`"]
pub type RAMN63 = crate::Reg<ramn63::RAMN63_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn63;
#[doc = "RAMn64 register accessor: an alias for `Reg<RAMN64_SPEC>`"]
pub type RAMN64 = crate::Reg<ramn64::RAMN64_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn64;
#[doc = "RAMn65 register accessor: an alias for `Reg<RAMN65_SPEC>`"]
pub type RAMN65 = crate::Reg<ramn65::RAMN65_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn65;
#[doc = "RAMn66 register accessor: an alias for `Reg<RAMN66_SPEC>`"]
pub type RAMN66 = crate::Reg<ramn66::RAMN66_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn66;
#[doc = "RAMn67 register accessor: an alias for `Reg<RAMN67_SPEC>`"]
pub type RAMN67 = crate::Reg<ramn67::RAMN67_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn67;
#[doc = "RAMn68 register accessor: an alias for `Reg<RAMN68_SPEC>`"]
pub type RAMN68 = crate::Reg<ramn68::RAMN68_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn68;
#[doc = "RAMn69 register accessor: an alias for `Reg<RAMN69_SPEC>`"]
pub type RAMN69 = crate::Reg<ramn69::RAMN69_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn69;
#[doc = "RAMn70 register accessor: an alias for `Reg<RAMN70_SPEC>`"]
pub type RAMN70 = crate::Reg<ramn70::RAMN70_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn70;
#[doc = "RAMn71 register accessor: an alias for `Reg<RAMN71_SPEC>`"]
pub type RAMN71 = crate::Reg<ramn71::RAMN71_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn71;
#[doc = "RAMn72 register accessor: an alias for `Reg<RAMN72_SPEC>`"]
pub type RAMN72 = crate::Reg<ramn72::RAMN72_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn72;
#[doc = "RAMn73 register accessor: an alias for `Reg<RAMN73_SPEC>`"]
pub type RAMN73 = crate::Reg<ramn73::RAMN73_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn73;
#[doc = "RAMn74 register accessor: an alias for `Reg<RAMN74_SPEC>`"]
pub type RAMN74 = crate::Reg<ramn74::RAMN74_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn74;
#[doc = "RAMn75 register accessor: an alias for `Reg<RAMN75_SPEC>`"]
pub type RAMN75 = crate::Reg<ramn75::RAMN75_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn75;
#[doc = "RAMn76 register accessor: an alias for `Reg<RAMN76_SPEC>`"]
pub type RAMN76 = crate::Reg<ramn76::RAMN76_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn76;
#[doc = "RAMn77 register accessor: an alias for `Reg<RAMN77_SPEC>`"]
pub type RAMN77 = crate::Reg<ramn77::RAMN77_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn77;
#[doc = "RAMn78 register accessor: an alias for `Reg<RAMN78_SPEC>`"]
pub type RAMN78 = crate::Reg<ramn78::RAMN78_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn78;
#[doc = "RAMn79 register accessor: an alias for `Reg<RAMN79_SPEC>`"]
pub type RAMN79 = crate::Reg<ramn79::RAMN79_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn79;
#[doc = "RAMn80 register accessor: an alias for `Reg<RAMN80_SPEC>`"]
pub type RAMN80 = crate::Reg<ramn80::RAMN80_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn80;
#[doc = "RAMn81 register accessor: an alias for `Reg<RAMN81_SPEC>`"]
pub type RAMN81 = crate::Reg<ramn81::RAMN81_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn81;
#[doc = "RAMn82 register accessor: an alias for `Reg<RAMN82_SPEC>`"]
pub type RAMN82 = crate::Reg<ramn82::RAMN82_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn82;
#[doc = "RAMn83 register accessor: an alias for `Reg<RAMN83_SPEC>`"]
pub type RAMN83 = crate::Reg<ramn83::RAMN83_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn83;
#[doc = "RAMn84 register accessor: an alias for `Reg<RAMN84_SPEC>`"]
pub type RAMN84 = crate::Reg<ramn84::RAMN84_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn84;
#[doc = "RAMn85 register accessor: an alias for `Reg<RAMN85_SPEC>`"]
pub type RAMN85 = crate::Reg<ramn85::RAMN85_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn85;
#[doc = "RAMn86 register accessor: an alias for `Reg<RAMN86_SPEC>`"]
pub type RAMN86 = crate::Reg<ramn86::RAMN86_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn86;
#[doc = "RAMn87 register accessor: an alias for `Reg<RAMN87_SPEC>`"]
pub type RAMN87 = crate::Reg<ramn87::RAMN87_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn87;
#[doc = "RAMn88 register accessor: an alias for `Reg<RAMN88_SPEC>`"]
pub type RAMN88 = crate::Reg<ramn88::RAMN88_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn88;
#[doc = "RAMn89 register accessor: an alias for `Reg<RAMN89_SPEC>`"]
pub type RAMN89 = crate::Reg<ramn89::RAMN89_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn89;
#[doc = "RAMn90 register accessor: an alias for `Reg<RAMN90_SPEC>`"]
pub type RAMN90 = crate::Reg<ramn90::RAMN90_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn90;
#[doc = "RAMn91 register accessor: an alias for `Reg<RAMN91_SPEC>`"]
pub type RAMN91 = crate::Reg<ramn91::RAMN91_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn91;
#[doc = "RAMn92 register accessor: an alias for `Reg<RAMN92_SPEC>`"]
pub type RAMN92 = crate::Reg<ramn92::RAMN92_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn92;
#[doc = "RAMn93 register accessor: an alias for `Reg<RAMN93_SPEC>`"]
pub type RAMN93 = crate::Reg<ramn93::RAMN93_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn93;
#[doc = "RAMn94 register accessor: an alias for `Reg<RAMN94_SPEC>`"]
pub type RAMN94 = crate::Reg<ramn94::RAMN94_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn94;
#[doc = "RAMn95 register accessor: an alias for `Reg<RAMN95_SPEC>`"]
pub type RAMN95 = crate::Reg<ramn95::RAMN95_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn95;
#[doc = "RAMn96 register accessor: an alias for `Reg<RAMN96_SPEC>`"]
pub type RAMN96 = crate::Reg<ramn96::RAMN96_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn96;
#[doc = "RAMn97 register accessor: an alias for `Reg<RAMN97_SPEC>`"]
pub type RAMN97 = crate::Reg<ramn97::RAMN97_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn97;
#[doc = "RAMn98 register accessor: an alias for `Reg<RAMN98_SPEC>`"]
pub type RAMN98 = crate::Reg<ramn98::RAMN98_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn98;
#[doc = "RAMn99 register accessor: an alias for `Reg<RAMN99_SPEC>`"]
pub type RAMN99 = crate::Reg<ramn99::RAMN99_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn99;
#[doc = "RAMn100 register accessor: an alias for `Reg<RAMN100_SPEC>`"]
pub type RAMN100 = crate::Reg<ramn100::RAMN100_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn100;
#[doc = "RAMn101 register accessor: an alias for `Reg<RAMN101_SPEC>`"]
pub type RAMN101 = crate::Reg<ramn101::RAMN101_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn101;
#[doc = "RAMn102 register accessor: an alias for `Reg<RAMN102_SPEC>`"]
pub type RAMN102 = crate::Reg<ramn102::RAMN102_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn102;
#[doc = "RAMn103 register accessor: an alias for `Reg<RAMN103_SPEC>`"]
pub type RAMN103 = crate::Reg<ramn103::RAMN103_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn103;
#[doc = "RAMn104 register accessor: an alias for `Reg<RAMN104_SPEC>`"]
pub type RAMN104 = crate::Reg<ramn104::RAMN104_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn104;
#[doc = "RAMn105 register accessor: an alias for `Reg<RAMN105_SPEC>`"]
pub type RAMN105 = crate::Reg<ramn105::RAMN105_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn105;
#[doc = "RAMn106 register accessor: an alias for `Reg<RAMN106_SPEC>`"]
pub type RAMN106 = crate::Reg<ramn106::RAMN106_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn106;
#[doc = "RAMn107 register accessor: an alias for `Reg<RAMN107_SPEC>`"]
pub type RAMN107 = crate::Reg<ramn107::RAMN107_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn107;
#[doc = "RAMn108 register accessor: an alias for `Reg<RAMN108_SPEC>`"]
pub type RAMN108 = crate::Reg<ramn108::RAMN108_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn108;
#[doc = "RAMn109 register accessor: an alias for `Reg<RAMN109_SPEC>`"]
pub type RAMN109 = crate::Reg<ramn109::RAMN109_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn109;
#[doc = "RAMn110 register accessor: an alias for `Reg<RAMN110_SPEC>`"]
pub type RAMN110 = crate::Reg<ramn110::RAMN110_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn110;
#[doc = "RAMn111 register accessor: an alias for `Reg<RAMN111_SPEC>`"]
pub type RAMN111 = crate::Reg<ramn111::RAMN111_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn111;
#[doc = "RAMn112 register accessor: an alias for `Reg<RAMN112_SPEC>`"]
pub type RAMN112 = crate::Reg<ramn112::RAMN112_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn112;
#[doc = "RAMn113 register accessor: an alias for `Reg<RAMN113_SPEC>`"]
pub type RAMN113 = crate::Reg<ramn113::RAMN113_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn113;
#[doc = "RAMn114 register accessor: an alias for `Reg<RAMN114_SPEC>`"]
pub type RAMN114 = crate::Reg<ramn114::RAMN114_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn114;
#[doc = "RAMn115 register accessor: an alias for `Reg<RAMN115_SPEC>`"]
pub type RAMN115 = crate::Reg<ramn115::RAMN115_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn115;
#[doc = "RAMn116 register accessor: an alias for `Reg<RAMN116_SPEC>`"]
pub type RAMN116 = crate::Reg<ramn116::RAMN116_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn116;
#[doc = "RAMn117 register accessor: an alias for `Reg<RAMN117_SPEC>`"]
pub type RAMN117 = crate::Reg<ramn117::RAMN117_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn117;
#[doc = "RAMn118 register accessor: an alias for `Reg<RAMN118_SPEC>`"]
pub type RAMN118 = crate::Reg<ramn118::RAMN118_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn118;
#[doc = "RAMn119 register accessor: an alias for `Reg<RAMN119_SPEC>`"]
pub type RAMN119 = crate::Reg<ramn119::RAMN119_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn119;
#[doc = "RAMn120 register accessor: an alias for `Reg<RAMN120_SPEC>`"]
pub type RAMN120 = crate::Reg<ramn120::RAMN120_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn120;
#[doc = "RAMn121 register accessor: an alias for `Reg<RAMN121_SPEC>`"]
pub type RAMN121 = crate::Reg<ramn121::RAMN121_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn121;
#[doc = "RAMn122 register accessor: an alias for `Reg<RAMN122_SPEC>`"]
pub type RAMN122 = crate::Reg<ramn122::RAMN122_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn122;
#[doc = "RAMn123 register accessor: an alias for `Reg<RAMN123_SPEC>`"]
pub type RAMN123 = crate::Reg<ramn123::RAMN123_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn123;
#[doc = "RAMn124 register accessor: an alias for `Reg<RAMN124_SPEC>`"]
pub type RAMN124 = crate::Reg<ramn124::RAMN124_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn124;
#[doc = "RAMn125 register accessor: an alias for `Reg<RAMN125_SPEC>`"]
pub type RAMN125 = crate::Reg<ramn125::RAMN125_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn125;
#[doc = "RAMn126 register accessor: an alias for `Reg<RAMN126_SPEC>`"]
pub type RAMN126 = crate::Reg<ramn126::RAMN126_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn126;
#[doc = "RAMn127 register accessor: an alias for `Reg<RAMN127_SPEC>`"]
pub type RAMN127 = crate::Reg<ramn127::RAMN127_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn127;
#[doc = "RXIMR0 register accessor: an alias for `Reg<RXIMR0_SPEC>`"]
pub type RXIMR0 = crate::Reg<rximr0::RXIMR0_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr0;
#[doc = "RXIMR1 register accessor: an alias for `Reg<RXIMR1_SPEC>`"]
pub type RXIMR1 = crate::Reg<rximr1::RXIMR1_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr1;
#[doc = "RXIMR2 register accessor: an alias for `Reg<RXIMR2_SPEC>`"]
pub type RXIMR2 = crate::Reg<rximr2::RXIMR2_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr2;
#[doc = "RXIMR3 register accessor: an alias for `Reg<RXIMR3_SPEC>`"]
pub type RXIMR3 = crate::Reg<rximr3::RXIMR3_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr3;
#[doc = "RXIMR4 register accessor: an alias for `Reg<RXIMR4_SPEC>`"]
pub type RXIMR4 = crate::Reg<rximr4::RXIMR4_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr4;
#[doc = "RXIMR5 register accessor: an alias for `Reg<RXIMR5_SPEC>`"]
pub type RXIMR5 = crate::Reg<rximr5::RXIMR5_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr5;
#[doc = "RXIMR6 register accessor: an alias for `Reg<RXIMR6_SPEC>`"]
pub type RXIMR6 = crate::Reg<rximr6::RXIMR6_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr6;
#[doc = "RXIMR7 register accessor: an alias for `Reg<RXIMR7_SPEC>`"]
pub type RXIMR7 = crate::Reg<rximr7::RXIMR7_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr7;
#[doc = "RXIMR8 register accessor: an alias for `Reg<RXIMR8_SPEC>`"]
pub type RXIMR8 = crate::Reg<rximr8::RXIMR8_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr8;
#[doc = "RXIMR9 register accessor: an alias for `Reg<RXIMR9_SPEC>`"]
pub type RXIMR9 = crate::Reg<rximr9::RXIMR9_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr9;
#[doc = "RXIMR10 register accessor: an alias for `Reg<RXIMR10_SPEC>`"]
pub type RXIMR10 = crate::Reg<rximr10::RXIMR10_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr10;
#[doc = "RXIMR11 register accessor: an alias for `Reg<RXIMR11_SPEC>`"]
pub type RXIMR11 = crate::Reg<rximr11::RXIMR11_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr11;
#[doc = "RXIMR12 register accessor: an alias for `Reg<RXIMR12_SPEC>`"]
pub type RXIMR12 = crate::Reg<rximr12::RXIMR12_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr12;
#[doc = "RXIMR13 register accessor: an alias for `Reg<RXIMR13_SPEC>`"]
pub type RXIMR13 = crate::Reg<rximr13::RXIMR13_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr13;
#[doc = "RXIMR14 register accessor: an alias for `Reg<RXIMR14_SPEC>`"]
pub type RXIMR14 = crate::Reg<rximr14::RXIMR14_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr14;
#[doc = "RXIMR15 register accessor: an alias for `Reg<RXIMR15_SPEC>`"]
pub type RXIMR15 = crate::Reg<rximr15::RXIMR15_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr15;
#[doc = "RXIMR16 register accessor: an alias for `Reg<RXIMR16_SPEC>`"]
pub type RXIMR16 = crate::Reg<rximr16::RXIMR16_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr16;
#[doc = "RXIMR17 register accessor: an alias for `Reg<RXIMR17_SPEC>`"]
pub type RXIMR17 = crate::Reg<rximr17::RXIMR17_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr17;
#[doc = "RXIMR18 register accessor: an alias for `Reg<RXIMR18_SPEC>`"]
pub type RXIMR18 = crate::Reg<rximr18::RXIMR18_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr18;
#[doc = "RXIMR19 register accessor: an alias for `Reg<RXIMR19_SPEC>`"]
pub type RXIMR19 = crate::Reg<rximr19::RXIMR19_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr19;
#[doc = "RXIMR20 register accessor: an alias for `Reg<RXIMR20_SPEC>`"]
pub type RXIMR20 = crate::Reg<rximr20::RXIMR20_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr20;
#[doc = "RXIMR21 register accessor: an alias for `Reg<RXIMR21_SPEC>`"]
pub type RXIMR21 = crate::Reg<rximr21::RXIMR21_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr21;
#[doc = "RXIMR22 register accessor: an alias for `Reg<RXIMR22_SPEC>`"]
pub type RXIMR22 = crate::Reg<rximr22::RXIMR22_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr22;
#[doc = "RXIMR23 register accessor: an alias for `Reg<RXIMR23_SPEC>`"]
pub type RXIMR23 = crate::Reg<rximr23::RXIMR23_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr23;
#[doc = "RXIMR24 register accessor: an alias for `Reg<RXIMR24_SPEC>`"]
pub type RXIMR24 = crate::Reg<rximr24::RXIMR24_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr24;
#[doc = "RXIMR25 register accessor: an alias for `Reg<RXIMR25_SPEC>`"]
pub type RXIMR25 = crate::Reg<rximr25::RXIMR25_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr25;
#[doc = "RXIMR26 register accessor: an alias for `Reg<RXIMR26_SPEC>`"]
pub type RXIMR26 = crate::Reg<rximr26::RXIMR26_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr26;
#[doc = "RXIMR27 register accessor: an alias for `Reg<RXIMR27_SPEC>`"]
pub type RXIMR27 = crate::Reg<rximr27::RXIMR27_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr27;
#[doc = "RXIMR28 register accessor: an alias for `Reg<RXIMR28_SPEC>`"]
pub type RXIMR28 = crate::Reg<rximr28::RXIMR28_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr28;
#[doc = "RXIMR29 register accessor: an alias for `Reg<RXIMR29_SPEC>`"]
pub type RXIMR29 = crate::Reg<rximr29::RXIMR29_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr29;
#[doc = "RXIMR30 register accessor: an alias for `Reg<RXIMR30_SPEC>`"]
pub type RXIMR30 = crate::Reg<rximr30::RXIMR30_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr30;
#[doc = "RXIMR31 register accessor: an alias for `Reg<RXIMR31_SPEC>`"]
pub type RXIMR31 = crate::Reg<rximr31::RXIMR31_SPEC>;
#[doc = "Rx Individual Mask Registers"]
pub mod rximr31;
#[doc = "CTRL1_PN register accessor: an alias for `Reg<CTRL1_PN_SPEC>`"]
pub type CTRL1_PN = crate::Reg<ctrl1_pn::CTRL1_PN_SPEC>;
#[doc = "Pretended Networking Control 1 Register"]
pub mod ctrl1_pn;
#[doc = "CTRL2_PN register accessor: an alias for `Reg<CTRL2_PN_SPEC>`"]
pub type CTRL2_PN = crate::Reg<ctrl2_pn::CTRL2_PN_SPEC>;
#[doc = "Pretended Networking Control 2 Register"]
pub mod ctrl2_pn;
#[doc = "WU_MTC register accessor: an alias for `Reg<WU_MTC_SPEC>`"]
pub type WU_MTC = crate::Reg<wu_mtc::WU_MTC_SPEC>;
#[doc = "Pretended Networking Wake Up Match Register"]
pub mod wu_mtc;
#[doc = "FLT_ID1 register accessor: an alias for `Reg<FLT_ID1_SPEC>`"]
pub type FLT_ID1 = crate::Reg<flt_id1::FLT_ID1_SPEC>;
#[doc = "Pretended Networking ID Filter 1 Register"]
pub mod flt_id1;
#[doc = "FLT_DLC register accessor: an alias for `Reg<FLT_DLC_SPEC>`"]
pub type FLT_DLC = crate::Reg<flt_dlc::FLT_DLC_SPEC>;
#[doc = "Pretended Networking DLC Filter Register"]
pub mod flt_dlc;
#[doc = "PL1_LO register accessor: an alias for `Reg<PL1_LO_SPEC>`"]
pub type PL1_LO = crate::Reg<pl1_lo::PL1_LO_SPEC>;
#[doc = "Pretended Networking Payload Low Filter 1 Register"]
pub mod pl1_lo;
#[doc = "PL1_HI register accessor: an alias for `Reg<PL1_HI_SPEC>`"]
pub type PL1_HI = crate::Reg<pl1_hi::PL1_HI_SPEC>;
#[doc = "Pretended Networking Payload High Filter 1 Register"]
pub mod pl1_hi;
#[doc = "FLT_ID2_IDMASK register accessor: an alias for `Reg<FLT_ID2_IDMASK_SPEC>`"]
pub type FLT_ID2_IDMASK = crate::Reg<flt_id2_idmask::FLT_ID2_IDMASK_SPEC>;
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
pub mod flt_id2_idmask;
#[doc = "PL2_PLMASK_LO register accessor: an alias for `Reg<PL2_PLMASK_LO_SPEC>`"]
pub type PL2_PLMASK_LO = crate::Reg<pl2_plmask_lo::PL2_PLMASK_LO_SPEC>;
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask Register"]
pub mod pl2_plmask_lo;
#[doc = "PL2_PLMASK_HI register accessor: an alias for `Reg<PL2_PLMASK_HI_SPEC>`"]
pub type PL2_PLMASK_HI = crate::Reg<pl2_plmask_hi::PL2_PLMASK_HI_SPEC>;
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask Register"]
pub mod pl2_plmask_hi;
#[doc = "WMB0_CS register accessor: an alias for `Reg<WMB0_CS_SPEC>`"]
pub type WMB0_CS = crate::Reg<wmb0_cs::WMB0_CS_SPEC>;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb0_cs;
#[doc = "WMB0_ID register accessor: an alias for `Reg<WMB0_ID_SPEC>`"]
pub type WMB0_ID = crate::Reg<wmb0_id::WMB0_ID_SPEC>;
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb0_id;
#[doc = "WMB0_D03 register accessor: an alias for `Reg<WMB0_D03_SPEC>`"]
pub type WMB0_D03 = crate::Reg<wmb0_d03::WMB0_D03_SPEC>;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb0_d03;
#[doc = "WMB0_D47 register accessor: an alias for `Reg<WMB0_D47_SPEC>`"]
pub type WMB0_D47 = crate::Reg<wmb0_d47::WMB0_D47_SPEC>;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb0_d47;
#[doc = "WMB1_CS register accessor: an alias for `Reg<WMB1_CS_SPEC>`"]
pub type WMB1_CS = crate::Reg<wmb1_cs::WMB1_CS_SPEC>;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb1_cs;
#[doc = "WMB1_ID register accessor: an alias for `Reg<WMB1_ID_SPEC>`"]
pub type WMB1_ID = crate::Reg<wmb1_id::WMB1_ID_SPEC>;
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb1_id;
#[doc = "WMB1_D03 register accessor: an alias for `Reg<WMB1_D03_SPEC>`"]
pub type WMB1_D03 = crate::Reg<wmb1_d03::WMB1_D03_SPEC>;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb1_d03;
#[doc = "WMB1_D47 register accessor: an alias for `Reg<WMB1_D47_SPEC>`"]
pub type WMB1_D47 = crate::Reg<wmb1_d47::WMB1_D47_SPEC>;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb1_d47;
#[doc = "WMB2_CS register accessor: an alias for `Reg<WMB2_CS_SPEC>`"]
pub type WMB2_CS = crate::Reg<wmb2_cs::WMB2_CS_SPEC>;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb2_cs;
#[doc = "WMB2_ID register accessor: an alias for `Reg<WMB2_ID_SPEC>`"]
pub type WMB2_ID = crate::Reg<wmb2_id::WMB2_ID_SPEC>;
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb2_id;
#[doc = "WMB2_D03 register accessor: an alias for `Reg<WMB2_D03_SPEC>`"]
pub type WMB2_D03 = crate::Reg<wmb2_d03::WMB2_D03_SPEC>;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb2_d03;
#[doc = "WMB2_D47 register accessor: an alias for `Reg<WMB2_D47_SPEC>`"]
pub type WMB2_D47 = crate::Reg<wmb2_d47::WMB2_D47_SPEC>;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb2_d47;
#[doc = "WMB3_CS register accessor: an alias for `Reg<WMB3_CS_SPEC>`"]
pub type WMB3_CS = crate::Reg<wmb3_cs::WMB3_CS_SPEC>;
#[doc = "Wake Up Message Buffer Register for C/S"]
pub mod wmb3_cs;
#[doc = "WMB3_ID register accessor: an alias for `Reg<WMB3_ID_SPEC>`"]
pub type WMB3_ID = crate::Reg<wmb3_id::WMB3_ID_SPEC>;
#[doc = "Wake Up Message Buffer Register for ID"]
pub mod wmb3_id;
#[doc = "WMB3_D03 register accessor: an alias for `Reg<WMB3_D03_SPEC>`"]
pub type WMB3_D03 = crate::Reg<wmb3_d03::WMB3_D03_SPEC>;
#[doc = "Wake Up Message Buffer Register for Data 0-3"]
pub mod wmb3_d03;
#[doc = "WMB3_D47 register accessor: an alias for `Reg<WMB3_D47_SPEC>`"]
pub type WMB3_D47 = crate::Reg<wmb3_d47::WMB3_D47_SPEC>;
#[doc = "Wake Up Message Buffer Register Data 4-7"]
pub mod wmb3_d47;
#[doc = "FDCTRL register accessor: an alias for `Reg<FDCTRL_SPEC>`"]
pub type FDCTRL = crate::Reg<fdctrl::FDCTRL_SPEC>;
#[doc = "CAN FD Control Register"]
pub mod fdctrl;
#[doc = "FDCBT register accessor: an alias for `Reg<FDCBT_SPEC>`"]
pub type FDCBT = crate::Reg<fdcbt::FDCBT_SPEC>;
#[doc = "CAN FD Bit Timing Register"]
pub mod fdcbt;
#[doc = "FDCRC register accessor: an alias for `Reg<FDCRC_SPEC>`"]
pub type FDCRC = crate::Reg<fdcrc::FDCRC_SPEC>;
#[doc = "CAN FD CRC Register"]
pub mod fdcrc;
