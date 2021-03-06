#[doc = "Register `SAI_TDR0` writer"]
pub struct W(crate::W<SAI_TDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_TDR0_SPEC>;
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
impl From<crate::W<SAI_TDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_TDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDR` writer - Transmit Data Register"]
pub struct TDR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TDR_W {
        TDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_tdr0](index.html) module"]
pub struct SAI_TDR0_SPEC;
impl crate::RegisterSpec for SAI_TDR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sai_tdr0::W](W) writer structure"]
impl crate::Writable for SAI_TDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_TDR0 to value 0"]
impl crate::Resettable for SAI_TDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
