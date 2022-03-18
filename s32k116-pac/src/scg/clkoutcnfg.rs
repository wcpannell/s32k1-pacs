#[doc = "Register `CLKOUTCNFG` reader"]
pub struct R(crate::R<CLKOUTCNFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUTCNFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUTCNFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUTCNFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUTCNFG` writer"]
pub struct W(crate::W<CLKOUTCNFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUTCNFG_SPEC>;
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
impl From<crate::W<CLKOUTCNFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUTCNFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SCG Clkout Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: SCG SLOW Clock"]
    _0000 = 0,
    #[doc = "1: System OSC (SOSC_CLK)"]
    _0001 = 1,
    #[doc = "2: Slow IRC (SIRC_CLK)"]
    _0010 = 2,
    #[doc = "3: Fast IRC (FIRC_CLK)"]
    _0011 = 3,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKOUTSEL` reader - SCG Clkout Select"]
pub struct CLKOUTSEL_R(crate::FieldReader<u8, CLKOUTSEL_A>);
impl CLKOUTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKOUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            0 => Some(CLKOUTSEL_A::_0000),
            1 => Some(CLKOUTSEL_A::_0001),
            2 => Some(CLKOUTSEL_A::_0010),
            3 => Some(CLKOUTSEL_A::_0011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        **self == CLKOUTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        **self == CLKOUTSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        **self == CLKOUTSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        **self == CLKOUTSEL_A::_0011
    }
}
impl core::ops::Deref for CLKOUTSEL_R {
    type Target = crate::FieldReader<u8, CLKOUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOUTSEL` writer - SCG Clkout Select"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCG SLOW Clock"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0000)
    }
    #[doc = "System OSC (SOSC_CLK)"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0001)
    }
    #[doc = "Slow IRC (SIRC_CLK)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0010)
    }
    #[doc = "Fast IRC (FIRC_CLK)"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - SCG Clkout Select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCG CLKOUT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutcnfg](index.html) module"]
pub struct CLKOUTCNFG_SPEC;
impl crate::RegisterSpec for CLKOUTCNFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkoutcnfg::R](R) reader structure"]
impl crate::Readable for CLKOUTCNFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkoutcnfg::W](W) writer structure"]
impl crate::Writable for CLKOUTCNFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUTCNFG to value 0x0300_0000"]
impl crate::Resettable for CLKOUTCNFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300_0000
    }
}
