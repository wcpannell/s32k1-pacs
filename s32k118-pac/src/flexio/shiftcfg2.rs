#[doc = "Register `SHIFTCFG2` reader"]
pub struct R(crate::R<SHIFTCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCFG2` writer"]
pub struct W(crate::W<SHIFTCFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCFG2_SPEC>;
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
impl From<crate::W<SHIFTCFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSTART_A {
    #[doc = "0: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    _0 = 0,
    #[doc = "1: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    _1 = 1,
    #[doc = "2: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    _10 = 2,
    #[doc = "3: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    _11 = 3,
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
            0 => SSTART_A::_0,
            1 => SSTART_A::_1,
            2 => SSTART_A::_10,
            3 => SSTART_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSTART_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSTART_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SSTART_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == SSTART_A::_11
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
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTART_A::_0)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTART_A::_1)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTART_A::_10)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTART_A::_11)
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
    _0 = 0,
    #[doc = "1: Reserved for transmitter/receiver/match store"]
    _1 = 1,
    #[doc = "2: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    _10 = 2,
    #[doc = "3: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    _11 = 3,
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
    pub fn variant(&self) -> SSTOP_A {
        match self.bits {
            0 => SSTOP_A::_0,
            1 => SSTOP_A::_1,
            2 => SSTOP_A::_10,
            3 => SSTOP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SSTOP_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SSTOP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == SSTOP_A::_11
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
        self.bits(variant.into())
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTOP_A::_0)
    }
    #[doc = "Reserved for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTOP_A::_1)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTOP_A::_10)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTOP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSRC_A {
    #[doc = "0: Pin"]
    _0 = 0,
    #[doc = "1: Shifter N+1 Output"]
    _1 = 1,
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
            false => INSRC_A::_0,
            true => INSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INSRC_A::_1
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
    pub fn _0(self) -> &'a mut W {
        self.variant(INSRC_A::_0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INSRC_A::_1)
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
#[doc = "Shifter Configuration N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcfg2](index.html) module"]
pub struct SHIFTCFG2_SPEC;
impl crate::RegisterSpec for SHIFTCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftcfg2::R](R) reader structure"]
impl crate::Readable for SHIFTCFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftcfg2::W](W) writer structure"]
impl crate::Writable for SHIFTCFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTCFG2 to value 0"]
impl crate::Resettable for SHIFTCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
