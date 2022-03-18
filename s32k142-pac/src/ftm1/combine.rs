#[doc = "Register `COMBINE` reader"]
pub struct R(crate::R<COMBINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMBINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMBINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMBINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMBINE` writer"]
pub struct W(crate::W<COMBINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBINE_SPEC>;
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
impl From<crate::W<COMBINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMBINE0` reader - Combine Channels For n = 0"]
pub struct COMBINE0_R(crate::FieldReader<bool, bool>);
impl COMBINE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMBINE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE0` writer - Combine Channels For n = 0"]
pub struct COMBINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE0_W<'a> {
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
#[doc = "Complement Of Channel (n) For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP0_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP0_A> for bool {
    #[inline(always)]
    fn from(variant: COMP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP0` reader - Complement Of Channel (n) For n = 0"]
pub struct COMP0_R(crate::FieldReader<bool, COMP0_A>);
impl COMP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP0_A {
        match self.bits {
            false => COMP0_A::_0,
            true => COMP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMP0_A::_1
    }
}
impl core::ops::Deref for COMP0_R {
    type Target = crate::FieldReader<bool, COMP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP0` writer - Complement Of Channel (n) For n = 0"]
pub struct COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP0_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP0_A::_1)
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
#[doc = "Field `DECAPEN0` reader - Dual Edge Capture Mode Enable For n = 0"]
pub struct DECAPEN0_R(crate::FieldReader<bool, bool>);
impl DECAPEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECAPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAPEN0` writer - Dual Edge Capture Mode Enable For n = 0"]
pub struct DECAPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN0_W<'a> {
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
#[doc = "Dual Edge Capture Mode Captures For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP0_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP0_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP0` reader - Dual Edge Capture Mode Captures For n = 0"]
pub struct DECAP0_R(crate::FieldReader<bool, DECAP0_A>);
impl DECAP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP0_A {
        match self.bits {
            false => DECAP0_A::_0,
            true => DECAP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DECAP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DECAP0_A::_1
    }
}
impl core::ops::Deref for DECAP0_R {
    type Target = crate::FieldReader<bool, DECAP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAP0` writer - Dual Edge Capture Mode Captures For n = 0"]
pub struct DECAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP0_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP0_A::_1)
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
#[doc = "Deadtime Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN0_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN0` reader - Deadtime Enable For n = 0"]
pub struct DTEN0_R(crate::FieldReader<bool, DTEN0_A>);
impl DTEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN0_A {
        match self.bits {
            false => DTEN0_A::_0,
            true => DTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEN0_A::_1
    }
}
impl core::ops::Deref for DTEN0_R {
    type Target = crate::FieldReader<bool, DTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN0` writer - Deadtime Enable For n = 0"]
pub struct DTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN0_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN0_A::_1)
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
#[doc = "Synchronization Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN0_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN0` reader - Synchronization Enable For n = 0"]
pub struct SYNCEN0_R(crate::FieldReader<bool, SYNCEN0_A>);
impl SYNCEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN0_A {
        match self.bits {
            false => SYNCEN0_A::_0,
            true => SYNCEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCEN0_A::_1
    }
}
impl core::ops::Deref for SYNCEN0_R {
    type Target = crate::FieldReader<bool, SYNCEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCEN0` writer - Synchronization Enable For n = 0"]
pub struct SYNCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN0_A::_1)
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
#[doc = "Fault Control Enable For n = 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN0_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTEN0` reader - Fault Control Enable For n = 0"]
pub struct FAULTEN0_R(crate::FieldReader<bool, FAULTEN0_A>);
impl FAULTEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN0_A {
        match self.bits {
            false => FAULTEN0_A::_0,
            true => FAULTEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTEN0_A::_1
    }
}
impl core::ops::Deref for FAULTEN0_R {
    type Target = crate::FieldReader<bool, FAULTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTEN0` writer - Fault Control Enable For n = 0"]
pub struct FAULTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN0_A::_1)
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
#[doc = "Field `MCOMBINE0` reader - Modified Combine Mode For n = 0"]
pub struct MCOMBINE0_R(crate::FieldReader<bool, bool>);
impl MCOMBINE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCOMBINE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOMBINE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCOMBINE0` writer - Modified Combine Mode For n = 0"]
pub struct MCOMBINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE0_W<'a> {
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
#[doc = "Field `COMBINE1` reader - Combine Channels For n = 2"]
pub struct COMBINE1_R(crate::FieldReader<bool, bool>);
impl COMBINE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMBINE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE1` writer - Combine Channels For n = 2"]
pub struct COMBINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE1_W<'a> {
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
#[doc = "Complement Of Channel (n) For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP1_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP1_A> for bool {
    #[inline(always)]
    fn from(variant: COMP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP1` reader - Complement Of Channel (n) For n = 2"]
pub struct COMP1_R(crate::FieldReader<bool, COMP1_A>);
impl COMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP1_A {
        match self.bits {
            false => COMP1_A::_0,
            true => COMP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMP1_A::_1
    }
}
impl core::ops::Deref for COMP1_R {
    type Target = crate::FieldReader<bool, COMP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1` writer - Complement Of Channel (n) For n = 2"]
pub struct COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP1_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP1_A::_1)
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
#[doc = "Field `DECAPEN1` reader - Dual Edge Capture Mode Enable For n = 2"]
pub struct DECAPEN1_R(crate::FieldReader<bool, bool>);
impl DECAPEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECAPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAPEN1` writer - Dual Edge Capture Mode Enable For n = 2"]
pub struct DECAPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN1_W<'a> {
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
#[doc = "Dual Edge Capture Mode Captures For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP1_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP1_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP1` reader - Dual Edge Capture Mode Captures For n = 2"]
pub struct DECAP1_R(crate::FieldReader<bool, DECAP1_A>);
impl DECAP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP1_A {
        match self.bits {
            false => DECAP1_A::_0,
            true => DECAP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DECAP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DECAP1_A::_1
    }
}
impl core::ops::Deref for DECAP1_R {
    type Target = crate::FieldReader<bool, DECAP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAP1` writer - Dual Edge Capture Mode Captures For n = 2"]
pub struct DECAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP1_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP1_A::_1)
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
#[doc = "Deadtime Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN1_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN1` reader - Deadtime Enable For n = 2"]
pub struct DTEN1_R(crate::FieldReader<bool, DTEN1_A>);
impl DTEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN1_A {
        match self.bits {
            false => DTEN1_A::_0,
            true => DTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEN1_A::_1
    }
}
impl core::ops::Deref for DTEN1_R {
    type Target = crate::FieldReader<bool, DTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN1` writer - Deadtime Enable For n = 2"]
pub struct DTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN1_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN1_A::_1)
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
#[doc = "Synchronization Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN1_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN1` reader - Synchronization Enable For n = 2"]
pub struct SYNCEN1_R(crate::FieldReader<bool, SYNCEN1_A>);
impl SYNCEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN1_A {
        match self.bits {
            false => SYNCEN1_A::_0,
            true => SYNCEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCEN1_A::_1
    }
}
impl core::ops::Deref for SYNCEN1_R {
    type Target = crate::FieldReader<bool, SYNCEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCEN1` writer - Synchronization Enable For n = 2"]
pub struct SYNCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN1_A::_1)
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
#[doc = "Fault Control Enable For n = 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN1_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTEN1` reader - Fault Control Enable For n = 2"]
pub struct FAULTEN1_R(crate::FieldReader<bool, FAULTEN1_A>);
impl FAULTEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN1_A {
        match self.bits {
            false => FAULTEN1_A::_0,
            true => FAULTEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTEN1_A::_1
    }
}
impl core::ops::Deref for FAULTEN1_R {
    type Target = crate::FieldReader<bool, FAULTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTEN1` writer - Fault Control Enable For n = 2"]
pub struct FAULTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN1_A::_1)
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
#[doc = "Field `MCOMBINE1` reader - Modified Combine Mode For n = 2"]
pub struct MCOMBINE1_R(crate::FieldReader<bool, bool>);
impl MCOMBINE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCOMBINE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOMBINE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCOMBINE1` writer - Modified Combine Mode For n = 2"]
pub struct MCOMBINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE1_W<'a> {
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
#[doc = "Field `COMBINE2` reader - Combine Channels For n = 4"]
pub struct COMBINE2_R(crate::FieldReader<bool, bool>);
impl COMBINE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMBINE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE2` writer - Combine Channels For n = 4"]
pub struct COMBINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE2_W<'a> {
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
#[doc = "Complement Of Channel (n) For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP2_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP2_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP2` reader - Complement Of Channel (n) For n = 4"]
pub struct COMP2_R(crate::FieldReader<bool, COMP2_A>);
impl COMP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2_A {
        match self.bits {
            false => COMP2_A::_0,
            true => COMP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMP2_A::_1
    }
}
impl core::ops::Deref for COMP2_R {
    type Target = crate::FieldReader<bool, COMP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2` writer - Complement Of Channel (n) For n = 4"]
pub struct COMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP2_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP2_A::_1)
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
#[doc = "Field `DECAPEN2` reader - Dual Edge Capture Mode Enable For n = 4"]
pub struct DECAPEN2_R(crate::FieldReader<bool, bool>);
impl DECAPEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECAPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAPEN2` writer - Dual Edge Capture Mode Enable For n = 4"]
pub struct DECAPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN2_W<'a> {
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
#[doc = "Dual Edge Capture Mode Captures For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP2_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP2_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP2` reader - Dual Edge Capture Mode Captures For n = 4"]
pub struct DECAP2_R(crate::FieldReader<bool, DECAP2_A>);
impl DECAP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP2_A {
        match self.bits {
            false => DECAP2_A::_0,
            true => DECAP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DECAP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DECAP2_A::_1
    }
}
impl core::ops::Deref for DECAP2_R {
    type Target = crate::FieldReader<bool, DECAP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAP2` writer - Dual Edge Capture Mode Captures For n = 4"]
pub struct DECAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP2_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Deadtime Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN2_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN2` reader - Deadtime Enable For n = 4"]
pub struct DTEN2_R(crate::FieldReader<bool, DTEN2_A>);
impl DTEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN2_A {
        match self.bits {
            false => DTEN2_A::_0,
            true => DTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEN2_A::_1
    }
}
impl core::ops::Deref for DTEN2_R {
    type Target = crate::FieldReader<bool, DTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN2` writer - Deadtime Enable For n = 4"]
pub struct DTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN2_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Synchronization Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN2_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN2_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN2` reader - Synchronization Enable For n = 4"]
pub struct SYNCEN2_R(crate::FieldReader<bool, SYNCEN2_A>);
impl SYNCEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN2_A {
        match self.bits {
            false => SYNCEN2_A::_0,
            true => SYNCEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCEN2_A::_1
    }
}
impl core::ops::Deref for SYNCEN2_R {
    type Target = crate::FieldReader<bool, SYNCEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCEN2` writer - Synchronization Enable For n = 4"]
pub struct SYNCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Fault Control Enable For n = 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN2_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTEN2` reader - Fault Control Enable For n = 4"]
pub struct FAULTEN2_R(crate::FieldReader<bool, FAULTEN2_A>);
impl FAULTEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN2_A {
        match self.bits {
            false => FAULTEN2_A::_0,
            true => FAULTEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTEN2_A::_1
    }
}
impl core::ops::Deref for FAULTEN2_R {
    type Target = crate::FieldReader<bool, FAULTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTEN2` writer - Fault Control Enable For n = 4"]
pub struct FAULTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN2_A::_1)
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
#[doc = "Field `MCOMBINE2` reader - Modified Combine Mode For n = 4"]
pub struct MCOMBINE2_R(crate::FieldReader<bool, bool>);
impl MCOMBINE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCOMBINE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOMBINE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCOMBINE2` writer - Modified Combine Mode For n = 4"]
pub struct MCOMBINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE2_W<'a> {
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
#[doc = "Field `COMBINE3` reader - Combine Channels For n = 6"]
pub struct COMBINE3_R(crate::FieldReader<bool, bool>);
impl COMBINE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMBINE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMBINE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMBINE3` writer - Combine Channels For n = 6"]
pub struct COMBINE3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBINE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Complement Of Channel (n) for n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMP3_A {
    #[doc = "0: The channel (n+1) output is the same as the channel (n) output."]
    _0 = 0,
    #[doc = "1: The channel (n+1) output is the complement of the channel (n) output."]
    _1 = 1,
}
impl From<COMP3_A> for bool {
    #[inline(always)]
    fn from(variant: COMP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMP3` reader - Complement Of Channel (n) for n = 6"]
pub struct COMP3_R(crate::FieldReader<bool, COMP3_A>);
impl COMP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP3_A {
        match self.bits {
            false => COMP3_A::_0,
            true => COMP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == COMP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COMP3_A::_1
    }
}
impl core::ops::Deref for COMP3_R {
    type Target = crate::FieldReader<bool, COMP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP3` writer - Complement Of Channel (n) for n = 6"]
pub struct COMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel (n+1) output is the same as the channel (n) output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COMP3_A::_0)
    }
    #[doc = "The channel (n+1) output is the complement of the channel (n) output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COMP3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `DECAPEN3` reader - Dual Edge Capture Mode Enable For n = 6"]
pub struct DECAPEN3_R(crate::FieldReader<bool, bool>);
impl DECAPEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DECAPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAPEN3` writer - Dual Edge Capture Mode Enable For n = 6"]
pub struct DECAPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAPEN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Dual Edge Capture Mode Captures For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DECAP3_A {
    #[doc = "0: The dual edge captures are inactive."]
    _0 = 0,
    #[doc = "1: The dual edge captures are active."]
    _1 = 1,
}
impl From<DECAP3_A> for bool {
    #[inline(always)]
    fn from(variant: DECAP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECAP3` reader - Dual Edge Capture Mode Captures For n = 6"]
pub struct DECAP3_R(crate::FieldReader<bool, DECAP3_A>);
impl DECAP3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DECAP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DECAP3_A {
        match self.bits {
            false => DECAP3_A::_0,
            true => DECAP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DECAP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DECAP3_A::_1
    }
}
impl core::ops::Deref for DECAP3_R {
    type Target = crate::FieldReader<bool, DECAP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DECAP3` writer - Dual Edge Capture Mode Captures For n = 6"]
pub struct DECAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> DECAP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DECAP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The dual edge captures are inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DECAP3_A::_0)
    }
    #[doc = "The dual edge captures are active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DECAP3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Deadtime Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN3_A {
    #[doc = "0: The deadtime insertion in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The deadtime insertion in this pair of channels is enabled."]
    _1 = 1,
}
impl From<DTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEN3` reader - Deadtime Enable For n = 6"]
pub struct DTEN3_R(crate::FieldReader<bool, DTEN3_A>);
impl DTEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN3_A {
        match self.bits {
            false => DTEN3_A::_0,
            true => DTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEN3_A::_1
    }
}
impl core::ops::Deref for DTEN3_R {
    type Target = crate::FieldReader<bool, DTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN3` writer - Deadtime Enable For n = 6"]
pub struct DTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The deadtime insertion in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTEN3_A::_0)
    }
    #[doc = "The deadtime insertion in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Synchronization Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEN3_A {
    #[doc = "0: The PWM synchronization in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The PWM synchronization in this pair of channels is enabled."]
    _1 = 1,
}
impl From<SYNCEN3_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCEN3` reader - Synchronization Enable For n = 6"]
pub struct SYNCEN3_R(crate::FieldReader<bool, SYNCEN3_A>);
impl SYNCEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN3_A {
        match self.bits {
            false => SYNCEN3_A::_0,
            true => SYNCEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SYNCEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SYNCEN3_A::_1
    }
}
impl core::ops::Deref for SYNCEN3_R {
    type Target = crate::FieldReader<bool, SYNCEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCEN3` writer - Synchronization Enable For n = 6"]
pub struct SYNCEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM synchronization in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_0)
    }
    #[doc = "The PWM synchronization in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Fault Control Enable For n = 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTEN3_A {
    #[doc = "0: The fault control in this pair of channels is disabled."]
    _0 = 0,
    #[doc = "1: The fault control in this pair of channels is enabled."]
    _1 = 1,
}
impl From<FAULTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAULTEN3` reader - Fault Control Enable For n = 6"]
pub struct FAULTEN3_R(crate::FieldReader<bool, FAULTEN3_A>);
impl FAULTEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAULTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTEN3_A {
        match self.bits {
            false => FAULTEN3_A::_0,
            true => FAULTEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FAULTEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FAULTEN3_A::_1
    }
}
impl core::ops::Deref for FAULTEN3_R {
    type Target = crate::FieldReader<bool, FAULTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULTEN3` writer - Fault Control Enable For n = 6"]
pub struct FAULTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAULTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The fault control in this pair of channels is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_0)
    }
    #[doc = "The fault control in this pair of channels is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `MCOMBINE3` reader - Modified Combine Mode For n = 6"]
pub struct MCOMBINE3_R(crate::FieldReader<bool, bool>);
impl MCOMBINE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCOMBINE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCOMBINE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCOMBINE3` writer - Modified Combine Mode For n = 6"]
pub struct MCOMBINE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOMBINE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    pub fn combine0(&self) -> COMBINE0_R {
        COMBINE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    pub fn decapen0(&self) -> DECAPEN0_R {
        DECAPEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    pub fn decap0(&self) -> DECAP0_R {
        DECAP0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    pub fn dten0(&self) -> DTEN0_R {
        DTEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    pub fn syncen0(&self) -> SYNCEN0_R {
        SYNCEN0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    pub fn faulten0(&self) -> FAULTEN0_R {
        FAULTEN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline(always)]
    pub fn mcombine0(&self) -> MCOMBINE0_R {
        MCOMBINE0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    pub fn combine1(&self) -> COMBINE1_R {
        COMBINE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    pub fn decapen1(&self) -> DECAPEN1_R {
        DECAPEN1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    pub fn decap1(&self) -> DECAP1_R {
        DECAP1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    pub fn dten1(&self) -> DTEN1_R {
        DTEN1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    pub fn syncen1(&self) -> SYNCEN1_R {
        SYNCEN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    pub fn faulten1(&self) -> FAULTEN1_R {
        FAULTEN1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline(always)]
    pub fn mcombine1(&self) -> MCOMBINE1_R {
        MCOMBINE1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    pub fn combine2(&self) -> COMBINE2_R {
        COMBINE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    pub fn decapen2(&self) -> DECAPEN2_R {
        DECAPEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    pub fn decap2(&self) -> DECAP2_R {
        DECAP2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    pub fn dten2(&self) -> DTEN2_R {
        DTEN2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    pub fn syncen2(&self) -> SYNCEN2_R {
        SYNCEN2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    pub fn faulten2(&self) -> FAULTEN2_R {
        FAULTEN2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline(always)]
    pub fn mcombine2(&self) -> MCOMBINE2_R {
        MCOMBINE2_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    pub fn combine3(&self) -> COMBINE3_R {
        COMBINE3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    pub fn comp3(&self) -> COMP3_R {
        COMP3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    pub fn decapen3(&self) -> DECAPEN3_R {
        DECAPEN3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    pub fn decap3(&self) -> DECAP3_R {
        DECAP3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    pub fn dten3(&self) -> DTEN3_R {
        DTEN3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    pub fn syncen3(&self) -> SYNCEN3_R {
        SYNCEN3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    pub fn faulten3(&self) -> FAULTEN3_R {
        FAULTEN3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline(always)]
    pub fn mcombine3(&self) -> MCOMBINE3_R {
        MCOMBINE3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Combine Channels For n = 0"]
    #[inline(always)]
    pub fn combine0(&mut self) -> COMBINE0_W {
        COMBINE0_W { w: self }
    }
    #[doc = "Bit 1 - Complement Of Channel (n) For n = 0"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W { w: self }
    }
    #[doc = "Bit 2 - Dual Edge Capture Mode Enable For n = 0"]
    #[inline(always)]
    pub fn decapen0(&mut self) -> DECAPEN0_W {
        DECAPEN0_W { w: self }
    }
    #[doc = "Bit 3 - Dual Edge Capture Mode Captures For n = 0"]
    #[inline(always)]
    pub fn decap0(&mut self) -> DECAP0_W {
        DECAP0_W { w: self }
    }
    #[doc = "Bit 4 - Deadtime Enable For n = 0"]
    #[inline(always)]
    pub fn dten0(&mut self) -> DTEN0_W {
        DTEN0_W { w: self }
    }
    #[doc = "Bit 5 - Synchronization Enable For n = 0"]
    #[inline(always)]
    pub fn syncen0(&mut self) -> SYNCEN0_W {
        SYNCEN0_W { w: self }
    }
    #[doc = "Bit 6 - Fault Control Enable For n = 0"]
    #[inline(always)]
    pub fn faulten0(&mut self) -> FAULTEN0_W {
        FAULTEN0_W { w: self }
    }
    #[doc = "Bit 7 - Modified Combine Mode For n = 0"]
    #[inline(always)]
    pub fn mcombine0(&mut self) -> MCOMBINE0_W {
        MCOMBINE0_W { w: self }
    }
    #[doc = "Bit 8 - Combine Channels For n = 2"]
    #[inline(always)]
    pub fn combine1(&mut self) -> COMBINE1_W {
        COMBINE1_W { w: self }
    }
    #[doc = "Bit 9 - Complement Of Channel (n) For n = 2"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W { w: self }
    }
    #[doc = "Bit 10 - Dual Edge Capture Mode Enable For n = 2"]
    #[inline(always)]
    pub fn decapen1(&mut self) -> DECAPEN1_W {
        DECAPEN1_W { w: self }
    }
    #[doc = "Bit 11 - Dual Edge Capture Mode Captures For n = 2"]
    #[inline(always)]
    pub fn decap1(&mut self) -> DECAP1_W {
        DECAP1_W { w: self }
    }
    #[doc = "Bit 12 - Deadtime Enable For n = 2"]
    #[inline(always)]
    pub fn dten1(&mut self) -> DTEN1_W {
        DTEN1_W { w: self }
    }
    #[doc = "Bit 13 - Synchronization Enable For n = 2"]
    #[inline(always)]
    pub fn syncen1(&mut self) -> SYNCEN1_W {
        SYNCEN1_W { w: self }
    }
    #[doc = "Bit 14 - Fault Control Enable For n = 2"]
    #[inline(always)]
    pub fn faulten1(&mut self) -> FAULTEN1_W {
        FAULTEN1_W { w: self }
    }
    #[doc = "Bit 15 - Modified Combine Mode For n = 2"]
    #[inline(always)]
    pub fn mcombine1(&mut self) -> MCOMBINE1_W {
        MCOMBINE1_W { w: self }
    }
    #[doc = "Bit 16 - Combine Channels For n = 4"]
    #[inline(always)]
    pub fn combine2(&mut self) -> COMBINE2_W {
        COMBINE2_W { w: self }
    }
    #[doc = "Bit 17 - Complement Of Channel (n) For n = 4"]
    #[inline(always)]
    pub fn comp2(&mut self) -> COMP2_W {
        COMP2_W { w: self }
    }
    #[doc = "Bit 18 - Dual Edge Capture Mode Enable For n = 4"]
    #[inline(always)]
    pub fn decapen2(&mut self) -> DECAPEN2_W {
        DECAPEN2_W { w: self }
    }
    #[doc = "Bit 19 - Dual Edge Capture Mode Captures For n = 4"]
    #[inline(always)]
    pub fn decap2(&mut self) -> DECAP2_W {
        DECAP2_W { w: self }
    }
    #[doc = "Bit 20 - Deadtime Enable For n = 4"]
    #[inline(always)]
    pub fn dten2(&mut self) -> DTEN2_W {
        DTEN2_W { w: self }
    }
    #[doc = "Bit 21 - Synchronization Enable For n = 4"]
    #[inline(always)]
    pub fn syncen2(&mut self) -> SYNCEN2_W {
        SYNCEN2_W { w: self }
    }
    #[doc = "Bit 22 - Fault Control Enable For n = 4"]
    #[inline(always)]
    pub fn faulten2(&mut self) -> FAULTEN2_W {
        FAULTEN2_W { w: self }
    }
    #[doc = "Bit 23 - Modified Combine Mode For n = 4"]
    #[inline(always)]
    pub fn mcombine2(&mut self) -> MCOMBINE2_W {
        MCOMBINE2_W { w: self }
    }
    #[doc = "Bit 24 - Combine Channels For n = 6"]
    #[inline(always)]
    pub fn combine3(&mut self) -> COMBINE3_W {
        COMBINE3_W { w: self }
    }
    #[doc = "Bit 25 - Complement Of Channel (n) for n = 6"]
    #[inline(always)]
    pub fn comp3(&mut self) -> COMP3_W {
        COMP3_W { w: self }
    }
    #[doc = "Bit 26 - Dual Edge Capture Mode Enable For n = 6"]
    #[inline(always)]
    pub fn decapen3(&mut self) -> DECAPEN3_W {
        DECAPEN3_W { w: self }
    }
    #[doc = "Bit 27 - Dual Edge Capture Mode Captures For n = 6"]
    #[inline(always)]
    pub fn decap3(&mut self) -> DECAP3_W {
        DECAP3_W { w: self }
    }
    #[doc = "Bit 28 - Deadtime Enable For n = 6"]
    #[inline(always)]
    pub fn dten3(&mut self) -> DTEN3_W {
        DTEN3_W { w: self }
    }
    #[doc = "Bit 29 - Synchronization Enable For n = 6"]
    #[inline(always)]
    pub fn syncen3(&mut self) -> SYNCEN3_W {
        SYNCEN3_W { w: self }
    }
    #[doc = "Bit 30 - Fault Control Enable For n = 6"]
    #[inline(always)]
    pub fn faulten3(&mut self) -> FAULTEN3_W {
        FAULTEN3_W { w: self }
    }
    #[doc = "Bit 31 - Modified Combine Mode For n = 6"]
    #[inline(always)]
    pub fn mcombine3(&mut self) -> MCOMBINE3_W {
        MCOMBINE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function For Linked Channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combine](index.html) module"]
pub struct COMBINE_SPEC;
impl crate::RegisterSpec for COMBINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [combine::R](R) reader structure"]
impl crate::Readable for COMBINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [combine::W](W) writer structure"]
impl crate::Writable for COMBINE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMBINE to value 0"]
impl crate::Resettable for COMBINE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
