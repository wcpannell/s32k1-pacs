#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Configuration register"]
    pub chcfg0: crate::Reg<chcfg0::CHCFG0_SPEC>,
    #[doc = "0x01 - Channel Configuration register"]
    pub chcfg1: crate::Reg<chcfg1::CHCFG1_SPEC>,
    #[doc = "0x02 - Channel Configuration register"]
    pub chcfg2: crate::Reg<chcfg2::CHCFG2_SPEC>,
    #[doc = "0x03 - Channel Configuration register"]
    pub chcfg3: crate::Reg<chcfg3::CHCFG3_SPEC>,
    #[doc = "0x04 - Channel Configuration register"]
    pub chcfg4: crate::Reg<chcfg4::CHCFG4_SPEC>,
    #[doc = "0x05 - Channel Configuration register"]
    pub chcfg5: crate::Reg<chcfg5::CHCFG5_SPEC>,
    #[doc = "0x06 - Channel Configuration register"]
    pub chcfg6: crate::Reg<chcfg6::CHCFG6_SPEC>,
    #[doc = "0x07 - Channel Configuration register"]
    pub chcfg7: crate::Reg<chcfg7::CHCFG7_SPEC>,
    #[doc = "0x08 - Channel Configuration register"]
    pub chcfg8: crate::Reg<chcfg8::CHCFG8_SPEC>,
    #[doc = "0x09 - Channel Configuration register"]
    pub chcfg9: crate::Reg<chcfg9::CHCFG9_SPEC>,
    #[doc = "0x0a - Channel Configuration register"]
    pub chcfg10: crate::Reg<chcfg10::CHCFG10_SPEC>,
    #[doc = "0x0b - Channel Configuration register"]
    pub chcfg11: crate::Reg<chcfg11::CHCFG11_SPEC>,
    #[doc = "0x0c - Channel Configuration register"]
    pub chcfg12: crate::Reg<chcfg12::CHCFG12_SPEC>,
    #[doc = "0x0d - Channel Configuration register"]
    pub chcfg13: crate::Reg<chcfg13::CHCFG13_SPEC>,
    #[doc = "0x0e - Channel Configuration register"]
    pub chcfg14: crate::Reg<chcfg14::CHCFG14_SPEC>,
    #[doc = "0x0f - Channel Configuration register"]
    pub chcfg15: crate::Reg<chcfg15::CHCFG15_SPEC>,
}
#[doc = "CHCFG0 register accessor: an alias for `Reg<CHCFG0_SPEC>`"]
pub type CHCFG0 = crate::Reg<chcfg0::CHCFG0_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg0;
#[doc = "CHCFG1 register accessor: an alias for `Reg<CHCFG1_SPEC>`"]
pub type CHCFG1 = crate::Reg<chcfg1::CHCFG1_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg1;
#[doc = "CHCFG2 register accessor: an alias for `Reg<CHCFG2_SPEC>`"]
pub type CHCFG2 = crate::Reg<chcfg2::CHCFG2_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg2;
#[doc = "CHCFG3 register accessor: an alias for `Reg<CHCFG3_SPEC>`"]
pub type CHCFG3 = crate::Reg<chcfg3::CHCFG3_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg3;
#[doc = "CHCFG4 register accessor: an alias for `Reg<CHCFG4_SPEC>`"]
pub type CHCFG4 = crate::Reg<chcfg4::CHCFG4_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg4;
#[doc = "CHCFG5 register accessor: an alias for `Reg<CHCFG5_SPEC>`"]
pub type CHCFG5 = crate::Reg<chcfg5::CHCFG5_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg5;
#[doc = "CHCFG6 register accessor: an alias for `Reg<CHCFG6_SPEC>`"]
pub type CHCFG6 = crate::Reg<chcfg6::CHCFG6_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg6;
#[doc = "CHCFG7 register accessor: an alias for `Reg<CHCFG7_SPEC>`"]
pub type CHCFG7 = crate::Reg<chcfg7::CHCFG7_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg7;
#[doc = "CHCFG8 register accessor: an alias for `Reg<CHCFG8_SPEC>`"]
pub type CHCFG8 = crate::Reg<chcfg8::CHCFG8_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg8;
#[doc = "CHCFG9 register accessor: an alias for `Reg<CHCFG9_SPEC>`"]
pub type CHCFG9 = crate::Reg<chcfg9::CHCFG9_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg9;
#[doc = "CHCFG10 register accessor: an alias for `Reg<CHCFG10_SPEC>`"]
pub type CHCFG10 = crate::Reg<chcfg10::CHCFG10_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg10;
#[doc = "CHCFG11 register accessor: an alias for `Reg<CHCFG11_SPEC>`"]
pub type CHCFG11 = crate::Reg<chcfg11::CHCFG11_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg11;
#[doc = "CHCFG12 register accessor: an alias for `Reg<CHCFG12_SPEC>`"]
pub type CHCFG12 = crate::Reg<chcfg12::CHCFG12_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg12;
#[doc = "CHCFG13 register accessor: an alias for `Reg<CHCFG13_SPEC>`"]
pub type CHCFG13 = crate::Reg<chcfg13::CHCFG13_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg13;
#[doc = "CHCFG14 register accessor: an alias for `Reg<CHCFG14_SPEC>`"]
pub type CHCFG14 = crate::Reg<chcfg14::CHCFG14_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg14;
#[doc = "CHCFG15 register accessor: an alias for `Reg<CHCFG15_SPEC>`"]
pub type CHCFG15 = crate::Reg<chcfg15::CHCFG15_SPEC>;
#[doc = "Channel Configuration register"]
pub mod chcfg15;
