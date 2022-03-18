#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub sai_verid: crate::Reg<sai_verid::SAI_VERID_SPEC>,
    #[doc = "0x04 - Parameter Register"]
    pub sai_param: crate::Reg<sai_param::SAI_PARAM_SPEC>,
    #[doc = "0x08 - SAI Transmit Control Register"]
    pub sai_tcsr: crate::Reg<sai_tcsr::SAI_TCSR_SPEC>,
    #[doc = "0x0c - SAI Transmit Configuration 1 Register"]
    pub sai_tcr1: crate::Reg<sai_tcr1::SAI_TCR1_SPEC>,
    #[doc = "0x10 - SAI Transmit Configuration 2 Register"]
    pub sai_tcr2: crate::Reg<sai_tcr2::SAI_TCR2_SPEC>,
    #[doc = "0x14 - SAI Transmit Configuration 3 Register"]
    pub sai_tcr3: crate::Reg<sai_tcr3::SAI_TCR3_SPEC>,
    #[doc = "0x18 - SAI Transmit Configuration 4 Register"]
    pub sai_tcr4: crate::Reg<sai_tcr4::SAI_TCR4_SPEC>,
    #[doc = "0x1c - SAI Transmit Configuration 5 Register"]
    pub sai_tcr5: crate::Reg<sai_tcr5::SAI_TCR5_SPEC>,
    #[doc = "0x20 - SAI Transmit Data Register"]
    pub sai_tdr0: crate::Reg<sai_tdr0::SAI_TDR0_SPEC>,
    #[doc = "0x24 - SAI Transmit Data Register"]
    pub sai_tdr1: crate::Reg<sai_tdr1::SAI_TDR1_SPEC>,
    #[doc = "0x28 - SAI Transmit Data Register"]
    pub sai_tdr2: crate::Reg<sai_tdr2::SAI_TDR2_SPEC>,
    #[doc = "0x2c - SAI Transmit Data Register"]
    pub sai_tdr3: crate::Reg<sai_tdr3::SAI_TDR3_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - SAI Transmit FIFO Register"]
    pub sai_tfr0: crate::Reg<sai_tfr0::SAI_TFR0_SPEC>,
    #[doc = "0x44 - SAI Transmit FIFO Register"]
    pub sai_tfr1: crate::Reg<sai_tfr1::SAI_TFR1_SPEC>,
    #[doc = "0x48 - SAI Transmit FIFO Register"]
    pub sai_tfr2: crate::Reg<sai_tfr2::SAI_TFR2_SPEC>,
    #[doc = "0x4c - SAI Transmit FIFO Register"]
    pub sai_tfr3: crate::Reg<sai_tfr3::SAI_TFR3_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - SAI Transmit Mask Register"]
    pub sai_tmr: crate::Reg<sai_tmr::SAI_TMR_SPEC>,
    _reserved17: [u8; 0x24],
    #[doc = "0x88 - SAI Receive Control Register"]
    pub sai_rcsr: crate::Reg<sai_rcsr::SAI_RCSR_SPEC>,
    #[doc = "0x8c - SAI Receive Configuration 1 Register"]
    pub sai_rcr1: crate::Reg<sai_rcr1::SAI_RCR1_SPEC>,
    #[doc = "0x90 - SAI Receive Configuration 2 Register"]
    pub sai_rcr2: crate::Reg<sai_rcr2::SAI_RCR2_SPEC>,
    #[doc = "0x94 - SAI Receive Configuration 3 Register"]
    pub sai_rcr3: crate::Reg<sai_rcr3::SAI_RCR3_SPEC>,
    #[doc = "0x98 - SAI Receive Configuration 4 Register"]
    pub sai_rcr4: crate::Reg<sai_rcr4::SAI_RCR4_SPEC>,
    #[doc = "0x9c - SAI Receive Configuration 5 Register"]
    pub sai_rcr5: crate::Reg<sai_rcr5::SAI_RCR5_SPEC>,
    #[doc = "0xa0 - SAI Receive Data Register"]
    pub sai_rdr0: crate::Reg<sai_rdr0::SAI_RDR0_SPEC>,
    #[doc = "0xa4 - SAI Receive Data Register"]
    pub sai_rdr1: crate::Reg<sai_rdr1::SAI_RDR1_SPEC>,
    #[doc = "0xa8 - SAI Receive Data Register"]
    pub sai_rdr2: crate::Reg<sai_rdr2::SAI_RDR2_SPEC>,
    #[doc = "0xac - SAI Receive Data Register"]
    pub sai_rdr3: crate::Reg<sai_rdr3::SAI_RDR3_SPEC>,
    _reserved27: [u8; 0x10],
    #[doc = "0xc0 - SAI Receive FIFO Register"]
    pub sai_rfr0: crate::Reg<sai_rfr0::SAI_RFR0_SPEC>,
    #[doc = "0xc4 - SAI Receive FIFO Register"]
    pub sai_rfr1: crate::Reg<sai_rfr1::SAI_RFR1_SPEC>,
    #[doc = "0xc8 - SAI Receive FIFO Register"]
    pub sai_rfr2: crate::Reg<sai_rfr2::SAI_RFR2_SPEC>,
    #[doc = "0xcc - SAI Receive FIFO Register"]
    pub sai_rfr3: crate::Reg<sai_rfr3::SAI_RFR3_SPEC>,
    _reserved31: [u8; 0x10],
    #[doc = "0xe0 - SAI Receive Mask Register"]
    pub sai_rmr: crate::Reg<sai_rmr::SAI_RMR_SPEC>,
}
#[doc = "SAI_VERID register accessor: an alias for `Reg<SAI_VERID_SPEC>`"]
pub type SAI_VERID = crate::Reg<sai_verid::SAI_VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod sai_verid;
#[doc = "SAI_PARAM register accessor: an alias for `Reg<SAI_PARAM_SPEC>`"]
pub type SAI_PARAM = crate::Reg<sai_param::SAI_PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod sai_param;
#[doc = "SAI_TCSR register accessor: an alias for `Reg<SAI_TCSR_SPEC>`"]
pub type SAI_TCSR = crate::Reg<sai_tcsr::SAI_TCSR_SPEC>;
#[doc = "SAI Transmit Control Register"]
pub mod sai_tcsr;
#[doc = "SAI_TCR1 register accessor: an alias for `Reg<SAI_TCR1_SPEC>`"]
pub type SAI_TCR1 = crate::Reg<sai_tcr1::SAI_TCR1_SPEC>;
#[doc = "SAI Transmit Configuration 1 Register"]
pub mod sai_tcr1;
#[doc = "SAI_TCR2 register accessor: an alias for `Reg<SAI_TCR2_SPEC>`"]
pub type SAI_TCR2 = crate::Reg<sai_tcr2::SAI_TCR2_SPEC>;
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod sai_tcr2;
#[doc = "SAI_TCR3 register accessor: an alias for `Reg<SAI_TCR3_SPEC>`"]
pub type SAI_TCR3 = crate::Reg<sai_tcr3::SAI_TCR3_SPEC>;
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod sai_tcr3;
#[doc = "SAI_TCR4 register accessor: an alias for `Reg<SAI_TCR4_SPEC>`"]
pub type SAI_TCR4 = crate::Reg<sai_tcr4::SAI_TCR4_SPEC>;
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod sai_tcr4;
#[doc = "SAI_TCR5 register accessor: an alias for `Reg<SAI_TCR5_SPEC>`"]
pub type SAI_TCR5 = crate::Reg<sai_tcr5::SAI_TCR5_SPEC>;
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod sai_tcr5;
#[doc = "SAI_TDR0 register accessor: an alias for `Reg<SAI_TDR0_SPEC>`"]
pub type SAI_TDR0 = crate::Reg<sai_tdr0::SAI_TDR0_SPEC>;
#[doc = "SAI Transmit Data Register"]
pub mod sai_tdr0;
#[doc = "SAI_TDR1 register accessor: an alias for `Reg<SAI_TDR1_SPEC>`"]
pub type SAI_TDR1 = crate::Reg<sai_tdr1::SAI_TDR1_SPEC>;
#[doc = "SAI Transmit Data Register"]
pub mod sai_tdr1;
#[doc = "SAI_TDR2 register accessor: an alias for `Reg<SAI_TDR2_SPEC>`"]
pub type SAI_TDR2 = crate::Reg<sai_tdr2::SAI_TDR2_SPEC>;
#[doc = "SAI Transmit Data Register"]
pub mod sai_tdr2;
#[doc = "SAI_TDR3 register accessor: an alias for `Reg<SAI_TDR3_SPEC>`"]
pub type SAI_TDR3 = crate::Reg<sai_tdr3::SAI_TDR3_SPEC>;
#[doc = "SAI Transmit Data Register"]
pub mod sai_tdr3;
#[doc = "SAI_TFR0 register accessor: an alias for `Reg<SAI_TFR0_SPEC>`"]
pub type SAI_TFR0 = crate::Reg<sai_tfr0::SAI_TFR0_SPEC>;
#[doc = "SAI Transmit FIFO Register"]
pub mod sai_tfr0;
#[doc = "SAI_TFR1 register accessor: an alias for `Reg<SAI_TFR1_SPEC>`"]
pub type SAI_TFR1 = crate::Reg<sai_tfr1::SAI_TFR1_SPEC>;
#[doc = "SAI Transmit FIFO Register"]
pub mod sai_tfr1;
#[doc = "SAI_TFR2 register accessor: an alias for `Reg<SAI_TFR2_SPEC>`"]
pub type SAI_TFR2 = crate::Reg<sai_tfr2::SAI_TFR2_SPEC>;
#[doc = "SAI Transmit FIFO Register"]
pub mod sai_tfr2;
#[doc = "SAI_TFR3 register accessor: an alias for `Reg<SAI_TFR3_SPEC>`"]
pub type SAI_TFR3 = crate::Reg<sai_tfr3::SAI_TFR3_SPEC>;
#[doc = "SAI Transmit FIFO Register"]
pub mod sai_tfr3;
#[doc = "SAI_TMR register accessor: an alias for `Reg<SAI_TMR_SPEC>`"]
pub type SAI_TMR = crate::Reg<sai_tmr::SAI_TMR_SPEC>;
#[doc = "SAI Transmit Mask Register"]
pub mod sai_tmr;
#[doc = "SAI_RCSR register accessor: an alias for `Reg<SAI_RCSR_SPEC>`"]
pub type SAI_RCSR = crate::Reg<sai_rcsr::SAI_RCSR_SPEC>;
#[doc = "SAI Receive Control Register"]
pub mod sai_rcsr;
#[doc = "SAI_RCR1 register accessor: an alias for `Reg<SAI_RCR1_SPEC>`"]
pub type SAI_RCR1 = crate::Reg<sai_rcr1::SAI_RCR1_SPEC>;
#[doc = "SAI Receive Configuration 1 Register"]
pub mod sai_rcr1;
#[doc = "SAI_RCR2 register accessor: an alias for `Reg<SAI_RCR2_SPEC>`"]
pub type SAI_RCR2 = crate::Reg<sai_rcr2::SAI_RCR2_SPEC>;
#[doc = "SAI Receive Configuration 2 Register"]
pub mod sai_rcr2;
#[doc = "SAI_RCR3 register accessor: an alias for `Reg<SAI_RCR3_SPEC>`"]
pub type SAI_RCR3 = crate::Reg<sai_rcr3::SAI_RCR3_SPEC>;
#[doc = "SAI Receive Configuration 3 Register"]
pub mod sai_rcr3;
#[doc = "SAI_RCR4 register accessor: an alias for `Reg<SAI_RCR4_SPEC>`"]
pub type SAI_RCR4 = crate::Reg<sai_rcr4::SAI_RCR4_SPEC>;
#[doc = "SAI Receive Configuration 4 Register"]
pub mod sai_rcr4;
#[doc = "SAI_RCR5 register accessor: an alias for `Reg<SAI_RCR5_SPEC>`"]
pub type SAI_RCR5 = crate::Reg<sai_rcr5::SAI_RCR5_SPEC>;
#[doc = "SAI Receive Configuration 5 Register"]
pub mod sai_rcr5;
#[doc = "SAI_RDR0 register accessor: an alias for `Reg<SAI_RDR0_SPEC>`"]
pub type SAI_RDR0 = crate::Reg<sai_rdr0::SAI_RDR0_SPEC>;
#[doc = "SAI Receive Data Register"]
pub mod sai_rdr0;
#[doc = "SAI_RDR1 register accessor: an alias for `Reg<SAI_RDR1_SPEC>`"]
pub type SAI_RDR1 = crate::Reg<sai_rdr1::SAI_RDR1_SPEC>;
#[doc = "SAI Receive Data Register"]
pub mod sai_rdr1;
#[doc = "SAI_RDR2 register accessor: an alias for `Reg<SAI_RDR2_SPEC>`"]
pub type SAI_RDR2 = crate::Reg<sai_rdr2::SAI_RDR2_SPEC>;
#[doc = "SAI Receive Data Register"]
pub mod sai_rdr2;
#[doc = "SAI_RDR3 register accessor: an alias for `Reg<SAI_RDR3_SPEC>`"]
pub type SAI_RDR3 = crate::Reg<sai_rdr3::SAI_RDR3_SPEC>;
#[doc = "SAI Receive Data Register"]
pub mod sai_rdr3;
#[doc = "SAI_RFR0 register accessor: an alias for `Reg<SAI_RFR0_SPEC>`"]
pub type SAI_RFR0 = crate::Reg<sai_rfr0::SAI_RFR0_SPEC>;
#[doc = "SAI Receive FIFO Register"]
pub mod sai_rfr0;
#[doc = "SAI_RFR1 register accessor: an alias for `Reg<SAI_RFR1_SPEC>`"]
pub type SAI_RFR1 = crate::Reg<sai_rfr1::SAI_RFR1_SPEC>;
#[doc = "SAI Receive FIFO Register"]
pub mod sai_rfr1;
#[doc = "SAI_RFR2 register accessor: an alias for `Reg<SAI_RFR2_SPEC>`"]
pub type SAI_RFR2 = crate::Reg<sai_rfr2::SAI_RFR2_SPEC>;
#[doc = "SAI Receive FIFO Register"]
pub mod sai_rfr2;
#[doc = "SAI_RFR3 register accessor: an alias for `Reg<SAI_RFR3_SPEC>`"]
pub type SAI_RFR3 = crate::Reg<sai_rfr3::SAI_RFR3_SPEC>;
#[doc = "SAI Receive FIFO Register"]
pub mod sai_rfr3;
#[doc = "SAI_RMR register accessor: an alias for `Reg<SAI_RMR_SPEC>`"]
pub type SAI_RMR = crate::Reg<sai_rmr::SAI_RMR_SPEC>;
#[doc = "SAI Receive Mask Register"]
pub mod sai_rmr;
