#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    #[doc = "0x08 - System Reset Status Register"]
    pub srs: crate::Reg<srs::SRS_SPEC>,
    #[doc = "0x0c - Reset Pin Control register"]
    pub rpc: crate::Reg<rpc::RPC_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Sticky System Reset Status Register"]
    pub ssrs: crate::Reg<ssrs::SSRS_SPEC>,
    #[doc = "0x1c - System Reset Interrupt Enable Register"]
    pub srie: crate::Reg<srie::SRIE_SPEC>,
}
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "SRS register accessor: an alias for `Reg<SRS_SPEC>`"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "System Reset Status Register"]
pub mod srs;
#[doc = "RPC register accessor: an alias for `Reg<RPC_SPEC>`"]
pub type RPC = crate::Reg<rpc::RPC_SPEC>;
#[doc = "Reset Pin Control register"]
pub mod rpc;
#[doc = "SSRS register accessor: an alias for `Reg<SSRS_SPEC>`"]
pub type SSRS = crate::Reg<ssrs::SSRS_SPEC>;
#[doc = "Sticky System Reset Status Register"]
pub mod ssrs;
#[doc = "SRIE register accessor: an alias for `Reg<SRIE_SPEC>`"]
pub type SRIE = crate::Reg<srie::SRIE_SPEC>;
#[doc = "System Reset Interrupt Enable Register"]
pub mod srie;
