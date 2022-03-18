#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Timer Control Status Register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x04 - Low Power Timer Prescale Register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x08 - Low Power Timer Compare Register"]
    pub cmr: crate::Reg<cmr::CMR_SPEC>,
    #[doc = "0x0c - Low Power Timer Counter Register"]
    pub cnr: crate::Reg<cnr::CNR_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Low Power Timer Control Status Register"]
pub mod csr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Low Power Timer Prescale Register"]
pub mod psr;
#[doc = "CMR register accessor: an alias for `Reg<CMR_SPEC>`"]
pub type CMR = crate::Reg<cmr::CMR_SPEC>;
#[doc = "Low Power Timer Compare Register"]
pub mod cmr;
#[doc = "CNR register accessor: an alias for `Reg<CNR_SPEC>`"]
pub type CNR = crate::Reg<cnr::CNR_SPEC>;
#[doc = "Low Power Timer Counter Register"]
pub mod cnr;
