#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x08 - Module Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x0c - Module Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x10 - Module Interrupt Enable Register"]
    pub mier: crate::Reg<mier::MIER_SPEC>,
    #[doc = "0x14 - Set Timer Enable Register"]
    pub setten: crate::Reg<setten::SETTEN_SPEC>,
    #[doc = "0x18 - Clear Timer Enable Register"]
    pub clrten: crate::Reg<clrten::CLRTEN_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20..0x2c - no description available"]
    pub tmr0: TMR,
    _reserved8: [u8; 0x04],
    #[doc = "0x30..0x3c - no description available"]
    pub tmr1: TMR,
    _reserved9: [u8; 0x04],
    #[doc = "0x40..0x4c - no description available"]
    pub tmr2: TMR,
    _reserved10: [u8; 0x04],
    #[doc = "0x50..0x5c - no description available"]
    pub tmr3: TMR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TMR {
    #[doc = "0x00 - Timer Value Register"]
    pub tval: crate::Reg<self::tmr::tval::TVAL_SPEC>,
    #[doc = "0x04 - Current Timer Value"]
    pub cval: crate::Reg<self::tmr::cval::CVAL_SPEC>,
    #[doc = "0x08 - Timer Control Register"]
    pub tctrl: crate::Reg<self::tmr::tctrl::TCTRL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod tmr;
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Control Register"]
pub mod mcr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Module Status Register"]
pub mod msr;
#[doc = "MIER register accessor: an alias for `Reg<MIER_SPEC>`"]
pub type MIER = crate::Reg<mier::MIER_SPEC>;
#[doc = "Module Interrupt Enable Register"]
pub mod mier;
#[doc = "SETTEN register accessor: an alias for `Reg<SETTEN_SPEC>`"]
pub type SETTEN = crate::Reg<setten::SETTEN_SPEC>;
#[doc = "Set Timer Enable Register"]
pub mod setten;
#[doc = "CLRTEN register accessor: an alias for `Reg<CLRTEN_SPEC>`"]
pub type CLRTEN = crate::Reg<clrten::CLRTEN_SPEC>;
#[doc = "Clear Timer Enable Register"]
pub mod clrten;
