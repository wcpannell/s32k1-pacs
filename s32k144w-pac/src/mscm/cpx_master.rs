#[doc = "Register `CPxMASTER` reader"]
pub struct R(crate::R<CPXMASTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPXMASTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPXMASTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPXMASTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPMN` reader - Processor x Physical Master Number"]
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
    #[doc = "Bits 0:5 - Processor x Physical Master Number"]
    #[inline(always)]
    pub fn ppmn(&self) -> PPMN_R {
        PPMN_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Processor X Master Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_master](index.html) module"]
pub struct CPXMASTER_SPEC;
impl crate::RegisterSpec for CPXMASTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpx_master::R](R) reader structure"]
impl crate::Readable for CPXMASTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPxMASTER to value 0"]
impl crate::Resettable for CPXMASTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
