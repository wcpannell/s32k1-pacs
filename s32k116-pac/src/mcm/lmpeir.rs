#[doc = "Register `LMPEIR` reader"]
pub struct R(crate::R<LMPEIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMPEIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMPEIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMPEIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LMPEIR` writer"]
pub struct W(crate::W<LMPEIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LMPEIR_SPEC>;
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
impl From<crate::W<LMPEIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LMPEIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENC` reader - ENCn = ECC Noncorrectable Error n"]
pub struct ENC_R(crate::FieldReader<u8, u8>);
impl ENC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ENC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENC` writer - ENCn = ECC Noncorrectable Error n"]
pub struct ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `E1B` reader - E1Bn = ECC 1-bit Error n"]
pub struct E1B_R(crate::FieldReader<u8, u8>);
impl E1B_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        E1B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E1B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E1B` writer - E1Bn = ECC 1-bit Error n"]
pub struct E1B_W<'a> {
    w: &'a mut W,
}
impl<'a> E1B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Parity or ECC Error Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEELOC_A {
    #[doc = "0: Non-correctable ECC event from SRAM_L"]
    _00 = 0,
    #[doc = "1: Non-correctable ECC event from SRAM_U"]
    _01 = 1,
}
impl From<PEELOC_A> for u8 {
    #[inline(always)]
    fn from(variant: PEELOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PEELOC` reader - Parity or ECC Error Location"]
pub struct PEELOC_R(crate::FieldReader<u8, PEELOC_A>);
impl PEELOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PEELOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PEELOC_A> {
        match self.bits {
            0 => Some(PEELOC_A::_00),
            1 => Some(PEELOC_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == PEELOC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == PEELOC_A::_01
    }
}
impl core::ops::Deref for PEELOC_R {
    type Target = crate::FieldReader<u8, PEELOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V` reader - Valid Bit"]
pub struct V_R(crate::FieldReader<bool, bool>);
impl V_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - ENCn = ECC Noncorrectable Error n"]
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - E1Bn = ECC 1-bit Error n"]
    #[inline(always)]
    pub fn e1b(&self) -> E1B_R {
        E1B_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Parity or ECC Error Location"]
    #[inline(always)]
    pub fn peeloc(&self) -> PEELOC_R {
        PEELOC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Valid Bit"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ENCn = ECC Noncorrectable Error n"]
    #[inline(always)]
    pub fn enc(&mut self) -> ENC_W {
        ENC_W { w: self }
    }
    #[doc = "Bits 8:15 - E1Bn = ECC 1-bit Error n"]
    #[inline(always)]
    pub fn e1b(&mut self) -> E1B_W {
        E1B_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LMEM Parity and ECC Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmpeir](index.html) module"]
pub struct LMPEIR_SPEC;
impl crate::RegisterSpec for LMPEIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmpeir::R](R) reader structure"]
impl crate::Readable for LMPEIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lmpeir::W](W) writer structure"]
impl crate::Writable for LMPEIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LMPEIR to value 0"]
impl crate::Resettable for LMPEIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
