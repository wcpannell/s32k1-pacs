#[doc = "Register `PMSTAT` reader"]
pub struct R(crate::R<PMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PMSTAT` reader - Power Mode Status"]
pub struct PMSTAT_R(crate::FieldReader<u8, u8>);
impl PMSTAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PMSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMSTAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Power Mode Status"]
    #[inline(always)]
    pub fn pmstat(&self) -> PMSTAT_R {
        PMSTAT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Power Mode Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstat](index.html) module"]
pub struct PMSTAT_SPEC;
impl crate::RegisterSpec for PMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmstat::R](R) reader structure"]
impl crate::Readable for PMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMSTAT to value 0x01"]
impl crate::Resettable for PMSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
