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
    #[doc = "0x94 - PCC FlexCAN1 Register"]
    pub pcc_flex_can1: crate::Reg<pcc_flex_can1::PCC_FLEXCAN1_SPEC>,
    #[doc = "0x98 - PCC FTM3 Register"]
    pub pcc_ftm3: crate::Reg<pcc_ftm3::PCC_FTM3_SPEC>,
    #[doc = "0x9c - PCC ADC1 Register"]
    pub pcc_adc1: crate::Reg<pcc_adc1::PCC_ADC1_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0xac - PCC FlexCAN2 Register"]
    pub pcc_flex_can2: crate::Reg<pcc_flex_can2::PCC_FLEXCAN2_SPEC>,
    #[doc = "0xb0 - PCC LPSPI0 Register"]
    pub pcc_lpspi0: crate::Reg<pcc_lpspi0::PCC_LPSPI0_SPEC>,
    #[doc = "0xb4 - PCC LPSPI1 Register"]
    pub pcc_lpspi1: crate::Reg<pcc_lpspi1::PCC_LPSPI1_SPEC>,
    #[doc = "0xb8 - PCC LPSPI2 Register"]
    pub pcc_lpspi2: crate::Reg<pcc_lpspi2::PCC_LPSPI2_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0xc4 - PCC PDB1 Register"]
    pub pcc_pdb1: crate::Reg<pcc_pdb1::PCC_PDB1_SPEC>,
    #[doc = "0xc8 - PCC CRC Register"]
    pub pcc_crc: crate::Reg<pcc_crc::PCC_CRC_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0xd8 - PCC PDB0 Register"]
    pub pcc_pdb0: crate::Reg<pcc_pdb0::PCC_PDB0_SPEC>,
    #[doc = "0xdc - PCC LPIT Register"]
    pub pcc_lpit: crate::Reg<pcc_lpit::PCC_LPIT_SPEC>,
    #[doc = "0xe0 - PCC FTM0 Register"]
    pub pcc_ftm0: crate::Reg<pcc_ftm0::PCC_FTM0_SPEC>,
    #[doc = "0xe4 - PCC FTM1 Register"]
    pub pcc_ftm1: crate::Reg<pcc_ftm1::PCC_FTM1_SPEC>,
    #[doc = "0xe8 - PCC FTM2 Register"]
    pub pcc_ftm2: crate::Reg<pcc_ftm2::PCC_FTM2_SPEC>,
    #[doc = "0xec - PCC ADC0 Register"]
    pub pcc_adc0: crate::Reg<pcc_adc0::PCC_ADC0_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0xf4 - PCC RTC Register"]
    pub pcc_rtc: crate::Reg<pcc_rtc::PCC_RTC_SPEC>,
    _reserved19: [u8; 0x08],
    #[doc = "0x100 - PCC LPTMR0 Register"]
    pub pcc_lptmr0: crate::Reg<pcc_lptmr0::PCC_LPTMR0_SPEC>,
    _reserved20: [u8; 0x20],
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
    _reserved25: [u8; 0x18],
    #[doc = "0x150 - PCC SAI0 Register"]
    pub pcc_sai0: crate::Reg<pcc_sai0::PCC_SAI0_SPEC>,
    #[doc = "0x154 - PCC SAI1 Register"]
    pub pcc_sai1: crate::Reg<pcc_sai1::PCC_SAI1_SPEC>,
    _reserved27: [u8; 0x10],
    #[doc = "0x168 - PCC FlexIO Register"]
    pub pcc_flex_io: crate::Reg<pcc_flex_io::PCC_FLEXIO_SPEC>,
    _reserved28: [u8; 0x18],
    #[doc = "0x184 - PCC EWM Register"]
    pub pcc_ewm: crate::Reg<pcc_ewm::PCC_EWM_SPEC>,
    _reserved29: [u8; 0x10],
    #[doc = "0x198 - PCC LPI2C0 Register"]
    pub pcc_lpi2c0: crate::Reg<pcc_lpi2c0::PCC_LPI2C0_SPEC>,
    #[doc = "0x19c - PCC LPI2C1 Register"]
    pub pcc_lpi2c1: crate::Reg<pcc_lpi2c1::PCC_LPI2C1_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0x1a8 - PCC LPUART0 Register"]
    pub pcc_lpuart0: crate::Reg<pcc_lpuart0::PCC_LPUART0_SPEC>,
    #[doc = "0x1ac - PCC LPUART1 Register"]
    pub pcc_lpuart1: crate::Reg<pcc_lpuart1::PCC_LPUART1_SPEC>,
    #[doc = "0x1b0 - PCC LPUART2 Register"]
    pub pcc_lpuart2: crate::Reg<pcc_lpuart2::PCC_LPUART2_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x1b8 - PCC FTM4 Register"]
    pub pcc_ftm4: crate::Reg<pcc_ftm4::PCC_FTM4_SPEC>,
    #[doc = "0x1bc - PCC FTM5 Register"]
    pub pcc_ftm5: crate::Reg<pcc_ftm5::PCC_FTM5_SPEC>,
    #[doc = "0x1c0 - PCC FTM6 Register"]
    pub pcc_ftm6: crate::Reg<pcc_ftm6::PCC_FTM6_SPEC>,
    #[doc = "0x1c4 - PCC FTM7 Register"]
    pub pcc_ftm7: crate::Reg<pcc_ftm7::PCC_FTM7_SPEC>,
    _reserved38: [u8; 0x04],
    #[doc = "0x1cc - PCC CMP0 Register"]
    pub pcc_cmp0: crate::Reg<pcc_cmp0::PCC_CMP0_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x1d8 - PCC QSPI Register"]
    pub pcc_qspi: crate::Reg<pcc_qspi::PCC_QSPI_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0x1e4 - PCC ENET Register"]
    pub pcc_enet: crate::Reg<pcc_enet::PCC_ENET_SPEC>,
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
#[doc = "PCC_FlexCAN1 register accessor: an alias for `Reg<PCC_FLEXCAN1_SPEC>`"]
pub type PCC_FLEXCAN1 = crate::Reg<pcc_flex_can1::PCC_FLEXCAN1_SPEC>;
#[doc = "PCC FlexCAN1 Register"]
pub mod pcc_flex_can1;
#[doc = "PCC_FTM3 register accessor: an alias for `Reg<PCC_FTM3_SPEC>`"]
pub type PCC_FTM3 = crate::Reg<pcc_ftm3::PCC_FTM3_SPEC>;
#[doc = "PCC FTM3 Register"]
pub mod pcc_ftm3;
#[doc = "PCC_ADC1 register accessor: an alias for `Reg<PCC_ADC1_SPEC>`"]
pub type PCC_ADC1 = crate::Reg<pcc_adc1::PCC_ADC1_SPEC>;
#[doc = "PCC ADC1 Register"]
pub mod pcc_adc1;
#[doc = "PCC_FlexCAN2 register accessor: an alias for `Reg<PCC_FLEXCAN2_SPEC>`"]
pub type PCC_FLEXCAN2 = crate::Reg<pcc_flex_can2::PCC_FLEXCAN2_SPEC>;
#[doc = "PCC FlexCAN2 Register"]
pub mod pcc_flex_can2;
#[doc = "PCC_LPSPI0 register accessor: an alias for `Reg<PCC_LPSPI0_SPEC>`"]
pub type PCC_LPSPI0 = crate::Reg<pcc_lpspi0::PCC_LPSPI0_SPEC>;
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC_LPSPI1 register accessor: an alias for `Reg<PCC_LPSPI1_SPEC>`"]
pub type PCC_LPSPI1 = crate::Reg<pcc_lpspi1::PCC_LPSPI1_SPEC>;
#[doc = "PCC LPSPI1 Register"]
pub mod pcc_lpspi1;
#[doc = "PCC_LPSPI2 register accessor: an alias for `Reg<PCC_LPSPI2_SPEC>`"]
pub type PCC_LPSPI2 = crate::Reg<pcc_lpspi2::PCC_LPSPI2_SPEC>;
#[doc = "PCC LPSPI2 Register"]
pub mod pcc_lpspi2;
#[doc = "PCC_PDB1 register accessor: an alias for `Reg<PCC_PDB1_SPEC>`"]
pub type PCC_PDB1 = crate::Reg<pcc_pdb1::PCC_PDB1_SPEC>;
#[doc = "PCC PDB1 Register"]
pub mod pcc_pdb1;
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
#[doc = "PCC_FTM2 register accessor: an alias for `Reg<PCC_FTM2_SPEC>`"]
pub type PCC_FTM2 = crate::Reg<pcc_ftm2::PCC_FTM2_SPEC>;
#[doc = "PCC FTM2 Register"]
pub mod pcc_ftm2;
#[doc = "PCC_ADC0 register accessor: an alias for `Reg<PCC_ADC0_SPEC>`"]
pub type PCC_ADC0 = crate::Reg<pcc_adc0::PCC_ADC0_SPEC>;
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC_RTC register accessor: an alias for `Reg<PCC_RTC_SPEC>`"]
pub type PCC_RTC = crate::Reg<pcc_rtc::PCC_RTC_SPEC>;
#[doc = "PCC RTC Register"]
pub mod pcc_rtc;
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
#[doc = "PCC_SAI0 register accessor: an alias for `Reg<PCC_SAI0_SPEC>`"]
pub type PCC_SAI0 = crate::Reg<pcc_sai0::PCC_SAI0_SPEC>;
#[doc = "PCC SAI0 Register"]
pub mod pcc_sai0;
#[doc = "PCC_SAI1 register accessor: an alias for `Reg<PCC_SAI1_SPEC>`"]
pub type PCC_SAI1 = crate::Reg<pcc_sai1::PCC_SAI1_SPEC>;
#[doc = "PCC SAI1 Register"]
pub mod pcc_sai1;
#[doc = "PCC_FlexIO register accessor: an alias for `Reg<PCC_FLEXIO_SPEC>`"]
pub type PCC_FLEXIO = crate::Reg<pcc_flex_io::PCC_FLEXIO_SPEC>;
#[doc = "PCC FlexIO Register"]
pub mod pcc_flex_io;
#[doc = "PCC_EWM register accessor: an alias for `Reg<PCC_EWM_SPEC>`"]
pub type PCC_EWM = crate::Reg<pcc_ewm::PCC_EWM_SPEC>;
#[doc = "PCC EWM Register"]
pub mod pcc_ewm;
#[doc = "PCC_LPI2C0 register accessor: an alias for `Reg<PCC_LPI2C0_SPEC>`"]
pub type PCC_LPI2C0 = crate::Reg<pcc_lpi2c0::PCC_LPI2C0_SPEC>;
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC_LPI2C1 register accessor: an alias for `Reg<PCC_LPI2C1_SPEC>`"]
pub type PCC_LPI2C1 = crate::Reg<pcc_lpi2c1::PCC_LPI2C1_SPEC>;
#[doc = "PCC LPI2C1 Register"]
pub mod pcc_lpi2c1;
#[doc = "PCC_LPUART0 register accessor: an alias for `Reg<PCC_LPUART0_SPEC>`"]
pub type PCC_LPUART0 = crate::Reg<pcc_lpuart0::PCC_LPUART0_SPEC>;
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC_LPUART1 register accessor: an alias for `Reg<PCC_LPUART1_SPEC>`"]
pub type PCC_LPUART1 = crate::Reg<pcc_lpuart1::PCC_LPUART1_SPEC>;
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC_LPUART2 register accessor: an alias for `Reg<PCC_LPUART2_SPEC>`"]
pub type PCC_LPUART2 = crate::Reg<pcc_lpuart2::PCC_LPUART2_SPEC>;
#[doc = "PCC LPUART2 Register"]
pub mod pcc_lpuart2;
#[doc = "PCC_FTM4 register accessor: an alias for `Reg<PCC_FTM4_SPEC>`"]
pub type PCC_FTM4 = crate::Reg<pcc_ftm4::PCC_FTM4_SPEC>;
#[doc = "PCC FTM4 Register"]
pub mod pcc_ftm4;
#[doc = "PCC_FTM5 register accessor: an alias for `Reg<PCC_FTM5_SPEC>`"]
pub type PCC_FTM5 = crate::Reg<pcc_ftm5::PCC_FTM5_SPEC>;
#[doc = "PCC FTM5 Register"]
pub mod pcc_ftm5;
#[doc = "PCC_FTM6 register accessor: an alias for `Reg<PCC_FTM6_SPEC>`"]
pub type PCC_FTM6 = crate::Reg<pcc_ftm6::PCC_FTM6_SPEC>;
#[doc = "PCC FTM6 Register"]
pub mod pcc_ftm6;
#[doc = "PCC_FTM7 register accessor: an alias for `Reg<PCC_FTM7_SPEC>`"]
pub type PCC_FTM7 = crate::Reg<pcc_ftm7::PCC_FTM7_SPEC>;
#[doc = "PCC FTM7 Register"]
pub mod pcc_ftm7;
#[doc = "PCC_CMP0 register accessor: an alias for `Reg<PCC_CMP0_SPEC>`"]
pub type PCC_CMP0 = crate::Reg<pcc_cmp0::PCC_CMP0_SPEC>;
#[doc = "PCC CMP0 Register"]
pub mod pcc_cmp0;
#[doc = "PCC_QSPI register accessor: an alias for `Reg<PCC_QSPI_SPEC>`"]
pub type PCC_QSPI = crate::Reg<pcc_qspi::PCC_QSPI_SPEC>;
#[doc = "PCC QSPI Register"]
pub mod pcc_qspi;
#[doc = "PCC_ENET register accessor: an alias for `Reg<PCC_ENET_SPEC>`"]
pub type PCC_ENET = crate::Reg<pcc_enet::PCC_ENET_SPEC>;
#[doc = "PCC ENET Register"]
pub mod pcc_enet;
