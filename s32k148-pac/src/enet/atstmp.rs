#[doc = "Register `ATSTMP` reader"]
pub struct R(crate::R<ATSTMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATSTMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATSTMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATSTMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMESTAMP` reader - Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\]
set"]
pub struct TIMESTAMP_R(crate::FieldReader<u32, u32>);
impl TIMESTAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TIMESTAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Timestamp of the last frame transmitted by the core that had TxBD\\[TS\\]
set"]
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(self.bits)
    }
}
#[doc = "Timestamp of Last Transmitted Frame\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atstmp](index.html) module"]
pub struct ATSTMP_SPEC;
impl crate::RegisterSpec for ATSTMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atstmp::R](R) reader structure"]
impl crate::Readable for ATSTMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ATSTMP to value 0"]
impl crate::Resettable for ATSTMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
