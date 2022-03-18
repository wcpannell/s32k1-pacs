#[doc = "Register `CLKPRESCALER` writer"]
pub struct W(crate::W<CLKPRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPRESCALER_SPEC>;
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
impl From<crate::W<CLKPRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_DIV` writer - CLK_DIV"]
pub struct CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Prescaler Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkprescaler](index.html) module"]
pub struct CLKPRESCALER_SPEC;
impl crate::RegisterSpec for CLKPRESCALER_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [clkprescaler::W](W) writer structure"]
impl crate::Writable for CLKPRESCALER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKPRESCALER to value 0"]
impl crate::Resettable for CLKPRESCALER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
