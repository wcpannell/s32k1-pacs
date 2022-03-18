#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub lmem_pcccr: crate::Reg<lmem_pcccr::LMEM_PCCCR_SPEC>,
    #[doc = "0x04 - Cache line control register"]
    pub lmem_pcclcr: crate::Reg<lmem_pcclcr::LMEM_PCCLCR_SPEC>,
    #[doc = "0x08 - Cache search address register"]
    pub lmem_pccsar: crate::Reg<lmem_pccsar::LMEM_PCCSAR_SPEC>,
    #[doc = "0x0c - Cache read/write value register"]
    pub lmem_pcccvr: crate::Reg<lmem_pcccvr::LMEM_PCCCVR_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: crate::Reg<pccrmr::PCCRMR_SPEC>,
}
#[doc = "LMEM_PCCCR register accessor: an alias for `Reg<LMEM_PCCCR_SPEC>`"]
pub type LMEM_PCCCR = crate::Reg<lmem_pcccr::LMEM_PCCCR_SPEC>;
#[doc = "Cache control register"]
pub mod lmem_pcccr;
#[doc = "LMEM_PCCLCR register accessor: an alias for `Reg<LMEM_PCCLCR_SPEC>`"]
pub type LMEM_PCCLCR = crate::Reg<lmem_pcclcr::LMEM_PCCLCR_SPEC>;
#[doc = "Cache line control register"]
pub mod lmem_pcclcr;
#[doc = "LMEM_PCCSAR register accessor: an alias for `Reg<LMEM_PCCSAR_SPEC>`"]
pub type LMEM_PCCSAR = crate::Reg<lmem_pccsar::LMEM_PCCSAR_SPEC>;
#[doc = "Cache search address register"]
pub mod lmem_pccsar;
#[doc = "LMEM_PCCCVR register accessor: an alias for `Reg<LMEM_PCCCVR_SPEC>`"]
pub type LMEM_PCCCVR = crate::Reg<lmem_pcccvr::LMEM_PCCCVR_SPEC>;
#[doc = "Cache read/write value register"]
pub mod lmem_pcccvr;
#[doc = "PCCRMR register accessor: an alias for `Reg<PCCRMR_SPEC>`"]
pub type PCCRMR = crate::Reg<pccrmr::PCCRMR_SPEC>;
#[doc = "Cache regions mode register"]
pub mod pccrmr;
