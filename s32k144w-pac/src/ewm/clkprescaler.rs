#[doc = "Register `CLKPRESCALER` reader"]
pub struct R(crate::R<CLKPRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `CLK_DIV` reader - CLK_DIV"]
pub struct CLK_DIV_R(crate::FieldReader<u8, u8>);
impl CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
impl R {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(self.bits)
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
#[doc = "Clock Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkprescaler](index.html) module"]
pub struct CLKPRESCALER_SPEC;
impl crate::RegisterSpec for CLKPRESCALER_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clkprescaler::R](R) reader structure"]
impl crate::Readable for CLKPRESCALER_SPEC {
    type Reader = R;
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
