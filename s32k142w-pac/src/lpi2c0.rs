#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: crate::Reg<verid::VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub param: crate::Reg<param::PARAM_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Master Control Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x14 - Master Status Register"]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x18 - Master Interrupt Enable Register"]
    pub mier: crate::Reg<mier::MIER_SPEC>,
    #[doc = "0x1c - Master DMA Enable Register"]
    pub mder: crate::Reg<mder::MDER_SPEC>,
    #[doc = "0x20 - Master Configuration Register 0"]
    pub mcfgr0: crate::Reg<mcfgr0::MCFGR0_SPEC>,
    #[doc = "0x24 - Master Configuration Register 1"]
    pub mcfgr1: crate::Reg<mcfgr1::MCFGR1_SPEC>,
    #[doc = "0x28 - Master Configuration Register 2"]
    pub mcfgr2: crate::Reg<mcfgr2::MCFGR2_SPEC>,
    #[doc = "0x2c - Master Configuration Register 3"]
    pub mcfgr3: crate::Reg<mcfgr3::MCFGR3_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x40 - Master Data Match Register"]
    pub mdmr: crate::Reg<mdmr::MDMR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x48 - Master Clock Configuration Register 0"]
    pub mccr0: crate::Reg<mccr0::MCCR0_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x50 - Master Clock Configuration Register 1"]
    pub mccr1: crate::Reg<mccr1::MCCR1_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x58 - Master FIFO Control Register"]
    pub mfcr: crate::Reg<mfcr::MFCR_SPEC>,
    #[doc = "0x5c - Master FIFO Status Register"]
    pub mfsr: crate::Reg<mfsr::MFSR_SPEC>,
    #[doc = "0x60 - Master Transmit Data Register"]
    pub mtdr: crate::Reg<mtdr::MTDR_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x70 - Master Receive Data Register"]
    pub mrdr: crate::Reg<mrdr::MRDR_SPEC>,
    _reserved17: [u8; 0x9c],
    #[doc = "0x110 - Slave Control Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x114 - Slave Status Register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x118 - Slave Interrupt Enable Register"]
    pub sier: crate::Reg<sier::SIER_SPEC>,
    #[doc = "0x11c - Slave DMA Enable Register"]
    pub sder: crate::Reg<sder::SDER_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x124 - Slave Configuration Register 1"]
    pub scfgr1: crate::Reg<scfgr1::SCFGR1_SPEC>,
    #[doc = "0x128 - Slave Configuration Register 2"]
    pub scfgr2: crate::Reg<scfgr2::SCFGR2_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0x140 - Slave Address Match Register"]
    pub samr: crate::Reg<samr::SAMR_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0x150 - Slave Address Status Register"]
    pub sasr: crate::Reg<sasr::SASR_SPEC>,
    #[doc = "0x154 - Slave Transmit ACK Register"]
    pub star: crate::Reg<star::STAR_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0x160 - Slave Transmit Data Register"]
    pub stdr: crate::Reg<stdr::STDR_SPEC>,
    _reserved27: [u8; 0x0c],
    #[doc = "0x170 - Slave Receive Data Register"]
    pub srdr: crate::Reg<srdr::SRDR_SPEC>,
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
#[doc = "Master Control Register"]
pub mod mcr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Master Status Register"]
pub mod msr;
#[doc = "MIER register accessor: an alias for `Reg<MIER_SPEC>`"]
pub type MIER = crate::Reg<mier::MIER_SPEC>;
#[doc = "Master Interrupt Enable Register"]
pub mod mier;
#[doc = "MDER register accessor: an alias for `Reg<MDER_SPEC>`"]
pub type MDER = crate::Reg<mder::MDER_SPEC>;
#[doc = "Master DMA Enable Register"]
pub mod mder;
#[doc = "MCFGR0 register accessor: an alias for `Reg<MCFGR0_SPEC>`"]
pub type MCFGR0 = crate::Reg<mcfgr0::MCFGR0_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod mcfgr0;
#[doc = "MCFGR1 register accessor: an alias for `Reg<MCFGR1_SPEC>`"]
pub type MCFGR1 = crate::Reg<mcfgr1::MCFGR1_SPEC>;
#[doc = "Master Configuration Register 1"]
pub mod mcfgr1;
#[doc = "MCFGR2 register accessor: an alias for `Reg<MCFGR2_SPEC>`"]
pub type MCFGR2 = crate::Reg<mcfgr2::MCFGR2_SPEC>;
#[doc = "Master Configuration Register 2"]
pub mod mcfgr2;
#[doc = "MCFGR3 register accessor: an alias for `Reg<MCFGR3_SPEC>`"]
pub type MCFGR3 = crate::Reg<mcfgr3::MCFGR3_SPEC>;
#[doc = "Master Configuration Register 3"]
pub mod mcfgr3;
#[doc = "MDMR register accessor: an alias for `Reg<MDMR_SPEC>`"]
pub type MDMR = crate::Reg<mdmr::MDMR_SPEC>;
#[doc = "Master Data Match Register"]
pub mod mdmr;
#[doc = "MCCR0 register accessor: an alias for `Reg<MCCR0_SPEC>`"]
pub type MCCR0 = crate::Reg<mccr0::MCCR0_SPEC>;
#[doc = "Master Clock Configuration Register 0"]
pub mod mccr0;
#[doc = "MCCR1 register accessor: an alias for `Reg<MCCR1_SPEC>`"]
pub type MCCR1 = crate::Reg<mccr1::MCCR1_SPEC>;
#[doc = "Master Clock Configuration Register 1"]
pub mod mccr1;
#[doc = "MFCR register accessor: an alias for `Reg<MFCR_SPEC>`"]
pub type MFCR = crate::Reg<mfcr::MFCR_SPEC>;
#[doc = "Master FIFO Control Register"]
pub mod mfcr;
#[doc = "MFSR register accessor: an alias for `Reg<MFSR_SPEC>`"]
pub type MFSR = crate::Reg<mfsr::MFSR_SPEC>;
#[doc = "Master FIFO Status Register"]
pub mod mfsr;
#[doc = "MTDR register accessor: an alias for `Reg<MTDR_SPEC>`"]
pub type MTDR = crate::Reg<mtdr::MTDR_SPEC>;
#[doc = "Master Transmit Data Register"]
pub mod mtdr;
#[doc = "MRDR register accessor: an alias for `Reg<MRDR_SPEC>`"]
pub type MRDR = crate::Reg<mrdr::MRDR_SPEC>;
#[doc = "Master Receive Data Register"]
pub mod mrdr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Slave Control Register"]
pub mod scr;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "Slave Status Register"]
pub mod ssr;
#[doc = "SIER register accessor: an alias for `Reg<SIER_SPEC>`"]
pub type SIER = crate::Reg<sier::SIER_SPEC>;
#[doc = "Slave Interrupt Enable Register"]
pub mod sier;
#[doc = "SDER register accessor: an alias for `Reg<SDER_SPEC>`"]
pub type SDER = crate::Reg<sder::SDER_SPEC>;
#[doc = "Slave DMA Enable Register"]
pub mod sder;
#[doc = "SCFGR1 register accessor: an alias for `Reg<SCFGR1_SPEC>`"]
pub type SCFGR1 = crate::Reg<scfgr1::SCFGR1_SPEC>;
#[doc = "Slave Configuration Register 1"]
pub mod scfgr1;
#[doc = "SCFGR2 register accessor: an alias for `Reg<SCFGR2_SPEC>`"]
pub type SCFGR2 = crate::Reg<scfgr2::SCFGR2_SPEC>;
#[doc = "Slave Configuration Register 2"]
pub mod scfgr2;
#[doc = "SAMR register accessor: an alias for `Reg<SAMR_SPEC>`"]
pub type SAMR = crate::Reg<samr::SAMR_SPEC>;
#[doc = "Slave Address Match Register"]
pub mod samr;
#[doc = "SASR register accessor: an alias for `Reg<SASR_SPEC>`"]
pub type SASR = crate::Reg<sasr::SASR_SPEC>;
#[doc = "Slave Address Status Register"]
pub mod sasr;
#[doc = "STAR register accessor: an alias for `Reg<STAR_SPEC>`"]
pub type STAR = crate::Reg<star::STAR_SPEC>;
#[doc = "Slave Transmit ACK Register"]
pub mod star;
#[doc = "STDR register accessor: an alias for `Reg<STDR_SPEC>`"]
pub type STDR = crate::Reg<stdr::STDR_SPEC>;
#[doc = "Slave Transmit Data Register"]
pub mod stdr;
#[doc = "SRDR register accessor: an alias for `Reg<SRDR_SPEC>`"]
pub type SRDR = crate::Reg<srdr::SRDR_SPEC>;
#[doc = "Slave Receive Data Register"]
pub mod srdr;
