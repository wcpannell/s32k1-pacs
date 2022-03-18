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
    #[doc = "0x20 - Timer Value Register"]
    pub tval0: crate::Reg<tval0::TVAL0_SPEC>,
    #[doc = "0x24 - Current Timer Value"]
    pub cval0: crate::Reg<cval0::CVAL0_SPEC>,
    #[doc = "0x28 - Timer Control Register"]
    pub tctrl0: crate::Reg<tctrl0::TCTRL0_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - Timer Value Register"]
    pub tval1: crate::Reg<tval1::TVAL1_SPEC>,
    #[doc = "0x34 - Current Timer Value"]
    pub cval1: crate::Reg<cval1::CVAL1_SPEC>,
    #[doc = "0x38 - Timer Control Register"]
    pub tctrl1: crate::Reg<tctrl1::TCTRL1_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x40 - Timer Value Register"]
    pub tval2: crate::Reg<tval2::TVAL2_SPEC>,
    #[doc = "0x44 - Current Timer Value"]
    pub cval2: crate::Reg<cval2::CVAL2_SPEC>,
    #[doc = "0x48 - Timer Control Register"]
    pub tctrl2: crate::Reg<tctrl2::TCTRL2_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x50 - Timer Value Register"]
    pub tval3: crate::Reg<tval3::TVAL3_SPEC>,
    #[doc = "0x54 - Current Timer Value"]
    pub cval3: crate::Reg<cval3::CVAL3_SPEC>,
    #[doc = "0x58 - Timer Control Register"]
    pub tctrl3: crate::Reg<tctrl3::TCTRL3_SPEC>,
}
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
#[doc = "TVAL0 register accessor: an alias for `Reg<TVAL0_SPEC>`"]
pub type TVAL0 = crate::Reg<tval0::TVAL0_SPEC>;
#[doc = "Timer Value Register"]
pub mod tval0;
#[doc = "CVAL0 register accessor: an alias for `Reg<CVAL0_SPEC>`"]
pub type CVAL0 = crate::Reg<cval0::CVAL0_SPEC>;
#[doc = "Current Timer Value"]
pub mod cval0;
#[doc = "TCTRL0 register accessor: an alias for `Reg<TCTRL0_SPEC>`"]
pub type TCTRL0 = crate::Reg<tctrl0::TCTRL0_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl0;
#[doc = "TVAL1 register accessor: an alias for `Reg<TVAL1_SPEC>`"]
pub type TVAL1 = crate::Reg<tval1::TVAL1_SPEC>;
#[doc = "Timer Value Register"]
pub mod tval1;
#[doc = "CVAL1 register accessor: an alias for `Reg<CVAL1_SPEC>`"]
pub type CVAL1 = crate::Reg<cval1::CVAL1_SPEC>;
#[doc = "Current Timer Value"]
pub mod cval1;
#[doc = "TCTRL1 register accessor: an alias for `Reg<TCTRL1_SPEC>`"]
pub type TCTRL1 = crate::Reg<tctrl1::TCTRL1_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl1;
#[doc = "TVAL2 register accessor: an alias for `Reg<TVAL2_SPEC>`"]
pub type TVAL2 = crate::Reg<tval2::TVAL2_SPEC>;
#[doc = "Timer Value Register"]
pub mod tval2;
#[doc = "CVAL2 register accessor: an alias for `Reg<CVAL2_SPEC>`"]
pub type CVAL2 = crate::Reg<cval2::CVAL2_SPEC>;
#[doc = "Current Timer Value"]
pub mod cval2;
#[doc = "TCTRL2 register accessor: an alias for `Reg<TCTRL2_SPEC>`"]
pub type TCTRL2 = crate::Reg<tctrl2::TCTRL2_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl2;
#[doc = "TVAL3 register accessor: an alias for `Reg<TVAL3_SPEC>`"]
pub type TVAL3 = crate::Reg<tval3::TVAL3_SPEC>;
#[doc = "Timer Value Register"]
pub mod tval3;
#[doc = "CVAL3 register accessor: an alias for `Reg<CVAL3_SPEC>`"]
pub type CVAL3 = crate::Reg<cval3::CVAL3_SPEC>;
#[doc = "Current Timer Value"]
pub mod cval3;
#[doc = "TCTRL3 register accessor: an alias for `Reg<TCTRL3_SPEC>`"]
pub type TCTRL3 = crate::Reg<tctrl3::TCTRL3_SPEC>;
#[doc = "Timer Control Register"]
pub mod tctrl3;
