#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Error Status Register"]
    pub es: crate::Reg<es::ES_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: crate::Reg<erq::ERQ_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: crate::Reg<eei::EEI_SPEC>,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: crate::Reg<ceei::CEEI_SPEC>,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: crate::Reg<seei::SEEI_SPEC>,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: crate::Reg<cerq::CERQ_SPEC>,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: crate::Reg<serq::SERQ_SPEC>,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: crate::Reg<cdne::CDNE_SPEC>,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: crate::Reg<ssrt::SSRT_SPEC>,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: crate::Reg<cerr::CERR_SPEC>,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: crate::Reg<cint::CINT_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: crate::Reg<int::INT_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x2c - Error Register"]
    pub err: crate::Reg<err::ERR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: crate::Reg<hrs::HRS_SPEC>,
    _reserved15: [u8; 0x0c],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: crate::Reg<ears::EARS_SPEC>,
    _reserved16: [u8; 0xb8],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: crate::Reg<dchpri3::DCHPRI3_SPEC>,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: crate::Reg<dchpri2::DCHPRI2_SPEC>,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: crate::Reg<dchpri1::DCHPRI1_SPEC>,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: crate::Reg<dchpri0::DCHPRI0_SPEC>,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: crate::Reg<dchpri7::DCHPRI7_SPEC>,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: crate::Reg<dchpri6::DCHPRI6_SPEC>,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: crate::Reg<dchpri5::DCHPRI5_SPEC>,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: crate::Reg<dchpri4::DCHPRI4_SPEC>,
    #[doc = "0x108 - Channel n Priority Register"]
    pub dchpri11: crate::Reg<dchpri11::DCHPRI11_SPEC>,
    #[doc = "0x109 - Channel n Priority Register"]
    pub dchpri10: crate::Reg<dchpri10::DCHPRI10_SPEC>,
    #[doc = "0x10a - Channel n Priority Register"]
    pub dchpri9: crate::Reg<dchpri9::DCHPRI9_SPEC>,
    #[doc = "0x10b - Channel n Priority Register"]
    pub dchpri8: crate::Reg<dchpri8::DCHPRI8_SPEC>,
    #[doc = "0x10c - Channel n Priority Register"]
    pub dchpri15: crate::Reg<dchpri15::DCHPRI15_SPEC>,
    #[doc = "0x10d - Channel n Priority Register"]
    pub dchpri14: crate::Reg<dchpri14::DCHPRI14_SPEC>,
    #[doc = "0x10e - Channel n Priority Register"]
    pub dchpri13: crate::Reg<dchpri13::DCHPRI13_SPEC>,
    #[doc = "0x10f - Channel n Priority Register"]
    pub dchpri12: crate::Reg<dchpri12::DCHPRI12_SPEC>,
    _reserved32: [u8; 0x0ef0],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: crate::Reg<tcd0_saddr::TCD0_SADDR_SPEC>,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: crate::Reg<tcd0_soff::TCD0_SOFF_SPEC>,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: crate::Reg<tcd0_attr::TCD0_ATTR_SPEC>,
    _reserved_35_dma_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: crate::Reg<tcd0_slast::TCD0_SLAST_SPEC>,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: crate::Reg<tcd0_daddr::TCD0_DADDR_SPEC>,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: crate::Reg<tcd0_doff::TCD0_DOFF_SPEC>,
    _reserved_39_dma_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: crate::Reg<tcd0_dlastsga::TCD0_DLASTSGA_SPEC>,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: crate::Reg<tcd0_csr::TCD0_CSR_SPEC>,
    _reserved_42_dma_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: crate::Reg<tcd1_saddr::TCD1_SADDR_SPEC>,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: crate::Reg<tcd1_soff::TCD1_SOFF_SPEC>,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: crate::Reg<tcd1_attr::TCD1_ATTR_SPEC>,
    _reserved_46_dma_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: crate::Reg<tcd1_slast::TCD1_SLAST_SPEC>,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: crate::Reg<tcd1_daddr::TCD1_DADDR_SPEC>,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: crate::Reg<tcd1_doff::TCD1_DOFF_SPEC>,
    _reserved_50_dma_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: crate::Reg<tcd1_dlastsga::TCD1_DLASTSGA_SPEC>,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: crate::Reg<tcd1_csr::TCD1_CSR_SPEC>,
    _reserved_53_dma_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: crate::Reg<tcd2_saddr::TCD2_SADDR_SPEC>,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: crate::Reg<tcd2_soff::TCD2_SOFF_SPEC>,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: crate::Reg<tcd2_attr::TCD2_ATTR_SPEC>,
    _reserved_57_dma_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: crate::Reg<tcd2_slast::TCD2_SLAST_SPEC>,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: crate::Reg<tcd2_daddr::TCD2_DADDR_SPEC>,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: crate::Reg<tcd2_doff::TCD2_DOFF_SPEC>,
    _reserved_61_dma_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: crate::Reg<tcd2_dlastsga::TCD2_DLASTSGA_SPEC>,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: crate::Reg<tcd2_csr::TCD2_CSR_SPEC>,
    _reserved_64_dma_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: crate::Reg<tcd3_saddr::TCD3_SADDR_SPEC>,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: crate::Reg<tcd3_soff::TCD3_SOFF_SPEC>,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: crate::Reg<tcd3_attr::TCD3_ATTR_SPEC>,
    _reserved_68_dma_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: crate::Reg<tcd3_slast::TCD3_SLAST_SPEC>,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: crate::Reg<tcd3_daddr::TCD3_DADDR_SPEC>,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: crate::Reg<tcd3_doff::TCD3_DOFF_SPEC>,
    _reserved_72_dma_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: crate::Reg<tcd3_dlastsga::TCD3_DLASTSGA_SPEC>,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: crate::Reg<tcd3_csr::TCD3_CSR_SPEC>,
    _reserved_75_dma_tcd3_biter: [u8; 0x02],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: crate::Reg<tcd4_saddr::TCD4_SADDR_SPEC>,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: crate::Reg<tcd4_soff::TCD4_SOFF_SPEC>,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: crate::Reg<tcd4_attr::TCD4_ATTR_SPEC>,
    _reserved_79_dma_tcd4_nbytes: [u8; 0x04],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: crate::Reg<tcd4_slast::TCD4_SLAST_SPEC>,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: crate::Reg<tcd4_daddr::TCD4_DADDR_SPEC>,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: crate::Reg<tcd4_doff::TCD4_DOFF_SPEC>,
    _reserved_83_dma_tcd4_citer: [u8; 0x02],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: crate::Reg<tcd4_dlastsga::TCD4_DLASTSGA_SPEC>,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: crate::Reg<tcd4_csr::TCD4_CSR_SPEC>,
    _reserved_86_dma_tcd4_biter: [u8; 0x02],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: crate::Reg<tcd5_saddr::TCD5_SADDR_SPEC>,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: crate::Reg<tcd5_soff::TCD5_SOFF_SPEC>,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: crate::Reg<tcd5_attr::TCD5_ATTR_SPEC>,
    _reserved_90_dma_tcd5_nbytes: [u8; 0x04],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: crate::Reg<tcd5_slast::TCD5_SLAST_SPEC>,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: crate::Reg<tcd5_daddr::TCD5_DADDR_SPEC>,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: crate::Reg<tcd5_doff::TCD5_DOFF_SPEC>,
    _reserved_94_dma_tcd5_citer: [u8; 0x02],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: crate::Reg<tcd5_dlastsga::TCD5_DLASTSGA_SPEC>,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: crate::Reg<tcd5_csr::TCD5_CSR_SPEC>,
    _reserved_97_dma_tcd5_biter: [u8; 0x02],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: crate::Reg<tcd6_saddr::TCD6_SADDR_SPEC>,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: crate::Reg<tcd6_soff::TCD6_SOFF_SPEC>,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: crate::Reg<tcd6_attr::TCD6_ATTR_SPEC>,
    _reserved_101_dma_tcd6_nbytes: [u8; 0x04],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: crate::Reg<tcd6_slast::TCD6_SLAST_SPEC>,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: crate::Reg<tcd6_daddr::TCD6_DADDR_SPEC>,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: crate::Reg<tcd6_doff::TCD6_DOFF_SPEC>,
    _reserved_105_dma_tcd6_citer: [u8; 0x02],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: crate::Reg<tcd6_dlastsga::TCD6_DLASTSGA_SPEC>,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: crate::Reg<tcd6_csr::TCD6_CSR_SPEC>,
    _reserved_108_dma_tcd6_biter: [u8; 0x02],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: crate::Reg<tcd7_saddr::TCD7_SADDR_SPEC>,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: crate::Reg<tcd7_soff::TCD7_SOFF_SPEC>,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: crate::Reg<tcd7_attr::TCD7_ATTR_SPEC>,
    _reserved_112_dma_tcd7_nbytes: [u8; 0x04],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: crate::Reg<tcd7_slast::TCD7_SLAST_SPEC>,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: crate::Reg<tcd7_daddr::TCD7_DADDR_SPEC>,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: crate::Reg<tcd7_doff::TCD7_DOFF_SPEC>,
    _reserved_116_dma_tcd7_citer: [u8; 0x02],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: crate::Reg<tcd7_dlastsga::TCD7_DLASTSGA_SPEC>,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: crate::Reg<tcd7_csr::TCD7_CSR_SPEC>,
    _reserved_119_dma_tcd7_biter: [u8; 0x02],
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: crate::Reg<tcd8_saddr::TCD8_SADDR_SPEC>,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: crate::Reg<tcd8_soff::TCD8_SOFF_SPEC>,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: crate::Reg<tcd8_attr::TCD8_ATTR_SPEC>,
    _reserved_123_dma_tcd8_nbytes: [u8; 0x04],
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: crate::Reg<tcd8_slast::TCD8_SLAST_SPEC>,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: crate::Reg<tcd8_daddr::TCD8_DADDR_SPEC>,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: crate::Reg<tcd8_doff::TCD8_DOFF_SPEC>,
    _reserved_127_dma_tcd8_citer: [u8; 0x02],
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: crate::Reg<tcd8_dlastsga::TCD8_DLASTSGA_SPEC>,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: crate::Reg<tcd8_csr::TCD8_CSR_SPEC>,
    _reserved_130_dma_tcd8_biter: [u8; 0x02],
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: crate::Reg<tcd9_saddr::TCD9_SADDR_SPEC>,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: crate::Reg<tcd9_soff::TCD9_SOFF_SPEC>,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: crate::Reg<tcd9_attr::TCD9_ATTR_SPEC>,
    _reserved_134_dma_tcd9_nbytes: [u8; 0x04],
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: crate::Reg<tcd9_slast::TCD9_SLAST_SPEC>,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: crate::Reg<tcd9_daddr::TCD9_DADDR_SPEC>,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: crate::Reg<tcd9_doff::TCD9_DOFF_SPEC>,
    _reserved_138_dma_tcd9_citer: [u8; 0x02],
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: crate::Reg<tcd9_dlastsga::TCD9_DLASTSGA_SPEC>,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: crate::Reg<tcd9_csr::TCD9_CSR_SPEC>,
    _reserved_141_dma_tcd9_biter: [u8; 0x02],
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: crate::Reg<tcd10_saddr::TCD10_SADDR_SPEC>,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: crate::Reg<tcd10_soff::TCD10_SOFF_SPEC>,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: crate::Reg<tcd10_attr::TCD10_ATTR_SPEC>,
    _reserved_145_dma_tcd10_nbytes: [u8; 0x04],
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: crate::Reg<tcd10_slast::TCD10_SLAST_SPEC>,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: crate::Reg<tcd10_daddr::TCD10_DADDR_SPEC>,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: crate::Reg<tcd10_doff::TCD10_DOFF_SPEC>,
    _reserved_149_dma_tcd10_citer: [u8; 0x02],
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: crate::Reg<tcd10_dlastsga::TCD10_DLASTSGA_SPEC>,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: crate::Reg<tcd10_csr::TCD10_CSR_SPEC>,
    _reserved_152_dma_tcd10_biter: [u8; 0x02],
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: crate::Reg<tcd11_saddr::TCD11_SADDR_SPEC>,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: crate::Reg<tcd11_soff::TCD11_SOFF_SPEC>,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: crate::Reg<tcd11_attr::TCD11_ATTR_SPEC>,
    _reserved_156_dma_tcd11_nbytes: [u8; 0x04],
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: crate::Reg<tcd11_slast::TCD11_SLAST_SPEC>,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: crate::Reg<tcd11_daddr::TCD11_DADDR_SPEC>,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: crate::Reg<tcd11_doff::TCD11_DOFF_SPEC>,
    _reserved_160_dma_tcd11_citer: [u8; 0x02],
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: crate::Reg<tcd11_dlastsga::TCD11_DLASTSGA_SPEC>,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: crate::Reg<tcd11_csr::TCD11_CSR_SPEC>,
    _reserved_163_dma_tcd11_biter: [u8; 0x02],
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: crate::Reg<tcd12_saddr::TCD12_SADDR_SPEC>,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: crate::Reg<tcd12_soff::TCD12_SOFF_SPEC>,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: crate::Reg<tcd12_attr::TCD12_ATTR_SPEC>,
    _reserved_167_dma_tcd12_nbytes: [u8; 0x04],
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: crate::Reg<tcd12_slast::TCD12_SLAST_SPEC>,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: crate::Reg<tcd12_daddr::TCD12_DADDR_SPEC>,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: crate::Reg<tcd12_doff::TCD12_DOFF_SPEC>,
    _reserved_171_dma_tcd12_citer: [u8; 0x02],
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: crate::Reg<tcd12_dlastsga::TCD12_DLASTSGA_SPEC>,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: crate::Reg<tcd12_csr::TCD12_CSR_SPEC>,
    _reserved_174_dma_tcd12_biter: [u8; 0x02],
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: crate::Reg<tcd13_saddr::TCD13_SADDR_SPEC>,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: crate::Reg<tcd13_soff::TCD13_SOFF_SPEC>,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: crate::Reg<tcd13_attr::TCD13_ATTR_SPEC>,
    _reserved_178_dma_tcd13_nbytes: [u8; 0x04],
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: crate::Reg<tcd13_slast::TCD13_SLAST_SPEC>,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: crate::Reg<tcd13_daddr::TCD13_DADDR_SPEC>,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: crate::Reg<tcd13_doff::TCD13_DOFF_SPEC>,
    _reserved_182_dma_tcd13_citer: [u8; 0x02],
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: crate::Reg<tcd13_dlastsga::TCD13_DLASTSGA_SPEC>,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: crate::Reg<tcd13_csr::TCD13_CSR_SPEC>,
    _reserved_185_dma_tcd13_biter: [u8; 0x02],
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: crate::Reg<tcd14_saddr::TCD14_SADDR_SPEC>,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: crate::Reg<tcd14_soff::TCD14_SOFF_SPEC>,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: crate::Reg<tcd14_attr::TCD14_ATTR_SPEC>,
    _reserved_189_dma_tcd14_nbytes: [u8; 0x04],
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: crate::Reg<tcd14_slast::TCD14_SLAST_SPEC>,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: crate::Reg<tcd14_daddr::TCD14_DADDR_SPEC>,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: crate::Reg<tcd14_doff::TCD14_DOFF_SPEC>,
    _reserved_193_dma_tcd14_citer: [u8; 0x02],
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: crate::Reg<tcd14_dlastsga::TCD14_DLASTSGA_SPEC>,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: crate::Reg<tcd14_csr::TCD14_CSR_SPEC>,
    _reserved_196_dma_tcd14_biter: [u8; 0x02],
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: crate::Reg<tcd15_saddr::TCD15_SADDR_SPEC>,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: crate::Reg<tcd15_soff::TCD15_SOFF_SPEC>,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: crate::Reg<tcd15_attr::TCD15_ATTR_SPEC>,
    _reserved_200_dma_tcd15_nbytes: [u8; 0x04],
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: crate::Reg<tcd15_slast::TCD15_SLAST_SPEC>,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: crate::Reg<tcd15_daddr::TCD15_DADDR_SPEC>,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: crate::Reg<tcd15_doff::TCD15_DOFF_SPEC>,
    _reserved_204_dma_tcd15_citer: [u8; 0x02],
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: crate::Reg<tcd15_dlastsga::TCD15_DLASTSGA_SPEC>,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: crate::Reg<tcd15_csr::TCD15_CSR_SPEC>,
    _reserved_207_dma_tcd15_biter: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd0_nbytes_mloffyes::DMA_TCD0_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd0_nbytes_mloffyes::DMA_TCD0_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd0_nbytes_mloffno::DMA_TCD0_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd0_nbytes_mloffno::DMA_TCD0_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd0_nbytes_mlno::DMA_TCD0_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4104usize)
                as *const crate::Reg<dma_tcd0_nbytes_mlno::DMA_TCD0_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd0_citer_elinkyes::DMA_TCD0_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize)
                as *const crate::Reg<dma_tcd0_citer_elinkyes::DMA_TCD0_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd0_citer_elinkno::DMA_TCD0_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4118usize)
                as *const crate::Reg<dma_tcd0_citer_elinkno::DMA_TCD0_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd0_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd0_biter_elinkyes::DMA_TCD0_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize)
                as *const crate::Reg<dma_tcd0_biter_elinkyes::DMA_TCD0_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd0_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd0_biter_elinkno::DMA_TCD0_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4126usize)
                as *const crate::Reg<dma_tcd0_biter_elinkno::DMA_TCD0_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd1_nbytes_mloffyes::DMA_TCD1_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd1_nbytes_mloffyes::DMA_TCD1_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd1_nbytes_mloffno::DMA_TCD1_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd1_nbytes_mloffno::DMA_TCD1_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd1_nbytes_mlno::DMA_TCD1_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4136usize)
                as *const crate::Reg<dma_tcd1_nbytes_mlno::DMA_TCD1_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd1_citer_elinkyes::DMA_TCD1_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize)
                as *const crate::Reg<dma_tcd1_citer_elinkyes::DMA_TCD1_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd1_citer_elinkno::DMA_TCD1_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4150usize)
                as *const crate::Reg<dma_tcd1_citer_elinkno::DMA_TCD1_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd1_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd1_biter_elinkyes::DMA_TCD1_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize)
                as *const crate::Reg<dma_tcd1_biter_elinkyes::DMA_TCD1_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd1_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd1_biter_elinkno::DMA_TCD1_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4158usize)
                as *const crate::Reg<dma_tcd1_biter_elinkno::DMA_TCD1_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd2_nbytes_mloffyes::DMA_TCD2_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd2_nbytes_mloffyes::DMA_TCD2_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd2_nbytes_mloffno::DMA_TCD2_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd2_nbytes_mloffno::DMA_TCD2_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd2_nbytes_mlno::DMA_TCD2_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4168usize)
                as *const crate::Reg<dma_tcd2_nbytes_mlno::DMA_TCD2_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd2_citer_elinkyes::DMA_TCD2_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize)
                as *const crate::Reg<dma_tcd2_citer_elinkyes::DMA_TCD2_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd2_citer_elinkno::DMA_TCD2_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4182usize)
                as *const crate::Reg<dma_tcd2_citer_elinkno::DMA_TCD2_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd2_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd2_biter_elinkyes::DMA_TCD2_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize)
                as *const crate::Reg<dma_tcd2_biter_elinkyes::DMA_TCD2_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd2_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd2_biter_elinkno::DMA_TCD2_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4190usize)
                as *const crate::Reg<dma_tcd2_biter_elinkno::DMA_TCD2_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd3_nbytes_mloffyes::DMA_TCD3_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd3_nbytes_mloffyes::DMA_TCD3_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd3_nbytes_mloffno::DMA_TCD3_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd3_nbytes_mloffno::DMA_TCD3_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd3_nbytes_mlno::DMA_TCD3_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4200usize)
                as *const crate::Reg<dma_tcd3_nbytes_mlno::DMA_TCD3_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd3_citer_elinkyes::DMA_TCD3_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize)
                as *const crate::Reg<dma_tcd3_citer_elinkyes::DMA_TCD3_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd3_citer_elinkno::DMA_TCD3_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4214usize)
                as *const crate::Reg<dma_tcd3_citer_elinkno::DMA_TCD3_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd3_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd3_biter_elinkyes::DMA_TCD3_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize)
                as *const crate::Reg<dma_tcd3_biter_elinkyes::DMA_TCD3_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd3_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd3_biter_elinkno::DMA_TCD3_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4222usize)
                as *const crate::Reg<dma_tcd3_biter_elinkno::DMA_TCD3_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd4_nbytes_mloffyes::DMA_TCD4_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd4_nbytes_mloffyes::DMA_TCD4_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd4_nbytes_mloffno::DMA_TCD4_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd4_nbytes_mloffno::DMA_TCD4_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd4_nbytes_mlno::DMA_TCD4_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4232usize)
                as *const crate::Reg<dma_tcd4_nbytes_mlno::DMA_TCD4_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd4_citer_elinkyes::DMA_TCD4_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize)
                as *const crate::Reg<dma_tcd4_citer_elinkyes::DMA_TCD4_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd4_citer_elinkno::DMA_TCD4_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4246usize)
                as *const crate::Reg<dma_tcd4_citer_elinkno::DMA_TCD4_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd4_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd4_biter_elinkyes::DMA_TCD4_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize)
                as *const crate::Reg<dma_tcd4_biter_elinkyes::DMA_TCD4_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd4_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd4_biter_elinkno::DMA_TCD4_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4254usize)
                as *const crate::Reg<dma_tcd4_biter_elinkno::DMA_TCD4_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd5_nbytes_mloffyes::DMA_TCD5_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd5_nbytes_mloffyes::DMA_TCD5_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd5_nbytes_mloffno::DMA_TCD5_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd5_nbytes_mloffno::DMA_TCD5_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd5_nbytes_mlno::DMA_TCD5_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4264usize)
                as *const crate::Reg<dma_tcd5_nbytes_mlno::DMA_TCD5_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd5_citer_elinkyes::DMA_TCD5_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize)
                as *const crate::Reg<dma_tcd5_citer_elinkyes::DMA_TCD5_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd5_citer_elinkno::DMA_TCD5_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4278usize)
                as *const crate::Reg<dma_tcd5_citer_elinkno::DMA_TCD5_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd5_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd5_biter_elinkyes::DMA_TCD5_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize)
                as *const crate::Reg<dma_tcd5_biter_elinkyes::DMA_TCD5_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd5_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd5_biter_elinkno::DMA_TCD5_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4286usize)
                as *const crate::Reg<dma_tcd5_biter_elinkno::DMA_TCD5_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd6_nbytes_mloffyes::DMA_TCD6_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd6_nbytes_mloffyes::DMA_TCD6_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd6_nbytes_mloffno::DMA_TCD6_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd6_nbytes_mloffno::DMA_TCD6_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd6_nbytes_mlno::DMA_TCD6_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4296usize)
                as *const crate::Reg<dma_tcd6_nbytes_mlno::DMA_TCD6_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd6_citer_elinkyes::DMA_TCD6_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize)
                as *const crate::Reg<dma_tcd6_citer_elinkyes::DMA_TCD6_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd6_citer_elinkno::DMA_TCD6_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4310usize)
                as *const crate::Reg<dma_tcd6_citer_elinkno::DMA_TCD6_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd6_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd6_biter_elinkyes::DMA_TCD6_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize)
                as *const crate::Reg<dma_tcd6_biter_elinkyes::DMA_TCD6_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd6_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd6_biter_elinkno::DMA_TCD6_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4318usize)
                as *const crate::Reg<dma_tcd6_biter_elinkno::DMA_TCD6_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd7_nbytes_mloffyes::DMA_TCD7_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd7_nbytes_mloffyes::DMA_TCD7_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd7_nbytes_mloffno::DMA_TCD7_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd7_nbytes_mloffno::DMA_TCD7_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd7_nbytes_mlno::DMA_TCD7_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4328usize)
                as *const crate::Reg<dma_tcd7_nbytes_mlno::DMA_TCD7_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd7_citer_elinkyes::DMA_TCD7_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize)
                as *const crate::Reg<dma_tcd7_citer_elinkyes::DMA_TCD7_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd7_citer_elinkno::DMA_TCD7_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4342usize)
                as *const crate::Reg<dma_tcd7_citer_elinkno::DMA_TCD7_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd7_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd7_biter_elinkyes::DMA_TCD7_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize)
                as *const crate::Reg<dma_tcd7_biter_elinkyes::DMA_TCD7_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd7_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd7_biter_elinkno::DMA_TCD7_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4350usize)
                as *const crate::Reg<dma_tcd7_biter_elinkno::DMA_TCD7_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd8_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd8_nbytes_mloffyes::DMA_TCD8_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4360usize)
                as *const crate::Reg<dma_tcd8_nbytes_mloffyes::DMA_TCD8_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd8_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd8_nbytes_mloffno::DMA_TCD8_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4360usize)
                as *const crate::Reg<dma_tcd8_nbytes_mloffno::DMA_TCD8_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd8_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd8_nbytes_mlno::DMA_TCD8_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4360usize)
                as *const crate::Reg<dma_tcd8_nbytes_mlno::DMA_TCD8_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd8_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd8_citer_elinkyes::DMA_TCD8_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4374usize)
                as *const crate::Reg<dma_tcd8_citer_elinkyes::DMA_TCD8_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd8_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd8_citer_elinkno::DMA_TCD8_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4374usize)
                as *const crate::Reg<dma_tcd8_citer_elinkno::DMA_TCD8_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd8_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd8_biter_elinkyes::DMA_TCD8_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4382usize)
                as *const crate::Reg<dma_tcd8_biter_elinkyes::DMA_TCD8_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd8_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd8_biter_elinkno::DMA_TCD8_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4382usize)
                as *const crate::Reg<dma_tcd8_biter_elinkno::DMA_TCD8_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd9_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd9_nbytes_mloffyes::DMA_TCD9_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4392usize)
                as *const crate::Reg<dma_tcd9_nbytes_mloffyes::DMA_TCD9_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd9_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd9_nbytes_mloffno::DMA_TCD9_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4392usize)
                as *const crate::Reg<dma_tcd9_nbytes_mloffno::DMA_TCD9_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd9_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd9_nbytes_mlno::DMA_TCD9_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4392usize)
                as *const crate::Reg<dma_tcd9_nbytes_mlno::DMA_TCD9_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd9_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd9_citer_elinkyes::DMA_TCD9_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4406usize)
                as *const crate::Reg<dma_tcd9_citer_elinkyes::DMA_TCD9_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd9_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd9_citer_elinkno::DMA_TCD9_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4406usize)
                as *const crate::Reg<dma_tcd9_citer_elinkno::DMA_TCD9_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd9_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd9_biter_elinkyes::DMA_TCD9_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4414usize)
                as *const crate::Reg<dma_tcd9_biter_elinkyes::DMA_TCD9_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd9_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd9_biter_elinkno::DMA_TCD9_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4414usize)
                as *const crate::Reg<dma_tcd9_biter_elinkno::DMA_TCD9_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd10_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd10_nbytes_mloffyes::DMA_TCD10_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4424usize)
                as *const crate::Reg<dma_tcd10_nbytes_mloffyes::DMA_TCD10_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd10_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd10_nbytes_mloffno::DMA_TCD10_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4424usize)
                as *const crate::Reg<dma_tcd10_nbytes_mloffno::DMA_TCD10_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd10_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd10_nbytes_mlno::DMA_TCD10_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4424usize)
                as *const crate::Reg<dma_tcd10_nbytes_mlno::DMA_TCD10_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd10_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd10_citer_elinkyes::DMA_TCD10_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4438usize)
                as *const crate::Reg<dma_tcd10_citer_elinkyes::DMA_TCD10_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd10_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd10_citer_elinkno::DMA_TCD10_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4438usize)
                as *const crate::Reg<dma_tcd10_citer_elinkno::DMA_TCD10_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd10_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd10_biter_elinkyes::DMA_TCD10_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4446usize)
                as *const crate::Reg<dma_tcd10_biter_elinkyes::DMA_TCD10_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd10_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd10_biter_elinkno::DMA_TCD10_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4446usize)
                as *const crate::Reg<dma_tcd10_biter_elinkno::DMA_TCD10_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd11_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd11_nbytes_mloffyes::DMA_TCD11_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4456usize)
                as *const crate::Reg<dma_tcd11_nbytes_mloffyes::DMA_TCD11_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd11_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd11_nbytes_mloffno::DMA_TCD11_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4456usize)
                as *const crate::Reg<dma_tcd11_nbytes_mloffno::DMA_TCD11_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd11_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd11_nbytes_mlno::DMA_TCD11_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4456usize)
                as *const crate::Reg<dma_tcd11_nbytes_mlno::DMA_TCD11_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd11_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd11_citer_elinkyes::DMA_TCD11_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4470usize)
                as *const crate::Reg<dma_tcd11_citer_elinkyes::DMA_TCD11_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd11_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd11_citer_elinkno::DMA_TCD11_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4470usize)
                as *const crate::Reg<dma_tcd11_citer_elinkno::DMA_TCD11_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd11_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd11_biter_elinkyes::DMA_TCD11_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4478usize)
                as *const crate::Reg<dma_tcd11_biter_elinkyes::DMA_TCD11_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd11_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd11_biter_elinkno::DMA_TCD11_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4478usize)
                as *const crate::Reg<dma_tcd11_biter_elinkno::DMA_TCD11_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd12_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd12_nbytes_mloffyes::DMA_TCD12_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4488usize)
                as *const crate::Reg<dma_tcd12_nbytes_mloffyes::DMA_TCD12_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd12_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd12_nbytes_mloffno::DMA_TCD12_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4488usize)
                as *const crate::Reg<dma_tcd12_nbytes_mloffno::DMA_TCD12_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd12_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd12_nbytes_mlno::DMA_TCD12_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4488usize)
                as *const crate::Reg<dma_tcd12_nbytes_mlno::DMA_TCD12_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd12_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd12_citer_elinkyes::DMA_TCD12_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4502usize)
                as *const crate::Reg<dma_tcd12_citer_elinkyes::DMA_TCD12_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd12_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd12_citer_elinkno::DMA_TCD12_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4502usize)
                as *const crate::Reg<dma_tcd12_citer_elinkno::DMA_TCD12_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd12_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd12_biter_elinkyes::DMA_TCD12_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4510usize)
                as *const crate::Reg<dma_tcd12_biter_elinkyes::DMA_TCD12_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd12_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd12_biter_elinkno::DMA_TCD12_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4510usize)
                as *const crate::Reg<dma_tcd12_biter_elinkno::DMA_TCD12_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd13_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd13_nbytes_mloffyes::DMA_TCD13_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4520usize)
                as *const crate::Reg<dma_tcd13_nbytes_mloffyes::DMA_TCD13_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd13_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd13_nbytes_mloffno::DMA_TCD13_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4520usize)
                as *const crate::Reg<dma_tcd13_nbytes_mloffno::DMA_TCD13_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd13_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd13_nbytes_mlno::DMA_TCD13_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4520usize)
                as *const crate::Reg<dma_tcd13_nbytes_mlno::DMA_TCD13_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd13_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd13_citer_elinkyes::DMA_TCD13_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4534usize)
                as *const crate::Reg<dma_tcd13_citer_elinkyes::DMA_TCD13_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd13_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd13_citer_elinkno::DMA_TCD13_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4534usize)
                as *const crate::Reg<dma_tcd13_citer_elinkno::DMA_TCD13_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd13_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd13_biter_elinkyes::DMA_TCD13_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4542usize)
                as *const crate::Reg<dma_tcd13_biter_elinkyes::DMA_TCD13_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd13_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd13_biter_elinkno::DMA_TCD13_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4542usize)
                as *const crate::Reg<dma_tcd13_biter_elinkno::DMA_TCD13_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd14_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd14_nbytes_mloffyes::DMA_TCD14_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4552usize)
                as *const crate::Reg<dma_tcd14_nbytes_mloffyes::DMA_TCD14_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd14_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd14_nbytes_mloffno::DMA_TCD14_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4552usize)
                as *const crate::Reg<dma_tcd14_nbytes_mloffno::DMA_TCD14_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd14_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd14_nbytes_mlno::DMA_TCD14_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4552usize)
                as *const crate::Reg<dma_tcd14_nbytes_mlno::DMA_TCD14_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd14_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd14_citer_elinkyes::DMA_TCD14_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4566usize)
                as *const crate::Reg<dma_tcd14_citer_elinkyes::DMA_TCD14_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd14_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd14_citer_elinkno::DMA_TCD14_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4566usize)
                as *const crate::Reg<dma_tcd14_citer_elinkno::DMA_TCD14_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd14_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd14_biter_elinkyes::DMA_TCD14_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4574usize)
                as *const crate::Reg<dma_tcd14_biter_elinkyes::DMA_TCD14_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd14_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd14_biter_elinkno::DMA_TCD14_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4574usize)
                as *const crate::Reg<dma_tcd14_biter_elinkno::DMA_TCD14_BITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn dma_tcd15_nbytes_mloffyes(
        &self,
    ) -> &crate::Reg<dma_tcd15_nbytes_mloffyes::DMA_TCD15_NBYTES_MLOFFYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4584usize)
                as *const crate::Reg<dma_tcd15_nbytes_mloffyes::DMA_TCD15_NBYTES_MLOFFYES_SPEC>)
        }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn dma_tcd15_nbytes_mloffno(
        &self,
    ) -> &crate::Reg<dma_tcd15_nbytes_mloffno::DMA_TCD15_NBYTES_MLOFFNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4584usize)
                as *const crate::Reg<dma_tcd15_nbytes_mloffno::DMA_TCD15_NBYTES_MLOFFNO_SPEC>)
        }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn dma_tcd15_nbytes_mlno(
        &self,
    ) -> &crate::Reg<dma_tcd15_nbytes_mlno::DMA_TCD15_NBYTES_MLNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4584usize)
                as *const crate::Reg<dma_tcd15_nbytes_mlno::DMA_TCD15_NBYTES_MLNO_SPEC>)
        }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd15_citer_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd15_citer_elinkyes::DMA_TCD15_CITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4598usize)
                as *const crate::Reg<dma_tcd15_citer_elinkyes::DMA_TCD15_CITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd15_citer_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd15_citer_elinkno::DMA_TCD15_CITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4598usize)
                as *const crate::Reg<dma_tcd15_citer_elinkno::DMA_TCD15_CITER_ELINKNO_SPEC>)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn dma_tcd15_biter_elinkyes(
        &self,
    ) -> &crate::Reg<dma_tcd15_biter_elinkyes::DMA_TCD15_BITER_ELINKYES_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4606usize)
                as *const crate::Reg<dma_tcd15_biter_elinkyes::DMA_TCD15_BITER_ELINKYES_SPEC>)
        }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn dma_tcd15_biter_elinkno(
        &self,
    ) -> &crate::Reg<dma_tcd15_biter_elinkno::DMA_TCD15_BITER_ELINKNO_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4606usize)
                as *const crate::Reg<dma_tcd15_biter_elinkno::DMA_TCD15_BITER_ELINKNO_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ES register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "Error Status Register"]
