#[doc = "Register `LPOTRIM` reader"]
pub struct R(crate::R<LPOTRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPOTRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPOTRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPOTRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPOTRIM` writer"]
pub struct W(crate::W<LPOTRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPOTRIM_SPEC>;
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
impl From<crate::W<LPOTRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPOTRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPOTRIM` reader - LPO trimming bits"]
pub struct LPOTRIM_R(crate::FieldReader<u8, u8>);
impl LPOTRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPOTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPOTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOTRIM` writer - LPO trimming bits"]
pub struct LPOTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u8 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - LPO trimming bits"]
    #[inline(always)]
    pub fn lpotrim(&self) -> LPOTRIM_R {
        LPOTRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - LPO trimming bits"]
    #[inline(always)]
    pub fn lpotrim(&mut self) -> LPOTRIM_W {
        LPOTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Oscillator Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpotrim](index.html) module"]
pub struct LPOTRIM_SPEC;
impl crate::RegisterSpec for LPOTRIM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lpotrim::R](R) reader structure"]
impl crate::Readable for LPOTRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpotrim::W](W) writer structure"]
impl crate::Writable for LPOTRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPOTRIM to value 0"]
impl crate::Resettable for LPOTRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
