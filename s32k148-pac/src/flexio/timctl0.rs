#[doc = "Register `TIMCTL0` reader"]
pub struct R(crate::R<TIMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCTL0` writer"]
pub struct W(crate::W<TIMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCTL0_SPEC>;
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
impl From<crate::W<TIMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMOD_A {
    #[doc = "0: Timer Disabled."]
    _0 = 0,
    #[doc = "1: Dual 8-bit counters baud/bit mode."]
    _1 = 1,
    #[doc = "2: Dual 8-bit counters PWM mode."]
    _10 = 2,
    #[doc = "3: Single 16-bit counter mode."]
    _11 = 3,
}
impl From<TIMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMOD` reader - Timer Mode"]
pub struct TIMOD_R(crate::FieldReader<u8, TIMOD_A>);
impl TIMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOD_A {
        match self.bits {
            0 => TIMOD_A::_0,
            1 => TIMOD_A::_1,
            2 => TIMOD_A::_10,
            3 => TIMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TIMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMOD_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TIMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TIMOD_A::_11
    }
}
impl core::ops::Deref for TIMOD_R {
    type Target = crate::FieldReader<u8, TIMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMOD` writer - Timer Mode"]
pub struct TIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMOD_A::_0)
    }
    #[doc = "Dual 8-bit counters baud/bit mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMOD_A::_1)
    }
    #[doc = "Dual 8-bit counters PWM mode."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMOD_A::_10)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMOD_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOL_A {
    #[doc = "0: Pin is active high"]
    _0 = 0,
    #[doc = "1: Pin is active low"]
    _1 = 1,
}
impl From<PINPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PINPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINPOL` reader - Timer Pin Polarity"]
pub struct PINPOL_R(crate::FieldReader<bool, PINPOL_A>);
impl PINPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PINPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPOL_A {
        match self.bits {
            false => PINPOL_A::_0,
            true => PINPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINPOL_A::_1
    }
}
impl core::ops::Deref for PINPOL_R {
    type Target = crate::FieldReader<bool, PINPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINPOL` writer - Timer Pin Polarity"]
pub struct PINPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINPOL_A::_0)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PINSEL` reader - Timer Pin Select"]
pub struct PINSEL_R(crate::FieldReader<u8, u8>);
impl PINSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PINSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINSEL` writer - Timer Pin Select"]
pub struct PINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PINSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Timer Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Timer pin output disabled"]
    _0 = 0,
    #[doc = "1: Timer pin open drain or bidirectional output enable"]
    _1 = 1,
    #[doc = "2: Timer pin bidirectional output data"]
    _10 = 2,
    #[doc = "3: Timer pin output"]
    _11 = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINCFG` reader - Timer Pin Configuration"]
pub struct PINCFG_R(crate::FieldReader<u8, PINCFG_A>);
impl PINCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PINCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::_0,
            1 => PINCFG_A::_1,
            2 => PINCFG_A::_10,
            3 => PINCFG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PINCFG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PINCFG_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == PINCFG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == PINCFG_A::_11
    }
}
impl core::ops::Deref for PINCFG_R {
    type Target = crate::FieldReader<u8, PINCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINCFG` writer - Timer Pin Configuration"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Timer pin output disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINCFG_A::_0)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINCFG_A::_1)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PINCFG_A::_10)
    }
    #[doc = "Timer pin output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PINCFG_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSRC_A {
    #[doc = "0: External trigger selected"]
    _0 = 0,
    #[doc = "1: Internal trigger selected"]
    _1 = 1,
}
impl From<TRGSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub struct TRGSRC_R(crate::FieldReader<bool, TRGSRC_A>);
impl TRGSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRGSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSRC_A {
        match self.bits {
            false => TRGSRC_A::_0,
            true => TRGSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGSRC_A::_1
    }
}
impl core::ops::Deref for TRGSRC_R {
    type Target = crate::FieldReader<bool, TRGSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub struct TRGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger selected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGSRC_A::_0)
    }
    #[doc = "Internal trigger selected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGPOL_A {
    #[doc = "0: Trigger active high"]
    _0 = 0,
    #[doc = "1: Trigger active low"]
    _1 = 1,
}
impl From<TRGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPOL` reader - Trigger Polarity"]
pub struct TRGPOL_R(crate::FieldReader<bool, TRGPOL_A>);
impl TRGPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRGPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPOL_A {
        match self.bits {
            false => TRGPOL_A::_0,
            true => TRGPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRGPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRGPOL_A::_1
    }
}
impl core::ops::Deref for TRGPOL_R {
    type Target = crate::FieldReader<bool, TRGPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGPOL` writer - Trigger Polarity"]
pub struct TRGPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger active high"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRGPOL_A::_0)
    }
    #[doc = "Trigger active low"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRGPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub struct TRGSEL_R(crate::FieldReader<u8, u8>);
impl TRGSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&self) -> TIMOD_R {
        TIMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TRGPOL_R {
        TRGPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&mut self) -> TIMOD_W {
        TIMOD_W { w: self }
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&mut self) -> PINPOL_W {
        PINPOL_W { w: self }
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W {
        TRGSRC_W { w: self }
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&mut self) -> TRGPOL_W {
        TRGPOL_W { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl0](index.html) module"]
pub struct TIMCTL0_SPEC;
impl crate::RegisterSpec for TIMCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timctl0::R](R) reader structure"]
impl crate::Readable for TIMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timctl0::W](W) writer structure"]
impl crate::Writable for TIMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCTL0 to value 0"]
impl crate::Resettable for TIMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
