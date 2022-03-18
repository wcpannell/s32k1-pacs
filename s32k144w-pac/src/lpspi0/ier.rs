#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIE` reader - Transmit Data Interrupt Enable"]
pub struct TDIE_R(crate::FieldReader<bool, TDIE_A>);
impl TDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::DISABLED,
            true => TDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDIE_A::ENABLED
    }
}
impl core::ops::Deref for TDIE_R {
    type Target = crate::FieldReader<bool, TDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIE` writer - Transmit Data Interrupt Enable"]
pub struct TDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDIE_A::ENABLED)
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
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIE` reader - Receive Data Interrupt Enable"]
pub struct RDIE_R(crate::FieldReader<bool, RDIE_A>);
impl RDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::DISABLED,
            true => RDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RDIE_A::ENABLED
    }
}
impl core::ops::Deref for RDIE_R {
    type Target = crate::FieldReader<bool, RDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIE` writer - Receive Data Interrupt Enable"]
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDIE_A::ENABLED)
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
#[doc = "Word Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WCIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCIE` reader - Word Complete Interrupt Enable"]
pub struct WCIE_R(crate::FieldReader<bool, WCIE_A>);
impl WCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCIE_A {
        match self.bits {
            false => WCIE_A::DISABLED,
            true => WCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WCIE_A::ENABLED
    }
}
impl core::ops::Deref for WCIE_R {
    type Target = crate::FieldReader<bool, WCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCIE` writer - Word Complete Interrupt Enable"]
pub struct WCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WCIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WCIE_A::ENABLED)
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
#[doc = "Frame Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<FCIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCIE` reader - Frame Complete Interrupt Enable"]
pub struct FCIE_R(crate::FieldReader<bool, FCIE_A>);
impl FCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCIE_A {
        match self.bits {
            false => FCIE_A::DISABLED,
            true => FCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FCIE_A::ENABLED
    }
}
impl core::ops::Deref for FCIE_R {
    type Target = crate::FieldReader<bool, FCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCIE` writer - Frame Complete Interrupt Enable"]
pub struct FCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FCIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FCIE_A::ENABLED)
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
#[doc = "Transfer Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer Complete Interrupt Enable"]
pub struct TCIE_R(crate::FieldReader<bool, TCIE_A>);
impl TCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TCIE_A::ENABLED
    }
}
impl core::ops::Deref for TCIE_R {
    type Target = crate::FieldReader<bool, TCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIE` writer - Transfer Complete Interrupt Enable"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
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
#[doc = "Transmit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transmit Error Interrupt Enable"]
pub struct TEIE_R(crate::FieldReader<bool, TEIE_A>);
impl TEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::DISABLED,
            true => TEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TEIE_A::ENABLED
    }
}
impl core::ops::Deref for TEIE_R {
    type Target = crate::FieldReader<bool, TEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIE` writer - Transmit Error Interrupt Enable"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::ENABLED)
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
#[doc = "Receive Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REIE` reader - Receive Error Interrupt Enable"]
pub struct REIE_R(crate::FieldReader<bool, REIE_A>);
impl REIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::DISABLED,
            true => REIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REIE_A::ENABLED
    }
}
impl core::ops::Deref for REIE_R {
    type Target = crate::FieldReader<bool, REIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REIE` writer - Receive Error Interrupt Enable"]
pub struct REIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REIE_A::ENABLED)
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
#[doc = "Data Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIE` reader - Data Match Interrupt Enable"]
pub struct DMIE_R(crate::FieldReader<bool, DMIE_A>);
impl DMIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIE_A {
        match self.bits {
            false => DMIE_A::DISABLED,
            true => DMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DMIE_A::ENABLED
    }
}
impl core::ops::Deref for DMIE_R {
    type Target = crate::FieldReader<bool, DMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIE` writer - Data Match Interrupt Enable"]
pub struct DMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMIE_A::ENABLED)
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
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline(always)]
    pub fn wcie(&self) -> WCIE_R {
        WCIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fcie(&self) -> FCIE_R {
        FCIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&self) -> DMIE_R {
        DMIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&mut self) -> TDIE_W {
        TDIE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    #[doc = "Bit 8 - Word Complete Interrupt Enable"]
    #[inline(always)]
    pub fn wcie(&mut self) -> WCIE_W {
        WCIE_W { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fcie(&mut self) -> FCIE_W {
        FCIE_W { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 12 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W {
        REIE_W { w: self }
    }
    #[doc = "Bit 13 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&mut self) -> DMIE_W {
        DMIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
