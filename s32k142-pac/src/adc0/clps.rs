#[doc = "Register `CLPS` reader"]
pub struct R(crate::R<CLPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPS` writer"]
pub struct W(crate::W<CLPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPS` reader - Calibration Value"]
pub struct CLPS_R(crate::FieldReader<u8, u8>);
impl CLPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLPS` writer - Calibration Value"]
pub struct CLPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clps(&self) -> CLPS_R {
        CLPS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clps(&mut self) -> CLPS_W {
        CLPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC General Calibration Value Register S\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps](index.html) module"]
pub struct CLPS_SPEC;
impl crate::RegisterSpec for CLPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clps::R](R) reader structure"]
impl crate::Readable for CLPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clps::W](W) writer structure"]
impl crate::Writable for CLPS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLPS to value 0x2e"]
impl crate::Resettable for CLPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2e
    }
}
