#[doc = "Register `OUTINIT` reader"]
pub struct R(crate::R<OUTINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTINIT` writer"]
pub struct W(crate::W<OUTINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTINIT_SPEC>;
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
impl From<crate::W<OUTINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OI_A {
    #[doc = "0: The initialization value is 0."]
    CH0OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH0OI_1 = 1,
}
impl From<CH0OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OI` reader - Channel 0 Output Initialization Value"]
pub struct CH0OI_R(crate::FieldReader<bool, CH0OI_A>);
impl CH0OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OI_A {
        match self.bits {
            false => CH0OI_A::CH0OI_0,
            true => CH0OI_A::CH0OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH0OI_0`"]
    #[inline(always)]
    pub fn is_ch0oi_0(&self) -> bool {
        **self == CH0OI_A::CH0OI_0
    }
    #[doc = "Checks if the value of the field is `CH0OI_1`"]
    #[inline(always)]
    pub fn is_ch0oi_1(&self) -> bool {
        **self == CH0OI_A::CH0OI_1
    }
}
impl core::ops::Deref for CH0OI_R {
    type Target = crate::FieldReader<bool, CH0OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0OI` writer - Channel 0 Output Initialization Value"]
pub struct CH0OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch0oi_0(self) -> &'a mut W {
        self.variant(CH0OI_A::CH0OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch0oi_1(self) -> &'a mut W {
        self.variant(CH0OI_A::CH0OI_1)
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
#[doc = "Channel 1 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OI_A {
    #[doc = "0: The initialization value is 0."]
    CH1OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH1OI_1 = 1,
}
impl From<CH1OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OI` reader - Channel 1 Output Initialization Value"]
pub struct CH1OI_R(crate::FieldReader<bool, CH1OI_A>);
impl CH1OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OI_A {
        match self.bits {
            false => CH1OI_A::CH1OI_0,
            true => CH1OI_A::CH1OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH1OI_0`"]
    #[inline(always)]
    pub fn is_ch1oi_0(&self) -> bool {
        **self == CH1OI_A::CH1OI_0
    }
    #[doc = "Checks if the value of the field is `CH1OI_1`"]
    #[inline(always)]
    pub fn is_ch1oi_1(&self) -> bool {
        **self == CH1OI_A::CH1OI_1
    }
}
impl core::ops::Deref for CH1OI_R {
    type Target = crate::FieldReader<bool, CH1OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OI` writer - Channel 1 Output Initialization Value"]
pub struct CH1OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch1oi_0(self) -> &'a mut W {
        self.variant(CH1OI_A::CH1OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch1oi_1(self) -> &'a mut W {
        self.variant(CH1OI_A::CH1OI_1)
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
#[doc = "Channel 2 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OI_A {
    #[doc = "0: The initialization value is 0."]
    CH2OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH2OI_1 = 1,
}
impl From<CH2OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OI` reader - Channel 2 Output Initialization Value"]
pub struct CH2OI_R(crate::FieldReader<bool, CH2OI_A>);
impl CH2OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OI_A {
        match self.bits {
            false => CH2OI_A::CH2OI_0,
            true => CH2OI_A::CH2OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH2OI_0`"]
    #[inline(always)]
    pub fn is_ch2oi_0(&self) -> bool {
        **self == CH2OI_A::CH2OI_0
    }
    #[doc = "Checks if the value of the field is `CH2OI_1`"]
    #[inline(always)]
    pub fn is_ch2oi_1(&self) -> bool {
        **self == CH2OI_A::CH2OI_1
    }
}
impl core::ops::Deref for CH2OI_R {
    type Target = crate::FieldReader<bool, CH2OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2OI` writer - Channel 2 Output Initialization Value"]
pub struct CH2OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch2oi_0(self) -> &'a mut W {
        self.variant(CH2OI_A::CH2OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch2oi_1(self) -> &'a mut W {
        self.variant(CH2OI_A::CH2OI_1)
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
#[doc = "Channel 3 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OI_A {
    #[doc = "0: The initialization value is 0."]
    CH3OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH3OI_1 = 1,
}
impl From<CH3OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OI` reader - Channel 3 Output Initialization Value"]
pub struct CH3OI_R(crate::FieldReader<bool, CH3OI_A>);
impl CH3OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OI_A {
        match self.bits {
            false => CH3OI_A::CH3OI_0,
            true => CH3OI_A::CH3OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH3OI_0`"]
    #[inline(always)]
    pub fn is_ch3oi_0(&self) -> bool {
        **self == CH3OI_A::CH3OI_0
    }
    #[doc = "Checks if the value of the field is `CH3OI_1`"]
    #[inline(always)]
    pub fn is_ch3oi_1(&self) -> bool {
        **self == CH3OI_A::CH3OI_1
    }
}
impl core::ops::Deref for CH3OI_R {
    type Target = crate::FieldReader<bool, CH3OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3OI` writer - Channel 3 Output Initialization Value"]
pub struct CH3OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch3oi_0(self) -> &'a mut W {
        self.variant(CH3OI_A::CH3OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch3oi_1(self) -> &'a mut W {
        self.variant(CH3OI_A::CH3OI_1)
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
#[doc = "Channel 4 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OI_A {
    #[doc = "0: The initialization value is 0."]
    CH4OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH4OI_1 = 1,
}
impl From<CH4OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OI` reader - Channel 4 Output Initialization Value"]
pub struct CH4OI_R(crate::FieldReader<bool, CH4OI_A>);
impl CH4OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OI_A {
        match self.bits {
            false => CH4OI_A::CH4OI_0,
            true => CH4OI_A::CH4OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH4OI_0`"]
    #[inline(always)]
    pub fn is_ch4oi_0(&self) -> bool {
        **self == CH4OI_A::CH4OI_0
    }
    #[doc = "Checks if the value of the field is `CH4OI_1`"]
    #[inline(always)]
    pub fn is_ch4oi_1(&self) -> bool {
        **self == CH4OI_A::CH4OI_1
    }
}
impl core::ops::Deref for CH4OI_R {
    type Target = crate::FieldReader<bool, CH4OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4OI` writer - Channel 4 Output Initialization Value"]
pub struct CH4OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch4oi_0(self) -> &'a mut W {
        self.variant(CH4OI_A::CH4OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch4oi_1(self) -> &'a mut W {
        self.variant(CH4OI_A::CH4OI_1)
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
#[doc = "Channel 5 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OI_A {
    #[doc = "0: The initialization value is 0."]
    CH5OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH5OI_1 = 1,
}
impl From<CH5OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OI` reader - Channel 5 Output Initialization Value"]
pub struct CH5OI_R(crate::FieldReader<bool, CH5OI_A>);
impl CH5OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OI_A {
        match self.bits {
            false => CH5OI_A::CH5OI_0,
            true => CH5OI_A::CH5OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH5OI_0`"]
    #[inline(always)]
    pub fn is_ch5oi_0(&self) -> bool {
        **self == CH5OI_A::CH5OI_0
    }
    #[doc = "Checks if the value of the field is `CH5OI_1`"]
    #[inline(always)]
    pub fn is_ch5oi_1(&self) -> bool {
        **self == CH5OI_A::CH5OI_1
    }
}
impl core::ops::Deref for CH5OI_R {
    type Target = crate::FieldReader<bool, CH5OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5OI` writer - Channel 5 Output Initialization Value"]
pub struct CH5OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch5oi_0(self) -> &'a mut W {
        self.variant(CH5OI_A::CH5OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch5oi_1(self) -> &'a mut W {
        self.variant(CH5OI_A::CH5OI_1)
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
#[doc = "Channel 6 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OI_A {
    #[doc = "0: The initialization value is 0."]
    CH6OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH6OI_1 = 1,
}
impl From<CH6OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OI` reader - Channel 6 Output Initialization Value"]
pub struct CH6OI_R(crate::FieldReader<bool, CH6OI_A>);
impl CH6OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OI_A {
        match self.bits {
            false => CH6OI_A::CH6OI_0,
            true => CH6OI_A::CH6OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH6OI_0`"]
    #[inline(always)]
    pub fn is_ch6oi_0(&self) -> bool {
        **self == CH6OI_A::CH6OI_0
    }
    #[doc = "Checks if the value of the field is `CH6OI_1`"]
    #[inline(always)]
    pub fn is_ch6oi_1(&self) -> bool {
        **self == CH6OI_A::CH6OI_1
    }
}
impl core::ops::Deref for CH6OI_R {
    type Target = crate::FieldReader<bool, CH6OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6OI` writer - Channel 6 Output Initialization Value"]
pub struct CH6OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch6oi_0(self) -> &'a mut W {
        self.variant(CH6OI_A::CH6OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch6oi_1(self) -> &'a mut W {
        self.variant(CH6OI_A::CH6OI_1)
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
#[doc = "Channel 7 Output Initialization Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OI_A {
    #[doc = "0: The initialization value is 0."]
    CH7OI_0 = 0,
    #[doc = "1: The initialization value is 1."]
    CH7OI_1 = 1,
}
impl From<CH7OI_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OI` reader - Channel 7 Output Initialization Value"]
pub struct CH7OI_R(crate::FieldReader<bool, CH7OI_A>);
impl CH7OI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7OI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OI_A {
        match self.bits {
            false => CH7OI_A::CH7OI_0,
            true => CH7OI_A::CH7OI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH7OI_0`"]
    #[inline(always)]
    pub fn is_ch7oi_0(&self) -> bool {
        **self == CH7OI_A::CH7OI_0
    }
    #[doc = "Checks if the value of the field is `CH7OI_1`"]
    #[inline(always)]
    pub fn is_ch7oi_1(&self) -> bool {
        **self == CH7OI_A::CH7OI_1
    }
}
impl core::ops::Deref for CH7OI_R {
    type Target = crate::FieldReader<bool, CH7OI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7OI` writer - Channel 7 Output Initialization Value"]
pub struct CH7OI_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The initialization value is 0."]
    #[inline(always)]
    pub fn ch7oi_0(self) -> &'a mut W {
        self.variant(CH7OI_A::CH7OI_0)
    }
    #[doc = "The initialization value is 1."]
    #[inline(always)]
    pub fn ch7oi_1(self) -> &'a mut W {
        self.variant(CH7OI_A::CH7OI_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&self) -> CH0OI_R {
        CH0OI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&self) -> CH1OI_R {
        CH1OI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&self) -> CH2OI_R {
        CH2OI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&self) -> CH3OI_R {
        CH3OI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&self) -> CH4OI_R {
        CH4OI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&self) -> CH5OI_R {
        CH5OI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&self) -> CH6OI_R {
        CH6OI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&self) -> CH7OI_R {
        CH7OI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Initialization Value"]
    #[inline(always)]
    pub fn ch0oi(&mut self) -> CH0OI_W {
        CH0OI_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Initialization Value"]
    #[inline(always)]
    pub fn ch1oi(&mut self) -> CH1OI_W {
        CH1OI_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Initialization Value"]
    #[inline(always)]
    pub fn ch2oi(&mut self) -> CH2OI_W {
        CH2OI_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Initialization Value"]
    #[inline(always)]
    pub fn ch3oi(&mut self) -> CH3OI_W {
        CH3OI_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Initialization Value"]
    #[inline(always)]
    pub fn ch4oi(&mut self) -> CH4OI_W {
        CH4OI_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Initialization Value"]
    #[inline(always)]
    pub fn ch5oi(&mut self) -> CH5OI_W {
        CH5OI_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Initialization Value"]
    #[inline(always)]
    pub fn ch6oi(&mut self) -> CH6OI_W {
        CH6OI_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Initialization Value"]
    #[inline(always)]
    pub fn ch7oi(&mut self) -> CH7OI_W {
        CH7OI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initial State For Channels Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outinit](index.html) module"]
pub struct OUTINIT_SPEC;
impl crate::RegisterSpec for OUTINIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outinit::R](R) reader structure"]
impl crate::Readable for OUTINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outinit::W](W) writer structure"]
impl crate::Writable for OUTINIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTINIT to value 0"]
impl crate::Resettable for OUTINIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