pub mod es;
#[doc = "ERQ register accessor: an alias for `Reg<ERQ_SPEC>`"]
pub type ERQ = crate::Reg<erq::ERQ_SPEC>;
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "EEI register accessor: an alias for `Reg<EEI_SPEC>`"]
pub type EEI = crate::Reg<eei::EEI_SPEC>;
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "CEEI register accessor: an alias for `Reg<CEEI_SPEC>`"]
pub type CEEI = crate::Reg<ceei::CEEI_SPEC>;
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "SEEI register accessor: an alias for `Reg<SEEI_SPEC>`"]
pub type SEEI = crate::Reg<seei::SEEI_SPEC>;
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "CERQ register accessor: an alias for `Reg<CERQ_SPEC>`"]
pub type CERQ = crate::Reg<cerq::CERQ_SPEC>;
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "SERQ register accessor: an alias for `Reg<SERQ_SPEC>`"]
pub type SERQ = crate::Reg<serq::SERQ_SPEC>;
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "CDNE register accessor: an alias for `Reg<CDNE_SPEC>`"]
pub type CDNE = crate::Reg<cdne::CDNE_SPEC>;
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "SSRT register accessor: an alias for `Reg<SSRT_SPEC>`"]
pub type SSRT = crate::Reg<ssrt::SSRT_SPEC>;
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "CERR register accessor: an alias for `Reg<CERR_SPEC>`"]
pub type CERR = crate::Reg<cerr::CERR_SPEC>;
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "CINT register accessor: an alias for `Reg<CINT_SPEC>`"]
pub type CINT = crate::Reg<cint::CINT_SPEC>;
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "ERR register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Register"]
pub mod err;
#[doc = "HRS register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "EARS register accessor: an alias for `Reg<EARS_SPEC>`"]
pub type EARS = crate::Reg<ears::EARS_SPEC>;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "DCHPRI3 register accessor: an alias for `Reg<DCHPRI3_SPEC>`"]
pub type DCHPRI3 = crate::Reg<dchpri3::DCHPRI3_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri3;
#[doc = "DCHPRI2 register accessor: an alias for `Reg<DCHPRI2_SPEC>`"]
pub type DCHPRI2 = crate::Reg<dchpri2::DCHPRI2_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri2;
#[doc = "DCHPRI1 register accessor: an alias for `Reg<DCHPRI1_SPEC>`"]
pub type DCHPRI1 = crate::Reg<dchpri1::DCHPRI1_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri1;
#[doc = "DCHPRI0 register accessor: an alias for `Reg<DCHPRI0_SPEC>`"]
pub type DCHPRI0 = crate::Reg<dchpri0::DCHPRI0_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri0;
#[doc = "DCHPRI7 register accessor: an alias for `Reg<DCHPRI7_SPEC>`"]
pub type DCHPRI7 = crate::Reg<dchpri7::DCHPRI7_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri7;
#[doc = "DCHPRI6 register accessor: an alias for `Reg<DCHPRI6_SPEC>`"]
pub type DCHPRI6 = crate::Reg<dchpri6::DCHPRI6_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri6;
#[doc = "DCHPRI5 register accessor: an alias for `Reg<DCHPRI5_SPEC>`"]
pub type DCHPRI5 = crate::Reg<dchpri5::DCHPRI5_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri5;
#[doc = "DCHPRI4 register accessor: an alias for `Reg<DCHPRI4_SPEC>`"]
pub type DCHPRI4 = crate::Reg<dchpri4::DCHPRI4_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri4;
#[doc = "DCHPRI11 register accessor: an alias for `Reg<DCHPRI11_SPEC>`"]
pub type DCHPRI11 = crate::Reg<dchpri11::DCHPRI11_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri11;
#[doc = "DCHPRI10 register accessor: an alias for `Reg<DCHPRI10_SPEC>`"]
pub type DCHPRI10 = crate::Reg<dchpri10::DCHPRI10_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri10;
#[doc = "DCHPRI9 register accessor: an alias for `Reg<DCHPRI9_SPEC>`"]
pub type DCHPRI9 = crate::Reg<dchpri9::DCHPRI9_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri9;
#[doc = "DCHPRI8 register accessor: an alias for `Reg<DCHPRI8_SPEC>`"]
pub type DCHPRI8 = crate::Reg<dchpri8::DCHPRI8_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri8;
#[doc = "DCHPRI15 register accessor: an alias for `Reg<DCHPRI15_SPEC>`"]
pub type DCHPRI15 = crate::Reg<dchpri15::DCHPRI15_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri15;
#[doc = "DCHPRI14 register accessor: an alias for `Reg<DCHPRI14_SPEC>`"]
pub type DCHPRI14 = crate::Reg<dchpri14::DCHPRI14_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri14;
#[doc = "DCHPRI13 register accessor: an alias for `Reg<DCHPRI13_SPEC>`"]
pub type DCHPRI13 = crate::Reg<dchpri13::DCHPRI13_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri13;
#[doc = "DCHPRI12 register accessor: an alias for `Reg<DCHPRI12_SPEC>`"]
pub type DCHPRI12 = crate::Reg<dchpri12::DCHPRI12_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri12;
#[doc = "TCD0_SADDR register accessor: an alias for `Reg<TCD0_SADDR_SPEC>`"]
pub type TCD0_SADDR = crate::Reg<tcd0_saddr::TCD0_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd0_saddr;
#[doc = "TCD0_SOFF register accessor: an alias for `Reg<TCD0_SOFF_SPEC>`"]
pub type TCD0_SOFF = crate::Reg<tcd0_soff::TCD0_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd0_soff;
#[doc = "TCD0_ATTR register accessor: an alias for `Reg<TCD0_ATTR_SPEC>`"]
pub type TCD0_ATTR = crate::Reg<tcd0_attr::TCD0_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd0_attr;
#[doc = "DMA_TCD0_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD0_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD0_NBYTES_MLNO = crate::Reg<dma_tcd0_nbytes_mlno::DMA_TCD0_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd0_nbytes_mlno;
#[doc = "DMA_TCD0_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD0_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD0_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd0_nbytes_mloffno::DMA_TCD0_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd0_nbytes_mloffno;
#[doc = "DMA_TCD0_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD0_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD0_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd0_nbytes_mloffyes::DMA_TCD0_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd0_nbytes_mloffyes;
#[doc = "TCD0_SLAST register accessor: an alias for `Reg<TCD0_SLAST_SPEC>`"]
pub type TCD0_SLAST = crate::Reg<tcd0_slast::TCD0_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd0_slast;
#[doc = "TCD0_DADDR register accessor: an alias for `Reg<TCD0_DADDR_SPEC>`"]
pub type TCD0_DADDR = crate::Reg<tcd0_daddr::TCD0_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd0_daddr;
#[doc = "TCD0_DOFF register accessor: an alias for `Reg<TCD0_DOFF_SPEC>`"]
pub type TCD0_DOFF = crate::Reg<tcd0_doff::TCD0_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd0_doff;
#[doc = "DMA_TCD0_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD0_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD0_CITER_ELINKNO = crate::Reg<dma_tcd0_citer_elinkno::DMA_TCD0_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd0_citer_elinkno;
#[doc = "DMA_TCD0_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD0_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD0_CITER_ELINKYES =
    crate::Reg<dma_tcd0_citer_elinkyes::DMA_TCD0_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd0_citer_elinkyes;
