#[doc = "Register `FLTCTRL` reader"]
pub struct R(crate::R<FLTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLTCTRL` writer"]
pub struct W(crate::W<FLTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTCTRL_SPEC>;
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
impl From<crate::W<FLTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fault Input 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT0EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT0EN` reader - Fault Input 0 Enable"]
pub struct FAULT0EN_R(crate::FieldReader<bool, FAULT0EN_A>);
impl FAULT0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT0EN_A {
        match self.bits {
            false => FAULT0EN_A::_0,
            true => FAULT0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULT0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULT0EN_A::_1
    }
}
impl core::ops::Deref for FAULT0EN_R {
    type Target = crate::FieldReader<bool, FAULT0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT0EN` writer - Fault Input 0 Enable"]
pub struct FAULT0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT0EN_A::_1)
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
#[doc = "Fault Input 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT1EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT1EN` reader - Fault Input 1 Enable"]
pub struct FAULT1EN_R(crate::FieldReader<bool, FAULT1EN_A>);
impl FAULT1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT1EN_A {
        match self.bits {
            false => FAULT1EN_A::_0,
            true => FAULT1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULT1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULT1EN_A::_1
    }
}
impl core::ops::Deref for FAULT1EN_R {
    type Target = crate::FieldReader<bool, FAULT1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT1EN` writer - Fault Input 1 Enable"]
pub struct FAULT1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT1EN_A::_1)
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
#[doc = "Fault Input 2 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT2EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT2EN` reader - Fault Input 2 Enable"]
pub struct FAULT2EN_R(crate::FieldReader<bool, FAULT2EN_A>);
impl FAULT2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT2EN_A {
        match self.bits {
            false => FAULT2EN_A::_0,
            true => FAULT2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULT2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULT2EN_A::_1
    }
}
impl core::ops::Deref for FAULT2EN_R {
    type Target = crate::FieldReader<bool, FAULT2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT2EN` writer - Fault Input 2 Enable"]
pub struct FAULT2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT2EN_A::_1)
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
#[doc = "Fault Input 3 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULT3EN_A {
    #[doc = "0: Fault input is disabled."]
    _0 = 0,
    #[doc = "1: Fault input is enabled."]
    _1 = 1,
}
impl From<FAULT3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULT3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULT3EN` reader - Fault Input 3 Enable"]
pub struct FAULT3EN_R(crate::FieldReader<bool, FAULT3EN_A>);
impl FAULT3EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULT3EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULT3EN_A {
        match self.bits {
            false => FAULT3EN_A::_0,
            true => FAULT3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULT3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULT3EN_A::_1
    }
}
impl core::ops::Deref for FAULT3EN_R {
    type Target = crate::FieldReader<bool, FAULT3EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT3EN` writer - Fault Input 3 Enable"]
pub struct FAULT3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULT3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_0)
    }
    #[doc = "Fault input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULT3EN_A::_1)
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
#[doc = "Fault Input 0 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR0EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR0EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR0EN` reader - Fault Input 0 Filter Enable"]
pub struct FFLTR0EN_R(crate::FieldReader<bool, FFLTR0EN_A>);
impl FFLTR0EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFLTR0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR0EN_A {
        match self.bits {
            false => FFLTR0EN_A::_0,
            true => FFLTR0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FFLTR0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FFLTR0EN_A::_1
    }
}
impl core::ops::Deref for FFLTR0EN_R {
    type Target = crate::FieldReader<bool, FFLTR0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLTR0EN` writer - Fault Input 0 Filter Enable"]
pub struct FFLTR0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR0EN_A::_1)
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
#[doc = "Fault Input 1 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR1EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR1EN` reader - Fault Input 1 Filter Enable"]
pub struct FFLTR1EN_R(crate::FieldReader<bool, FFLTR1EN_A>);
impl FFLTR1EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFLTR1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR1EN_A {
        match self.bits {
            false => FFLTR1EN_A::_0,
            true => FFLTR1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FFLTR1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FFLTR1EN_A::_1
    }
}
impl core::ops::Deref for FFLTR1EN_R {
    type Target = crate::FieldReader<bool, FFLTR1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLTR1EN` writer - Fault Input 1 Filter Enable"]
pub struct FFLTR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR1EN_A::_1)
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
#[doc = "Fault Input 2 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR2EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR2EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR2EN` reader - Fault Input 2 Filter Enable"]
pub struct FFLTR2EN_R(crate::FieldReader<bool, FFLTR2EN_A>);
impl FFLTR2EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFLTR2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR2EN_A {
        match self.bits {
            false => FFLTR2EN_A::_0,
            true => FFLTR2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FFLTR2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FFLTR2EN_A::_1
    }
}
impl core::ops::Deref for FFLTR2EN_R {
    type Target = crate::FieldReader<bool, FFLTR2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLTR2EN` writer - Fault Input 2 Filter Enable"]
pub struct FFLTR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR2EN_A::_1)
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
#[doc = "Fault Input 3 Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLTR3EN_A {
    #[doc = "0: Fault input filter is disabled."]
    _0 = 0,
    #[doc = "1: Fault input filter is enabled."]
    _1 = 1,
}
impl From<FFLTR3EN_A> for bool {
    #[inline(always)]
    fn from(variant: FFLTR3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFLTR3EN` reader - Fault Input 3 Filter Enable"]
pub struct FFLTR3EN_R(crate::FieldReader<bool, FFLTR3EN_A>);
impl FFLTR3EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FFLTR3EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLTR3EN_A {
        match self.bits {
            false => FFLTR3EN_A::_0,
            true => FFLTR3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FFLTR3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FFLTR3EN_A::_1
    }
}
impl core::ops::Deref for FFLTR3EN_R {
    type Target = crate::FieldReader<bool, FFLTR3EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFLTR3EN` writer - Fault Input 3 Filter Enable"]
pub struct FFLTR3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLTR3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLTR3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fault input filter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_0)
    }
    #[doc = "Fault input filter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FFLTR3EN_A::_1)
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
#[doc = "Field `FFVAL` reader - Fault Input Filter"]
pub struct FFVAL_R(crate::FieldReader<u8, u8>);
impl FFVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FFVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFVAL` writer - Fault Input Filter"]
pub struct FFVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Fault output state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSTATE_A {
    #[doc = "0: FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    _0 = 0,
    #[doc = "1: FTM outputs will be tri-stated when fault event is ongoing"]
    _1 = 1,
}
impl From<FSTATE_A> for bool {
    #[inline(always)]
    fn from(variant: FSTATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSTATE` reader - Fault output state"]
pub struct FSTATE_R(crate::FieldReader<bool, FSTATE_A>);
impl FSTATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSTATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSTATE_A {
        match self.bits {
            false => FSTATE_A::_0,
            true => FSTATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSTATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSTATE_A::_1
    }
}
impl core::ops::Deref for FSTATE_R {
    type Target = crate::FieldReader<bool, FSTATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSTATE` writer - Fault output state"]
pub struct FSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSTATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM outputs will be placed into safe values when fault events in ongoing (defined by POL bits)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSTATE_A::_0)
    }
    #[doc = "FTM outputs will be tri-stated when fault event is ongoing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSTATE_A::_1)
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
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&self) -> FAULT0EN_R {
        FAULT0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&self) -> FAULT1EN_R {
        FAULT1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&self) -> FAULT2EN_R {
        FAULT2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&self) -> FAULT3EN_R {
        FAULT3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&self) -> FFLTR0EN_R {
        FFLTR0EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&self) -> FFLTR1EN_R {
        FFLTR1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&self) -> FFLTR2EN_R {
        FFLTR2EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&self) -> FFLTR3EN_R {
        FFLTR3EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&self) -> FFVAL_R {
        FFVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline(always)]
    pub fn fstate(&self) -> FSTATE_R {
        FSTATE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input 0 Enable"]
    #[inline(always)]
    pub fn fault0en(&mut self) -> FAULT0EN_W {
        FAULT0EN_W { w: self }
    }
    #[doc = "Bit 1 - Fault Input 1 Enable"]
    #[inline(always)]
    pub fn fault1en(&mut self) -> FAULT1EN_W {
        FAULT1EN_W { w: self }
    }
    #[doc = "Bit 2 - Fault Input 2 Enable"]
    #[inline(always)]
    pub fn fault2en(&mut self) -> FAULT2EN_W {
        FAULT2EN_W { w: self }
    }
    #[doc = "Bit 3 - Fault Input 3 Enable"]
    #[inline(always)]
    pub fn fault3en(&mut self) -> FAULT3EN_W {
        FAULT3EN_W { w: self }
    }
    #[doc = "Bit 4 - Fault Input 0 Filter Enable"]
    #[inline(always)]
    pub fn ffltr0en(&mut self) -> FFLTR0EN_W {
        FFLTR0EN_W { w: self }
    }
    #[doc = "Bit 5 - Fault Input 1 Filter Enable"]
    #[inline(always)]
    pub fn ffltr1en(&mut self) -> FFLTR1EN_W {
        FFLTR1EN_W { w: self }
    }
    #[doc = "Bit 6 - Fault Input 2 Filter Enable"]
    #[inline(always)]
    pub fn ffltr2en(&mut self) -> FFLTR2EN_W {
        FFLTR2EN_W { w: self }
    }
    #[doc = "Bit 7 - Fault Input 3 Filter Enable"]
    #[inline(always)]
    pub fn ffltr3en(&mut self) -> FFLTR3EN_W {
        FFLTR3EN_W { w: self }
    }
    #[doc = "Bits 8:11 - Fault Input Filter"]
    #[inline(always)]
    pub fn ffval(&mut self) -> FFVAL_W {
        FFVAL_W { w: self }
    }
    #[doc = "Bit 15 - Fault output state"]
    #[inline(always)]
    pub fn fstate(&mut self) -> FSTATE_W {
        FSTATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltctrl](index.html) module"]
pub struct FLTCTRL_SPEC;
impl crate::RegisterSpec for FLTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fltctrl::R](R) reader structure"]
impl crate::Readable for FLTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fltctrl::W](W) writer structure"]
impl crate::Writable for FLTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLTCTRL to value 0"]
impl crate::Resettable for FLTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
