#[doc = "Register `ERQ` reader"]
pub struct R(crate::R<ERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERQ` writer"]
pub struct W(crate::W<ERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERQ_SPEC>;
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
impl From<crate::W<ERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ0_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ0_1 = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ0` reader - Enable DMA Request 0"]
pub struct ERQ0_R(crate::FieldReader<bool, ERQ0_A>);
impl ERQ0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::ERQ0_0,
            true => ERQ0_A::ERQ0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ0_0`"]
    #[inline(always)]
    pub fn is_erq0_0(&self) -> bool {
        **self == ERQ0_A::ERQ0_0
    }
    #[doc = "Checks if the value of the field is `ERQ0_1`"]
    #[inline(always)]
    pub fn is_erq0_1(&self) -> bool {
        **self == ERQ0_A::ERQ0_1
    }
}
impl core::ops::Deref for ERQ0_R {
    type Target = crate::FieldReader<bool, ERQ0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ0` writer - Enable DMA Request 0"]
pub struct ERQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq0_0(self) -> &'a mut W {
        self.variant(ERQ0_A::ERQ0_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq0_1(self) -> &'a mut W {
        self.variant(ERQ0_A::ERQ0_1)
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
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ1_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ1_1 = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ1` reader - Enable DMA Request 1"]
pub struct ERQ1_R(crate::FieldReader<bool, ERQ1_A>);
impl ERQ1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::ERQ1_0,
            true => ERQ1_A::ERQ1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ1_0`"]
    #[inline(always)]
    pub fn is_erq1_0(&self) -> bool {
        **self == ERQ1_A::ERQ1_0
    }
    #[doc = "Checks if the value of the field is `ERQ1_1`"]
    #[inline(always)]
    pub fn is_erq1_1(&self) -> bool {
        **self == ERQ1_A::ERQ1_1
    }
}
impl core::ops::Deref for ERQ1_R {
    type Target = crate::FieldReader<bool, ERQ1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ1` writer - Enable DMA Request 1"]
pub struct ERQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq1_0(self) -> &'a mut W {
        self.variant(ERQ1_A::ERQ1_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq1_1(self) -> &'a mut W {
        self.variant(ERQ1_A::ERQ1_1)
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
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ2_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ2_1 = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ2` reader - Enable DMA Request 2"]
pub struct ERQ2_R(crate::FieldReader<bool, ERQ2_A>);
impl ERQ2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::ERQ2_0,
            true => ERQ2_A::ERQ2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ2_0`"]
    #[inline(always)]
    pub fn is_erq2_0(&self) -> bool {
        **self == ERQ2_A::ERQ2_0
    }
    #[doc = "Checks if the value of the field is `ERQ2_1`"]
    #[inline(always)]
    pub fn is_erq2_1(&self) -> bool {
        **self == ERQ2_A::ERQ2_1
    }
}
impl core::ops::Deref for ERQ2_R {
    type Target = crate::FieldReader<bool, ERQ2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ2` writer - Enable DMA Request 2"]
pub struct ERQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq2_0(self) -> &'a mut W {
        self.variant(ERQ2_A::ERQ2_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq2_1(self) -> &'a mut W {
        self.variant(ERQ2_A::ERQ2_1)
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
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ3_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ3_1 = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ3` reader - Enable DMA Request 3"]
pub struct ERQ3_R(crate::FieldReader<bool, ERQ3_A>);
impl ERQ3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::ERQ3_0,
            true => ERQ3_A::ERQ3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ3_0`"]
    #[inline(always)]
    pub fn is_erq3_0(&self) -> bool {
        **self == ERQ3_A::ERQ3_0
    }
    #[doc = "Checks if the value of the field is `ERQ3_1`"]
    #[inline(always)]
    pub fn is_erq3_1(&self) -> bool {
        **self == ERQ3_A::ERQ3_1
    }
}
impl core::ops::Deref for ERQ3_R {
    type Target = crate::FieldReader<bool, ERQ3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ3` writer - Enable DMA Request 3"]
pub struct ERQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq3_0(self) -> &'a mut W {
        self.variant(ERQ3_A::ERQ3_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq3_1(self) -> &'a mut W {
        self.variant(ERQ3_A::ERQ3_1)
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
#[doc = "Enable DMA Request 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ4_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ4_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ4_1 = 1,
}
impl From<ERQ4_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ4` reader - Enable DMA Request 4"]
pub struct ERQ4_R(crate::FieldReader<bool, ERQ4_A>);
impl ERQ4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ4_A {
        match self.bits {
            false => ERQ4_A::ERQ4_0,
            true => ERQ4_A::ERQ4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ4_0`"]
    #[inline(always)]
    pub fn is_erq4_0(&self) -> bool {
        **self == ERQ4_A::ERQ4_0
    }
    #[doc = "Checks if the value of the field is `ERQ4_1`"]
    #[inline(always)]
    pub fn is_erq4_1(&self) -> bool {
        **self == ERQ4_A::ERQ4_1
    }
}
impl core::ops::Deref for ERQ4_R {
    type Target = crate::FieldReader<bool, ERQ4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ4` writer - Enable DMA Request 4"]
pub struct ERQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq4_0(self) -> &'a mut W {
        self.variant(ERQ4_A::ERQ4_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq4_1(self) -> &'a mut W {
        self.variant(ERQ4_A::ERQ4_1)
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
#[doc = "Enable DMA Request 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ5_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ5_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ5_1 = 1,
}
impl From<ERQ5_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ5` reader - Enable DMA Request 5"]
pub struct ERQ5_R(crate::FieldReader<bool, ERQ5_A>);
impl ERQ5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ5_A {
        match self.bits {
            false => ERQ5_A::ERQ5_0,
            true => ERQ5_A::ERQ5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ5_0`"]
    #[inline(always)]
    pub fn is_erq5_0(&self) -> bool {
        **self == ERQ5_A::ERQ5_0
    }
    #[doc = "Checks if the value of the field is `ERQ5_1`"]
    #[inline(always)]
    pub fn is_erq5_1(&self) -> bool {
        **self == ERQ5_A::ERQ5_1
    }
}
impl core::ops::Deref for ERQ5_R {
    type Target = crate::FieldReader<bool, ERQ5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ5` writer - Enable DMA Request 5"]
pub struct ERQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq5_0(self) -> &'a mut W {
        self.variant(ERQ5_A::ERQ5_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq5_1(self) -> &'a mut W {
        self.variant(ERQ5_A::ERQ5_1)
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
#[doc = "Enable DMA Request 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ6_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ6_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ6_1 = 1,
}
impl From<ERQ6_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ6` reader - Enable DMA Request 6"]
pub struct ERQ6_R(crate::FieldReader<bool, ERQ6_A>);
impl ERQ6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ6_A {
        match self.bits {
            false => ERQ6_A::ERQ6_0,
            true => ERQ6_A::ERQ6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ6_0`"]
    #[inline(always)]
    pub fn is_erq6_0(&self) -> bool {
        **self == ERQ6_A::ERQ6_0
    }
    #[doc = "Checks if the value of the field is `ERQ6_1`"]
    #[inline(always)]
    pub fn is_erq6_1(&self) -> bool {
        **self == ERQ6_A::ERQ6_1
    }
}
impl core::ops::Deref for ERQ6_R {
    type Target = crate::FieldReader<bool, ERQ6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ6` writer - Enable DMA Request 6"]
pub struct ERQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq6_0(self) -> &'a mut W {
        self.variant(ERQ6_A::ERQ6_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq6_1(self) -> &'a mut W {
        self.variant(ERQ6_A::ERQ6_1)
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
#[doc = "Enable DMA Request 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ7_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ7_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ7_1 = 1,
}
impl From<ERQ7_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ7` reader - Enable DMA Request 7"]
pub struct ERQ7_R(crate::FieldReader<bool, ERQ7_A>);
impl ERQ7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ7_A {
        match self.bits {
            false => ERQ7_A::ERQ7_0,
            true => ERQ7_A::ERQ7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ7_0`"]
    #[inline(always)]
    pub fn is_erq7_0(&self) -> bool {
        **self == ERQ7_A::ERQ7_0
    }
    #[doc = "Checks if the value of the field is `ERQ7_1`"]
    #[inline(always)]
    pub fn is_erq7_1(&self) -> bool {
        **self == ERQ7_A::ERQ7_1
    }
}
impl core::ops::Deref for ERQ7_R {
    type Target = crate::FieldReader<bool, ERQ7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ7` writer - Enable DMA Request 7"]
pub struct ERQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq7_0(self) -> &'a mut W {
        self.variant(ERQ7_A::ERQ7_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq7_1(self) -> &'a mut W {
        self.variant(ERQ7_A::ERQ7_1)
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
#[doc = "Enable DMA Request 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ8_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ8_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ8_1 = 1,
}
impl From<ERQ8_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ8` reader - Enable DMA Request 8"]
pub struct ERQ8_R(crate::FieldReader<bool, ERQ8_A>);
impl ERQ8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ8_A {
        match self.bits {
            false => ERQ8_A::ERQ8_0,
            true => ERQ8_A::ERQ8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ8_0`"]
    #[inline(always)]
    pub fn is_erq8_0(&self) -> bool {
        **self == ERQ8_A::ERQ8_0
    }
    #[doc = "Checks if the value of the field is `ERQ8_1`"]
    #[inline(always)]
    pub fn is_erq8_1(&self) -> bool {
        **self == ERQ8_A::ERQ8_1
    }
}
impl core::ops::Deref for ERQ8_R {
    type Target = crate::FieldReader<bool, ERQ8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ8` writer - Enable DMA Request 8"]
pub struct ERQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq8_0(self) -> &'a mut W {
        self.variant(ERQ8_A::ERQ8_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq8_1(self) -> &'a mut W {
        self.variant(ERQ8_A::ERQ8_1)
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
#[doc = "Enable DMA Request 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ9_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ9_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ9_1 = 1,
}
impl From<ERQ9_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ9` reader - Enable DMA Request 9"]
pub struct ERQ9_R(crate::FieldReader<bool, ERQ9_A>);
impl ERQ9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ9_A {
        match self.bits {
            false => ERQ9_A::ERQ9_0,
            true => ERQ9_A::ERQ9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ9_0`"]
    #[inline(always)]
    pub fn is_erq9_0(&self) -> bool {
        **self == ERQ9_A::ERQ9_0
    }
    #[doc = "Checks if the value of the field is `ERQ9_1`"]
    #[inline(always)]
    pub fn is_erq9_1(&self) -> bool {
        **self == ERQ9_A::ERQ9_1
    }
}
impl core::ops::Deref for ERQ9_R {
    type Target = crate::FieldReader<bool, ERQ9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ9` writer - Enable DMA Request 9"]
pub struct ERQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq9_0(self) -> &'a mut W {
        self.variant(ERQ9_A::ERQ9_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq9_1(self) -> &'a mut W {
        self.variant(ERQ9_A::ERQ9_1)
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
#[doc = "Enable DMA Request 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ10_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ10_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ10_1 = 1,
}
impl From<ERQ10_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ10` reader - Enable DMA Request 10"]
pub struct ERQ10_R(crate::FieldReader<bool, ERQ10_A>);
impl ERQ10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ10_A {
        match self.bits {
            false => ERQ10_A::ERQ10_0,
            true => ERQ10_A::ERQ10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ10_0`"]
    #[inline(always)]
    pub fn is_erq10_0(&self) -> bool {
        **self == ERQ10_A::ERQ10_0
    }
    #[doc = "Checks if the value of the field is `ERQ10_1`"]
    #[inline(always)]
    pub fn is_erq10_1(&self) -> bool {
        **self == ERQ10_A::ERQ10_1
    }
}
impl core::ops::Deref for ERQ10_R {
    type Target = crate::FieldReader<bool, ERQ10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ10` writer - Enable DMA Request 10"]
pub struct ERQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq10_0(self) -> &'a mut W {
        self.variant(ERQ10_A::ERQ10_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq10_1(self) -> &'a mut W {
        self.variant(ERQ10_A::ERQ10_1)
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
#[doc = "Enable DMA Request 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ11_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ11_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ11_1 = 1,
}
impl From<ERQ11_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ11` reader - Enable DMA Request 11"]
pub struct ERQ11_R(crate::FieldReader<bool, ERQ11_A>);
impl ERQ11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ11_A {
        match self.bits {
            false => ERQ11_A::ERQ11_0,
            true => ERQ11_A::ERQ11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ11_0`"]
    #[inline(always)]
    pub fn is_erq11_0(&self) -> bool {
        **self == ERQ11_A::ERQ11_0
    }
    #[doc = "Checks if the value of the field is `ERQ11_1`"]
    #[inline(always)]
    pub fn is_erq11_1(&self) -> bool {
        **self == ERQ11_A::ERQ11_1
    }
}
impl core::ops::Deref for ERQ11_R {
    type Target = crate::FieldReader<bool, ERQ11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ11` writer - Enable DMA Request 11"]
pub struct ERQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq11_0(self) -> &'a mut W {
        self.variant(ERQ11_A::ERQ11_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq11_1(self) -> &'a mut W {
        self.variant(ERQ11_A::ERQ11_1)
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
#[doc = "Enable DMA Request 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ12_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ12_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ12_1 = 1,
}
impl From<ERQ12_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ12` reader - Enable DMA Request 12"]
pub struct ERQ12_R(crate::FieldReader<bool, ERQ12_A>);
impl ERQ12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ12_A {
        match self.bits {
            false => ERQ12_A::ERQ12_0,
            true => ERQ12_A::ERQ12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ12_0`"]
    #[inline(always)]
    pub fn is_erq12_0(&self) -> bool {
        **self == ERQ12_A::ERQ12_0
    }
    #[doc = "Checks if the value of the field is `ERQ12_1`"]
    #[inline(always)]
    pub fn is_erq12_1(&self) -> bool {
        **self == ERQ12_A::ERQ12_1
    }
}
impl core::ops::Deref for ERQ12_R {
    type Target = crate::FieldReader<bool, ERQ12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ12` writer - Enable DMA Request 12"]
pub struct ERQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq12_0(self) -> &'a mut W {
        self.variant(ERQ12_A::ERQ12_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq12_1(self) -> &'a mut W {
        self.variant(ERQ12_A::ERQ12_1)
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
#[doc = "Enable DMA Request 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ13_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ13_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ13_1 = 1,
}
impl From<ERQ13_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ13` reader - Enable DMA Request 13"]
pub struct ERQ13_R(crate::FieldReader<bool, ERQ13_A>);
impl ERQ13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ13_A {
        match self.bits {
            false => ERQ13_A::ERQ13_0,
            true => ERQ13_A::ERQ13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ13_0`"]
    #[inline(always)]
    pub fn is_erq13_0(&self) -> bool {
        **self == ERQ13_A::ERQ13_0
    }
    #[doc = "Checks if the value of the field is `ERQ13_1`"]
    #[inline(always)]
    pub fn is_erq13_1(&self) -> bool {
        **self == ERQ13_A::ERQ13_1
    }
}
impl core::ops::Deref for ERQ13_R {
    type Target = crate::FieldReader<bool, ERQ13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ13` writer - Enable DMA Request 13"]
pub struct ERQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq13_0(self) -> &'a mut W {
        self.variant(ERQ13_A::ERQ13_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq13_1(self) -> &'a mut W {
        self.variant(ERQ13_A::ERQ13_1)
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
#[doc = "Enable DMA Request 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ14_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ14_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ14_1 = 1,
}
impl From<ERQ14_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ14` reader - Enable DMA Request 14"]
pub struct ERQ14_R(crate::FieldReader<bool, ERQ14_A>);
impl ERQ14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ14_A {
        match self.bits {
            false => ERQ14_A::ERQ14_0,
            true => ERQ14_A::ERQ14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ14_0`"]
    #[inline(always)]
    pub fn is_erq14_0(&self) -> bool {
        **self == ERQ14_A::ERQ14_0
    }
    #[doc = "Checks if the value of the field is `ERQ14_1`"]
    #[inline(always)]
    pub fn is_erq14_1(&self) -> bool {
        **self == ERQ14_A::ERQ14_1
    }
}
impl core::ops::Deref for ERQ14_R {
    type Target = crate::FieldReader<bool, ERQ14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ14` writer - Enable DMA Request 14"]
pub struct ERQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq14_0(self) -> &'a mut W {
        self.variant(ERQ14_A::ERQ14_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq14_1(self) -> &'a mut W {
        self.variant(ERQ14_A::ERQ14_1)
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
#[doc = "Enable DMA Request 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERQ15_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    ERQ15_0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    ERQ15_1 = 1,
}
impl From<ERQ15_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ15` reader - Enable DMA Request 15"]
pub struct ERQ15_R(crate::FieldReader<bool, ERQ15_A>);
impl ERQ15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERQ15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ15_A {
        match self.bits {
            false => ERQ15_A::ERQ15_0,
            true => ERQ15_A::ERQ15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERQ15_0`"]
    #[inline(always)]
    pub fn is_erq15_0(&self) -> bool {
        **self == ERQ15_A::ERQ15_0
    }
    #[doc = "Checks if the value of the field is `ERQ15_1`"]
    #[inline(always)]
    pub fn is_erq15_1(&self) -> bool {
        **self == ERQ15_A::ERQ15_1
    }
}
impl core::ops::Deref for ERQ15_R {
    type Target = crate::FieldReader<bool, ERQ15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERQ15` writer - Enable DMA Request 15"]
pub struct ERQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> ERQ15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERQ15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn erq15_0(self) -> &'a mut W {
        self.variant(ERQ15_A::ERQ15_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn erq15_1(self) -> &'a mut W {
        self.variant(ERQ15_A::ERQ15_1)
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
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&self) -> ERQ4_R {
        ERQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&self) -> ERQ5_R {
        ERQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&self) -> ERQ6_R {
        ERQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&self) -> ERQ7_R {
        ERQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&self) -> ERQ8_R {
        ERQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&self) -> ERQ9_R {
        ERQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&self) -> ERQ10_R {
        ERQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&self) -> ERQ11_R {
        ERQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&self) -> ERQ12_R {
        ERQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&self) -> ERQ13_R {
        ERQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&self) -> ERQ14_R {
        ERQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&self) -> ERQ15_R {
        ERQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&mut self) -> ERQ0_W {
        ERQ0_W { w: self }
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&mut self) -> ERQ1_W {
        ERQ1_W { w: self }
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&mut self) -> ERQ2_W {
        ERQ2_W { w: self }
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&mut self) -> ERQ3_W {
        ERQ3_W { w: self }
    }
    #[doc = "Bit 4 - Enable DMA Request 4"]
    #[inline(always)]
    pub fn erq4(&mut self) -> ERQ4_W {
        ERQ4_W { w: self }
    }
    #[doc = "Bit 5 - Enable DMA Request 5"]
    #[inline(always)]
    pub fn erq5(&mut self) -> ERQ5_W {
        ERQ5_W { w: self }
    }
    #[doc = "Bit 6 - Enable DMA Request 6"]
    #[inline(always)]
    pub fn erq6(&mut self) -> ERQ6_W {
        ERQ6_W { w: self }
    }
    #[doc = "Bit 7 - Enable DMA Request 7"]
    #[inline(always)]
    pub fn erq7(&mut self) -> ERQ7_W {
        ERQ7_W { w: self }
    }
    #[doc = "Bit 8 - Enable DMA Request 8"]
    #[inline(always)]
    pub fn erq8(&mut self) -> ERQ8_W {
        ERQ8_W { w: self }
    }
    #[doc = "Bit 9 - Enable DMA Request 9"]
    #[inline(always)]
    pub fn erq9(&mut self) -> ERQ9_W {
        ERQ9_W { w: self }
    }
    #[doc = "Bit 10 - Enable DMA Request 10"]
    #[inline(always)]
    pub fn erq10(&mut self) -> ERQ10_W {
        ERQ10_W { w: self }
    }
    #[doc = "Bit 11 - Enable DMA Request 11"]
    #[inline(always)]
    pub fn erq11(&mut self) -> ERQ11_W {
        ERQ11_W { w: self }
    }
    #[doc = "Bit 12 - Enable DMA Request 12"]
    #[inline(always)]
    pub fn erq12(&mut self) -> ERQ12_W {
        ERQ12_W { w: self }
    }
    #[doc = "Bit 13 - Enable DMA Request 13"]
    #[inline(always)]
    pub fn erq13(&mut self) -> ERQ13_W {
        ERQ13_W { w: self }
    }
    #[doc = "Bit 14 - Enable DMA Request 14"]
    #[inline(always)]
    pub fn erq14(&mut self) -> ERQ14_W {
        ERQ14_W { w: self }
    }
    #[doc = "Bit 15 - Enable DMA Request 15"]
    #[inline(always)]
    pub fn erq15(&mut self) -> ERQ15_W {
        ERQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](index.html) module"]
pub struct ERQ_SPEC;
impl crate::RegisterSpec for ERQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erq::R](R) reader structure"]
impl crate::Readable for ERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erq::W](W) writer structure"]
impl crate::Writable for ERQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERQ to value 0"]
impl crate::Resettable for ERQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