#[doc = "TCD0_DLASTSGA register accessor: an alias for `Reg<TCD0_DLASTSGA_SPEC>`"]
pub type TCD0_DLASTSGA = crate::Reg<tcd0_dlastsga::TCD0_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd0_dlastsga;
#[doc = "TCD0_CSR register accessor: an alias for `Reg<TCD0_CSR_SPEC>`"]
pub type TCD0_CSR = crate::Reg<tcd0_csr::TCD0_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd0_csr;
#[doc = "DMA_TCD0_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD0_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD0_BITER_ELINKNO = crate::Reg<dma_tcd0_biter_elinkno::DMA_TCD0_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd0_biter_elinkno;
#[doc = "DMA_TCD0_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD0_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD0_BITER_ELINKYES =
    crate::Reg<dma_tcd0_biter_elinkyes::DMA_TCD0_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd0_biter_elinkyes;
#[doc = "TCD1_SADDR register accessor: an alias for `Reg<TCD1_SADDR_SPEC>`"]
pub type TCD1_SADDR = crate::Reg<tcd1_saddr::TCD1_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd1_saddr;
#[doc = "TCD1_SOFF register accessor: an alias for `Reg<TCD1_SOFF_SPEC>`"]
pub type TCD1_SOFF = crate::Reg<tcd1_soff::TCD1_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd1_soff;
#[doc = "TCD1_ATTR register accessor: an alias for `Reg<TCD1_ATTR_SPEC>`"]
pub type TCD1_ATTR = crate::Reg<tcd1_attr::TCD1_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd1_attr;
#[doc = "DMA_TCD1_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD1_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD1_NBYTES_MLNO = crate::Reg<dma_tcd1_nbytes_mlno::DMA_TCD1_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd1_nbytes_mlno;
#[doc = "DMA_TCD1_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD1_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD1_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd1_nbytes_mloffno::DMA_TCD1_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd1_nbytes_mloffno;
#[doc = "DMA_TCD1_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD1_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD1_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd1_nbytes_mloffyes::DMA_TCD1_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd1_nbytes_mloffyes;
#[doc = "TCD1_SLAST register accessor: an alias for `Reg<TCD1_SLAST_SPEC>`"]
pub type TCD1_SLAST = crate::Reg<tcd1_slast::TCD1_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd1_slast;
#[doc = "TCD1_DADDR register accessor: an alias for `Reg<TCD1_DADDR_SPEC>`"]
pub type TCD1_DADDR = crate::Reg<tcd1_daddr::TCD1_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd1_daddr;
#[doc = "TCD1_DOFF register accessor: an alias for `Reg<TCD1_DOFF_SPEC>`"]
pub type TCD1_DOFF = crate::Reg<tcd1_doff::TCD1_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd1_doff;
#[doc = "DMA_TCD1_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD1_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD1_CITER_ELINKNO = crate::Reg<dma_tcd1_citer_elinkno::DMA_TCD1_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd1_citer_elinkno;
#[doc = "DMA_TCD1_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD1_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD1_CITER_ELINKYES =
    crate::Reg<dma_tcd1_citer_elinkyes::DMA_TCD1_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd1_citer_elinkyes;
