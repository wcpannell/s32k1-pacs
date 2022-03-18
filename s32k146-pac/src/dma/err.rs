#[doc = "Register `ERR` reader"]
pub struct R(crate::R<ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERR` writer"]
pub struct W(crate::W<ERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERR_SPEC>;
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
impl From<crate::W<ERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error In Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR0_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR0_A> for bool {
    #[inline(always)]
    fn from(variant: ERR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR0` reader - Error In Channel 0"]
pub struct ERR0_R(crate::FieldReader<bool, ERR0_A>);
impl ERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR0_A {
        match self.bits {
            false => ERR0_A::_0,
            true => ERR0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR0_A::_1
    }
}
impl core::ops::Deref for ERR0_R {
    type Target = crate::FieldReader<bool, ERR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR0` writer - Error In Channel 0"]
pub struct ERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR0_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR0_A::_1)
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
#[doc = "Error In Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR1_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR1_A> for bool {
    #[inline(always)]
    fn from(variant: ERR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR1` reader - Error In Channel 1"]
pub struct ERR1_R(crate::FieldReader<bool, ERR1_A>);
impl ERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR1_A {
        match self.bits {
            false => ERR1_A::_0,
            true => ERR1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR1_A::_1
    }
}
impl core::ops::Deref for ERR1_R {
    type Target = crate::FieldReader<bool, ERR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR1` writer - Error In Channel 1"]
pub struct ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR1_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR1_A::_1)
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
#[doc = "Error In Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR2_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR2_A> for bool {
    #[inline(always)]
    fn from(variant: ERR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR2` reader - Error In Channel 2"]
pub struct ERR2_R(crate::FieldReader<bool, ERR2_A>);
impl ERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR2_A {
        match self.bits {
            false => ERR2_A::_0,
            true => ERR2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR2_A::_1
    }
}
impl core::ops::Deref for ERR2_R {
    type Target = crate::FieldReader<bool, ERR2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR2` writer - Error In Channel 2"]
pub struct ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR2_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR2_A::_1)
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
#[doc = "Error In Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR3_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR3_A> for bool {
    #[inline(always)]
    fn from(variant: ERR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR3` reader - Error In Channel 3"]
pub struct ERR3_R(crate::FieldReader<bool, ERR3_A>);
impl ERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR3_A {
        match self.bits {
            false => ERR3_A::_0,
            true => ERR3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR3_A::_1
    }
}
impl core::ops::Deref for ERR3_R {
    type Target = crate::FieldReader<bool, ERR3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR3` writer - Error In Channel 3"]
pub struct ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR3_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR3_A::_1)
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
#[doc = "Error In Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR4_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR4_A> for bool {
    #[inline(always)]
    fn from(variant: ERR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR4` reader - Error In Channel 4"]
pub struct ERR4_R(crate::FieldReader<bool, ERR4_A>);
impl ERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR4_A {
        match self.bits {
            false => ERR4_A::_0,
            true => ERR4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR4_A::_1
    }
}
impl core::ops::Deref for ERR4_R {
    type Target = crate::FieldReader<bool, ERR4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR4` writer - Error In Channel 4"]
pub struct ERR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR4_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR4_A::_1)
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
#[doc = "Error In Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR5_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR5_A> for bool {
    #[inline(always)]
    fn from(variant: ERR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR5` reader - Error In Channel 5"]
pub struct ERR5_R(crate::FieldReader<bool, ERR5_A>);
impl ERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR5_A {
        match self.bits {
            false => ERR5_A::_0,
            true => ERR5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR5_A::_1
    }
}
impl core::ops::Deref for ERR5_R {
    type Target = crate::FieldReader<bool, ERR5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR5` writer - Error In Channel 5"]
pub struct ERR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR5_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR5_A::_1)
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
#[doc = "Error In Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR6_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR6_A> for bool {
    #[inline(always)]
    fn from(variant: ERR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR6` reader - Error In Channel 6"]
pub struct ERR6_R(crate::FieldReader<bool, ERR6_A>);
impl ERR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR6_A {
        match self.bits {
            false => ERR6_A::_0,
            true => ERR6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR6_A::_1
    }
}
impl core::ops::Deref for ERR6_R {
    type Target = crate::FieldReader<bool, ERR6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR6` writer - Error In Channel 6"]
pub struct ERR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR6_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR6_A::_1)
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
#[doc = "Error In Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR7_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR7_A> for bool {
    #[inline(always)]
    fn from(variant: ERR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR7` reader - Error In Channel 7"]
pub struct ERR7_R(crate::FieldReader<bool, ERR7_A>);
impl ERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR7_A {
        match self.bits {
            false => ERR7_A::_0,
            true => ERR7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR7_A::_1
    }
}
impl core::ops::Deref for ERR7_R {
    type Target = crate::FieldReader<bool, ERR7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR7` writer - Error In Channel 7"]
pub struct ERR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR7_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR7_A::_1)
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
#[doc = "Error In Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR8_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR8_A> for bool {
    #[inline(always)]
    fn from(variant: ERR8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR8` reader - Error In Channel 8"]
pub struct ERR8_R(crate::FieldReader<bool, ERR8_A>);
impl ERR8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR8_A {
        match self.bits {
            false => ERR8_A::_0,
            true => ERR8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR8_A::_1
    }
}
impl core::ops::Deref for ERR8_R {
    type Target = crate::FieldReader<bool, ERR8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR8` writer - Error In Channel 8"]
pub struct ERR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR8_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR8_A::_1)
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
#[doc = "Error In Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR9_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR9_A> for bool {
    #[inline(always)]
    fn from(variant: ERR9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR9` reader - Error In Channel 9"]
pub struct ERR9_R(crate::FieldReader<bool, ERR9_A>);
impl ERR9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR9_A {
        match self.bits {
            false => ERR9_A::_0,
            true => ERR9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR9_A::_1
    }
}
impl core::ops::Deref for ERR9_R {
    type Target = crate::FieldReader<bool, ERR9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR9` writer - Error In Channel 9"]
pub struct ERR9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR9_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR9_A::_1)
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
#[doc = "Error In Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR10_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR10_A> for bool {
    #[inline(always)]
    fn from(variant: ERR10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR10` reader - Error In Channel 10"]
pub struct ERR10_R(crate::FieldReader<bool, ERR10_A>);
impl ERR10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR10_A {
        match self.bits {
            false => ERR10_A::_0,
            true => ERR10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR10_A::_1
    }
}
impl core::ops::Deref for ERR10_R {
    type Target = crate::FieldReader<bool, ERR10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR10` writer - Error In Channel 10"]
pub struct ERR10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR10_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR10_A::_1)
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
#[doc = "Error In Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR11_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR11_A> for bool {
    #[inline(always)]
    fn from(variant: ERR11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR11` reader - Error In Channel 11"]
pub struct ERR11_R(crate::FieldReader<bool, ERR11_A>);
impl ERR11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR11_A {
        match self.bits {
            false => ERR11_A::_0,
            true => ERR11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR11_A::_1
    }
}
impl core::ops::Deref for ERR11_R {
    type Target = crate::FieldReader<bool, ERR11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR11` writer - Error In Channel 11"]
pub struct ERR11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR11_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR11_A::_1)
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
#[doc = "Error In Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR12_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR12_A> for bool {
    #[inline(always)]
    fn from(variant: ERR12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR12` reader - Error In Channel 12"]
pub struct ERR12_R(crate::FieldReader<bool, ERR12_A>);
impl ERR12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR12_A {
        match self.bits {
            false => ERR12_A::_0,
            true => ERR12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR12_A::_1
    }
}
impl core::ops::Deref for ERR12_R {
    type Target = crate::FieldReader<bool, ERR12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR12` writer - Error In Channel 12"]
pub struct ERR12_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR12_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR12_A::_1)
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
#[doc = "Error In Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR13_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR13_A> for bool {
    #[inline(always)]
    fn from(variant: ERR13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR13` reader - Error In Channel 13"]
pub struct ERR13_R(crate::FieldReader<bool, ERR13_A>);
impl ERR13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR13_A {
        match self.bits {
            false => ERR13_A::_0,
            true => ERR13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR13_A::_1
    }
}
impl core::ops::Deref for ERR13_R {
    type Target = crate::FieldReader<bool, ERR13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR13` writer - Error In Channel 13"]
pub struct ERR13_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR13_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR13_A::_1)
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
#[doc = "Error In Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR14_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR14_A> for bool {
    #[inline(always)]
    fn from(variant: ERR14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR14` reader - Error In Channel 14"]
pub struct ERR14_R(crate::FieldReader<bool, ERR14_A>);
impl ERR14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR14_A {
        match self.bits {
            false => ERR14_A::_0,
            true => ERR14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR14_A::_1
    }
}
impl core::ops::Deref for ERR14_R {
    type Target = crate::FieldReader<bool, ERR14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR14` writer - Error In Channel 14"]
pub struct ERR14_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR14_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR14_A::_1)
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
#[doc = "Error In Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR15_A {
    #[doc = "0: An error in this channel has not occurred"]
    _0 = 0,
    #[doc = "1: An error in this channel has occurred"]
    _1 = 1,
}
impl From<ERR15_A> for bool {
    #[inline(always)]
    fn from(variant: ERR15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR15` reader - Error In Channel 15"]
pub struct ERR15_R(crate::FieldReader<bool, ERR15_A>);
impl ERR15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERR15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR15_A {
        match self.bits {
            false => ERR15_A::_0,
            true => ERR15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ERR15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ERR15_A::_1
    }
}
impl core::ops::Deref for ERR15_R {
    type Target = crate::FieldReader<bool, ERR15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR15` writer - Error In Channel 15"]
pub struct ERR15_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "An error in this channel has not occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR15_A::_0)
    }
    #[doc = "An error in this channel has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR15_A::_1)
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
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    pub fn err8(&self) -> ERR8_R {
        ERR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    pub fn err9(&self) -> ERR9_R {
        ERR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    pub fn err10(&self) -> ERR10_R {
        ERR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    pub fn err11(&self) -> ERR11_R {
        ERR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    pub fn err12(&self) -> ERR12_R {
        ERR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    pub fn err13(&self) -> ERR13_R {
        ERR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    pub fn err14(&self) -> ERR14_R {
        ERR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    pub fn err15(&self) -> ERR15_R {
        ERR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error In Channel 0"]
    #[inline(always)]
    pub fn err0(&mut self) -> ERR0_W {
        ERR0_W { w: self }
    }
    #[doc = "Bit 1 - Error In Channel 1"]
    #[inline(always)]
    pub fn err1(&mut self) -> ERR1_W {
        ERR1_W { w: self }
    }
    #[doc = "Bit 2 - Error In Channel 2"]
    #[inline(always)]
    pub fn err2(&mut self) -> ERR2_W {
        ERR2_W { w: self }
    }
    #[doc = "Bit 3 - Error In Channel 3"]
    #[inline(always)]
    pub fn err3(&mut self) -> ERR3_W {
        ERR3_W { w: self }
    }
    #[doc = "Bit 4 - Error In Channel 4"]
    #[inline(always)]
    pub fn err4(&mut self) -> ERR4_W {
        ERR4_W { w: self }
    }
    #[doc = "Bit 5 - Error In Channel 5"]
    #[inline(always)]
    pub fn err5(&mut self) -> ERR5_W {
        ERR5_W { w: self }
    }
    #[doc = "Bit 6 - Error In Channel 6"]
    #[inline(always)]
    pub fn err6(&mut self) -> ERR6_W {
        ERR6_W { w: self }
    }
    #[doc = "Bit 7 - Error In Channel 7"]
    #[inline(always)]
    pub fn err7(&mut self) -> ERR7_W {
        ERR7_W { w: self }
    }
    #[doc = "Bit 8 - Error In Channel 8"]
    #[inline(always)]
    pub fn err8(&mut self) -> ERR8_W {
        ERR8_W { w: self }
    }
    #[doc = "Bit 9 - Error In Channel 9"]
    #[inline(always)]
    pub fn err9(&mut self) -> ERR9_W {
        ERR9_W { w: self }
    }
    #[doc = "Bit 10 - Error In Channel 10"]
    #[inline(always)]
    pub fn err10(&mut self) -> ERR10_W {
        ERR10_W { w: self }
    }
    #[doc = "Bit 11 - Error In Channel 11"]
    #[inline(always)]
    pub fn err11(&mut self) -> ERR11_W {
        ERR11_W { w: self }
    }
    #[doc = "Bit 12 - Error In Channel 12"]
    #[inline(always)]
    pub fn err12(&mut self) -> ERR12_W {
        ERR12_W { w: self }
    }
    #[doc = "Bit 13 - Error In Channel 13"]
    #[inline(always)]
    pub fn err13(&mut self) -> ERR13_W {
        ERR13_W { w: self }
    }
    #[doc = "Bit 14 - Error In Channel 14"]
    #[inline(always)]
    pub fn err14(&mut self) -> ERR14_W {
        ERR14_W { w: self }
    }
    #[doc = "Bit 15 - Error In Channel 15"]
    #[inline(always)]
    pub fn err15(&mut self) -> ERR15_W {
        ERR15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](index.html) module"]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [err::R](R) reader structure"]
impl crate::Readable for ERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [err::W](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
