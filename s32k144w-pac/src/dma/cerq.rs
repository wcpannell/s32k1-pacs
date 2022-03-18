#[doc = "Register `CERQ` reader"]
pub struct R(crate::R<CERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CERQ` writer"]
pub struct W(crate::W<CERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERQ_SPEC>;
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
impl From<crate::W<CERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERQ` reader - Clear Enable Request"]
pub struct CERQ_R(crate::FieldReader<u8, u8>);
impl CERQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CERQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CERQ` writer - Clear Enable Request"]
pub struct CERQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CERQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Clear All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAER_A {
    #[doc = "0: Clear only the ERQ bit specified in the CERQ field"]
    CAER_0 = 0,
    #[doc = "1: Clear all bits in ERQ"]
    CAER_1 = 1,
}
impl From<CAER_A> for bool {
    #[inline(always)]
    fn from(variant: CAER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAER` reader - Clear All Enable Requests"]
pub struct CAER_R(crate::FieldReader<bool, CAER_A>);
impl CAER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAER_A {
        match self.bits {
            false => CAER_A::CAER_0,
            true => CAER_A::CAER_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAER_0`"]
    #[inline(always)]
    pub fn is_caer_0(&self) -> bool {
        **self == CAER_A::CAER_0
    }
    #[doc = "Checks if the value of the field is `CAER_1`"]
    #[inline(always)]
    pub fn is_caer_1(&self) -> bool {
        **self == CAER_A::CAER_1
    }
}
impl core::ops::Deref for CAER_R {
    type Target = crate::FieldReader<bool, CAER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAER` writer - Clear All Enable Requests"]
pub struct CAER_W<'a> {
    w: &'a mut W,
}
impl<'a> CAER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear only the ERQ bit specified in the CERQ field"]
    #[inline(always)]
    pub fn caer_0(self) -> &'a mut W {
        self.variant(CAER_A::CAER_0)
    }
    #[doc = "Clear all bits in ERQ"]
    #[inline(always)]
    pub fn caer_1(self) -> &'a mut W {
        self.variant(CAER_A::CAER_1)
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
    #[doc = "Bits 0:3 - Clear Enable Request"]
    #[inline(always)]
    pub fn cerq(&self) -> CERQ_R {
        CERQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    pub fn caer(&self) -> CAER_R {
        CAER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear Enable Request"]
    #[inline(always)]
    pub fn cerq(&mut self) -> CERQ_W {
        CERQ_W { w: self }
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    pub fn caer(&mut self) -> CAER_W {
        CAER_W { w: self }
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
#[doc = "Clear Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](index.html) module"]
pub struct CERQ_SPEC;
impl crate::RegisterSpec for CERQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cerq::R](R) reader structure"]
impl crate::Readable for CERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cerq::W](W) writer structure"]
impl crate::Writable for CERQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CERQ to value 0"]
impl crate::Resettable for CERQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
