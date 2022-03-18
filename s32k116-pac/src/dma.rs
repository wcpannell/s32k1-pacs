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
    _reserved20: [u8; 0x0efc],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: crate::Reg<tcd0_saddr::TCD0_SADDR_SPEC>,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: crate::Reg<tcd0_soff::TCD0_SOFF_SPEC>,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: crate::Reg<tcd0_attr::TCD0_ATTR_SPEC>,
    _reserved_23_dma_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: crate::Reg<tcd0_slast::TCD0_SLAST_SPEC>,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: crate::Reg<tcd0_daddr::TCD0_DADDR_SPEC>,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: crate::Reg<tcd0_doff::TCD0_DOFF_SPEC>,
    _reserved_27_dma_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: crate::Reg<tcd0_dlastsga::TCD0_DLASTSGA_SPEC>,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: crate::Reg<tcd0_csr::TCD0_CSR_SPEC>,
    _reserved_30_dma_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: crate::Reg<tcd1_saddr::TCD1_SADDR_SPEC>,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: crate::Reg<tcd1_soff::TCD1_SOFF_SPEC>,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: crate::Reg<tcd1_attr::TCD1_ATTR_SPEC>,
    _reserved_34_dma_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: crate::Reg<tcd1_slast::TCD1_SLAST_SPEC>,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: crate::Reg<tcd1_daddr::TCD1_DADDR_SPEC>,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: crate::Reg<tcd1_doff::TCD1_DOFF_SPEC>,
    _reserved_38_dma_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: crate::Reg<tcd1_dlastsga::TCD1_DLASTSGA_SPEC>,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: crate::Reg<tcd1_csr::TCD1_CSR_SPEC>,
    _reserved_41_dma_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: crate::Reg<tcd2_saddr::TCD2_SADDR_SPEC>,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: crate::Reg<tcd2_soff::TCD2_SOFF_SPEC>,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: crate::Reg<tcd2_attr::TCD2_ATTR_SPEC>,
    _reserved_45_dma_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: crate::Reg<tcd2_slast::TCD2_SLAST_SPEC>,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: crate::Reg<tcd2_daddr::TCD2_DADDR_SPEC>,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: crate::Reg<tcd2_doff::TCD2_DOFF_SPEC>,
    _reserved_49_dma_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: crate::Reg<tcd2_dlastsga::TCD2_DLASTSGA_SPEC>,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: crate::Reg<tcd2_csr::TCD2_CSR_SPEC>,
    _reserved_52_dma_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: crate::Reg<tcd3_saddr::TCD3_SADDR_SPEC>,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: crate::Reg<tcd3_soff::TCD3_SOFF_SPEC>,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: crate::Reg<tcd3_attr::TCD3_ATTR_SPEC>,
    _reserved_56_dma_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: crate::Reg<tcd3_slast::TCD3_SLAST_SPEC>,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: crate::Reg<tcd3_daddr::TCD3_DADDR_SPEC>,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: crate::Reg<tcd3_doff::TCD3_DOFF_SPEC>,
    _reserved_60_dma_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: crate::Reg<tcd3_dlastsga::TCD3_DLASTSGA_SPEC>,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: crate::Reg<tcd3_csr::TCD3_CSR_SPEC>,
    _reserved_63_dma_tcd3_biter: [u8; 0x02],
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
