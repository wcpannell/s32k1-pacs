#[doc = "Register `EEI` reader"]
pub struct R(crate::R<EEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEI` writer"]
pub struct W(crate::W<EEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEI_SPEC>;
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
impl From<crate::W<EEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI0_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI0_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI0_1 = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub struct EEI0_R(crate::FieldReader<bool, EEI0_A>);
impl EEI0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::EEI0_0,
            true => EEI0_A::EEI0_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI0_0`"]
    #[inline(always)]
    pub fn is_eei0_0(&self) -> bool {
        **self == EEI0_A::EEI0_0
    }
    #[doc = "Checks if the value of the field is `EEI0_1`"]
    #[inline(always)]
    pub fn is_eei0_1(&self) -> bool {
        **self == EEI0_A::EEI0_1
    }
}
impl core::ops::Deref for EEI0_R {
    type Target = crate::FieldReader<bool, EEI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub struct EEI0_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei0_0(self) -> &'a mut W {
        self.variant(EEI0_A::EEI0_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei0_1(self) -> &'a mut W {
        self.variant(EEI0_A::EEI0_1)
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
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI1_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI1_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI1_1 = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub struct EEI1_R(crate::FieldReader<bool, EEI1_A>);
impl EEI1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::EEI1_0,
            true => EEI1_A::EEI1_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI1_0`"]
    #[inline(always)]
    pub fn is_eei1_0(&self) -> bool {
        **self == EEI1_A::EEI1_0
    }
    #[doc = "Checks if the value of the field is `EEI1_1`"]
    #[inline(always)]
    pub fn is_eei1_1(&self) -> bool {
        **self == EEI1_A::EEI1_1
    }
}
impl core::ops::Deref for EEI1_R {
    type Target = crate::FieldReader<bool, EEI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub struct EEI1_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei1_0(self) -> &'a mut W {
        self.variant(EEI1_A::EEI1_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei1_1(self) -> &'a mut W {
        self.variant(EEI1_A::EEI1_1)
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
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI2_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI2_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI2_1 = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub struct EEI2_R(crate::FieldReader<bool, EEI2_A>);
impl EEI2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::EEI2_0,
            true => EEI2_A::EEI2_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI2_0`"]
    #[inline(always)]
    pub fn is_eei2_0(&self) -> bool {
        **self == EEI2_A::EEI2_0
    }
    #[doc = "Checks if the value of the field is `EEI2_1`"]
    #[inline(always)]
    pub fn is_eei2_1(&self) -> bool {
        **self == EEI2_A::EEI2_1
    }
}
impl core::ops::Deref for EEI2_R {
    type Target = crate::FieldReader<bool, EEI2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub struct EEI2_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei2_0(self) -> &'a mut W {
        self.variant(EEI2_A::EEI2_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei2_1(self) -> &'a mut W {
        self.variant(EEI2_A::EEI2_1)
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
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI3_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI3_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI3_1 = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub struct EEI3_R(crate::FieldReader<bool, EEI3_A>);
impl EEI3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::EEI3_0,
            true => EEI3_A::EEI3_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI3_0`"]
    #[inline(always)]
    pub fn is_eei3_0(&self) -> bool {
        **self == EEI3_A::EEI3_0
    }
    #[doc = "Checks if the value of the field is `EEI3_1`"]
    #[inline(always)]
    pub fn is_eei3_1(&self) -> bool {
        **self == EEI3_A::EEI3_1
    }
}
impl core::ops::Deref for EEI3_R {
    type Target = crate::FieldReader<bool, EEI3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub struct EEI3_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei3_0(self) -> &'a mut W {
        self.variant(EEI3_A::EEI3_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei3_1(self) -> &'a mut W {
        self.variant(EEI3_A::EEI3_1)
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
#[doc = "Enable Error Interrupt 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI4_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI4_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI4_1 = 1,
}
impl From<EEI4_A> for bool {
    #[inline(always)]
    fn from(variant: EEI4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI4` reader - Enable Error Interrupt 4"]
pub struct EEI4_R(crate::FieldReader<bool, EEI4_A>);
impl EEI4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI4_A {
        match self.bits {
            false => EEI4_A::EEI4_0,
            true => EEI4_A::EEI4_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI4_0`"]
    #[inline(always)]
    pub fn is_eei4_0(&self) -> bool {
        **self == EEI4_A::EEI4_0
    }
    #[doc = "Checks if the value of the field is `EEI4_1`"]
    #[inline(always)]
    pub fn is_eei4_1(&self) -> bool {
        **self == EEI4_A::EEI4_1
    }
}
impl core::ops::Deref for EEI4_R {
    type Target = crate::FieldReader<bool, EEI4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI4` writer - Enable Error Interrupt 4"]
pub struct EEI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei4_0(self) -> &'a mut W {
        self.variant(EEI4_A::EEI4_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei4_1(self) -> &'a mut W {
        self.variant(EEI4_A::EEI4_1)
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
#[doc = "Enable Error Interrupt 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI5_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI5_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI5_1 = 1,
}
impl From<EEI5_A> for bool {
    #[inline(always)]
    fn from(variant: EEI5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI5` reader - Enable Error Interrupt 5"]
pub struct EEI5_R(crate::FieldReader<bool, EEI5_A>);
impl EEI5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI5_A {
        match self.bits {
            false => EEI5_A::EEI5_0,
            true => EEI5_A::EEI5_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI5_0`"]
    #[inline(always)]
    pub fn is_eei5_0(&self) -> bool {
        **self == EEI5_A::EEI5_0
    }
    #[doc = "Checks if the value of the field is `EEI5_1`"]
    #[inline(always)]
    pub fn is_eei5_1(&self) -> bool {
        **self == EEI5_A::EEI5_1
    }
}
impl core::ops::Deref for EEI5_R {
    type Target = crate::FieldReader<bool, EEI5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI5` writer - Enable Error Interrupt 5"]
pub struct EEI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei5_0(self) -> &'a mut W {
        self.variant(EEI5_A::EEI5_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei5_1(self) -> &'a mut W {
        self.variant(EEI5_A::EEI5_1)
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
#[doc = "Enable Error Interrupt 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI6_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI6_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI6_1 = 1,
}
impl From<EEI6_A> for bool {
    #[inline(always)]
    fn from(variant: EEI6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI6` reader - Enable Error Interrupt 6"]
pub struct EEI6_R(crate::FieldReader<bool, EEI6_A>);
impl EEI6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI6_A {
        match self.bits {
            false => EEI6_A::EEI6_0,
            true => EEI6_A::EEI6_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI6_0`"]
    #[inline(always)]
    pub fn is_eei6_0(&self) -> bool {
        **self == EEI6_A::EEI6_0
    }
    #[doc = "Checks if the value of the field is `EEI6_1`"]
    #[inline(always)]
    pub fn is_eei6_1(&self) -> bool {
        **self == EEI6_A::EEI6_1
    }
}
impl core::ops::Deref for EEI6_R {
    type Target = crate::FieldReader<bool, EEI6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI6` writer - Enable Error Interrupt 6"]
pub struct EEI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei6_0(self) -> &'a mut W {
        self.variant(EEI6_A::EEI6_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei6_1(self) -> &'a mut W {
        self.variant(EEI6_A::EEI6_1)
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
#[doc = "Enable Error Interrupt 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI7_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI7_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI7_1 = 1,
}
impl From<EEI7_A> for bool {
    #[inline(always)]
    fn from(variant: EEI7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI7` reader - Enable Error Interrupt 7"]
pub struct EEI7_R(crate::FieldReader<bool, EEI7_A>);
impl EEI7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI7_A {
        match self.bits {
            false => EEI7_A::EEI7_0,
            true => EEI7_A::EEI7_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI7_0`"]
    #[inline(always)]
    pub fn is_eei7_0(&self) -> bool {
        **self == EEI7_A::EEI7_0
    }
    #[doc = "Checks if the value of the field is `EEI7_1`"]
    #[inline(always)]
    pub fn is_eei7_1(&self) -> bool {
        **self == EEI7_A::EEI7_1
    }
}
impl core::ops::Deref for EEI7_R {
    type Target = crate::FieldReader<bool, EEI7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI7` writer - Enable Error Interrupt 7"]
pub struct EEI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei7_0(self) -> &'a mut W {
        self.variant(EEI7_A::EEI7_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei7_1(self) -> &'a mut W {
        self.variant(EEI7_A::EEI7_1)
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
#[doc = "Enable Error Interrupt 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI8_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI8_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI8_1 = 1,
}
impl From<EEI8_A> for bool {
    #[inline(always)]
    fn from(variant: EEI8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI8` reader - Enable Error Interrupt 8"]
pub struct EEI8_R(crate::FieldReader<bool, EEI8_A>);
impl EEI8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI8_A {
        match self.bits {
            false => EEI8_A::EEI8_0,
            true => EEI8_A::EEI8_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI8_0`"]
    #[inline(always)]
    pub fn is_eei8_0(&self) -> bool {
        **self == EEI8_A::EEI8_0
    }
    #[doc = "Checks if the value of the field is `EEI8_1`"]
    #[inline(always)]
    pub fn is_eei8_1(&self) -> bool {
        **self == EEI8_A::EEI8_1
    }
}
impl core::ops::Deref for EEI8_R {
    type Target = crate::FieldReader<bool, EEI8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI8` writer - Enable Error Interrupt 8"]
pub struct EEI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei8_0(self) -> &'a mut W {
        self.variant(EEI8_A::EEI8_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei8_1(self) -> &'a mut W {
        self.variant(EEI8_A::EEI8_1)
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
#[doc = "Enable Error Interrupt 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI9_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI9_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI9_1 = 1,
}
impl From<EEI9_A> for bool {
    #[inline(always)]
    fn from(variant: EEI9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI9` reader - Enable Error Interrupt 9"]
pub struct EEI9_R(crate::FieldReader<bool, EEI9_A>);
impl EEI9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI9_A {
        match self.bits {
            false => EEI9_A::EEI9_0,
            true => EEI9_A::EEI9_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI9_0`"]
    #[inline(always)]
    pub fn is_eei9_0(&self) -> bool {
        **self == EEI9_A::EEI9_0
    }
    #[doc = "Checks if the value of the field is `EEI9_1`"]
    #[inline(always)]
    pub fn is_eei9_1(&self) -> bool {
        **self == EEI9_A::EEI9_1
    }
}
impl core::ops::Deref for EEI9_R {
    type Target = crate::FieldReader<bool, EEI9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI9` writer - Enable Error Interrupt 9"]
pub struct EEI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei9_0(self) -> &'a mut W {
        self.variant(EEI9_A::EEI9_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei9_1(self) -> &'a mut W {
        self.variant(EEI9_A::EEI9_1)
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
#[doc = "Enable Error Interrupt 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI10_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI10_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI10_1 = 1,
}
impl From<EEI10_A> for bool {
    #[inline(always)]
    fn from(variant: EEI10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI10` reader - Enable Error Interrupt 10"]
pub struct EEI10_R(crate::FieldReader<bool, EEI10_A>);
impl EEI10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI10_A {
        match self.bits {
            false => EEI10_A::EEI10_0,
            true => EEI10_A::EEI10_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI10_0`"]
    #[inline(always)]
    pub fn is_eei10_0(&self) -> bool {
        **self == EEI10_A::EEI10_0
    }
    #[doc = "Checks if the value of the field is `EEI10_1`"]
    #[inline(always)]
    pub fn is_eei10_1(&self) -> bool {
        **self == EEI10_A::EEI10_1
    }
}
impl core::ops::Deref for EEI10_R {
    type Target = crate::FieldReader<bool, EEI10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI10` writer - Enable Error Interrupt 10"]
pub struct EEI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei10_0(self) -> &'a mut W {
        self.variant(EEI10_A::EEI10_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei10_1(self) -> &'a mut W {
        self.variant(EEI10_A::EEI10_1)
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
#[doc = "Enable Error Interrupt 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI11_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI11_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI11_1 = 1,
}
impl From<EEI11_A> for bool {
    #[inline(always)]
    fn from(variant: EEI11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI11` reader - Enable Error Interrupt 11"]
pub struct EEI11_R(crate::FieldReader<bool, EEI11_A>);
impl EEI11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI11_A {
        match self.bits {
            false => EEI11_A::EEI11_0,
            true => EEI11_A::EEI11_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI11_0`"]
    #[inline(always)]
    pub fn is_eei11_0(&self) -> bool {
        **self == EEI11_A::EEI11_0
    }
    #[doc = "Checks if the value of the field is `EEI11_1`"]
    #[inline(always)]
    pub fn is_eei11_1(&self) -> bool {
        **self == EEI11_A::EEI11_1
    }
}
impl core::ops::Deref for EEI11_R {
    type Target = crate::FieldReader<bool, EEI11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI11` writer - Enable Error Interrupt 11"]
pub struct EEI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei11_0(self) -> &'a mut W {
        self.variant(EEI11_A::EEI11_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei11_1(self) -> &'a mut W {
        self.variant(EEI11_A::EEI11_1)
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
#[doc = "Enable Error Interrupt 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI12_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI12_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI12_1 = 1,
}
impl From<EEI12_A> for bool {
    #[inline(always)]
    fn from(variant: EEI12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI12` reader - Enable Error Interrupt 12"]
pub struct EEI12_R(crate::FieldReader<bool, EEI12_A>);
impl EEI12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI12_A {
        match self.bits {
            false => EEI12_A::EEI12_0,
            true => EEI12_A::EEI12_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI12_0`"]
    #[inline(always)]
    pub fn is_eei12_0(&self) -> bool {
        **self == EEI12_A::EEI12_0
    }
    #[doc = "Checks if the value of the field is `EEI12_1`"]
    #[inline(always)]
    pub fn is_eei12_1(&self) -> bool {
        **self == EEI12_A::EEI12_1
    }
}
impl core::ops::Deref for EEI12_R {
    type Target = crate::FieldReader<bool, EEI12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI12` writer - Enable Error Interrupt 12"]
pub struct EEI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei12_0(self) -> &'a mut W {
        self.variant(EEI12_A::EEI12_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei12_1(self) -> &'a mut W {
        self.variant(EEI12_A::EEI12_1)
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
#[doc = "Enable Error Interrupt 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI13_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI13_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI13_1 = 1,
}
impl From<EEI13_A> for bool {
    #[inline(always)]
    fn from(variant: EEI13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI13` reader - Enable Error Interrupt 13"]
pub struct EEI13_R(crate::FieldReader<bool, EEI13_A>);
impl EEI13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI13_A {
        match self.bits {
            false => EEI13_A::EEI13_0,
            true => EEI13_A::EEI13_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI13_0`"]
    #[inline(always)]
    pub fn is_eei13_0(&self) -> bool {
        **self == EEI13_A::EEI13_0
    }
    #[doc = "Checks if the value of the field is `EEI13_1`"]
    #[inline(always)]
    pub fn is_eei13_1(&self) -> bool {
        **self == EEI13_A::EEI13_1
    }
}
impl core::ops::Deref for EEI13_R {
    type Target = crate::FieldReader<bool, EEI13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI13` writer - Enable Error Interrupt 13"]
pub struct EEI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei13_0(self) -> &'a mut W {
        self.variant(EEI13_A::EEI13_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei13_1(self) -> &'a mut W {
        self.variant(EEI13_A::EEI13_1)
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
#[doc = "Enable Error Interrupt 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI14_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI14_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI14_1 = 1,
}
impl From<EEI14_A> for bool {
    #[inline(always)]
    fn from(variant: EEI14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI14` reader - Enable Error Interrupt 14"]
pub struct EEI14_R(crate::FieldReader<bool, EEI14_A>);
impl EEI14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI14_A {
        match self.bits {
            false => EEI14_A::EEI14_0,
            true => EEI14_A::EEI14_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI14_0`"]
    #[inline(always)]
    pub fn is_eei14_0(&self) -> bool {
        **self == EEI14_A::EEI14_0
    }
    #[doc = "Checks if the value of the field is `EEI14_1`"]
    #[inline(always)]
    pub fn is_eei14_1(&self) -> bool {
        **self == EEI14_A::EEI14_1
    }
}
impl core::ops::Deref for EEI14_R {
    type Target = crate::FieldReader<bool, EEI14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI14` writer - Enable Error Interrupt 14"]
pub struct EEI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei14_0(self) -> &'a mut W {
        self.variant(EEI14_A::EEI14_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei14_1(self) -> &'a mut W {
        self.variant(EEI14_A::EEI14_1)
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
#[doc = "Enable Error Interrupt 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEI15_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    EEI15_0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    EEI15_1 = 1,
}
impl From<EEI15_A> for bool {
    #[inline(always)]
    fn from(variant: EEI15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEI15` reader - Enable Error Interrupt 15"]
pub struct EEI15_R(crate::FieldReader<bool, EEI15_A>);
impl EEI15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EEI15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI15_A {
        match self.bits {
            false => EEI15_A::EEI15_0,
            true => EEI15_A::EEI15_1,
        }
    }
    #[doc = "Checks if the value of the field is `EEI15_0`"]
    #[inline(always)]
    pub fn is_eei15_0(&self) -> bool {
        **self == EEI15_A::EEI15_0
    }
    #[doc = "Checks if the value of the field is `EEI15_1`"]
    #[inline(always)]
    pub fn is_eei15_1(&self) -> bool {
        **self == EEI15_A::EEI15_1
    }
}
impl core::ops::Deref for EEI15_R {
    type Target = crate::FieldReader<bool, EEI15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEI15` writer - Enable Error Interrupt 15"]
pub struct EEI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EEI15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEI15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn eei15_0(self) -> &'a mut W {
        self.variant(EEI15_A::EEI15_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn eei15_1(self) -> &'a mut W {
        self.variant(EEI15_A::EEI15_1)
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
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&self) -> EEI4_R {
        EEI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&self) -> EEI5_R {
        EEI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&self) -> EEI6_R {
        EEI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&self) -> EEI7_R {
        EEI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&self) -> EEI8_R {
        EEI8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&self) -> EEI9_R {
        EEI9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&self) -> EEI10_R {
        EEI10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&self) -> EEI11_R {
        EEI11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&self) -> EEI12_R {
        EEI12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&self) -> EEI13_R {
        EEI13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&self) -> EEI14_R {
        EEI14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&self) -> EEI15_R {
        EEI15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&mut self) -> EEI0_W {
        EEI0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&mut self) -> EEI1_W {
        EEI1_W { w: self }
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&mut self) -> EEI2_W {
        EEI2_W { w: self }
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&mut self) -> EEI3_W {
        EEI3_W { w: self }
    }
    #[doc = "Bit 4 - Enable Error Interrupt 4"]
    #[inline(always)]
    pub fn eei4(&mut self) -> EEI4_W {
        EEI4_W { w: self }
    }
    #[doc = "Bit 5 - Enable Error Interrupt 5"]
    #[inline(always)]
    pub fn eei5(&mut self) -> EEI5_W {
        EEI5_W { w: self }
    }
    #[doc = "Bit 6 - Enable Error Interrupt 6"]
    #[inline(always)]
    pub fn eei6(&mut self) -> EEI6_W {
        EEI6_W { w: self }
    }
    #[doc = "Bit 7 - Enable Error Interrupt 7"]
    #[inline(always)]
    pub fn eei7(&mut self) -> EEI7_W {
        EEI7_W { w: self }
    }
    #[doc = "Bit 8 - Enable Error Interrupt 8"]
    #[inline(always)]
    pub fn eei8(&mut self) -> EEI8_W {
        EEI8_W { w: self }
    }
    #[doc = "Bit 9 - Enable Error Interrupt 9"]
    #[inline(always)]
    pub fn eei9(&mut self) -> EEI9_W {
        EEI9_W { w: self }
    }
    #[doc = "Bit 10 - Enable Error Interrupt 10"]
    #[inline(always)]
    pub fn eei10(&mut self) -> EEI10_W {
        EEI10_W { w: self }
    }
    #[doc = "Bit 11 - Enable Error Interrupt 11"]
    #[inline(always)]
    pub fn eei11(&mut self) -> EEI11_W {
        EEI11_W { w: self }
    }
    #[doc = "Bit 12 - Enable Error Interrupt 12"]
    #[inline(always)]
    pub fn eei12(&mut self) -> EEI12_W {
        EEI12_W { w: self }
    }
    #[doc = "Bit 13 - Enable Error Interrupt 13"]
    #[inline(always)]
    pub fn eei13(&mut self) -> EEI13_W {
        EEI13_W { w: self }
    }
    #[doc = "Bit 14 - Enable Error Interrupt 14"]
    #[inline(always)]
    pub fn eei14(&mut self) -> EEI14_W {
        EEI14_W { w: self }
    }
    #[doc = "Bit 15 - Enable Error Interrupt 15"]
    #[inline(always)]
    pub fn eei15(&mut self) -> EEI15_W {
        EEI15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](index.html) module"]
pub struct EEI_SPEC;
impl crate::RegisterSpec for EEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eei::R](R) reader structure"]
impl crate::Readable for EEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eei::W](W) writer structure"]
impl crate::Writable for EEI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EEI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
