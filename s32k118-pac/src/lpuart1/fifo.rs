#[doc = "Register `FIFO` reader"]
pub struct R(crate::R<FIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO` writer"]
pub struct W(crate::W<FIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_SPEC>;
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
impl From<crate::W<FIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive FIFO. Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIFOSIZE_A {
    #[doc = "0: Receive FIFO/Buffer depth = 1 dataword."]
    _000 = 0,
    #[doc = "1: Receive FIFO/Buffer depth = 4 datawords."]
    _001 = 1,
    #[doc = "2: Receive FIFO/Buffer depth = 8 datawords."]
    _010 = 2,
    #[doc = "3: Receive FIFO/Buffer depth = 16 datawords."]
    _011 = 3,
    #[doc = "4: Receive FIFO/Buffer depth = 32 datawords."]
    _100 = 4,
    #[doc = "5: Receive FIFO/Buffer depth = 64 datawords."]
    _101 = 5,
    #[doc = "6: Receive FIFO/Buffer depth = 128 datawords."]
    _110 = 6,
    #[doc = "7: Receive FIFO/Buffer depth = 256 datawords."]
    _111 = 7,
}
impl From<RXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXFIFOSIZE` reader - Receive FIFO. Buffer Depth"]
pub struct RXFIFOSIZE_R(crate::FieldReader<u8, RXFIFOSIZE_A>);
impl RXFIFOSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFOSIZE_A {
        match self.bits {
            0 => RXFIFOSIZE_A::_000,
            1 => RXFIFOSIZE_A::_001,
            2 => RXFIFOSIZE_A::_010,
            3 => RXFIFOSIZE_A::_011,
            4 => RXFIFOSIZE_A::_100,
            5 => RXFIFOSIZE_A::_101,
            6 => RXFIFOSIZE_A::_110,
            7 => RXFIFOSIZE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == RXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == RXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == RXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == RXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == RXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == RXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == RXFIFOSIZE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == RXFIFOSIZE_A::_111
    }
}
impl core::ops::Deref for RXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, RXFIFOSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0 = 0,
    #[doc = "1: Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1 = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Receive FIFO Enable"]
pub struct RXFE_R(crate::FieldReader<bool, RXFE_A>);
impl RXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::_0,
            true => RXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXFE_A::_1
    }
}
impl core::ops::Deref for RXFE_R {
    type Target = crate::FieldReader<bool, RXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFE` writer - Receive FIFO Enable"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFE_A::_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFE_A::_1)
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
#[doc = "Transmit FIFO. Buffer Depth\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFOSIZE_A {
    #[doc = "0: Transmit FIFO/Buffer depth = 1 dataword."]
    _000 = 0,
    #[doc = "1: Transmit FIFO/Buffer depth = 4 datawords."]
    _001 = 1,
    #[doc = "2: Transmit FIFO/Buffer depth = 8 datawords."]
    _010 = 2,
    #[doc = "3: Transmit FIFO/Buffer depth = 16 datawords."]
    _011 = 3,
    #[doc = "4: Transmit FIFO/Buffer depth = 32 datawords."]
    _100 = 4,
    #[doc = "5: Transmit FIFO/Buffer depth = 64 datawords."]
    _101 = 5,
    #[doc = "6: Transmit FIFO/Buffer depth = 128 datawords."]
    _110 = 6,
    #[doc = "7: Transmit FIFO/Buffer depth = 256 datawords"]
    _111 = 7,
}
impl From<TXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXFIFOSIZE` reader - Transmit FIFO. Buffer Depth"]
pub struct TXFIFOSIZE_R(crate::FieldReader<u8, TXFIFOSIZE_A>);
impl TXFIFOSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFOSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIFOSIZE_A {
        match self.bits {
            0 => TXFIFOSIZE_A::_000,
            1 => TXFIFOSIZE_A::_001,
            2 => TXFIFOSIZE_A::_010,
            3 => TXFIFOSIZE_A::_011,
            4 => TXFIFOSIZE_A::_100,
            5 => TXFIFOSIZE_A::_101,
            6 => TXFIFOSIZE_A::_110,
            7 => TXFIFOSIZE_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == TXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == TXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == TXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == TXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == TXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == TXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == TXFIFOSIZE_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == TXFIFOSIZE_A::_111
    }
}
impl core::ops::Deref for TXFIFOSIZE_R {
    type Target = crate::FieldReader<u8, TXFIFOSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "0: Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0 = 0,
    #[doc = "1: Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1 = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFE` reader - Transmit FIFO Enable"]
pub struct TXFE_R(crate::FieldReader<bool, TXFE_A>);
impl TXFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::_0,
            true => TXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXFE_A::_1
    }
}
impl core::ops::Deref for TXFE_R {
    type Target = crate::FieldReader<bool, TXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFE` writer - Transmit FIFO Enable"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFE_A::_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFE_A::_1)
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
#[doc = "Receive FIFO Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFE_A {
    #[doc = "0: RXUF flag does not generate an interrupt to the host."]
    _0 = 0,
    #[doc = "1: RXUF flag generates an interrupt to the host."]
    _1 = 1,
}
impl From<RXUFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXUFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUFE` reader - Receive FIFO Underflow Interrupt Enable"]
pub struct RXUFE_R(crate::FieldReader<bool, RXUFE_A>);
impl RXUFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUFE_A {
        match self.bits {
            false => RXUFE_A::_0,
            true => RXUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXUFE_A::_1
    }
}
impl core::ops::Deref for RXUFE_R {
    type Target = crate::FieldReader<bool, RXUFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUFE` writer - Receive FIFO Underflow Interrupt Enable"]
pub struct RXUFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFE_A::_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFE_A::_1)
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
#[doc = "Transmit FIFO Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFE_A {
    #[doc = "0: TXOF flag does not generate an interrupt to the host."]
    _0 = 0,
    #[doc = "1: TXOF flag generates an interrupt to the host."]
    _1 = 1,
}
impl From<TXOFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXOFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOFE` reader - Transmit FIFO Overflow Interrupt Enable"]
pub struct TXOFE_R(crate::FieldReader<bool, TXOFE_A>);
impl TXOFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOFE_A {
        match self.bits {
            false => TXOFE_A::_0,
            true => TXOFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXOFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXOFE_A::_1
    }
}
impl core::ops::Deref for TXOFE_R {
    type Target = crate::FieldReader<bool, TXOFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOFE` writer - Transmit FIFO Overflow Interrupt Enable"]
pub struct TXOFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFE_A::_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Receiver Idle Empty Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXIDEN_A {
    #[doc = "0: Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    _000 = 0,
    #[doc = "1: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    _001 = 1,
    #[doc = "2: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    _010 = 2,
    #[doc = "3: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    _011 = 3,
    #[doc = "4: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    _100 = 4,
    #[doc = "5: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    _101 = 5,
    #[doc = "6: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    _110 = 6,
    #[doc = "7: Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    _111 = 7,
}
impl From<RXIDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXIDEN` reader - Receiver Idle Empty Enable"]
pub struct RXIDEN_R(crate::FieldReader<u8, RXIDEN_A>);
impl RXIDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXIDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIDEN_A {
        match self.bits {
            0 => RXIDEN_A::_000,
            1 => RXIDEN_A::_001,
            2 => RXIDEN_A::_010,
            3 => RXIDEN_A::_011,
            4 => RXIDEN_A::_100,
            5 => RXIDEN_A::_101,
            6 => RXIDEN_A::_110,
            7 => RXIDEN_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == RXIDEN_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == RXIDEN_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        **self == RXIDEN_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        **self == RXIDEN_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        **self == RXIDEN_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        **self == RXIDEN_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        **self == RXIDEN_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        **self == RXIDEN_A::_111
    }
}
impl core::ops::Deref for RXIDEN_R {
    type Target = crate::FieldReader<u8, RXIDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIDEN` writer - Receiver Idle Empty Enable"]
pub struct RXIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIDEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RXIDEN_A::_000)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RXIDEN_A::_001)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RXIDEN_A::_010)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RXIDEN_A::_011)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RXIDEN_A::_100)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RXIDEN_A::_101)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RXIDEN_A::_110)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RXIDEN_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Receive FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0 = 0,
    #[doc = "1: All data in the receive FIFO/buffer is cleared out."]
    _1 = 1,
}
impl From<RXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFLUSH` writer - Receive FIFO/Buffer Flush"]
pub struct RXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFLUSH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFLUSH_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Transmit FIFO/Buffer Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFLUSH_AW {
    #[doc = "0: No flush operation occurs."]
    _0 = 0,
    #[doc = "1: All data in the transmit FIFO/Buffer is cleared out."]
    _1 = 1,
}
impl From<TXFLUSH_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFLUSH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFLUSH` writer - Transmit FIFO/Buffer Flush"]
pub struct TXFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFLUSH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No flush operation occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFLUSH_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Receiver Buffer Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUF_A {
    #[doc = "0: No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<RXUF_A> for bool {
    #[inline(always)]
    fn from(variant: RXUF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXUF` reader - Receiver Buffer Underflow Flag"]
pub struct RXUF_R(crate::FieldReader<bool, RXUF_A>);
impl RXUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXUF_A {
        match self.bits {
            false => RXUF_A::_0,
            true => RXUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXUF_A::_1
    }
}
impl core::ops::Deref for RXUF_R {
    type Target = crate::FieldReader<bool, RXUF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUF` writer - Receiver Buffer Underflow Flag"]
pub struct RXUF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXUF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUF_A::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Transmitter Buffer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOF_A {
    #[doc = "0: No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0 = 0,
    #[doc = "1: At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1 = 1,
}
impl From<TXOF_A> for bool {
    #[inline(always)]
    fn from(variant: TXOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXOF` reader - Transmitter Buffer Overflow Flag"]
pub struct TXOF_R(crate::FieldReader<bool, TXOF_A>);
impl TXOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXOF_A {
        match self.bits {
            false => TXOF_A::_0,
            true => TXOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXOF_A::_1
    }
}
impl core::ops::Deref for TXOF_R {
    type Target = crate::FieldReader<bool, TXOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOF` writer - Transmitter Buffer Overflow Flag"]
pub struct TXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOF_A::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Receive Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPT_A {
    #[doc = "0: Receive buffer is not empty."]
    _0 = 0,
    #[doc = "1: Receive buffer is empty."]
    _1 = 1,
}
impl From<RXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: RXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer/FIFO Empty"]
pub struct RXEMPT_R(crate::FieldReader<bool, RXEMPT_A>);
impl RXEMPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXEMPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEMPT_A {
        match self.bits {
            false => RXEMPT_A::_0,
            true => RXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXEMPT_A::_1
    }
}
impl core::ops::Deref for RXEMPT_R {
    type Target = crate::FieldReader<bool, RXEMPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Buffer/FIFO Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPT_A {
    #[doc = "0: Transmit buffer is not empty."]
    _0 = 0,
    #[doc = "1: Transmit buffer is empty."]
    _1 = 1,
}
impl From<TXEMPT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPT` reader - Transmit Buffer/FIFO Empty"]
pub struct TXEMPT_R(crate::FieldReader<bool, TXEMPT_A>);
impl TXEMPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPT_A {
        match self.bits {
            false => TXEMPT_A::_0,
            true => TXEMPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXEMPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPT_A::_1
    }
}
impl core::ops::Deref for TXEMPT_R {
    type Target = crate::FieldReader<bool, TXEMPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&self) -> RXUFE_R {
        RXUFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&self) -> TXOFE_R {
        TXOFE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    pub fn rxiden(&self) -> RXIDEN_R {
        RXIDEN_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Receive Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RXEMPT_R {
        RXEMPT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Transmit Buffer/FIFO Empty"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxufe(&mut self) -> RXUFE_W {
        RXUFE_W { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txofe(&mut self) -> TXOFE_W {
        TXOFE_W { w: self }
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline(always)]
    pub fn rxiden(&mut self) -> RXIDEN_W {
        RXIDEN_W { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn rxflush(&mut self) -> RXFLUSH_W {
        RXFLUSH_W { w: self }
    }
    #[doc = "Bit 15 - Transmit FIFO/Buffer Flush"]
    #[inline(always)]
    pub fn txflush(&mut self) -> TXFLUSH_W {
        TXFLUSH_W { w: self }
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W { w: self }
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART FIFO Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](index.html) module"]
pub struct FIFO_SPEC;
impl crate::RegisterSpec for FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo::R](R) reader structure"]
impl crate::Readable for FIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo::W](W) writer structure"]
impl crate::Writable for FIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO to value 0x00c0_0011"]
impl crate::Resettable for FIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00c0_0011
    }
}
