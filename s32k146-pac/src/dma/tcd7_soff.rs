#[doc = "Register `TCD7_SOFF` reader"]
pub struct R(crate::R<TCD7_SOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD7_SOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD7_SOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD7_SOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD7_SOFF` writer"]
pub struct W(crate::W<TCD7_SOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD7_SOFF_SPEC>;
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
impl From<crate::W<TCD7_SOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD7_SOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFF` reader - Source address signed offset"]
pub struct SOFF_R(crate::FieldReader<u16, u16>);
impl SOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFF` writer - Source address signed offset"]
pub struct SOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&self) -> SOFF_R {
        SOFF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source address signed offset"]
    #[inline(always)]
    pub fn soff(&mut self) -> SOFF_W {
        SOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_soff](index.html) module"]
pub struct TCD7_SOFF_SPEC;
impl crate::RegisterSpec for TCD7_SOFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd7_soff::R](R) reader structure"]
impl crate::Readable for TCD7_SOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd7_soff::W](W) writer structure"]
impl crate::Writable for TCD7_SOFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD7_SOFF to value 0"]
impl crate::Resettable for TCD7_SOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
