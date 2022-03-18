#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Configuration register"]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x04 - Control 1 register"]
    pub ctrl1: crate::Reg<ctrl1::CTRL1_SPEC>,
    #[doc = "0x08 - Free Running Timer"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Rx Mailboxes Global Mask register"]
    pub rxmgmask: crate::Reg<rxmgmask::RXMGMASK_SPEC>,
    #[doc = "0x14 - Rx 14 Mask register"]
    pub rx14mask: crate::Reg<rx14mask::RX14MASK_SPEC>,
    #[doc = "0x18 - Rx 15 Mask register"]
    pub rx15mask: crate::Reg<rx15mask::RX15MASK_SPEC>,
    #[doc = "0x1c - Error Counter"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x20 - Error and Status 1 register"]
    pub esr1: crate::Reg<esr1::ESR1_SPEC>,
    #[doc = "0x24 - Interrupt Masks 2 register"]
    pub imask2: crate::Reg<imask2::IMASK2_SPEC>,
    #[doc = "0x28 - Interrupt Masks 1 register"]
    pub imask1: crate::Reg<imask1::IMASK1_SPEC>,
    #[doc = "0x2c - Interrupt Flags 2 register"]
    pub iflag2: crate::Reg<iflag2::IFLAG2_SPEC>,
    #[doc = "0x30 - Interrupt Flags 1 register"]
    pub iflag1: crate::Reg<iflag1::IFLAG1_SPEC>,
    #[doc = "0x34 - Control 2 register"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x38 - Error and Status 2 register"]
    pub esr2: crate::Reg<esr2::ESR2_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x44 - CRC register"]
    pub crcr: crate::Reg<crcr::CRCR_SPEC>,
    #[doc = "0x48 - Rx FIFO Global Mask register"]
    pub rxfgmask: crate::Reg<rxfgmask::RXFGMASK_SPEC>,
    #[doc = "0x4c - Rx FIFO Information register"]
    pub rxfir: crate::Reg<rxfir::RXFIR_SPEC>,
    #[doc = "0x50 - CAN Bit Timing register"]
    pub cbt: crate::Reg<cbt::CBT_SPEC>,
    _reserved18: [u8; 0x2c],
    #[doc = "0x80..0x480 - Embedded RAM"]
    pub ramn: [crate::Reg<ramn::RAMN_SPEC>; 256],
    _reserved19: [u8; 0x0400],
    #[doc = "0x880..0x980 - Rx Individual Mask registers"]
    pub rximr: [crate::Reg<rximr::RXIMR_SPEC>; 64],
    _reserved20: [u8; 0x0180],
    #[doc = "0xb00 - Pretended Networking Control 1 register"]
    pub ctrl1_pn: crate::Reg<ctrl1_pn::CTRL1_PN_SPEC>,
    #[doc = "0xb04 - Pretended Networking Control 2 register"]
    pub ctrl2_pn: crate::Reg<ctrl2_pn::CTRL2_PN_SPEC>,
    #[doc = "0xb08 - Pretended Networking Wake Up Match register"]
    pub wu_mtc: crate::Reg<wu_mtc::WU_MTC_SPEC>,
    #[doc = "0xb0c - Pretended Networking ID Filter 1 register"]
    pub flt_id1: crate::Reg<flt_id1::FLT_ID1_SPEC>,
    #[doc = "0xb10 - Pretended Networking DLC Filter register"]
    pub flt_dlc: crate::Reg<flt_dlc::FLT_DLC_SPEC>,
    #[doc = "0xb14 - Pretended Networking Payload Low Filter 1 register"]
    pub pl1_lo: crate::Reg<pl1_lo::PL1_LO_SPEC>,
    #[doc = "0xb18 - Pretended Networking Payload High Filter 1 register"]
    pub pl1_hi: crate::Reg<pl1_hi::PL1_HI_SPEC>,
    #[doc = "0xb1c - Pretended Networking ID Filter 2 Register / ID Mask register"]
    pub flt_id2_idmask: crate::Reg<flt_id2_idmask::FLT_ID2_IDMASK_SPEC>,
    #[doc = "0xb20 - Pretended Networking Payload Low Filter 2 Register / Payload Low Mask register"]
    pub pl2_plmask_lo: crate::Reg<pl2_plmask_lo::PL2_PLMASK_LO_SPEC>,
    #[doc = "0xb24 - Pretended Networking Payload High Filter 2 low order bits / Payload High Mask register"]
    pub pl2_plmask_hi: crate::Reg<pl2_plmask_hi::PL2_PLMASK_HI_SPEC>,
    _reserved30: [u8; 0x18],
    #[doc = "0xb40..0xb80 - no description available"]
    pub wmb: [WMB; 4],
    _reserved31: [u8; 0x80],
    #[doc = "0xc00 - CAN FD Control register"]
    pub fdctrl: crate::Reg<fdctrl::FDCTRL_SPEC>,
    #[doc = "0xc04 - CAN FD Bit Timing register"]
    pub fdcbt: crate::Reg<fdcbt::FDCBT_SPEC>,
    #[doc = "0xc08 - CAN FD CRC register"]
    pub fdcrc: crate::Reg<fdcrc::FDCRC_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct WMB {
    #[doc = "0x00 - Wake Up Message Buffer register for C/S"]
    pub wmbn_cs: crate::Reg<self::wmb::wmbn_cs::WMBN_CS_SPEC>,
    #[doc = "0x04 - Wake Up Message Buffer Register for ID"]
    pub wmbn_id: crate::Reg<self::wmb::wmbn_id::WMBN_ID_SPEC>,
    #[doc = "0x08 - Wake Up Message Buffer Register for Data 0-3"]
    pub wmbn_d03: crate::Reg<self::wmb::wmbn_d03::WMBN_D03_SPEC>,
    #[doc = "0x0c - Wake Up Message Buffer Register Data 4-7"]
    pub wmbn_d47: crate::Reg<self::wmb::wmbn_d47::WMBN_D47_SPEC>,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod wmb;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Module Configuration register"]
pub mod mcr;
#[doc = "CTRL1 register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control 1 register"]
pub mod ctrl1;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "Free Running Timer"]
pub mod timer;
#[doc = "RXMGMASK register accessor: an alias for `Reg<RXMGMASK_SPEC>`"]
pub type RXMGMASK = crate::Reg<rxmgmask::RXMGMASK_SPEC>;
#[doc = "Rx Mailboxes Global Mask register"]
pub mod rxmgmask;
#[doc = "RX14MASK register accessor: an alias for `Reg<RX14MASK_SPEC>`"]
pub type RX14MASK = crate::Reg<rx14mask::RX14MASK_SPEC>;
#[doc = "Rx 14 Mask register"]
pub mod rx14mask;
#[doc = "RX15MASK register accessor: an alias for `Reg<RX15MASK_SPEC>`"]
pub type RX15MASK = crate::Reg<rx15mask::RX15MASK_SPEC>;
#[doc = "Rx 15 Mask register"]
pub mod rx15mask;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter"]
pub mod ecr;
#[doc = "ESR1 register accessor: an alias for `Reg<ESR1_SPEC>`"]
pub type ESR1 = crate::Reg<esr1::ESR1_SPEC>;
#[doc = "Error and Status 1 register"]
pub mod esr1;
#[doc = "IMASK2 register accessor: an alias for `Reg<IMASK2_SPEC>`"]
pub type IMASK2 = crate::Reg<imask2::IMASK2_SPEC>;
#[doc = "Interrupt Masks 2 register"]
pub mod imask2;
#[doc = "IMASK1 register accessor: an alias for `Reg<IMASK1_SPEC>`"]
pub type IMASK1 = crate::Reg<imask1::IMASK1_SPEC>;
#[doc = "Interrupt Masks 1 register"]
pub mod imask1;
#[doc = "IFLAG2 register accessor: an alias for `Reg<IFLAG2_SPEC>`"]
pub type IFLAG2 = crate::Reg<iflag2::IFLAG2_SPEC>;
#[doc = "Interrupt Flags 2 register"]
pub mod iflag2;
#[doc = "IFLAG1 register accessor: an alias for `Reg<IFLAG1_SPEC>`"]
pub type IFLAG1 = crate::Reg<iflag1::IFLAG1_SPEC>;
#[doc = "Interrupt Flags 1 register"]
pub mod iflag1;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ESR2 register accessor: an alias for `Reg<ESR2_SPEC>`"]
pub type ESR2 = crate::Reg<esr2::ESR2_SPEC>;
#[doc = "Error and Status 2 register"]
pub mod esr2;
#[doc = "CRCR register accessor: an alias for `Reg<CRCR_SPEC>`"]
pub type CRCR = crate::Reg<crcr::CRCR_SPEC>;
#[doc = "CRC register"]
pub mod crcr;
#[doc = "RXFGMASK register accessor: an alias for `Reg<RXFGMASK_SPEC>`"]
pub type RXFGMASK = crate::Reg<rxfgmask::RXFGMASK_SPEC>;
#[doc = "Rx FIFO Global Mask register"]
pub mod rxfgmask;
#[doc = "RXFIR register accessor: an alias for `Reg<RXFIR_SPEC>`"]
pub type RXFIR = crate::Reg<rxfir::RXFIR_SPEC>;
#[doc = "Rx FIFO Information register"]
pub mod rxfir;
#[doc = "CBT register accessor: an alias for `Reg<CBT_SPEC>`"]
pub type CBT = crate::Reg<cbt::CBT_SPEC>;
#[doc = "CAN Bit Timing register"]
pub mod cbt;
#[doc = "RAMn register accessor: an alias for `Reg<RAMN_SPEC>`"]
pub type RAMN = crate::Reg<ramn::RAMN_SPEC>;
#[doc = "Embedded RAM"]
pub mod ramn;
#[doc = "RXIMR register accessor: an alias for `Reg<RXIMR_SPEC>`"]
pub type RXIMR = crate::Reg<rximr::RXIMR_SPEC>;
#[doc = "Rx Individual Mask registers"]
pub mod rximr;
#[doc = "CTRL1_PN register accessor: an alias for `Reg<CTRL1_PN_SPEC>`"]
pub type CTRL1_PN = crate::Reg<ctrl1_pn::CTRL1_PN_SPEC>;
#[doc = "Pretended Networking Control 1 register"]
pub mod ctrl1_pn;
#[doc = "CTRL2_PN register accessor: an alias for `Reg<CTRL2_PN_SPEC>`"]
pub type CTRL2_PN = crate::Reg<ctrl2_pn::CTRL2_PN_SPEC>;
#[doc = "Pretended Networking Control 2 register"]
pub mod ctrl2_pn;
#[doc = "WU_MTC register accessor: an alias for `Reg<WU_MTC_SPEC>`"]
pub type WU_MTC = crate::Reg<wu_mtc::WU_MTC_SPEC>;
#[doc = "Pretended Networking Wake Up Match register"]
pub mod wu_mtc;
#[doc = "FLT_ID1 register accessor: an alias for `Reg<FLT_ID1_SPEC>`"]
pub type FLT_ID1 = crate::Reg<flt_id1::FLT_ID1_SPEC>;
#[doc = "Pretended Networking ID Filter 1 register"]
pub mod flt_id1;
#[doc = "FLT_DLC register accessor: an alias for `Reg<FLT_DLC_SPEC>`"]
pub type FLT_DLC = crate::Reg<flt_dlc::FLT_DLC_SPEC>;
#[doc = "Pretended Networking DLC Filter register"]
pub mod flt_dlc;
#[doc = "PL1_LO register accessor: an alias for `Reg<PL1_LO_SPEC>`"]
pub type PL1_LO = crate::Reg<pl1_lo::PL1_LO_SPEC>;
#[doc = "Pretended Networking Payload Low Filter 1 register"]
pub mod pl1_lo;
#[doc = "PL1_HI register accessor: an alias for `Reg<PL1_HI_SPEC>`"]
pub type PL1_HI = crate::Reg<pl1_hi::PL1_HI_SPEC>;
#[doc = "Pretended Networking Payload High Filter 1 register"]
pub mod pl1_hi;
#[doc = "FLT_ID2_IDMASK register accessor: an alias for `Reg<FLT_ID2_IDMASK_SPEC>`"]
pub type FLT_ID2_IDMASK = crate::Reg<flt_id2_idmask::FLT_ID2_IDMASK_SPEC>;
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask register"]
pub mod flt_id2_idmask;
#[doc = "PL2_PLMASK_LO register accessor: an alias for `Reg<PL2_PLMASK_LO_SPEC>`"]
pub type PL2_PLMASK_LO = crate::Reg<pl2_plmask_lo::PL2_PLMASK_LO_SPEC>;
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask register"]
pub mod pl2_plmask_lo;
#[doc = "PL2_PLMASK_HI register accessor: an alias for `Reg<PL2_PLMASK_HI_SPEC>`"]
pub type PL2_PLMASK_HI = crate::Reg<pl2_plmask_hi::PL2_PLMASK_HI_SPEC>;
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask register"]
pub mod pl2_plmask_hi;
#[doc = "FDCTRL register accessor: an alias for `Reg<FDCTRL_SPEC>`"]
pub type FDCTRL = crate::Reg<fdctrl::FDCTRL_SPEC>;
#[doc = "CAN FD Control register"]
pub mod fdctrl;
#[doc = "FDCBT register accessor: an alias for `Reg<FDCBT_SPEC>`"]
pub type FDCBT = crate::Reg<fdcbt::FDCBT_SPEC>;
#[doc = "CAN FD Bit Timing register"]
pub mod fdcbt;
#[doc = "FDCRC register accessor: an alias for `Reg<FDCRC_SPEC>`"]
pub type FDCRC = crate::Reg<fdcrc::FDCRC_SPEC>;
#[doc = "CAN FD CRC register"]
pub mod fdcrc;
