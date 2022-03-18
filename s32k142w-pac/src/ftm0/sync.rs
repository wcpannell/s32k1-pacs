#[doc = "Register `SYNC` reader"]
pub struct R(crate::R<SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC` writer"]
pub struct W(crate::W<SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_SPEC>;
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
impl From<crate::W<SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Minimum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMIN_A {
    #[doc = "0: The minimum loading point is disabled."]
    CNTMIN_0 = 0,
    #[doc = "1: The minimum loading point is enabled."]
    CNTMIN_1 = 1,
}
impl From<CNTMIN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMIN` reader - Minimum Loading Point Enable"]
pub struct CNTMIN_R(crate::FieldReader<bool, CNTMIN_A>);
impl CNTMIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNTMIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMIN_A {
        match self.bits {
            false => CNTMIN_A::CNTMIN_0,
            true => CNTMIN_A::CNTMIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNTMIN_0`"]
    #[inline(always)]
    pub fn is_cntmin_0(&self) -> bool {
        **self == CNTMIN_A::CNTMIN_0
    }
    #[doc = "Checks if the value of the field is `CNTMIN_1`"]
    #[inline(always)]
    pub fn is_cntmin_1(&self) -> bool {
        **self == CNTMIN_A::CNTMIN_1
    }
}
impl core::ops::Deref for CNTMIN_R {
    type Target = crate::FieldReader<bool, CNTMIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMIN` writer - Minimum Loading Point Enable"]
pub struct CNTMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMIN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The minimum loading point is disabled."]
    #[inline(always)]
    pub fn cntmin_0(self) -> &'a mut W {
        self.variant(CNTMIN_A::CNTMIN_0)
    }
    #[doc = "The minimum loading point is enabled."]
    #[inline(always)]
    pub fn cntmin_1(self) -> &'a mut W {
        self.variant(CNTMIN_A::CNTMIN_1)
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
#[doc = "Maximum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTMAX_A {
    #[doc = "0: The maximum loading point is disabled."]
    CNTMAX_0 = 0,
    #[doc = "1: The maximum loading point is enabled."]
    CNTMAX_1 = 1,
}
impl From<CNTMAX_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTMAX` reader - Maximum Loading Point Enable"]
pub struct CNTMAX_R(crate::FieldReader<bool, CNTMAX_A>);
impl CNTMAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CNTMAX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX_A {
        match self.bits {
            false => CNTMAX_A::CNTMAX_0,
            true => CNTMAX_A::CNTMAX_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNTMAX_0`"]
    #[inline(always)]
    pub fn is_cntmax_0(&self) -> bool {
        **self == CNTMAX_A::CNTMAX_0
    }
    #[doc = "Checks if the value of the field is `CNTMAX_1`"]
    #[inline(always)]
    pub fn is_cntmax_1(&self) -> bool {
        **self == CNTMAX_A::CNTMAX_1
    }
}
impl core::ops::Deref for CNTMAX_R {
    type Target = crate::FieldReader<bool, CNTMAX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTMAX` writer - Maximum Loading Point Enable"]
pub struct CNTMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTMAX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTMAX_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The maximum loading point is disabled."]
    #[inline(always)]
    pub fn cntmax_0(self) -> &'a mut W {
        self.variant(CNTMAX_A::CNTMAX_0)
    }
    #[doc = "The maximum loading point is enabled."]
    #[inline(always)]
    pub fn cntmax_1(self) -> &'a mut W {
        self.variant(CNTMAX_A::CNTMAX_1)
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
#[doc = "FTM Counter Reinitialization by Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REINIT_A {
    #[doc = "0: FTM counter continues to count normally."]
    REINIT_0 = 0,
    #[doc = "1: FTM counter is updated with its initial value when the selected trigger is detected."]
    REINIT_1 = 1,
}
impl From<REINIT_A> for bool {
    #[inline(always)]
    fn from(variant: REINIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REINIT` reader - FTM Counter Reinitialization by Synchronization"]
pub struct REINIT_R(crate::FieldReader<bool, REINIT_A>);
impl REINIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REINIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REINIT_A {
        match self.bits {
            false => REINIT_A::REINIT_0,
            true => REINIT_A::REINIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REINIT_0`"]
    #[inline(always)]
    pub fn is_reinit_0(&self) -> bool {
        **self == REINIT_A::REINIT_0
    }
    #[doc = "Checks if the value of the field is `REINIT_1`"]
    #[inline(always)]
    pub fn is_reinit_1(&self) -> bool {
        **self == REINIT_A::REINIT_1
    }
}
impl core::ops::Deref for REINIT_R {
    type Target = crate::FieldReader<bool, REINIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REINIT` writer - FTM Counter Reinitialization by Synchronization"]
pub struct REINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> REINIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REINIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM counter continues to count normally."]
    #[inline(always)]
    pub fn reinit_0(self) -> &'a mut W {
        self.variant(REINIT_A::REINIT_0)
    }
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    #[inline(always)]
    pub fn reinit_1(self) -> &'a mut W {
        self.variant(REINIT_A::REINIT_1)
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
#[doc = "Output Mask Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCHOM_A {
    #[doc = "0: OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    SYNCHOM_0 = 0,
    #[doc = "1: OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    SYNCHOM_1 = 1,
}
impl From<SYNCHOM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCHOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCHOM` reader - Output Mask Synchronization"]
pub struct SYNCHOM_R(crate::FieldReader<bool, SYNCHOM_A>);
impl SYNCHOM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCHOM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCHOM_A {
        match self.bits {
            false => SYNCHOM_A::SYNCHOM_0,
            true => SYNCHOM_A::SYNCHOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHOM_0`"]
    #[inline(always)]
    pub fn is_synchom_0(&self) -> bool {
        **self == SYNCHOM_A::SYNCHOM_0
    }
    #[doc = "Checks if the value of the field is `SYNCHOM_1`"]
    #[inline(always)]
    pub fn is_synchom_1(&self) -> bool {
        **self == SYNCHOM_A::SYNCHOM_1
    }
}
impl core::ops::Deref for SYNCHOM_R {
    type Target = crate::FieldReader<bool, SYNCHOM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCHOM` writer - Output Mask Synchronization"]
pub struct SYNCHOM_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCHOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCHOM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the FTM input clock."]
    #[inline(always)]
    pub fn synchom_0(self) -> &'a mut W {
        self.variant(SYNCHOM_A::SYNCHOM_0)
    }
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    #[inline(always)]
    pub fn synchom_1(self) -> &'a mut W {
        self.variant(SYNCHOM_A::SYNCHOM_1)
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
#[doc = "PWM Synchronization Hardware Trigger 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG0_A {
    #[doc = "0: Trigger is disabled."]
    TRIG0_0 = 0,
    #[doc = "1: Trigger is enabled."]
    TRIG0_1 = 1,
}
impl From<TRIG0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG0` reader - PWM Synchronization Hardware Trigger 0"]
pub struct TRIG0_R(crate::FieldReader<bool, TRIG0_A>);
impl TRIG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_A {
        match self.bits {
            false => TRIG0_A::TRIG0_0,
            true => TRIG0_A::TRIG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG0_0`"]
    #[inline(always)]
    pub fn is_trig0_0(&self) -> bool {
        **self == TRIG0_A::TRIG0_0
    }
    #[doc = "Checks if the value of the field is `TRIG0_1`"]
    #[inline(always)]
    pub fn is_trig0_1(&self) -> bool {
        **self == TRIG0_A::TRIG0_1
    }
}
impl core::ops::Deref for TRIG0_R {
    type Target = crate::FieldReader<bool, TRIG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIG0` writer - PWM Synchronization Hardware Trigger 0"]
pub struct TRIG0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn trig0_0(self) -> &'a mut W {
        self.variant(TRIG0_A::TRIG0_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn trig0_1(self) -> &'a mut W {
        self.variant(TRIG0_A::TRIG0_1)
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
#[doc = "PWM Synchronization Hardware Trigger 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG1_A {
    #[doc = "0: Trigger is disabled."]
    TRIG1_0 = 0,
    #[doc = "1: Trigger is enabled."]
    TRIG1_1 = 1,
}
impl From<TRIG1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG1` reader - PWM Synchronization Hardware Trigger 1"]
pub struct TRIG1_R(crate::FieldReader<bool, TRIG1_A>);
impl TRIG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_A {
        match self.bits {
            false => TRIG1_A::TRIG1_0,
            true => TRIG1_A::TRIG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG1_0`"]
    #[inline(always)]
    pub fn is_trig1_0(&self) -> bool {
        **self == TRIG1_A::TRIG1_0
    }
    #[doc = "Checks if the value of the field is `TRIG1_1`"]
    #[inline(always)]
    pub fn is_trig1_1(&self) -> bool {
        **self == TRIG1_A::TRIG1_1
    }
}
impl core::ops::Deref for TRIG1_R {
    type Target = crate::FieldReader<bool, TRIG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIG1` writer - PWM Synchronization Hardware Trigger 1"]
pub struct TRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn trig1_0(self) -> &'a mut W {
        self.variant(TRIG1_A::TRIG1_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn trig1_1(self) -> &'a mut W {
        self.variant(TRIG1_A::TRIG1_1)
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
#[doc = "PWM Synchronization Hardware Trigger 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIG2_A {
    #[doc = "0: Trigger is disabled."]
    TRIG2_0 = 0,
    #[doc = "1: Trigger is enabled."]
    TRIG2_1 = 1,
}
impl From<TRIG2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG2` reader - PWM Synchronization Hardware Trigger 2"]
pub struct TRIG2_R(crate::FieldReader<bool, TRIG2_A>);
impl TRIG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_A {
        match self.bits {
            false => TRIG2_A::TRIG2_0,
            true => TRIG2_A::TRIG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG2_0`"]
    #[inline(always)]
    pub fn is_trig2_0(&self) -> bool {
        **self == TRIG2_A::TRIG2_0
    }
    #[doc = "Checks if the value of the field is `TRIG2_1`"]
    #[inline(always)]
    pub fn is_trig2_1(&self) -> bool {
        **self == TRIG2_A::TRIG2_1
    }
}
impl core::ops::Deref for TRIG2_R {
    type Target = crate::FieldReader<bool, TRIG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIG2` writer - PWM Synchronization Hardware Trigger 2"]
pub struct TRIG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn trig2_0(self) -> &'a mut W {
        self.variant(TRIG2_A::TRIG2_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn trig2_1(self) -> &'a mut W {
        self.variant(TRIG2_A::TRIG2_1)
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
#[doc = "PWM Synchronization Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSYNC_A {
    #[doc = "0: Software trigger is not selected."]
    SWSYNC_0 = 0,
    #[doc = "1: Software trigger is selected."]
    SWSYNC_1 = 1,
}
impl From<SWSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWSYNC` reader - PWM Synchronization Software Trigger"]
pub struct SWSYNC_R(crate::FieldReader<bool, SWSYNC_A>);
impl SWSYNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSYNC_A {
        match self.bits {
            false => SWSYNC_A::SWSYNC_0,
            true => SWSYNC_A::SWSYNC_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWSYNC_0`"]
    #[inline(always)]
    pub fn is_swsync_0(&self) -> bool {
        **self == SWSYNC_A::SWSYNC_0
    }
    #[doc = "Checks if the value of the field is `SWSYNC_1`"]
    #[inline(always)]
    pub fn is_swsync_1(&self) -> bool {
        **self == SWSYNC_A::SWSYNC_1
    }
}
impl core::ops::Deref for SWSYNC_R {
    type Target = crate::FieldReader<bool, SWSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWSYNC` writer - PWM Synchronization Software Trigger"]
pub struct SWSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software trigger is not selected."]
    #[inline(always)]
    pub fn swsync_0(self) -> &'a mut W {
        self.variant(SWSYNC_A::SWSYNC_0)
    }
    #[doc = "Software trigger is selected."]
    #[inline(always)]
    pub fn swsync_1(self) -> &'a mut W {
        self.variant(SWSYNC_A::SWSYNC_1)
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
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmin(&self) -> CNTMIN_R {
        CNTMIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmax(&self) -> CNTMAX_R {
        CNTMAX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline(always)]
    pub fn reinit(&self) -> REINIT_R {
        REINIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    pub fn synchom(&self) -> SYNCHOM_R {
        SYNCHOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    pub fn trig0(&self) -> TRIG0_R {
        TRIG0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    pub fn trig1(&self) -> TRIG1_R {
        TRIG1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    pub fn trig2(&self) -> TRIG2_R {
        TRIG2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmin(&mut self) -> CNTMIN_W {
        CNTMIN_W { w: self }
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmax(&mut self) -> CNTMAX_W {
        CNTMAX_W { w: self }
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization by Synchronization"]
    #[inline(always)]
    pub fn reinit(&mut self) -> REINIT_W {
        REINIT_W { w: self }
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    pub fn synchom(&mut self) -> SYNCHOM_W {
        SYNCHOM_W { w: self }
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    pub fn trig0(&mut self) -> TRIG0_W {
        TRIG0_W { w: self }
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    pub fn trig1(&mut self) -> TRIG1_W {
        TRIG1_W { w: self }
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    pub fn trig2(&mut self) -> TRIG2_W {
        TRIG2_W { w: self }
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    pub fn swsync(&mut self) -> SWSYNC_W {
        SWSYNC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](index.html) module"]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync::R](R) reader structure"]
impl crate::Readable for SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync::W](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
