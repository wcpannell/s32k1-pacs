#[doc = "Register `DEVICECFG` reader"]
pub struct R(crate::R<DEVICECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICECFG` reader - DEVICECFG"]
pub struct DEVICECFG_R(crate::FieldReader<u32, u32>);
impl DEVICECFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEVICECFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICECFG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - DEVICECFG"]
    #[inline(always)]
    pub fn devicecfg(&self) -> DEVICECFG_R {
        DEVICECFG_R::new(self.bits)
    }
}
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicecfg](index.html) module"]
pub struct DEVICECFG_SPEC;
impl crate::RegisterSpec for DEVICECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devicecfg::R](R) reader structure"]
impl crate::Readable for DEVICECFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICECFG to value 0"]
impl crate::Resettable for DEVICECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
