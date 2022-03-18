#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache control register"]
    pub pcccr: crate::Reg<pcccr::PCCCR_SPEC>,
    #[doc = "0x04 - Cache line control register"]
    pub pcclcr: crate::Reg<pcclcr::PCCLCR_SPEC>,
    #[doc = "0x08 - Cache search address register"]
    pub pccsar: crate::Reg<pccsar::PCCSAR_SPEC>,
    #[doc = "0x0c - Cache read/write value register"]
    pub pcccvr: crate::Reg<pcccvr::PCCCVR_SPEC>,
    _reserved4: [u8; 0x10],
    #[doc = "0x20 - Cache regions mode register"]
    pub pccrmr: crate::Reg<pccrmr::PCCRMR_SPEC>,
}
#[doc = "PCCCR register accessor: an alias for `Reg<PCCCR_SPEC>`"]
pub type PCCCR = crate::Reg<pcccr::PCCCR_SPEC>;
#[doc = "Cache control register"]
pub mod pcccr;
#[doc = "PCCLCR register accessor: an alias for `Reg<PCCLCR_SPEC>`"]
pub type PCCLCR = crate::Reg<pcclcr::PCCLCR_SPEC>;
#[doc = "Cache line control register"]
pub mod pcclcr;
#[doc = "PCCSAR register accessor: an alias for `Reg<PCCSAR_SPEC>`"]
pub type PCCSAR = crate::Reg<pccsar::PCCSAR_SPEC>;
#[doc = "Cache search address register"]
pub mod pccsar;
#[doc = "PCCCVR register accessor: an alias for `Reg<PCCCVR_SPEC>`"]
pub type PCCCVR = crate::Reg<pcccvr::PCCCVR_SPEC>;
#[doc = "Cache read/write value register"]
pub mod pcccvr;
#[doc = "PCCRMR register accessor: an alias for `Reg<PCCRMR_SPEC>`"]
pub type PCCRMR = crate::Reg<pccrmr::PCCRMR_SPEC>;
#[doc = "Cache regions mode register"]
pub mod pccrmr;
