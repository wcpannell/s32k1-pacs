#[doc = "Register `SEEI` reader"]
pub struct R(crate::R<SEEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEEI` writer"]
pub struct W(crate::W<SEEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEEI_SPEC>;
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
impl From<crate::W<SEEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEEI` reader - Set Enable Error Interrupt"]
pub struct SEEI_R(crate::FieldReader<u8, u8>);
impl SEEI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEEI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEEI` writer - Set Enable Error Interrupt"]
pub struct SEEI_W<'a> {
    w: &'a mut W,
}
impl<'a> SEEI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Sets All Enable Error Interrupts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAEE_A {
    #[doc = "0: Set only the EEI bit specified in the SEEI field."]
    SAEE_0 = 0,
    #[doc = "1: Sets all bits in EEI"]
    SAEE_1 = 1,
}
impl From<SAEE_A> for bool {
    #[inline(always)]
    fn from(variant: SAEE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAEE` reader - Sets All Enable Error Interrupts"]
pub struct SAEE_R(crate::FieldReader<bool, SAEE_A>);
impl SAEE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAEE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEE_A {
        match self.bits {
            false => SAEE_A::SAEE_0,
            true => SAEE_A::SAEE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAEE_0`"]
    #[inline(always)]
    pub fn is_saee_0(&self) -> bool {
        **self == SAEE_A::SAEE_0
    }
    #[doc = "Checks if the value of the field is `SAEE_1`"]
    #[inline(always)]
    pub fn is_saee_1(&self) -> bool {
        **self == SAEE_A::SAEE_1
    }
}
impl core::ops::Deref for SAEE_R {
    type Target = crate::FieldReader<bool, SAEE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAEE` writer - Sets All Enable Error Interrupts"]
pub struct SAEE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAEE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAEE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set only the EEI bit specified in the SEEI field."]
    #[inline(always)]
    pub fn saee_0(self) -> &'a mut W {
        self.variant(SAEE_A::SAEE_0)
    }
    #[doc = "Sets all bits in EEI"]
    #[inline(always)]
    pub fn saee_1(self) -> &'a mut W {
        self.variant(SAEE_A::SAEE_1)
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
    #[doc = "Bits 0:3 - Set Enable Error Interrupt"]
    #[inline(always)]
    pub fn seei(&self) -> SEEI_R {
        SEEI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Sets All Enable Error Interrupts"]
    #[inline(always)]
    pub fn saee(&self) -> SAEE_R {
        SAEE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set Enable Error Interrupt"]
    #[inline(always)]
    pub fn seei(&mut self) -> SEEI_W {
        SEEI_W { w: self }
    }
    #[doc = "Bit 6 - Sets All Enable Error Interrupts"]
    #[inline(always)]
    pub fn saee(&mut self) -> SAEE_W {
        SAEE_W { w: self }
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
#[doc = "Set Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seei](index.html) module"]
pub struct SEEI_SPEC;
impl crate::RegisterSpec for SEEI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [seei::R](R) reader structure"]
impl crate::Readable for SEEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seei::W](W) writer structure"]
impl crate::Writable for SEEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEEI to value 0"]
impl crate::Resettable for SEEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
