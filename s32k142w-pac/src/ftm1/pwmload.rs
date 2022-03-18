#[doc = "Register `PWMLOAD` reader"]
pub struct R(crate::R<PWMLOAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMLOAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMLOAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMLOAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMLOAD` writer"]
pub struct W(crate::W<PWMLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMLOAD_SPEC>;
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
impl From<crate::W<PWMLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel 0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH0SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH0SEL_1 = 1,
}
impl From<CH0SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0SEL` reader - Channel 0 Select"]
pub struct CH0SEL_R(crate::FieldReader<bool, CH0SEL_A>);
impl CH0SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0SEL_A {
        match self.bits {
            false => CH0SEL_A::CH0SEL_0,
            true => CH0SEL_A::CH0SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH0SEL_0`"]
    #[inline(always)]
    pub fn is_ch0sel_0(&self) -> bool {
        **self == CH0SEL_A::CH0SEL_0
    }
    #[doc = "Checks if the value of the field is `CH0SEL_1`"]
    #[inline(always)]
    pub fn is_ch0sel_1(&self) -> bool {
        **self == CH0SEL_A::CH0SEL_1
    }
}
impl core::ops::Deref for CH0SEL_R {
    type Target = crate::FieldReader<bool, CH0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0SEL` writer - Channel 0 Select"]
pub struct CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch0sel_0(self) -> &'a mut W {
        self.variant(CH0SEL_A::CH0SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch0sel_1(self) -> &'a mut W {
        self.variant(CH0SEL_A::CH0SEL_1)
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
#[doc = "Channel 1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH1SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH1SEL_1 = 1,
}
impl From<CH1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1SEL` reader - Channel 1 Select"]
pub struct CH1SEL_R(crate::FieldReader<bool, CH1SEL_A>);
impl CH1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1SEL_A {
        match self.bits {
            false => CH1SEL_A::CH1SEL_0,
            true => CH1SEL_A::CH1SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH1SEL_0`"]
    #[inline(always)]
    pub fn is_ch1sel_0(&self) -> bool {
        **self == CH1SEL_A::CH1SEL_0
    }
    #[doc = "Checks if the value of the field is `CH1SEL_1`"]
    #[inline(always)]
    pub fn is_ch1sel_1(&self) -> bool {
        **self == CH1SEL_A::CH1SEL_1
    }
}
impl core::ops::Deref for CH1SEL_R {
    type Target = crate::FieldReader<bool, CH1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1SEL` writer - Channel 1 Select"]
pub struct CH1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch1sel_0(self) -> &'a mut W {
        self.variant(CH1SEL_A::CH1SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch1sel_1(self) -> &'a mut W {
        self.variant(CH1SEL_A::CH1SEL_1)
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
#[doc = "Channel 2 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH2SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH2SEL_1 = 1,
}
impl From<CH2SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2SEL` reader - Channel 2 Select"]
pub struct CH2SEL_R(crate::FieldReader<bool, CH2SEL_A>);
impl CH2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2SEL_A {
        match self.bits {
            false => CH2SEL_A::CH2SEL_0,
            true => CH2SEL_A::CH2SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH2SEL_0`"]
    #[inline(always)]
    pub fn is_ch2sel_0(&self) -> bool {
        **self == CH2SEL_A::CH2SEL_0
    }
    #[doc = "Checks if the value of the field is `CH2SEL_1`"]
    #[inline(always)]
    pub fn is_ch2sel_1(&self) -> bool {
        **self == CH2SEL_A::CH2SEL_1
    }
}
impl core::ops::Deref for CH2SEL_R {
    type Target = crate::FieldReader<bool, CH2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2SEL` writer - Channel 2 Select"]
pub struct CH2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch2sel_0(self) -> &'a mut W {
        self.variant(CH2SEL_A::CH2SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch2sel_1(self) -> &'a mut W {
        self.variant(CH2SEL_A::CH2SEL_1)
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
#[doc = "Channel 3 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH3SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH3SEL_1 = 1,
}
impl From<CH3SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3SEL` reader - Channel 3 Select"]
pub struct CH3SEL_R(crate::FieldReader<bool, CH3SEL_A>);
impl CH3SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH3SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3SEL_A {
        match self.bits {
            false => CH3SEL_A::CH3SEL_0,
            true => CH3SEL_A::CH3SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH3SEL_0`"]
    #[inline(always)]
    pub fn is_ch3sel_0(&self) -> bool {
        **self == CH3SEL_A::CH3SEL_0
    }
    #[doc = "Checks if the value of the field is `CH3SEL_1`"]
    #[inline(always)]
    pub fn is_ch3sel_1(&self) -> bool {
        **self == CH3SEL_A::CH3SEL_1
    }
}
impl core::ops::Deref for CH3SEL_R {
    type Target = crate::FieldReader<bool, CH3SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3SEL` writer - Channel 3 Select"]
pub struct CH3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch3sel_0(self) -> &'a mut W {
        self.variant(CH3SEL_A::CH3SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch3sel_1(self) -> &'a mut W {
        self.variant(CH3SEL_A::CH3SEL_1)
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
#[doc = "Channel 4 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH4SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH4SEL_1 = 1,
}
impl From<CH4SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4SEL` reader - Channel 4 Select"]
pub struct CH4SEL_R(crate::FieldReader<bool, CH4SEL_A>);
impl CH4SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH4SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4SEL_A {
        match self.bits {
            false => CH4SEL_A::CH4SEL_0,
            true => CH4SEL_A::CH4SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH4SEL_0`"]
    #[inline(always)]
    pub fn is_ch4sel_0(&self) -> bool {
        **self == CH4SEL_A::CH4SEL_0
    }
    #[doc = "Checks if the value of the field is `CH4SEL_1`"]
    #[inline(always)]
    pub fn is_ch4sel_1(&self) -> bool {
        **self == CH4SEL_A::CH4SEL_1
    }
}
impl core::ops::Deref for CH4SEL_R {
    type Target = crate::FieldReader<bool, CH4SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4SEL` writer - Channel 4 Select"]
pub struct CH4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch4sel_0(self) -> &'a mut W {
        self.variant(CH4SEL_A::CH4SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch4sel_1(self) -> &'a mut W {
        self.variant(CH4SEL_A::CH4SEL_1)
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
#[doc = "Channel 5 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH5SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH5SEL_1 = 1,
}
impl From<CH5SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5SEL` reader - Channel 5 Select"]
pub struct CH5SEL_R(crate::FieldReader<bool, CH5SEL_A>);
impl CH5SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH5SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5SEL_A {
        match self.bits {
            false => CH5SEL_A::CH5SEL_0,
            true => CH5SEL_A::CH5SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH5SEL_0`"]
    #[inline(always)]
    pub fn is_ch5sel_0(&self) -> bool {
        **self == CH5SEL_A::CH5SEL_0
    }
    #[doc = "Checks if the value of the field is `CH5SEL_1`"]
    #[inline(always)]
    pub fn is_ch5sel_1(&self) -> bool {
        **self == CH5SEL_A::CH5SEL_1
    }
}
impl core::ops::Deref for CH5SEL_R {
    type Target = crate::FieldReader<bool, CH5SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5SEL` writer - Channel 5 Select"]
pub struct CH5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch5sel_0(self) -> &'a mut W {
        self.variant(CH5SEL_A::CH5SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch5sel_1(self) -> &'a mut W {
        self.variant(CH5SEL_A::CH5SEL_1)
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
#[doc = "Channel 6 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH6SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH6SEL_1 = 1,
}
impl From<CH6SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6SEL` reader - Channel 6 Select"]
pub struct CH6SEL_R(crate::FieldReader<bool, CH6SEL_A>);
impl CH6SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH6SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6SEL_A {
        match self.bits {
            false => CH6SEL_A::CH6SEL_0,
            true => CH6SEL_A::CH6SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH6SEL_0`"]
    #[inline(always)]
    pub fn is_ch6sel_0(&self) -> bool {
        **self == CH6SEL_A::CH6SEL_0
    }
    #[doc = "Checks if the value of the field is `CH6SEL_1`"]
    #[inline(always)]
    pub fn is_ch6sel_1(&self) -> bool {
        **self == CH6SEL_A::CH6SEL_1
    }
}
impl core::ops::Deref for CH6SEL_R {
    type Target = crate::FieldReader<bool, CH6SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6SEL` writer - Channel 6 Select"]
pub struct CH6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch6sel_0(self) -> &'a mut W {
        self.variant(CH6SEL_A::CH6SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch6sel_1(self) -> &'a mut W {
        self.variant(CH6SEL_A::CH6SEL_1)
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
#[doc = "Channel 7 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7SEL_A {
    #[doc = "0: Channel match is not included as a reload opportunity."]
    CH7SEL_0 = 0,
    #[doc = "1: Channel match is included as a reload opportunity."]
    CH7SEL_1 = 1,
}
impl From<CH7SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7SEL` reader - Channel 7 Select"]
pub struct CH7SEL_R(crate::FieldReader<bool, CH7SEL_A>);
impl CH7SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH7SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7SEL_A {
        match self.bits {
            false => CH7SEL_A::CH7SEL_0,
            true => CH7SEL_A::CH7SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `CH7SEL_0`"]
    #[inline(always)]
    pub fn is_ch7sel_0(&self) -> bool {
        **self == CH7SEL_A::CH7SEL_0
    }
    #[doc = "Checks if the value of the field is `CH7SEL_1`"]
    #[inline(always)]
    pub fn is_ch7sel_1(&self) -> bool {
        **self == CH7SEL_A::CH7SEL_1
    }
}
impl core::ops::Deref for CH7SEL_R {
    type Target = crate::FieldReader<bool, CH7SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7SEL` writer - Channel 7 Select"]
pub struct CH7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel match is not included as a reload opportunity."]
    #[inline(always)]
    pub fn ch7sel_0(self) -> &'a mut W {
        self.variant(CH7SEL_A::CH7SEL_0)
    }
    #[doc = "Channel match is included as a reload opportunity."]
    #[inline(always)]
    pub fn ch7sel_1(self) -> &'a mut W {
        self.variant(CH7SEL_A::CH7SEL_1)
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
#[doc = "Half Cycle Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCSEL_A {
    #[doc = "0: Half cycle reload is disabled and it is not considered as a reload opportunity."]
    HCSEL_0 = 0,
    #[doc = "1: Half cycle reload is enabled and it is considered as a reload opportunity."]
    HCSEL_1 = 1,
}
impl From<HCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCSEL` reader - Half Cycle Select"]
pub struct HCSEL_R(crate::FieldReader<bool, HCSEL_A>);
impl HCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCSEL_A {
        match self.bits {
            false => HCSEL_A::HCSEL_0,
            true => HCSEL_A::HCSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `HCSEL_0`"]
    #[inline(always)]
    pub fn is_hcsel_0(&self) -> bool {
        **self == HCSEL_A::HCSEL_0
    }
    #[doc = "Checks if the value of the field is `HCSEL_1`"]
    #[inline(always)]
    pub fn is_hcsel_1(&self) -> bool {
        **self == HCSEL_A::HCSEL_1
    }
}
impl core::ops::Deref for HCSEL_R {
    type Target = crate::FieldReader<bool, HCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCSEL` writer - Half Cycle Select"]
pub struct HCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Half cycle reload is disabled and it is not considered as a reload opportunity."]
    #[inline(always)]
    pub fn hcsel_0(self) -> &'a mut W {
        self.variant(HCSEL_A::HCSEL_0)
    }
    #[doc = "Half cycle reload is enabled and it is considered as a reload opportunity."]
    #[inline(always)]
    pub fn hcsel_1(self) -> &'a mut W {
        self.variant(HCSEL_A::HCSEL_1)
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
#[doc = "Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDOK_A {
    #[doc = "0: Loading updated values is disabled."]
    LDOK_0 = 0,
    #[doc = "1: Loading updated values is enabled."]
    LDOK_1 = 1,
}
impl From<LDOK_A> for bool {
    #[inline(always)]
    fn from(variant: LDOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDOK` reader - Load Enable"]
pub struct LDOK_R(crate::FieldReader<bool, LDOK_A>);
impl LDOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOK_A {
        match self.bits {
            false => LDOK_A::LDOK_0,
            true => LDOK_A::LDOK_1,
        }
    }
    #[doc = "Checks if the value of the field is `LDOK_0`"]
    #[inline(always)]
    pub fn is_ldok_0(&self) -> bool {
        **self == LDOK_A::LDOK_0
    }
    #[doc = "Checks if the value of the field is `LDOK_1`"]
    #[inline(always)]
    pub fn is_ldok_1(&self) -> bool {
        **self == LDOK_A::LDOK_1
    }
}
impl core::ops::Deref for LDOK_R {
    type Target = crate::FieldReader<bool, LDOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDOK` writer - Load Enable"]
pub struct LDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loading updated values is disabled."]
    #[inline(always)]
    pub fn ldok_0(self) -> &'a mut W {
        self.variant(LDOK_A::LDOK_0)
    }
    #[doc = "Loading updated values is enabled."]
    #[inline(always)]
    pub fn ldok_1(self) -> &'a mut W {
        self.variant(LDOK_A::LDOK_1)
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
#[doc = "Global Load Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLEN_A {
    #[doc = "0: Global Load Ok disabled."]
    GLEN_0 = 0,
    #[doc = "1: Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    GLEN_1 = 1,
}
impl From<GLEN_A> for bool {
    #[inline(always)]
    fn from(variant: GLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLEN` reader - Global Load Enable"]
pub struct GLEN_R(crate::FieldReader<bool, GLEN_A>);
impl GLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLEN_A {
        match self.bits {
            false => GLEN_A::GLEN_0,
            true => GLEN_A::GLEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GLEN_0`"]
    #[inline(always)]
    pub fn is_glen_0(&self) -> bool {
        **self == GLEN_A::GLEN_0
    }
    #[doc = "Checks if the value of the field is `GLEN_1`"]
    #[inline(always)]
    pub fn is_glen_1(&self) -> bool {
        **self == GLEN_A::GLEN_1
    }
}
impl core::ops::Deref for GLEN_R {
    type Target = crate::FieldReader<bool, GLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLEN` writer - Global Load Enable"]
pub struct GLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Global Load Ok disabled."]
    #[inline(always)]
    pub fn glen_0(self) -> &'a mut W {
        self.variant(GLEN_A::GLEN_0)
    }
    #[doc = "Global Load OK enabled. A pulse event on the module global load input sets the LDOK bit."]
    #[inline(always)]
    pub fn glen_1(self) -> &'a mut W {
        self.variant(GLEN_A::GLEN_1)
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
#[doc = "Global Load OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLDOK_A {
    #[doc = "0: No action."]
    GLDOK_0 = 0,
    #[doc = "1: LDOK bit is set."]
    GLDOK_1 = 1,
}
impl From<GLDOK_A> for bool {
    #[inline(always)]
    fn from(variant: GLDOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLDOK` reader - Global Load OK"]
pub struct GLDOK_R(crate::FieldReader<bool, GLDOK_A>);
impl GLDOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GLDOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLDOK_A {
        match self.bits {
            false => GLDOK_A::GLDOK_0,
            true => GLDOK_A::GLDOK_1,
        }
    }
    #[doc = "Checks if the value of the field is `GLDOK_0`"]
    #[inline(always)]
    pub fn is_gldok_0(&self) -> bool {
        **self == GLDOK_A::GLDOK_0
    }
    #[doc = "Checks if the value of the field is `GLDOK_1`"]
    #[inline(always)]
    pub fn is_gldok_1(&self) -> bool {
        **self == GLDOK_A::GLDOK_1
    }
}
impl core::ops::Deref for GLDOK_R {
    type Target = crate::FieldReader<bool, GLDOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLDOK` writer - Global Load OK"]
pub struct GLDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> GLDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLDOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action."]
    #[inline(always)]
    pub fn gldok_0(self) -> &'a mut W {
        self.variant(GLDOK_A::GLDOK_0)
    }
    #[doc = "LDOK bit is set."]
    #[inline(always)]
    pub fn gldok_1(self) -> &'a mut W {
        self.variant(GLDOK_A::GLDOK_1)
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
impl R {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline(always)]
    pub fn hcsel(&self) -> HCSEL_R {
        HCSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline(always)]
    pub fn glen(&self) -> GLEN_R {
        GLEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Global Load OK"]
    #[inline(always)]
    pub fn gldok(&self) -> GLDOK_R {
        GLDOK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Select"]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> CH0SEL_W {
        CH0SEL_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Select"]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> CH1SEL_W {
        CH1SEL_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Select"]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> CH2SEL_W {
        CH2SEL_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Select"]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> CH3SEL_W {
        CH3SEL_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Select"]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> CH4SEL_W {
        CH4SEL_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Select"]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> CH5SEL_W {
        CH5SEL_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Select"]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> CH6SEL_W {
        CH6SEL_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Select"]
    #[inline(always)]
    pub fn ch7sel(&mut self) -> CH7SEL_W {
        CH7SEL_W { w: self }
    }
    #[doc = "Bit 8 - Half Cycle Select"]
    #[inline(always)]
    pub fn hcsel(&mut self) -> HCSEL_W {
        HCSEL_W { w: self }
    }
    #[doc = "Bit 9 - Load Enable"]
    #[inline(always)]
    pub fn ldok(&mut self) -> LDOK_W {
        LDOK_W { w: self }
    }
    #[doc = "Bit 10 - Global Load Enable"]
    #[inline(always)]
    pub fn glen(&mut self) -> GLEN_W {
        GLEN_W { w: self }
    }
    #[doc = "Bit 11 - Global Load OK"]
    #[inline(always)]
    pub fn gldok(&mut self) -> GLDOK_W {
        GLDOK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM PWM Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmload](index.html) module"]
pub struct PWMLOAD_SPEC;
impl crate::RegisterSpec for PWMLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmload::R](R) reader structure"]
impl crate::Readable for PWMLOAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmload::W](W) writer structure"]
impl crate::Writable for PWMLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMLOAD to value 0"]
impl crate::Resettable for PWMLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
