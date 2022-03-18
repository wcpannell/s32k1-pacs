#[doc = "Register `PERIPHID%s` reader"]
pub struct R(crate::R<PERIPHID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERIPHID` reader - PERIPHID"]
pub struct PERIPHID_R(crate::FieldReader<u32, u32>);
impl PERIPHID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIPHID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPHID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - PERIPHID"]
    #[inline(always)]
    pub fn periphid(&self) -> PERIPHID_R {
        PERIPHID_R::new(self.bits)
    }
}
#[doc = "Peripheral ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periphid](index.html) module"]
pub struct PERIPHID_SPEC;
impl crate::RegisterSpec for PERIPHID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periphid::R](R) reader structure"]
impl crate::Readable for PERIPHID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERIPHID%s to value 0"]
impl crate::Resettable for PERIPHID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
