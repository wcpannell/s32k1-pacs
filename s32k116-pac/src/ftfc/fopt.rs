#[doc = "Register `FOPT` reader"]
pub struct R(crate::R<FOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPT` reader - Nonvolatile Option"]
pub struct OPT_R(crate::FieldReader<u8, u8>);
impl OPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Nonvolatile Option"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new(self.bits)
    }
}
#[doc = "Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](index.html) module"]
pub struct FOPT_SPEC;
impl crate::RegisterSpec for FOPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fopt::R](R) reader structure"]
impl crate::Readable for FOPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FOPT to value 0"]
impl crate::Resettable for FOPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
