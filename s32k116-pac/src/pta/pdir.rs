#[doc = "Register `PDIR` reader"]
pub struct R(crate::R<PDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI` reader - Port Data Input"]
pub struct PDI_R(crate::FieldReader<u32, u32>);
impl PDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(self.bits)
    }
}
#[doc = "Port Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdir](index.html) module"]
pub struct PDIR_SPEC;
impl crate::RegisterSpec for PDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdir::R](R) reader structure"]
impl crate::Readable for PDIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDIR to value 0"]
impl crate::Resettable for PDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
