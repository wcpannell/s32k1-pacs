#[doc = "Register `SSRT` reader"]
pub struct R(crate::R<SSRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRT` writer"]
pub struct W(crate::W<SSRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRT_SPEC>;
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
impl From<crate::W<SSRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSRT` reader - Set START Bit"]
pub struct SSRT_R(crate::FieldReader<u8, u8>);
impl SSRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSRT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSRT` writer - Set START Bit"]
pub struct SSRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
#[doc = "Set All START Bits (activates all channels)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAST_A {
    #[doc = "0: Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    SAST_0 = 0,
    #[doc = "1: Set all bits in TCDn_CSR\\[START\\]"]
    SAST_1 = 1,
}
impl From<SAST_A> for bool {
    #[inline(always)]
    fn from(variant: SAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAST` reader - Set All START Bits (activates all channels)"]
pub struct SAST_R(crate::FieldReader<bool, SAST_A>);
impl SAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAST_A {
        match self.bits {
            false => SAST_A::SAST_0,
            true => SAST_A::SAST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAST_0`"]
    #[inline(always)]
    pub fn is_sast_0(&self) -> bool {
        **self == SAST_A::SAST_0
    }
    #[doc = "Checks if the value of the field is `SAST_1`"]
    #[inline(always)]
    pub fn is_sast_1(&self) -> bool {
        **self == SAST_A::SAST_1
    }
}
impl core::ops::Deref for SAST_R {
    type Target = crate::FieldReader<bool, SAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAST` writer - Set All START Bits (activates all channels)"]
pub struct SAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    #[inline(always)]
    pub fn sast_0(self) -> &'a mut W {
        self.variant(SAST_A::SAST_0)
    }
    #[doc = "Set all bits in TCDn_CSR\\[START\\]"]
    #[inline(always)]
    pub fn sast_1(self) -> &'a mut W {
        self.variant(SAST_A::SAST_1)
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
    #[doc = "Bits 0:3 - Set START Bit"]
    #[inline(always)]
    pub fn ssrt(&self) -> SSRT_R {
        SSRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Set All START Bits (activates all channels)"]
    #[inline(always)]
    pub fn sast(&self) -> SAST_R {
        SAST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set START Bit"]
    #[inline(always)]
    pub fn ssrt(&mut self) -> SSRT_W {
        SSRT_W { w: self }
    }
    #[doc = "Bit 6 - Set All START Bits (activates all channels)"]
    #[inline(always)]
    pub fn sast(&mut self) -> SAST_W {
        SAST_W { w: self }
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
#[doc = "Set START Bit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](index.html) module"]
pub struct SSRT_SPEC;
impl crate::RegisterSpec for SSRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssrt::R](R) reader structure"]
impl crate::Readable for SSRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrt::W](W) writer structure"]
impl crate::Writable for SSRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSRT to value 0"]
impl crate::Resettable for SSRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
