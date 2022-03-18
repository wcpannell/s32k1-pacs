#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Pin Control Register n"]
    pub pcr: [crate::Reg<pcr::PCR_SPEC>; 32],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: crate::Reg<gpclr::GPCLR_SPEC>,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: crate::Reg<gpchr::GPCHR_SPEC>,
    #[doc = "0x88 - Global Interrupt Control Low Register"]
    pub giclr: crate::Reg<giclr::GICLR_SPEC>,
    #[doc = "0x8c - Global Interrupt Control High Register"]
    pub gichr: crate::Reg<gichr::GICHR_SPEC>,
    _reserved5: [u8; 0x10],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: crate::Reg<isfr::ISFR_SPEC>,
    _reserved6: [u8; 0x1c],
    #[doc = "0xc0 - Digital Filter Enable Register"]
    pub dfer: crate::Reg<dfer::DFER_SPEC>,
    #[doc = "0xc4 - Digital Filter Clock Register"]
    pub dfcr: crate::Reg<dfcr::DFCR_SPEC>,
    #[doc = "0xc8 - Digital Filter Width Register"]
    pub dfwr: crate::Reg<dfwr::DFWR_SPEC>,
}
#[doc = "PCR register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "GPCLR register accessor: an alias for `Reg<GPCLR_SPEC>`"]
pub type GPCLR = crate::Reg<gpclr::GPCLR_SPEC>;
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "GPCHR register accessor: an alias for `Reg<GPCHR_SPEC>`"]
pub type GPCHR = crate::Reg<gpchr::GPCHR_SPEC>;
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "GICLR register accessor: an alias for `Reg<GICLR_SPEC>`"]
pub type GICLR = crate::Reg<giclr::GICLR_SPEC>;
#[doc = "Global Interrupt Control Low Register"]
pub mod giclr;
#[doc = "GICHR register accessor: an alias for `Reg<GICHR_SPEC>`"]
pub type GICHR = crate::Reg<gichr::GICHR_SPEC>;
#[doc = "Global Interrupt Control High Register"]
pub mod gichr;
#[doc = "ISFR register accessor: an alias for `Reg<ISFR_SPEC>`"]
pub type ISFR = crate::Reg<isfr::ISFR_SPEC>;
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
#[doc = "DFER register accessor: an alias for `Reg<DFER_SPEC>`"]
pub type DFER = crate::Reg<dfer::DFER_SPEC>;
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "DFCR register accessor: an alias for `Reg<DFCR_SPEC>`"]
pub type DFCR = crate::Reg<dfcr::DFCR_SPEC>;
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "DFWR register accessor: an alias for `Reg<DFWR_SPEC>`"]
pub type DFWR = crate::Reg<dfwr::DFWR_SPEC>;
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
