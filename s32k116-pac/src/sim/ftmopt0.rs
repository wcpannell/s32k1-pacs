#[doc = "Register `FTMOPT0` reader"]
pub struct R(crate::R<FTMOPT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTMOPT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTMOPT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTMOPT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTMOPT0` writer"]
pub struct W(crate::W<FTMOPT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTMOPT0_SPEC>;
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
impl From<crate::W<FTMOPT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTMOPT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FTM0 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0FLTXSEL_A {
    #[doc = "0: FTM0_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM0 out"]
    _001 = 1,
}
impl From<FTM0FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM0FLTxSEL` reader - FTM0 Fault X Select"]
pub struct FTM0FLTXSEL_R(crate::FieldReader<u8, FTM0FLTXSEL_A>);
impl FTM0FLTXSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM0FLTXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM0FLTXSEL_A> {
        match self.bits {
            0 => Some(FTM0FLTXSEL_A::_000),
            1 => Some(FTM0FLTXSEL_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FTM0FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FTM0FLTXSEL_A::_001
    }
}
impl core::ops::Deref for FTM0FLTXSEL_R {
    type Target = crate::FieldReader<u8, FTM0FLTXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0FLTxSEL` writer - FTM0 Fault X Select"]
pub struct FTM0FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM0_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM0FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM0 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM0FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "FTM1 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1FLTXSEL_A {
    #[doc = "0: FTM1_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM1 out"]
    _001 = 1,
}
impl From<FTM1FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM1FLTxSEL` reader - FTM1 Fault X Select"]
pub struct FTM1FLTXSEL_R(crate::FieldReader<u8, FTM1FLTXSEL_A>);
impl FTM1FLTXSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM1FLTXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM1FLTXSEL_A> {
        match self.bits {
            0 => Some(FTM1FLTXSEL_A::_000),
            1 => Some(FTM1FLTXSEL_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FTM1FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FTM1FLTXSEL_A::_001
    }
}
impl core::ops::Deref for FTM1FLTXSEL_R {
    type Target = crate::FieldReader<u8, FTM1FLTXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1FLTxSEL` writer - FTM1 Fault X Select"]
pub struct FTM1FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM1_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM1FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM1 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM1FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "FTM2 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2FLTXSEL_A {
    #[doc = "0: FTM2_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM2 out"]
    _001 = 1,
}
impl From<FTM2FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM2FLTxSEL` reader - FTM2 Fault X Select"]
pub struct FTM2FLTXSEL_R(crate::FieldReader<u8, FTM2FLTXSEL_A>);
impl FTM2FLTXSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM2FLTXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM2FLTXSEL_A> {
        match self.bits {
            0 => Some(FTM2FLTXSEL_A::_000),
            1 => Some(FTM2FLTXSEL_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FTM2FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FTM2FLTXSEL_A::_001
    }
}
impl core::ops::Deref for FTM2FLTXSEL_R {
    type Target = crate::FieldReader<u8, FTM2FLTXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2FLTxSEL` writer - FTM2 Fault X Select"]
pub struct FTM2FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM2_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM2FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM2 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM2FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "FTM3 Fault X Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM3FLTXSEL_A {
    #[doc = "0: FTM3_FLTx pin"]
    _000 = 0,
    #[doc = "1: TRGMUX_FTM3 out"]
    _001 = 1,
}
impl From<FTM3FLTXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM3FLTXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM3FLTxSEL` reader - FTM3 Fault X Select"]
pub struct FTM3FLTXSEL_R(crate::FieldReader<u8, FTM3FLTXSEL_A>);
impl FTM3FLTXSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM3FLTXSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FTM3FLTXSEL_A> {
        match self.bits {
            0 => Some(FTM3FLTXSEL_A::_000),
            1 => Some(FTM3FLTXSEL_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == FTM3FLTXSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        **self == FTM3FLTXSEL_A::_001
    }
}
impl core::ops::Deref for FTM3FLTXSEL_R {
    type Target = crate::FieldReader<u8, FTM3FLTXSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM3FLTxSEL` writer - FTM3 Fault X Select"]
pub struct FTM3FLTXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3FLTXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3FLTXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FTM3_FLTx pin"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FTM3FLTXSEL_A::_000)
    }
    #[doc = "TRGMUX_FTM3 out"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FTM3FLTXSEL_A::_001)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "FTM0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM0CLKSEL_A {
    #[doc = "0: FTM0 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM0 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM0 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM0CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM0CLKSEL` reader - FTM0 External Clock Pin Select"]
pub struct FTM0CLKSEL_R(crate::FieldReader<u8, FTM0CLKSEL_A>);
impl FTM0CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM0CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0CLKSEL_A {
        match self.bits {
            0 => FTM0CLKSEL_A::_00,
            1 => FTM0CLKSEL_A::_01,
            2 => FTM0CLKSEL_A::_10,
            3 => FTM0CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM0CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM0CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FTM0CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FTM0CLKSEL_A::_11
    }
}
impl core::ops::Deref for FTM0CLKSEL_R {
    type Target = crate::FieldReader<u8, FTM0CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM0CLKSEL` writer - FTM0 External Clock Pin Select"]
pub struct FTM0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM0CLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FTM0 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_00)
    }
    #[doc = "FTM0 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_01)
    }
    #[doc = "FTM0 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM0CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "FTM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM1CLKSEL_A {
    #[doc = "0: FTM1 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM1 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM1 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM1CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM1CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM1CLKSEL` reader - FTM1 External Clock Pin Select"]
pub struct FTM1CLKSEL_R(crate::FieldReader<u8, FTM1CLKSEL_A>);
impl FTM1CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM1CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1CLKSEL_A {
        match self.bits {
            0 => FTM1CLKSEL_A::_00,
            1 => FTM1CLKSEL_A::_01,
            2 => FTM1CLKSEL_A::_10,
            3 => FTM1CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM1CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM1CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FTM1CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FTM1CLKSEL_A::_11
    }
}
impl core::ops::Deref for FTM1CLKSEL_R {
    type Target = crate::FieldReader<u8, FTM1CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM1CLKSEL` writer - FTM1 External Clock Pin Select"]
pub struct FTM1CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM1CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM1CLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FTM1 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_00)
    }
    #[doc = "FTM1 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_01)
    }
    #[doc = "FTM1 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM1CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "FTM2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM2CLKSEL_A {
    #[doc = "0: FTM2 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM2 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM2 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM2CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM2CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM2CLKSEL` reader - FTM2 External Clock Pin Select"]
pub struct FTM2CLKSEL_R(crate::FieldReader<u8, FTM2CLKSEL_A>);
impl FTM2CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM2CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2CLKSEL_A {
        match self.bits {
            0 => FTM2CLKSEL_A::_00,
            1 => FTM2CLKSEL_A::_01,
            2 => FTM2CLKSEL_A::_10,
            3 => FTM2CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM2CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM2CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FTM2CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FTM2CLKSEL_A::_11
    }
}
impl core::ops::Deref for FTM2CLKSEL_R {
    type Target = crate::FieldReader<u8, FTM2CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM2CLKSEL` writer - FTM2 External Clock Pin Select"]
pub struct FTM2CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2CLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FTM2 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_00)
    }
    #[doc = "FTM2 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_01)
    }
    #[doc = "FTM2 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM2CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "FTM3 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTM3CLKSEL_A {
    #[doc = "0: FTM3 external clock driven by TCLK0 pin."]
    _00 = 0,
    #[doc = "1: FTM3 external clock driven by TCLK1 pin."]
    _01 = 1,
    #[doc = "2: FTM3 external clock driven by TCLK2 pin."]
    _10 = 2,
    #[doc = "3: No clock input"]
    _11 = 3,
}
impl From<FTM3CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FTM3CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FTM3CLKSEL` reader - FTM3 External Clock Pin Select"]
pub struct FTM3CLKSEL_R(crate::FieldReader<u8, FTM3CLKSEL_A>);
impl FTM3CLKSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTM3CLKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3CLKSEL_A {
        match self.bits {
            0 => FTM3CLKSEL_A::_00,
            1 => FTM3CLKSEL_A::_01,
            2 => FTM3CLKSEL_A::_10,
            3 => FTM3CLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FTM3CLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FTM3CLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FTM3CLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FTM3CLKSEL_A::_11
    }
}
impl core::ops::Deref for FTM3CLKSEL_R {
    type Target = crate::FieldReader<u8, FTM3CLKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTM3CLKSEL` writer - FTM3 External Clock Pin Select"]
pub struct FTM3CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM3CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM3CLKSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FTM3 external clock driven by TCLK0 pin."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_00)
    }
    #[doc = "FTM3 external clock driven by TCLK1 pin."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_01)
    }
    #[doc = "FTM3 external clock driven by TCLK2 pin."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_10)
    }
    #[doc = "No clock input"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FTM3CLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline(always)]
    pub fn ftm0fltx_sel(&self) -> FTM0FLTXSEL_R {
        FTM0FLTXSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline(always)]
    pub fn ftm1fltx_sel(&self) -> FTM1FLTXSEL_R {
        FTM1FLTXSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline(always)]
    pub fn ftm2fltx_sel(&self) -> FTM2FLTXSEL_R {
        FTM2FLTXSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline(always)]
    pub fn ftm3fltx_sel(&self) -> FTM3FLTXSEL_R {
        FTM3FLTXSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&self) -> FTM0CLKSEL_R {
        FTM0CLKSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&self) -> FTM1CLKSEL_R {
        FTM1CLKSEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&self) -> FTM2CLKSEL_R {
        FTM2CLKSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&self) -> FTM3CLKSEL_R {
        FTM3CLKSEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FTM0 Fault X Select"]
    #[inline(always)]
    pub fn ftm0fltx_sel(&mut self) -> FTM0FLTXSEL_W {
        FTM0FLTXSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - FTM1 Fault X Select"]
    #[inline(always)]
    pub fn ftm1fltx_sel(&mut self) -> FTM1FLTXSEL_W {
        FTM1FLTXSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - FTM2 Fault X Select"]
    #[inline(always)]
    pub fn ftm2fltx_sel(&mut self) -> FTM2FLTXSEL_W {
        FTM2FLTXSEL_W { w: self }
    }
    #[doc = "Bits 12:14 - FTM3 Fault X Select"]
    #[inline(always)]
    pub fn ftm3fltx_sel(&mut self) -> FTM3FLTXSEL_W {
        FTM3FLTXSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - FTM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm0clksel(&mut self) -> FTM0CLKSEL_W {
        FTM0CLKSEL_W { w: self }
    }
    #[doc = "Bits 26:27 - FTM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm1clksel(&mut self) -> FTM1CLKSEL_W {
        FTM1CLKSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - FTM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm2clksel(&mut self) -> FTM2CLKSEL_W {
        FTM2CLKSEL_W { w: self }
    }
    #[doc = "Bits 30:31 - FTM3 External Clock Pin Select"]
    #[inline(always)]
    pub fn ftm3clksel(&mut self) -> FTM3CLKSEL_W {
        FTM3CLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FTM Option Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftmopt0](index.html) module"]
pub struct FTMOPT0_SPEC;
impl crate::RegisterSpec for FTMOPT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftmopt0::R](R) reader structure"]
impl crate::Readable for FTMOPT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftmopt0::W](W) writer structure"]
impl crate::Writable for FTMOPT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTMOPT0 to value 0"]
impl crate::Resettable for FTMOPT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
