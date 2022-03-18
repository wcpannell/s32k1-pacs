#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHANNEL` reader - Number of Timer Channels"]
pub struct CHANNEL_R(crate::FieldReader<u8, u8>);
impl CHANNEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_TRIG` reader - Number of External Trigger Inputs"]
pub struct EXT_TRIG_R(crate::FieldReader<u8, u8>);
impl EXT_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXT_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Timer Channels"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of External Trigger Inputs"]
    #[inline(always)]
    pub fn ext_trig(&self) -> EXT_TRIG_R {
        EXT_TRIG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x0404"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0404
    }
}
