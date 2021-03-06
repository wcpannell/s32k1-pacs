#[doc = "Register `TCTRL2` reader"]
pub struct R(crate::R<TCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCTRL2` writer"]
pub struct W(crate::W<TCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCTRL2_SPEC>;
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
impl From<crate::W<TCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T_EN_A {
    #[doc = "0: Timer Channel is disabled"]
    _0 = 0,
    #[doc = "1: Timer Channel is enabled"]
    _1 = 1,
}
impl From<T_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `T_EN` reader - Timer Enable"]
pub struct T_EN_R(crate::FieldReader<bool, T_EN_A>);
impl T_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        T_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T_EN_A {
        match self.bits {
            false => T_EN_A::_0,
            true => T_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == T_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == T_EN_A::_1
    }
}
impl core::ops::Deref for T_EN_R {
    type Target = crate::FieldReader<bool, T_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_EN` writer - Timer Enable"]
pub struct T_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> T_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: T_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer Channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(T_EN_A::_0)
    }
    #[doc = "Timer Channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(T_EN_A::_1)
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
#[doc = "Chain Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHAIN_A {
    #[doc = "0: Channel Chaining is disabled. Channel Timer runs independently."]
    _0 = 0,
    #[doc = "1: Channel Chaining is enabled. Timer decrements on previous channel's timeout"]
    _1 = 1,
}
impl From<CHAIN_A> for bool {
    #[inline(always)]
    fn from(variant: CHAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHAIN` reader - Chain Channel"]
pub struct CHAIN_R(crate::FieldReader<bool, CHAIN_A>);
impl CHAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHAIN_A {
        match self.bits {
            false => CHAIN_A::_0,
            true => CHAIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHAIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHAIN_A::_1
    }
}
impl core::ops::Deref for CHAIN_R {
    type Target = crate::FieldReader<bool, CHAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAIN` writer - Chain Channel"]
pub struct CHAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHAIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel Chaining is disabled. Channel Timer runs independently."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAIN_A::_0)
    }
    #[doc = "Channel Chaining is enabled. Timer decrements on previous channel's timeout"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAIN_A::_1)
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
#[doc = "Timer Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 32-bit Periodic Counter"]
    _0 = 0,
    #[doc = "1: Dual 16-bit Periodic Counter"]
    _1 = 1,
    #[doc = "2: 32-bit Trigger Accumulator"]
    _10 = 2,
    #[doc = "3: 32-bit Trigger Input Capture"]
    _11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Timer Operation Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_0,
            1 => MODE_A::_1,
            2 => MODE_A::_10,
            3 => MODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODE_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == MODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == MODE_A::_11
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Timer Operation Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "32-bit Periodic Counter"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE_A::_0)
    }
    #[doc = "Dual 16-bit Periodic Counter"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE_A::_1)
    }
    #[doc = "32-bit Trigger Accumulator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_A::_10)
    }
    #[doc = "32-bit Trigger Input Capture"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODE_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Timer Start On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOT_A {
    #[doc = "0: Timer starts to decrement immediately based on restart condition (controlled by TSOI bit)"]
    _0 = 0,
    #[doc = "1: Timer starts to decrement when rising edge on selected trigger is detected"]
    _1 = 1,
}
impl From<TSOT_A> for bool {
    #[inline(always)]
    fn from(variant: TSOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOT` reader - Timer Start On Trigger"]
pub struct TSOT_R(crate::FieldReader<bool, TSOT_A>);
impl TSOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOT_A {
        match self.bits {
            false => TSOT_A::_0,
            true => TSOT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSOT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSOT_A::_1
    }
}
impl core::ops::Deref for TSOT_R {
    type Target = crate::FieldReader<bool, TSOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOT` writer - Timer Start On Trigger"]
pub struct TSOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSOT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer starts to decrement immediately based on restart condition (controlled by TSOI bit)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOT_A::_0)
    }
    #[doc = "Timer starts to decrement when rising edge on selected trigger is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOT_A::_1)
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
#[doc = "Timer Stop On Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOI_A {
    #[doc = "0: The channel timer does not stop after timeout."]
    _0 = 0,
    #[doc = "1: The channel timer will stop after a timeout, and the channel timer will restart based on TSOT. When TSOT = 0, the channel timer will restart after a rising edge on the T_EN bit is detected (which means that the timer channel is disabled and then enabled); when TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
    _1 = 1,
}
impl From<TSOI_A> for bool {
    #[inline(always)]
    fn from(variant: TSOI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSOI` reader - Timer Stop On Interrupt"]
pub struct TSOI_R(crate::FieldReader<bool, TSOI_A>);
impl TSOI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSOI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSOI_A {
        match self.bits {
            false => TSOI_A::_0,
            true => TSOI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TSOI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TSOI_A::_1
    }
}
impl core::ops::Deref for TSOI_R {
    type Target = crate::FieldReader<bool, TSOI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSOI` writer - Timer Stop On Interrupt"]
pub struct TSOI_W<'a> {
    w: &'a mut W,
}
impl<'a> TSOI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSOI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel timer does not stop after timeout."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOI_A::_0)
    }
    #[doc = "The channel timer will stop after a timeout, and the channel timer will restart based on TSOT. When TSOT = 0, the channel timer will restart after a rising edge on the T_EN bit is detected (which means that the timer channel is disabled and then enabled); when TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOI_A::_1)
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
#[doc = "Timer Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TROT_A {
    #[doc = "0: Timer will not reload on selected trigger"]
    _0 = 0,
    #[doc = "1: Timer will reload on selected trigger"]
    _1 = 1,
}
impl From<TROT_A> for bool {
    #[inline(always)]
    fn from(variant: TROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TROT` reader - Timer Reload On Trigger"]
pub struct TROT_R(crate::FieldReader<bool, TROT_A>);
impl TROT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TROT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TROT_A {
        match self.bits {
            false => TROT_A::_0,
            true => TROT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TROT_A::_1
    }
}
impl core::ops::Deref for TROT_R {
    type Target = crate::FieldReader<bool, TROT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TROT` writer - Timer Reload On Trigger"]
pub struct TROT_W<'a> {
    w: &'a mut W,
}
impl<'a> TROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TROT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer will not reload on selected trigger"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TROT_A::_0)
    }
    #[doc = "Timer will reload on selected trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TROT_A::_1)
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
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRG_SRC_A {
    #[doc = "0: Trigger source selected in external"]
    _0 = 0,
    #[doc = "1: Trigger source selected is the internal trigger"]
    _1 = 1,
}
impl From<TRG_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: TRG_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRG_SRC` reader - Trigger Source"]
pub struct TRG_SRC_R(crate::FieldReader<bool, TRG_SRC_A>);
impl TRG_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRG_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRG_SRC_A {
        match self.bits {
            false => TRG_SRC_A::_0,
            true => TRG_SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TRG_SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TRG_SRC_A::_1
    }
}
impl core::ops::Deref for TRG_SRC_R {
    type Target = crate::FieldReader<bool, TRG_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRG_SRC` writer - Trigger Source"]
