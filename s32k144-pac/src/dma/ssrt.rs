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
pub enum SAST_AW {
    #[doc = "0: Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    _0 = 0,
    #[doc = "1: Set all bits in TCDn_CSR\\[START\\]"]
    _1 = 1,
}
impl From<SAST_AW> for bool {
    #[inline(always)]
    fn from(variant: SAST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAST` writer - Set All START Bits (activates all channels)"]
pub struct SAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Set only the TCDn_CSR\\[START\\]
bit specified in the SSRT field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAST_AW::_0)
    }
    #[doc = "Set all bits in TCDn_CSR\\[START\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAST_AW::_1)
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
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOP` writer - No Op enable"]
pub struct NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
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
#[doc = "Set START Bit Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](index.html) module"]
pub struct SSRT_SPEC;
impl crate::RegisterSpec for SSRT_SPEC {
    type Ux = u8;
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
