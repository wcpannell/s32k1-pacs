#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Data register"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x04 - CRC Polynomial register"]
    pub gpoly: crate::Reg<gpoly::GPOLY_SPEC>,
    #[doc = "0x08 - CRC Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
}
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "CRC Data register"]
pub mod data;
#[doc = "GPOLY register accessor: an alias for `Reg<GPOLY_SPEC>`"]
pub type GPOLY = crate::Reg<gpoly::GPOLY_SPEC>;
#[doc = "CRC Polynomial register"]
pub mod gpoly;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control register"]
pub mod ctrl;
