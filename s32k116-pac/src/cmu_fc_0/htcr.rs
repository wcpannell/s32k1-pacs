#[doc = "Register `HTCR` reader"]
pub struct R(crate::R<HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTCR` writer"]
pub struct W(crate::W<HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTCR_SPEC>;
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
impl From<crate::W<HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFREF` reader - High Frequency Reference Threshold"]
pub struct HFREF_R(crate::FieldReader<u32, u32>);
impl HFREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HFREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFREF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFREF` writer - High Frequency Reference Threshold"]
pub struct HFREF_W<'a> {
    w: &'a mut W,
}
impl<'a> HFREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - High Frequency Reference Threshold"]
    #[inline(always)]
    pub fn hfref(&self) -> HFREF_R {
        HFREF_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - High Frequency Reference Threshold"]
    #[inline(always)]
    pub fn hfref(&mut self) -> HFREF_W {
        HFREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Frequency Check High Threshold Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcr](index.html) module"]
pub struct HTCR_SPEC;
impl crate::RegisterSpec for HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcr::R](R) reader structure"]
impl crate::Readable for HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htcr::W](W) writer structure"]
impl crate::Writable for HTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTCR to value 0x00ff_ffff"]
impl crate::Resettable for HTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ffff
    }
}
