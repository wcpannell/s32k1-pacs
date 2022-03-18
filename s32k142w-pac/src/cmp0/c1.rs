#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOSEL` reader - DAC Output Voltage Select"]
pub struct VOSEL_R(crate::FieldReader<u8, u8>);
impl VOSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOSEL` writer - DAC Output Voltage Select"]
pub struct VOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Minus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    MSEL_0 = 0,
    #[doc = "1: IN1"]
    MSEL_1 = 1,
    #[doc = "2: IN2"]
    MSEL_2 = 2,
    #[doc = "3: IN3"]
    MSEL_3 = 3,
    #[doc = "4: IN4"]
    MSEL_4 = 4,
    #[doc = "5: IN5"]
    MSEL_5 = 5,
    #[doc = "6: IN6"]
    MSEL_6 = 6,
    #[doc = "7: IN7"]
    MSEL_7 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MSEL` reader - Minus Input MUX Control"]
pub struct MSEL_R(crate::FieldReader<u8, MSEL_A>);
impl MSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            4 => MSEL_A::MSEL_4,
            5 => MSEL_A::MSEL_5,
            6 => MSEL_A::MSEL_6,
            7 => MSEL_A::MSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        **self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        **self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        **self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        **self == MSEL_A::MSEL_3
    }
    #[doc = "Checks if the value of the field is `MSEL_4`"]
    #[inline(always)]
    pub fn is_msel_4(&self) -> bool {
        **self == MSEL_A::MSEL_4
    }
    #[doc = "Checks if the value of the field is `MSEL_5`"]
    #[inline(always)]
    pub fn is_msel_5(&self) -> bool {
        **self == MSEL_A::MSEL_5
    }
    #[doc = "Checks if the value of the field is `MSEL_6`"]
    #[inline(always)]
    pub fn is_msel_6(&self) -> bool {
        **self == MSEL_A::MSEL_6
    }
    #[doc = "Checks if the value of the field is `MSEL_7`"]
    #[inline(always)]
    pub fn is_msel_7(&self) -> bool {
        **self == MSEL_A::MSEL_7
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, MSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSEL` writer - Minus Input MUX Control"]
pub struct MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn msel_4(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn msel_5(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn msel_6(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn msel_7(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Plus Input MUX Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    PSEL_0 = 0,
    #[doc = "1: IN1"]
    PSEL_1 = 1,
    #[doc = "2: IN2"]
    PSEL_2 = 2,
    #[doc = "3: IN3"]
    PSEL_3 = 3,
    #[doc = "4: IN4"]
    PSEL_4 = 4,
    #[doc = "5: IN5"]
    PSEL_5 = 5,
    #[doc = "6: IN6"]
    PSEL_6 = 6,
    #[doc = "7: IN7"]
    PSEL_7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - Plus Input MUX Control"]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            4 => PSEL_A::PSEL_4,
            5 => PSEL_A::PSEL_5,
            6 => PSEL_A::PSEL_6,
            7 => PSEL_A::PSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        **self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        **self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        **self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        **self == PSEL_A::PSEL_3
    }
    #[doc = "Checks if the value of the field is `PSEL_4`"]
    #[inline(always)]
    pub fn is_psel_4(&self) -> bool {
        **self == PSEL_A::PSEL_4
    }
    #[doc = "Checks if the value of the field is `PSEL_5`"]
    #[inline(always)]
    pub fn is_psel_5(&self) -> bool {
        **self == PSEL_A::PSEL_5
    }
    #[doc = "Checks if the value of the field is `PSEL_6`"]
    #[inline(always)]
    pub fn is_psel_6(&self) -> bool {
        **self == PSEL_A::PSEL_6
    }
    #[doc = "Checks if the value of the field is `PSEL_7`"]
    #[inline(always)]
    pub fn is_psel_7(&self) -> bool {
        **self == PSEL_A::PSEL_7
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, PSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - Plus Input MUX Control"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn psel_4(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn psel_5(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn psel_6(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn psel_7(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | ((value as u32 & 0x07) << 11);
        self.w
    }
}
#[doc = "Supply Voltage Reference Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSEL_A {
    #[doc = "0: Vin1 is selected as resistor ladder network supply reference Vin."]
    VRSEL_0 = 0,
    #[doc = "1: Vin2 is selected as resistor ladder network supply reference Vin."]
    VRSEL_1 = 1,
}
impl From<VRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: VRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VRSEL` reader - Supply Voltage Reference Source Select"]
pub struct VRSEL_R(crate::FieldReader<bool, VRSEL_A>);
impl VRSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VRSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VRSEL_A {
        match self.bits {
            false => VRSEL_A::VRSEL_0,
            true => VRSEL_A::VRSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `VRSEL_0`"]
    #[inline(always)]
    pub fn is_vrsel_0(&self) -> bool {
        **self == VRSEL_A::VRSEL_0
    }
    #[doc = "Checks if the value of the field is `VRSEL_1`"]
    #[inline(always)]
    pub fn is_vrsel_1(&self) -> bool {
        **self == VRSEL_A::VRSEL_1
    }
}
impl core::ops::Deref for VRSEL_R {
    type Target = crate::FieldReader<bool, VRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VRSEL` writer - Supply Voltage Reference Source Select"]
pub struct VRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VRSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn vrsel_0(self) -> &'a mut W {
        self.variant(VRSEL_A::VRSEL_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    #[inline(always)]
    pub fn vrsel_1(self) -> &'a mut W {
        self.variant(VRSEL_A::VRSEL_1)
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
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACEN_A {
    #[doc = "0: DAC is disabled."]
    DACEN_0 = 0,
    #[doc = "1: DAC is enabled."]
    DACEN_1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub struct DACEN_R(crate::FieldReader<bool, DACEN_A>);
impl DACEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::DACEN_0,
            true => DACEN_A::DACEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DACEN_0`"]
    #[inline(always)]
    pub fn is_dacen_0(&self) -> bool {
        **self == DACEN_A::DACEN_0
    }
    #[doc = "Checks if the value of the field is `DACEN_1`"]
    #[inline(always)]
    pub fn is_dacen_1(&self) -> bool {
        **self == DACEN_A::DACEN_1
    }
}
impl core::ops::Deref for DACEN_R {
    type Target = crate::FieldReader<bool, DACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub struct DACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DAC is disabled."]
    #[inline(always)]
    pub fn dacen_0(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_0)
    }
    #[doc = "DAC is enabled."]
    #[inline(always)]
    pub fn dacen_1(self) -> &'a mut W {
        self.variant(DACEN_A::DACEN_1)
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
#[doc = "Field `CHN0` reader - Channel 0 input enable"]
pub struct CHN0_R(crate::FieldReader<bool, bool>);
impl CHN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN0` writer - Channel 0 input enable"]
pub struct CHN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CHN1` reader - Channel 1 input enable"]
pub struct CHN1_R(crate::FieldReader<bool, bool>);
impl CHN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN1` writer - Channel 1 input enable"]
pub struct CHN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CHN2` reader - Channel 2 input enable"]
pub struct CHN2_R(crate::FieldReader<bool, bool>);
impl CHN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN2` writer - Channel 2 input enable"]
pub struct CHN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CHN3` reader - Channel 3 input enable"]
pub struct CHN3_R(crate::FieldReader<bool, bool>);
impl CHN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN3` writer - Channel 3 input enable"]
pub struct CHN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CHN4` reader - Channel 4 input enable"]
pub struct CHN4_R(crate::FieldReader<bool, bool>);
impl CHN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN4` writer - Channel 4 input enable"]
pub struct CHN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CHN5` reader - Channel 5 input enable"]
pub struct CHN5_R(crate::FieldReader<bool, bool>);
impl CHN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN5` writer - Channel 5 input enable"]
pub struct CHN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CHN6` reader - Channel 6 input enable"]
pub struct CHN6_R(crate::FieldReader<bool, bool>);
impl CHN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN6` writer - Channel 6 input enable"]
pub struct CHN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CHN7` reader - Channel 7 input enable"]
pub struct CHN7_R(crate::FieldReader<bool, bool>);
impl CHN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHN7` writer - Channel 7 input enable"]
pub struct CHN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Selection of the input to the negative port of the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INNSEL_A {
    #[doc = "0: IN0, from the 8-bit DAC output"]
    INNSEL_0 = 0,
    #[doc = "1: IN1, from the analog 8-1 mux"]
    INNSEL_1 = 1,
}
impl From<INNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INNSEL` reader - Selection of the input to the negative port of the comparator"]
pub struct INNSEL_R(crate::FieldReader<u8, INNSEL_A>);
impl INNSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INNSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INNSEL_A> {
        match self.bits {
            0 => Some(INNSEL_A::INNSEL_0),
            1 => Some(INNSEL_A::INNSEL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INNSEL_0`"]
    #[inline(always)]
    pub fn is_innsel_0(&self) -> bool {
        **self == INNSEL_A::INNSEL_0
    }
    #[doc = "Checks if the value of the field is `INNSEL_1`"]
    #[inline(always)]
    pub fn is_innsel_1(&self) -> bool {
        **self == INNSEL_A::INNSEL_1
    }
}
impl core::ops::Deref for INNSEL_R {
    type Target = crate::FieldReader<u8, INNSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INNSEL` writer - Selection of the input to the negative port of the comparator"]
pub struct INNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline(always)]
    pub fn innsel_0(self) -> &'a mut W {
        self.variant(INNSEL_A::INNSEL_0)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline(always)]
    pub fn innsel_1(self) -> &'a mut W {
        self.variant(INNSEL_A::INNSEL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Selection of the input to the positive port of the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPSEL_A {
    #[doc = "0: IN0, from the 8-bit DAC output"]
    INPSEL_0 = 0,
    #[doc = "1: IN1, from the analog 8-1 mux"]
    INPSEL_1 = 1,
}
impl From<INPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INPSEL` reader - Selection of the input to the positive port of the comparator"]
pub struct INPSEL_R(crate::FieldReader<u8, INPSEL_A>);
impl INPSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INPSEL_A> {
        match self.bits {
            0 => Some(INPSEL_A::INPSEL_0),
            1 => Some(INPSEL_A::INPSEL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INPSEL_0`"]
    #[inline(always)]
    pub fn is_inpsel_0(&self) -> bool {
        **self == INPSEL_A::INPSEL_0
    }
    #[doc = "Checks if the value of the field is `INPSEL_1`"]
    #[inline(always)]
    pub fn is_inpsel_1(&self) -> bool {
        **self == INPSEL_A::INPSEL_1
    }
}
impl core::ops::Deref for INPSEL_R {
    type Target = crate::FieldReader<u8, INPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INPSEL` writer - Selection of the input to the positive port of the comparator"]
pub struct INPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline(always)]
    pub fn inpsel_0(self) -> &'a mut W {
        self.variant(INPSEL_A::INPSEL_0)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline(always)]
    pub fn inpsel_1(self) -> &'a mut W {
        self.variant(INPSEL_A::INPSEL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&self) -> VOSEL_R {
        VOSEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&self) -> CHN0_R {
        CHN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&self) -> CHN1_R {
        CHN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&self) -> CHN2_R {
        CHN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&self) -> CHN3_R {
        CHN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&self) -> CHN4_R {
        CHN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&self) -> CHN5_R {
        CHN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline(always)]
    pub fn chn6(&self) -> CHN6_R {
        CHN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline(always)]
    pub fn chn7(&self) -> CHN7_R {
        CHN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline(always)]
    pub fn innsel(&self) -> INNSEL_R {
        INNSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline(always)]
    pub fn vosel(&mut self) -> VOSEL_W {
        VOSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline(always)]
    pub fn msel(&mut self) -> MSEL_W {
        MSEL_W { w: self }
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VRSEL_W {
        VRSEL_W { w: self }
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W {
        DACEN_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline(always)]
    pub fn chn0(&mut self) -> CHN0_W {
        CHN0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline(always)]
    pub fn chn1(&mut self) -> CHN1_W {
        CHN1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline(always)]
    pub fn chn2(&mut self) -> CHN2_W {
        CHN2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline(always)]
    pub fn chn3(&mut self) -> CHN3_W {
        CHN3_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline(always)]
    pub fn chn4(&mut self) -> CHN4_W {
        CHN4_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline(always)]
    pub fn chn5(&mut self) -> CHN5_W {
        CHN5_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline(always)]
    pub fn chn6(&mut self) -> CHN6_W {
        CHN6_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline(always)]
    pub fn chn7(&mut self) -> CHN7_W {
        CHN7_W { w: self }
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline(always)]
    pub fn innsel(&mut self) -> INNSEL_W {
        INNSEL_W { w: self }
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> INPSEL_W {
        INPSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
