#[doc = "Register `CDNE` reader"]
pub struct R(crate::R<CDNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDNE` writer"]
pub struct W(crate::W<CDNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDNE_SPEC>;
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
impl From<crate::W<CDNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDNE` reader - Clear DONE Bit"]
pub struct CDNE_R(crate::FieldReader<u8, u8>);
impl CDNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CDNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDNE` writer - Clear DONE Bit"]
pub struct CDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> CDNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Clears All DONE Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CADN_A {
    #[doc = "0: Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    CADN_0 = 0,
    #[doc = "1: Clears all bits in TCDn_CSR\\[DONE\\]"]
    CADN_1 = 1,
}
impl From<CADN_A> for bool {
    #[inline(always)]
    fn from(variant: CADN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CADN` reader - Clears All DONE Bits"]
pub struct CADN_R(crate::FieldReader<bool, CADN_A>);
impl CADN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CADN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CADN_A {
        match self.bits {
            false => CADN_A::CADN_0,
            true => CADN_A::CADN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CADN_0`"]
    #[inline(always)]
    pub fn is_cadn_0(&self) -> bool {
        **self == CADN_A::CADN_0
    }
    #[doc = "Checks if the value of the field is `CADN_1`"]
    #[inline(always)]
    pub fn is_cadn_1(&self) -> bool {
        **self == CADN_A::CADN_1
    }
}
impl core::ops::Deref for CADN_R {
    type Target = crate::FieldReader<bool, CADN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CADN` writer - Clears All DONE Bits"]
pub struct CADN_W<'a> {
    w: &'a mut W,
}
impl<'a> CADN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CADN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    #[inline(always)]
    pub fn cadn_0(self) -> &'a mut W {
        self.variant(CADN_A::CADN_0)
    }
    #[doc = "Clears all bits in TCDn_CSR\\[DONE\\]"]
    #[inline(always)]
    pub fn cadn_1(self) -> &'a mut W {
        self.variant(CADN_A::CADN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u8 & 0x01) << 6);
        self.w
    }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOP_A {
    #[doc = "0: Normal operation"]
    NOP_0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    NOP_1 = 1,
}
impl From<NOP_A> for bool {
    #[inline(always)]
    fn from(variant: NOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOP` reader - No Op enable"]
pub struct NOP_R(crate::FieldReader<bool, NOP_A>);
impl NOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOP_A {
        match self.bits {
            false => NOP_A::NOP_0,
            true => NOP_A::NOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOP_0`"]
    #[inline(always)]
    pub fn is_nop_0(&self) -> bool {
        **self == NOP_A::NOP_0
    }
    #[doc = "Checks if the value of the field is `NOP_1`"]
    #[inline(always)]
    pub fn is_nop_1(&self) -> bool {
        **self == NOP_A::NOP_1
    }
}
impl core::ops::Deref for NOP_R {
    type Target = crate::FieldReader<bool, NOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOP` writer - No Op enable"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn nop_0(self) -> &'a mut W {
        self.variant(NOP_A::NOP_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn nop_1(self) -> &'a mut W {
        self.variant(NOP_A::NOP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u8 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clear DONE Bit"]
    #[inline(always)]
    pub fn cdne(&self) -> CDNE_R {
        CDNE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    pub fn cadn(&self) -> CADN_R {
        CADN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear DONE Bit"]
    #[inline(always)]
    pub fn cdne(&mut self) -> CDNE_W {
        CDNE_W { w: self }
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    pub fn cadn(&mut self) -> CADN_W {
        CADN_W { w: self }
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&mut self) -> NOP_W {
        NOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](index.html) module"]
pub struct CDNE_SPEC;
impl crate::RegisterSpec for CDNE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cdne::R](R) reader structure"]
impl crate::Readable for CDNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdne::W](W) writer structure"]
impl crate::Writable for CDNE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDNE to value 0"]
impl crate::Resettable for CDNE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
