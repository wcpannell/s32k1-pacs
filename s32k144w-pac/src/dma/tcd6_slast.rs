#[doc = "Register `TCD6_SLAST` reader"]
pub struct R(crate::R<TCD6_SLAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD6_SLAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD6_SLAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD6_SLAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD6_SLAST` writer"]
pub struct W(crate::W<TCD6_SLAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD6_SLAST_SPEC>;
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
impl From<crate::W<TCD6_SLAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD6_SLAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAST` reader - Last Source Address Adjustment"]
pub struct SLAST_R(crate::FieldReader<u32, u32>);
impl SLAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SLAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLAST` writer - Last Source Address Adjustment"]
pub struct SLAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Last Source Address Adjustment"]
    #[inline(always)]
    pub fn slast(&self) -> SLAST_R {
        SLAST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last Source Address Adjustment"]
    #[inline(always)]
    pub fn slast(&mut self) -> SLAST_W {
        SLAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_slast](index.html) module"]
pub struct TCD6_SLAST_SPEC;
impl crate::RegisterSpec for TCD6_SLAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd6_slast::R](R) reader structure"]
impl crate::Readable for TCD6_SLAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd6_slast::W](W) writer structure"]
impl crate::Writable for TCD6_SLAST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD6_SLAST to value 0"]
impl crate::Resettable for TCD6_SLAST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
