#[doc = "Register `SAI_TCR4` reader"]
pub struct R(crate::R<SAI_TCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_TCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_TCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_TCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_TCR4` writer"]
pub struct W(crate::W<SAI_TCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_TCR4_SPEC>;
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
impl From<crate::W<SAI_TCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_TCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSD_A {
    #[doc = "0: Frame sync is generated externally in Slave mode."]
    _0 = 0,
    #[doc = "1: Frame sync is generated internally in Master mode."]
    _1 = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSD` reader - Frame Sync Direction"]
pub struct FSD_R(crate::FieldReader<bool, FSD_A>);
impl FSD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::_0,
            true => FSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSD_A::_1
    }
}
impl core::ops::Deref for FSD_R {
    type Target = crate::FieldReader<bool, FSD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSD` writer - Frame Sync Direction"]
pub struct FSD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frame sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSD_A::_0)
    }
    #[doc = "Frame sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSD_A::_1)
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
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    _0 = 0,
    #[doc = "1: Frame sync is active low."]
    _1 = 1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSP` reader - Frame Sync Polarity"]
pub struct FSP_R(crate::FieldReader<bool, FSP_A>);
impl FSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::_0,
            true => FSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSP_A::_1
    }
}
impl core::ops::Deref for FSP_R {
    type Target = crate::FieldReader<bool, FSP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSP` writer - Frame Sync Polarity"]
pub struct FSP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSP_A::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSP_A::_1)
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
#[doc = "On Demand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONDEM_A {
    #[doc = "0: Internal frame sync is generated continuously."]
    _0 = 0,
    #[doc = "1: Internal frame sync is generated when the FIFO warning flag is clear."]
    _1 = 1,
}
impl From<ONDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ONDEM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONDEM` reader - On Demand Mode"]
pub struct ONDEM_R(crate::FieldReader<bool, ONDEM_A>);
impl ONDEM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONDEM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONDEM_A {
        match self.bits {
            false => ONDEM_A::_0,
            true => ONDEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ONDEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ONDEM_A::_1
    }
}
impl core::ops::Deref for ONDEM_R {
    type Target = crate::FieldReader<bool, ONDEM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONDEM` writer - On Demand Mode"]
pub struct ONDEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONDEM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONDEM_A::_0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONDEM_A::_1)
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
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    _0 = 0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    _1 = 1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSE` reader - Frame Sync Early"]
pub struct FSE_R(crate::FieldReader<bool, FSE_A>);
impl FSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::_0,
            true => FSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FSE_A::_1
    }
}
impl core::ops::Deref for FSE_R {
    type Target = crate::FieldReader<bool, FSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSE` writer - Frame Sync Early"]
pub struct FSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSE_A::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSE_A::_1)
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
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MF_A {
    #[doc = "0: LSB is transmitted first."]
    _0 = 0,
    #[doc = "1: MSB is transmitted first."]
    _1 = 1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MF` reader - MSB First"]
pub struct MF_R(crate::FieldReader<bool, MF_A>);
impl MF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::_0,
            true => MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MF_A::_1
    }
}
impl core::ops::Deref for MF_R {
    type Target = crate::FieldReader<bool, MF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MF` writer - MSB First"]
pub struct MF_W<'a> {
    w: &'a mut W,
}
impl<'a> MF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MF_A::_0)
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MF_A::_1)
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
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMOD_A {
    #[doc = "0: TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    _0 = 0,
    #[doc = "1: Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    _1 = 1,
}
impl From<CHMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHMOD` reader - Channel Mode"]
pub struct CHMOD_R(crate::FieldReader<bool, CHMOD_A>);
impl CHMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMOD_A {
        match self.bits {
            false => CHMOD_A::_0,
            true => CHMOD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CHMOD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CHMOD_A::_1
    }
}
impl core::ops::Deref for CHMOD_R {
    type Target = crate::FieldReader<bool, CHMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHMOD` writer - Channel Mode"]
pub struct CHMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHMOD_A::_0)
    }
    #[doc = "Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHMOD_A::_1)
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
#[doc = "Field `SYWD` reader - Sync Width"]
pub struct SYWD_R(crate::FieldReader<u8, u8>);
impl SYWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYWD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYWD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYWD` writer - Sync Width"]
pub struct SYWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYWD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `FRSZ` reader - Frame size"]
pub struct FRSZ_R(crate::FieldReader<u8, u8>);
impl FRSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRSZ` writer - Frame size"]
pub struct FRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "FIFO Packing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FPACK_A {
    #[doc = "0: FIFO packing is disabled"]
    _00 = 0,
    #[doc = "2: 8-bit FIFO packing is enabled"]
    _10 = 2,
    #[doc = "3: 16-bit FIFO packing is enabled"]
    _11 = 3,
}
impl From<FPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: FPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FPACK` reader - FIFO Packing Mode"]
pub struct FPACK_R(crate::FieldReader<u8, FPACK_A>);
impl FPACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FPACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FPACK_A> {
        match self.bits {
            0 => Some(FPACK_A::_00),
            2 => Some(FPACK_A::_10),
            3 => Some(FPACK_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FPACK_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FPACK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FPACK_A::_11
    }
}
impl core::ops::Deref for FPACK_R {
    type Target = crate::FieldReader<u8, FPACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPACK` writer - FIFO Packing Mode"]
pub struct FPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FPACK_A::_00)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FPACK_A::_10)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FPACK_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "FIFO Combine Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCOMB_A {
    #[doc = "0: FIFO combine mode disabled."]
    _00 = 0,
    #[doc = "1: FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    _01 = 1,
    #[doc = "2: FIFO combine mode enabled on FIFO writes (by software)."]
    _10 = 2,
    #[doc = "3: FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    _11 = 3,
}
impl From<FCOMB_A> for u8 {
    #[inline(always)]
    fn from(variant: FCOMB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCOMB` reader - FIFO Combine Mode"]
pub struct FCOMB_R(crate::FieldReader<u8, FCOMB_A>);
impl FCOMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCOMB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCOMB_A {
        match self.bits {
            0 => FCOMB_A::_00,
            1 => FCOMB_A::_01,
            2 => FCOMB_A::_10,
            3 => FCOMB_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        **self == FCOMB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        **self == FCOMB_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == FCOMB_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == FCOMB_A::_11
    }
}
impl core::ops::Deref for FCOMB_R {
    type Target = crate::FieldReader<u8, FCOMB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCOMB` writer - FIFO Combine Mode"]
pub struct FCOMB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCOMB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCOMB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "FIFO combine mode disabled."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FCOMB_A::_00)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers)."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FCOMB_A::_01)
    }
    #[doc = "FIFO combine mode enabled on FIFO writes (by software)."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FCOMB_A::_10)
    }
    #[doc = "FIFO combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FCOMB_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "FIFO Continue on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCONT_A {
    #[doc = "0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    _0 = 0,
    #[doc = "1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    _1 = 1,
}
impl From<FCONT_A> for bool {
    #[inline(always)]
    fn from(variant: FCONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCONT` reader - FIFO Continue on Error"]
pub struct FCONT_R(crate::FieldReader<bool, FCONT_A>);
impl FCONT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCONT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCONT_A {
        match self.bits {
            false => FCONT_A::_0,
            true => FCONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FCONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FCONT_A::_1
    }
}
impl core::ops::Deref for FCONT_R {
    type Target = crate::FieldReader<bool, FCONT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCONT` writer - FIFO Continue on Error"]
pub struct FCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCONT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCONT_A::_0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCONT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&self) -> ONDEM_R {
        ONDEM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Frame size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&self) -> FPACK_R {
        FPACK_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&self) -> FCOMB_R {
        FCOMB_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&self) -> FCONT_R {
        FCONT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&mut self) -> FSD_W {
        FSD_W { w: self }
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&mut self) -> FSP_W {
        FSP_W { w: self }
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&mut self) -> ONDEM_W {
        ONDEM_W { w: self }
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&mut self) -> FSE_W {
        FSE_W { w: self }
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&mut self) -> MF_W {
        MF_W { w: self }
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    pub fn chmod(&mut self) -> CHMOD_W {
        CHMOD_W { w: self }
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&mut self) -> SYWD_W {
        SYWD_W { w: self }
    }
    #[doc = "Bits 16:19 - Frame size"]
    #[inline(always)]
    pub fn frsz(&mut self) -> FRSZ_W {
        FRSZ_W { w: self }
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&mut self) -> FPACK_W {
        FPACK_W { w: self }
    }
    #[doc = "Bits 26:27 - FIFO Combine Mode"]
    #[inline(always)]
    pub fn fcomb(&mut self) -> FCOMB_W {
        FCOMB_W { w: self }
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&mut self) -> FCONT_W {
        FCONT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Configuration 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_tcr4](index.html) module"]
pub struct SAI_TCR4_SPEC;
impl crate::RegisterSpec for SAI_TCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_tcr4::R](R) reader structure"]
impl crate::Readable for SAI_TCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_tcr4::W](W) writer structure"]
impl crate::Writable for SAI_TCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_TCR4 to value 0"]
impl crate::Resettable for SAI_TCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
