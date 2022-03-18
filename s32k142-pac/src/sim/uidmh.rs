#[doc = "Register `UIDMH` reader"]
pub struct R(crate::R<UIDMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UIDMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UIDMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UIDMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UID95_64` reader - Unique Identification"]
pub struct UID95_64_R(crate::FieldReader<u32, u32>);
impl UID95_64_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UID95_64_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UID95_64_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid95_64(&self) -> UID95_64_R {
        UID95_64_R::new(self.bits)
    }
}
#[doc = "Unique Identification Register Mid-High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidmh](index.html) module"]
pub struct UIDMH_SPEC;
impl crate::RegisterSpec for UIDMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uidmh::R](R) reader structure"]
impl crate::Readable for UIDMH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UIDMH to value 0"]
impl crate::Resettable for UIDMH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
