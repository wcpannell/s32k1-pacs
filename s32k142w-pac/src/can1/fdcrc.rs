#[doc = "Register `FDCRC` reader"]
pub struct R(crate::R<FDCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FD_TXCRC` reader - Extended Transmitted CRC value"]
pub struct FD_TXCRC_R(crate::FieldReader<u32, u32>);
impl FD_TXCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FD_TXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FD_TXCRC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FD_MBCRC` reader - CRC Mailbox Number for FD_TXCRC"]
pub struct FD_MBCRC_R(crate::FieldReader<u8, u8>);
impl FD_MBCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FD_MBCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FD_MBCRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:20 - Extended Transmitted CRC value"]
    #[inline(always)]
    pub fn fd_txcrc(&self) -> FD_TXCRC_R {
        FD_TXCRC_R::new((self.bits & 0x001f_ffff) as u32)
    }
    #[doc = "Bits 24:30 - CRC Mailbox Number for FD_TXCRC"]
    #[inline(always)]
    pub fn fd_mbcrc(&self) -> FD_MBCRC_R {
        FD_MBCRC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "CAN FD CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcrc](index.html) module"]
pub struct FDCRC_SPEC;
impl crate::RegisterSpec for FDCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcrc::R](R) reader structure"]
impl crate::Readable for FDCRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCRC to value 0"]
impl crate::Resettable for FDCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
