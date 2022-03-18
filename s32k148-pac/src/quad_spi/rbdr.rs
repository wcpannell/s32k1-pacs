#[doc = "Register `RBDR%s` reader"]
pub struct R(crate::R<RBDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA` reader - RX Data"]
pub struct RXDATA_R(crate::FieldReader<u32, u32>);
impl RXDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RXDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new(self.bits)
    }
}
#[doc = "RX Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbdr](index.html) module"]
pub struct RBDR_SPEC;
impl crate::RegisterSpec for RBDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbdr::R](R) reader structure"]
impl crate::Readable for RBDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBDR%s to value 0"]
impl crate::Resettable for RBDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
