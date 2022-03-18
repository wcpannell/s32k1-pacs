#[doc = "Register `SHIFTCTL[%s]` reader"]
pub struct R(crate::R<SHIFTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCTL[%s]` writer"]
pub struct W(crate::W<SHIFTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCTL_SPEC>;
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
impl From<crate::W<SHIFTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shifter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Disabled."]
    SMOD_0 = 0,
    #[doc = "1: Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    SMOD_1 = 1,
    #[doc = "2: Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    SMOD_2 = 2,
    #[doc = "4: Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    SMOD_4 = 4,
    #[doc = "5: Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    SMOD_5 = 5,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD` reader - Shifter Mode"]
pub struct SMOD_R(crate::FieldReader<u8, SMOD_A>);
impl SMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::SMOD_0),
            1 => Some(SMOD_A::SMOD_1),
            2 => Some(SMOD_A::SMOD_2),
            4 => Some(SMOD_A::SMOD_4),
            5 => Some(SMOD_A::SMOD_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SMOD_0`"]
    #[inline(always)]
    pub fn is_smod_0(&self) -> bool {
        **self == SMOD_A::SMOD_0
    }
    #[doc = "Checks if the value of the field is `SMOD_1`"]
    #[inline(always)]
    pub fn is_smod_1(&self) -> bool {
        **self == SMOD_A::SMOD_1
    }
    #[doc = "Checks if the value of the field is `SMOD_2`"]
    #[inline(always)]
    pub fn is_smod_2(&self) -> bool {
        **self == SMOD_A::SMOD_2
    }
    #[doc = "Checks if the value of the field is `SMOD_4`"]
    #[inline(always)]
    pub fn is_smod_4(&self) -> bool {
        **self == SMOD_A::SMOD_4
    }
    #[doc = "Checks if the value of the field is `SMOD_5`"]
    #[inline(always)]
    pub fn is_smod_5(&self) -> bool {
        **self == SMOD_A::SMOD_5
    }
}
impl core::ops::Deref for SMOD_R {
    type Target = crate::FieldReader<u8, SMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD` writer - Shifter Mode"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn smod_0(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_0)
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline(always)]
    pub fn smod_1(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_1)
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline(always)]
    pub fn smod_2(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_2)
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline(always)]
    pub fn smod_4(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_4)
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline(always)]
    pub fn smod_5(self) -> &'a mut W {
        self.variant(SMOD_A::SMOD_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Shifter Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINPOL_A {
    #[doc = "0: Pin is active high"]
    PINPOL_0 = 0,
    #[doc = "1: Pin is active low"]
    PINPOL_1 = 1,
}
impl From<PINPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PINPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINPOL` reader - Shifter Pin Polarity"]
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
            false => PINPOL_A::PINPOL_0,
            true => PINPOL_A::PINPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `PINPOL_0`"]
    #[inline(always)]
    pub fn is_pinpol_0(&self) -> bool {
        **self == PINPOL_A::PINPOL_0
    }
    #[doc = "Checks if the value of the field is `PINPOL_1`"]
    #[inline(always)]
    pub fn is_pinpol_1(&self) -> bool {
        **self == PINPOL_A::PINPOL_1
    }
}
impl core::ops::Deref for PINPOL_R {
    type Target = crate::FieldReader<bool, PINPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINPOL` writer - Shifter Pin Polarity"]
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
    pub fn pinpol_0(self) -> &'a mut W {
        self.variant(PINPOL_A::PINPOL_0)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn pinpol_1(self) -> &'a mut W {
        self.variant(PINPOL_A::PINPOL_1)
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
#[doc = "Field `PINSEL` reader - Shifter Pin Select"]
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
#[doc = "Field `PINSEL` writer - Shifter Pin Select"]
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
#[doc = "Shifter Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Shifter pin output disabled"]
    PINCFG_0 = 0,
    #[doc = "1: Shifter pin open drain or bidirectional output enable"]
    PINCFG_1 = 1,
    #[doc = "2: Shifter pin bidirectional output data"]
    PINCFG_2 = 2,
    #[doc = "3: Shifter pin output"]
    PINCFG_3 = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PINCFG` reader - Shifter Pin Configuration"]
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
            0 => PINCFG_A::PINCFG_0,
            1 => PINCFG_A::PINCFG_1,
            2 => PINCFG_A::PINCFG_2,
            3 => PINCFG_A::PINCFG_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PINCFG_0`"]
    #[inline(always)]
    pub fn is_pincfg_0(&self) -> bool {
        **self == PINCFG_A::PINCFG_0
    }
    #[doc = "Checks if the value of the field is `PINCFG_1`"]
    #[inline(always)]
    pub fn is_pincfg_1(&self) -> bool {
        **self == PINCFG_A::PINCFG_1
    }
    #[doc = "Checks if the value of the field is `PINCFG_2`"]
    #[inline(always)]
    pub fn is_pincfg_2(&self) -> bool {
        **self == PINCFG_A::PINCFG_2
    }
    #[doc = "Checks if the value of the field is `PINCFG_3`"]
    #[inline(always)]
    pub fn is_pincfg_3(&self) -> bool {
        **self == PINCFG_A::PINCFG_3
    }
}
impl core::ops::Deref for PINCFG_R {
    type Target = crate::FieldReader<u8, PINCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINCFG` writer - Shifter Pin Configuration"]
pub struct PINCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PINCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Shifter pin output disabled"]
    #[inline(always)]
    pub fn pincfg_0(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_0)
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn pincfg_1(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_1)
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline(always)]
    pub fn pincfg_2(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_2)
    }
    #[doc = "Shifter pin output"]
    #[inline(always)]
    pub fn pincfg_3(self) -> &'a mut W {
        self.variant(PINCFG_A::PINCFG_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Timer Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPOL_A {
    #[doc = "0: Shift on posedge of Shift clock"]
    TIMPOL_0 = 0,
    #[doc = "1: Shift on negedge of Shift clock"]
    TIMPOL_1 = 1,
}
impl From<TIMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPOL` reader - Timer Polarity"]
pub struct TIMPOL_R(crate::FieldReader<bool, TIMPOL_A>);
impl TIMPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPOL_A {
        match self.bits {
            false => TIMPOL_A::TIMPOL_0,
            true => TIMPOL_A::TIMPOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMPOL_0`"]
    #[inline(always)]
    pub fn is_timpol_0(&self) -> bool {
        **self == TIMPOL_A::TIMPOL_0
    }
    #[doc = "Checks if the value of the field is `TIMPOL_1`"]
    #[inline(always)]
    pub fn is_timpol_1(&self) -> bool {
        **self == TIMPOL_A::TIMPOL_1
    }
}
impl core::ops::Deref for TIMPOL_R {
    type Target = crate::FieldReader<bool, TIMPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPOL` writer - Timer Polarity"]
pub struct TIMPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shift on posedge of Shift clock"]
    #[inline(always)]
    pub fn timpol_0(self) -> &'a mut W {
        self.variant(TIMPOL_A::TIMPOL_0)
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline(always)]
    pub fn timpol_1(self) -> &'a mut W {
        self.variant(TIMPOL_A::TIMPOL_1)
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
#[doc = "Field `TIMSEL` reader - Timer Select"]
pub struct TIMSEL_R(crate::FieldReader<u8, u8>);
impl TIMSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMSEL` writer - Timer Select"]
pub struct TIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&self) -> TIMPOL_R {
        TIMPOL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&self) -> TIMSEL_R {
        TIMSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&mut self) -> PINPOL_W {
        PINPOL_W { w: self }
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&mut self) -> PINSEL_W {
        PINSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&mut self) -> PINCFG_W {
        PINCFG_W { w: self }
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&mut self) -> TIMPOL_W {
        TIMPOL_W { w: self }
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&mut self) -> TIMSEL_W {
        TIMSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl](index.html) module"]
pub struct SHIFTCTL_SPEC;
impl crate::RegisterSpec for SHIFTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftctl::R](R) reader structure"]
impl crate::Readable for SHIFTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftctl::W](W) writer structure"]
impl crate::Writable for SHIFTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTCTL[%s]
to value 0"]
impl crate::Resettable for SHIFTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
