#[doc = "Register `MISCTRL0` reader"]
pub struct R(crate::R<MISCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCTRL0` writer"]
pub struct W(crate::W<MISCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCTRL0_SPEC>;
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
impl From<crate::W<MISCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "STOP1 monitor bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP1_MONITOR_A {
    #[doc = "0: Bus clock enabled or STOP1 entry aborted"]
    _0 = 0,
    #[doc = "1: STOP1 entry successful"]
    _1 = 1,
}
impl From<STOP1_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: STOP1_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP1_MONITOR` reader - STOP1 monitor bit"]
pub struct STOP1_MONITOR_R(crate::FieldReader<bool, STOP1_MONITOR_A>);
impl STOP1_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP1_MONITOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP1_MONITOR_A {
        match self.bits {
            false => STOP1_MONITOR_A::_0,
            true => STOP1_MONITOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOP1_MONITOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOP1_MONITOR_A::_1
    }
}
impl core::ops::Deref for STOP1_MONITOR_R {
    type Target = crate::FieldReader<bool, STOP1_MONITOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP1_MONITOR` writer - STOP1 monitor bit"]
pub struct STOP1_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP1_MONITOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP1_MONITOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus clock enabled or STOP1 entry aborted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP1_MONITOR_A::_0)
    }
    #[doc = "STOP1 entry successful"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP1_MONITOR_A::_1)
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
#[doc = "STOP2 monitor bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP2_MONITOR_A {
    #[doc = "0: System clock enabled or STOP2 entry aborted"]
    _0 = 0,
    #[doc = "1: STOP2 entry successful"]
    _1 = 1,
}
impl From<STOP2_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: STOP2_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP2_MONITOR` reader - STOP2 monitor bit"]
pub struct STOP2_MONITOR_R(crate::FieldReader<bool, STOP2_MONITOR_A>);
impl STOP2_MONITOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STOP2_MONITOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP2_MONITOR_A {
        match self.bits {
            false => STOP2_MONITOR_A::_0,
            true => STOP2_MONITOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STOP2_MONITOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == STOP2_MONITOR_A::_1
    }
}
impl core::ops::Deref for STOP2_MONITOR_R {
    type Target = crate::FieldReader<bool, STOP2_MONITOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP2_MONITOR` writer - STOP2 monitor bit"]
pub struct STOP2_MONITOR_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP2_MONITOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP2_MONITOR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "System clock enabled or STOP2 entry aborted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOP2_MONITOR_A::_0)
    }
    #[doc = "STOP2 entry successful"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOP2_MONITOR_A::_1)
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
#[doc = "FTM GTB split enable/disable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM_GTB_SPLIT_EN_A {
    #[doc = "0: All the FTMs have a single global time-base"]
    _0 = 0,
    #[doc = "1: FTM0-3 have a common time-base and others have a different common time-base. Please refer 'FTM global time base' in FTM chapter for implementation details."]
    _1 = 1,
}
impl From<FTM_GTB_SPLIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FTM_GTB_SPLIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM_GTB_SPLIT_EN` reader - FTM GTB split enable/disable bit"]
pub struct FTM_GTB_SPLIT_EN_R(crate::FieldReader<bool, FTM_GTB_SPLIT_EN_A>);
impl FTM_GTB_SPLIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM_GTB_SPLIT_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM_GTB_SPLIT_EN_A {
        match self.bits {
            false => FTM_GTB_SPLIT_EN_A::_0,
            true => FTM_GTB_SPLIT_EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM_GTB_SPLIT_EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM_GTB_SPLIT_EN_A::_1
    }
}
impl core::ops::Deref for FTM_GTB_SPLIT_EN_R {
    type Target = crate::FieldReader<bool, FTM_GTB_SPLIT_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM_GTB_SPLIT_EN` writer - FTM GTB split enable/disable bit"]
pub struct FTM_GTB_SPLIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM_GTB_SPLIT_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM_GTB_SPLIT_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "All the FTMs have a single global time-base"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM_GTB_SPLIT_EN_A::_0)
    }
    #[doc = "FTM0-3 have a common time-base and others have a different common time-base. Please refer 'FTM global time base' in FTM chapter for implementation details."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM_GTB_SPLIT_EN_A::_1)
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
#[doc = "FTM0 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM0_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM0_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM0_OBE_CTRL` reader - FTM0 OBE CTRL bit"]
pub struct FTM0_OBE_CTRL_R(crate::FieldReader<bool, FTM0_OBE_CTRL_A>);
impl FTM0_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM0_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_OBE_CTRL_A {
        match self.bits {
            false => FTM0_OBE_CTRL_A::_0,
            true => FTM0_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM0_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM0_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM0_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM0_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0_OBE_CTRL` writer - FTM0 OBE CTRL bit"]
pub struct FTM0_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_OBE_CTRL_A::_1)
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
#[doc = "FTM1 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM1_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM1_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM1_OBE_CTRL` reader - FTM1 OBE CTRL bit"]
pub struct FTM1_OBE_CTRL_R(crate::FieldReader<bool, FTM1_OBE_CTRL_A>);
impl FTM1_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM1_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_OBE_CTRL_A {
        match self.bits {
            false => FTM1_OBE_CTRL_A::_0,
            true => FTM1_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM1_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM1_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM1_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM1_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1_OBE_CTRL` writer - FTM1 OBE CTRL bit"]
pub struct FTM1_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_OBE_CTRL_A::_1)
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
#[doc = "FTM2 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM2_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2_OBE_CTRL` reader - FTM2 OBE CTRL bit"]
pub struct FTM2_OBE_CTRL_R(crate::FieldReader<bool, FTM2_OBE_CTRL_A>);
impl FTM2_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM2_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_OBE_CTRL_A {
        match self.bits {
            false => FTM2_OBE_CTRL_A::_0,
            true => FTM2_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM2_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM2_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2_OBE_CTRL` writer - FTM2 OBE CTRL bit"]
pub struct FTM2_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_OBE_CTRL_A::_1)
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
#[doc = "FTM3 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM3_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM3_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM3_OBE_CTRL` reader - FTM3 OBE CTRL bit"]
pub struct FTM3_OBE_CTRL_R(crate::FieldReader<bool, FTM3_OBE_CTRL_A>);
impl FTM3_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM3_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3_OBE_CTRL_A {
        match self.bits {
            false => FTM3_OBE_CTRL_A::_0,
            true => FTM3_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM3_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM3_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM3_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM3_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM3_OBE_CTRL` writer - FTM3 OBE CTRL bit"]
pub struct FTM3_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3_OBE_CTRL_A::_1)
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
#[doc = "FTM4 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM4_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM4_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM4_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM4_OBE_CTRL` reader - FTM4 OBE CTRL bit"]
pub struct FTM4_OBE_CTRL_R(crate::FieldReader<bool, FTM4_OBE_CTRL_A>);
impl FTM4_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM4_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM4_OBE_CTRL_A {
        match self.bits {
            false => FTM4_OBE_CTRL_A::_0,
            true => FTM4_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM4_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM4_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM4_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM4_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM4_OBE_CTRL` writer - FTM4 OBE CTRL bit"]
pub struct FTM4_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM4_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM4_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM4_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM4_OBE_CTRL_A::_1)
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
#[doc = "FTM5 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM5_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM5_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM5_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM5_OBE_CTRL` reader - FTM5 OBE CTRL bit"]
pub struct FTM5_OBE_CTRL_R(crate::FieldReader<bool, FTM5_OBE_CTRL_A>);
impl FTM5_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM5_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM5_OBE_CTRL_A {
        match self.bits {
            false => FTM5_OBE_CTRL_A::_0,
            true => FTM5_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM5_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM5_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM5_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM5_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM5_OBE_CTRL` writer - FTM5 OBE CTRL bit"]
pub struct FTM5_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM5_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM5_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM5_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM5_OBE_CTRL_A::_1)
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
#[doc = "FTM6 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM6_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM6_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM6_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM6_OBE_CTRL` reader - FTM6 OBE CTRL bit"]
pub struct FTM6_OBE_CTRL_R(crate::FieldReader<bool, FTM6_OBE_CTRL_A>);
impl FTM6_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM6_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM6_OBE_CTRL_A {
        match self.bits {
            false => FTM6_OBE_CTRL_A::_0,
            true => FTM6_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM6_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM6_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM6_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM6_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM6_OBE_CTRL` writer - FTM6 OBE CTRL bit"]
pub struct FTM6_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM6_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM6_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM6_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM6_OBE_CTRL_A::_1)
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
#[doc = "FTM7 OBE CTRL bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM7_OBE_CTRL_A {
    #[doc = "0: The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    _0 = 0,
    #[doc = "1: The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    _1 = 1,
}
impl From<FTM7_OBE_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM7_OBE_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM7_OBE_CTRL` reader - FTM7 OBE CTRL bit"]
pub struct FTM7_OBE_CTRL_R(crate::FieldReader<bool, FTM7_OBE_CTRL_A>);
impl FTM7_OBE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM7_OBE_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM7_OBE_CTRL_A {
        match self.bits {
            false => FTM7_OBE_CTRL_A::_0,
            true => FTM7_OBE_CTRL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM7_OBE_CTRL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM7_OBE_CTRL_A::_1
    }
}
impl core::ops::Deref for FTM7_OBE_CTRL_R {
    type Target = crate::FieldReader<bool, FTM7_OBE_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM7_OBE_CTRL` writer - FTM7 OBE CTRL bit"]
pub struct FTM7_OBE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM7_OBE_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM7_OBE_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The FTM channel output is put to safe state when the FTM counter is enabled and the FTM channel output is enabled by Fault Control (FTM_MODE\\[FAULTM\\]!=2'b00 and FTM_FLTCTRL\\[FSTATE\\]=1'b0) and PWM is enabled (FTM_SC\\[PWMENn\\]
= 1'b1). Otherwise the channel output is tristated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM7_OBE_CTRL_A::_0)
    }
    #[doc = "The FTM channel output state is retained when the channel is in output mode. The output channel is tristated when the channel is in input capture \\[DECAPEN=1'b0, COMBINE=1'b0, MSnB:MSnA=2'b00\\]
or dual edge capture mode \\[DECAPEN=1'b1\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM7_OBE_CTRL_A::_1)
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
#[doc = "Field `RMII_CLK_OBE` reader - RMII CLK OBE bit"]
pub struct RMII_CLK_OBE_R(crate::FieldReader<bool, bool>);
impl RMII_CLK_OBE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_CLK_OBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMII_CLK_OBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_CLK_OBE` writer - RMII CLK OBE bit"]
pub struct RMII_CLK_OBE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_CLK_OBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RMII_CLK_SEL` reader - RMII CLK Select bit"]
pub struct RMII_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl RMII_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMII_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_CLK_SEL` writer - RMII CLK Select bit"]
pub struct RMII_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "QSPI CLK Select bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI_CLK_SEL_A {
    #[doc = "0: QuadSPI internal reference clock is gated."]
    _0 = 0,
    #[doc = "1: QuadSPI internal reference clock is enabled."]
    _1 = 1,
}
impl From<QSPI_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPI_CLK_SEL` reader - QSPI CLK Select bit"]
pub struct QSPI_CLK_SEL_R(crate::FieldReader<bool, QSPI_CLK_SEL_A>);
impl QSPI_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPI_CLK_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI_CLK_SEL_A {
        match self.bits {
            false => QSPI_CLK_SEL_A::_0,
            true => QSPI_CLK_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == QSPI_CLK_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == QSPI_CLK_SEL_A::_1
    }
}
impl core::ops::Deref for QSPI_CLK_SEL_R {
    type Target = crate::FieldReader<bool, QSPI_CLK_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI_CLK_SEL` writer - QSPI CLK Select bit"]
pub struct QSPI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI_CLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPI_CLK_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "QuadSPI internal reference clock is gated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(QSPI_CLK_SEL_A::_0)
    }
    #[doc = "QuadSPI internal reference clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(QSPI_CLK_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - STOP1 monitor bit"]
    #[inline(always)]
    pub fn stop1_monitor(&self) -> STOP1_MONITOR_R {
        STOP1_MONITOR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - STOP2 monitor bit"]
    #[inline(always)]
    pub fn stop2_monitor(&self) -> STOP2_MONITOR_R {
        STOP2_MONITOR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FTM GTB split enable/disable bit"]
    #[inline(always)]
    pub fn ftm_gtb_split_en(&self) -> FTM_GTB_SPLIT_EN_R {
        FTM_GTB_SPLIT_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm0_obe_ctrl(&self) -> FTM0_OBE_CTRL_R {
        FTM0_OBE_CTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm1_obe_ctrl(&self) -> FTM1_OBE_CTRL_R {
        FTM1_OBE_CTRL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm2_obe_ctrl(&self) -> FTM2_OBE_CTRL_R {
        FTM2_OBE_CTRL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm3_obe_ctrl(&self) -> FTM3_OBE_CTRL_R {
        FTM3_OBE_CTRL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - FTM4 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm4_obe_ctrl(&self) -> FTM4_OBE_CTRL_R {
        FTM4_OBE_CTRL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - FTM5 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm5_obe_ctrl(&self) -> FTM5_OBE_CTRL_R {
        FTM5_OBE_CTRL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - FTM6 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm6_obe_ctrl(&self) -> FTM6_OBE_CTRL_R {
        FTM6_OBE_CTRL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - FTM7 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm7_obe_ctrl(&self) -> FTM7_OBE_CTRL_R {
        FTM7_OBE_CTRL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RMII CLK OBE bit"]
    #[inline(always)]
    pub fn rmii_clk_obe(&self) -> RMII_CLK_OBE_R {
        RMII_CLK_OBE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RMII CLK Select bit"]
    #[inline(always)]
    pub fn rmii_clk_sel(&self) -> RMII_CLK_SEL_R {
        RMII_CLK_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - QSPI CLK Select bit"]
    #[inline(always)]
    pub fn qspi_clk_sel(&self) -> QSPI_CLK_SEL_R {
        QSPI_CLK_SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - STOP1 monitor bit"]
    #[inline(always)]
    pub fn stop1_monitor(&mut self) -> STOP1_MONITOR_W {
        STOP1_MONITOR_W { w: self }
    }
    #[doc = "Bit 10 - STOP2 monitor bit"]
    #[inline(always)]
    pub fn stop2_monitor(&mut self) -> STOP2_MONITOR_W {
        STOP2_MONITOR_W { w: self }
    }
    #[doc = "Bit 14 - FTM GTB split enable/disable bit"]
    #[inline(always)]
    pub fn ftm_gtb_split_en(&mut self) -> FTM_GTB_SPLIT_EN_W {
        FTM_GTB_SPLIT_EN_W { w: self }
    }
    #[doc = "Bit 16 - FTM0 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm0_obe_ctrl(&mut self) -> FTM0_OBE_CTRL_W {
        FTM0_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 17 - FTM1 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm1_obe_ctrl(&mut self) -> FTM1_OBE_CTRL_W {
        FTM1_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 18 - FTM2 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm2_obe_ctrl(&mut self) -> FTM2_OBE_CTRL_W {
        FTM2_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 19 - FTM3 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm3_obe_ctrl(&mut self) -> FTM3_OBE_CTRL_W {
        FTM3_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 20 - FTM4 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm4_obe_ctrl(&mut self) -> FTM4_OBE_CTRL_W {
        FTM4_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 21 - FTM5 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm5_obe_ctrl(&mut self) -> FTM5_OBE_CTRL_W {
        FTM5_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 22 - FTM6 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm6_obe_ctrl(&mut self) -> FTM6_OBE_CTRL_W {
        FTM6_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 23 - FTM7 OBE CTRL bit"]
    #[inline(always)]
    pub fn ftm7_obe_ctrl(&mut self) -> FTM7_OBE_CTRL_W {
        FTM7_OBE_CTRL_W { w: self }
    }
    #[doc = "Bit 24 - RMII CLK OBE bit"]
    #[inline(always)]
    pub fn rmii_clk_obe(&mut self) -> RMII_CLK_OBE_W {
        RMII_CLK_OBE_W { w: self }
    }
    #[doc = "Bit 25 - RMII CLK Select bit"]
    #[inline(always)]
    pub fn rmii_clk_sel(&mut self) -> RMII_CLK_SEL_W {
        RMII_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 26 - QSPI CLK Select bit"]
    #[inline(always)]
    pub fn qspi_clk_sel(&mut self) -> QSPI_CLK_SEL_W {
        QSPI_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misctrl0](index.html) module"]
pub struct MISCTRL0_SPEC;
impl crate::RegisterSpec for MISCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misctrl0::R](R) reader structure"]
impl crate::Readable for MISCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misctrl0::W](W) writer structure"]
impl crate::Writable for MISCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISCTRL0 to value 0"]
impl crate::Resettable for MISCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
