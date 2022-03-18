#[doc = "Register `RU` reader"]
pub struct R(crate::R<RU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D` reader - Data result"]
pub struct D_R(crate::FieldReader<u16, u16>);
impl D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "ADC Data Result Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ru](index.html) module"]
pub struct RU_SPEC;
impl crate::RegisterSpec for RU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ru::R](R) reader structure"]
impl crate::Readable for RU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RU to value 0"]
impl crate::Resettable for RU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
