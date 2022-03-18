#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH0F_A> for bool {
    #[inline(always)]
    fn from(variant: CH0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0F` reader - Channel 0 Flag"]
pub struct CH0F_R(crate::FieldReader<bool, CH0F_A>);
impl CH0F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0F_A {
        match self.bits {
            false => CH0F_A::_0,
            true => CH0F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH0F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH0F_A::_1
    }
}
impl core::ops::Deref for CH0F_R {
    type Target = crate::FieldReader<bool, CH0F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1F` reader - Channel 1 Flag"]
pub struct CH1F_R(crate::FieldReader<bool, CH1F_A>);
impl CH1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::_0,
            true => CH1F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH1F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH1F_A::_1
    }
}
impl core::ops::Deref for CH1F_R {
    type Target = crate::FieldReader<bool, CH1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH2F_A> for bool {
    #[inline(always)]
    fn from(variant: CH2F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2F` reader - Channel 2 Flag"]
pub struct CH2F_R(crate::FieldReader<bool, CH2F_A>);
impl CH2F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2F_A {
        match self.bits {
            false => CH2F_A::_0,
            true => CH2F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH2F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH2F_A::_1
    }
}
impl core::ops::Deref for CH2F_R {
    type Target = crate::FieldReader<bool, CH2F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 3 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH3F_A> for bool {
    #[inline(always)]
    fn from(variant: CH3F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3F` reader - Channel 3 Flag"]
pub struct CH3F_R(crate::FieldReader<bool, CH3F_A>);
impl CH3F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3F_A {
        match self.bits {
            false => CH3F_A::_0,
            true => CH3F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH3F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH3F_A::_1
    }
}
impl core::ops::Deref for CH3F_R {
    type Target = crate::FieldReader<bool, CH3F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 4 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH4F_A> for bool {
    #[inline(always)]
    fn from(variant: CH4F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4F` reader - Channel 4 Flag"]
pub struct CH4F_R(crate::FieldReader<bool, CH4F_A>);
impl CH4F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4F_A {
        match self.bits {
            false => CH4F_A::_0,
            true => CH4F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH4F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH4F_A::_1
    }
}
impl core::ops::Deref for CH4F_R {
    type Target = crate::FieldReader<bool, CH4F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 5 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH5F_A> for bool {
    #[inline(always)]
    fn from(variant: CH5F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5F` reader - Channel 5 Flag"]
pub struct CH5F_R(crate::FieldReader<bool, CH5F_A>);
impl CH5F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5F_A {
        match self.bits {
            false => CH5F_A::_0,
            true => CH5F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH5F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH5F_A::_1
    }
}
impl core::ops::Deref for CH5F_R {
    type Target = crate::FieldReader<bool, CH5F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 6 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH6F_A> for bool {
    #[inline(always)]
    fn from(variant: CH6F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6F` reader - Channel 6 Flag"]
pub struct CH6F_R(crate::FieldReader<bool, CH6F_A>);
impl CH6F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6F_A {
        match self.bits {
            false => CH6F_A::_0,
            true => CH6F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH6F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH6F_A::_1
    }
}
impl core::ops::Deref for CH6F_R {
    type Target = crate::FieldReader<bool, CH6F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Channel 7 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7F_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CH7F_A> for bool {
    #[inline(always)]
    fn from(variant: CH7F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7F` reader - Channel 7 Flag"]
pub struct CH7F_R(crate::FieldReader<bool, CH7F_A>);
impl CH7F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7F_A {
        match self.bits {
            false => CH7F_A::_0,
            true => CH7F_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH7F_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH7F_A::_1
    }
}
impl core::ops::Deref for CH7F_R {
    type Target = crate::FieldReader<bool, CH7F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> CH0F_R {
        CH0F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Flag"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Flag"]
    #[inline(always)]
    pub fn ch7f(&self) -> CH7F_R {
        CH7F_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Capture And Compare Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
