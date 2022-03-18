#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRGMUX DMAMUX0 Register"]
    pub dmamux0: crate::Reg<dmamux0::DMAMUX0_SPEC>,
    #[doc = "0x04 - TRGMUX EXTOUT0 Register"]
    pub extout0: crate::Reg<extout0::EXTOUT0_SPEC>,
    #[doc = "0x08 - TRGMUX EXTOUT1 Register"]
    pub extout1: crate::Reg<extout1::EXTOUT1_SPEC>,
    #[doc = "0x0c - TRGMUX ADC0 Register"]
    pub adc0: crate::Reg<adc0::ADC0_SPEC>,
    #[doc = "0x10 - TRGMUX ADC1 Register"]
    pub adc1: crate::Reg<adc1::ADC1_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - TRGMUX CMP0 Register"]
    pub cmp0: crate::Reg<cmp0::CMP0_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x28 - TRGMUX FTM0 Register"]
    pub ftm0: crate::Reg<ftm0::FTM0_SPEC>,
    #[doc = "0x2c - TRGMUX FTM1 Register"]
    pub ftm1: crate::Reg<ftm1::FTM1_SPEC>,
    #[doc = "0x30 - TRGMUX FTM2 Register"]
    pub ftm2: crate::Reg<ftm2::FTM2_SPEC>,
    #[doc = "0x34 - TRGMUX FTM3 Register"]
    pub ftm3: crate::Reg<ftm3::FTM3_SPEC>,
    #[doc = "0x38 - TRGMUX PDB0 Register"]
    pub pdb0: crate::Reg<pdb0::PDB0_SPEC>,
    #[doc = "0x3c - TRGMUX PDB1 Register"]
    pub pdb1: crate::Reg<pdb1::PDB1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x44 - TRGMUX FLEXIO Register"]
    pub flexio: crate::Reg<flexio::FLEXIO_SPEC>,
    #[doc = "0x48 - TRGMUX LPIT0 Register"]
    pub lpit0: crate::Reg<lpit0::LPIT0_SPEC>,
    #[doc = "0x4c - TRGMUX LPUART0 Register"]
    pub lpuart0: crate::Reg<lpuart0::LPUART0_SPEC>,
    #[doc = "0x50 - TRGMUX LPUART1 Register"]
    pub lpuart1: crate::Reg<lpuart1::LPUART1_SPEC>,
    #[doc = "0x54 - TRGMUX LPI2C0 Register"]
    pub lpi2c0: crate::Reg<lpi2c0::LPI2C0_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x5c - TRGMUX LPSPI0 Register"]
    pub lpspi0: crate::Reg<lpspi0::LPSPI0_SPEC>,
    #[doc = "0x60 - TRGMUX LPSPI1 Register"]
    pub lpspi1: crate::Reg<lpspi1::LPSPI1_SPEC>,
    #[doc = "0x64 - TRGMUX LPTMR0 Register"]
    pub lptmr0: crate::Reg<lptmr0::LPTMR0_SPEC>,
}
#[doc = "DMAMUX0 register accessor: an alias for `Reg<DMAMUX0_SPEC>`"]
pub type DMAMUX0 = crate::Reg<dmamux0::DMAMUX0_SPEC>;
#[doc = "TRGMUX DMAMUX0 Register"]
pub mod dmamux0;
#[doc = "EXTOUT0 register accessor: an alias for `Reg<EXTOUT0_SPEC>`"]
pub type EXTOUT0 = crate::Reg<extout0::EXTOUT0_SPEC>;
#[doc = "TRGMUX EXTOUT0 Register"]
pub mod extout0;
#[doc = "EXTOUT1 register accessor: an alias for `Reg<EXTOUT1_SPEC>`"]
pub type EXTOUT1 = crate::Reg<extout1::EXTOUT1_SPEC>;
#[doc = "TRGMUX EXTOUT1 Register"]
pub mod extout1;
#[doc = "ADC0 register accessor: an alias for `Reg<ADC0_SPEC>`"]
pub type ADC0 = crate::Reg<adc0::ADC0_SPEC>;
#[doc = "TRGMUX ADC0 Register"]
pub mod adc0;
#[doc = "ADC1 register accessor: an alias for `Reg<ADC1_SPEC>`"]
pub type ADC1 = crate::Reg<adc1::ADC1_SPEC>;
#[doc = "TRGMUX ADC1 Register"]
pub mod adc1;
#[doc = "CMP0 register accessor: an alias for `Reg<CMP0_SPEC>`"]
pub type CMP0 = crate::Reg<cmp0::CMP0_SPEC>;
#[doc = "TRGMUX CMP0 Register"]
pub mod cmp0;
#[doc = "FTM0 register accessor: an alias for `Reg<FTM0_SPEC>`"]
pub type FTM0 = crate::Reg<ftm0::FTM0_SPEC>;
#[doc = "TRGMUX FTM0 Register"]
pub mod ftm0;
#[doc = "FTM1 register accessor: an alias for `Reg<FTM1_SPEC>`"]
pub type FTM1 = crate::Reg<ftm1::FTM1_SPEC>;
#[doc = "TRGMUX FTM1 Register"]
pub mod ftm1;
#[doc = "FTM2 register accessor: an alias for `Reg<FTM2_SPEC>`"]
pub type FTM2 = crate::Reg<ftm2::FTM2_SPEC>;
#[doc = "TRGMUX FTM2 Register"]
pub mod ftm2;
#[doc = "FTM3 register accessor: an alias for `Reg<FTM3_SPEC>`"]
pub type FTM3 = crate::Reg<ftm3::FTM3_SPEC>;
#[doc = "TRGMUX FTM3 Register"]
pub mod ftm3;
#[doc = "PDB0 register accessor: an alias for `Reg<PDB0_SPEC>`"]
pub type PDB0 = crate::Reg<pdb0::PDB0_SPEC>;
#[doc = "TRGMUX PDB0 Register"]
pub mod pdb0;
#[doc = "PDB1 register accessor: an alias for `Reg<PDB1_SPEC>`"]
pub type PDB1 = crate::Reg<pdb1::PDB1_SPEC>;
#[doc = "TRGMUX PDB1 Register"]
pub mod pdb1;
#[doc = "FLEXIO register accessor: an alias for `Reg<FLEXIO_SPEC>`"]
pub type FLEXIO = crate::Reg<flexio::FLEXIO_SPEC>;
#[doc = "TRGMUX FLEXIO Register"]
pub mod flexio;
#[doc = "LPIT0 register accessor: an alias for `Reg<LPIT0_SPEC>`"]
pub type LPIT0 = crate::Reg<lpit0::LPIT0_SPEC>;
#[doc = "TRGMUX LPIT0 Register"]
pub mod lpit0;
#[doc = "LPUART0 register accessor: an alias for `Reg<LPUART0_SPEC>`"]
pub type LPUART0 = crate::Reg<lpuart0::LPUART0_SPEC>;
#[doc = "TRGMUX LPUART0 Register"]
pub mod lpuart0;
#[doc = "LPUART1 register accessor: an alias for `Reg<LPUART1_SPEC>`"]
pub type LPUART1 = crate::Reg<lpuart1::LPUART1_SPEC>;
#[doc = "TRGMUX LPUART1 Register"]
pub mod lpuart1;
#[doc = "LPI2C0 register accessor: an alias for `Reg<LPI2C0_SPEC>`"]
pub type LPI2C0 = crate::Reg<lpi2c0::LPI2C0_SPEC>;
#[doc = "TRGMUX LPI2C0 Register"]
pub mod lpi2c0;
#[doc = "LPSPI0 register accessor: an alias for `Reg<LPSPI0_SPEC>`"]
pub type LPSPI0 = crate::Reg<lpspi0::LPSPI0_SPEC>;
#[doc = "TRGMUX LPSPI0 Register"]
pub mod lpspi0;
#[doc = "LPSPI1 register accessor: an alias for `Reg<LPSPI1_SPEC>`"]
pub type LPSPI1 = crate::Reg<lpspi1::LPSPI1_SPEC>;
#[doc = "TRGMUX LPSPI1 Register"]
pub mod lpspi1;
#[doc = "LPTMR0 register accessor: an alias for `Reg<LPTMR0_SPEC>`"]
pub type LPTMR0 = crate::Reg<lptmr0::LPTMR0_SPEC>;
#[doc = "TRGMUX LPTMR0 Register"]
pub mod lptmr0;
