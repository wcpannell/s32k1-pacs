#[doc = "Register `CINT` reader"]
pub struct R(crate::R<CINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CINT` writer"]
pub struct W(crate::W<CINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CINT_SPEC>;
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
impl From<crate::W<CINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CINT` reader - Clear Interrupt Request"]
pub struct CINT_R(crate::FieldReader<u8, u8>);
impl CINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CINT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CINT` writer - Clear Interrupt Request"]
pub struct CINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Clear All Interrupt Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAIR_A {
    #[doc = "0: Clear only the INT bit specified in the CINT field"]
    CAIR_0 = 0,
    #[doc = "1: Clear all bits in INT"]
    CAIR_1 = 1,
}
impl From<CAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CAIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAIR` reader - Clear All Interrupt Requests"]
pub struct CAIR_R(crate::FieldReader<bool, CAIR_A>);
impl CAIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAIR_A {
        match self.bits {
            false => CAIR_A::CAIR_0,
            true => CAIR_A::CAIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAIR_0`"]
    #[inline(always)]
    pub fn is_cair_0(&self) -> bool {
        **self == CAIR_A::CAIR_0
    }
    #[doc = "Checks if the value of the field is `CAIR_1`"]
    #[inline(always)]
    pub fn is_cair_1(&self) -> bool {
        **self == CAIR_A::CAIR_1
    }
}
impl core::ops::Deref for CAIR_R {
    type Target = crate::FieldReader<bool, CAIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAIR` writer - Clear All Interrupt Requests"]
pub struct CAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAIR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear only the INT bit specified in the CINT field"]
    #[inline(always)]
    pub fn cair_0(self) -> &'a mut W {
        self.variant(CAIR_A::CAIR_0)
    }
    #[doc = "Clear all bits in INT"]
    #[inline(always)]
    pub fn cair_1(self) -> &'a mut W {
        self.variant(CAIR_A::CAIR_1)
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
    #[doc = "Bits 0:3 - Clear Interrupt Request"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    pub fn cair(&self) -> CAIR_R {
        CAIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear Interrupt Request"]
    #[inline(always)]
    pub fn cint(&mut self) -> CINT_W {
        CINT_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Interrupt Requests"]
    #[inline(always)]
    pub fn cair(&mut self) -> CAIR_W {
        CAIR_W { w: self }
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
#[doc = "Clear Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cint](index.html) module"]
pub struct CINT_SPEC;
impl crate::RegisterSpec for CINT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cint::R](R) reader structure"]
impl crate::Readable for CINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cint::W](W) writer structure"]
impl crate::Writable for CINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CINT to value 0"]
impl crate::Resettable for CINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
