#[doc = "Register `TCD8_DADDR` reader"]
pub struct R(crate::R<TCD8_DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD8_DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD8_DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD8_DADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD8_DADDR` writer"]
pub struct W(crate::W<TCD8_DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD8_DADDR_SPEC>;
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
impl From<crate::W<TCD8_DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD8_DADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADDR` reader - Destination Address"]
pub struct DADDR_R(crate::FieldReader<u32, u32>);
impl DADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DADDR` writer - Destination Address"]
pub struct DADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Destination Address"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Address"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DADDR_W {
        DADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_daddr](index.html) module"]
pub struct TCD8_DADDR_SPEC;
impl crate::RegisterSpec for TCD8_DADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd8_daddr::R](R) reader structure"]
impl crate::Readable for TCD8_DADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd8_daddr::W](W) writer structure"]
impl crate::Writable for TCD8_DADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD8_DADDR to value 0"]
impl crate::Resettable for TCD8_DADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
