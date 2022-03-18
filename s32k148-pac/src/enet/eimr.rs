#[doc = "Register `EIMR` reader"]
pub struct R(crate::R<EIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMR` writer"]
pub struct W(crate::W<EIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMR_SPEC>;
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
impl From<crate::W<EIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_TIMER` reader - TS_TIMER Interrupt Mask"]
pub struct TS_TIMER_R(crate::FieldReader<bool, bool>);
impl TS_TIMER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TS_TIMER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_TIMER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_TIMER` writer - TS_TIMER Interrupt Mask"]
pub struct TS_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_TIMER_W<'a> {
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
#[doc = "Field `TS_AVAIL` reader - TS_AVAIL Interrupt Mask"]
pub struct TS_AVAIL_R(crate::FieldReader<bool, bool>);
impl TS_AVAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TS_AVAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_AVAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_AVAIL` writer - TS_AVAIL Interrupt Mask"]
pub struct TS_AVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_AVAIL_W<'a> {
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
#[doc = "Field `WAKEUP` reader - WAKEUP Interrupt Mask"]
pub struct WAKEUP_R(crate::FieldReader<bool, bool>);
impl WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUP` writer - WAKEUP Interrupt Mask"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
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
#[doc = "Field `PLR` reader - PLR Interrupt Mask"]
pub struct PLR_R(crate::FieldReader<bool, bool>);
impl PLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLR` writer - PLR Interrupt Mask"]
pub struct PLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLR_W<'a> {
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
#[doc = "Field `UN` reader - UN Interrupt Mask"]
pub struct UN_R(crate::FieldReader<bool, bool>);
impl UN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UN` writer - UN Interrupt Mask"]
pub struct UN_W<'a> {
    w: &'a mut W,
}
impl<'a> UN_W<'a> {
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
#[doc = "Field `RL` reader - RL Interrupt Mask"]
pub struct RL_R(crate::FieldReader<bool, bool>);
impl RL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RL` writer - RL Interrupt Mask"]
pub struct RL_W<'a> {
    w: &'a mut W,
}
impl<'a> RL_W<'a> {
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
#[doc = "Field `LC` reader - LC Interrupt Mask"]
pub struct LC_R(crate::FieldReader<bool, bool>);
impl LC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LC` writer - LC Interrupt Mask"]
pub struct LC_W<'a> {
    w: &'a mut W,
}
impl<'a> LC_W<'a> {
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
#[doc = "Field `EBERR` reader - EBERR Interrupt Mask"]
pub struct EBERR_R(crate::FieldReader<bool, bool>);
impl EBERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EBERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EBERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EBERR` writer - EBERR Interrupt Mask"]
pub struct EBERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EBERR_W<'a> {
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
#[doc = "Field `MII` reader - MII Interrupt Mask"]
pub struct MII_R(crate::FieldReader<bool, bool>);
impl MII_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MII_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MII_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MII` writer - MII Interrupt Mask"]
pub struct MII_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_W<'a> {
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
#[doc = "Field `RXB` reader - RXB Interrupt Mask"]
pub struct RXB_R(crate::FieldReader<bool, bool>);
impl RXB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXB` writer - RXB Interrupt Mask"]
pub struct RXB_W<'a> {
    w: &'a mut W,
}
impl<'a> RXB_W<'a> {
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
#[doc = "Field `RXF` reader - RXF Interrupt Mask"]
pub struct RXF_R(crate::FieldReader<bool, bool>);
impl RXF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXF` writer - RXF Interrupt Mask"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
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
#[doc = "TXB Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXB_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<TXB_A> for bool {
    #[inline(always)]
    fn from(variant: TXB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXB` reader - TXB Interrupt Mask"]
pub struct TXB_R(crate::FieldReader<bool, TXB_A>);
impl TXB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXB_A {
        match self.bits {
            false => TXB_A::_0,
            true => TXB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXB_A::_1
    }
}
impl core::ops::Deref for TXB_R {
    type Target = crate::FieldReader<bool, TXB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXB` writer - TXB Interrupt Mask"]
pub struct TXB_W<'a> {
    w: &'a mut W,
}
impl<'a> TXB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXB_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXB_A::_1)
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
#[doc = "TXF Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXF_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXF` reader - TXF Interrupt Mask"]
pub struct TXF_R(crate::FieldReader<bool, TXF_A>);
impl TXF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::_0,
            true => TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXF_A::_1
    }
}
impl core::ops::Deref for TXF_R {
    type Target = crate::FieldReader<bool, TXF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXF` writer - TXF Interrupt Mask"]
pub struct TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXF_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "GRA Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GRA_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<GRA_A> for bool {
    #[inline(always)]
    fn from(variant: GRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GRA` reader - GRA Interrupt Mask"]
pub struct GRA_R(crate::FieldReader<bool, GRA_A>);
impl GRA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GRA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRA_A {
        match self.bits {
            false => GRA_A::_0,
            true => GRA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == GRA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == GRA_A::_1
    }
}
impl core::ops::Deref for GRA_R {
    type Target = crate::FieldReader<bool, GRA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GRA` writer - GRA Interrupt Mask"]
pub struct GRA_W<'a> {
    w: &'a mut W,
}
impl<'a> GRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GRA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRA_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRA_A::_1)
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
#[doc = "BABT Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABT_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<BABT_A> for bool {
    #[inline(always)]
    fn from(variant: BABT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BABT` reader - BABT Interrupt Mask"]
pub struct BABT_R(crate::FieldReader<bool, BABT_A>);
impl BABT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BABT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABT_A {
        match self.bits {
            false => BABT_A::_0,
            true => BABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BABT_A::_1
    }
}
impl core::ops::Deref for BABT_R {
    type Target = crate::FieldReader<bool, BABT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BABT` writer - BABT Interrupt Mask"]
pub struct BABT_W<'a> {
    w: &'a mut W,
}
impl<'a> BABT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BABT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABT_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "BABR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BABR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<BABR_A> for bool {
    #[inline(always)]
    fn from(variant: BABR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BABR` reader - BABR Interrupt Mask"]
pub struct BABR_R(crate::FieldReader<bool, BABR_A>);
impl BABR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BABR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABR_A {
        match self.bits {
            false => BABR_A::_0,
            true => BABR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BABR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BABR_A::_1
    }
}
impl core::ops::Deref for BABR_R {
    type Target = crate::FieldReader<bool, BABR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BABR` writer - BABR Interrupt Mask"]
pub struct BABR_W<'a> {
    w: &'a mut W,
}
impl<'a> BABR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BABR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABR_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&mut self) -> TS_TIMER_W {
        TS_TIMER_W { w: self }
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W {
        TS_AVAIL_W { w: self }
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&mut self) -> PLR_W {
        PLR_W { w: self }
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&mut self) -> UN_W {
        UN_W { w: self }
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W { w: self }
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W {
        LC_W { w: self }
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&mut self) -> EBERR_W {
        EBERR_W { w: self }
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&mut self) -> RXB_W {
        RXB_W { w: self }
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&mut self) -> TXB_W {
        TXB_W { w: self }
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W {
        TXF_W { w: self }
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&mut self) -> GRA_W {
        GRA_W { w: self }
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&mut self) -> BABT_W {
        BABT_W { w: self }
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&mut self) -> BABR_W {
        BABR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimr](index.html) module"]
pub struct EIMR_SPEC;
impl crate::RegisterSpec for EIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eimr::R](R) reader structure"]
impl crate::Readable for EIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimr::W](W) writer structure"]
impl crate::Writable for EIMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EIMR to value 0"]
impl crate::Resettable for EIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