#[doc = "TCD1_DLASTSGA register accessor: an alias for `Reg<TCD1_DLASTSGA_SPEC>`"]
pub type TCD1_DLASTSGA = crate::Reg<tcd1_dlastsga::TCD1_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd1_dlastsga;
#[doc = "TCD1_CSR register accessor: an alias for `Reg<TCD1_CSR_SPEC>`"]
pub type TCD1_CSR = crate::Reg<tcd1_csr::TCD1_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd1_csr;
#[doc = "DMA_TCD1_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD1_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD1_BITER_ELINKNO = crate::Reg<dma_tcd1_biter_elinkno::DMA_TCD1_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd1_biter_elinkno;
#[doc = "DMA_TCD1_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD1_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD1_BITER_ELINKYES =
    crate::Reg<dma_tcd1_biter_elinkyes::DMA_TCD1_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd1_biter_elinkyes;
#[doc = "TCD2_SADDR register accessor: an alias for `Reg<TCD2_SADDR_SPEC>`"]
pub type TCD2_SADDR = crate::Reg<tcd2_saddr::TCD2_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd2_saddr;
#[doc = "TCD2_SOFF register accessor: an alias for `Reg<TCD2_SOFF_SPEC>`"]
pub type TCD2_SOFF = crate::Reg<tcd2_soff::TCD2_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd2_soff;
#[doc = "TCD2_ATTR register accessor: an alias for `Reg<TCD2_ATTR_SPEC>`"]
pub type TCD2_ATTR = crate::Reg<tcd2_attr::TCD2_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd2_attr;
#[doc = "DMA_TCD2_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD2_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD2_NBYTES_MLNO = crate::Reg<dma_tcd2_nbytes_mlno::DMA_TCD2_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd2_nbytes_mlno;
#[doc = "DMA_TCD2_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD2_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD2_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd2_nbytes_mloffno::DMA_TCD2_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd2_nbytes_mloffno;
#[doc = "DMA_TCD2_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD2_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD2_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd2_nbytes_mloffyes::DMA_TCD2_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd2_nbytes_mloffyes;
#[doc = "TCD2_SLAST register accessor: an alias for `Reg<TCD2_SLAST_SPEC>`"]
pub type TCD2_SLAST = crate::Reg<tcd2_slast::TCD2_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd2_slast;
#[doc = "TCD2_DADDR register accessor: an alias for `Reg<TCD2_DADDR_SPEC>`"]
pub type TCD2_DADDR = crate::Reg<tcd2_daddr::TCD2_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd2_daddr;
#[doc = "TCD2_DOFF register accessor: an alias for `Reg<TCD2_DOFF_SPEC>`"]
pub type TCD2_DOFF = crate::Reg<tcd2_doff::TCD2_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd2_doff;
#[doc = "DMA_TCD2_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD2_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD2_CITER_ELINKNO = crate::Reg<dma_tcd2_citer_elinkno::DMA_TCD2_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd2_citer_elinkno;
#[doc = "DMA_TCD2_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD2_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD2_CITER_ELINKYES =
    crate::Reg<dma_tcd2_citer_elinkyes::DMA_TCD2_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd2_citer_elinkyes;
