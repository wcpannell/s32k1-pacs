#[doc = "Register `SIRCCFG` reader"]
pub struct R(crate::R<SIRCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIRCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIRCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIRCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIRCCFG` writer"]
pub struct W(crate::W<SIRCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIRCCFG_SPEC>;
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
impl From<crate::W<SIRCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIRCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frequency Range\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGE_A {
    #[doc = "0: Slow IRC low range clock (2 MHz)"]
    _0 = 0,
    #[doc = "1: Slow IRC high range clock (8 MHz )"]
    _1 = 1,
}
impl From<RANGE_A> for bool {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RANGE` reader - Frequency Range"]
pub struct RANGE_R(crate::FieldReader<bool, RANGE_A>);
impl RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGE_A {
        match self.bits {
            false => RANGE_A::_0,
            true => RANGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RANGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RANGE_A::_1
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<bool, RANGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - Frequency Range"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slow IRC low range clock (2 MHz)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RANGE_A::_0)
    }
    #[doc = "Slow IRC high range clock (8 MHz )"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RANGE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Frequency Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frequency Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow IRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sirccfg](index.html) module"]
pub struct SIRCCFG_SPEC;
impl crate::RegisterSpec for SIRCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sirccfg::R](R) reader structure"]
impl crate::Readable for SIRCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sirccfg::W](W) writer structure"]
impl crate::Writable for SIRCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIRCCFG to value 0x01"]
impl crate::Resettable for SIRCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
