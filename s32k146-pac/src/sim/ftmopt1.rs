#[doc = "Register `FTMOPT1` reader"]
pub struct R(crate::R<FTMOPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTMOPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTMOPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTMOPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTMOPT1` writer"]
pub struct W(crate::W<FTMOPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTMOPT1_SPEC>;
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
impl From<crate::W<FTMOPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTMOPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTM0SYNCBIT` reader - FTM0 Sync Bit"]
pub struct FTM0SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM0SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM0SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM0SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0SYNCBIT` writer - FTM0 Sync Bit"]
pub struct FTM0SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0SYNCBIT_W<'a> {
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
#[doc = "Field `FTM1SYNCBIT` reader - FTM1 Sync Bit"]
pub struct FTM1SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM1SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM1SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM1SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1SYNCBIT` writer - FTM1 Sync Bit"]
pub struct FTM1SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1SYNCBIT_W<'a> {
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
#[doc = "Field `FTM2SYNCBIT` reader - FTM2 Sync Bit"]
pub struct FTM2SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM2SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM2SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM2SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2SYNCBIT` writer - FTM2 Sync Bit"]
pub struct FTM2SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2SYNCBIT_W<'a> {
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
#[doc = "Field `FTM3SYNCBIT` reader - FTM3 Sync Bit"]
pub struct FTM3SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM3SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM3SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM3SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM3SYNCBIT` writer - FTM3 Sync Bit"]
pub struct FTM3SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3SYNCBIT_W<'a> {
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
#[doc = "FTM1 CH0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1CH0SEL_A {
    #[doc = "0: FTM1_CH0 input"]
    _00 = 0,
    #[doc = "1: CMP0 output"]
    _01 = 1,
}
impl From<FTM1CH0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CH0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM1CH0SEL` reader - FTM1 CH0 Select"]
pub struct FTM1CH0SEL_R(crate::FieldReader<u8, FTM1CH0SEL_A>);
impl FTM1CH0SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM1CH0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM1CH0SEL_A> {
        match self.bits {
            0 => Some(FTM1CH0SEL_A::_00),
            1 => Some(FTM1CH0SEL_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM1CH0SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM1CH0SEL_A::_01
    }
}
impl core::ops::Deref for FTM1CH0SEL_R {
    type Target = crate::FieldReader<u8, FTM1CH0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1CH0SEL` writer - FTM1 CH0 Select"]
pub struct FTM1CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CH0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CH0SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM1_CH0 input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CH0SEL_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CH0SEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "FTM2 CH0 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2CH0SEL_A {
    #[doc = "0: FTM2_CH0 input"]
    _00 = 0,
    #[doc = "1: CMP0 output"]
    _01 = 1,
}
impl From<FTM2CH0SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CH0SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM2CH0SEL` reader - FTM2 CH0 Select"]
pub struct FTM2CH0SEL_R(crate::FieldReader<u8, FTM2CH0SEL_A>);
impl FTM2CH0SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM2CH0SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM2CH0SEL_A> {
        match self.bits {
            0 => Some(FTM2CH0SEL_A::_00),
            1 => Some(FTM2CH0SEL_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM2CH0SEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM2CH0SEL_A::_01
    }
}
impl core::ops::Deref for FTM2CH0SEL_R {
    type Target = crate::FieldReader<u8, FTM2CH0SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2CH0SEL` writer - FTM2 CH0 Select"]
pub struct FTM2CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CH0SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CH0SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2_CH0 input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CH0SEL_A::_00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CH0SEL_A::_01)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "FTM2 CH1 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2CH1SEL_A {
    #[doc = "0: FTM2_CH1 input"]
    _0 = 0,
    #[doc = "1: exclusive OR of FTM2_CH0,FTM2_CH1,and FTM1_CH1"]
    _1 = 1,
}
impl From<FTM2CH1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2CH1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTM2CH1SEL` reader - FTM2 CH1 Select"]
pub struct FTM2CH1SEL_R(crate::FieldReader<bool, FTM2CH1SEL_A>);
impl FTM2CH1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM2CH1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CH1SEL_A {
        match self.bits {
            false => FTM2CH1SEL_A::_0,
            true => FTM2CH1SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTM2CH1SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTM2CH1SEL_A::_1
    }
}
impl core::ops::Deref for FTM2CH1SEL_R {
    type Target = crate::FieldReader<bool, FTM2CH1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2CH1SEL` writer - FTM2 CH1 Select"]
