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
