#[doc = "Register `G` reader"]
pub struct R(crate::R<G_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<G_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<G_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<G_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `G` writer"]
pub struct W(crate::W<G_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<G_SPEC>;
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
impl From<crate::W<G_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<G_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `G` reader - Gain error adjustment factor for the overall conversion"]
pub struct G_R(crate::FieldReader<u16, u16>);
impl G_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        G_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G` writer - Gain error adjustment factor for the overall conversion"]
pub struct G_W<'a> {
    w: &'a mut W,
}
impl<'a> G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Gain error adjustment factor for the overall conversion"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Gain error adjustment factor for the overall conversion"]
    #[inline(always)]
    pub fn g(&mut self) -> G_W {
        G_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [g](index.html) module"]
pub struct G_SPEC;
impl crate::RegisterSpec for G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [g::R](R) reader structure"]
impl crate::Readable for G_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [g::W](W) writer structure"]
impl crate::Writable for G_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets G to value 0x02f0"]
impl crate::Resettable for G_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02f0
    }
}