pub struct FTM2CH1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CH1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CH1SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM2_CH1 input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2CH1SEL_A::_0)
    }
    #[doc = "exclusive OR of FTM2_CH0,FTM2_CH1,and FTM1_CH1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2CH1SEL_A::_1)
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
#[doc = "Field `FTM4SYNCBIT` reader - FTM4 Sync Bit"]
pub struct FTM4SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM4SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM4SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM4SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM4SYNCBIT` writer - FTM4 Sync Bit"]
pub struct FTM4SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM4SYNCBIT_W<'a> {
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
#[doc = "Field `FTM5SYNCBIT` reader - FTM5 Sync Bit"]
pub struct FTM5SYNCBIT_R(crate::FieldReader<bool, bool>);
impl FTM5SYNCBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTM5SYNCBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTM5SYNCBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM5SYNCBIT` writer - FTM5 Sync Bit"]
pub struct FTM5SYNCBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM5SYNCBIT_W<'a> {
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
#[doc = "FTM global load enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTMGLDOK_A {
    #[doc = "0: FTM Global load mechanism disabled."]
    _0 = 0,
    #[doc = "1: FTM Global load mechanism enabled"]
    _1 = 1,
}
impl From<FTMGLDOK_A> for bool {
    #[inline(always)]
    fn from(variant: FTMGLDOK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTMGLDOK` reader - FTM global load enable"]
pub struct FTMGLDOK_R(crate::FieldReader<bool, FTMGLDOK_A>);
impl FTMGLDOK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FTMGLDOK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTMGLDOK_A {
        match self.bits {
            false => FTMGLDOK_A::_0,
            true => FTMGLDOK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FTMGLDOK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FTMGLDOK_A::_1
    }
}
impl core::ops::Deref for FTMGLDOK_R {
    type Target = crate::FieldReader<bool, FTMGLDOK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTMGLDOK` writer - FTM global load enable"]
pub struct FTMGLDOK_W<'a> {
    w: &'a mut W,
}
impl<'a> FTMGLDOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTMGLDOK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FTM Global load mechanism disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMGLDOK_A::_0)
    }
    #[doc = "FTM Global load mechanism enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMGLDOK_A::_1)
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
#[doc = "FTM0 channel modulation select with FTM1_CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0_OUTSEL_A {
    #[doc = "0: No modulation with FTM1_CH1"]
    _00000000 = 0,
    #[doc = "1: Modulation with FTM1_CH1"]
    _00000001 = 1,
}
impl From<FTM0_OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0_OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM0_OUTSEL` reader - FTM0 channel modulation select with FTM1_CH1"]
pub struct FTM0_OUTSEL_R(crate::FieldReader<u8, FTM0_OUTSEL_A>);
impl FTM0_OUTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM0_OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM0_OUTSEL_A> {
        match self.bits {
            0 => Some(FTM0_OUTSEL_A::_00000000),
            1 => Some(FTM0_OUTSEL_A::_00000001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline(always)]
    pub fn is_00000000(&self) -> bool {
        **self == FTM0_OUTSEL_A::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline(always)]
    pub fn is_00000001(&self) -> bool {
        **self == FTM0_OUTSEL_A::_00000001
    }
}
impl core::ops::Deref for FTM0_OUTSEL_R {
    type Target = crate::FieldReader<u8, FTM0_OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0_OUTSEL` writer - FTM0 channel modulation select with FTM1_CH1"]
pub struct FTM0_OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0_OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0_OUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No modulation with FTM1_CH1"]
    #[inline(always)]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(FTM0_OUTSEL_A::_00000000)
    }
    #[doc = "Modulation with FTM1_CH1"]
    #[inline(always)]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(FTM0_OUTSEL_A::_00000001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "FTM3 channel modulation select with FTM2_CH1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM3_OUTSEL_A {
    #[doc = "0: No modulation with FTM2_CH1"]
    _00000000 = 0,
    #[doc = "1: Modulation with FTM2_CH1"]
    _00000001 = 1,
}
impl From<FTM3_OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM3_OUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM3_OUTSEL` reader - FTM3 channel modulation select with FTM2_CH1"]
pub struct FTM3_OUTSEL_R(crate::FieldReader<u8, FTM3_OUTSEL_A>);
impl FTM3_OUTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM3_OUTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM3_OUTSEL_A> {
        match self.bits {
            0 => Some(FTM3_OUTSEL_A::_00000000),
            1 => Some(FTM3_OUTSEL_A::_00000001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline(always)]
    pub fn is_00000000(&self) -> bool {
        **self == FTM3_OUTSEL_A::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline(always)]
    pub fn is_00000001(&self) -> bool {
        **self == FTM3_OUTSEL_A::_00000001
    }
}
impl core::ops::Deref for FTM3_OUTSEL_R {
    type Target = crate::FieldReader<u8, FTM3_OUTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM3_OUTSEL` writer - FTM3 channel modulation select with FTM2_CH1"]
