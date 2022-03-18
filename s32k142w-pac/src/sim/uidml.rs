#[doc = "Register `UIDML` reader"]
pub struct R(crate::R<UIDML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UIDML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UIDML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UIDML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UID63_32` reader - Unique Identification"]
pub struct UID63_32_R(crate::FieldReader<u32, u32>);
impl UID63_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UID63_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UID63_32_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid63_32(&self) -> UID63_32_R {
        UID63_32_R::new(self.bits)
    }
}
#[doc = "Unique Identification Register Mid Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidml](index.html) module"]
pub struct UIDML_SPEC;
impl crate::RegisterSpec for UIDML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uidml::R](R) reader structure"]
impl crate::Readable for UIDML_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UIDML to value 0"]
impl crate::Resettable for UIDML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
