#[doc = "Register `CP0MASTER` reader"]
pub struct R(crate::R<CP0MASTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CP0MASTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CP0MASTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CP0MASTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPMN` reader - Processor 0 Physical Master Number"]
pub struct PPMN_R(crate::FieldReader<u8, u8>);
impl PPMN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPMN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPMN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - Processor 0 Physical Master Number"]
    #[inline(always)]
    pub fn ppmn(&self) -> PPMN_R {
        PPMN_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Processor 0 Master Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cp0master](index.html) module"]
pub struct CP0MASTER_SPEC;
impl crate::RegisterSpec for CP0MASTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cp0master::R](R) reader structure"]
impl crate::Readable for CP0MASTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CP0MASTER to value 0"]
impl crate::Resettable for CP0MASTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