#[doc = "TCD2_DLASTSGA register accessor: an alias for `Reg<TCD2_DLASTSGA_SPEC>`"]
pub type TCD2_DLASTSGA = crate::Reg<tcd2_dlastsga::TCD2_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd2_dlastsga;
#[doc = "TCD2_CSR register accessor: an alias for `Reg<TCD2_CSR_SPEC>`"]
pub type TCD2_CSR = crate::Reg<tcd2_csr::TCD2_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd2_csr;
#[doc = "DMA_TCD2_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD2_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD2_BITER_ELINKNO = crate::Reg<dma_tcd2_biter_elinkno::DMA_TCD2_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd2_biter_elinkno;
#[doc = "DMA_TCD2_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD2_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD2_BITER_ELINKYES =
    crate::Reg<dma_tcd2_biter_elinkyes::DMA_TCD2_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd2_biter_elinkyes;
#[doc = "TCD3_SADDR register accessor: an alias for `Reg<TCD3_SADDR_SPEC>`"]
pub type TCD3_SADDR = crate::Reg<tcd3_saddr::TCD3_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd3_saddr;
#[doc = "TCD3_SOFF register accessor: an alias for `Reg<TCD3_SOFF_SPEC>`"]
pub type TCD3_SOFF = crate::Reg<tcd3_soff::TCD3_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd3_soff;
#[doc = "TCD3_ATTR register accessor: an alias for `Reg<TCD3_ATTR_SPEC>`"]
pub type TCD3_ATTR = crate::Reg<tcd3_attr::TCD3_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd3_attr;
#[doc = "DMA_TCD3_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD3_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD3_NBYTES_MLNO = crate::Reg<dma_tcd3_nbytes_mlno::DMA_TCD3_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd3_nbytes_mlno;
#[doc = "DMA_TCD3_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD3_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD3_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd3_nbytes_mloffno::DMA_TCD3_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd3_nbytes_mloffno;
#[doc = "DMA_TCD3_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD3_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD3_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd3_nbytes_mloffyes::DMA_TCD3_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd3_nbytes_mloffyes;
#[doc = "TCD3_SLAST register accessor: an alias for `Reg<TCD3_SLAST_SPEC>`"]
pub type TCD3_SLAST = crate::Reg<tcd3_slast::TCD3_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd3_slast;
#[doc = "TCD3_DADDR register accessor: an alias for `Reg<TCD3_DADDR_SPEC>`"]
pub type TCD3_DADDR = crate::Reg<tcd3_daddr::TCD3_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd3_daddr;
#[doc = "TCD3_DOFF register accessor: an alias for `Reg<TCD3_DOFF_SPEC>`"]
pub type TCD3_DOFF = crate::Reg<tcd3_doff::TCD3_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd3_doff;
#[doc = "DMA_TCD3_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD3_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD3_CITER_ELINKNO = crate::Reg<dma_tcd3_citer_elinkno::DMA_TCD3_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd3_citer_elinkno;
#[doc = "DMA_TCD3_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD3_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD3_CITER_ELINKYES =
    crate::Reg<dma_tcd3_citer_elinkyes::DMA_TCD3_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd3_citer_elinkyes;
#[doc = "TCD3_DLASTSGA register accessor: an alias for `Reg<TCD3_DLASTSGA_SPEC>`"]
pub type TCD3_DLASTSGA = crate::Reg<tcd3_dlastsga::TCD3_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd3_dlastsga;
#[doc = "TCD3_CSR register accessor: an alias for `Reg<TCD3_CSR_SPEC>`"]
pub type TCD3_CSR = crate::Reg<tcd3_csr::TCD3_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd3_csr;
#[doc = "DMA_TCD3_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD3_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD3_BITER_ELINKNO = crate::Reg<dma_tcd3_biter_elinkno::DMA_TCD3_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd3_biter_elinkno;
#[doc = "DMA_TCD3_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD3_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD3_BITER_ELINKYES =
    crate::Reg<dma_tcd3_biter_elinkyes::DMA_TCD3_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd3_biter_elinkyes;
#[doc = "TCD4_SADDR register accessor: an alias for `Reg<TCD4_SADDR_SPEC>`"]
pub type TCD4_SADDR = crate::Reg<tcd4_saddr::TCD4_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd4_saddr;
#[doc = "TCD4_SOFF register accessor: an alias for `Reg<TCD4_SOFF_SPEC>`"]
pub type TCD4_SOFF = crate::Reg<tcd4_soff::TCD4_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd4_soff;
#[doc = "TCD4_ATTR register accessor: an alias for `Reg<TCD4_ATTR_SPEC>`"]
pub type TCD4_ATTR = crate::Reg<tcd4_attr::TCD4_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd4_attr;
#[doc = "DMA_TCD4_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD4_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD4_NBYTES_MLNO = crate::Reg<dma_tcd4_nbytes_mlno::DMA_TCD4_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd4_nbytes_mlno;
#[doc = "DMA_TCD4_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD4_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD4_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd4_nbytes_mloffno::DMA_TCD4_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd4_nbytes_mloffno;
#[doc = "DMA_TCD4_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD4_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD4_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd4_nbytes_mloffyes::DMA_TCD4_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd4_nbytes_mloffyes;
#[doc = "TCD4_SLAST register accessor: an alias for `Reg<TCD4_SLAST_SPEC>`"]
pub type TCD4_SLAST = crate::Reg<tcd4_slast::TCD4_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd4_slast;
#[doc = "TCD4_DADDR register accessor: an alias for `Reg<TCD4_DADDR_SPEC>`"]
pub type TCD4_DADDR = crate::Reg<tcd4_daddr::TCD4_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd4_daddr;
#[doc = "TCD4_DOFF register accessor: an alias for `Reg<TCD4_DOFF_SPEC>`"]
pub type TCD4_DOFF = crate::Reg<tcd4_doff::TCD4_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd4_doff;
#[doc = "DMA_TCD4_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD4_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD4_CITER_ELINKNO = crate::Reg<dma_tcd4_citer_elinkno::DMA_TCD4_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd4_citer_elinkno;
#[doc = "DMA_TCD4_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD4_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD4_CITER_ELINKYES =
    crate::Reg<dma_tcd4_citer_elinkyes::DMA_TCD4_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd4_citer_elinkyes;
#[doc = "TCD4_DLASTSGA register accessor: an alias for `Reg<TCD4_DLASTSGA_SPEC>`"]
pub type TCD4_DLASTSGA = crate::Reg<tcd4_dlastsga::TCD4_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd4_dlastsga;
#[doc = "TCD4_CSR register accessor: an alias for `Reg<TCD4_CSR_SPEC>`"]
pub type TCD4_CSR = crate::Reg<tcd4_csr::TCD4_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd4_csr;
#[doc = "DMA_TCD4_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD4_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD4_BITER_ELINKNO = crate::Reg<dma_tcd4_biter_elinkno::DMA_TCD4_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd4_biter_elinkno;
#[doc = "DMA_TCD4_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD4_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD4_BITER_ELINKYES =
    crate::Reg<dma_tcd4_biter_elinkyes::DMA_TCD4_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd4_biter_elinkyes;
