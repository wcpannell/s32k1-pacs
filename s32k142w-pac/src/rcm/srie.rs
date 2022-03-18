#[doc = "Register `SRIE` reader"]
pub struct R(crate::R<SRIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRIE` writer"]
pub struct W(crate::W<SRIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRIE_SPEC>;
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
impl From<crate::W<SRIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset Delay Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DELAY_A {
    #[doc = "0: 10 LPO cycles"]
    DELAY_0 = 0,
    #[doc = "1: 34 LPO cycles"]
    DELAY_1 = 1,
    #[doc = "2: 130 LPO cycles"]
    DELAY_2 = 2,
    #[doc = "3: 514 LPO cycles"]
    DELAY_3 = 3,
}
impl From<DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DELAY` reader - Reset Delay Time"]
pub struct DELAY_R(crate::FieldReader<u8, DELAY_A>);
impl DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DELAY_A {
        match self.bits {
            0 => DELAY_A::DELAY_0,
            1 => DELAY_A::DELAY_1,
            2 => DELAY_A::DELAY_2,
            3 => DELAY_A::DELAY_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DELAY_0`"]
    #[inline(always)]
    pub fn is_delay_0(&self) -> bool {
        **self == DELAY_A::DELAY_0
    }
    #[doc = "Checks if the value of the field is `DELAY_1`"]
    #[inline(always)]
    pub fn is_delay_1(&self) -> bool {
        **self == DELAY_A::DELAY_1
    }
    #[doc = "Checks if the value of the field is `DELAY_2`"]
    #[inline(always)]
    pub fn is_delay_2(&self) -> bool {
        **self == DELAY_A::DELAY_2
    }
    #[doc = "Checks if the value of the field is `DELAY_3`"]
    #[inline(always)]
    pub fn is_delay_3(&self) -> bool {
        **self == DELAY_A::DELAY_3
    }
}
impl core::ops::Deref for DELAY_R {
    type Target = crate::FieldReader<u8, DELAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DELAY` writer - Reset Delay Time"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DELAY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "10 LPO cycles"]
    #[inline(always)]
    pub fn delay_0(self) -> &'a mut W {
        self.variant(DELAY_A::DELAY_0)
    }
    #[doc = "34 LPO cycles"]
    #[inline(always)]
    pub fn delay_1(self) -> &'a mut W {
        self.variant(DELAY_A::DELAY_1)
    }
    #[doc = "130 LPO cycles"]
    #[inline(always)]
    pub fn delay_2(self) -> &'a mut W {
        self.variant(DELAY_A::DELAY_2)
    }
    #[doc = "514 LPO cycles"]
    #[inline(always)]
    pub fn delay_3(self) -> &'a mut W {
        self.variant(DELAY_A::DELAY_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Loss-of-Clock Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOC_A {
    #[doc = "0: Interrupt disabled."]
    LOC_0 = 0,
    #[doc = "1: Interrupt enabled."]
    LOC_1 = 1,
}
impl From<LOC_A> for bool {
    #[inline(always)]
    fn from(variant: LOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOC` reader - Loss-of-Clock Interrupt"]
pub struct LOC_R(crate::FieldReader<bool, LOC_A>);
impl LOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOC_A {
        match self.bits {
            false => LOC_A::LOC_0,
            true => LOC_A::LOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOC_0`"]
    #[inline(always)]
    pub fn is_loc_0(&self) -> bool {
        **self == LOC_A::LOC_0
    }
    #[doc = "Checks if the value of the field is `LOC_1`"]
    #[inline(always)]
    pub fn is_loc_1(&self) -> bool {
        **self == LOC_A::LOC_1
    }
}
impl core::ops::Deref for LOC_R {
    type Target = crate::FieldReader<bool, LOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOC` writer - Loss-of-Clock Interrupt"]
pub struct LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn loc_0(self) -> &'a mut W {
        self.variant(LOC_A::LOC_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn loc_1(self) -> &'a mut W {
        self.variant(LOC_A::LOC_1)
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
#[doc = "Loss-of-Lock Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOL_A {
    #[doc = "0: Interrupt disabled."]
    LOL_0 = 0,
    #[doc = "1: Interrupt enabled."]
    LOL_1 = 1,
}
impl From<LOL_A> for bool {
    #[inline(always)]
    fn from(variant: LOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOL` reader - Loss-of-Lock Interrupt"]
pub struct LOL_R(crate::FieldReader<bool, LOL_A>);
impl LOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOL_A {
        match self.bits {
            false => LOL_A::LOL_0,
            true => LOL_A::LOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOL_0`"]
    #[inline(always)]
    pub fn is_lol_0(&self) -> bool {
        **self == LOL_A::LOL_0
    }
    #[doc = "Checks if the value of the field is `LOL_1`"]
    #[inline(always)]
    pub fn is_lol_1(&self) -> bool {
        **self == LOL_A::LOL_1
    }
}
impl core::ops::Deref for LOL_R {
    type Target = crate::FieldReader<bool, LOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOL` writer - Loss-of-Lock Interrupt"]
pub struct LOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn lol_0(self) -> &'a mut W {
        self.variant(LOL_A::LOL_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn lol_1(self) -> &'a mut W {
        self.variant(LOL_A::LOL_1)
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
#[doc = "Watchdog Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG_A {
    #[doc = "0: Interrupt disabled."]
    WDOG_0 = 0,
    #[doc = "1: Interrupt enabled."]
    WDOG_1 = 1,
}
impl From<WDOG_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog Interrupt"]
pub struct WDOG_R(crate::FieldReader<bool, WDOG_A>);
impl WDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG_A {
        match self.bits {
            false => WDOG_A::WDOG_0,
            true => WDOG_A::WDOG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG_0`"]
    #[inline(always)]
    pub fn is_wdog_0(&self) -> bool {
        **self == WDOG_A::WDOG_0
    }
    #[doc = "Checks if the value of the field is `WDOG_1`"]
    #[inline(always)]
    pub fn is_wdog_1(&self) -> bool {
        **self == WDOG_A::WDOG_1
    }
}
impl core::ops::Deref for WDOG_R {
    type Target = crate::FieldReader<bool, WDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG` writer - Watchdog Interrupt"]
pub struct WDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDOG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn wdog_0(self) -> &'a mut W {
        self.variant(WDOG_A::WDOG_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn wdog_1(self) -> &'a mut W {
        self.variant(WDOG_A::WDOG_1)
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
#[doc = "External Reset Pin Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    PIN_0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    PIN_1 = 1,
}
impl From<PIN_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External Reset Pin Interrupt"]
pub struct PIN_R(crate::FieldReader<bool, PIN_A>);
impl PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_A {
        match self.bits {
            false => PIN_A::PIN_0,
            true => PIN_A::PIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_0`"]
    #[inline(always)]
    pub fn is_pin_0(&self) -> bool {
        **self == PIN_A::PIN_0
    }
    #[doc = "Checks if the value of the field is `PIN_1`"]
    #[inline(always)]
    pub fn is_pin_1(&self) -> bool {
        **self == PIN_A::PIN_1
    }
}
impl core::ops::Deref for PIN_R {
    type Target = crate::FieldReader<bool, PIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN` writer - External Reset Pin Interrupt"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn pin_0(self) -> &'a mut W {
        self.variant(PIN_A::PIN_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn pin_1(self) -> &'a mut W {
        self.variant(PIN_A::PIN_1)
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
#[doc = "Global Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIE_A {
    #[doc = "0: All interrupt sources disabled."]
    GIE_0 = 0,
    #[doc = "1: All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    GIE_1 = 1,
}
impl From<GIE_A> for bool {
    #[inline(always)]
    fn from(variant: GIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GIE` reader - Global Interrupt Enable"]
pub struct GIE_R(crate::FieldReader<bool, GIE_A>);
impl GIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIE_A {
        match self.bits {
            false => GIE_A::GIE_0,
            true => GIE_A::GIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GIE_0`"]
    #[inline(always)]
    pub fn is_gie_0(&self) -> bool {
        **self == GIE_A::GIE_0
    }
    #[doc = "Checks if the value of the field is `GIE_1`"]
    #[inline(always)]
    pub fn is_gie_1(&self) -> bool {
        **self == GIE_A::GIE_1
    }
}
impl core::ops::Deref for GIE_R {
    type Target = crate::FieldReader<bool, GIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GIE` writer - Global Interrupt Enable"]
pub struct GIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All interrupt sources disabled."]
    #[inline(always)]
    pub fn gie_0(self) -> &'a mut W {
        self.variant(GIE_A::GIE_0)
    }
    #[doc = "All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    #[inline(always)]
    pub fn gie_1(self) -> &'a mut W {
        self.variant(GIE_A::GIE_1)
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
#[doc = "JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAG_A {
    #[doc = "0: Interrupt disabled."]
    JTAG_0 = 0,
    #[doc = "1: Interrupt enabled."]
    JTAG_1 = 1,
}
impl From<JTAG_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JTAG` reader - JTAG generated reset"]
pub struct JTAG_R(crate::FieldReader<bool, JTAG_A>);
impl JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_A {
        match self.bits {
            false => JTAG_A::JTAG_0,
            true => JTAG_A::JTAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `JTAG_0`"]
    #[inline(always)]
    pub fn is_jtag_0(&self) -> bool {
        **self == JTAG_A::JTAG_0
    }
    #[doc = "Checks if the value of the field is `JTAG_1`"]
    #[inline(always)]
    pub fn is_jtag_1(&self) -> bool {
        **self == JTAG_A::JTAG_1
    }
}
impl core::ops::Deref for JTAG_R {
    type Target = crate::FieldReader<bool, JTAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG` writer - JTAG generated reset"]
pub struct JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JTAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn jtag_0(self) -> &'a mut W {
        self.variant(JTAG_A::JTAG_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn jtag_1(self) -> &'a mut W {
        self.variant(JTAG_A::JTAG_1)
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
#[doc = "Core Lockup Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUP_A {
    #[doc = "0: Interrupt disabled."]
    LOCKUP_0 = 0,
    #[doc = "1: Interrupt enabled."]
    LOCKUP_1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup Interrupt"]
pub struct LOCKUP_R(crate::FieldReader<bool, LOCKUP_A>);
impl LOCKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::LOCKUP_0,
            true => LOCKUP_A::LOCKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKUP_0`"]
    #[inline(always)]
    pub fn is_lockup_0(&self) -> bool {
        **self == LOCKUP_A::LOCKUP_0
    }
    #[doc = "Checks if the value of the field is `LOCKUP_1`"]
    #[inline(always)]
    pub fn is_lockup_1(&self) -> bool {
        **self == LOCKUP_A::LOCKUP_1
    }
}
impl core::ops::Deref for LOCKUP_R {
    type Target = crate::FieldReader<bool, LOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUP` writer - Core Lockup Interrupt"]
pub struct LOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn lockup_0(self) -> &'a mut W {
        self.variant(LOCKUP_A::LOCKUP_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn lockup_1(self) -> &'a mut W {
        self.variant(LOCKUP_A::LOCKUP_1)
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
#[doc = "Software Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_A {
    #[doc = "0: Interrupt disabled."]
    SW_0 = 0,
    #[doc = "1: Interrupt enabled."]
    SW_1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - Software Interrupt"]
pub struct SW_R(crate::FieldReader<bool, SW_A>);
impl SW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::SW_0,
            true => SW_A::SW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SW_0`"]
    #[inline(always)]
    pub fn is_sw_0(&self) -> bool {
        **self == SW_A::SW_0
    }
    #[doc = "Checks if the value of the field is `SW_1`"]
    #[inline(always)]
    pub fn is_sw_1(&self) -> bool {
        **self == SW_A::SW_1
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<bool, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - Software Interrupt"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn sw_0(self) -> &'a mut W {
        self.variant(SW_A::SW_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn sw_1(self) -> &'a mut W {
        self.variant(SW_A::SW_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_AP_A {
    #[doc = "0: Interrupt disabled."]
    MDM_AP_0 = 0,
    #[doc = "1: Interrupt enabled."]
    MDM_AP_1 = 1,
}
impl From<MDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: MDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDM_AP` reader - MDM-AP System Reset Request"]
pub struct MDM_AP_R(crate::FieldReader<bool, MDM_AP_A>);
impl MDM_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDM_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDM_AP_A {
        match self.bits {
            false => MDM_AP_A::MDM_AP_0,
            true => MDM_AP_A::MDM_AP_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDM_AP_0`"]
    #[inline(always)]
    pub fn is_mdm_ap_0(&self) -> bool {
        **self == MDM_AP_A::MDM_AP_0
    }
    #[doc = "Checks if the value of the field is `MDM_AP_1`"]
    #[inline(always)]
    pub fn is_mdm_ap_1(&self) -> bool {
        **self == MDM_AP_A::MDM_AP_1
    }
}
impl core::ops::Deref for MDM_AP_R {
    type Target = crate::FieldReader<bool, MDM_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDM_AP` writer - MDM-AP System Reset Request"]
pub struct MDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> MDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDM_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn mdm_ap_0(self) -> &'a mut W {
        self.variant(MDM_AP_A::MDM_AP_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn mdm_ap_1(self) -> &'a mut W {
        self.variant(MDM_AP_A::MDM_AP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Stop Acknowledge Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERR_A {
    #[doc = "0: Interrupt disabled."]
    SACKERR_0 = 0,
    #[doc = "1: Interrupt enabled."]
    SACKERR_1 = 1,
}
impl From<SACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKERR` reader - Stop Acknowledge Error Interrupt"]
pub struct SACKERR_R(crate::FieldReader<bool, SACKERR_A>);
impl SACKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKERR_A {
        match self.bits {
            false => SACKERR_A::SACKERR_0,
            true => SACKERR_A::SACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SACKERR_0`"]
    #[inline(always)]
    pub fn is_sackerr_0(&self) -> bool {
        **self == SACKERR_A::SACKERR_0
    }
    #[doc = "Checks if the value of the field is `SACKERR_1`"]
    #[inline(always)]
    pub fn is_sackerr_1(&self) -> bool {
        **self == SACKERR_A::SACKERR_1
    }
}
impl core::ops::Deref for SACKERR_R {
    type Target = crate::FieldReader<bool, SACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SACKERR` writer - Stop Acknowledge Error Interrupt"]
pub struct SACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SACKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn sackerr_0(self) -> &'a mut W {
        self.variant(SACKERR_A::SACKERR_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn sackerr_1(self) -> &'a mut W {
        self.variant(SACKERR_A::SACKERR_1)
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
impl R {
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline(always)]
    pub fn lol(&self) -> LOL_R {
        LOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdog(&self) -> WDOG_R {
        WDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn gie(&self) -> GIE_R {
        GIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MDM_AP_R {
        MDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn sackerr(&self) -> SACKERR_R {
        SACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline(always)]
    pub fn loc(&mut self) -> LOC_W {
        LOC_W { w: self }
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline(always)]
    pub fn lol(&mut self) -> LOL_W {
        LOL_W { w: self }
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WDOG_W {
        WDOG_W { w: self }
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn gie(&mut self) -> GIE_W {
        GIE_W { w: self }
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline(always)]
    pub fn jtag(&mut self) -> JTAG_W {
        JTAG_W { w: self }
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline(always)]
    pub fn lockup(&mut self) -> LOCKUP_W {
        LOCKUP_W { w: self }
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&mut self) -> MDM_AP_W {
        MDM_AP_W { w: self }
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline(always)]
    pub fn sackerr(&mut self) -> SACKERR_W {
        SACKERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Reset Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srie](index.html) module"]
pub struct SRIE_SPEC;
impl crate::RegisterSpec for SRIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srie::R](R) reader structure"]
impl crate::Readable for SRIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srie::W](W) writer structure"]
impl crate::Writable for SRIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRIE to value 0"]
impl crate::Resettable for SRIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
