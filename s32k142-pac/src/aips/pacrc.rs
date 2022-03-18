#[doc = "Register `PACRC` reader"]
pub struct R(crate::R<PACRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrc](index.html) module"]
pub struct PACRC_SPEC;
impl crate::RegisterSpec for PACRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pacrc::R](R) reader structure"]
impl crate::Readable for PACRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACRC to value 0"]
impl crate::Resettable for PACRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
