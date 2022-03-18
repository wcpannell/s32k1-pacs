#[doc = "Register `COMPID%s` reader"]
pub struct R(crate::R<COMPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMPID` reader - Component ID"]
pub struct COMPID_R(crate::FieldReader<u32, u32>);
impl COMPID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COMPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID"]
    #[inline(always)]
    pub fn compid(&self) -> COMPID_R {
        COMPID_R::new(self.bits)
    }
}
#[doc = "Component ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compid](index.html) module"]
pub struct COMPID_SPEC;
impl crate::RegisterSpec for COMPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compid::R](R) reader structure"]
impl crate::Readable for COMPID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMPID%s to value 0"]
impl crate::Resettable for COMPID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
