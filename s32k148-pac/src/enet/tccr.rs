#[doc = "Register `TCCR%s` reader"]
pub struct R(crate::R<TCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR%s` writer"]
pub struct W(crate::W<TCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR_SPEC>;
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
impl From<crate::W<TCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCC` reader - Timer Capture Compare"]
pub struct TCC_R(crate::FieldReader<u32, u32>);
impl TCC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC` writer - Timer Capture Compare"]
pub struct TCC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Timer Capture Compare"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Capture Compare"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Compare Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr](index.html) module"]
pub struct TCCR_SPEC;
impl crate::RegisterSpec for TCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr::R](R) reader structure"]
impl crate::Readable for TCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr::W](W) writer structure"]
impl crate::Writable for TCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR%s to value 0"]
impl crate::Resettable for TCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