pub struct FTM3_OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3_OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3_OUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No modulation with FTM2_CH1"]
    #[inline(always)]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(FTM3_OUTSEL_A::_00000000)
    }
    #[doc = "Modulation with FTM2_CH1"]
    #[inline(always)]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(FTM3_OUTSEL_A::_00000001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FTM0 Sync Bit"]
    #[inline(always)]
    pub fn ftm0syncbit(&self) -> FTM0SYNCBIT_R {
        FTM0SYNCBIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FTM1 Sync Bit"]
    #[inline(always)]
    pub fn ftm1syncbit(&self) -> FTM1SYNCBIT_R {
        FTM1SYNCBIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FTM2 Sync Bit"]
    #[inline(always)]
    pub fn ftm2syncbit(&self) -> FTM2SYNCBIT_R {
        FTM2SYNCBIT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FTM3 Sync Bit"]
    #[inline(always)]
    pub fn ftm3syncbit(&self) -> FTM3SYNCBIT_R {
        FTM3SYNCBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - FTM1 CH0 Select"]
    #[inline(always)]
    pub fn ftm1ch0sel(&self) -> FTM1CH0SEL_R {
        FTM1CH0SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - FTM2 CH0 Select"]
    #[inline(always)]
    pub fn ftm2ch0sel(&self) -> FTM2CH0SEL_R {
        FTM2CH0SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - FTM2 CH1 Select"]
    #[inline(always)]
    pub fn ftm2ch1sel(&self) -> FTM2CH1SEL_R {
        FTM2CH1SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FTM4 Sync Bit"]
    #[inline(always)]
    pub fn ftm4syncbit(&self) -> FTM4SYNCBIT_R {
        FTM4SYNCBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FTM5 Sync Bit"]
    #[inline(always)]
    pub fn ftm5syncbit(&self) -> FTM5SYNCBIT_R {
        FTM5SYNCBIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FTM global load enable"]
    #[inline(always)]
    pub fn ftmgldok(&self) -> FTMGLDOK_R {
        FTMGLDOK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - FTM0 channel modulation select with FTM1_CH1"]
    #[inline(always)]
    pub fn ftm0_outsel(&self) -> FTM0_OUTSEL_R {
        FTM0_OUTSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FTM3 channel modulation select with FTM2_CH1"]
    #[inline(always)]
    pub fn ftm3_outsel(&self) -> FTM3_OUTSEL_R {
        FTM3_OUTSEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FTM0 Sync Bit"]
    #[inline(always)]
    pub fn ftm0syncbit(&mut self) -> FTM0SYNCBIT_W {
        FTM0SYNCBIT_W { w: self }
    }
    #[doc = "Bit 1 - FTM1 Sync Bit"]
    #[inline(always)]
    pub fn ftm1syncbit(&mut self) -> FTM1SYNCBIT_W {
        FTM1SYNCBIT_W { w: self }
    }
    #[doc = "Bit 2 - FTM2 Sync Bit"]
    #[inline(always)]
    pub fn ftm2syncbit(&mut self) -> FTM2SYNCBIT_W {
        FTM2SYNCBIT_W { w: self }
    }
    #[doc = "Bit 3 - FTM3 Sync Bit"]
    #[inline(always)]
    pub fn ftm3syncbit(&mut self) -> FTM3SYNCBIT_W {
        FTM3SYNCBIT_W { w: self }
    }
    #[doc = "Bits 4:5 - FTM1 CH0 Select"]
    #[inline(always)]
    pub fn ftm1ch0sel(&mut self) -> FTM1CH0SEL_W {
        FTM1CH0SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - FTM2 CH0 Select"]
    #[inline(always)]
    pub fn ftm2ch0sel(&mut self) -> FTM2CH0SEL_W {
        FTM2CH0SEL_W { w: self }
    }
    #[doc = "Bit 8 - FTM2 CH1 Select"]
    #[inline(always)]
    pub fn ftm2ch1sel(&mut self) -> FTM2CH1SEL_W {
        FTM2CH1SEL_W { w: self }
    }
    #[doc = "Bit 11 - FTM4 Sync Bit"]
    #[inline(always)]
    pub fn ftm4syncbit(&mut self) -> FTM4SYNCBIT_W {
        FTM4SYNCBIT_W { w: self }
    }
    #[doc = "Bit 12 - FTM5 Sync Bit"]
    #[inline(always)]
    pub fn ftm5syncbit(&mut self) -> FTM5SYNCBIT_W {
        FTM5SYNCBIT_W { w: self }
    }
    #[doc = "Bit 15 - FTM global load enable"]
    #[inline(always)]
    pub fn ftmgldok(&mut self) -> FTMGLDOK_W {
        FTMGLDOK_W { w: self }
    }
    #[doc = "Bits 16:23 - FTM0 channel modulation select with FTM1_CH1"]
    #[inline(always)]
    pub fn ftm0_outsel(&mut self) -> FTM0_OUTSEL_W {
        FTM0_OUTSEL_W { w: self }
    }
    #[doc = "Bits 24:31 - FTM3 channel modulation select with FTM2_CH1"]
    #[inline(always)]
    pub fn ftm3_outsel(&mut self) -> FTM3_OUTSEL_W {
        FTM3_OUTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Option Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftmopt1](index.html) module"]
pub struct FTMOPT1_SPEC;
impl crate::RegisterSpec for FTMOPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftmopt1::R](R) reader structure"]
impl crate::Readable for FTMOPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftmopt1::W](W) writer structure"]
impl crate::Writable for FTMOPT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTMOPT1 to value 0"]
impl crate::Resettable for FTMOPT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
