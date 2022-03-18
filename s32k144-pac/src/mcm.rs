#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: crate::Reg<plasc::PLASC_SPEC>,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: crate::Reg<plamc::PLAMC_SPEC>,
    #[doc = "0x0c - Core Platform Control Register"]
    pub cpcr: crate::Reg<cpcr::CPCR_SPEC>,
    #[doc = "0x10 - Interrupt Status and Control Register"]
    pub iscr: crate::Reg<iscr::ISCR_SPEC>,
    _reserved4: [u8; 0x1c],
    #[doc = "0x30 - Process ID Register"]
    pub pid: crate::Reg<pid::PID_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: crate::Reg<cpo::CPO_SPEC>,
    _reserved6: [u8; 0x03bc],
    #[doc = "0x400 - Local Memory Descriptor Register"]
    pub lmdr0: crate::Reg<lmdr0::LMDR0_SPEC>,
    #[doc = "0x404 - Local Memory Descriptor Register"]
    pub lmdr1: crate::Reg<lmdr1::LMDR1_SPEC>,
    #[doc = "0x408 - Local Memory Descriptor Register2"]
    pub lmdr2: crate::Reg<lmdr2::LMDR2_SPEC>,
    _reserved9: [u8; 0x74],
    #[doc = "0x480 - LMEM Parity and ECC Control Register"]
    pub lmpecr: crate::Reg<lmpecr::LMPECR_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x488 - LMEM Parity and ECC Interrupt Register"]
    pub lmpeir: crate::Reg<lmpeir::LMPEIR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x490 - LMEM Fault Address Register"]
    pub lmfar: crate::Reg<lmfar::LMFAR_SPEC>,
    #[doc = "0x494 - LMEM Fault Attribute Register"]
    pub lmfatr: crate::Reg<lmfatr::LMFATR_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x4a0 - LMEM Fault Data High Register"]
    pub lmfdhr: crate::Reg<lmfdhr::LMFDHR_SPEC>,
    #[doc = "0x4a4 - LMEM Fault Data Low Register"]
    pub lmfdlr: crate::Reg<lmfdlr::LMFDLR_SPEC>,
}
#[doc = "PLASC register accessor: an alias for `Reg<PLASC_SPEC>`"]
pub type PLASC = crate::Reg<plasc::PLASC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "PLAMC register accessor: an alias for `Reg<PLAMC_SPEC>`"]
pub type PLAMC = crate::Reg<plamc::PLAMC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "CPCR register accessor: an alias for `Reg<CPCR_SPEC>`"]
pub type CPCR = crate::Reg<cpcr::CPCR_SPEC>;
#[doc = "Core Platform Control Register"]
pub mod cpcr;
#[doc = "ISCR register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
#[doc = "PID register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Process ID Register"]
pub mod pid;
#[doc = "CPO register accessor: an alias for `Reg<CPO_SPEC>`"]
pub type CPO = crate::Reg<cpo::CPO_SPEC>;
#[doc = "Compute Operation Control Register"]
pub mod cpo;
#[doc = "LMDR0 register accessor: an alias for `Reg<LMDR0_SPEC>`"]
pub type LMDR0 = crate::Reg<lmdr0::LMDR0_SPEC>;
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr0;
#[doc = "LMDR1 register accessor: an alias for `Reg<LMDR1_SPEC>`"]
pub type LMDR1 = crate::Reg<lmdr1::LMDR1_SPEC>;
#[doc = "Local Memory Descriptor Register"]
pub mod lmdr1;
#[doc = "LMDR2 register accessor: an alias for `Reg<LMDR2_SPEC>`"]
pub type LMDR2 = crate::Reg<lmdr2::LMDR2_SPEC>;
#[doc = "Local Memory Descriptor Register2"]
pub mod lmdr2;
#[doc = "LMPECR register accessor: an alias for `Reg<LMPECR_SPEC>`"]
pub type LMPECR = crate::Reg<lmpecr::LMPECR_SPEC>;
#[doc = "LMEM Parity and ECC Control Register"]
pub mod lmpecr;
#[doc = "LMPEIR register accessor: an alias for `Reg<LMPEIR_SPEC>`"]
pub type LMPEIR = crate::Reg<lmpeir::LMPEIR_SPEC>;
#[doc = "LMEM Parity and ECC Interrupt Register"]
pub mod lmpeir;
#[doc = "LMFAR register accessor: an alias for `Reg<LMFAR_SPEC>`"]
pub type LMFAR = crate::Reg<lmfar::LMFAR_SPEC>;
#[doc = "LMEM Fault Address Register"]
pub mod lmfar;
#[doc = "LMFATR register accessor: an alias for `Reg<LMFATR_SPEC>`"]
pub type LMFATR = crate::Reg<lmfatr::LMFATR_SPEC>;
#[doc = "LMEM Fault Attribute Register"]
pub mod lmfatr;
#[doc = "LMFDHR register accessor: an alias for `Reg<LMFDHR_SPEC>`"]
pub type LMFDHR = crate::Reg<lmfdhr::LMFDHR_SPEC>;
#[doc = "LMEM Fault Data High Register"]
pub mod lmfdhr;
#[doc = "LMFDLR register accessor: an alias for `Reg<LMFDLR_SPEC>`"]
pub type LMFDLR = crate::Reg<lmfdlr::LMFDLR_SPEC>;
#[doc = "LMEM Fault Data Low Register"]
pub mod lmfdlr;
