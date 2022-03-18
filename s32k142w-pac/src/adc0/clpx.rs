#[doc = "Register `CLPX` reader"]
pub struct R(crate::R<CLPX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPX` writer"]
pub struct W(crate::W<CLPX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPX_SPEC>;
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
impl From<crate::W<CLPX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPX` reader - CLPX"]
pub struct CLPX_R(crate::FieldReader<u8, u8>);
impl CLPX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLPX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLPX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLPX` writer - CLPX"]
pub struct CLPX_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - CLPX"]
    #[inline(always)]
    pub fn clpx(&self) -> CLPX_R {
        CLPX_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - CLPX"]
    #[inline(always)]
    pub fn clpx(&mut self) -> CLPX_W {
        CLPX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpx](index.html) module"]
pub struct CLPX_SPEC;
impl crate::RegisterSpec for CLPX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clpx::R](R) reader structure"]
impl crate::Readable for CLPX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clpx::W](W) writer structure"]
impl crate::Writable for CLPX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLPX to value 0"]
impl crate::Resettable for CLPX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
