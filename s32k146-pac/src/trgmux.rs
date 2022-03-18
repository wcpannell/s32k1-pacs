#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRGMUX DMAMUX0 Register"]
    pub trgmux_dmamux0: crate::Reg<trgmux_dmamux0::TRGMUX_DMAMUX0_SPEC>,
    #[doc = "0x04 - TRGMUX EXTOUT0 Register"]
    pub trgmux_extout0: crate::Reg<trgmux_extout0::TRGMUX_EXTOUT0_SPEC>,
    #[doc = "0x08 - TRGMUX EXTOUT1 Register"]
    pub trgmux_extout1: crate::Reg<trgmux_extout1::TRGMUX_EXTOUT1_SPEC>,
    #[doc = "0x0c - TRGMUX ADC0 Register"]
    pub trgmux_adc0: crate::Reg<trgmux_adc0::TRGMUX_ADC0_SPEC>,
    #[doc = "0x10 - TRGMUX ADC1 Register"]
    pub trgmux_adc1: crate::Reg<trgmux_adc1::TRGMUX_ADC1_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x1c - TRGMUX CMP0 Register"]
    pub trgmux_cmp0: crate::Reg<trgmux_cmp0::TRGMUX_CMP0_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x28 - TRGMUX FTM0 Register"]
    pub trgmux_ftm0: crate::Reg<trgmux_ftm0::TRGMUX_FTM0_SPEC>,
    #[doc = "0x2c - TRGMUX FTM1 Register"]
    pub trgmux_ftm1: crate::Reg<trgmux_ftm1::TRGMUX_FTM1_SPEC>,
    #[doc = "0x30 - TRGMUX FTM2 Register"]
    pub trgmux_ftm2: crate::Reg<trgmux_ftm2::TRGMUX_FTM2_SPEC>,
    #[doc = "0x34 - TRGMUX FTM3 Register"]
    pub trgmux_ftm3: crate::Reg<trgmux_ftm3::TRGMUX_FTM3_SPEC>,
    #[doc = "0x38 - TRGMUX PDB0 Register"]
    pub trgmux_pdb0: crate::Reg<trgmux_pdb0::TRGMUX_PDB0_SPEC>,
    #[doc = "0x3c - TRGMUX PDB1 Register"]
    pub trgmux_pdb1: crate::Reg<trgmux_pdb1::TRGMUX_PDB1_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x44 - TRGMUX FLEXIO Register"]
    pub trgmux_flexio: crate::Reg<trgmux_flexio::TRGMUX_FLEXIO_SPEC>,
    #[doc = "0x48 - TRGMUX LPIT0 Register"]
    pub trgmux_lpit0: crate::Reg<trgmux_lpit0::TRGMUX_LPIT0_SPEC>,
    #[doc = "0x4c - TRGMUX LPUART0 Register"]
    pub trgmux_lpuart0: crate::Reg<trgmux_lpuart0::TRGMUX_LPUART0_SPEC>,
    #[doc = "0x50 - TRGMUX LPUART1 Register"]
    pub trgmux_lpuart1: crate::Reg<trgmux_lpuart1::TRGMUX_LPUART1_SPEC>,
    #[doc = "0x54 - TRGMUX LPI2C0 Register"]
    pub trgmux_lpi2c0: crate::Reg<trgmux_lpi2c0::TRGMUX_LPI2C0_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x5c - TRGMUX LPSPI0 Register"]
    pub trgmux_lpspi0: crate::Reg<trgmux_lpspi0::TRGMUX_LPSPI0_SPEC>,
    #[doc = "0x60 - TRGMUX LPSPI1 Register"]
    pub trgmux_lpspi1: crate::Reg<trgmux_lpspi1::TRGMUX_LPSPI1_SPEC>,
    #[doc = "0x64 - TRGMUX LPTMR0 Register"]
    pub trgmux_lptmr0: crate::Reg<trgmux_lptmr0::TRGMUX_LPTMR0_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0x70 - TRGMUX FTM4 Register"]
    pub trgmux_ftm4: crate::Reg<trgmux_ftm4::TRGMUX_FTM4_SPEC>,
    #[doc = "0x74 - TRGMUX FTM5 Register"]
    pub trgmux_ftm5: crate::Reg<trgmux_ftm5::TRGMUX_FTM5_SPEC>,
}
#[doc = "TRGMUX_DMAMUX0 register accessor: an alias for `Reg<TRGMUX_DMAMUX0_SPEC>`"]
pub type TRGMUX_DMAMUX0 = crate::Reg<trgmux_dmamux0::TRGMUX_DMAMUX0_SPEC>;
#[doc = "TRGMUX DMAMUX0 Register"]
pub mod trgmux_dmamux0;
#[doc = "TRGMUX_EXTOUT0 register accessor: an alias for `Reg<TRGMUX_EXTOUT0_SPEC>`"]
pub type TRGMUX_EXTOUT0 = crate::Reg<trgmux_extout0::TRGMUX_EXTOUT0_SPEC>;
#[doc = "TRGMUX EXTOUT0 Register"]
pub mod trgmux_extout0;
#[doc = "TRGMUX_EXTOUT1 register accessor: an alias for `Reg<TRGMUX_EXTOUT1_SPEC>`"]
pub type TRGMUX_EXTOUT1 = crate::Reg<trgmux_extout1::TRGMUX_EXTOUT1_SPEC>;
#[doc = "TRGMUX EXTOUT1 Register"]
pub mod trgmux_extout1;
#[doc = "TRGMUX_ADC0 register accessor: an alias for `Reg<TRGMUX_ADC0_SPEC>`"]
pub type TRGMUX_ADC0 = crate::Reg<trgmux_adc0::TRGMUX_ADC0_SPEC>;
#[doc = "TRGMUX ADC0 Register"]
pub mod trgmux_adc0;
#[doc = "TRGMUX_ADC1 register accessor: an alias for `Reg<TRGMUX_ADC1_SPEC>`"]
pub type TRGMUX_ADC1 = crate::Reg<trgmux_adc1::TRGMUX_ADC1_SPEC>;
#[doc = "TRGMUX ADC1 Register"]
pub mod trgmux_adc1;
#[doc = "TRGMUX_CMP0 register accessor: an alias for `Reg<TRGMUX_CMP0_SPEC>`"]
pub type TRGMUX_CMP0 = crate::Reg<trgmux_cmp0::TRGMUX_CMP0_SPEC>;
#[doc = "TRGMUX CMP0 Register"]
pub mod trgmux_cmp0;
#[doc = "TRGMUX_FTM0 register accessor: an alias for `Reg<TRGMUX_FTM0_SPEC>`"]
pub type TRGMUX_FTM0 = crate::Reg<trgmux_ftm0::TRGMUX_FTM0_SPEC>;
#[doc = "TRGMUX FTM0 Register"]
pub mod trgmux_ftm0;
#[doc = "TRGMUX_FTM1 register accessor: an alias for `Reg<TRGMUX_FTM1_SPEC>`"]
pub type TRGMUX_FTM1 = crate::Reg<trgmux_ftm1::TRGMUX_FTM1_SPEC>;
#[doc = "TRGMUX FTM1 Register"]
pub mod trgmux_ftm1;
#[doc = "TRGMUX_FTM2 register accessor: an alias for `Reg<TRGMUX_FTM2_SPEC>`"]
pub type TRGMUX_FTM2 = crate::Reg<trgmux_ftm2::TRGMUX_FTM2_SPEC>;
#[doc = "TRGMUX FTM2 Register"]
pub mod trgmux_ftm2;
#[doc = "TRGMUX_FTM3 register accessor: an alias for `Reg<TRGMUX_FTM3_SPEC>`"]
pub type TRGMUX_FTM3 = crate::Reg<trgmux_ftm3::TRGMUX_FTM3_SPEC>;
#[doc = "TRGMUX FTM3 Register"]
pub mod trgmux_ftm3;
#[doc = "TRGMUX_PDB0 register accessor: an alias for `Reg<TRGMUX_PDB0_SPEC>`"]
pub type TRGMUX_PDB0 = crate::Reg<trgmux_pdb0::TRGMUX_PDB0_SPEC>;
#[doc = "TRGMUX PDB0 Register"]
pub mod trgmux_pdb0;
#[doc = "TRGMUX_PDB1 register accessor: an alias for `Reg<TRGMUX_PDB1_SPEC>`"]
pub type TRGMUX_PDB1 = crate::Reg<trgmux_pdb1::TRGMUX_PDB1_SPEC>;
#[doc = "TRGMUX PDB1 Register"]
pub mod trgmux_pdb1;
#[doc = "TRGMUX_FLEXIO register accessor: an alias for `Reg<TRGMUX_FLEXIO_SPEC>`"]
pub type TRGMUX_FLEXIO = crate::Reg<trgmux_flexio::TRGMUX_FLEXIO_SPEC>;
#[doc = "TRGMUX FLEXIO Register"]
pub mod trgmux_flexio;
#[doc = "TRGMUX_LPIT0 register accessor: an alias for `Reg<TRGMUX_LPIT0_SPEC>`"]
pub type TRGMUX_LPIT0 = crate::Reg<trgmux_lpit0::TRGMUX_LPIT0_SPEC>;
#[doc = "TRGMUX LPIT0 Register"]
pub mod trgmux_lpit0;
#[doc = "TRGMUX_LPUART0 register accessor: an alias for `Reg<TRGMUX_LPUART0_SPEC>`"]
pub type TRGMUX_LPUART0 = crate::Reg<trgmux_lpuart0::TRGMUX_LPUART0_SPEC>;
#[doc = "TRGMUX LPUART0 Register"]
pub mod trgmux_lpuart0;
#[doc = "TRGMUX_LPUART1 register accessor: an alias for `Reg<TRGMUX_LPUART1_SPEC>`"]
pub type TRGMUX_LPUART1 = crate::Reg<trgmux_lpuart1::TRGMUX_LPUART1_SPEC>;
#[doc = "TRGMUX LPUART1 Register"]
pub mod trgmux_lpuart1;
#[doc = "TRGMUX_LPI2C0 register accessor: an alias for `Reg<TRGMUX_LPI2C0_SPEC>`"]
pub type TRGMUX_LPI2C0 = crate::Reg<trgmux_lpi2c0::TRGMUX_LPI2C0_SPEC>;
#[doc = "TRGMUX LPI2C0 Register"]
pub mod trgmux_lpi2c0;
#[doc = "TRGMUX_LPSPI0 register accessor: an alias for `Reg<TRGMUX_LPSPI0_SPEC>`"]
pub type TRGMUX_LPSPI0 = crate::Reg<trgmux_lpspi0::TRGMUX_LPSPI0_SPEC>;
#[doc = "TRGMUX LPSPI0 Register"]
pub mod trgmux_lpspi0;
#[doc = "TRGMUX_LPSPI1 register accessor: an alias for `Reg<TRGMUX_LPSPI1_SPEC>`"]
pub type TRGMUX_LPSPI1 = crate::Reg<trgmux_lpspi1::TRGMUX_LPSPI1_SPEC>;
#[doc = "TRGMUX LPSPI1 Register"]
pub mod trgmux_lpspi1;
#[doc = "TRGMUX_LPTMR0 register accessor: an alias for `Reg<TRGMUX_LPTMR0_SPEC>`"]
pub type TRGMUX_LPTMR0 = crate::Reg<trgmux_lptmr0::TRGMUX_LPTMR0_SPEC>;
#[doc = "TRGMUX LPTMR0 Register"]
pub mod trgmux_lptmr0;
#[doc = "TRGMUX_FTM4 register accessor: an alias for `Reg<TRGMUX_FTM4_SPEC>`"]
pub type TRGMUX_FTM4 = crate::Reg<trgmux_ftm4::TRGMUX_FTM4_SPEC>;
#[doc = "TRGMUX FTM4 Register"]
pub mod trgmux_ftm4;
#[doc = "TRGMUX_FTM5 register accessor: an alias for `Reg<TRGMUX_FTM5_SPEC>`"]
pub type TRGMUX_FTM5 = crate::Reg<trgmux_ftm5::TRGMUX_FTM5_SPEC>;
#[doc = "TRGMUX FTM5 Register"]
pub mod trgmux_ftm5;
