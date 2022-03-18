#[doc = "Register `IEEE_R_OCTETS_OK` reader"]
pub struct R(crate::R<IEEE_R_OCTETS_OK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_R_OCTETS_OK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_R_OCTETS_OK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_R_OCTETS_OK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of octets for frames received without error"]
pub struct COUNT_R(crate::FieldReader<u32, u32>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of octets for frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "Octet Count for Frames Received without Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_octets_ok](index.html) module"]
pub struct IEEE_R_OCTETS_OK_SPEC;
impl crate::RegisterSpec for IEEE_R_OCTETS_OK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_r_octets_ok::R](R) reader structure"]
impl crate::Readable for IEEE_R_OCTETS_OK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IEEE_R_OCTETS_OK to value 0"]
impl crate::Resettable for IEEE_R_OCTETS_OK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
