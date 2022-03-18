#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Control Register n"]
    pub pcr0: crate::Reg<pcr0::PCR0_SPEC>,
    #[doc = "0x04 - Pin Control Register n"]
    pub pcr1: crate::Reg<pcr1::PCR1_SPEC>,
    #[doc = "0x08 - Pin Control Register n"]
    pub pcr2: crate::Reg<pcr2::PCR2_SPEC>,
    #[doc = "0x0c - Pin Control Register n"]
    pub pcr3: crate::Reg<pcr3::PCR3_SPEC>,
    #[doc = "0x10 - Pin Control Register n"]
    pub pcr4: crate::Reg<pcr4::PCR4_SPEC>,
    #[doc = "0x14 - Pin Control Register n"]
    pub pcr5: crate::Reg<pcr5::PCR5_SPEC>,
    #[doc = "0x18 - Pin Control Register n"]
    pub pcr6: crate::Reg<pcr6::PCR6_SPEC>,
    #[doc = "0x1c - Pin Control Register n"]
    pub pcr7: crate::Reg<pcr7::PCR7_SPEC>,
    #[doc = "0x20 - Pin Control Register n"]
    pub pcr8: crate::Reg<pcr8::PCR8_SPEC>,
    #[doc = "0x24 - Pin Control Register n"]
    pub pcr9: crate::Reg<pcr9::PCR9_SPEC>,
    #[doc = "0x28 - Pin Control Register n"]
    pub pcr10: crate::Reg<pcr10::PCR10_SPEC>,
    #[doc = "0x2c - Pin Control Register n"]
    pub pcr11: crate::Reg<pcr11::PCR11_SPEC>,
    #[doc = "0x30 - Pin Control Register n"]
    pub pcr12: crate::Reg<pcr12::PCR12_SPEC>,
    #[doc = "0x34 - Pin Control Register n"]
    pub pcr13: crate::Reg<pcr13::PCR13_SPEC>,
    #[doc = "0x38 - Pin Control Register n"]
    pub pcr14: crate::Reg<pcr14::PCR14_SPEC>,
    #[doc = "0x3c - Pin Control Register n"]
    pub pcr15: crate::Reg<pcr15::PCR15_SPEC>,
    #[doc = "0x40 - Pin Control Register n"]
    pub pcr16: crate::Reg<pcr16::PCR16_SPEC>,
    #[doc = "0x44 - Pin Control Register n"]
    pub pcr17: crate::Reg<pcr17::PCR17_SPEC>,
    #[doc = "0x48 - Pin Control Register n"]
    pub pcr18: crate::Reg<pcr18::PCR18_SPEC>,
    #[doc = "0x4c - Pin Control Register n"]
    pub pcr19: crate::Reg<pcr19::PCR19_SPEC>,
    #[doc = "0x50 - Pin Control Register n"]
    pub pcr20: crate::Reg<pcr20::PCR20_SPEC>,
    #[doc = "0x54 - Pin Control Register n"]
    pub pcr21: crate::Reg<pcr21::PCR21_SPEC>,
    #[doc = "0x58 - Pin Control Register n"]
    pub pcr22: crate::Reg<pcr22::PCR22_SPEC>,
    #[doc = "0x5c - Pin Control Register n"]
    pub pcr23: crate::Reg<pcr23::PCR23_SPEC>,
    #[doc = "0x60 - Pin Control Register n"]
    pub pcr24: crate::Reg<pcr24::PCR24_SPEC>,
    #[doc = "0x64 - Pin Control Register n"]
    pub pcr25: crate::Reg<pcr25::PCR25_SPEC>,
    #[doc = "0x68 - Pin Control Register n"]
    pub pcr26: crate::Reg<pcr26::PCR26_SPEC>,
    #[doc = "0x6c - Pin Control Register n"]
    pub pcr27: crate::Reg<pcr27::PCR27_SPEC>,
    #[doc = "0x70 - Pin Control Register n"]
    pub pcr28: crate::Reg<pcr28::PCR28_SPEC>,
    #[doc = "0x74 - Pin Control Register n"]
    pub pcr29: crate::Reg<pcr29::PCR29_SPEC>,
    #[doc = "0x78 - Pin Control Register n"]
    pub pcr30: crate::Reg<pcr30::PCR30_SPEC>,
    #[doc = "0x7c - Pin Control Register n"]
    pub pcr31: crate::Reg<pcr31::PCR31_SPEC>,
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: crate::Reg<gpclr::GPCLR_SPEC>,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: crate::Reg<gpchr::GPCHR_SPEC>,
    #[doc = "0x88 - Global Interrupt Control Low Register"]
    pub giclr: crate::Reg<giclr::GICLR_SPEC>,
    #[doc = "0x8c - Global Interrupt Control High Register"]
    pub gichr: crate::Reg<gichr::GICHR_SPEC>,
    _reserved36: [u8; 0x10],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: crate::Reg<isfr::ISFR_SPEC>,
    _reserved37: [u8; 0x1c],
    #[doc = "0xc0 - Digital Filter Enable Register"]
    pub dfer: crate::Reg<dfer::DFER_SPEC>,
    #[doc = "0xc4 - Digital Filter Clock Register"]
    pub dfcr: crate::Reg<dfcr::DFCR_SPEC>,
    #[doc = "0xc8 - Digital Filter Width Register"]
    pub dfwr: crate::Reg<dfwr::DFWR_SPEC>,
}
#[doc = "PCR0 register accessor: an alias for `Reg<PCR0_SPEC>`"]
pub type PCR0 = crate::Reg<pcr0::PCR0_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr0;
#[doc = "PCR1 register accessor: an alias for `Reg<PCR1_SPEC>`"]
pub type PCR1 = crate::Reg<pcr1::PCR1_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr1;
#[doc = "PCR2 register accessor: an alias for `Reg<PCR2_SPEC>`"]
pub type PCR2 = crate::Reg<pcr2::PCR2_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr2;
#[doc = "PCR3 register accessor: an alias for `Reg<PCR3_SPEC>`"]
pub type PCR3 = crate::Reg<pcr3::PCR3_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr3;
#[doc = "PCR4 register accessor: an alias for `Reg<PCR4_SPEC>`"]
pub type PCR4 = crate::Reg<pcr4::PCR4_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr4;
#[doc = "PCR5 register accessor: an alias for `Reg<PCR5_SPEC>`"]
pub type PCR5 = crate::Reg<pcr5::PCR5_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr5;
#[doc = "PCR6 register accessor: an alias for `Reg<PCR6_SPEC>`"]
pub type PCR6 = crate::Reg<pcr6::PCR6_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr6;
#[doc = "PCR7 register accessor: an alias for `Reg<PCR7_SPEC>`"]
pub type PCR7 = crate::Reg<pcr7::PCR7_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr7;
#[doc = "PCR8 register accessor: an alias for `Reg<PCR8_SPEC>`"]
pub type PCR8 = crate::Reg<pcr8::PCR8_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr8;
#[doc = "PCR9 register accessor: an alias for `Reg<PCR9_SPEC>`"]
pub type PCR9 = crate::Reg<pcr9::PCR9_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr9;
#[doc = "PCR10 register accessor: an alias for `Reg<PCR10_SPEC>`"]
pub type PCR10 = crate::Reg<pcr10::PCR10_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr10;
#[doc = "PCR11 register accessor: an alias for `Reg<PCR11_SPEC>`"]
pub type PCR11 = crate::Reg<pcr11::PCR11_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr11;
#[doc = "PCR12 register accessor: an alias for `Reg<PCR12_SPEC>`"]
pub type PCR12 = crate::Reg<pcr12::PCR12_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr12;
#[doc = "PCR13 register accessor: an alias for `Reg<PCR13_SPEC>`"]
pub type PCR13 = crate::Reg<pcr13::PCR13_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr13;
#[doc = "PCR14 register accessor: an alias for `Reg<PCR14_SPEC>`"]
pub type PCR14 = crate::Reg<pcr14::PCR14_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr14;
#[doc = "PCR15 register accessor: an alias for `Reg<PCR15_SPEC>`"]
pub type PCR15 = crate::Reg<pcr15::PCR15_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr15;
#[doc = "PCR16 register accessor: an alias for `Reg<PCR16_SPEC>`"]
pub type PCR16 = crate::Reg<pcr16::PCR16_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr16;
#[doc = "PCR17 register accessor: an alias for `Reg<PCR17_SPEC>`"]
pub type PCR17 = crate::Reg<pcr17::PCR17_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr17;
#[doc = "PCR18 register accessor: an alias for `Reg<PCR18_SPEC>`"]
pub type PCR18 = crate::Reg<pcr18::PCR18_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr18;
#[doc = "PCR19 register accessor: an alias for `Reg<PCR19_SPEC>`"]
pub type PCR19 = crate::Reg<pcr19::PCR19_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr19;
#[doc = "PCR20 register accessor: an alias for `Reg<PCR20_SPEC>`"]
pub type PCR20 = crate::Reg<pcr20::PCR20_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr20;
#[doc = "PCR21 register accessor: an alias for `Reg<PCR21_SPEC>`"]
pub type PCR21 = crate::Reg<pcr21::PCR21_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr21;
#[doc = "PCR22 register accessor: an alias for `Reg<PCR22_SPEC>`"]
pub type PCR22 = crate::Reg<pcr22::PCR22_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr22;
#[doc = "PCR23 register accessor: an alias for `Reg<PCR23_SPEC>`"]
pub type PCR23 = crate::Reg<pcr23::PCR23_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr23;
#[doc = "PCR24 register accessor: an alias for `Reg<PCR24_SPEC>`"]
pub type PCR24 = crate::Reg<pcr24::PCR24_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr24;
#[doc = "PCR25 register accessor: an alias for `Reg<PCR25_SPEC>`"]
pub type PCR25 = crate::Reg<pcr25::PCR25_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr25;
#[doc = "PCR26 register accessor: an alias for `Reg<PCR26_SPEC>`"]
pub type PCR26 = crate::Reg<pcr26::PCR26_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr26;
#[doc = "PCR27 register accessor: an alias for `Reg<PCR27_SPEC>`"]
pub type PCR27 = crate::Reg<pcr27::PCR27_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr27;
#[doc = "PCR28 register accessor: an alias for `Reg<PCR28_SPEC>`"]
pub type PCR28 = crate::Reg<pcr28::PCR28_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr28;
#[doc = "PCR29 register accessor: an alias for `Reg<PCR29_SPEC>`"]
pub type PCR29 = crate::Reg<pcr29::PCR29_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr29;
#[doc = "PCR30 register accessor: an alias for `Reg<PCR30_SPEC>`"]
pub type PCR30 = crate::Reg<pcr30::PCR30_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr30;
#[doc = "PCR31 register accessor: an alias for `Reg<PCR31_SPEC>`"]
pub type PCR31 = crate::Reg<pcr31::PCR31_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr31;
#[doc = "GPCLR register accessor: an alias for `Reg<GPCLR_SPEC>`"]
pub type GPCLR = crate::Reg<gpclr::GPCLR_SPEC>;
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "GPCHR register accessor: an alias for `Reg<GPCHR_SPEC>`"]
pub type GPCHR = crate::Reg<gpchr::GPCHR_SPEC>;
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "GICLR register accessor: an alias for `Reg<GICLR_SPEC>`"]
pub type GICLR = crate::Reg<giclr::GICLR_SPEC>;
#[doc = "Global Interrupt Control Low Register"]
pub mod giclr;
#[doc = "GICHR register accessor: an alias for `Reg<GICHR_SPEC>`"]
pub type GICHR = crate::Reg<gichr::GICHR_SPEC>;
#[doc = "Global Interrupt Control High Register"]
pub mod gichr;
#[doc = "ISFR register accessor: an alias for `Reg<ISFR_SPEC>`"]
pub type ISFR = crate::Reg<isfr::ISFR_SPEC>;
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
#[doc = "DFER register accessor: an alias for `Reg<DFER_SPEC>`"]
pub type DFER = crate::Reg<dfer::DFER_SPEC>;
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "DFCR register accessor: an alias for `Reg<DFCR_SPEC>`"]
pub type DFCR = crate::Reg<dfcr::DFCR_SPEC>;
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "DFWR register accessor: an alias for `Reg<DFWR_SPEC>`"]
pub type DFWR = crate::Reg<dfwr::DFWR_SPEC>;
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
