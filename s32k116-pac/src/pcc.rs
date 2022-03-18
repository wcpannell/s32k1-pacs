#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    #[doc = "0x80 - PCC FTFC Register"]
    pub pcc_ftfc: crate::Reg<pcc_ftfc::PCC_FTFC_SPEC>,
    #[doc = "0x84 - PCC DMAMUX Register"]
    pub pcc_dmamux: crate::Reg<pcc_dmamux::PCC_DMAMUX_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x90 - PCC FlexCAN0 Register"]
    pub pcc_flex_can0: crate::Reg<pcc_flex_can0::PCC_FLEXCAN0_SPEC>,
    _reserved3: [u8; 0x1c],
    #[doc = "0xb0 - PCC LPSPI0 Register"]
    pub pcc_lpspi0: crate::Reg<pcc_lpspi0::PCC_LPSPI0_SPEC>,
    _reserved4: [u8; 0x14],
    #[doc = "0xc8 - PCC CRC Register"]
    pub pcc_crc: crate::Reg<pcc_crc::PCC_CRC_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0xd8 - PCC PDB0 Register"]
    pub pcc_pdb0: crate::Reg<pcc_pdb0::PCC_PDB0_SPEC>,
    #[doc = "0xdc - PCC LPIT Register"]
    pub pcc_lpit: crate::Reg<pcc_lpit::PCC_LPIT_SPEC>,
    #[doc = "0xe0 - PCC FTM0 Register"]
    pub pcc_ftm0: crate::Reg<pcc_ftm0::PCC_FTM0_SPEC>,
    #[doc = "0xe4 - PCC FTM1 Register"]
    pub pcc_ftm1: crate::Reg<pcc_ftm1::PCC_FTM1_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xec - PCC ADC0 Register"]
    pub pcc_adc0: crate::Reg<pcc_adc0::PCC_ADC0_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0xf4 - PCC RTC Register"]
    pub pcc_rtc: crate::Reg<pcc_rtc::PCC_RTC_SPEC>,
    #[doc = "0xf8 - PCC CMU0 Register"]
    pub pcc_cmu0: crate::Reg<pcc_cmu0::PCC_CMU0_SPEC>,
    #[doc = "0xfc - PCC CMU1 Register"]
    pub pcc_cmu1: crate::Reg<pcc_cmu1::PCC_CMU1_SPEC>,
    #[doc = "0x100 - PCC LPTMR0 Register"]
    pub pcc_lptmr0: crate::Reg<pcc_lptmr0::PCC_LPTMR0_SPEC>,
    _reserved14: [u8; 0x20],
    #[doc = "0x124 - PCC PORTA Register"]
    pub pcc_porta: crate::Reg<pcc_porta::PCC_PORTA_SPEC>,
    #[doc = "0x128 - PCC PORTB Register"]
    pub pcc_portb: crate::Reg<pcc_portb::PCC_PORTB_SPEC>,
    #[doc = "0x12c - PCC PORTC Register"]
    pub pcc_portc: crate::Reg<pcc_portc::PCC_PORTC_SPEC>,
    #[doc = "0x130 - PCC PORTD Register"]
    pub pcc_portd: crate::Reg<pcc_portd::PCC_PORTD_SPEC>,
    #[doc = "0x134 - PCC PORTE Register"]
    pub pcc_porte: crate::Reg<pcc_porte::PCC_PORTE_SPEC>,
    _reserved19: [u8; 0x30],
    #[doc = "0x168 - PCC FlexIO Register"]
    pub pcc_flex_io: crate::Reg<pcc_flex_io::PCC_FLEXIO_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0x198 - PCC LPI2C0 Register"]
    pub pcc_lpi2c0: crate::Reg<pcc_lpi2c0::PCC_LPI2C0_SPEC>,
    _reserved21: [u8; 0x0c],
    #[doc = "0x1a8 - PCC LPUART0 Register"]
    pub pcc_lpuart0: crate::Reg<pcc_lpuart0::PCC_LPUART0_SPEC>,
    #[doc = "0x1ac - PCC LPUART1 Register"]
    pub pcc_lpuart1: crate::Reg<pcc_lpuart1::PCC_LPUART1_SPEC>,
    _reserved23: [u8; 0x1c],
    #[doc = "0x1cc - PCC CMP0 Register"]
    pub pcc_cmp0: crate::Reg<pcc_cmp0::PCC_CMP0_SPEC>,
}
#[doc = "PCC_FTFC register accessor: an alias for `Reg<PCC_FTFC_SPEC>`"]
pub type PCC_FTFC = crate::Reg<pcc_ftfc::PCC_FTFC_SPEC>;
#[doc = "PCC FTFC Register"]
pub mod pcc_ftfc;
#[doc = "PCC_DMAMUX register accessor: an alias for `Reg<PCC_DMAMUX_SPEC>`"]
pub type PCC_DMAMUX = crate::Reg<pcc_dmamux::PCC_DMAMUX_SPEC>;
#[doc = "PCC DMAMUX Register"]
pub mod pcc_dmamux;
#[doc = "PCC_FlexCAN0 register accessor: an alias for `Reg<PCC_FLEXCAN0_SPEC>`"]
pub type PCC_FLEXCAN0 = crate::Reg<pcc_flex_can0::PCC_FLEXCAN0_SPEC>;
#[doc = "PCC FlexCAN0 Register"]
pub mod pcc_flex_can0;
#[doc = "PCC_LPSPI0 register accessor: an alias for `Reg<PCC_LPSPI0_SPEC>`"]
pub type PCC_LPSPI0 = crate::Reg<pcc_lpspi0::PCC_LPSPI0_SPEC>;
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC_CRC register accessor: an alias for `Reg<PCC_CRC_SPEC>`"]
pub type PCC_CRC = crate::Reg<pcc_crc::PCC_CRC_SPEC>;
#[doc = "PCC CRC Register"]
pub mod pcc_crc;
#[doc = "PCC_PDB0 register accessor: an alias for `Reg<PCC_PDB0_SPEC>`"]
pub type PCC_PDB0 = crate::Reg<pcc_pdb0::PCC_PDB0_SPEC>;
#[doc = "PCC PDB0 Register"]
pub mod pcc_pdb0;
#[doc = "PCC_LPIT register accessor: an alias for `Reg<PCC_LPIT_SPEC>`"]
pub type PCC_LPIT = crate::Reg<pcc_lpit::PCC_LPIT_SPEC>;
#[doc = "PCC LPIT Register"]
pub mod pcc_lpit;
#[doc = "PCC_FTM0 register accessor: an alias for `Reg<PCC_FTM0_SPEC>`"]
pub type PCC_FTM0 = crate::Reg<pcc_ftm0::PCC_FTM0_SPEC>;
#[doc = "PCC FTM0 Register"]
pub mod pcc_ftm0;
#[doc = "PCC_FTM1 register accessor: an alias for `Reg<PCC_FTM1_SPEC>`"]
pub type PCC_FTM1 = crate::Reg<pcc_ftm1::PCC_FTM1_SPEC>;
#[doc = "PCC FTM1 Register"]
pub mod pcc_ftm1;
#[doc = "PCC_ADC0 register accessor: an alias for `Reg<PCC_ADC0_SPEC>`"]
pub type PCC_ADC0 = crate::Reg<pcc_adc0::PCC_ADC0_SPEC>;
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC_RTC register accessor: an alias for `Reg<PCC_RTC_SPEC>`"]
pub type PCC_RTC = crate::Reg<pcc_rtc::PCC_RTC_SPEC>;
#[doc = "PCC RTC Register"]
pub mod pcc_rtc;
#[doc = "PCC_CMU0 register accessor: an alias for `Reg<PCC_CMU0_SPEC>`"]
pub type PCC_CMU0 = crate::Reg<pcc_cmu0::PCC_CMU0_SPEC>;
#[doc = "PCC CMU0 Register"]
pub mod pcc_cmu0;
#[doc = "PCC_CMU1 register accessor: an alias for `Reg<PCC_CMU1_SPEC>`"]
pub type PCC_CMU1 = crate::Reg<pcc_cmu1::PCC_CMU1_SPEC>;
#[doc = "PCC CMU1 Register"]
pub mod pcc_cmu1;
#[doc = "PCC_LPTMR0 register accessor: an alias for `Reg<PCC_LPTMR0_SPEC>`"]
pub type PCC_LPTMR0 = crate::Reg<pcc_lptmr0::PCC_LPTMR0_SPEC>;
#[doc = "PCC LPTMR0 Register"]
pub mod pcc_lptmr0;
#[doc = "PCC_PORTA register accessor: an alias for `Reg<PCC_PORTA_SPEC>`"]
pub type PCC_PORTA = crate::Reg<pcc_porta::PCC_PORTA_SPEC>;
#[doc = "PCC PORTA Register"]
pub mod pcc_porta;
#[doc = "PCC_PORTB register accessor: an alias for `Reg<PCC_PORTB_SPEC>`"]
pub type PCC_PORTB = crate::Reg<pcc_portb::PCC_PORTB_SPEC>;
#[doc = "PCC PORTB Register"]
pub mod pcc_portb;
#[doc = "PCC_PORTC register accessor: an alias for `Reg<PCC_PORTC_SPEC>`"]
pub type PCC_PORTC = crate::Reg<pcc_portc::PCC_PORTC_SPEC>;
#[doc = "PCC PORTC Register"]
pub mod pcc_portc;
#[doc = "PCC_PORTD register accessor: an alias for `Reg<PCC_PORTD_SPEC>`"]
pub type PCC_PORTD = crate::Reg<pcc_portd::PCC_PORTD_SPEC>;
#[doc = "PCC PORTD Register"]
pub mod pcc_portd;
#[doc = "PCC_PORTE register accessor: an alias for `Reg<PCC_PORTE_SPEC>`"]
pub type PCC_PORTE = crate::Reg<pcc_porte::PCC_PORTE_SPEC>;
#[doc = "PCC PORTE Register"]
pub mod pcc_porte;
#[doc = "PCC_FlexIO register accessor: an alias for `Reg<PCC_FLEXIO_SPEC>`"]
pub type PCC_FLEXIO = crate::Reg<pcc_flex_io::PCC_FLEXIO_SPEC>;
#[doc = "PCC FlexIO Register"]
pub mod pcc_flex_io;
#[doc = "PCC_LPI2C0 register accessor: an alias for `Reg<PCC_LPI2C0_SPEC>`"]
pub type PCC_LPI2C0 = crate::Reg<pcc_lpi2c0::PCC_LPI2C0_SPEC>;
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC_LPUART0 register accessor: an alias for `Reg<PCC_LPUART0_SPEC>`"]
pub type PCC_LPUART0 = crate::Reg<pcc_lpuart0::PCC_LPUART0_SPEC>;
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC_LPUART1 register accessor: an alias for `Reg<PCC_LPUART1_SPEC>`"]
pub type PCC_LPUART1 = crate::Reg<pcc_lpuart1::PCC_LPUART1_SPEC>;
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC_CMP0 register accessor: an alias for `Reg<PCC_CMP0_SPEC>`"]
pub type PCC_CMP0 = crate::Reg<pcc_cmp0::PCC_CMP0_SPEC>;
#[doc = "PCC CMP0 Register"]
pub mod pcc_cmp0;
