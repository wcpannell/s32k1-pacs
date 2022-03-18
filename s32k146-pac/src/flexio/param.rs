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
#[doc = "Field `SHIFTER` reader - Shifter Number"]
pub struct SHIFTER_R(crate::FieldReader<u8, u8>);
impl SHIFTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER` reader - Timer Number"]
pub struct TIMER_R(crate::FieldReader<u8, u8>);
impl TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN` reader - Pin Number"]
pub struct PIN_R(crate::FieldReader<u8, u8>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGGER` reader - Trigger Number"]
pub struct TRIGGER_R(crate::FieldReader<u8, u8>);
impl TRIGGER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIGGER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Shifter Number"]
    #[inline(always)]
    pub fn shifter(&self) -> SHIFTER_R {
        SHIFTER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer Number"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pin Number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Trigger Number"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "`reset()` method sets PARAM to value 0x0408_0404"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0408_0404
    }
}
