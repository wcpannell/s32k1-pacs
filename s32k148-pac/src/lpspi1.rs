#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x14 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x1c - DMA Enable Register"]
    pub der: crate::Reg<der::DER_SPEC>,
    #[doc = "0x20 - Configuration Register 0"]
    pub cfgr0: crate::Reg<cfgr0::CFGR0_SPEC>,
    #[doc = "0x24 - Configuration Register 1"]
    pub cfgr1: crate::Reg<cfgr1::CFGR1_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - Data Match Register 0"]
    pub dmr0: crate::Reg<dmr0::DMR0_SPEC>,
    #[doc = "0x34 - Data Match Register 1"]
    pub dmr1: crate::Reg<dmr1::DMR1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - Clock Configuration Register"]
    pub ccr: crate::Reg<ccr::CCR_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x58 - FIFO Control Register"]
    pub fcr: crate::Reg<fcr::FCR_SPEC>,
    #[doc = "0x5c - FIFO Status Register"]
    pub fsr: crate::Reg<fsr::FSR_SPEC>,
    #[doc = "0x60 - Transmit Command Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x64 - Transmit Data Register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x70 - Receive Status Register"]
    pub rsr: crate::Reg<rsr::RSR_SPEC>,
    #[doc = "0x74 - Receive Data Register"]
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
}
#[doc = "VERID register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "DER register accessor: an alias for `Reg<DER_SPEC>`"]
pub type DER = crate::Reg<der::DER_SPEC>;
#[doc = "DMA Enable Register"]
pub mod der;
#[doc = "CFGR0 register accessor: an alias for `Reg<CFGR0_SPEC>`"]
pub type CFGR0 = crate::Reg<cfgr0::CFGR0_SPEC>;
#[doc = "Configuration Register 0"]
pub mod cfgr0;
#[doc = "CFGR1 register accessor: an alias for `Reg<CFGR1_SPEC>`"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
#[doc = "Configuration Register 1"]
pub mod cfgr1;
#[doc = "DMR0 register accessor: an alias for `Reg<DMR0_SPEC>`"]
pub type DMR0 = crate::Reg<dmr0::DMR0_SPEC>;
#[doc = "Data Match Register 0"]
pub mod dmr0;
#[doc = "DMR1 register accessor: an alias for `Reg<DMR1_SPEC>`"]
pub type DMR1 = crate::Reg<dmr1::DMR1_SPEC>;
#[doc = "Data Match Register 1"]
pub mod dmr1;
#[doc = "CCR register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "Clock Configuration Register"]
pub mod ccr;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "FSR register accessor: an alias for `Reg<FSR_SPEC>`"]
pub type FSR = crate::Reg<fsr::FSR_SPEC>;
#[doc = "FIFO Status Register"]
pub mod fsr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Command Register"]
pub mod tcr;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "RSR register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "RDR register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
