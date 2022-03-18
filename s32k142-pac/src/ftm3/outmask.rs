#[doc = "Register `OUTMASK` reader"]
pub struct R(crate::R<OUTMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTMASK` writer"]
pub struct W(crate::W<OUTMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTMASK_SPEC>;
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
impl From<crate::W<OUTMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH0OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH0OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0OM` reader - Channel 0 Output Mask"]
pub struct CH0OM_R(crate::FieldReader<bool, CH0OM_A>);
impl CH0OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0OM_A {
        match self.bits {
            false => CH0OM_A::_0,
            true => CH0OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH0OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH0OM_A::_1
    }
}
impl core::ops::Deref for CH0OM_R {
    type Target = crate::FieldReader<bool, CH0OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0OM` writer - Channel 0 Output Mask"]
pub struct CH0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH0OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH0OM_A::_1)
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
#[doc = "Channel 1 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH1OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH1OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1OM` reader - Channel 1 Output Mask"]
pub struct CH1OM_R(crate::FieldReader<bool, CH1OM_A>);
impl CH1OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1OM_A {
        match self.bits {
            false => CH1OM_A::_0,
            true => CH1OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH1OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH1OM_A::_1
    }
}
impl core::ops::Deref for CH1OM_R {
    type Target = crate::FieldReader<bool, CH1OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1OM` writer - Channel 1 Output Mask"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH1OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH1OM_A::_1)
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
#[doc = "Channel 2 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH2OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH2OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2OM` reader - Channel 2 Output Mask"]
pub struct CH2OM_R(crate::FieldReader<bool, CH2OM_A>);
impl CH2OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2OM_A {
        match self.bits {
            false => CH2OM_A::_0,
            true => CH2OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH2OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH2OM_A::_1
    }
}
impl core::ops::Deref for CH2OM_R {
    type Target = crate::FieldReader<bool, CH2OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2OM` writer - Channel 2 Output Mask"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH2OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH2OM_A::_1)
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
#[doc = "Channel 3 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH3OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH3OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3OM` reader - Channel 3 Output Mask"]
pub struct CH3OM_R(crate::FieldReader<bool, CH3OM_A>);
impl CH3OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3OM_A {
        match self.bits {
            false => CH3OM_A::_0,
            true => CH3OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH3OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH3OM_A::_1
    }
}
impl core::ops::Deref for CH3OM_R {
    type Target = crate::FieldReader<bool, CH3OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3OM` writer - Channel 3 Output Mask"]
pub struct CH3OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH3OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH3OM_A::_1)
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
#[doc = "Channel 4 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH4OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH4OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4OM` reader - Channel 4 Output Mask"]
pub struct CH4OM_R(crate::FieldReader<bool, CH4OM_A>);
impl CH4OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4OM_A {
        match self.bits {
            false => CH4OM_A::_0,
            true => CH4OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH4OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH4OM_A::_1
    }
}
impl core::ops::Deref for CH4OM_R {
    type Target = crate::FieldReader<bool, CH4OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4OM` writer - Channel 4 Output Mask"]
pub struct CH4OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH4OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH4OM_A::_1)
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
#[doc = "Channel 5 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH5OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH5OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5OM` reader - Channel 5 Output Mask"]
pub struct CH5OM_R(crate::FieldReader<bool, CH5OM_A>);
impl CH5OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5OM_A {
        match self.bits {
            false => CH5OM_A::_0,
            true => CH5OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH5OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH5OM_A::_1
    }
}
impl core::ops::Deref for CH5OM_R {
    type Target = crate::FieldReader<bool, CH5OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5OM` writer - Channel 5 Output Mask"]
pub struct CH5OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH5OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH5OM_A::_1)
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
#[doc = "Channel 6 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH6OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH6OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6OM` reader - Channel 6 Output Mask"]
pub struct CH6OM_R(crate::FieldReader<bool, CH6OM_A>);
impl CH6OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6OM_A {
        match self.bits {
            false => CH6OM_A::_0,
            true => CH6OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH6OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH6OM_A::_1
    }
}
impl core::ops::Deref for CH6OM_R {
    type Target = crate::FieldReader<bool, CH6OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6OM` writer - Channel 6 Output Mask"]
pub struct CH6OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH6OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH6OM_A::_1)
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
#[doc = "Channel 7 Output Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7OM_A {
    #[doc = "0: Channel output is not masked. It continues to operate normally."]
    _0 = 0,
    #[doc = "1: Channel output is masked. It is forced to its inactive state."]
    _1 = 1,
}
impl From<CH7OM_A> for bool {
    #[inline(always)]
    fn from(variant: CH7OM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7OM` reader - Channel 7 Output Mask"]
pub struct CH7OM_R(crate::FieldReader<bool, CH7OM_A>);
impl CH7OM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7OM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7OM_A {
        match self.bits {
            false => CH7OM_A::_0,
            true => CH7OM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CH7OM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CH7OM_A::_1
    }
}
impl core::ops::Deref for CH7OM_R {
    type Target = crate::FieldReader<bool, CH7OM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7OM` writer - Channel 7 Output Mask"]
pub struct CH7OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7OM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7OM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output is not masked. It continues to operate normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CH7OM_A::_0)
    }
    #[doc = "Channel output is masked. It is forced to its inactive state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CH7OM_A::_1)
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
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&self) -> CH0OM_R {
        CH0OM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&self) -> CH3OM_R {
        CH3OM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&self) -> CH4OM_R {
        CH4OM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&self) -> CH5OM_R {
        CH5OM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&self) -> CH6OM_R {
        CH6OM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&self) -> CH7OM_R {
        CH7OM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Output Mask"]
    #[inline(always)]
    pub fn ch0om(&mut self) -> CH0OM_W {
        CH0OM_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Output Mask"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Output Mask"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Output Mask"]
    #[inline(always)]
    pub fn ch3om(&mut self) -> CH3OM_W {
        CH3OM_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Output Mask"]
    #[inline(always)]
    pub fn ch4om(&mut self) -> CH4OM_W {
        CH4OM_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Output Mask"]
    #[inline(always)]
    pub fn ch5om(&mut self) -> CH5OM_W {
        CH5OM_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Output Mask"]
    #[inline(always)]
    pub fn ch6om(&mut self) -> CH6OM_W {
        CH6OM_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Output Mask"]
    #[inline(always)]
    pub fn ch7om(&mut self) -> CH7OM_W {
        CH7OM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outmask](index.html) module"]
pub struct OUTMASK_SPEC;
impl crate::RegisterSpec for OUTMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outmask::R](R) reader structure"]
impl crate::Readable for OUTMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outmask::W](W) writer structure"]
impl crate::Writable for OUTMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTMASK to value 0"]
impl crate::Resettable for OUTMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
