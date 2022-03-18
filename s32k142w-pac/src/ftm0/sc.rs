#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Divide by 1"]
    PS_0 = 0,
    #[doc = "1: Divide by 2"]
    PS_1 = 1,
    #[doc = "2: Divide by 4"]
    PS_2 = 2,
    #[doc = "3: Divide by 8"]
    PS_3 = 3,
    #[doc = "4: Divide by 16"]
    PS_4 = 4,
    #[doc = "5: Divide by 32"]
    PS_5 = 5,
    #[doc = "6: Divide by 64"]
    PS_6 = 6,
    #[doc = "7: Divide by 128"]
    PS_7 = 7,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PS` reader - Prescale Factor Selection"]
pub struct PS_R(crate::FieldReader<u8, PS_A>);
impl PS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::PS_0,
            1 => PS_A::PS_1,
            2 => PS_A::PS_2,
            3 => PS_A::PS_3,
            4 => PS_A::PS_4,
            5 => PS_A::PS_5,
            6 => PS_A::PS_6,
            7 => PS_A::PS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS_0`"]
    #[inline(always)]
    pub fn is_ps_0(&self) -> bool {
        **self == PS_A::PS_0
    }
    #[doc = "Checks if the value of the field is `PS_1`"]
    #[inline(always)]
    pub fn is_ps_1(&self) -> bool {
        **self == PS_A::PS_1
    }
    #[doc = "Checks if the value of the field is `PS_2`"]
    #[inline(always)]
    pub fn is_ps_2(&self) -> bool {
        **self == PS_A::PS_2
    }
    #[doc = "Checks if the value of the field is `PS_3`"]
    #[inline(always)]
    pub fn is_ps_3(&self) -> bool {
        **self == PS_A::PS_3
    }
    #[doc = "Checks if the value of the field is `PS_4`"]
    #[inline(always)]
    pub fn is_ps_4(&self) -> bool {
        **self == PS_A::PS_4
    }
    #[doc = "Checks if the value of the field is `PS_5`"]
    #[inline(always)]
    pub fn is_ps_5(&self) -> bool {
        **self == PS_A::PS_5
    }
    #[doc = "Checks if the value of the field is `PS_6`"]
    #[inline(always)]
    pub fn is_ps_6(&self) -> bool {
        **self == PS_A::PS_6
    }
    #[doc = "Checks if the value of the field is `PS_7`"]
    #[inline(always)]
    pub fn is_ps_7(&self) -> bool {
        **self == PS_A::PS_7
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<u8, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Prescale Factor Selection"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn ps_0(self) -> &'a mut W {
        self.variant(PS_A::PS_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn ps_1(self) -> &'a mut W {
        self.variant(PS_A::PS_1)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn ps_2(self) -> &'a mut W {
        self.variant(PS_A::PS_2)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn ps_3(self) -> &'a mut W {
        self.variant(PS_A::PS_3)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn ps_4(self) -> &'a mut W {
        self.variant(PS_A::PS_4)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn ps_5(self) -> &'a mut W {
        self.variant(PS_A::PS_5)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn ps_6(self) -> &'a mut W {
        self.variant(PS_A::PS_6)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn ps_7(self) -> &'a mut W {
        self.variant(PS_A::PS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: No clock selected. This in effect disables the FTM counter."]
    CLKS_0 = 0,
    #[doc = "1: FTM input clock"]
    CLKS_1 = 1,
    #[doc = "2: Fixed frequency clock"]
    CLKS_2 = 2,
    #[doc = "3: External clock"]
    CLKS_3 = 3,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKS` reader - Clock Source Selection"]
pub struct CLKS_R(crate::FieldReader<u8, CLKS_A>);
impl CLKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::CLKS_0,
            1 => CLKS_A::CLKS_1,
            2 => CLKS_A::CLKS_2,
            3 => CLKS_A::CLKS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLKS_0`"]
    #[inline(always)]
    pub fn is_clks_0(&self) -> bool {
        **self == CLKS_A::CLKS_0
    }
    #[doc = "Checks if the value of the field is `CLKS_1`"]
    #[inline(always)]
    pub fn is_clks_1(&self) -> bool {
        **self == CLKS_A::CLKS_1
    }
    #[doc = "Checks if the value of the field is `CLKS_2`"]
    #[inline(always)]
    pub fn is_clks_2(&self) -> bool {
        **self == CLKS_A::CLKS_2
    }
    #[doc = "Checks if the value of the field is `CLKS_3`"]
    #[inline(always)]
    pub fn is_clks_3(&self) -> bool {
        **self == CLKS_A::CLKS_3
    }
}
impl core::ops::Deref for CLKS_R {
    type Target = crate::FieldReader<u8, CLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKS` writer - Clock Source Selection"]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    #[inline(always)]
    pub fn clks_0(self) -> &'a mut W {
        self.variant(CLKS_A::CLKS_0)
    }
    #[doc = "FTM input clock"]
    #[inline(always)]
    pub fn clks_1(self) -> &'a mut W {
        self.variant(CLKS_A::CLKS_1)
    }
    #[doc = "Fixed frequency clock"]
    #[inline(always)]
    pub fn clks_2(self) -> &'a mut W {
        self.variant(CLKS_A::CLKS_2)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn clks_3(self) -> &'a mut W {
        self.variant(CLKS_A::CLKS_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPWMS_A {
    #[doc = "0: FTM counter operates in Up Counting mode."]
    CPWMS_0 = 0,
    #[doc = "1: FTM counter operates in Up-Down Counting mode."]
    CPWMS_1 = 1,
}
impl From<CPWMS_A> for bool {
    #[inline(always)]
    fn from(variant: CPWMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPWMS` reader - Center-Aligned PWM Select"]
pub struct CPWMS_R(crate::FieldReader<bool, CPWMS_A>);
impl CPWMS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPWMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPWMS_A {
        match self.bits {
            false => CPWMS_A::CPWMS_0,
            true => CPWMS_A::CPWMS_1,
        }
    }
    #[doc = "Checks if the value of the field is `CPWMS_0`"]
    #[inline(always)]
    pub fn is_cpwms_0(&self) -> bool {
        **self == CPWMS_A::CPWMS_0
    }
    #[doc = "Checks if the value of the field is `CPWMS_1`"]
    #[inline(always)]
    pub fn is_cpwms_1(&self) -> bool {
        **self == CPWMS_A::CPWMS_1
    }
}
impl core::ops::Deref for CPWMS_R {
    type Target = crate::FieldReader<bool, CPWMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPWMS` writer - Center-Aligned PWM Select"]
pub struct CPWMS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPWMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPWMS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM counter operates in Up Counting mode."]
    #[inline(always)]
    pub fn cpwms_0(self) -> &'a mut W {
        self.variant(CPWMS_A::CPWMS_0)
    }
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    #[inline(always)]
    pub fn cpwms_1(self) -> &'a mut W {
        self.variant(CPWMS_A::CPWMS_1)
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
#[doc = "Reload Point Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: Reload point interrupt is disabled."]
    RIE_0 = 0,
    #[doc = "1: Reload point interrupt is enabled."]
    RIE_1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Reload Point Interrupt Enable"]
pub struct RIE_R(crate::FieldReader<bool, RIE_A>);
impl RIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::RIE_0,
            true => RIE_A::RIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RIE_0`"]
    #[inline(always)]
    pub fn is_rie_0(&self) -> bool {
        **self == RIE_A::RIE_0
    }
    #[doc = "Checks if the value of the field is `RIE_1`"]
    #[inline(always)]
    pub fn is_rie_1(&self) -> bool {
        **self == RIE_A::RIE_1
    }
}
impl core::ops::Deref for RIE_R {
    type Target = crate::FieldReader<bool, RIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIE` writer - Reload Point Interrupt Enable"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reload point interrupt is disabled."]
    #[inline(always)]
    pub fn rie_0(self) -> &'a mut W {
        self.variant(RIE_A::RIE_0)
    }
    #[doc = "Reload point interrupt is enabled."]
    #[inline(always)]
    pub fn rie_1(self) -> &'a mut W {
        self.variant(RIE_A::RIE_1)
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
#[doc = "Reload Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF_A {
    #[doc = "0: A selected reload point did not happen."]
    RF_0 = 0,
    #[doc = "1: A selected reload point happened."]
    RF_1 = 1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RF` reader - Reload Flag"]
pub struct RF_R(crate::FieldReader<bool, RF_A>);
impl RF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_A {
        match self.bits {
            false => RF_A::RF_0,
            true => RF_A::RF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RF_0`"]
    #[inline(always)]
    pub fn is_rf_0(&self) -> bool {
        **self == RF_A::RF_0
    }
    #[doc = "Checks if the value of the field is `RF_1`"]
    #[inline(always)]
    pub fn is_rf_1(&self) -> bool {
        **self == RF_A::RF_1
    }
}
impl core::ops::Deref for RF_R {
    type Target = crate::FieldReader<bool, RF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF` writer - Reload Flag"]
pub struct RF_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A selected reload point did not happen."]
    #[inline(always)]
    pub fn rf_0(self) -> &'a mut W {
        self.variant(RF_A::RF_0)
    }
    #[doc = "A selected reload point happened."]
    #[inline(always)]
    pub fn rf_1(self) -> &'a mut W {
        self.variant(RF_A::RF_1)
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
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOIE_A {
    #[doc = "0: Disable TOF interrupts. Use software polling."]
    TOIE_0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    TOIE_1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Timer Overflow Interrupt Enable"]
pub struct TOIE_R(crate::FieldReader<bool, TOIE_A>);
impl TOIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::TOIE_0,
            true => TOIE_A::TOIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOIE_0`"]
    #[inline(always)]
    pub fn is_toie_0(&self) -> bool {
        **self == TOIE_A::TOIE_0
    }
    #[doc = "Checks if the value of the field is `TOIE_1`"]
    #[inline(always)]
    pub fn is_toie_1(&self) -> bool {
        **self == TOIE_A::TOIE_1
    }
}
impl core::ops::Deref for TOIE_R {
    type Target = crate::FieldReader<bool, TOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOIE` writer - Timer Overflow Interrupt Enable"]
pub struct TOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline(always)]
    pub fn toie_0(self) -> &'a mut W {
        self.variant(TOIE_A::TOIE_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn toie_1(self) -> &'a mut W {
        self.variant(TOIE_A::TOIE_1)
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
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOF_A {
    #[doc = "0: FTM counter has not overflowed."]
    TOF_0 = 0,
    #[doc = "1: FTM counter has overflowed."]
    TOF_1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub struct TOF_R(crate::FieldReader<bool, TOF_A>);
impl TOF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::TOF_0,
            true => TOF_A::TOF_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOF_0`"]
    #[inline(always)]
    pub fn is_tof_0(&self) -> bool {
        **self == TOF_A::TOF_0
    }
    #[doc = "Checks if the value of the field is `TOF_1`"]
    #[inline(always)]
    pub fn is_tof_1(&self) -> bool {
        **self == TOF_A::TOF_1
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, TOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub struct TOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM counter has not overflowed."]
    #[inline(always)]
    pub fn tof_0(self) -> &'a mut W {
        self.variant(TOF_A::TOF_0)
    }
    #[doc = "FTM counter has overflowed."]
    #[inline(always)]
    pub fn tof_1(self) -> &'a mut W {
        self.variant(TOF_A::TOF_1)
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
#[doc = "Channel 0 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN0_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN0_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN0_1 = 1,
}
impl From<PWMEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN0` reader - Channel 0 PWM enable bit"]
pub struct PWMEN0_R(crate::FieldReader<bool, PWMEN0_A>);
impl PWMEN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN0_A {
        match self.bits {
            false => PWMEN0_A::PWMEN0_0,
            true => PWMEN0_A::PWMEN0_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN0_0`"]
    #[inline(always)]
    pub fn is_pwmen0_0(&self) -> bool {
        **self == PWMEN0_A::PWMEN0_0
    }
    #[doc = "Checks if the value of the field is `PWMEN0_1`"]
    #[inline(always)]
    pub fn is_pwmen0_1(&self) -> bool {
        **self == PWMEN0_A::PWMEN0_1
    }
}
impl core::ops::Deref for PWMEN0_R {
    type Target = crate::FieldReader<bool, PWMEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN0` writer - Channel 0 PWM enable bit"]
pub struct PWMEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen0_0(self) -> &'a mut W {
        self.variant(PWMEN0_A::PWMEN0_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen0_1(self) -> &'a mut W {
        self.variant(PWMEN0_A::PWMEN0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Channel 1 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN1_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN1_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN1_1 = 1,
}
impl From<PWMEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN1` reader - Channel 1 PWM enable bit"]
pub struct PWMEN1_R(crate::FieldReader<bool, PWMEN1_A>);
impl PWMEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN1_A {
        match self.bits {
            false => PWMEN1_A::PWMEN1_0,
            true => PWMEN1_A::PWMEN1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN1_0`"]
    #[inline(always)]
    pub fn is_pwmen1_0(&self) -> bool {
        **self == PWMEN1_A::PWMEN1_0
    }
    #[doc = "Checks if the value of the field is `PWMEN1_1`"]
    #[inline(always)]
    pub fn is_pwmen1_1(&self) -> bool {
        **self == PWMEN1_A::PWMEN1_1
    }
}
impl core::ops::Deref for PWMEN1_R {
    type Target = crate::FieldReader<bool, PWMEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN1` writer - Channel 1 PWM enable bit"]
pub struct PWMEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen1_0(self) -> &'a mut W {
        self.variant(PWMEN1_A::PWMEN1_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen1_1(self) -> &'a mut W {
        self.variant(PWMEN1_A::PWMEN1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Channel 2 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN2_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN2_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN2_1 = 1,
}
impl From<PWMEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN2` reader - Channel 2 PWM enable bit"]
pub struct PWMEN2_R(crate::FieldReader<bool, PWMEN2_A>);
impl PWMEN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN2_A {
        match self.bits {
            false => PWMEN2_A::PWMEN2_0,
            true => PWMEN2_A::PWMEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN2_0`"]
    #[inline(always)]
    pub fn is_pwmen2_0(&self) -> bool {
        **self == PWMEN2_A::PWMEN2_0
    }
    #[doc = "Checks if the value of the field is `PWMEN2_1`"]
    #[inline(always)]
    pub fn is_pwmen2_1(&self) -> bool {
        **self == PWMEN2_A::PWMEN2_1
    }
}
impl core::ops::Deref for PWMEN2_R {
    type Target = crate::FieldReader<bool, PWMEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN2` writer - Channel 2 PWM enable bit"]
pub struct PWMEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen2_0(self) -> &'a mut W {
        self.variant(PWMEN2_A::PWMEN2_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen2_1(self) -> &'a mut W {
        self.variant(PWMEN2_A::PWMEN2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Channel 3 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN3_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN3_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN3_1 = 1,
}
impl From<PWMEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN3` reader - Channel 3 PWM enable bit"]
pub struct PWMEN3_R(crate::FieldReader<bool, PWMEN3_A>);
impl PWMEN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN3_A {
        match self.bits {
            false => PWMEN3_A::PWMEN3_0,
            true => PWMEN3_A::PWMEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN3_0`"]
    #[inline(always)]
    pub fn is_pwmen3_0(&self) -> bool {
        **self == PWMEN3_A::PWMEN3_0
    }
    #[doc = "Checks if the value of the field is `PWMEN3_1`"]
    #[inline(always)]
    pub fn is_pwmen3_1(&self) -> bool {
        **self == PWMEN3_A::PWMEN3_1
    }
}
impl core::ops::Deref for PWMEN3_R {
    type Target = crate::FieldReader<bool, PWMEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN3` writer - Channel 3 PWM enable bit"]
pub struct PWMEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen3_0(self) -> &'a mut W {
        self.variant(PWMEN3_A::PWMEN3_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen3_1(self) -> &'a mut W {
        self.variant(PWMEN3_A::PWMEN3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Channel 4 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN4_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN4_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN4_1 = 1,
}
impl From<PWMEN4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN4` reader - Channel 4 PWM enable bit"]
pub struct PWMEN4_R(crate::FieldReader<bool, PWMEN4_A>);
impl PWMEN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN4_A {
        match self.bits {
            false => PWMEN4_A::PWMEN4_0,
            true => PWMEN4_A::PWMEN4_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN4_0`"]
    #[inline(always)]
    pub fn is_pwmen4_0(&self) -> bool {
        **self == PWMEN4_A::PWMEN4_0
    }
    #[doc = "Checks if the value of the field is `PWMEN4_1`"]
    #[inline(always)]
    pub fn is_pwmen4_1(&self) -> bool {
        **self == PWMEN4_A::PWMEN4_1
    }
}
impl core::ops::Deref for PWMEN4_R {
    type Target = crate::FieldReader<bool, PWMEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN4` writer - Channel 4 PWM enable bit"]
pub struct PWMEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen4_0(self) -> &'a mut W {
        self.variant(PWMEN4_A::PWMEN4_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen4_1(self) -> &'a mut W {
        self.variant(PWMEN4_A::PWMEN4_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Channel 5 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN5_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN5_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN5_1 = 1,
}
impl From<PWMEN5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN5` reader - Channel 5 PWM enable bit"]
pub struct PWMEN5_R(crate::FieldReader<bool, PWMEN5_A>);
impl PWMEN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN5_A {
        match self.bits {
            false => PWMEN5_A::PWMEN5_0,
            true => PWMEN5_A::PWMEN5_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN5_0`"]
    #[inline(always)]
    pub fn is_pwmen5_0(&self) -> bool {
        **self == PWMEN5_A::PWMEN5_0
    }
    #[doc = "Checks if the value of the field is `PWMEN5_1`"]
    #[inline(always)]
    pub fn is_pwmen5_1(&self) -> bool {
        **self == PWMEN5_A::PWMEN5_1
    }
}
impl core::ops::Deref for PWMEN5_R {
    type Target = crate::FieldReader<bool, PWMEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN5` writer - Channel 5 PWM enable bit"]
pub struct PWMEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen5_0(self) -> &'a mut W {
        self.variant(PWMEN5_A::PWMEN5_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen5_1(self) -> &'a mut W {
        self.variant(PWMEN5_A::PWMEN5_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Channel 6 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN6_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN6_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN6_1 = 1,
}
impl From<PWMEN6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN6` reader - Channel 6 PWM enable bit"]
pub struct PWMEN6_R(crate::FieldReader<bool, PWMEN6_A>);
impl PWMEN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN6_A {
        match self.bits {
            false => PWMEN6_A::PWMEN6_0,
            true => PWMEN6_A::PWMEN6_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN6_0`"]
    #[inline(always)]
    pub fn is_pwmen6_0(&self) -> bool {
        **self == PWMEN6_A::PWMEN6_0
    }
    #[doc = "Checks if the value of the field is `PWMEN6_1`"]
    #[inline(always)]
    pub fn is_pwmen6_1(&self) -> bool {
        **self == PWMEN6_A::PWMEN6_1
    }
}
impl core::ops::Deref for PWMEN6_R {
    type Target = crate::FieldReader<bool, PWMEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN6` writer - Channel 6 PWM enable bit"]
pub struct PWMEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen6_0(self) -> &'a mut W {
        self.variant(PWMEN6_A::PWMEN6_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen6_1(self) -> &'a mut W {
        self.variant(PWMEN6_A::PWMEN6_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Channel 7 PWM enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMEN7_A {
    #[doc = "0: Channel output port is disabled."]
    PWMEN7_0 = 0,
    #[doc = "1: Channel output port is enabled."]
    PWMEN7_1 = 1,
}
impl From<PWMEN7_A> for bool {
    #[inline(always)]
    fn from(variant: PWMEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN7` reader - Channel 7 PWM enable bit"]
pub struct PWMEN7_R(crate::FieldReader<bool, PWMEN7_A>);
impl PWMEN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWMEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMEN7_A {
        match self.bits {
            false => PWMEN7_A::PWMEN7_0,
            true => PWMEN7_A::PWMEN7_1,
        }
    }
    #[doc = "Checks if the value of the field is `PWMEN7_0`"]
    #[inline(always)]
    pub fn is_pwmen7_0(&self) -> bool {
        **self == PWMEN7_A::PWMEN7_0
    }
    #[doc = "Checks if the value of the field is `PWMEN7_1`"]
    #[inline(always)]
    pub fn is_pwmen7_1(&self) -> bool {
        **self == PWMEN7_A::PWMEN7_1
    }
}
impl core::ops::Deref for PWMEN7_R {
    type Target = crate::FieldReader<bool, PWMEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMEN7` writer - Channel 7 PWM enable bit"]
pub struct PWMEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel output port is disabled."]
    #[inline(always)]
    pub fn pwmen7_0(self) -> &'a mut W {
        self.variant(PWMEN7_A::PWMEN7_0)
    }
    #[doc = "Channel output port is enabled."]
    #[inline(always)]
    pub fn pwmen7_1(self) -> &'a mut W {
        self.variant(PWMEN7_A::PWMEN7_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Filter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTPS_A {
    #[doc = "0: Divide by 1"]
    FLTPS_0 = 0,
    #[doc = "1: Divide by 2"]
    FLTPS_1 = 1,
    #[doc = "2: Divide by 3"]
    FLTPS_2 = 2,
    #[doc = "3: Divide by 4"]
    FLTPS_3 = 3,
    #[doc = "4: Divide by 5"]
    FLTPS_4 = 4,
    #[doc = "5: Divide by 6"]
    FLTPS_5 = 5,
    #[doc = "6: Divide by 7"]
    FLTPS_6 = 6,
    #[doc = "7: Divide by 8"]
    FLTPS_7 = 7,
    #[doc = "8: Divide by 9"]
    FLTPS_8 = 8,
    #[doc = "9: Divide by 10"]
    FLTPS_9 = 9,
    #[doc = "10: Divide by 11"]
    FLTPS_10 = 10,
    #[doc = "11: Divide by 12"]
    FLTPS_11 = 11,
    #[doc = "12: Divide by 13"]
    FLTPS_12 = 12,
    #[doc = "13: Divide by 14"]
    FLTPS_13 = 13,
    #[doc = "14: Divide by 15"]
    FLTPS_14 = 14,
    #[doc = "15: Divide by 16"]
    FLTPS_15 = 15,
}
impl From<FLTPS_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTPS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTPS` reader - Filter Prescaler"]
pub struct FLTPS_R(crate::FieldReader<u8, FLTPS_A>);
impl FLTPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLTPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPS_A {
        match self.bits {
            0 => FLTPS_A::FLTPS_0,
            1 => FLTPS_A::FLTPS_1,
            2 => FLTPS_A::FLTPS_2,
            3 => FLTPS_A::FLTPS_3,
            4 => FLTPS_A::FLTPS_4,
            5 => FLTPS_A::FLTPS_5,
            6 => FLTPS_A::FLTPS_6,
            7 => FLTPS_A::FLTPS_7,
            8 => FLTPS_A::FLTPS_8,
            9 => FLTPS_A::FLTPS_9,
            10 => FLTPS_A::FLTPS_10,
            11 => FLTPS_A::FLTPS_11,
            12 => FLTPS_A::FLTPS_12,
            13 => FLTPS_A::FLTPS_13,
            14 => FLTPS_A::FLTPS_14,
            15 => FLTPS_A::FLTPS_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLTPS_0`"]
    #[inline(always)]
    pub fn is_fltps_0(&self) -> bool {
        **self == FLTPS_A::FLTPS_0
    }
    #[doc = "Checks if the value of the field is `FLTPS_1`"]
    #[inline(always)]
    pub fn is_fltps_1(&self) -> bool {
        **self == FLTPS_A::FLTPS_1
    }
    #[doc = "Checks if the value of the field is `FLTPS_2`"]
    #[inline(always)]
    pub fn is_fltps_2(&self) -> bool {
        **self == FLTPS_A::FLTPS_2
    }
    #[doc = "Checks if the value of the field is `FLTPS_3`"]
    #[inline(always)]
    pub fn is_fltps_3(&self) -> bool {
        **self == FLTPS_A::FLTPS_3
    }
    #[doc = "Checks if the value of the field is `FLTPS_4`"]
    #[inline(always)]
    pub fn is_fltps_4(&self) -> bool {
        **self == FLTPS_A::FLTPS_4
    }
    #[doc = "Checks if the value of the field is `FLTPS_5`"]
    #[inline(always)]
    pub fn is_fltps_5(&self) -> bool {
        **self == FLTPS_A::FLTPS_5
    }
    #[doc = "Checks if the value of the field is `FLTPS_6`"]
    #[inline(always)]
    pub fn is_fltps_6(&self) -> bool {
        **self == FLTPS_A::FLTPS_6
    }
    #[doc = "Checks if the value of the field is `FLTPS_7`"]
    #[inline(always)]
    pub fn is_fltps_7(&self) -> bool {
        **self == FLTPS_A::FLTPS_7
    }
    #[doc = "Checks if the value of the field is `FLTPS_8`"]
    #[inline(always)]
    pub fn is_fltps_8(&self) -> bool {
        **self == FLTPS_A::FLTPS_8
    }
    #[doc = "Checks if the value of the field is `FLTPS_9`"]
    #[inline(always)]
    pub fn is_fltps_9(&self) -> bool {
        **self == FLTPS_A::FLTPS_9
    }
    #[doc = "Checks if the value of the field is `FLTPS_10`"]
    #[inline(always)]
    pub fn is_fltps_10(&self) -> bool {
        **self == FLTPS_A::FLTPS_10
    }
    #[doc = "Checks if the value of the field is `FLTPS_11`"]
    #[inline(always)]
    pub fn is_fltps_11(&self) -> bool {
        **self == FLTPS_A::FLTPS_11
    }
    #[doc = "Checks if the value of the field is `FLTPS_12`"]
    #[inline(always)]
    pub fn is_fltps_12(&self) -> bool {
        **self == FLTPS_A::FLTPS_12
    }
    #[doc = "Checks if the value of the field is `FLTPS_13`"]
    #[inline(always)]
    pub fn is_fltps_13(&self) -> bool {
        **self == FLTPS_A::FLTPS_13
    }
    #[doc = "Checks if the value of the field is `FLTPS_14`"]
    #[inline(always)]
    pub fn is_fltps_14(&self) -> bool {
        **self == FLTPS_A::FLTPS_14
    }
    #[doc = "Checks if the value of the field is `FLTPS_15`"]
    #[inline(always)]
    pub fn is_fltps_15(&self) -> bool {
        **self == FLTPS_A::FLTPS_15
    }
}
impl core::ops::Deref for FLTPS_R {
    type Target = crate::FieldReader<u8, FLTPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTPS` writer - Filter Prescaler"]
pub struct FLTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLTPS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn fltps_0(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_0)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn fltps_1(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_1)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn fltps_2(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn fltps_3(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_3)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn fltps_4(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_4)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn fltps_5(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_5)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn fltps_6(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_6)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn fltps_7(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_7)
    }
    #[doc = "Divide by 9"]
    #[inline(always)]
    pub fn fltps_8(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_8)
    }
    #[doc = "Divide by 10"]
    #[inline(always)]
    pub fn fltps_9(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_9)
    }
    #[doc = "Divide by 11"]
    #[inline(always)]
    pub fn fltps_10(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_10)
    }
    #[doc = "Divide by 12"]
    #[inline(always)]
    pub fn fltps_11(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_11)
    }
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn fltps_12(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_12)
    }
    #[doc = "Divide by 14"]
    #[inline(always)]
    pub fn fltps_13(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_13)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn fltps_14(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_14)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn fltps_15(self) -> &'a mut W {
        self.variant(FLTPS_A::FLTPS_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CPWMS_R {
        CPWMS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen0(&self) -> PWMEN0_R {
        PWMEN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen1(&self) -> PWMEN1_R {
        PWMEN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen2(&self) -> PWMEN2_R {
        PWMEN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen3(&self) -> PWMEN3_R {
        PWMEN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen4(&self) -> PWMEN4_R {
        PWMEN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen5(&self) -> PWMEN5_R {
        PWMEN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen6(&self) -> PWMEN6_R {
        PWMEN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen7(&self) -> PWMEN7_R {
        PWMEN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline(always)]
    pub fn fltps(&self) -> FLTPS_R {
        FLTPS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&mut self) -> CPWMS_W {
        CPWMS_W { w: self }
    }
    #[doc = "Bit 6 - Reload Point Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 7 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&mut self) -> RF_W {
        RF_W { w: self }
    }
    #[doc = "Bit 8 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&mut self) -> TOIE_W {
        TOIE_W { w: self }
    }
    #[doc = "Bit 9 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&mut self) -> TOF_W {
        TOF_W { w: self }
    }
    #[doc = "Bit 16 - Channel 0 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen0(&mut self) -> PWMEN0_W {
        PWMEN0_W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen1(&mut self) -> PWMEN1_W {
        PWMEN1_W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen2(&mut self) -> PWMEN2_W {
        PWMEN2_W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen3(&mut self) -> PWMEN3_W {
        PWMEN3_W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen4(&mut self) -> PWMEN4_W {
        PWMEN4_W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen5(&mut self) -> PWMEN5_W {
        PWMEN5_W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen6(&mut self) -> PWMEN6_W {
        PWMEN6_W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 PWM enable bit"]
    #[inline(always)]
    pub fn pwmen7(&mut self) -> PWMEN7_W {
        PWMEN7_W { w: self }
    }
    #[doc = "Bits 24:27 - Filter Prescaler"]
    #[inline(always)]
    pub fn fltps(&mut self) -> FLTPS_W {
        FLTPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