#[doc = "TCD5_SADDR register accessor: an alias for `Reg<TCD5_SADDR_SPEC>`"]
pub type TCD5_SADDR = crate::Reg<tcd5_saddr::TCD5_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd5_saddr;
#[doc = "TCD5_SOFF register accessor: an alias for `Reg<TCD5_SOFF_SPEC>`"]
pub type TCD5_SOFF = crate::Reg<tcd5_soff::TCD5_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd5_soff;
#[doc = "TCD5_ATTR register accessor: an alias for `Reg<TCD5_ATTR_SPEC>`"]
pub type TCD5_ATTR = crate::Reg<tcd5_attr::TCD5_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd5_attr;
#[doc = "DMA_TCD5_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD5_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD5_NBYTES_MLNO = crate::Reg<dma_tcd5_nbytes_mlno::DMA_TCD5_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd5_nbytes_mlno;
#[doc = "DMA_TCD5_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD5_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD5_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd5_nbytes_mloffno::DMA_TCD5_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd5_nbytes_mloffno;
#[doc = "DMA_TCD5_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD5_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD5_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd5_nbytes_mloffyes::DMA_TCD5_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd5_nbytes_mloffyes;
#[doc = "TCD5_SLAST register accessor: an alias for `Reg<TCD5_SLAST_SPEC>`"]
pub type TCD5_SLAST = crate::Reg<tcd5_slast::TCD5_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd5_slast;
#[doc = "TCD5_DADDR register accessor: an alias for `Reg<TCD5_DADDR_SPEC>`"]
pub type TCD5_DADDR = crate::Reg<tcd5_daddr::TCD5_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd5_daddr;
#[doc = "TCD5_DOFF register accessor: an alias for `Reg<TCD5_DOFF_SPEC>`"]
pub type TCD5_DOFF = crate::Reg<tcd5_doff::TCD5_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd5_doff;
#[doc = "DMA_TCD5_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD5_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD5_CITER_ELINKNO = crate::Reg<dma_tcd5_citer_elinkno::DMA_TCD5_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd5_citer_elinkno;
#[doc = "DMA_TCD5_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD5_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD5_CITER_ELINKYES =
    crate::Reg<dma_tcd5_citer_elinkyes::DMA_TCD5_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd5_citer_elinkyes;
#[doc = "TCD5_DLASTSGA register accessor: an alias for `Reg<TCD5_DLASTSGA_SPEC>`"]
pub type TCD5_DLASTSGA = crate::Reg<tcd5_dlastsga::TCD5_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd5_dlastsga;
#[doc = "TCD5_CSR register accessor: an alias for `Reg<TCD5_CSR_SPEC>`"]
pub type TCD5_CSR = crate::Reg<tcd5_csr::TCD5_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd5_csr;
#[doc = "DMA_TCD5_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD5_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD5_BITER_ELINKNO = crate::Reg<dma_tcd5_biter_elinkno::DMA_TCD5_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd5_biter_elinkno;
#[doc = "DMA_TCD5_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD5_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD5_BITER_ELINKYES =
    crate::Reg<dma_tcd5_biter_elinkyes::DMA_TCD5_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd5_biter_elinkyes;
#[doc = "TCD6_SADDR register accessor: an alias for `Reg<TCD6_SADDR_SPEC>`"]
pub type TCD6_SADDR = crate::Reg<tcd6_saddr::TCD6_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd6_saddr;
#[doc = "TCD6_SOFF register accessor: an alias for `Reg<TCD6_SOFF_SPEC>`"]
pub type TCD6_SOFF = crate::Reg<tcd6_soff::TCD6_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd6_soff;
#[doc = "TCD6_ATTR register accessor: an alias for `Reg<TCD6_ATTR_SPEC>`"]
pub type TCD6_ATTR = crate::Reg<tcd6_attr::TCD6_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd6_attr;
#[doc = "DMA_TCD6_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD6_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD6_NBYTES_MLNO = crate::Reg<dma_tcd6_nbytes_mlno::DMA_TCD6_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd6_nbytes_mlno;
#[doc = "DMA_TCD6_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD6_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD6_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd6_nbytes_mloffno::DMA_TCD6_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd6_nbytes_mloffno;
#[doc = "DMA_TCD6_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD6_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD6_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd6_nbytes_mloffyes::DMA_TCD6_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd6_nbytes_mloffyes;
#[doc = "TCD6_SLAST register accessor: an alias for `Reg<TCD6_SLAST_SPEC>`"]
pub type TCD6_SLAST = crate::Reg<tcd6_slast::TCD6_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd6_slast;
#[doc = "TCD6_DADDR register accessor: an alias for `Reg<TCD6_DADDR_SPEC>`"]
pub type TCD6_DADDR = crate::Reg<tcd6_daddr::TCD6_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd6_daddr;
#[doc = "TCD6_DOFF register accessor: an alias for `Reg<TCD6_DOFF_SPEC>`"]
pub type TCD6_DOFF = crate::Reg<tcd6_doff::TCD6_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd6_doff;
#[doc = "DMA_TCD6_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD6_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD6_CITER_ELINKNO = crate::Reg<dma_tcd6_citer_elinkno::DMA_TCD6_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd6_citer_elinkno;
#[doc = "DMA_TCD6_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD6_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD6_CITER_ELINKYES =
    crate::Reg<dma_tcd6_citer_elinkyes::DMA_TCD6_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd6_citer_elinkyes;
#[doc = "TCD6_DLASTSGA register accessor: an alias for `Reg<TCD6_DLASTSGA_SPEC>`"]
pub type TCD6_DLASTSGA = crate::Reg<tcd6_dlastsga::TCD6_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd6_dlastsga;
#[doc = "TCD6_CSR register accessor: an alias for `Reg<TCD6_CSR_SPEC>`"]
pub type TCD6_CSR = crate::Reg<tcd6_csr::TCD6_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd6_csr;
#[doc = "DMA_TCD6_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD6_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD6_BITER_ELINKNO = crate::Reg<dma_tcd6_biter_elinkno::DMA_TCD6_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd6_biter_elinkno;
#[doc = "DMA_TCD6_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD6_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD6_BITER_ELINKYES =
    crate::Reg<dma_tcd6_biter_elinkyes::DMA_TCD6_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd6_biter_elinkyes;
#[doc = "TCD7_SADDR register accessor: an alias for `Reg<TCD7_SADDR_SPEC>`"]
pub type TCD7_SADDR = crate::Reg<tcd7_saddr::TCD7_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd7_saddr;
#[doc = "TCD7_SOFF register accessor: an alias for `Reg<TCD7_SOFF_SPEC>`"]
pub type TCD7_SOFF = crate::Reg<tcd7_soff::TCD7_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd7_soff;
#[doc = "TCD7_ATTR register accessor: an alias for `Reg<TCD7_ATTR_SPEC>`"]
pub type TCD7_ATTR = crate::Reg<tcd7_attr::TCD7_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd7_attr;
#[doc = "DMA_TCD7_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD7_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD7_NBYTES_MLNO = crate::Reg<dma_tcd7_nbytes_mlno::DMA_TCD7_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd7_nbytes_mlno;
#[doc = "DMA_TCD7_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD7_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD7_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd7_nbytes_mloffno::DMA_TCD7_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd7_nbytes_mloffno;
#[doc = "DMA_TCD7_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD7_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD7_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd7_nbytes_mloffyes::DMA_TCD7_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd7_nbytes_mloffyes;
#[doc = "TCD7_SLAST register accessor: an alias for `Reg<TCD7_SLAST_SPEC>`"]
pub type TCD7_SLAST = crate::Reg<tcd7_slast::TCD7_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd7_slast;
#[doc = "TCD7_DADDR register accessor: an alias for `Reg<TCD7_DADDR_SPEC>`"]
pub type TCD7_DADDR = crate::Reg<tcd7_daddr::TCD7_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd7_daddr;
#[doc = "TCD7_DOFF register accessor: an alias for `Reg<TCD7_DOFF_SPEC>`"]
pub type TCD7_DOFF = crate::Reg<tcd7_doff::TCD7_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd7_doff;
#[doc = "DMA_TCD7_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD7_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD7_CITER_ELINKNO = crate::Reg<dma_tcd7_citer_elinkno::DMA_TCD7_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd7_citer_elinkno;
#[doc = "DMA_TCD7_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD7_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD7_CITER_ELINKYES =
    crate::Reg<dma_tcd7_citer_elinkyes::DMA_TCD7_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd7_citer_elinkyes;
#[doc = "TCD7_DLASTSGA register accessor: an alias for `Reg<TCD7_DLASTSGA_SPEC>`"]
pub type TCD7_DLASTSGA = crate::Reg<tcd7_dlastsga::TCD7_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd7_dlastsga;
#[doc = "TCD7_CSR register accessor: an alias for `Reg<TCD7_CSR_SPEC>`"]
pub type TCD7_CSR = crate::Reg<tcd7_csr::TCD7_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd7_csr;
#[doc = "DMA_TCD7_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD7_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD7_BITER_ELINKNO = crate::Reg<dma_tcd7_biter_elinkno::DMA_TCD7_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd7_biter_elinkno;
#[doc = "DMA_TCD7_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD7_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD7_BITER_ELINKYES =
    crate::Reg<dma_tcd7_biter_elinkyes::DMA_TCD7_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd7_biter_elinkyes;
#[doc = "TCD8_SADDR register accessor: an alias for `Reg<TCD8_SADDR_SPEC>`"]
pub type TCD8_SADDR = crate::Reg<tcd8_saddr::TCD8_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd8_saddr;
#[doc = "TCD8_SOFF register accessor: an alias for `Reg<TCD8_SOFF_SPEC>`"]
pub type TCD8_SOFF = crate::Reg<tcd8_soff::TCD8_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd8_soff;
#[doc = "TCD8_ATTR register accessor: an alias for `Reg<TCD8_ATTR_SPEC>`"]
pub type TCD8_ATTR = crate::Reg<tcd8_attr::TCD8_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd8_attr;
#[doc = "DMA_TCD8_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD8_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD8_NBYTES_MLNO = crate::Reg<dma_tcd8_nbytes_mlno::DMA_TCD8_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd8_nbytes_mlno;
#[doc = "DMA_TCD8_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD8_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD8_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd8_nbytes_mloffno::DMA_TCD8_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd8_nbytes_mloffno;
#[doc = "DMA_TCD8_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD8_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD8_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd8_nbytes_mloffyes::DMA_TCD8_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd8_nbytes_mloffyes;
#[doc = "TCD8_SLAST register accessor: an alias for `Reg<TCD8_SLAST_SPEC>`"]
pub type TCD8_SLAST = crate::Reg<tcd8_slast::TCD8_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd8_slast;
#[doc = "TCD8_DADDR register accessor: an alias for `Reg<TCD8_DADDR_SPEC>`"]
pub type TCD8_DADDR = crate::Reg<tcd8_daddr::TCD8_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd8_daddr;
#[doc = "TCD8_DOFF register accessor: an alias for `Reg<TCD8_DOFF_SPEC>`"]
pub type TCD8_DOFF = crate::Reg<tcd8_doff::TCD8_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd8_doff;
#[doc = "DMA_TCD8_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD8_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD8_CITER_ELINKNO = crate::Reg<dma_tcd8_citer_elinkno::DMA_TCD8_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd8_citer_elinkno;
#[doc = "DMA_TCD8_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD8_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD8_CITER_ELINKYES =
    crate::Reg<dma_tcd8_citer_elinkyes::DMA_TCD8_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd8_citer_elinkyes;
