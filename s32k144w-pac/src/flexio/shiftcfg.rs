#[doc = "Register `SHIFTCFG[%s]` reader"]
pub struct R(crate::R<SHIFTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCFG[%s]` writer"]
pub struct W(crate::W<SHIFTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCFG_SPEC>;
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
impl From<crate::W<SHIFTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSTART_A {
    #[doc = "0: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    SSTART_0 = 0,
    #[doc = "1: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    SSTART_1 = 1,
    #[doc = "2: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    SSTART_2 = 2,
    #[doc = "3: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    SSTART_3 = 3,
}
impl From<SSTART_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTART_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSTART` reader - Shifter Start bit"]
pub struct SSTART_R(crate::FieldReader<u8, SSTART_A>);
impl SSTART_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSTART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSTART_A {
        match self.bits {
            0 => SSTART_A::SSTART_0,
            1 => SSTART_A::SSTART_1,
            2 => SSTART_A::SSTART_2,
            3 => SSTART_A::SSTART_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSTART_0`"]
    #[inline(always)]
    pub fn is_sstart_0(&self) -> bool {
        **self == SSTART_A::SSTART_0
    }
    #[doc = "Checks if the value of the field is `SSTART_1`"]
    #[inline(always)]
    pub fn is_sstart_1(&self) -> bool {
        **self == SSTART_A::SSTART_1
    }
    #[doc = "Checks if the value of the field is `SSTART_2`"]
    #[inline(always)]
    pub fn is_sstart_2(&self) -> bool {
        **self == SSTART_A::SSTART_2
    }
    #[doc = "Checks if the value of the field is `SSTART_3`"]
    #[inline(always)]
    pub fn is_sstart_3(&self) -> bool {
        **self == SSTART_A::SSTART_3
    }
}
impl core::ops::Deref for SSTART_R {
    type Target = crate::FieldReader<u8, SSTART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTART` writer - Shifter Start bit"]
pub struct SSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTART_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline(always)]
    pub fn sstart_0(self) -> &'a mut W {
        self.variant(SSTART_A::SSTART_0)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn sstart_1(self) -> &'a mut W {
        self.variant(SSTART_A::SSTART_1)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn sstart_2(self) -> &'a mut W {
        self.variant(SSTART_A::SSTART_2)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn sstart_3(self) -> &'a mut W {
        self.variant(SSTART_A::SSTART_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Shifter Stop bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSTOP_A {
    #[doc = "0: Stop bit disabled for transmitter/receiver/match store"]
    SSTOP_0 = 0,
    #[doc = "2: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    SSTOP_2 = 2,
    #[doc = "3: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    SSTOP_3 = 3,
}
impl From<SSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSTOP` reader - Shifter Stop bit"]
pub struct SSTOP_R(crate::FieldReader<u8, SSTOP_A>);
impl SSTOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSTOP_A> {
        match self.bits {
            0 => Some(SSTOP_A::SSTOP_0),
            2 => Some(SSTOP_A::SSTOP_2),
            3 => Some(SSTOP_A::SSTOP_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSTOP_0`"]
    #[inline(always)]
    pub fn is_sstop_0(&self) -> bool {
        **self == SSTOP_A::SSTOP_0
    }
    #[doc = "Checks if the value of the field is `SSTOP_2`"]
    #[inline(always)]
    pub fn is_sstop_2(&self) -> bool {
        **self == SSTOP_A::SSTOP_2
    }
    #[doc = "Checks if the value of the field is `SSTOP_3`"]
    #[inline(always)]
    pub fn is_sstop_3(&self) -> bool {
        **self == SSTOP_A::SSTOP_3
    }
}
impl core::ops::Deref for SSTOP_R {
    type Target = crate::FieldReader<u8, SSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTOP` writer - Shifter Stop bit"]
pub struct SSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn sstop_0(self) -> &'a mut W {
        self.variant(SSTOP_A::SSTOP_0)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline(always)]
    pub fn sstop_2(self) -> &'a mut W {
        self.variant(SSTOP_A::SSTOP_2)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline(always)]
    pub fn sstop_3(self) -> &'a mut W {
        self.variant(SSTOP_A::SSTOP_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSRC_A {
    #[doc = "0: Pin"]
    INSRC_0 = 0,
    #[doc = "1: Shifter N+1 Output"]
    INSRC_1 = 1,
}
impl From<INSRC_A> for bool {
    #[inline(always)]
    fn from(variant: INSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSRC` reader - Input Source"]
pub struct INSRC_R(crate::FieldReader<bool, INSRC_A>);
impl INSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INSRC_A {
        match self.bits {
            false => INSRC_A::INSRC_0,
            true => INSRC_A::INSRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `INSRC_0`"]
    #[inline(always)]
    pub fn is_insrc_0(&self) -> bool {
        **self == INSRC_A::INSRC_0
    }
    #[doc = "Checks if the value of the field is `INSRC_1`"]
    #[inline(always)]
    pub fn is_insrc_1(&self) -> bool {
        **self == INSRC_A::INSRC_1
    }
}
impl core::ops::Deref for INSRC_R {
    type Target = crate::FieldReader<bool, INSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSRC` writer - Input Source"]
pub struct INSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> INSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin"]
    #[inline(always)]
    pub fn insrc_0(self) -> &'a mut W {
        self.variant(INSRC_A::INSRC_0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn insrc_1(self) -> &'a mut W {
        self.variant(INSRC_A::INSRC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&self) -> SSTART_R {
        SSTART_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&self) -> SSTOP_R {
        SSTOP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&self) -> INSRC_R {
        INSRC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&mut self) -> SSTART_W {
        SSTART_W { w: self }
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&mut self) -> SSTOP_W {
        SSTOP_W { w: self }
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&mut self) -> INSRC_W {
        INSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg](index.html) module"]
pub struct SHIFTCFG_SPEC;
impl crate::RegisterSpec for SHIFTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftcfg::R](R) reader structure"]
impl crate::Readable for SHIFTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftcfg::W](W) writer structure"]
impl crate::Writable for SHIFTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTCFG[%s]
to value 0"]
impl crate::Resettable for SHIFTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
