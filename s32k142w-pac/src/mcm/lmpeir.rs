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
#[doc = "Field `PE` reader - Cache Parity Error"]
pub struct PE_R(crate::FieldReader<u8, u8>);
impl PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE` writer - Cache Parity Error"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Parity or ECC Error Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEELOC_A {
    #[doc = "0: Non-correctable ECC event from SRAM_L"]
    PEELOC_0 = 0,
    #[doc = "1: Non-correctable ECC event from SRAM_U"]
    PEELOC_1 = 1,
    #[doc = "8: 1-bit correctable ECC event from SRAM_L"]
    PEELOC_8 = 8,
    #[doc = "9: 1-bit correctable ECC event from SRAM_U"]
    PEELOC_9 = 9,
    #[doc = "14: PC tag parity error"]
    PEELOC_14 = 14,
    #[doc = "15: PC data parity error"]
    PEELOC_15 = 15,
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
            0 => Some(PEELOC_A::PEELOC_0),
            1 => Some(PEELOC_A::PEELOC_1),
            8 => Some(PEELOC_A::PEELOC_8),
            9 => Some(PEELOC_A::PEELOC_9),
            14 => Some(PEELOC_A::PEELOC_14),
            15 => Some(PEELOC_A::PEELOC_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PEELOC_0`"]
    #[inline(always)]
    pub fn is_peeloc_0(&self) -> bool {
        **self == PEELOC_A::PEELOC_0
    }
    #[doc = "Checks if the value of the field is `PEELOC_1`"]
    #[inline(always)]
    pub fn is_peeloc_1(&self) -> bool {
        **self == PEELOC_A::PEELOC_1
    }
    #[doc = "Checks if the value of the field is `PEELOC_8`"]
    #[inline(always)]
    pub fn is_peeloc_8(&self) -> bool {
        **self == PEELOC_A::PEELOC_8
    }
    #[doc = "Checks if the value of the field is `PEELOC_9`"]
    #[inline(always)]
    pub fn is_peeloc_9(&self) -> bool {
        **self == PEELOC_A::PEELOC_9
    }
    #[doc = "Checks if the value of the field is `PEELOC_14`"]
    #[inline(always)]
    pub fn is_peeloc_14(&self) -> bool {
        **self == PEELOC_A::PEELOC_14
    }
    #[doc = "Checks if the value of the field is `PEELOC_15`"]
    #[inline(always)]
    pub fn is_peeloc_15(&self) -> bool {
        **self == PEELOC_A::PEELOC_15
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
    #[doc = "Bits 16:23 - Cache Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 16) & 0xff) as u8)
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
    #[doc = "Bits 16:23 - Cache Parity Error"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
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