#[doc = "TCD8_DLASTSGA register accessor: an alias for `Reg<TCD8_DLASTSGA_SPEC>`"]
pub type TCD8_DLASTSGA = crate::Reg<tcd8_dlastsga::TCD8_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd8_dlastsga;
#[doc = "TCD8_CSR register accessor: an alias for `Reg<TCD8_CSR_SPEC>`"]
pub type TCD8_CSR = crate::Reg<tcd8_csr::TCD8_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd8_csr;
#[doc = "DMA_TCD8_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD8_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD8_BITER_ELINKNO = crate::Reg<dma_tcd8_biter_elinkno::DMA_TCD8_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd8_biter_elinkno;
#[doc = "DMA_TCD8_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD8_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD8_BITER_ELINKYES =
    crate::Reg<dma_tcd8_biter_elinkyes::DMA_TCD8_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd8_biter_elinkyes;
#[doc = "TCD9_SADDR register accessor: an alias for `Reg<TCD9_SADDR_SPEC>`"]
pub type TCD9_SADDR = crate::Reg<tcd9_saddr::TCD9_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd9_saddr;
#[doc = "TCD9_SOFF register accessor: an alias for `Reg<TCD9_SOFF_SPEC>`"]
pub type TCD9_SOFF = crate::Reg<tcd9_soff::TCD9_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd9_soff;
#[doc = "TCD9_ATTR register accessor: an alias for `Reg<TCD9_ATTR_SPEC>`"]
pub type TCD9_ATTR = crate::Reg<tcd9_attr::TCD9_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd9_attr;
#[doc = "DMA_TCD9_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD9_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD9_NBYTES_MLNO = crate::Reg<dma_tcd9_nbytes_mlno::DMA_TCD9_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd9_nbytes_mlno;
#[doc = "DMA_TCD9_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD9_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD9_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd9_nbytes_mloffno::DMA_TCD9_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd9_nbytes_mloffno;
#[doc = "DMA_TCD9_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD9_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD9_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd9_nbytes_mloffyes::DMA_TCD9_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd9_nbytes_mloffyes;
#[doc = "TCD9_SLAST register accessor: an alias for `Reg<TCD9_SLAST_SPEC>`"]
pub type TCD9_SLAST = crate::Reg<tcd9_slast::TCD9_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd9_slast;
#[doc = "TCD9_DADDR register accessor: an alias for `Reg<TCD9_DADDR_SPEC>`"]
pub type TCD9_DADDR = crate::Reg<tcd9_daddr::TCD9_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd9_daddr;
#[doc = "TCD9_DOFF register accessor: an alias for `Reg<TCD9_DOFF_SPEC>`"]
pub type TCD9_DOFF = crate::Reg<tcd9_doff::TCD9_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd9_doff;
#[doc = "DMA_TCD9_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD9_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD9_CITER_ELINKNO = crate::Reg<dma_tcd9_citer_elinkno::DMA_TCD9_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd9_citer_elinkno;
#[doc = "DMA_TCD9_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD9_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD9_CITER_ELINKYES =
    crate::Reg<dma_tcd9_citer_elinkyes::DMA_TCD9_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd9_citer_elinkyes;
#[doc = "TCD9_DLASTSGA register accessor: an alias for `Reg<TCD9_DLASTSGA_SPEC>`"]
pub type TCD9_DLASTSGA = crate::Reg<tcd9_dlastsga::TCD9_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd9_dlastsga;
#[doc = "TCD9_CSR register accessor: an alias for `Reg<TCD9_CSR_SPEC>`"]
pub type TCD9_CSR = crate::Reg<tcd9_csr::TCD9_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd9_csr;
#[doc = "DMA_TCD9_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD9_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD9_BITER_ELINKNO = crate::Reg<dma_tcd9_biter_elinkno::DMA_TCD9_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd9_biter_elinkno;
#[doc = "DMA_TCD9_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD9_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD9_BITER_ELINKYES =
    crate::Reg<dma_tcd9_biter_elinkyes::DMA_TCD9_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd9_biter_elinkyes;
#[doc = "TCD10_SADDR register accessor: an alias for `Reg<TCD10_SADDR_SPEC>`"]
pub type TCD10_SADDR = crate::Reg<tcd10_saddr::TCD10_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd10_saddr;
#[doc = "TCD10_SOFF register accessor: an alias for `Reg<TCD10_SOFF_SPEC>`"]
pub type TCD10_SOFF = crate::Reg<tcd10_soff::TCD10_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd10_soff;
#[doc = "TCD10_ATTR register accessor: an alias for `Reg<TCD10_ATTR_SPEC>`"]
pub type TCD10_ATTR = crate::Reg<tcd10_attr::TCD10_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd10_attr;
#[doc = "DMA_TCD10_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD10_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD10_NBYTES_MLNO = crate::Reg<dma_tcd10_nbytes_mlno::DMA_TCD10_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd10_nbytes_mlno;
#[doc = "DMA_TCD10_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD10_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD10_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd10_nbytes_mloffno::DMA_TCD10_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd10_nbytes_mloffno;
#[doc = "DMA_TCD10_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD10_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD10_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd10_nbytes_mloffyes::DMA_TCD10_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd10_nbytes_mloffyes;
#[doc = "TCD10_SLAST register accessor: an alias for `Reg<TCD10_SLAST_SPEC>`"]
pub type TCD10_SLAST = crate::Reg<tcd10_slast::TCD10_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd10_slast;
#[doc = "TCD10_DADDR register accessor: an alias for `Reg<TCD10_DADDR_SPEC>`"]
pub type TCD10_DADDR = crate::Reg<tcd10_daddr::TCD10_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd10_daddr;
#[doc = "TCD10_DOFF register accessor: an alias for `Reg<TCD10_DOFF_SPEC>`"]
pub type TCD10_DOFF = crate::Reg<tcd10_doff::TCD10_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd10_doff;
#[doc = "DMA_TCD10_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD10_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD10_CITER_ELINKNO =
    crate::Reg<dma_tcd10_citer_elinkno::DMA_TCD10_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd10_citer_elinkno;
#[doc = "DMA_TCD10_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD10_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD10_CITER_ELINKYES =
    crate::Reg<dma_tcd10_citer_elinkyes::DMA_TCD10_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd10_citer_elinkyes;
#[doc = "TCD10_DLASTSGA register accessor: an alias for `Reg<TCD10_DLASTSGA_SPEC>`"]
pub type TCD10_DLASTSGA = crate::Reg<tcd10_dlastsga::TCD10_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd10_dlastsga;
#[doc = "TCD10_CSR register accessor: an alias for `Reg<TCD10_CSR_SPEC>`"]
pub type TCD10_CSR = crate::Reg<tcd10_csr::TCD10_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd10_csr;
#[doc = "DMA_TCD10_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD10_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD10_BITER_ELINKNO =
    crate::Reg<dma_tcd10_biter_elinkno::DMA_TCD10_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd10_biter_elinkno;
#[doc = "DMA_TCD10_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD10_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD10_BITER_ELINKYES =
    crate::Reg<dma_tcd10_biter_elinkyes::DMA_TCD10_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd10_biter_elinkyes;
#[doc = "TCD11_SADDR register accessor: an alias for `Reg<TCD11_SADDR_SPEC>`"]
pub type TCD11_SADDR = crate::Reg<tcd11_saddr::TCD11_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd11_saddr;
#[doc = "TCD11_SOFF register accessor: an alias for `Reg<TCD11_SOFF_SPEC>`"]
pub type TCD11_SOFF = crate::Reg<tcd11_soff::TCD11_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd11_soff;
#[doc = "TCD11_ATTR register accessor: an alias for `Reg<TCD11_ATTR_SPEC>`"]
pub type TCD11_ATTR = crate::Reg<tcd11_attr::TCD11_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd11_attr;
#[doc = "DMA_TCD11_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD11_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD11_NBYTES_MLNO = crate::Reg<dma_tcd11_nbytes_mlno::DMA_TCD11_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd11_nbytes_mlno;
#[doc = "DMA_TCD11_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD11_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD11_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd11_nbytes_mloffno::DMA_TCD11_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd11_nbytes_mloffno;
#[doc = "DMA_TCD11_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD11_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD11_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd11_nbytes_mloffyes::DMA_TCD11_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd11_nbytes_mloffyes;
#[doc = "TCD11_SLAST register accessor: an alias for `Reg<TCD11_SLAST_SPEC>`"]
pub type TCD11_SLAST = crate::Reg<tcd11_slast::TCD11_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd11_slast;
#[doc = "TCD11_DADDR register accessor: an alias for `Reg<TCD11_DADDR_SPEC>`"]
pub type TCD11_DADDR = crate::Reg<tcd11_daddr::TCD11_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd11_daddr;
#[doc = "TCD11_DOFF register accessor: an alias for `Reg<TCD11_DOFF_SPEC>`"]
pub type TCD11_DOFF = crate::Reg<tcd11_doff::TCD11_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd11_doff;
#[doc = "DMA_TCD11_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD11_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD11_CITER_ELINKNO =
    crate::Reg<dma_tcd11_citer_elinkno::DMA_TCD11_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd11_citer_elinkno;
#[doc = "DMA_TCD11_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD11_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD11_CITER_ELINKYES =
    crate::Reg<dma_tcd11_citer_elinkyes::DMA_TCD11_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd11_citer_elinkyes;
#[doc = "TCD11_DLASTSGA register accessor: an alias for `Reg<TCD11_DLASTSGA_SPEC>`"]
pub type TCD11_DLASTSGA = crate::Reg<tcd11_dlastsga::TCD11_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd11_dlastsga;
#[doc = "TCD11_CSR register accessor: an alias for `Reg<TCD11_CSR_SPEC>`"]
pub type TCD11_CSR = crate::Reg<tcd11_csr::TCD11_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd11_csr;
#[doc = "DMA_TCD11_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD11_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD11_BITER_ELINKNO =
    crate::Reg<dma_tcd11_biter_elinkno::DMA_TCD11_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd11_biter_elinkno;
#[doc = "DMA_TCD11_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD11_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD11_BITER_ELINKYES =
    crate::Reg<dma_tcd11_biter_elinkyes::DMA_TCD11_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd11_biter_elinkyes;
#[doc = "TCD12_SADDR register accessor: an alias for `Reg<TCD12_SADDR_SPEC>`"]
pub type TCD12_SADDR = crate::Reg<tcd12_saddr::TCD12_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd12_saddr;
#[doc = "TCD12_SOFF register accessor: an alias for `Reg<TCD12_SOFF_SPEC>`"]
pub type TCD12_SOFF = crate::Reg<tcd12_soff::TCD12_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd12_soff;
#[doc = "TCD12_ATTR register accessor: an alias for `Reg<TCD12_ATTR_SPEC>`"]
pub type TCD12_ATTR = crate::Reg<tcd12_attr::TCD12_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd12_attr;
#[doc = "DMA_TCD12_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD12_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD12_NBYTES_MLNO = crate::Reg<dma_tcd12_nbytes_mlno::DMA_TCD12_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd12_nbytes_mlno;
#[doc = "DMA_TCD12_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD12_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD12_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd12_nbytes_mloffno::DMA_TCD12_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd12_nbytes_mloffno;
#[doc = "DMA_TCD12_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD12_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD12_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd12_nbytes_mloffyes::DMA_TCD12_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd12_nbytes_mloffyes;
#[doc = "TCD12_SLAST register accessor: an alias for `Reg<TCD12_SLAST_SPEC>`"]
pub type TCD12_SLAST = crate::Reg<tcd12_slast::TCD12_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd12_slast;
#[doc = "TCD12_DADDR register accessor: an alias for `Reg<TCD12_DADDR_SPEC>`"]
pub type TCD12_DADDR = crate::Reg<tcd12_daddr::TCD12_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd12_daddr;
#[doc = "TCD12_DOFF register accessor: an alias for `Reg<TCD12_DOFF_SPEC>`"]
pub type TCD12_DOFF = crate::Reg<tcd12_doff::TCD12_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd12_doff;
#[doc = "DMA_TCD12_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD12_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD12_CITER_ELINKNO =
    crate::Reg<dma_tcd12_citer_elinkno::DMA_TCD12_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd12_citer_elinkno;
