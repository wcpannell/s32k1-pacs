#[doc = "Register `INT` reader"]
pub struct R(crate::R<INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT` writer"]
pub struct W(crate::W<INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SPEC>;
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
impl From<crate::W<INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT0_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT0_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT0_1 = 1,
}
impl From<INT0_A> for bool {
    #[inline(always)]
    fn from(variant: INT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT0` reader - Interrupt Request 0"]
pub struct INT0_R(crate::FieldReader<bool, INT0_A>);
impl INT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT0_A {
        match self.bits {
            false => INT0_A::INT0_0,
            true => INT0_A::INT0_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT0_0`"]
    #[inline(always)]
    pub fn is_int0_0(&self) -> bool {
        **self == INT0_A::INT0_0
    }
    #[doc = "Checks if the value of the field is `INT0_1`"]
    #[inline(always)]
    pub fn is_int0_1(&self) -> bool {
        **self == INT0_A::INT0_1
    }
}
impl core::ops::Deref for INT0_R {
    type Target = crate::FieldReader<bool, INT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0` writer - Interrupt Request 0"]
pub struct INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int0_0(self) -> &'a mut W {
        self.variant(INT0_A::INT0_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int0_1(self) -> &'a mut W {
        self.variant(INT0_A::INT0_1)
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
#[doc = "Interrupt Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT1_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT1_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT1_1 = 1,
}
impl From<INT1_A> for bool {
    #[inline(always)]
    fn from(variant: INT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT1` reader - Interrupt Request 1"]
pub struct INT1_R(crate::FieldReader<bool, INT1_A>);
impl INT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT1_A {
        match self.bits {
            false => INT1_A::INT1_0,
            true => INT1_A::INT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT1_0`"]
    #[inline(always)]
    pub fn is_int1_0(&self) -> bool {
        **self == INT1_A::INT1_0
    }
    #[doc = "Checks if the value of the field is `INT1_1`"]
    #[inline(always)]
    pub fn is_int1_1(&self) -> bool {
        **self == INT1_A::INT1_1
    }
}
impl core::ops::Deref for INT1_R {
    type Target = crate::FieldReader<bool, INT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1` writer - Interrupt Request 1"]
pub struct INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int1_0(self) -> &'a mut W {
        self.variant(INT1_A::INT1_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int1_1(self) -> &'a mut W {
        self.variant(INT1_A::INT1_1)
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
#[doc = "Interrupt Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT2_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT2_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT2_1 = 1,
}
impl From<INT2_A> for bool {
    #[inline(always)]
    fn from(variant: INT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT2` reader - Interrupt Request 2"]
pub struct INT2_R(crate::FieldReader<bool, INT2_A>);
impl INT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT2_A {
        match self.bits {
            false => INT2_A::INT2_0,
            true => INT2_A::INT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT2_0`"]
    #[inline(always)]
    pub fn is_int2_0(&self) -> bool {
        **self == INT2_A::INT2_0
    }
    #[doc = "Checks if the value of the field is `INT2_1`"]
    #[inline(always)]
    pub fn is_int2_1(&self) -> bool {
        **self == INT2_A::INT2_1
    }
}
impl core::ops::Deref for INT2_R {
    type Target = crate::FieldReader<bool, INT2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2` writer - Interrupt Request 2"]
pub struct INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int2_0(self) -> &'a mut W {
        self.variant(INT2_A::INT2_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int2_1(self) -> &'a mut W {
        self.variant(INT2_A::INT2_1)
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
#[doc = "Interrupt Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT3_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT3_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT3_1 = 1,
}
impl From<INT3_A> for bool {
    #[inline(always)]
    fn from(variant: INT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT3` reader - Interrupt Request 3"]
pub struct INT3_R(crate::FieldReader<bool, INT3_A>);
impl INT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT3_A {
        match self.bits {
            false => INT3_A::INT3_0,
            true => INT3_A::INT3_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT3_0`"]
    #[inline(always)]
    pub fn is_int3_0(&self) -> bool {
        **self == INT3_A::INT3_0
    }
    #[doc = "Checks if the value of the field is `INT3_1`"]
    #[inline(always)]
    pub fn is_int3_1(&self) -> bool {
        **self == INT3_A::INT3_1
    }
}
impl core::ops::Deref for INT3_R {
    type Target = crate::FieldReader<bool, INT3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3` writer - Interrupt Request 3"]
pub struct INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int3_0(self) -> &'a mut W {
        self.variant(INT3_A::INT3_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int3_1(self) -> &'a mut W {
        self.variant(INT3_A::INT3_1)
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
#[doc = "Interrupt Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT4_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT4_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT4_1 = 1,
}
impl From<INT4_A> for bool {
    #[inline(always)]
    fn from(variant: INT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT4` reader - Interrupt Request 4"]
pub struct INT4_R(crate::FieldReader<bool, INT4_A>);
impl INT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT4_A {
        match self.bits {
            false => INT4_A::INT4_0,
            true => INT4_A::INT4_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT4_0`"]
    #[inline(always)]
    pub fn is_int4_0(&self) -> bool {
        **self == INT4_A::INT4_0
    }
    #[doc = "Checks if the value of the field is `INT4_1`"]
    #[inline(always)]
    pub fn is_int4_1(&self) -> bool {
        **self == INT4_A::INT4_1
    }
}
impl core::ops::Deref for INT4_R {
    type Target = crate::FieldReader<bool, INT4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4` writer - Interrupt Request 4"]
pub struct INT4_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int4_0(self) -> &'a mut W {
        self.variant(INT4_A::INT4_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int4_1(self) -> &'a mut W {
        self.variant(INT4_A::INT4_1)
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
#[doc = "Interrupt Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT5_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT5_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT5_1 = 1,
}
impl From<INT5_A> for bool {
    #[inline(always)]
    fn from(variant: INT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT5` reader - Interrupt Request 5"]
pub struct INT5_R(crate::FieldReader<bool, INT5_A>);
impl INT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT5_A {
        match self.bits {
            false => INT5_A::INT5_0,
            true => INT5_A::INT5_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT5_0`"]
    #[inline(always)]
    pub fn is_int5_0(&self) -> bool {
        **self == INT5_A::INT5_0
    }
    #[doc = "Checks if the value of the field is `INT5_1`"]
    #[inline(always)]
    pub fn is_int5_1(&self) -> bool {
        **self == INT5_A::INT5_1
    }
}
impl core::ops::Deref for INT5_R {
    type Target = crate::FieldReader<bool, INT5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT5` writer - Interrupt Request 5"]
pub struct INT5_W<'a> {
    w: &'a mut W,
}
impl<'a> INT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int5_0(self) -> &'a mut W {
        self.variant(INT5_A::INT5_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int5_1(self) -> &'a mut W {
        self.variant(INT5_A::INT5_1)
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
#[doc = "Interrupt Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT6_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT6_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT6_1 = 1,
}
impl From<INT6_A> for bool {
    #[inline(always)]
    fn from(variant: INT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT6` reader - Interrupt Request 6"]
pub struct INT6_R(crate::FieldReader<bool, INT6_A>);
impl INT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT6_A {
        match self.bits {
            false => INT6_A::INT6_0,
            true => INT6_A::INT6_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT6_0`"]
    #[inline(always)]
    pub fn is_int6_0(&self) -> bool {
        **self == INT6_A::INT6_0
    }
    #[doc = "Checks if the value of the field is `INT6_1`"]
    #[inline(always)]
    pub fn is_int6_1(&self) -> bool {
        **self == INT6_A::INT6_1
    }
}
impl core::ops::Deref for INT6_R {
    type Target = crate::FieldReader<bool, INT6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT6` writer - Interrupt Request 6"]
pub struct INT6_W<'a> {
    w: &'a mut W,
}
impl<'a> INT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int6_0(self) -> &'a mut W {
        self.variant(INT6_A::INT6_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int6_1(self) -> &'a mut W {
        self.variant(INT6_A::INT6_1)
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
#[doc = "Interrupt Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT7_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT7_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT7_1 = 1,
}
impl From<INT7_A> for bool {
    #[inline(always)]
    fn from(variant: INT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT7` reader - Interrupt Request 7"]
pub struct INT7_R(crate::FieldReader<bool, INT7_A>);
impl INT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT7_A {
        match self.bits {
            false => INT7_A::INT7_0,
            true => INT7_A::INT7_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT7_0`"]
    #[inline(always)]
    pub fn is_int7_0(&self) -> bool {
        **self == INT7_A::INT7_0
    }
    #[doc = "Checks if the value of the field is `INT7_1`"]
    #[inline(always)]
    pub fn is_int7_1(&self) -> bool {
        **self == INT7_A::INT7_1
    }
}
impl core::ops::Deref for INT7_R {
    type Target = crate::FieldReader<bool, INT7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT7` writer - Interrupt Request 7"]
pub struct INT7_W<'a> {
    w: &'a mut W,
}
impl<'a> INT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int7_0(self) -> &'a mut W {
        self.variant(INT7_A::INT7_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int7_1(self) -> &'a mut W {
        self.variant(INT7_A::INT7_1)
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
#[doc = "Interrupt Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT8_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT8_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT8_1 = 1,
}
impl From<INT8_A> for bool {
    #[inline(always)]
    fn from(variant: INT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT8` reader - Interrupt Request 8"]
pub struct INT8_R(crate::FieldReader<bool, INT8_A>);
impl INT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT8_A {
        match self.bits {
            false => INT8_A::INT8_0,
            true => INT8_A::INT8_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT8_0`"]
    #[inline(always)]
    pub fn is_int8_0(&self) -> bool {
        **self == INT8_A::INT8_0
    }
    #[doc = "Checks if the value of the field is `INT8_1`"]
    #[inline(always)]
    pub fn is_int8_1(&self) -> bool {
        **self == INT8_A::INT8_1
    }
}
impl core::ops::Deref for INT8_R {
    type Target = crate::FieldReader<bool, INT8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT8` writer - Interrupt Request 8"]
pub struct INT8_W<'a> {
    w: &'a mut W,
}
impl<'a> INT8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int8_0(self) -> &'a mut W {
        self.variant(INT8_A::INT8_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int8_1(self) -> &'a mut W {
        self.variant(INT8_A::INT8_1)
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
#[doc = "Interrupt Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT9_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT9_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT9_1 = 1,
}
impl From<INT9_A> for bool {
    #[inline(always)]
    fn from(variant: INT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT9` reader - Interrupt Request 9"]
pub struct INT9_R(crate::FieldReader<bool, INT9_A>);
impl INT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT9_A {
        match self.bits {
            false => INT9_A::INT9_0,
            true => INT9_A::INT9_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT9_0`"]
    #[inline(always)]
    pub fn is_int9_0(&self) -> bool {
        **self == INT9_A::INT9_0
    }
    #[doc = "Checks if the value of the field is `INT9_1`"]
    #[inline(always)]
    pub fn is_int9_1(&self) -> bool {
        **self == INT9_A::INT9_1
    }
}
impl core::ops::Deref for INT9_R {
    type Target = crate::FieldReader<bool, INT9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT9` writer - Interrupt Request 9"]
pub struct INT9_W<'a> {
    w: &'a mut W,
}
impl<'a> INT9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int9_0(self) -> &'a mut W {
        self.variant(INT9_A::INT9_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int9_1(self) -> &'a mut W {
        self.variant(INT9_A::INT9_1)
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
#[doc = "Interrupt Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT10_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT10_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT10_1 = 1,
}
impl From<INT10_A> for bool {
    #[inline(always)]
    fn from(variant: INT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT10` reader - Interrupt Request 10"]
pub struct INT10_R(crate::FieldReader<bool, INT10_A>);
impl INT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT10_A {
        match self.bits {
            false => INT10_A::INT10_0,
            true => INT10_A::INT10_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT10_0`"]
    #[inline(always)]
    pub fn is_int10_0(&self) -> bool {
        **self == INT10_A::INT10_0
    }
    #[doc = "Checks if the value of the field is `INT10_1`"]
    #[inline(always)]
    pub fn is_int10_1(&self) -> bool {
        **self == INT10_A::INT10_1
    }
}
impl core::ops::Deref for INT10_R {
    type Target = crate::FieldReader<bool, INT10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT10` writer - Interrupt Request 10"]
pub struct INT10_W<'a> {
    w: &'a mut W,
}
impl<'a> INT10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int10_0(self) -> &'a mut W {
        self.variant(INT10_A::INT10_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int10_1(self) -> &'a mut W {
        self.variant(INT10_A::INT10_1)
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
#[doc = "Interrupt Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT11_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT11_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT11_1 = 1,
}
impl From<INT11_A> for bool {
    #[inline(always)]
    fn from(variant: INT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT11` reader - Interrupt Request 11"]
pub struct INT11_R(crate::FieldReader<bool, INT11_A>);
impl INT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT11_A {
        match self.bits {
            false => INT11_A::INT11_0,
            true => INT11_A::INT11_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT11_0`"]
    #[inline(always)]
    pub fn is_int11_0(&self) -> bool {
        **self == INT11_A::INT11_0
    }
    #[doc = "Checks if the value of the field is `INT11_1`"]
    #[inline(always)]
    pub fn is_int11_1(&self) -> bool {
        **self == INT11_A::INT11_1
    }
}
impl core::ops::Deref for INT11_R {
    type Target = crate::FieldReader<bool, INT11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT11` writer - Interrupt Request 11"]
pub struct INT11_W<'a> {
    w: &'a mut W,
}
impl<'a> INT11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int11_0(self) -> &'a mut W {
        self.variant(INT11_A::INT11_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int11_1(self) -> &'a mut W {
        self.variant(INT11_A::INT11_1)
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
#[doc = "Interrupt Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT12_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT12_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT12_1 = 1,
}
impl From<INT12_A> for bool {
    #[inline(always)]
    fn from(variant: INT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT12` reader - Interrupt Request 12"]
pub struct INT12_R(crate::FieldReader<bool, INT12_A>);
impl INT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT12_A {
        match self.bits {
            false => INT12_A::INT12_0,
            true => INT12_A::INT12_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT12_0`"]
    #[inline(always)]
    pub fn is_int12_0(&self) -> bool {
        **self == INT12_A::INT12_0
    }
    #[doc = "Checks if the value of the field is `INT12_1`"]
    #[inline(always)]
    pub fn is_int12_1(&self) -> bool {
        **self == INT12_A::INT12_1
    }
}
impl core::ops::Deref for INT12_R {
    type Target = crate::FieldReader<bool, INT12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT12` writer - Interrupt Request 12"]
pub struct INT12_W<'a> {
    w: &'a mut W,
}
impl<'a> INT12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int12_0(self) -> &'a mut W {
        self.variant(INT12_A::INT12_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int12_1(self) -> &'a mut W {
        self.variant(INT12_A::INT12_1)
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
#[doc = "Interrupt Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT13_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT13_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT13_1 = 1,
}
impl From<INT13_A> for bool {
    #[inline(always)]
    fn from(variant: INT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT13` reader - Interrupt Request 13"]
pub struct INT13_R(crate::FieldReader<bool, INT13_A>);
impl INT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT13_A {
        match self.bits {
            false => INT13_A::INT13_0,
            true => INT13_A::INT13_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT13_0`"]
    #[inline(always)]
    pub fn is_int13_0(&self) -> bool {
        **self == INT13_A::INT13_0
    }
    #[doc = "Checks if the value of the field is `INT13_1`"]
    #[inline(always)]
    pub fn is_int13_1(&self) -> bool {
        **self == INT13_A::INT13_1
    }
}
impl core::ops::Deref for INT13_R {
    type Target = crate::FieldReader<bool, INT13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT13` writer - Interrupt Request 13"]
pub struct INT13_W<'a> {
    w: &'a mut W,
}
impl<'a> INT13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int13_0(self) -> &'a mut W {
        self.variant(INT13_A::INT13_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int13_1(self) -> &'a mut W {
        self.variant(INT13_A::INT13_1)
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
#[doc = "Interrupt Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT14_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT14_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT14_1 = 1,
}
impl From<INT14_A> for bool {
    #[inline(always)]
    fn from(variant: INT14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT14` reader - Interrupt Request 14"]
pub struct INT14_R(crate::FieldReader<bool, INT14_A>);
impl INT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT14_A {
        match self.bits {
            false => INT14_A::INT14_0,
            true => INT14_A::INT14_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT14_0`"]
    #[inline(always)]
    pub fn is_int14_0(&self) -> bool {
        **self == INT14_A::INT14_0
    }
    #[doc = "Checks if the value of the field is `INT14_1`"]
    #[inline(always)]
    pub fn is_int14_1(&self) -> bool {
        **self == INT14_A::INT14_1
    }
}
impl core::ops::Deref for INT14_R {
    type Target = crate::FieldReader<bool, INT14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT14` writer - Interrupt Request 14"]
pub struct INT14_W<'a> {
    w: &'a mut W,
}
impl<'a> INT14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int14_0(self) -> &'a mut W {
        self.variant(INT14_A::INT14_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int14_1(self) -> &'a mut W {
        self.variant(INT14_A::INT14_1)
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
#[doc = "Interrupt Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT15_A {
    #[doc = "0: The interrupt request for corresponding channel is cleared"]
    INT15_0 = 0,
    #[doc = "1: The interrupt request for corresponding channel is active"]
    INT15_1 = 1,
}
impl From<INT15_A> for bool {
    #[inline(always)]
    fn from(variant: INT15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT15` reader - Interrupt Request 15"]
pub struct INT15_R(crate::FieldReader<bool, INT15_A>);
impl INT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT15_A {
        match self.bits {
            false => INT15_A::INT15_0,
            true => INT15_A::INT15_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT15_0`"]
    #[inline(always)]
    pub fn is_int15_0(&self) -> bool {
        **self == INT15_A::INT15_0
    }
    #[doc = "Checks if the value of the field is `INT15_1`"]
    #[inline(always)]
    pub fn is_int15_1(&self) -> bool {
        **self == INT15_A::INT15_1
    }
}
impl core::ops::Deref for INT15_R {
    type Target = crate::FieldReader<bool, INT15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT15` writer - Interrupt Request 15"]
pub struct INT15_W<'a> {
    w: &'a mut W,
}
impl<'a> INT15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INT15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The interrupt request for corresponding channel is cleared"]
    #[inline(always)]
    pub fn int15_0(self) -> &'a mut W {
        self.variant(INT15_A::INT15_0)
    }
    #[doc = "The interrupt request for corresponding channel is active"]
    #[inline(always)]
    pub fn int15_1(self) -> &'a mut W {
        self.variant(INT15_A::INT15_1)
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
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Request 0"]
    #[inline(always)]
    pub fn int0(&mut self) -> INT0_W {
        INT0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt Request 1"]
    #[inline(always)]
    pub fn int1(&mut self) -> INT1_W {
        INT1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt Request 2"]
    #[inline(always)]
    pub fn int2(&mut self) -> INT2_W {
        INT2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Request 3"]
    #[inline(always)]
    pub fn int3(&mut self) -> INT3_W {
        INT3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Request 4"]
    #[inline(always)]
    pub fn int4(&mut self) -> INT4_W {
        INT4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Request 5"]
    #[inline(always)]
    pub fn int5(&mut self) -> INT5_W {
        INT5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Request 6"]
    #[inline(always)]
    pub fn int6(&mut self) -> INT6_W {
        INT6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Request 7"]
    #[inline(always)]
    pub fn int7(&mut self) -> INT7_W {
        INT7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt Request 8"]
    #[inline(always)]
    pub fn int8(&mut self) -> INT8_W {
        INT8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt Request 9"]
    #[inline(always)]
    pub fn int9(&mut self) -> INT9_W {
        INT9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt Request 10"]
    #[inline(always)]
    pub fn int10(&mut self) -> INT10_W {
        INT10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt Request 11"]
    #[inline(always)]
    pub fn int11(&mut self) -> INT11_W {
        INT11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt Request 12"]
    #[inline(always)]
    pub fn int12(&mut self) -> INT12_W {
        INT12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt Request 13"]
    #[inline(always)]
    pub fn int13(&mut self) -> INT13_W {
        INT13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt Request 14"]
    #[inline(always)]
    pub fn int14(&mut self) -> INT14_W {
        INT14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt Request 15"]
    #[inline(always)]
    pub fn int15(&mut self) -> INT15_W {
        INT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](index.html) module"]
pub struct INT_SPEC;
impl crate::RegisterSpec for INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int::R](R) reader structure"]
impl crate::Readable for INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int::W](W) writer structure"]
impl crate::Writable for INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT to value 0"]
impl crate::Resettable for INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
