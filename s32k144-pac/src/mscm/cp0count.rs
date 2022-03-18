#[doc = "Register `CP0COUNT` reader"]
pub struct R(crate::R<CP0COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP0COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP0COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP0COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCNT` reader - Processor Count"]
pub struct PCNT_R(crate::FieldReader<u8, u8>);
impl PCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Processor Count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "Processor 0 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0count](index.html) module"]
pub struct CP0COUNT_SPEC;
impl crate::RegisterSpec for CP0COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cp0count::R](R) reader structure"]
impl crate::Readable for CP0COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CP0COUNT to value 0"]
impl crate::Resettable for CP0COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
