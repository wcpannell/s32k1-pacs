#[doc = "Register `POL` reader"]
pub struct R(crate::R<POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POL` writer"]
pub struct W(crate::W<POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POL_SPEC>;
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
impl From<crate::W<POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL0_A {
    #[doc = "0: The channel polarity is active high."]
    POL0_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL0_1 = 1,
}
impl From<POL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub struct POL0_R(crate::FieldReader<bool, POL0_A>);
impl POL0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL0_A {
        match self.bits {
            false => POL0_A::POL0_0,
            true => POL0_A::POL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL0_0`"]
    #[inline(always)]
    pub fn is_pol0_0(&self) -> bool {
        **self == POL0_A::POL0_0
    }
    #[doc = "Checks if the value of the field is `POL0_1`"]
    #[inline(always)]
    pub fn is_pol0_1(&self) -> bool {
        **self == POL0_A::POL0_1
    }
}
impl core::ops::Deref for POL0_R {
    type Target = crate::FieldReader<bool, POL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub struct POL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol0_0(self) -> &'a mut W {
        self.variant(POL0_A::POL0_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol0_1(self) -> &'a mut W {
        self.variant(POL0_A::POL0_1)
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
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL1_A {
    #[doc = "0: The channel polarity is active high."]
    POL1_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL1_1 = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub struct POL1_R(crate::FieldReader<bool, POL1_A>);
impl POL1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::POL1_0,
            true => POL1_A::POL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL1_0`"]
    #[inline(always)]
    pub fn is_pol1_0(&self) -> bool {
        **self == POL1_A::POL1_0
    }
    #[doc = "Checks if the value of the field is `POL1_1`"]
    #[inline(always)]
    pub fn is_pol1_1(&self) -> bool {
        **self == POL1_A::POL1_1
    }
}
impl core::ops::Deref for POL1_R {
    type Target = crate::FieldReader<bool, POL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub struct POL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol1_0(self) -> &'a mut W {
        self.variant(POL1_A::POL1_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol1_1(self) -> &'a mut W {
        self.variant(POL1_A::POL1_1)
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
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL2_A {
    #[doc = "0: The channel polarity is active high."]
    POL2_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL2_1 = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub struct POL2_R(crate::FieldReader<bool, POL2_A>);
impl POL2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::POL2_0,
            true => POL2_A::POL2_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL2_0`"]
    #[inline(always)]
    pub fn is_pol2_0(&self) -> bool {
        **self == POL2_A::POL2_0
    }
    #[doc = "Checks if the value of the field is `POL2_1`"]
    #[inline(always)]
    pub fn is_pol2_1(&self) -> bool {
        **self == POL2_A::POL2_1
    }
}
impl core::ops::Deref for POL2_R {
    type Target = crate::FieldReader<bool, POL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub struct POL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol2_0(self) -> &'a mut W {
        self.variant(POL2_A::POL2_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol2_1(self) -> &'a mut W {
        self.variant(POL2_A::POL2_1)
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
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL3_A {
    #[doc = "0: The channel polarity is active high."]
    POL3_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL3_1 = 1,
}
impl From<POL3_A> for bool {
    #[inline(always)]
    fn from(variant: POL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub struct POL3_R(crate::FieldReader<bool, POL3_A>);
impl POL3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL3_A {
        match self.bits {
            false => POL3_A::POL3_0,
            true => POL3_A::POL3_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL3_0`"]
    #[inline(always)]
    pub fn is_pol3_0(&self) -> bool {
        **self == POL3_A::POL3_0
    }
    #[doc = "Checks if the value of the field is `POL3_1`"]
    #[inline(always)]
    pub fn is_pol3_1(&self) -> bool {
        **self == POL3_A::POL3_1
    }
}
impl core::ops::Deref for POL3_R {
    type Target = crate::FieldReader<bool, POL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub struct POL3_W<'a> {
    w: &'a mut W,
}
impl<'a> POL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol3_0(self) -> &'a mut W {
        self.variant(POL3_A::POL3_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol3_1(self) -> &'a mut W {
        self.variant(POL3_A::POL3_1)
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
#[doc = "Channel 4 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL4_A {
    #[doc = "0: The channel polarity is active high."]
    POL4_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL4_1 = 1,
}
impl From<POL4_A> for bool {
    #[inline(always)]
    fn from(variant: POL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL4` reader - Channel 4 Polarity"]
pub struct POL4_R(crate::FieldReader<bool, POL4_A>);
impl POL4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL4_A {
        match self.bits {
            false => POL4_A::POL4_0,
            true => POL4_A::POL4_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL4_0`"]
    #[inline(always)]
    pub fn is_pol4_0(&self) -> bool {
        **self == POL4_A::POL4_0
    }
    #[doc = "Checks if the value of the field is `POL4_1`"]
    #[inline(always)]
    pub fn is_pol4_1(&self) -> bool {
        **self == POL4_A::POL4_1
    }
}
impl core::ops::Deref for POL4_R {
    type Target = crate::FieldReader<bool, POL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL4` writer - Channel 4 Polarity"]
pub struct POL4_W<'a> {
    w: &'a mut W,
}
impl<'a> POL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol4_0(self) -> &'a mut W {
        self.variant(POL4_A::POL4_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol4_1(self) -> &'a mut W {
        self.variant(POL4_A::POL4_1)
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
#[doc = "Channel 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL5_A {
    #[doc = "0: The channel polarity is active high."]
    POL5_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL5_1 = 1,
}
impl From<POL5_A> for bool {
    #[inline(always)]
    fn from(variant: POL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL5` reader - Channel 5 Polarity"]
pub struct POL5_R(crate::FieldReader<bool, POL5_A>);
impl POL5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL5_A {
        match self.bits {
            false => POL5_A::POL5_0,
            true => POL5_A::POL5_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL5_0`"]
    #[inline(always)]
    pub fn is_pol5_0(&self) -> bool {
        **self == POL5_A::POL5_0
    }
    #[doc = "Checks if the value of the field is `POL5_1`"]
    #[inline(always)]
    pub fn is_pol5_1(&self) -> bool {
        **self == POL5_A::POL5_1
    }
}
impl core::ops::Deref for POL5_R {
    type Target = crate::FieldReader<bool, POL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL5` writer - Channel 5 Polarity"]
pub struct POL5_W<'a> {
    w: &'a mut W,
}
impl<'a> POL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol5_0(self) -> &'a mut W {
        self.variant(POL5_A::POL5_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol5_1(self) -> &'a mut W {
        self.variant(POL5_A::POL5_1)
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
#[doc = "Channel 6 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL6_A {
    #[doc = "0: The channel polarity is active high."]
    POL6_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL6_1 = 1,
}
impl From<POL6_A> for bool {
    #[inline(always)]
    fn from(variant: POL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL6` reader - Channel 6 Polarity"]
pub struct POL6_R(crate::FieldReader<bool, POL6_A>);
impl POL6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL6_A {
        match self.bits {
            false => POL6_A::POL6_0,
            true => POL6_A::POL6_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL6_0`"]
    #[inline(always)]
    pub fn is_pol6_0(&self) -> bool {
        **self == POL6_A::POL6_0
    }
    #[doc = "Checks if the value of the field is `POL6_1`"]
    #[inline(always)]
    pub fn is_pol6_1(&self) -> bool {
        **self == POL6_A::POL6_1
    }
}
impl core::ops::Deref for POL6_R {
    type Target = crate::FieldReader<bool, POL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL6` writer - Channel 6 Polarity"]
pub struct POL6_W<'a> {
    w: &'a mut W,
}
impl<'a> POL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol6_0(self) -> &'a mut W {
        self.variant(POL6_A::POL6_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol6_1(self) -> &'a mut W {
        self.variant(POL6_A::POL6_1)
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
#[doc = "Channel 7 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL7_A {
    #[doc = "0: The channel polarity is active high."]
    POL7_0 = 0,
    #[doc = "1: The channel polarity is active low."]
    POL7_1 = 1,
}
impl From<POL7_A> for bool {
    #[inline(always)]
    fn from(variant: POL7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL7` reader - Channel 7 Polarity"]
pub struct POL7_R(crate::FieldReader<bool, POL7_A>);
impl POL7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POL7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL7_A {
        match self.bits {
            false => POL7_A::POL7_0,
            true => POL7_A::POL7_1,
        }
    }
    #[doc = "Checks if the value of the field is `POL7_0`"]
    #[inline(always)]
    pub fn is_pol7_0(&self) -> bool {
        **self == POL7_A::POL7_0
    }
    #[doc = "Checks if the value of the field is `POL7_1`"]
    #[inline(always)]
    pub fn is_pol7_1(&self) -> bool {
        **self == POL7_A::POL7_1
    }
}
impl core::ops::Deref for POL7_R {
    type Target = crate::FieldReader<bool, POL7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POL7` writer - Channel 7 Polarity"]
pub struct POL7_W<'a> {
    w: &'a mut W,
}
impl<'a> POL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn pol7_0(self) -> &'a mut W {
        self.variant(POL7_A::POL7_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn pol7_1(self) -> &'a mut W {
        self.variant(POL7_A::POL7_1)
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
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&self) -> POL4_R {
        POL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&self) -> POL5_R {
        POL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    pub fn pol6(&self) -> POL6_R {
        POL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    pub fn pol7(&self) -> POL7_R {
        POL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&mut self) -> POL0_W {
        POL0_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W {
        POL1_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W {
        POL2_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&mut self) -> POL3_W {
        POL3_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&mut self) -> POL4_W {
        POL4_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&mut self) -> POL5_W {
        POL5_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    pub fn pol6(&mut self) -> POL6_W {
        POL6_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    pub fn pol7(&mut self) -> POL7_W {
        POL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channels Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](index.html) module"]
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pol::R](R) reader structure"]
impl crate::Readable for POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pol::W](W) writer structure"]
impl crate::Writable for POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
