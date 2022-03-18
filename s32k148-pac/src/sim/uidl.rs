#[doc = "Register `UIDL` reader"]
pub struct R(crate::R<UIDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UIDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UIDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UIDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UID31_0` reader - Unique Identification"]
pub struct UID31_0_R(crate::FieldReader<u32, u32>);
impl UID31_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        UID31_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UID31_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid31_0(&self) -> UID31_0_R {
        UID31_0_R::new(self.bits)
    }
}
#[doc = "Unique Identification Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidl](index.html) module"]
pub struct UIDL_SPEC;
impl crate::RegisterSpec for UIDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uidl::R](R) reader structure"]
impl crate::Readable for UIDL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UIDL to value 0"]
impl crate::Resettable for UIDL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
