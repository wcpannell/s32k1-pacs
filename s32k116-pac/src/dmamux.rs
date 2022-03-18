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
