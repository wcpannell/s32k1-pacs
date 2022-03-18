#[doc = "Register `CEEI` reader"]
pub struct R(crate::R<CEEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEEI` writer"]
pub struct W(crate::W<CEEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEEI_SPEC>;
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
impl From<crate::W<CEEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEEI` reader - Clear Enable Error Interrupt"]
pub struct CEEI_R(crate::FieldReader<u8, u8>);
impl CEEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CEEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEEI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEEI` writer - Clear Enable Error Interrupt"]
pub struct CEEI_W<'a> {
    w: &'a mut W,
}
impl<'a> CEEI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Clear All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAEE_A {
    #[doc = "0: Clear only the EEI bit specified in the CEEI field"]
    CAEE_0 = 0,
    #[doc = "1: Clear all bits in EEI"]
    CAEE_1 = 1,
}
impl From<CAEE_A> for bool {
    #[inline(always)]
    fn from(variant: CAEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEE` reader - Clear All Enable Error Interrupts"]
pub struct CAEE_R(crate::FieldReader<bool, CAEE_A>);
impl CAEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAEE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAEE_A {
        match self.bits {
            false => CAEE_A::CAEE_0,
            true => CAEE_A::CAEE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAEE_0`"]
    #[inline(always)]
    pub fn is_caee_0(&self) -> bool {
        **self == CAEE_A::CAEE_0
    }
    #[doc = "Checks if the value of the field is `CAEE_1`"]
    #[inline(always)]
    pub fn is_caee_1(&self) -> bool {
        **self == CAEE_A::CAEE_1
    }
}
impl core::ops::Deref for CAEE_R {
    type Target = crate::FieldReader<bool, CAEE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAEE` writer - Clear All Enable Error Interrupts"]
pub struct CAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAEE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear only the EEI bit specified in the CEEI field"]
    #[inline(always)]
    pub fn caee_0(self) -> &'a mut W {
        self.variant(CAEE_A::CAEE_0)
    }
    #[doc = "Clear all bits in EEI"]
    #[inline(always)]
    pub fn caee_1(self) -> &'a mut W {
        self.variant(CAEE_A::CAEE_1)
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
    #[doc = "Bits 0:3 - Clear Enable Error Interrupt"]
    #[inline(always)]
    pub fn ceei(&self) -> CEEI_R {
        CEEI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub fn caee(&self) -> CAEE_R {
        CAEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear Enable Error Interrupt"]
    #[inline(always)]
    pub fn ceei(&mut self) -> CEEI_W {
        CEEI_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub fn caee(&mut self) -> CAEE_W {
        CAEE_W { w: self }
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
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](index.html) module"]
pub struct CEEI_SPEC;
impl crate::RegisterSpec for CEEI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ceei::R](R) reader structure"]
impl crate::Readable for CEEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ceei::W](W) writer structure"]
impl crate::Writable for CEEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEEI to value 0"]
impl crate::Resettable for CEEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