#[doc = "DMA_TCD12_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD12_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD12_CITER_ELINKYES =
    crate::Reg<dma_tcd12_citer_elinkyes::DMA_TCD12_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd12_citer_elinkyes;
#[doc = "TCD12_DLASTSGA register accessor: an alias for `Reg<TCD12_DLASTSGA_SPEC>`"]
pub type TCD12_DLASTSGA = crate::Reg<tcd12_dlastsga::TCD12_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd12_dlastsga;
#[doc = "TCD12_CSR register accessor: an alias for `Reg<TCD12_CSR_SPEC>`"]
pub type TCD12_CSR = crate::Reg<tcd12_csr::TCD12_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd12_csr;
#[doc = "DMA_TCD12_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD12_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD12_BITER_ELINKNO =
    crate::Reg<dma_tcd12_biter_elinkno::DMA_TCD12_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd12_biter_elinkno;
#[doc = "DMA_TCD12_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD12_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD12_BITER_ELINKYES =
    crate::Reg<dma_tcd12_biter_elinkyes::DMA_TCD12_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd12_biter_elinkyes;
#[doc = "TCD13_SADDR register accessor: an alias for `Reg<TCD13_SADDR_SPEC>`"]
pub type TCD13_SADDR = crate::Reg<tcd13_saddr::TCD13_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd13_saddr;
#[doc = "TCD13_SOFF register accessor: an alias for `Reg<TCD13_SOFF_SPEC>`"]
pub type TCD13_SOFF = crate::Reg<tcd13_soff::TCD13_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd13_soff;
#[doc = "TCD13_ATTR register accessor: an alias for `Reg<TCD13_ATTR_SPEC>`"]
pub type TCD13_ATTR = crate::Reg<tcd13_attr::TCD13_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd13_attr;
#[doc = "DMA_TCD13_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD13_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD13_NBYTES_MLNO = crate::Reg<dma_tcd13_nbytes_mlno::DMA_TCD13_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd13_nbytes_mlno;
#[doc = "DMA_TCD13_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD13_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD13_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd13_nbytes_mloffno::DMA_TCD13_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd13_nbytes_mloffno;
#[doc = "DMA_TCD13_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD13_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD13_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd13_nbytes_mloffyes::DMA_TCD13_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd13_nbytes_mloffyes;
#[doc = "TCD13_SLAST register accessor: an alias for `Reg<TCD13_SLAST_SPEC>`"]
pub type TCD13_SLAST = crate::Reg<tcd13_slast::TCD13_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd13_slast;
#[doc = "TCD13_DADDR register accessor: an alias for `Reg<TCD13_DADDR_SPEC>`"]
pub type TCD13_DADDR = crate::Reg<tcd13_daddr::TCD13_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd13_daddr;
#[doc = "TCD13_DOFF register accessor: an alias for `Reg<TCD13_DOFF_SPEC>`"]
pub type TCD13_DOFF = crate::Reg<tcd13_doff::TCD13_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd13_doff;
#[doc = "DMA_TCD13_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD13_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD13_CITER_ELINKNO =
    crate::Reg<dma_tcd13_citer_elinkno::DMA_TCD13_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd13_citer_elinkno;
#[doc = "DMA_TCD13_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD13_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD13_CITER_ELINKYES =
    crate::Reg<dma_tcd13_citer_elinkyes::DMA_TCD13_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd13_citer_elinkyes;
#[doc = "TCD13_DLASTSGA register accessor: an alias for `Reg<TCD13_DLASTSGA_SPEC>`"]
pub type TCD13_DLASTSGA = crate::Reg<tcd13_dlastsga::TCD13_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd13_dlastsga;
#[doc = "TCD13_CSR register accessor: an alias for `Reg<TCD13_CSR_SPEC>`"]
pub type TCD13_CSR = crate::Reg<tcd13_csr::TCD13_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd13_csr;
#[doc = "DMA_TCD13_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD13_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD13_BITER_ELINKNO =
    crate::Reg<dma_tcd13_biter_elinkno::DMA_TCD13_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd13_biter_elinkno;
#[doc = "DMA_TCD13_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD13_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD13_BITER_ELINKYES =
    crate::Reg<dma_tcd13_biter_elinkyes::DMA_TCD13_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd13_biter_elinkyes;
#[doc = "TCD14_SADDR register accessor: an alias for `Reg<TCD14_SADDR_SPEC>`"]
pub type TCD14_SADDR = crate::Reg<tcd14_saddr::TCD14_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd14_saddr;
#[doc = "TCD14_SOFF register accessor: an alias for `Reg<TCD14_SOFF_SPEC>`"]
pub type TCD14_SOFF = crate::Reg<tcd14_soff::TCD14_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd14_soff;
#[doc = "TCD14_ATTR register accessor: an alias for `Reg<TCD14_ATTR_SPEC>`"]
pub type TCD14_ATTR = crate::Reg<tcd14_attr::TCD14_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd14_attr;
#[doc = "DMA_TCD14_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD14_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD14_NBYTES_MLNO = crate::Reg<dma_tcd14_nbytes_mlno::DMA_TCD14_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd14_nbytes_mlno;
#[doc = "DMA_TCD14_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD14_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD14_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd14_nbytes_mloffno::DMA_TCD14_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd14_nbytes_mloffno;
#[doc = "DMA_TCD14_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD14_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD14_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd14_nbytes_mloffyes::DMA_TCD14_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd14_nbytes_mloffyes;
#[doc = "TCD14_SLAST register accessor: an alias for `Reg<TCD14_SLAST_SPEC>`"]
pub type TCD14_SLAST = crate::Reg<tcd14_slast::TCD14_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd14_slast;
#[doc = "TCD14_DADDR register accessor: an alias for `Reg<TCD14_DADDR_SPEC>`"]
pub type TCD14_DADDR = crate::Reg<tcd14_daddr::TCD14_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd14_daddr;
#[doc = "TCD14_DOFF register accessor: an alias for `Reg<TCD14_DOFF_SPEC>`"]
pub type TCD14_DOFF = crate::Reg<tcd14_doff::TCD14_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd14_doff;
#[doc = "DMA_TCD14_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD14_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD14_CITER_ELINKNO =
    crate::Reg<dma_tcd14_citer_elinkno::DMA_TCD14_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd14_citer_elinkno;
#[doc = "DMA_TCD14_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD14_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD14_CITER_ELINKYES =
    crate::Reg<dma_tcd14_citer_elinkyes::DMA_TCD14_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd14_citer_elinkyes;
#[doc = "TCD14_DLASTSGA register accessor: an alias for `Reg<TCD14_DLASTSGA_SPEC>`"]
pub type TCD14_DLASTSGA = crate::Reg<tcd14_dlastsga::TCD14_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd14_dlastsga;
#[doc = "TCD14_CSR register accessor: an alias for `Reg<TCD14_CSR_SPEC>`"]
pub type TCD14_CSR = crate::Reg<tcd14_csr::TCD14_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd14_csr;
#[doc = "DMA_TCD14_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD14_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD14_BITER_ELINKNO =
    crate::Reg<dma_tcd14_biter_elinkno::DMA_TCD14_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd14_biter_elinkno;
#[doc = "DMA_TCD14_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD14_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD14_BITER_ELINKYES =
    crate::Reg<dma_tcd14_biter_elinkyes::DMA_TCD14_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd14_biter_elinkyes;
#[doc = "TCD15_SADDR register accessor: an alias for `Reg<TCD15_SADDR_SPEC>`"]
pub type TCD15_SADDR = crate::Reg<tcd15_saddr::TCD15_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd15_saddr;
#[doc = "TCD15_SOFF register accessor: an alias for `Reg<TCD15_SOFF_SPEC>`"]
pub type TCD15_SOFF = crate::Reg<tcd15_soff::TCD15_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd15_soff;
#[doc = "TCD15_ATTR register accessor: an alias for `Reg<TCD15_ATTR_SPEC>`"]
pub type TCD15_ATTR = crate::Reg<tcd15_attr::TCD15_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd15_attr;
#[doc = "DMA_TCD15_NBYTES_MLNO register accessor: an alias for `Reg<DMA_TCD15_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD15_NBYTES_MLNO = crate::Reg<dma_tcd15_nbytes_mlno::DMA_TCD15_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod dma_tcd15_nbytes_mlno;
#[doc = "DMA_TCD15_NBYTES_MLOFFNO register accessor: an alias for `Reg<DMA_TCD15_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD15_NBYTES_MLOFFNO =
    crate::Reg<dma_tcd15_nbytes_mloffno::DMA_TCD15_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod dma_tcd15_nbytes_mloffno;
#[doc = "DMA_TCD15_NBYTES_MLOFFYES register accessor: an alias for `Reg<DMA_TCD15_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD15_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd15_nbytes_mloffyes::DMA_TCD15_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod dma_tcd15_nbytes_mloffyes;
#[doc = "TCD15_SLAST register accessor: an alias for `Reg<TCD15_SLAST_SPEC>`"]
pub type TCD15_SLAST = crate::Reg<tcd15_slast::TCD15_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd15_slast;
#[doc = "TCD15_DADDR register accessor: an alias for `Reg<TCD15_DADDR_SPEC>`"]
pub type TCD15_DADDR = crate::Reg<tcd15_daddr::TCD15_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd15_daddr;
#[doc = "TCD15_DOFF register accessor: an alias for `Reg<TCD15_DOFF_SPEC>`"]
pub type TCD15_DOFF = crate::Reg<tcd15_doff::TCD15_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd15_doff;
#[doc = "DMA_TCD15_CITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD15_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD15_CITER_ELINKNO =
    crate::Reg<dma_tcd15_citer_elinkno::DMA_TCD15_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd15_citer_elinkno;
#[doc = "DMA_TCD15_CITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD15_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD15_CITER_ELINKYES =
    crate::Reg<dma_tcd15_citer_elinkyes::DMA_TCD15_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd15_citer_elinkyes;
#[doc = "TCD15_DLASTSGA register accessor: an alias for `Reg<TCD15_DLASTSGA_SPEC>`"]
pub type TCD15_DLASTSGA = crate::Reg<tcd15_dlastsga::TCD15_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd15_dlastsga;
#[doc = "TCD15_CSR register accessor: an alias for `Reg<TCD15_CSR_SPEC>`"]
pub type TCD15_CSR = crate::Reg<tcd15_csr::TCD15_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd15_csr;
#[doc = "DMA_TCD15_BITER_ELINKNO register accessor: an alias for `Reg<DMA_TCD15_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD15_BITER_ELINKNO =
    crate::Reg<dma_tcd15_biter_elinkno::DMA_TCD15_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd15_biter_elinkno;
#[doc = "DMA_TCD15_BITER_ELINKYES register accessor: an alias for `Reg<DMA_TCD15_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD15_BITER_ELINKYES =
    crate::Reg<dma_tcd15_biter_elinkyes::DMA_TCD15_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd15_biter_elinkyes;
