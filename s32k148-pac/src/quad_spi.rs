#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration Register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - IP Configuration Register"]
    pub ipcr: crate::Reg<ipcr::IPCR_SPEC>,
    #[doc = "0x0c - Flash Configuration Register"]
    pub flshcr: crate::Reg<flshcr::FLSHCR_SPEC>,
    #[doc = "0x10 - Buffer0 Configuration Register"]
    pub buf0cr: crate::Reg<buf0cr::BUF0CR_SPEC>,
    #[doc = "0x14 - Buffer1 Configuration Register"]
    pub buf1cr: crate::Reg<buf1cr::BUF1CR_SPEC>,
    #[doc = "0x18 - Buffer2 Configuration Register"]
    pub buf2cr: crate::Reg<buf2cr::BUF2CR_SPEC>,
    #[doc = "0x1c - Buffer3 Configuration Register"]
    pub buf3cr: crate::Reg<buf3cr::BUF3CR_SPEC>,
    #[doc = "0x20 - Buffer Generic Configuration Register"]
    pub bfgencr: crate::Reg<bfgencr::BFGENCR_SPEC>,
    #[doc = "0x24 - SOC Configuration Register"]
    pub soccr: crate::Reg<soccr::SOCCR_SPEC>,
    _reserved9: [u8; 0x08],
    #[doc = "0x30 - Buffer0 Top Index Register"]
    pub buf0ind: crate::Reg<buf0ind::BUF0IND_SPEC>,
    #[doc = "0x34 - Buffer1 Top Index Register"]
    pub buf1ind: crate::Reg<buf1ind::BUF1IND_SPEC>,
    #[doc = "0x38 - Buffer2 Top Index Register"]
    pub buf2ind: crate::Reg<buf2ind::BUF2IND_SPEC>,
    _reserved12: [u8; 0xc4],
    #[doc = "0x100 - Serial Flash Address Register"]
    pub sfar: crate::Reg<sfar::SFAR_SPEC>,
    #[doc = "0x104 - Serial Flash Address Configuration Register"]
    pub sfacr: crate::Reg<sfacr::SFACR_SPEC>,
    #[doc = "0x108 - Sampling Register"]
    pub smpr: crate::Reg<smpr::SMPR_SPEC>,
    #[doc = "0x10c - RX Buffer Status Register"]
    pub rbsr: crate::Reg<rbsr::RBSR_SPEC>,
    #[doc = "0x110 - RX Buffer Control Register"]
    pub rbct: crate::Reg<rbct::RBCT_SPEC>,
    _reserved17: [u8; 0x3c],
    #[doc = "0x150 - TX Buffer Status Register"]
    pub tbsr: crate::Reg<tbsr::TBSR_SPEC>,
    #[doc = "0x154 - TX Buffer Data Register"]
    pub tbdr: crate::Reg<tbdr::TBDR_SPEC>,
    #[doc = "0x158 - Tx Buffer Control Register"]
    pub tbct: crate::Reg<tbct::TBCT_SPEC>,
    #[doc = "0x15c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x160 - Flag Register"]
    pub fr: crate::Reg<fr::FR_SPEC>,
    #[doc = "0x164 - Interrupt and DMA Request Select and Enable Register"]
    pub rser: crate::Reg<rser::RSER_SPEC>,
    #[doc = "0x168 - Sequence Suspend Status Register"]
    pub spndst: crate::Reg<spndst::SPNDST_SPEC>,
    #[doc = "0x16c - Sequence Pointer Clear Register"]
    pub sptrclr: crate::Reg<sptrclr::SPTRCLR_SPEC>,
    _reserved25: [u8; 0x10],
    #[doc = "0x180 - Serial Flash A1 Top Address"]
    pub sfa1ad: crate::Reg<sfa1ad::SFA1AD_SPEC>,
    #[doc = "0x184 - Serial Flash A2 Top Address"]
    pub sfa2ad: crate::Reg<sfa2ad::SFA2AD_SPEC>,
    #[doc = "0x188 - Serial Flash B1 Top Address"]
    pub sfb1ad: crate::Reg<sfb1ad::SFB1AD_SPEC>,
    #[doc = "0x18c - Serial Flash B2 Top Address"]
    pub sfb2ad: crate::Reg<sfb2ad::SFB2AD_SPEC>,
    _reserved29: [u8; 0x70],
    #[doc = "0x200..0x280 - RX Buffer Data Register"]
    pub rbdr: [crate::Reg<rbdr::RBDR_SPEC>; 32],
    _reserved30: [u8; 0x80],
    #[doc = "0x300 - LUT Key Register"]
    pub lutkey: crate::Reg<lutkey::LUTKEY_SPEC>,
    #[doc = "0x304 - LUT Lock Configuration Register"]
    pub lckcr: crate::Reg<lckcr::LCKCR_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0x310..0x410 - Look-up Table register"]
    pub lut: [crate::Reg<lut::LUT_SPEC>; 64],
}
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration Register"]
pub mod mcr;
#[doc = "IPCR register accessor: an alias for `Reg<IPCR_SPEC>`"]
pub type IPCR = crate::Reg<ipcr::IPCR_SPEC>;
#[doc = "IP Configuration Register"]
pub mod ipcr;
#[doc = "FLSHCR register accessor: an alias for `Reg<FLSHCR_SPEC>`"]
pub type FLSHCR = crate::Reg<flshcr::FLSHCR_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod flshcr;
#[doc = "BUF0CR register accessor: an alias for `Reg<BUF0CR_SPEC>`"]
pub type BUF0CR = crate::Reg<buf0cr::BUF0CR_SPEC>;
#[doc = "Buffer0 Configuration Register"]
pub mod buf0cr;
#[doc = "BUF1CR register accessor: an alias for `Reg<BUF1CR_SPEC>`"]
pub type BUF1CR = crate::Reg<buf1cr::BUF1CR_SPEC>;
#[doc = "Buffer1 Configuration Register"]
pub mod buf1cr;
#[doc = "BUF2CR register accessor: an alias for `Reg<BUF2CR_SPEC>`"]
pub type BUF2CR = crate::Reg<buf2cr::BUF2CR_SPEC>;
#[doc = "Buffer2 Configuration Register"]
pub mod buf2cr;
#[doc = "BUF3CR register accessor: an alias for `Reg<BUF3CR_SPEC>`"]
pub type BUF3CR = crate::Reg<buf3cr::BUF3CR_SPEC>;
#[doc = "Buffer3 Configuration Register"]
pub mod buf3cr;
#[doc = "BFGENCR register accessor: an alias for `Reg<BFGENCR_SPEC>`"]
pub type BFGENCR = crate::Reg<bfgencr::BFGENCR_SPEC>;
#[doc = "Buffer Generic Configuration Register"]
pub mod bfgencr;
#[doc = "SOCCR register accessor: an alias for `Reg<SOCCR_SPEC>`"]
pub type SOCCR = crate::Reg<soccr::SOCCR_SPEC>;
#[doc = "SOC Configuration Register"]
pub mod soccr;
#[doc = "BUF0IND register accessor: an alias for `Reg<BUF0IND_SPEC>`"]
pub type BUF0IND = crate::Reg<buf0ind::BUF0IND_SPEC>;
#[doc = "Buffer0 Top Index Register"]
pub mod buf0ind;
#[doc = "BUF1IND register accessor: an alias for `Reg<BUF1IND_SPEC>`"]
pub type BUF1IND = crate::Reg<buf1ind::BUF1IND_SPEC>;
#[doc = "Buffer1 Top Index Register"]
pub mod buf1ind;
#[doc = "BUF2IND register accessor: an alias for `Reg<BUF2IND_SPEC>`"]
pub type BUF2IND = crate::Reg<buf2ind::BUF2IND_SPEC>;
#[doc = "Buffer2 Top Index Register"]
pub mod buf2ind;
#[doc = "SFAR register accessor: an alias for `Reg<SFAR_SPEC>`"]
pub type SFAR = crate::Reg<sfar::SFAR_SPEC>;
#[doc = "Serial Flash Address Register"]
pub mod sfar;
#[doc = "SFACR register accessor: an alias for `Reg<SFACR_SPEC>`"]
pub type SFACR = crate::Reg<sfacr::SFACR_SPEC>;
#[doc = "Serial Flash Address Configuration Register"]
pub mod sfacr;
#[doc = "SMPR register accessor: an alias for `Reg<SMPR_SPEC>`"]
pub type SMPR = crate::Reg<smpr::SMPR_SPEC>;
#[doc = "Sampling Register"]
pub mod smpr;
#[doc = "RBSR register accessor: an alias for `Reg<RBSR_SPEC>`"]
pub type RBSR = crate::Reg<rbsr::RBSR_SPEC>;
#[doc = "RX Buffer Status Register"]
pub mod rbsr;
#[doc = "RBCT register accessor: an alias for `Reg<RBCT_SPEC>`"]
pub type RBCT = crate::Reg<rbct::RBCT_SPEC>;
#[doc = "RX Buffer Control Register"]
pub mod rbct;
#[doc = "TBSR register accessor: an alias for `Reg<TBSR_SPEC>`"]
pub type TBSR = crate::Reg<tbsr::TBSR_SPEC>;
#[doc = "TX Buffer Status Register"]
pub mod tbsr;
#[doc = "TBDR register accessor: an alias for `Reg<TBDR_SPEC>`"]
pub type TBDR = crate::Reg<tbdr::TBDR_SPEC>;
#[doc = "TX Buffer Data Register"]
pub mod tbdr;
#[doc = "TBCT register accessor: an alias for `Reg<TBCT_SPEC>`"]
pub type TBCT = crate::Reg<tbct::TBCT_SPEC>;
#[doc = "Tx Buffer Control Register"]
pub mod tbct;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "FR register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Register"]
pub mod fr;
#[doc = "RSER register accessor: an alias for `Reg<RSER_SPEC>`"]
pub type RSER = crate::Reg<rser::RSER_SPEC>;
#[doc = "Interrupt and DMA Request Select and Enable Register"]
pub mod rser;
#[doc = "SPNDST register accessor: an alias for `Reg<SPNDST_SPEC>`"]
pub type SPNDST = crate::Reg<spndst::SPNDST_SPEC>;
#[doc = "Sequence Suspend Status Register"]
pub mod spndst;
#[doc = "SPTRCLR register accessor: an alias for `Reg<SPTRCLR_SPEC>`"]
pub type SPTRCLR = crate::Reg<sptrclr::SPTRCLR_SPEC>;
#[doc = "Sequence Pointer Clear Register"]
pub mod sptrclr;
#[doc = "SFA1AD register accessor: an alias for `Reg<SFA1AD_SPEC>`"]
pub type SFA1AD = crate::Reg<sfa1ad::SFA1AD_SPEC>;
#[doc = "Serial Flash A1 Top Address"]
pub mod sfa1ad;
#[doc = "SFA2AD register accessor: an alias for `Reg<SFA2AD_SPEC>`"]
pub type SFA2AD = crate::Reg<sfa2ad::SFA2AD_SPEC>;
#[doc = "Serial Flash A2 Top Address"]
pub mod sfa2ad;
#[doc = "SFB1AD register accessor: an alias for `Reg<SFB1AD_SPEC>`"]
pub type SFB1AD = crate::Reg<sfb1ad::SFB1AD_SPEC>;
#[doc = "Serial Flash B1 Top Address"]
pub mod sfb1ad;
#[doc = "SFB2AD register accessor: an alias for `Reg<SFB2AD_SPEC>`"]
pub type SFB2AD = crate::Reg<sfb2ad::SFB2AD_SPEC>;
#[doc = "Serial Flash B2 Top Address"]
pub mod sfb2ad;
#[doc = "RBDR register accessor: an alias for `Reg<RBDR_SPEC>`"]
pub type RBDR = crate::Reg<rbdr::RBDR_SPEC>;
#[doc = "RX Buffer Data Register"]
pub mod rbdr;
#[doc = "LUTKEY register accessor: an alias for `Reg<LUTKEY_SPEC>`"]
pub type LUTKEY = crate::Reg<lutkey::LUTKEY_SPEC>;
#[doc = "LUT Key Register"]
pub mod lutkey;
#[doc = "LCKCR register accessor: an alias for `Reg<LCKCR_SPEC>`"]
pub type LCKCR = crate::Reg<lckcr::LCKCR_SPEC>;
#[doc = "LUT Lock Configuration Register"]
pub mod lckcr;
#[doc = "LUT register accessor: an alias for `Reg<LUT_SPEC>`"]
pub type LUT = crate::Reg<lut::LUT_SPEC>;
#[doc = "Look-up Table register"]
pub mod lut;
