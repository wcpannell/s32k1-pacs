#[doc = "Register `SWOCTRL` reader"]
pub struct R(crate::R<SWOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWOCTRL` writer"]
pub struct W(crate::W<SWOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWOCTRL_SPEC>;
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
impl From<crate::W<SWOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH0OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH0OC_1 = 1,
}
impl From<CH0OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OC` reader - Channel 0 Software Output Control Enable"]
pub struct CH0OC_R(crate::FieldReader<bool, CH0OC_A>);
impl CH0OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OC_A {
        match self.bits {
            false => CH0OC_A::CH0OC_0,
            true => CH0OC_A::CH0OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH0OC_0`"]
    #[inline(always)]
    pub fn is_ch0oc_0(&self) -> bool {
        **self == CH0OC_A::CH0OC_0
    }
    #[doc = "Checks if the value of the field is `CH0OC_1`"]
    #[inline(always)]
    pub fn is_ch0oc_1(&self) -> bool {
        **self == CH0OC_A::CH0OC_1
    }
}
impl core::ops::Deref for CH0OC_R {
    type Target = crate::FieldReader<bool, CH0OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0OC` writer - Channel 0 Software Output Control Enable"]
pub struct CH0OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch0oc_0(self) -> &'a mut W {
        self.variant(CH0OC_A::CH0OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch0oc_1(self) -> &'a mut W {
        self.variant(CH0OC_A::CH0OC_1)
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
#[doc = "Channel 1 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH1OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH1OC_1 = 1,
}
impl From<CH1OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OC` reader - Channel 1 Software Output Control Enable"]
pub struct CH1OC_R(crate::FieldReader<bool, CH1OC_A>);
impl CH1OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OC_A {
        match self.bits {
            false => CH1OC_A::CH1OC_0,
            true => CH1OC_A::CH1OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH1OC_0`"]
    #[inline(always)]
    pub fn is_ch1oc_0(&self) -> bool {
        **self == CH1OC_A::CH1OC_0
    }
    #[doc = "Checks if the value of the field is `CH1OC_1`"]
    #[inline(always)]
    pub fn is_ch1oc_1(&self) -> bool {
        **self == CH1OC_A::CH1OC_1
    }
}
impl core::ops::Deref for CH1OC_R {
    type Target = crate::FieldReader<bool, CH1OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OC` writer - Channel 1 Software Output Control Enable"]
pub struct CH1OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch1oc_0(self) -> &'a mut W {
        self.variant(CH1OC_A::CH1OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch1oc_1(self) -> &'a mut W {
        self.variant(CH1OC_A::CH1OC_1)
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
#[doc = "Channel 2 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH2OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH2OC_1 = 1,
}
impl From<CH2OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OC` reader - Channel 2 Software Output Control Enable"]
pub struct CH2OC_R(crate::FieldReader<bool, CH2OC_A>);
impl CH2OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OC_A {
        match self.bits {
            false => CH2OC_A::CH2OC_0,
            true => CH2OC_A::CH2OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH2OC_0`"]
    #[inline(always)]
    pub fn is_ch2oc_0(&self) -> bool {
        **self == CH2OC_A::CH2OC_0
    }
    #[doc = "Checks if the value of the field is `CH2OC_1`"]
    #[inline(always)]
    pub fn is_ch2oc_1(&self) -> bool {
        **self == CH2OC_A::CH2OC_1
    }
}
impl core::ops::Deref for CH2OC_R {
    type Target = crate::FieldReader<bool, CH2OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2OC` writer - Channel 2 Software Output Control Enable"]
pub struct CH2OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch2oc_0(self) -> &'a mut W {
        self.variant(CH2OC_A::CH2OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch2oc_1(self) -> &'a mut W {
        self.variant(CH2OC_A::CH2OC_1)
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
#[doc = "Channel 3 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH3OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH3OC_1 = 1,
}
impl From<CH3OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OC` reader - Channel 3 Software Output Control Enable"]
pub struct CH3OC_R(crate::FieldReader<bool, CH3OC_A>);
impl CH3OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OC_A {
        match self.bits {
            false => CH3OC_A::CH3OC_0,
            true => CH3OC_A::CH3OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH3OC_0`"]
    #[inline(always)]
    pub fn is_ch3oc_0(&self) -> bool {
        **self == CH3OC_A::CH3OC_0
    }
    #[doc = "Checks if the value of the field is `CH3OC_1`"]
    #[inline(always)]
    pub fn is_ch3oc_1(&self) -> bool {
        **self == CH3OC_A::CH3OC_1
    }
}
impl core::ops::Deref for CH3OC_R {
    type Target = crate::FieldReader<bool, CH3OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3OC` writer - Channel 3 Software Output Control Enable"]
pub struct CH3OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch3oc_0(self) -> &'a mut W {
        self.variant(CH3OC_A::CH3OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch3oc_1(self) -> &'a mut W {
        self.variant(CH3OC_A::CH3OC_1)
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
#[doc = "Channel 4 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH4OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH4OC_1 = 1,
}
impl From<CH4OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OC` reader - Channel 4 Software Output Control Enable"]
pub struct CH4OC_R(crate::FieldReader<bool, CH4OC_A>);
impl CH4OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OC_A {
        match self.bits {
            false => CH4OC_A::CH4OC_0,
            true => CH4OC_A::CH4OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH4OC_0`"]
    #[inline(always)]
    pub fn is_ch4oc_0(&self) -> bool {
        **self == CH4OC_A::CH4OC_0
    }
    #[doc = "Checks if the value of the field is `CH4OC_1`"]
    #[inline(always)]
    pub fn is_ch4oc_1(&self) -> bool {
        **self == CH4OC_A::CH4OC_1
    }
}
impl core::ops::Deref for CH4OC_R {
    type Target = crate::FieldReader<bool, CH4OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4OC` writer - Channel 4 Software Output Control Enable"]
pub struct CH4OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch4oc_0(self) -> &'a mut W {
        self.variant(CH4OC_A::CH4OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch4oc_1(self) -> &'a mut W {
        self.variant(CH4OC_A::CH4OC_1)
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
#[doc = "Channel 5 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH5OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH5OC_1 = 1,
}
impl From<CH5OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OC` reader - Channel 5 Software Output Control Enable"]
pub struct CH5OC_R(crate::FieldReader<bool, CH5OC_A>);
impl CH5OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OC_A {
        match self.bits {
            false => CH5OC_A::CH5OC_0,
            true => CH5OC_A::CH5OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH5OC_0`"]
    #[inline(always)]
    pub fn is_ch5oc_0(&self) -> bool {
        **self == CH5OC_A::CH5OC_0
    }
    #[doc = "Checks if the value of the field is `CH5OC_1`"]
    #[inline(always)]
    pub fn is_ch5oc_1(&self) -> bool {
        **self == CH5OC_A::CH5OC_1
    }
}
impl core::ops::Deref for CH5OC_R {
    type Target = crate::FieldReader<bool, CH5OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5OC` writer - Channel 5 Software Output Control Enable"]
pub struct CH5OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch5oc_0(self) -> &'a mut W {
        self.variant(CH5OC_A::CH5OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch5oc_1(self) -> &'a mut W {
        self.variant(CH5OC_A::CH5OC_1)
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
#[doc = "Channel 6 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH6OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH6OC_1 = 1,
}
impl From<CH6OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OC` reader - Channel 6 Software Output Control Enable"]
pub struct CH6OC_R(crate::FieldReader<bool, CH6OC_A>);
impl CH6OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OC_A {
        match self.bits {
            false => CH6OC_A::CH6OC_0,
            true => CH6OC_A::CH6OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH6OC_0`"]
    #[inline(always)]
    pub fn is_ch6oc_0(&self) -> bool {
        **self == CH6OC_A::CH6OC_0
    }
    #[doc = "Checks if the value of the field is `CH6OC_1`"]
    #[inline(always)]
    pub fn is_ch6oc_1(&self) -> bool {
        **self == CH6OC_A::CH6OC_1
    }
}
impl core::ops::Deref for CH6OC_R {
    type Target = crate::FieldReader<bool, CH6OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6OC` writer - Channel 6 Software Output Control Enable"]
pub struct CH6OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch6oc_0(self) -> &'a mut W {
        self.variant(CH6OC_A::CH6OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch6oc_1(self) -> &'a mut W {
        self.variant(CH6OC_A::CH6OC_1)
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
#[doc = "Channel 7 Software Output Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OC_A {
    #[doc = "0: The channel output is not affected by software output control."]
    CH7OC_0 = 0,
    #[doc = "1: The channel output is affected by software output control."]
    CH7OC_1 = 1,
}
impl From<CH7OC_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OC` reader - Channel 7 Software Output Control Enable"]
pub struct CH7OC_R(crate::FieldReader<bool, CH7OC_A>);
impl CH7OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7OC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OC_A {
        match self.bits {
            false => CH7OC_A::CH7OC_0,
            true => CH7OC_A::CH7OC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH7OC_0`"]
    #[inline(always)]
    pub fn is_ch7oc_0(&self) -> bool {
        **self == CH7OC_A::CH7OC_0
    }
    #[doc = "Checks if the value of the field is `CH7OC_1`"]
    #[inline(always)]
    pub fn is_ch7oc_1(&self) -> bool {
        **self == CH7OC_A::CH7OC_1
    }
}
impl core::ops::Deref for CH7OC_R {
    type Target = crate::FieldReader<bool, CH7OC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7OC` writer - Channel 7 Software Output Control Enable"]
pub struct CH7OC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel output is not affected by software output control."]
    #[inline(always)]
    pub fn ch7oc_0(self) -> &'a mut W {
        self.variant(CH7OC_A::CH7OC_0)
    }
    #[doc = "The channel output is affected by software output control."]
    #[inline(always)]
    pub fn ch7oc_1(self) -> &'a mut W {
        self.variant(CH7OC_A::CH7OC_1)
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
#[doc = "Channel 0 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH0OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH0OCV_1 = 1,
}
impl From<CH0OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OCV` reader - Channel 0 Software Output Control Value"]
pub struct CH0OCV_R(crate::FieldReader<bool, CH0OCV_A>);
impl CH0OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OCV_A {
        match self.bits {
            false => CH0OCV_A::CH0OCV_0,
            true => CH0OCV_A::CH0OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH0OCV_0`"]
    #[inline(always)]
    pub fn is_ch0ocv_0(&self) -> bool {
        **self == CH0OCV_A::CH0OCV_0
    }
    #[doc = "Checks if the value of the field is `CH0OCV_1`"]
    #[inline(always)]
    pub fn is_ch0ocv_1(&self) -> bool {
        **self == CH0OCV_A::CH0OCV_1
    }
}
impl core::ops::Deref for CH0OCV_R {
    type Target = crate::FieldReader<bool, CH0OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0OCV` writer - Channel 0 Software Output Control Value"]
pub struct CH0OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch0ocv_0(self) -> &'a mut W {
        self.variant(CH0OCV_A::CH0OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch0ocv_1(self) -> &'a mut W {
        self.variant(CH0OCV_A::CH0OCV_1)
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
#[doc = "Channel 1 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH1OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH1OCV_1 = 1,
}
impl From<CH1OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OCV` reader - Channel 1 Software Output Control Value"]
pub struct CH1OCV_R(crate::FieldReader<bool, CH1OCV_A>);
impl CH1OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OCV_A {
        match self.bits {
            false => CH1OCV_A::CH1OCV_0,
            true => CH1OCV_A::CH1OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH1OCV_0`"]
    #[inline(always)]
    pub fn is_ch1ocv_0(&self) -> bool {
        **self == CH1OCV_A::CH1OCV_0
    }
    #[doc = "Checks if the value of the field is `CH1OCV_1`"]
    #[inline(always)]
    pub fn is_ch1ocv_1(&self) -> bool {
        **self == CH1OCV_A::CH1OCV_1
    }
}
impl core::ops::Deref for CH1OCV_R {
    type Target = crate::FieldReader<bool, CH1OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OCV` writer - Channel 1 Software Output Control Value"]
pub struct CH1OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch1ocv_0(self) -> &'a mut W {
        self.variant(CH1OCV_A::CH1OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch1ocv_1(self) -> &'a mut W {
        self.variant(CH1OCV_A::CH1OCV_1)
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
#[doc = "Channel 2 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH2OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH2OCV_1 = 1,
}
impl From<CH2OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OCV` reader - Channel 2 Software Output Control Value"]
pub struct CH2OCV_R(crate::FieldReader<bool, CH2OCV_A>);
impl CH2OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OCV_A {
        match self.bits {
            false => CH2OCV_A::CH2OCV_0,
            true => CH2OCV_A::CH2OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH2OCV_0`"]
    #[inline(always)]
    pub fn is_ch2ocv_0(&self) -> bool {
        **self == CH2OCV_A::CH2OCV_0
    }
    #[doc = "Checks if the value of the field is `CH2OCV_1`"]
    #[inline(always)]
    pub fn is_ch2ocv_1(&self) -> bool {
        **self == CH2OCV_A::CH2OCV_1
    }
}
impl core::ops::Deref for CH2OCV_R {
    type Target = crate::FieldReader<bool, CH2OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2OCV` writer - Channel 2 Software Output Control Value"]
pub struct CH2OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch2ocv_0(self) -> &'a mut W {
        self.variant(CH2OCV_A::CH2OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch2ocv_1(self) -> &'a mut W {
        self.variant(CH2OCV_A::CH2OCV_1)
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
#[doc = "Channel 3 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH3OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH3OCV_1 = 1,
}
impl From<CH3OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OCV` reader - Channel 3 Software Output Control Value"]
pub struct CH3OCV_R(crate::FieldReader<bool, CH3OCV_A>);
impl CH3OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OCV_A {
        match self.bits {
            false => CH3OCV_A::CH3OCV_0,
            true => CH3OCV_A::CH3OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH3OCV_0`"]
    #[inline(always)]
    pub fn is_ch3ocv_0(&self) -> bool {
        **self == CH3OCV_A::CH3OCV_0
    }
    #[doc = "Checks if the value of the field is `CH3OCV_1`"]
    #[inline(always)]
    pub fn is_ch3ocv_1(&self) -> bool {
        **self == CH3OCV_A::CH3OCV_1
    }
}
impl core::ops::Deref for CH3OCV_R {
    type Target = crate::FieldReader<bool, CH3OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3OCV` writer - Channel 3 Software Output Control Value"]
pub struct CH3OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch3ocv_0(self) -> &'a mut W {
        self.variant(CH3OCV_A::CH3OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch3ocv_1(self) -> &'a mut W {
        self.variant(CH3OCV_A::CH3OCV_1)
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
#[doc = "Channel 4 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH4OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH4OCV_1 = 1,
}
impl From<CH4OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OCV` reader - Channel 4 Software Output Control Value"]
pub struct CH4OCV_R(crate::FieldReader<bool, CH4OCV_A>);
impl CH4OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OCV_A {
        match self.bits {
            false => CH4OCV_A::CH4OCV_0,
            true => CH4OCV_A::CH4OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH4OCV_0`"]
    #[inline(always)]
    pub fn is_ch4ocv_0(&self) -> bool {
        **self == CH4OCV_A::CH4OCV_0
    }
    #[doc = "Checks if the value of the field is `CH4OCV_1`"]
    #[inline(always)]
    pub fn is_ch4ocv_1(&self) -> bool {
        **self == CH4OCV_A::CH4OCV_1
    }
}
impl core::ops::Deref for CH4OCV_R {
    type Target = crate::FieldReader<bool, CH4OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4OCV` writer - Channel 4 Software Output Control Value"]
pub struct CH4OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch4ocv_0(self) -> &'a mut W {
        self.variant(CH4OCV_A::CH4OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch4ocv_1(self) -> &'a mut W {
        self.variant(CH4OCV_A::CH4OCV_1)
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
#[doc = "Channel 5 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH5OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH5OCV_1 = 1,
}
impl From<CH5OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OCV` reader - Channel 5 Software Output Control Value"]
pub struct CH5OCV_R(crate::FieldReader<bool, CH5OCV_A>);
impl CH5OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OCV_A {
        match self.bits {
            false => CH5OCV_A::CH5OCV_0,
            true => CH5OCV_A::CH5OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH5OCV_0`"]
    #[inline(always)]
    pub fn is_ch5ocv_0(&self) -> bool {
        **self == CH5OCV_A::CH5OCV_0
    }
    #[doc = "Checks if the value of the field is `CH5OCV_1`"]
    #[inline(always)]
    pub fn is_ch5ocv_1(&self) -> bool {
        **self == CH5OCV_A::CH5OCV_1
    }
}
impl core::ops::Deref for CH5OCV_R {
    type Target = crate::FieldReader<bool, CH5OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5OCV` writer - Channel 5 Software Output Control Value"]
pub struct CH5OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch5ocv_0(self) -> &'a mut W {
        self.variant(CH5OCV_A::CH5OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch5ocv_1(self) -> &'a mut W {
        self.variant(CH5OCV_A::CH5OCV_1)
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
#[doc = "Channel 6 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH6OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH6OCV_1 = 1,
}
impl From<CH6OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OCV` reader - Channel 6 Software Output Control Value"]
pub struct CH6OCV_R(crate::FieldReader<bool, CH6OCV_A>);
impl CH6OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OCV_A {
        match self.bits {
            false => CH6OCV_A::CH6OCV_0,
            true => CH6OCV_A::CH6OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH6OCV_0`"]
    #[inline(always)]
    pub fn is_ch6ocv_0(&self) -> bool {
        **self == CH6OCV_A::CH6OCV_0
    }
    #[doc = "Checks if the value of the field is `CH6OCV_1`"]
    #[inline(always)]
    pub fn is_ch6ocv_1(&self) -> bool {
        **self == CH6OCV_A::CH6OCV_1
    }
}
impl core::ops::Deref for CH6OCV_R {
    type Target = crate::FieldReader<bool, CH6OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6OCV` writer - Channel 6 Software Output Control Value"]
pub struct CH6OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch6ocv_0(self) -> &'a mut W {
        self.variant(CH6OCV_A::CH6OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch6ocv_1(self) -> &'a mut W {
        self.variant(CH6OCV_A::CH6OCV_1)
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
#[doc = "Channel 7 Software Output Control Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OCV_A {
    #[doc = "0: The software output control forces 0 to the channel output."]
    CH7OCV_0 = 0,
    #[doc = "1: The software output control forces 1 to the channel output."]
    CH7OCV_1 = 1,
}
impl From<CH7OCV_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OCV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OCV` reader - Channel 7 Software Output Control Value"]
pub struct CH7OCV_R(crate::FieldReader<bool, CH7OCV_A>);
impl CH7OCV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7OCV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OCV_A {
        match self.bits {
            false => CH7OCV_A::CH7OCV_0,
            true => CH7OCV_A::CH7OCV_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH7OCV_0`"]
    #[inline(always)]
    pub fn is_ch7ocv_0(&self) -> bool {
        **self == CH7OCV_A::CH7OCV_0
    }
    #[doc = "Checks if the value of the field is `CH7OCV_1`"]
    #[inline(always)]
    pub fn is_ch7ocv_1(&self) -> bool {
        **self == CH7OCV_A::CH7OCV_1
    }
}
impl core::ops::Deref for CH7OCV_R {
    type Target = crate::FieldReader<bool, CH7OCV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7OCV` writer - Channel 7 Software Output Control Value"]
pub struct CH7OCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OCV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OCV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The software output control forces 0 to the channel output."]
    #[inline(always)]
    pub fn ch7ocv_0(self) -> &'a mut W {
        self.variant(CH7OCV_A::CH7OCV_0)
    }
    #[doc = "The software output control forces 1 to the channel output."]
    #[inline(always)]
    pub fn ch7ocv_1(self) -> &'a mut W {
        self.variant(CH7OCV_A::CH7OCV_1)
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
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&self) -> CH0OC_R {
        CH0OC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&self) -> CH1OC_R {
        CH1OC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&self) -> CH2OC_R {
        CH2OC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&self) -> CH3OC_R {
        CH3OC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&self) -> CH4OC_R {
        CH4OC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&self) -> CH5OC_R {
        CH5OC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&self) -> CH6OC_R {
        CH6OC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&self) -> CH7OC_R {
        CH7OC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&self) -> CH0OCV_R {
        CH0OCV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&self) -> CH1OCV_R {
        CH1OCV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&self) -> CH2OCV_R {
        CH2OCV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&self) -> CH3OCV_R {
        CH3OCV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&self) -> CH4OCV_R {
        CH4OCV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&self) -> CH5OCV_R {
        CH5OCV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&self) -> CH6OCV_R {
        CH6OCV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&self) -> CH7OCV_R {
        CH7OCV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch0oc(&mut self) -> CH0OC_W {
        CH0OC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch1oc(&mut self) -> CH1OC_W {
        CH1OC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch2oc(&mut self) -> CH2OC_W {
        CH2OC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch3oc(&mut self) -> CH3OC_W {
        CH3OC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch4oc(&mut self) -> CH4OC_W {
        CH4OC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch5oc(&mut self) -> CH5OC_W {
        CH5OC_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch6oc(&mut self) -> CH6OC_W {
        CH6OC_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Output Control Enable"]
    #[inline(always)]
    pub fn ch7oc(&mut self) -> CH7OC_W {
        CH7OC_W { w: self }
    }
    #[doc = "Bit 8 - Channel 0 Software Output Control Value"]
    #[inline(always)]
    pub fn ch0ocv(&mut self) -> CH0OCV_W {
        CH0OCV_W { w: self }
    }
    #[doc = "Bit 9 - Channel 1 Software Output Control Value"]
    #[inline(always)]
    pub fn ch1ocv(&mut self) -> CH1OCV_W {
        CH1OCV_W { w: self }
    }
    #[doc = "Bit 10 - Channel 2 Software Output Control Value"]
    #[inline(always)]
    pub fn ch2ocv(&mut self) -> CH2OCV_W {
        CH2OCV_W { w: self }
    }
    #[doc = "Bit 11 - Channel 3 Software Output Control Value"]
    #[inline(always)]
    pub fn ch3ocv(&mut self) -> CH3OCV_W {
        CH3OCV_W { w: self }
    }
    #[doc = "Bit 12 - Channel 4 Software Output Control Value"]
    #[inline(always)]
    pub fn ch4ocv(&mut self) -> CH4OCV_W {
        CH4OCV_W { w: self }
    }
    #[doc = "Bit 13 - Channel 5 Software Output Control Value"]
    #[inline(always)]
    pub fn ch5ocv(&mut self) -> CH5OCV_W {
        CH5OCV_W { w: self }
    }
    #[doc = "Bit 14 - Channel 6 Software Output Control Value"]
    #[inline(always)]
    pub fn ch6ocv(&mut self) -> CH6OCV_W {
        CH6OCV_W { w: self }
    }
    #[doc = "Bit 15 - Channel 7 Software Output Control Value"]
    #[inline(always)]
    pub fn ch7ocv(&mut self) -> CH7OCV_W {
        CH7OCV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Software Output Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swoctrl](index.html) module"]
pub struct SWOCTRL_SPEC;
impl crate::RegisterSpec for SWOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swoctrl::R](R) reader structure"]
impl crate::Readable for SWOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swoctrl::W](W) writer structure"]
impl crate::Writable for SWOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWOCTRL to value 0"]
impl crate::Resettable for SWOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
