#[doc = "Register `TBCT` reader"]
pub struct R(crate::R<TBCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBCT` writer"]
pub struct W(crate::W<TBCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBCT_SPEC>;
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
impl From<crate::W<TBCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMRK` reader - Determines the watermark for the TX Buffer"]
pub struct WMRK_R(crate::FieldReader<u8, u8>);
impl WMRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WMRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMRK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMRK` writer - Determines the watermark for the TX Buffer"]
pub struct WMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> WMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Determines the watermark for the TX Buffer"]
    #[inline(always)]
    pub fn wmrk(&self) -> WMRK_R {
        WMRK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Determines the watermark for the TX Buffer"]
    #[inline(always)]
    pub fn wmrk(&mut self) -> WMRK_W {
        WMRK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbct](index.html) module"]
pub struct TBCT_SPEC;
impl crate::RegisterSpec for TBCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbct::R](R) reader structure"]
impl crate::Readable for TBCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbct::W](W) writer structure"]
impl crate::Writable for TBCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBCT to value 0"]
impl crate::Resettable for TBCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
