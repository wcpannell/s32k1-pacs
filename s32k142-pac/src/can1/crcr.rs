#[doc = "Register `CRCR` reader"]
pub struct R(crate::R<CRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCRC` reader - Transmitted CRC value"]
pub struct TXCRC_R(crate::FieldReader<u16, u16>);
impl TXCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBCRC` reader - CRC Mailbox"]
pub struct MBCRC_R(crate::FieldReader<u8, u8>);
impl MBCRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MBCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MBCRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Transmitted CRC value"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:22 - CRC Mailbox"]
    #[inline(always)]
    pub fn mbcrc(&self) -> MBCRC_R {
        MBCRC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "CRC Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcr](index.html) module"]
pub struct CRCR_SPEC;
impl crate::RegisterSpec for CRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcr::R](R) reader structure"]
impl crate::Readable for CRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRCR to value 0"]
impl crate::Resettable for CRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
