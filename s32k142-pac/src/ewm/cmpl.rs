#[doc = "Register `CMPL` writer"]
pub struct W(crate::W<CMPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPL_SPEC>;
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
impl From<crate::W<CMPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPAREL` writer - COMPAREL"]
pub struct COMPAREL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPAREL"]
    #[inline(always)]
    pub fn comparel(&mut self) -> COMPAREL_W {
        COMPAREL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Low Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpl](index.html) module"]
pub struct CMPL_SPEC;
impl crate::RegisterSpec for CMPL_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [cmpl::W](W) writer structure"]
impl crate::Writable for CMPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPL to value 0"]
impl crate::Resettable for CMPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
