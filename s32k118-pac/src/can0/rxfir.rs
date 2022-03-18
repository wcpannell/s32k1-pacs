#[doc = "Register `RXFIR` reader"]
pub struct R(crate::R<RXFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDHIT` reader - Identifier Acceptance Filter Hit Indicator"]
pub struct IDHIT_R(crate::FieldReader<u16, u16>);
impl IDHIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IDHIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDHIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - Identifier Acceptance Filter Hit Indicator"]
    #[inline(always)]
    pub fn idhit(&self) -> IDHIT_R {
        IDHIT_R::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Rx FIFO Information Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfir](index.html) module"]
pub struct RXFIR_SPEC;
impl crate::RegisterSpec for RXFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfir::R](R) reader structure"]
impl crate::Readable for RXFIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIR to value 0"]
impl crate::Resettable for RXFIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
