#[doc = "Register `LMPECR` reader"]
pub struct R(crate::R<LMPECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMPECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMPECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMPECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LMEM Parity and ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmpecr](index.html) module"]
pub struct LMPECR_SPEC;
impl crate::RegisterSpec for LMPECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmpecr::R](R) reader structure"]
impl crate::Readable for LMPECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LMPECR to value 0"]
impl crate::Resettable for LMPECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
