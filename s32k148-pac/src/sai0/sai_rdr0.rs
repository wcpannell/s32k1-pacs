#[doc = "Register `SAI_RDR0` reader"]
pub struct R(crate::R<SAI_RDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_RDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_RDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_RDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDR` reader - Receive Data Register"]
pub struct RDR_R(crate::FieldReader<u32, u32>);
impl RDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive Data Register"]
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new(self.bits)
    }
}
#[doc = "SAI Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_rdr0](index.html) module"]
pub struct SAI_RDR0_SPEC;
impl crate::RegisterSpec for SAI_RDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_rdr0::R](R) reader structure"]
impl crate::Readable for SAI_RDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_RDR0 to value 0"]
impl crate::Resettable for SAI_RDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
