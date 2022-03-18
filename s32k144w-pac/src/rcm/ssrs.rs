#[doc = "Register `SSRS` reader"]
pub struct R(crate::R<SSRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRS` writer"]
pub struct W(crate::W<SSRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRS_SPEC>;
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
impl From<crate::W<SSRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    SLVD_0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    SLVD_1 = 1,
}
impl From<SLVD_A> for bool {
    #[inline(always)]
    fn from(variant: SLVD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVD` reader - Sticky Low-Voltage Detect Reset"]
pub struct SLVD_R(crate::FieldReader<bool, SLVD_A>);
impl SLVD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVD_A {
        match self.bits {
            false => SLVD_A::SLVD_0,
            true => SLVD_A::SLVD_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLVD_0`"]
    #[inline(always)]
    pub fn is_slvd_0(&self) -> bool {
        **self == SLVD_A::SLVD_0
    }
    #[doc = "Checks if the value of the field is `SLVD_1`"]
    #[inline(always)]
    pub fn is_slvd_1(&self) -> bool {
        **self == SLVD_A::SLVD_1
    }
}
impl core::ops::Deref for SLVD_R {
    type Target = crate::FieldReader<bool, SLVD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVD` writer - Sticky Low-Voltage Detect Reset"]
pub struct SLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn slvd_0(self) -> &'a mut W {
        self.variant(SLVD_A::SLVD_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn slvd_1(self) -> &'a mut W {
        self.variant(SLVD_A::SLVD_1)
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
#[doc = "Sticky Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    SLOC_0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    SLOC_1 = 1,
}
impl From<SLOC_A> for bool {
    #[inline(always)]
    fn from(variant: SLOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOC` reader - Sticky Loss-of-Clock Reset"]
pub struct SLOC_R(crate::FieldReader<bool, SLOC_A>);
impl SLOC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOC_A {
        match self.bits {
            false => SLOC_A::SLOC_0,
            true => SLOC_A::SLOC_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOC_0`"]
    #[inline(always)]
    pub fn is_sloc_0(&self) -> bool {
        **self == SLOC_A::SLOC_0
    }
    #[doc = "Checks if the value of the field is `SLOC_1`"]
    #[inline(always)]
    pub fn is_sloc_1(&self) -> bool {
        **self == SLOC_A::SLOC_1
    }
}
impl core::ops::Deref for SLOC_R {
    type Target = crate::FieldReader<bool, SLOC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOC` writer - Sticky Loss-of-Clock Reset"]
pub struct SLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline(always)]
    pub fn sloc_0(self) -> &'a mut W {
        self.variant(SLOC_A::SLOC_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline(always)]
    pub fn sloc_1(self) -> &'a mut W {
        self.variant(SLOC_A::SLOC_1)
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
#[doc = "Sticky Loss-of-Lock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOL_A {
    #[doc = "0: Reset not caused by a loss of lock in the PLL/FLL"]
    SLOL_0 = 0,
    #[doc = "1: Reset caused by a loss of lock in the PLL/FLL"]
    SLOL_1 = 1,
}
impl From<SLOL_A> for bool {
    #[inline(always)]
    fn from(variant: SLOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOL` reader - Sticky Loss-of-Lock Reset"]
pub struct SLOL_R(crate::FieldReader<bool, SLOL_A>);
impl SLOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOL_A {
        match self.bits {
            false => SLOL_A::SLOL_0,
            true => SLOL_A::SLOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOL_0`"]
    #[inline(always)]
    pub fn is_slol_0(&self) -> bool {
        **self == SLOL_A::SLOL_0
    }
    #[doc = "Checks if the value of the field is `SLOL_1`"]
    #[inline(always)]
    pub fn is_slol_1(&self) -> bool {
        **self == SLOL_A::SLOL_1
    }
}
impl core::ops::Deref for SLOL_R {
    type Target = crate::FieldReader<bool, SLOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOL` writer - Sticky Loss-of-Lock Reset"]
pub struct SLOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    #[inline(always)]
    pub fn slol_0(self) -> &'a mut W {
        self.variant(SLOL_A::SLOL_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    #[inline(always)]
    pub fn slol_1(self) -> &'a mut W {
        self.variant(SLOL_A::SLOL_1)
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
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    SWDOG_0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    SWDOG_1 = 1,
}
impl From<SWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: SWDOG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDOG` reader - Sticky Watchdog"]
pub struct SWDOG_R(crate::FieldReader<bool, SWDOG_A>);
impl SWDOG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWDOG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDOG_A {
        match self.bits {
            false => SWDOG_A::SWDOG_0,
            true => SWDOG_A::SWDOG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWDOG_0`"]
    #[inline(always)]
    pub fn is_swdog_0(&self) -> bool {
        **self == SWDOG_A::SWDOG_0
    }
    #[doc = "Checks if the value of the field is `SWDOG_1`"]
    #[inline(always)]
    pub fn is_swdog_1(&self) -> bool {
        **self == SWDOG_A::SWDOG_1
    }
}
impl core::ops::Deref for SWDOG_R {
    type Target = crate::FieldReader<bool, SWDOG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWDOG` writer - Sticky Watchdog"]
pub struct SWDOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDOG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWDOG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn swdog_0(self) -> &'a mut W {
        self.variant(SWDOG_A::SWDOG_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn swdog_1(self) -> &'a mut W {
        self.variant(SWDOG_A::SWDOG_1)
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
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    SPIN_0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    SPIN_1 = 1,
}
impl From<SPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIN` reader - Sticky External Reset Pin"]
pub struct SPIN_R(crate::FieldReader<bool, SPIN_A>);
impl SPIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_A {
        match self.bits {
            false => SPIN_A::SPIN_0,
            true => SPIN_A::SPIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPIN_0`"]
    #[inline(always)]
    pub fn is_spin_0(&self) -> bool {
        **self == SPIN_A::SPIN_0
    }
    #[doc = "Checks if the value of the field is `SPIN_1`"]
    #[inline(always)]
    pub fn is_spin_1(&self) -> bool {
        **self == SPIN_A::SPIN_1
    }
}
impl core::ops::Deref for SPIN_R {
    type Target = crate::FieldReader<bool, SPIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIN` writer - Sticky External Reset Pin"]
pub struct SPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn spin_0(self) -> &'a mut W {
        self.variant(SPIN_A::SPIN_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn spin_1(self) -> &'a mut W {
        self.variant(SPIN_A::SPIN_1)
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
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOR_A {
    #[doc = "0: Reset not caused by POR"]
    SPOR_0 = 0,
    #[doc = "1: Reset caused by POR"]
    SPOR_1 = 1,
}
impl From<SPOR_A> for bool {
    #[inline(always)]
    fn from(variant: SPOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOR` reader - Sticky Power-On Reset"]
pub struct SPOR_R(crate::FieldReader<bool, SPOR_A>);
impl SPOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOR_A {
        match self.bits {
            false => SPOR_A::SPOR_0,
            true => SPOR_A::SPOR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPOR_0`"]
    #[inline(always)]
    pub fn is_spor_0(&self) -> bool {
        **self == SPOR_A::SPOR_0
    }
    #[doc = "Checks if the value of the field is `SPOR_1`"]
    #[inline(always)]
    pub fn is_spor_1(&self) -> bool {
        **self == SPOR_A::SPOR_1
    }
}
impl core::ops::Deref for SPOR_R {
    type Target = crate::FieldReader<bool, SPOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOR` writer - Sticky Power-On Reset"]
pub struct SPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn spor_0(self) -> &'a mut W {
        self.variant(SPOR_A::SPOR_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn spor_1(self) -> &'a mut W {
        self.variant(SPOR_A::SPOR_1)
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
#[doc = "Sticky JTAG generated reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SJTAG_A {
    #[doc = "0: Reset not caused by JTAG"]
    SJTAG_0 = 0,
    #[doc = "1: Reset caused by JTAG"]
    SJTAG_1 = 1,
}
impl From<SJTAG_A> for bool {
    #[inline(always)]
    fn from(variant: SJTAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SJTAG` reader - Sticky JTAG generated reset"]
pub struct SJTAG_R(crate::FieldReader<bool, SJTAG_A>);
impl SJTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SJTAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SJTAG_A {
        match self.bits {
            false => SJTAG_A::SJTAG_0,
            true => SJTAG_A::SJTAG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SJTAG_0`"]
    #[inline(always)]
    pub fn is_sjtag_0(&self) -> bool {
        **self == SJTAG_A::SJTAG_0
    }
    #[doc = "Checks if the value of the field is `SJTAG_1`"]
    #[inline(always)]
    pub fn is_sjtag_1(&self) -> bool {
        **self == SJTAG_A::SJTAG_1
    }
}
impl core::ops::Deref for SJTAG_R {
    type Target = crate::FieldReader<bool, SJTAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJTAG` writer - Sticky JTAG generated reset"]
pub struct SJTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SJTAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SJTAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline(always)]
    pub fn sjtag_0(self) -> &'a mut W {
        self.variant(SJTAG_A::SJTAG_0)
    }
    #[doc = "Reset caused by JTAG"]
    #[inline(always)]
    pub fn sjtag_1(self) -> &'a mut W {
        self.variant(SJTAG_A::SJTAG_1)
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
#[doc = "Sticky Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    SLOCKUP_0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    SLOCKUP_1 = 1,
}
impl From<SLOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: SLOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOCKUP` reader - Sticky Core Lockup"]
pub struct SLOCKUP_R(crate::FieldReader<bool, SLOCKUP_A>);
impl SLOCKUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLOCKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOCKUP_A {
        match self.bits {
            false => SLOCKUP_A::SLOCKUP_0,
            true => SLOCKUP_A::SLOCKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOCKUP_0`"]
    #[inline(always)]
    pub fn is_slockup_0(&self) -> bool {
        **self == SLOCKUP_A::SLOCKUP_0
    }
    #[doc = "Checks if the value of the field is `SLOCKUP_1`"]
    #[inline(always)]
    pub fn is_slockup_1(&self) -> bool {
        **self == SLOCKUP_A::SLOCKUP_1
    }
}
impl core::ops::Deref for SLOCKUP_R {
    type Target = crate::FieldReader<bool, SLOCKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOCKUP` writer - Sticky Core Lockup"]
pub struct SLOCKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOCKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn slockup_0(self) -> &'a mut W {
        self.variant(SLOCKUP_A::SLOCKUP_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn slockup_1(self) -> &'a mut W {
        self.variant(SLOCKUP_A::SLOCKUP_1)
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
#[doc = "Sticky Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    SSW_0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    SSW_1 = 1,
}
impl From<SSW_A> for bool {
    #[inline(always)]
    fn from(variant: SSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSW` reader - Sticky Software"]
pub struct SSW_R(crate::FieldReader<bool, SSW_A>);
impl SSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSW_A {
        match self.bits {
            false => SSW_A::SSW_0,
            true => SSW_A::SSW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSW_0`"]
    #[inline(always)]
    pub fn is_ssw_0(&self) -> bool {
        **self == SSW_A::SSW_0
    }
    #[doc = "Checks if the value of the field is `SSW_1`"]
    #[inline(always)]
    pub fn is_ssw_1(&self) -> bool {
        **self == SSW_A::SSW_1
    }
}
impl core::ops::Deref for SSW_R {
    type Target = crate::FieldReader<bool, SSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSW` writer - Sticky Software"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn ssw_0(self) -> &'a mut W {
        self.variant(SSW_A::SSW_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn ssw_1(self) -> &'a mut W {
        self.variant(SSW_A::SSW_1)
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
#[doc = "Sticky MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_AP_A {
    #[doc = "0: Reset was not caused by host debugger system setting of the System Reset Request bit"]
    SMDM_AP_0 = 0,
    #[doc = "1: Reset was caused by host debugger system setting of the System Reset Request bit"]
    SMDM_AP_1 = 1,
}
impl From<SMDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: SMDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMDM_AP` reader - Sticky MDM-AP System Reset Request"]
pub struct SMDM_AP_R(crate::FieldReader<bool, SMDM_AP_A>);
impl SMDM_AP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMDM_AP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMDM_AP_A {
        match self.bits {
            false => SMDM_AP_A::SMDM_AP_0,
            true => SMDM_AP_A::SMDM_AP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMDM_AP_0`"]
    #[inline(always)]
    pub fn is_smdm_ap_0(&self) -> bool {
        **self == SMDM_AP_A::SMDM_AP_0
    }
    #[doc = "Checks if the value of the field is `SMDM_AP_1`"]
    #[inline(always)]
    pub fn is_smdm_ap_1(&self) -> bool {
        **self == SMDM_AP_A::SMDM_AP_1
    }
}
impl core::ops::Deref for SMDM_AP_R {
    type Target = crate::FieldReader<bool, SMDM_AP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMDM_AP` writer - Sticky MDM-AP System Reset Request"]
pub struct SMDM_AP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDM_AP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMDM_AP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn smdm_ap_0(self) -> &'a mut W {
        self.variant(SMDM_AP_A::SMDM_AP_0)
    }
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn smdm_ap_1(self) -> &'a mut W {
        self.variant(SMDM_AP_A::SMDM_AP_1)
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
#[doc = "Sticky Stop Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    SSACKERR_0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    SSACKERR_1 = 1,
}
impl From<SSACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SSACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACKERR` reader - Sticky Stop Acknowledge Error"]
pub struct SSACKERR_R(crate::FieldReader<bool, SSACKERR_A>);
impl SSACKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSACKERR_A {
        match self.bits {
            false => SSACKERR_A::SSACKERR_0,
            true => SSACKERR_A::SSACKERR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSACKERR_0`"]
    #[inline(always)]
    pub fn is_ssackerr_0(&self) -> bool {
        **self == SSACKERR_A::SSACKERR_0
    }
    #[doc = "Checks if the value of the field is `SSACKERR_1`"]
    #[inline(always)]
    pub fn is_ssackerr_1(&self) -> bool {
        **self == SSACKERR_A::SSACKERR_1
    }
}
impl core::ops::Deref for SSACKERR_R {
    type Target = crate::FieldReader<bool, SSACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSACKERR` writer - Sticky Stop Acknowledge Error"]
pub struct SSACKERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSACKERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSACKERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn ssackerr_0(self) -> &'a mut W {
        self.variant(SSACKERR_A::SSACKERR_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn ssackerr_1(self) -> &'a mut W {
        self.variant(SSACKERR_A::SSACKERR_1)
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
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SLVD_R {
        SLVD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&self) -> SLOC_R {
        SLOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&self) -> SLOL_R {
        SLOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SWDOG_R {
        SWDOG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SPIN_R {
        SPIN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SPOR_R {
        SPOR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline(always)]
    pub fn sjtag(&self) -> SJTAG_R {
        SJTAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&self) -> SLOCKUP_R {
        SLOCKUP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&self) -> SMDM_AP_R {
        SMDM_AP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline(always)]
    pub fn ssackerr(&self) -> SSACKERR_R {
        SSACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&mut self) -> SLVD_W {
        SLVD_W { w: self }
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&mut self) -> SLOC_W {
        SLOC_W { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline(always)]
    pub fn slol(&mut self) -> SLOL_W {
        SLOL_W { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&mut self) -> SWDOG_W {
        SWDOG_W { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&mut self) -> SPIN_W {
        SPIN_W { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&mut self) -> SPOR_W {
        SPOR_W { w: self }
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline(always)]
    pub fn sjtag(&mut self) -> SJTAG_W {
        SJTAG_W { w: self }
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&mut self) -> SLOCKUP_W {
        SLOCKUP_W { w: self }
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&mut self) -> SMDM_AP_W {
        SMDM_AP_W { w: self }
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline(always)]
    pub fn ssackerr(&mut self) -> SSACKERR_W {
        SSACKERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sticky System Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrs](index.html) module"]
pub struct SSRS_SPEC;
impl crate::RegisterSpec for SSRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssrs::R](R) reader structure"]
impl crate::Readable for SSRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrs::W](W) writer structure"]
impl crate::Writable for SSRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSRS to value 0x82"]
impl crate::Resettable for SSRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x82
    }
}
