#[doc = "Register `DEVICETYPID` reader"]
pub struct R(crate::R<DEVICETYPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICETYPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICETYPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICETYPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICETYPID` reader - DEVICETYPID"]
pub struct DEVICETYPID_R(crate::FieldReader<u32, u32>);
impl DEVICETYPID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DEVICETYPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICETYPID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - DEVICETYPID"]
    #[inline(always)]
    pub fn devicetypid(&self) -> DEVICETYPID_R {
        DEVICETYPID_R::new(self.bits)
    }
}
#[doc = "Device Type Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicetypid](index.html) module"]
pub struct DEVICETYPID_SPEC;
impl crate::RegisterSpec for DEVICETYPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devicetypid::R](R) reader structure"]
impl crate::Readable for DEVICETYPID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICETYPID to value 0x04"]
impl crate::Resettable for DEVICETYPID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
