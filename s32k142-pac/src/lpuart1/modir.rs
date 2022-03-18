#[doc = "Register `MODIR` reader"]
pub struct R(crate::R<MODIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODIR` writer"]
pub struct W(crate::W<MODIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODIR_SPEC>;
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
impl From<crate::W<MODIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmitter clear-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSE_A {
    #[doc = "0: CTS has no effect on the transmitter."]
    _0 = 0,
    #[doc = "1: Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    _1 = 1,
}
impl From<TXCTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCTSE` reader - Transmitter clear-to-send enable"]
pub struct TXCTSE_R(crate::FieldReader<bool, TXCTSE_A>);
impl TXCTSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCTSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSE_A {
        match self.bits {
            false => TXCTSE_A::_0,
            true => TXCTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXCTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXCTSE_A::_1
    }
}
impl core::ops::Deref for TXCTSE_R {
    type Target = crate::FieldReader<bool, TXCTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCTSE` writer - Transmitter clear-to-send enable"]
pub struct TXCTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CTS has no effect on the transmitter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSE_A::_0)
    }
    #[doc = "Enables clear-to-send operation. The transmitter checks the state of CTS each time it is ready to send a character. If CTS is asserted, the character is sent. If CTS is deasserted, the signal TXD remains in the mark state and transmission is delayed until CTS is asserted. Changes in CTS as a character is being sent do not affect its transmission."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSE_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Transmitter request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSE_A {
    #[doc = "0: The transmitter has no effect on RTS."]
    _0 = 0,
    #[doc = "1: When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
    _1 = 1,
}
impl From<TXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRTSE` reader - Transmitter request-to-send enable"]
pub struct TXRTSE_R(crate::FieldReader<bool, TXRTSE_A>);
impl TXRTSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRTSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSE_A {
        match self.bits {
            false => TXRTSE_A::_0,
            true => TXRTSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRTSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRTSE_A::_1
    }
}
impl core::ops::Deref for TXRTSE_R {
    type Target = crate::FieldReader<bool, TXRTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRTSE` writer - Transmitter request-to-send enable"]
pub struct TXRTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The transmitter has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSE_A::_0)
    }
    #[doc = "When a character is placed into an empty transmitter data buffer , RTS asserts one bit time before the start bit is transmitted. RTS deasserts one bit time after all characters in the transmitter data buffer and shift register are completely sent, including the last stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Transmitter request-to-send polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRTSPOL_A {
    #[doc = "0: Transmitter RTS is active low."]
    _0 = 0,
    #[doc = "1: Transmitter RTS is active high."]
    _1 = 1,
}
impl From<TXRTSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXRTSPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRTSPOL` reader - Transmitter request-to-send polarity"]
pub struct TXRTSPOL_R(crate::FieldReader<bool, TXRTSPOL_A>);
impl TXRTSPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRTSPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRTSPOL_A {
        match self.bits {
            false => TXRTSPOL_A::_0,
            true => TXRTSPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXRTSPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRTSPOL_A::_1
    }
}
impl core::ops::Deref for TXRTSPOL_R {
    type Target = crate::FieldReader<bool, TXRTSPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRTSPOL` writer - Transmitter request-to-send polarity"]
pub struct TXRTSPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRTSPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXRTSPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter RTS is active low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_0)
    }
    #[doc = "Transmitter RTS is active high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRTSPOL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Receiver request-to-send enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRTSE_A {
    #[doc = "0: The receiver has no effect on RTS."]
    _0 = 0,
}
impl From<RXRTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RXRTSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRTSE` reader - Receiver request-to-send enable"]
pub struct RXRTSE_R(crate::FieldReader<bool, RXRTSE_A>);
impl RXRTSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRTSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXRTSE_A> {
        match self.bits {
            false => Some(RXRTSE_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXRTSE_A::_0
    }
}
impl core::ops::Deref for RXRTSE_R {
    type Target = crate::FieldReader<bool, RXRTSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRTSE` writer - Receiver request-to-send enable"]
pub struct RXRTSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRTSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXRTSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The receiver has no effect on RTS."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXRTSE_A::_0)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Transmit CTS Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSC_A {
    #[doc = "0: CTS input is sampled at the start of each character."]
    _0 = 0,
    #[doc = "1: CTS input is sampled when the transmitter is idle."]
    _1 = 1,
}
impl From<TXCTSC_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCTSC` reader - Transmit CTS Configuration"]
pub struct TXCTSC_R(crate::FieldReader<bool, TXCTSC_A>);
impl TXCTSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCTSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSC_A {
        match self.bits {
            false => TXCTSC_A::_0,
            true => TXCTSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXCTSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXCTSC_A::_1
    }
}
impl core::ops::Deref for TXCTSC_R {
    type Target = crate::FieldReader<bool, TXCTSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCTSC` writer - Transmit CTS Configuration"]
pub struct TXCTSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCTSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCTSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CTS input is sampled at the start of each character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSC_A::_0)
    }
    #[doc = "CTS input is sampled when the transmitter is idle."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Transmit CTS Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCTSSRC_A {
    #[doc = "0: CTS input is the CTS_B pin."]
    _0 = 0,
    #[doc = "1: CTS input is the inverted Receiver Match result."]
    _1 = 1,
}
impl From<TXCTSSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TXCTSSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCTSSRC` reader - Transmit CTS Source"]
pub struct TXCTSSRC_R(crate::FieldReader<bool, TXCTSSRC_A>);
impl TXCTSSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCTSSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCTSSRC_A {
        match self.bits {
            false => TXCTSSRC_A::_0,
            true => TXCTSSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXCTSSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXCTSSRC_A::_1
    }
}
impl core::ops::Deref for TXCTSSRC_R {
    type Target = crate::FieldReader<bool, TXCTSSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCTSSRC` writer - Transmit CTS Source"]
pub struct TXCTSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCTSSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCTSSRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CTS input is the CTS_B pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCTSSRC_A::_0)
    }
    #[doc = "CTS input is the inverted Receiver Match result."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCTSSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RTSWATER` reader - Receive RTS Configuration"]
pub struct RTSWATER_R(crate::FieldReader<u8, u8>);
impl RTSWATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTSWATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTSWATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTSWATER` writer - Receive RTS Configuration"]
pub struct RTSWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Transmitter narrow pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TNP_A {
    #[doc = "0: 1/OSR."]
    _00 = 0,
    #[doc = "1: 2/OSR."]
    _01 = 1,
    #[doc = "2: 3/OSR."]
    _10 = 2,
    #[doc = "3: 4/OSR."]
    _11 = 3,
}
impl From<TNP_A> for u8 {
    #[inline(always)]
    fn from(variant: TNP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TNP` reader - Transmitter narrow pulse"]
pub struct TNP_R(crate::FieldReader<u8, TNP_A>);
impl TNP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TNP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNP_A {
        match self.bits {
            0 => TNP_A::_00,
            1 => TNP_A::_01,
            2 => TNP_A::_10,
            3 => TNP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TNP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TNP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TNP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TNP_A::_11
    }
}
impl core::ops::Deref for TNP_R {
    type Target = crate::FieldReader<u8, TNP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TNP` writer - Transmitter narrow pulse"]
pub struct TNP_W<'a> {
    w: &'a mut W,
}
impl<'a> TNP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TNP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1/OSR."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TNP_A::_00)
    }
    #[doc = "2/OSR."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TNP_A::_01)
    }
    #[doc = "3/OSR."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TNP_A::_10)
    }
    #[doc = "4/OSR."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TNP_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IREN_A {
    #[doc = "0: IR disabled."]
    _0 = 0,
    #[doc = "1: IR enabled."]
    _1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub struct IREN_R(crate::FieldReader<bool, IREN_A>);
impl IREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::_0,
            true => IREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IREN_A::_1
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, IREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREN_A::_0)
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&self) -> TXCTSE_R {
        TXCTSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&self) -> TXRTSE_R {
        TXRTSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&self) -> TXRTSPOL_R {
        TXRTSPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&self) -> RXRTSE_R {
        RXRTSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline(always)]
    pub fn txctsc(&self) -> TXCTSC_R {
        TXCTSC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline(always)]
    pub fn txctssrc(&self) -> TXCTSSRC_R {
        TXCTSSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline(always)]
    pub fn rtswater(&self) -> RTSWATER_R {
        RTSWATER_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&self) -> TNP_R {
        TNP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter clear-to-send enable"]
    #[inline(always)]
    pub fn txctse(&mut self) -> TXCTSE_W {
        TXCTSE_W { w: self }
    }
    #[doc = "Bit 1 - Transmitter request-to-send enable"]
    #[inline(always)]
    pub fn txrtse(&mut self) -> TXRTSE_W {
        TXRTSE_W { w: self }
    }
    #[doc = "Bit 2 - Transmitter request-to-send polarity"]
    #[inline(always)]
    pub fn txrtspol(&mut self) -> TXRTSPOL_W {
        TXRTSPOL_W { w: self }
    }
    #[doc = "Bit 3 - Receiver request-to-send enable"]
    #[inline(always)]
    pub fn rxrtse(&mut self) -> RXRTSE_W {
        RXRTSE_W { w: self }
    }
    #[doc = "Bit 4 - Transmit CTS Configuration"]
    #[inline(always)]
    pub fn txctsc(&mut self) -> TXCTSC_W {
        TXCTSC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit CTS Source"]
    #[inline(always)]
    pub fn txctssrc(&mut self) -> TXCTSSRC_W {
        TXCTSSRC_W { w: self }
    }
    #[doc = "Bits 8:9 - Receive RTS Configuration"]
    #[inline(always)]
    pub fn rtswater(&mut self) -> RTSWATER_W {
        RTSWATER_W { w: self }
    }
    #[doc = "Bits 16:17 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&mut self) -> TNP_W {
        TNP_W { w: self }
    }
    #[doc = "Bit 18 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Modem IrDA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modir](index.html) module"]
pub struct MODIR_SPEC;
impl crate::RegisterSpec for MODIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modir::R](R) reader structure"]
impl crate::Readable for MODIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modir::W](W) writer structure"]
impl crate::Writable for MODIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODIR to value 0"]
impl crate::Resettable for MODIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
