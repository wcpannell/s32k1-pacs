#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    pub pdor: crate::Reg<pdor::PDOR_SPEC>,
    #[doc = "0x04 - Port Set Output Register"]
    pub psor: crate::Reg<psor::PSOR_SPEC>,
    #[doc = "0x08 - Port Clear Output Register"]
    pub pcor: crate::Reg<pcor::PCOR_SPEC>,
    #[doc = "0x0c - Port Toggle Output Register"]
    pub ptor: crate::Reg<ptor::PTOR_SPEC>,
    #[doc = "0x10 - Port Data Input Register"]
    pub pdir: crate::Reg<pdir::PDIR_SPEC>,
    #[doc = "0x14 - Port Data Direction Register"]
    pub pddr: crate::Reg<pddr::PDDR_SPEC>,
    #[doc = "0x18 - Port Input Disable Register"]
    pub pidr: crate::Reg<pidr::PIDR_SPEC>,
}
#[doc = "PDOR register accessor: an alias for `Reg<PDOR_SPEC>`"]
pub type PDOR = crate::Reg<pdor::PDOR_SPEC>;
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "PSOR register accessor: an alias for `Reg<PSOR_SPEC>`"]
pub type PSOR = crate::Reg<psor::PSOR_SPEC>;
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "PCOR register accessor: an alias for `Reg<PCOR_SPEC>`"]
pub type PCOR = crate::Reg<pcor::PCOR_SPEC>;
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "PTOR register accessor: an alias for `Reg<PTOR_SPEC>`"]
pub type PTOR = crate::Reg<ptor::PTOR_SPEC>;
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "PDIR register accessor: an alias for `Reg<PDIR_SPEC>`"]
pub type PDIR = crate::Reg<pdir::PDIR_SPEC>;
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "PDDR register accessor: an alias for `Reg<PDDR_SPEC>`"]
pub type PDDR = crate::Reg<pddr::PDDR_SPEC>;
#[doc = "Port Data Direction Register"]
pub mod pddr;
#[doc = "PIDR register accessor: an alias for `Reg<PIDR_SPEC>`"]
pub type PIDR = crate::Reg<pidr::PIDR_SPEC>;
#[doc = "Port Input Disable Register"]
pub mod pidr;
