#[doc = "Register `TCD1_DOFF` reader"]
pub struct R(crate::R<TCD1_DOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD1_DOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD1_DOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD1_DOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD1_DOFF` writer"]
pub struct W(crate::W<TCD1_DOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD1_DOFF_SPEC>;
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
impl From<crate::W<TCD1_DOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD1_DOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOFF` reader - Destination Address Signed Offset"]
pub struct DOFF_R(crate::FieldReader<u16, u16>);
impl DOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOFF` writer - Destination Address Signed Offset"]
pub struct DOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> DOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Destination Address Signed Offset"]
    #[inline(always)]
    pub fn doff(&self) -> DOFF_R {
        DOFF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Destination Address Signed Offset"]
    #[inline(always)]
    pub fn doff(&mut self) -> DOFF_W {
        DOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_doff](index.html) module"]
pub struct TCD1_DOFF_SPEC;
impl crate::RegisterSpec for TCD1_DOFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd1_doff::R](R) reader structure"]
impl crate::Readable for TCD1_DOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd1_doff::W](W) writer structure"]
impl crate::Writable for TCD1_DOFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD1_DOFF to value 0"]
impl crate::Resettable for TCD1_DOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
