#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_A {
    #[doc = "0: Watchdog disabled in chip stop mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip stop mode."]
    _1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop Enable"]
pub struct STOP_R(crate::FieldReader<bool, STOP_A>);
impl STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::_0,
            true => STOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOP_A::_1
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Stop Enable"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP_A::_0)
    }
    #[doc = "Watchdog enabled in chip stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP_A::_1)
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
#[doc = "Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_A {
    #[doc = "0: Watchdog disabled in chip wait mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip wait mode."]
    _1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAIT` reader - Wait Enable"]
pub struct WAIT_R(crate::FieldReader<bool, WAIT_A>);
impl WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::_0,
            true => WAIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAIT_A::_1
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<bool, WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT` writer - Wait Enable"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAIT_A::_0)
    }
    #[doc = "Watchdog enabled in chip wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAIT_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_A {
    #[doc = "0: Watchdog disabled in chip debug mode."]
    _0 = 0,
    #[doc = "1: Watchdog enabled in chip debug mode."]
    _1 = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBG` reader - Debug Enable"]
pub struct DBG_R(crate::FieldReader<bool, DBG_A>);
impl DBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::_0,
            true => DBG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBG_A::_1
    }
}
impl core::ops::Deref for DBG_R {
    type Target = crate::FieldReader<bool, DBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG` writer - Debug Enable"]
pub struct DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled in chip debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBG_A::_0)
    }
    #[doc = "Watchdog enabled in chip debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBG_A::_1)
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
#[doc = "Watchdog Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TST_A {
    #[doc = "0: Watchdog test mode disabled."]
    _00 = 0,
    #[doc = "1: Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    _01 = 1,
    #[doc = "2: Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\]
is compared with TOVAL\\[TOVALLOW\\]."]
    _10 = 2,
    #[doc = "3: Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\]
is compared with TOVAL\\[TOVALHIGH\\]."]
    _11 = 3,
}
impl From<TST_A> for u8 {
    #[inline(always)]
    fn from(variant: TST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TST` reader - Watchdog Test"]
pub struct TST_R(crate::FieldReader<u8, TST_A>);
impl TST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TST_A {
        match self.bits {
            0 => TST_A::_00,
            1 => TST_A::_01,
            2 => TST_A::_10,
            3 => TST_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == TST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == TST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == TST_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == TST_A::_11
    }
}
impl core::ops::Deref for TST_R {
    type Target = crate::FieldReader<u8, TST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TST` writer - Watchdog Test"]
pub struct TST_W<'a> {
    w: &'a mut W,
}
impl<'a> TST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Watchdog test mode disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TST_A::_00)
    }
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TST_A::_01)
    }
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\]
is compared with TOVAL\\[TOVALLOW\\]."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TST_A::_10)
    }
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\]
is compared with TOVAL\\[TOVALHIGH\\]."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TST_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Allow updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATE_A {
    #[doc = "0: Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    _0 = 0,
    #[doc = "1: Updates allowed. Software can modify the watchdog configuration registers within 8'd128 bus clocks after performing the unlock write sequence."]
    _1 = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPDATE` reader - Allow updates"]
pub struct UPDATE_R(crate::FieldReader<bool, UPDATE_A>);
impl UPDATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UPDATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::_0,
            true => UPDATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == UPDATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == UPDATE_A::_1
    }
}
impl core::ops::Deref for UPDATE_R {
    type Target = crate::FieldReader<bool, UPDATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATE` writer - Allow updates"]
pub struct UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UPDATE_A::_0)
    }
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 8'd128 bus clocks after performing the unlock write sequence."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UPDATE_A::_1)
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
#[doc = "Watchdog Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_A {
    #[doc = "0: Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    _0 = 0,
    #[doc = "1: Watchdog interrupts are enabled. Watchdog resets are delayed by 8'd128 bus clocks from the interrupt vector fetch."]
    _1 = 1,
}
impl From<INT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT` reader - Watchdog Interrupt"]
pub struct INT_R(crate::FieldReader<bool, INT_A>);
impl INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_A {
        match self.bits {
            false => INT_A::_0,
            true => INT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INT_A::_1
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<bool, INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Watchdog Interrupt"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INT_A::_0)
    }
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 8'd128 bus clocks from the interrupt vector fetch."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Watchdog Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Watchdog disabled."]
    _0 = 0,
    #[doc = "1: Watchdog enabled."]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Watchdog Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EN_A::_1
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Watchdog Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Watchdog enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
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
#[doc = "Watchdog Clock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: Bus clock"]
    _00 = 0,
    #[doc = "1: LPO clock"]
    _01 = 1,
    #[doc = "2: INTCLK (internal clock)"]
    _10 = 2,
    #[doc = "3: ERCLK (external reference clock)"]
    _11 = 3,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK` reader - Watchdog Clock"]
