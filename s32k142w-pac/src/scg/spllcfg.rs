#[doc = "Register `SPLLCFG` reader"]
pub struct R(crate::R<SPLLCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLLCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLLCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLLCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPLLCFG` writer"]
pub struct W(crate::W<SPLLCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLLCFG_SPEC>;
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
impl From<crate::W<SPLLCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLLCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOURCE_A {
    #[doc = "0: System OSC (SOSC)"]
    SOURCE_0 = 0,
    #[doc = "1: Fast IRC (FIRC)"]
    SOURCE_1 = 1,
}
impl From<SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOURCE` reader - Clock Source"]
pub struct SOURCE_R(crate::FieldReader<bool, SOURCE_A>);
impl SOURCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOURCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOURCE_A {
        match self.bits {
            false => SOURCE_A::SOURCE_0,
            true => SOURCE_A::SOURCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOURCE_0`"]
    #[inline(always)]
    pub fn is_source_0(&self) -> bool {
        **self == SOURCE_A::SOURCE_0
    }
    #[doc = "Checks if the value of the field is `SOURCE_1`"]
    #[inline(always)]
    pub fn is_source_1(&self) -> bool {
        **self == SOURCE_A::SOURCE_1
    }
}
impl core::ops::Deref for SOURCE_R {
    type Target = crate::FieldReader<bool, SOURCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOURCE` writer - Clock Source"]
pub struct SOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System OSC (SOSC)"]
    #[inline(always)]
    pub fn source_0(self) -> &'a mut W {
        self.variant(SOURCE_A::SOURCE_0)
    }
    #[doc = "Fast IRC (FIRC)"]
    #[inline(always)]
    pub fn source_1(self) -> &'a mut W {
        self.variant(SOURCE_A::SOURCE_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PREDIV` reader - PLL Reference Clock Divider"]
pub struct PREDIV_R(crate::FieldReader<u8, u8>);
impl PREDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PREDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREDIV` writer - PLL Reference Clock Divider"]
pub struct PREDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PREDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `MULT` reader - System PLL Multiplier"]
pub struct MULT_R(crate::FieldReader<u8, u8>);
impl MULT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MULT` writer - System PLL Multiplier"]
pub struct MULT_W<'a> {
    w: &'a mut W,
}
impl<'a> MULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - PLL Reference Clock Divider"]
    #[inline(always)]
    pub fn prediv(&self) -> PREDIV_R {
        PREDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - System PLL Multiplier"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn source(&mut self) -> SOURCE_W {
        SOURCE_W { w: self }
    }
    #[doc = "Bits 8:10 - PLL Reference Clock Divider"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PREDIV_W {
        PREDIV_W { w: self }
    }
    #[doc = "Bits 16:20 - System PLL Multiplier"]
    #[inline(always)]
    pub fn mult(&mut self) -> MULT_W {
        MULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllcfg](index.html) module"]
pub struct SPLLCFG_SPEC;
impl crate::RegisterSpec for SPLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spllcfg::R](R) reader structure"]
impl crate::Readable for SPLLCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spllcfg::W](W) writer structure"]
impl crate::Writable for SPLLCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPLLCFG to value 0"]
impl crate::Resettable for SPLLCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
