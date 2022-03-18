#[doc = "Register `HCR` reader"]
pub struct R(crate::R<HCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCR` writer"]
pub struct W(crate::W<HCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCR_SPEC>;
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
impl From<crate::W<HCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCVAL` reader - Half Cycle Value"]
pub struct HCVAL_R(crate::FieldReader<u16, u16>);
impl HCVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCVAL` writer - Half Cycle Value"]
pub struct HCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Half Cycle Value"]
    #[inline(always)]
    pub fn hcval(&self) -> HCVAL_R {
        HCVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Half Cycle Value"]
    #[inline(always)]
    pub fn hcval(&mut self) -> HCVAL_W {
        HCVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Half Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcr](index.html) module"]
pub struct HCR_SPEC;
impl crate::RegisterSpec for HCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcr::R](R) reader structure"]
impl crate::Readable for HCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcr::W](W) writer structure"]
impl crate::Writable for HCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCR to value 0"]
impl crate::Resettable for HCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