pub struct CLK_R(crate::FieldReader<u8, CLK_A>);
impl CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::_00,
            1 => CLK_A::_01,
            2 => CLK_A::_10,
            3 => CLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == CLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == CLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == CLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CLK_A::_11
    }
}
impl core::ops::Deref for CLK_R {
    type Target = crate::FieldReader<u8, CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK` writer - Watchdog Clock"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLK_A::_00)
    }
    #[doc = "LPO clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLK_A::_01)
    }
    #[doc = "INTCLK (internal clock)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLK_A::_10)
    }
    #[doc = "ERCLK (external reference clock)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Reconfiguration Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCS_A {
    #[doc = "0: Reconfiguring WDOG."]
    _0 = 0,
    #[doc = "1: Reconfiguration is successful."]
    _1 = 1,
}
impl From<RCS_A> for bool {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCS` reader - Reconfiguration Success"]
pub struct RCS_R(crate::FieldReader<bool, RCS_A>);
impl RCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RCS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCS_A {
        match self.bits {
            false => RCS_A::_0,
            true => RCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RCS_A::_1
    }
}
impl core::ops::Deref for RCS_R {
    type Target = crate::FieldReader<bool, RCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Unlock status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULK_A {
    #[doc = "0: WDOG is locked."]
    _0 = 0,
    #[doc = "1: WDOG is unlocked."]
    _1 = 1,
}
impl From<ULK_A> for bool {
    #[inline(always)]
    fn from(variant: ULK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULK` reader - Unlock status"]
pub struct ULK_R(crate::FieldReader<bool, ULK_A>);
impl ULK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULK_A {
        match self.bits {
            false => ULK_A::_0,
            true => ULK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ULK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ULK_A::_1
    }
}
impl core::ops::Deref for ULK_R {
    type Target = crate::FieldReader<bool, ULK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Watchdog prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRES_A {
    #[doc = "0: 256 prescaler disabled."]
    _0 = 0,
    #[doc = "1: 256 prescaler enabled."]
    _1 = 1,
}
impl From<PRES_A> for bool {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRES` reader - Watchdog prescaler"]
pub struct PRES_R(crate::FieldReader<bool, PRES_A>);
impl PRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            false => PRES_A::_0,
            true => PRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PRES_A::_1
    }
}
impl core::ops::Deref for PRES_R {
    type Target = crate::FieldReader<bool, PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRES` writer - Watchdog prescaler"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "256 prescaler disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRES_A::_0)
    }
    #[doc = "256 prescaler enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRES_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD32EN_A {
    #[doc = "0: Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    _0 = 0,
    #[doc = "1: Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    _1 = 1,
}
impl From<CMD32EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD32EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD32EN` reader - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
pub struct CMD32EN_R(crate::FieldReader<bool, CMD32EN_A>);
impl CMD32EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD32EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD32EN_A {
        match self.bits {
            false => CMD32EN_A::_0,
            true => CMD32EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMD32EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMD32EN_A::_1
    }
}
impl core::ops::Deref for CMD32EN_R {
    type Target = crate::FieldReader<bool, CMD32EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD32EN` writer - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
pub struct CMD32EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD32EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD32EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMD32EN_A::_0)
    }
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMD32EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Watchdog Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLG_A {
    #[doc = "0: No interrupt occurred."]
    _0 = 0,
    #[doc = "1: An interrupt occurred."]
    _1 = 1,
}
impl From<FLG_A> for bool {
    #[inline(always)]
    fn from(variant: FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLG` reader - Watchdog Interrupt Flag"]
pub struct FLG_R(crate::FieldReader<bool, FLG_A>);
impl FLG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLG_A {
        match self.bits {
            false => FLG_A::_0,
            true => FLG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FLG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FLG_A::_1
    }
}
impl core::ops::Deref for FLG_R {
    type Target = crate::FieldReader<bool, FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLG` writer - Watchdog Interrupt Flag"]
pub struct FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLG_A::_0)
    }
    #[doc = "An interrupt occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLG_A::_1)
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
#[doc = "Watchdog Window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIN_A {
    #[doc = "0: Window mode disabled."]
    _0 = 0,
    #[doc = "1: Window mode enabled."]
    _1 = 1,
}
impl From<WIN_A> for bool {
    #[inline(always)]
    fn from(variant: WIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIN` reader - Watchdog Window"]
pub struct WIN_R(crate::FieldReader<bool, WIN_A>);
impl WIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIN_A {
        match self.bits {
            false => WIN_A::_0,
            true => WIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WIN_A::_1
    }
}
impl core::ops::Deref for WIN_R {
    type Target = crate::FieldReader<bool, WIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIN` writer - Watchdog Window"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Window mode disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WIN_A::_0)
    }
    #[doc = "Window mode enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WIN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    pub fn tst(&self) -> TST_R {
        TST_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Reconfiguration Success"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Unlock status"]
    #[inline(always)]
    pub fn ulk(&self) -> ULK_R {
        ULK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub fn cmd32en(&self) -> CMD32EN_R {
        CMD32EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn flg(&self) -> FLG_R {
        FLG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W { w: self }
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    pub fn tst(&mut self) -> TST_W {
        TST_W { w: self }
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub fn cmd32en(&mut self) -> CMD32EN_W {
        CMD32EN_W { w: self }
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn flg(&mut self) -> FLG_W {
        FLG_W { w: self }
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CS to value 0x2980"]
impl crate::Resettable for CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2980
    }
}