pub struct TRG_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRG_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger source selected in external"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRG_SRC_A::_0)
    }
    #[doc = "Trigger source selected is the internal trigger"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRG_SRC_A::_1)
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
#[doc = "Field `TRG_SEL` reader - Trigger Select"]
pub struct TRG_SEL_R(crate::FieldReader<u8, u8>);
impl TRG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRG_SEL` writer - Trigger Select"]
pub struct TRG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn t_en(&self) -> T_EN_R {
        T_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Chain Channel"]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Timer Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Timer Start On Trigger"]
    #[inline(always)]
    pub fn tsot(&self) -> TSOT_R {
        TSOT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer Stop On Interrupt"]
    #[inline(always)]
    pub fn tsoi(&self) -> TSOI_R {
        TSOI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer Reload On Trigger"]
    #[inline(always)]
    pub fn trot(&self) -> TROT_R {
        TROT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    pub fn trg_src(&self) -> TRG_SRC_R {
        TRG_SRC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trg_sel(&self) -> TRG_SEL_R {
        TRG_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn t_en(&mut self) -> T_EN_W {
        T_EN_W { w: self }
    }
    #[doc = "Bit 1 - Chain Channel"]
    #[inline(always)]
    pub fn chain(&mut self) -> CHAIN_W {
        CHAIN_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 16 - Timer Start On Trigger"]
    #[inline(always)]
    pub fn tsot(&mut self) -> TSOT_W {
        TSOT_W { w: self }
    }
    #[doc = "Bit 17 - Timer Stop On Interrupt"]
    #[inline(always)]
    pub fn tsoi(&mut self) -> TSOI_W {
        TSOI_W { w: self }
    }
    #[doc = "Bit 18 - Timer Reload On Trigger"]
    #[inline(always)]
    pub fn trot(&mut self) -> TROT_W {
        TROT_W { w: self }
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    pub fn trg_src(&mut self) -> TRG_SRC_W {
        TRG_SRC_W { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trg_sel(&mut self) -> TRG_SEL_W {
        TRG_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctrl2](index.html) module"]
pub struct TCTRL2_SPEC;
impl crate::RegisterSpec for TCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tctrl2::R](R) reader structure"]
impl crate::Readable for TCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tctrl2::W](W) writer structure"]
impl crate::Writable for TCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCTRL2 to value 0"]
impl crate::Resettable for TCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
