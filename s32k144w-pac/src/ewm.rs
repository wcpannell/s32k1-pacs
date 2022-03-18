#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x01 - Service Register"]
    pub serv: crate::Reg<serv::SERV_SPEC>,
    #[doc = "0x02 - Compare Low Register"]
    pub cmpl: crate::Reg<cmpl::CMPL_SPEC>,
    #[doc = "0x03 - Compare High Register"]
    pub cmph: crate::Reg<cmph::CMPH_SPEC>,
    _reserved4: [u8; 0x01],
    #[doc = "0x05 - Clock Prescaler Register"]
    pub clkprescaler: crate::Reg<clkprescaler::CLKPRESCALER_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SERV register accessor: an alias for `Reg<SERV_SPEC>`"]
pub type SERV = crate::Reg<serv::SERV_SPEC>;
#[doc = "Service Register"]
pub mod serv;
#[doc = "CMPL register accessor: an alias for `Reg<CMPL_SPEC>`"]
pub type CMPL = crate::Reg<cmpl::CMPL_SPEC>;
#[doc = "Compare Low Register"]
pub mod cmpl;
#[doc = "CMPH register accessor: an alias for `Reg<CMPH_SPEC>`"]
pub type CMPH = crate::Reg<cmph::CMPH_SPEC>;
#[doc = "Compare High Register"]
pub mod cmph;
#[doc = "CLKPRESCALER register accessor: an alias for `Reg<CLKPRESCALER_SPEC>`"]
pub type CLKPRESCALER = crate::Reg<clkprescaler::CLKPRESCALER_SPEC>;
#[doc = "Clock Prescaler Register"]
pub mod clkprescaler;
