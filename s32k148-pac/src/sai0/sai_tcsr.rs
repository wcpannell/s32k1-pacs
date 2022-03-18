#[doc = "Register `SAI_TCSR` reader"]
pub struct R(crate::R<SAI_TCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_TCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_TCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_TCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_TCSR` writer"]
pub struct W(crate::W<SAI_TCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_TCSR_SPEC>;
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
impl From<crate::W<SAI_TCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_TCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO Request DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRDE_A {
    #[doc = "0: Disables the DMA request."]
    _0 = 0,
    #[doc = "1: Enables the DMA request."]
    _1 = 1,
}
impl From<FRDE_A> for bool {
    #[inline(always)]
    fn from(variant: FRDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRDE` reader - FIFO Request DMA Enable"]
pub struct FRDE_R(crate::FieldReader<bool, FRDE_A>);
impl FRDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDE_A {
        match self.bits {
            false => FRDE_A::_0,
            true => FRDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRDE_A::_1
    }
}
impl core::ops::Deref for FRDE_R {
    type Target = crate::FieldReader<bool, FRDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRDE` writer - FIFO Request DMA Enable"]
pub struct FRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDE_A::_1)
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
#[doc = "FIFO Warning DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWDE_A {
    #[doc = "0: Disables the DMA request."]
    _0 = 0,
    #[doc = "1: Enables the DMA request."]
    _1 = 1,
}
impl From<FWDE_A> for bool {
    #[inline(always)]
    fn from(variant: FWDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWDE` reader - FIFO Warning DMA Enable"]
pub struct FWDE_R(crate::FieldReader<bool, FWDE_A>);
impl FWDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDE_A {
        match self.bits {
            false => FWDE_A::_0,
            true => FWDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FWDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FWDE_A::_1
    }
}
impl core::ops::Deref for FWDE_R {
    type Target = crate::FieldReader<bool, FWDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWDE` writer - FIFO Warning DMA Enable"]
pub struct FWDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWDE_A::_1)
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
#[doc = "FIFO Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRIE` reader - FIFO Request Interrupt Enable"]
pub struct FRIE_R(crate::FieldReader<bool, FRIE_A>);
impl FRIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIE_A {
        match self.bits {
            false => FRIE_A::_0,
            true => FRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRIE_A::_1
    }
}
impl core::ops::Deref for FRIE_R {
    type Target = crate::FieldReader<bool, FRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRIE` writer - FIFO Request Interrupt Enable"]
pub struct FRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIE_A::_1)
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
#[doc = "FIFO Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FWIE_A> for bool {
    #[inline(always)]
    fn from(variant: FWIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWIE` reader - FIFO Warning Interrupt Enable"]
pub struct FWIE_R(crate::FieldReader<bool, FWIE_A>);
impl FWIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWIE_A {
        match self.bits {
            false => FWIE_A::_0,
            true => FWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FWIE_A::_1
    }
}
impl core::ops::Deref for FWIE_R {
    type Target = crate::FieldReader<bool, FWIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWIE` writer - FIFO Warning Interrupt Enable"]
pub struct FWIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FWIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWIE_A::_1)
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
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub struct FEIE_R(crate::FieldReader<bool, FEIE_A>);
impl FEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEIE_A::_1
    }
}
impl core::ops::Deref for FEIE_R {
    type Target = crate::FieldReader<bool, FEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
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
#[doc = "Sync Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIE_A {
    #[doc = "0: Disables interrupt."]
    _0 = 0,
    #[doc = "1: Enables interrupt."]
    _1 = 1,
}
impl From<SEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIE` reader - Sync Error Interrupt Enable"]
pub struct SEIE_R(crate::FieldReader<bool, SEIE_A>);
impl SEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIE_A {
        match self.bits {
            false => SEIE_A::_0,
            true => SEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SEIE_A::_1
    }
}
impl core::ops::Deref for SEIE_R {
    type Target = crate::FieldReader<bool, SEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEIE` writer - Sync Error Interrupt Enable"]
pub struct SEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIE_A::_1)
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
#[doc = "Word Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSIE_A {
    #[doc = "0: Disables interrupt."]
    _0 = 0,
    #[doc = "1: Enables interrupt."]
    _1 = 1,
}
impl From<WSIE_A> for bool {
    #[inline(always)]
    fn from(variant: WSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSIE` reader - Word Start Interrupt Enable"]
pub struct WSIE_R(crate::FieldReader<bool, WSIE_A>);
impl WSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSIE_A {
        match self.bits {
            false => WSIE_A::_0,
            true => WSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WSIE_A::_1
    }
}
impl core::ops::Deref for WSIE_R {
    type Target = crate::FieldReader<bool, WSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSIE` writer - Word Start Interrupt Enable"]
pub struct WSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "FIFO Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRF_A {
    #[doc = "0: Transmit FIFO watermark has not been reached."]
    _0 = 0,
    #[doc = "1: Transmit FIFO watermark has been reached."]
    _1 = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - FIFO Request Flag"]
pub struct FRF_R(crate::FieldReader<bool, FRF_A>);
impl FRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::_0,
            true => FRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRF_A::_1
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<bool, FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FIFO Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWF_A {
    #[doc = "0: No enabled transmit FIFO is empty."]
    _0 = 0,
    #[doc = "1: Enabled transmit FIFO is empty."]
    _1 = 1,
}
impl From<FWF_A> for bool {
    #[inline(always)]
    fn from(variant: FWF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWF` reader - FIFO Warning Flag"]
pub struct FWF_R(crate::FieldReader<bool, FWF_A>);
impl FWF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FWF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWF_A {
        match self.bits {
            false => FWF_A::_0,
            true => FWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FWF_A::_1
    }
}
impl core::ops::Deref for FWF_R {
    type Target = crate::FieldReader<bool, FWF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEF_A {
    #[doc = "0: Transmit underrun not detected."]
    _0 = 0,
    #[doc = "1: Transmit underrun detected."]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub struct FEF_R(crate::FieldReader<bool, FEF_A>);
impl FEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FEF_A::_1
    }
}
impl core::ops::Deref for FEF_R {
    type Target = crate::FieldReader<bool, FEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEF` writer - FIFO Error Flag"]
pub struct FEF_W<'a> {
    w: &'a mut W,
}
impl<'a> FEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit underrun not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Transmit underrun detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
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
#[doc = "Sync Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEF_A {
    #[doc = "0: Sync error not detected."]
    _0 = 0,
    #[doc = "1: Frame sync error detected."]
    _1 = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEF` reader - Sync Error Flag"]
pub struct SEF_R(crate::FieldReader<bool, SEF_A>);
impl SEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::_0,
            true => SEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SEF_A::_1
    }
}
impl core::ops::Deref for SEF_R {
    type Target = crate::FieldReader<bool, SEF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEF` writer - Sync Error Flag"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEF_A::_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEF_A::_1)
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
#[doc = "Word Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSF_A {
    #[doc = "0: Start of word not detected."]
    _0 = 0,
    #[doc = "1: Start of word detected."]
    _1 = 1,
}
impl From<WSF_A> for bool {
    #[inline(always)]
    fn from(variant: WSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSF` reader - Word Start Flag"]
pub struct WSF_R(crate::FieldReader<bool, WSF_A>);
impl WSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WSF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSF_A {
        match self.bits {
            false => WSF_A::_0,
            true => WSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WSF_A::_1
    }
}
impl core::ops::Deref for WSF_R {
    type Target = crate::FieldReader<bool, WSF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WSF` writer - Word Start Flag"]
pub struct WSF_W<'a> {
    w: &'a mut W,
}
impl<'a> WSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSF_A::_0)
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSF_A::_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Software reset."]
    _1 = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Software Reset"]
pub struct SR_R(crate::FieldReader<bool, SR_A>);
impl SR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::_0,
            true => SR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SR_A::_1
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SR_A::_0)
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: FIFO reset."]
    _1 = 1,
}
impl From<FR_AW> for bool {
    #[inline(always)]
    fn from(variant: FR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FR` writer - FIFO Reset"]
pub struct FR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FR_AW::_0)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FR_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Bit Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCE_A {
    #[doc = "0: Transmit bit clock is disabled."]
    _0 = 0,
    #[doc = "1: Transmit bit clock is enabled."]
    _1 = 1,
}
impl From<BCE_A> for bool {
    #[inline(always)]
    fn from(variant: BCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCE` reader - Bit Clock Enable"]
pub struct BCE_R(crate::FieldReader<bool, BCE_A>);
impl BCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCE_A {
        match self.bits {
            false => BCE_A::_0,
            true => BCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BCE_A::_1
    }
}
impl core::ops::Deref for BCE_R {
    type Target = crate::FieldReader<bool, BCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCE` writer - Bit Clock Enable"]
pub struct BCE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit bit clock is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCE_A::_0)
    }
    #[doc = "Transmit bit clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCE_A::_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGE_A {
    #[doc = "0: Transmitter is disabled in Debug mode, after completing the current frame."]
    _0 = 0,
    #[doc = "1: Transmitter is enabled in Debug mode."]
    _1 = 1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub struct DBGE_R(crate::FieldReader<bool, DBGE_A>);
impl DBGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::_0,
            true => DBGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DBGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DBGE_A::_1
    }
}
impl core::ops::Deref for DBGE_R {
    type Target = crate::FieldReader<bool, DBGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub struct DBGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGE_A::_0)
    }
    #[doc = "Transmitter is enabled in Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGE_A::_1)
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
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Transmitter is disabled."]
    _0 = 0,
    #[doc = "1: Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub struct TE_R(crate::FieldReader<bool, TE_A>);
impl TE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TE_A::_1
    }
}
impl core::ops::Deref for TE_R {
    type Target = crate::FieldReader<bool, TE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmitter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&self) -> FRDE_R {
        FRDE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&self) -> FWDE_R {
        FWDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&self) -> FRIE_R {
        FRIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&self) -> FWIE_R {
        FWIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&self) -> WSIE_R {
        WSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline(always)]
    pub fn fwf(&self) -> FWF_R {
        FWF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&self) -> WSF_R {
        WSF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&self) -> BCE_R {
        BCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&mut self) -> FRDE_W {
        FRDE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&mut self) -> FWDE_W {
        FWDE_W { w: self }
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&mut self) -> FRIE_W {
        FRIE_W { w: self }
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&mut self) -> FWIE_W {
        FWIE_W { w: self }
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&mut self) -> SEIE_W {
        SEIE_W { w: self }
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&mut self) -> WSIE_W {
        WSIE_W { w: self }
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W {
        FEF_W { w: self }
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&mut self) -> WSF_W {
        WSF_W { w: self }
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W {
        FR_W { w: self }
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&mut self) -> BCE_W {
        BCE_W { w: self }
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&mut self) -> DBGE_W {
        DBGE_W { w: self }
    }
    #[doc = "Bit 31 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_tcsr](index.html) module"]
pub struct SAI_TCSR_SPEC;
impl crate::RegisterSpec for SAI_TCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_tcsr::R](R) reader structure"]
impl crate::Readable for SAI_TCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_tcsr::W](W) writer structure"]
impl crate::Writable for SAI_TCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_TCSR to value 0"]
impl crate::Resettable for SAI_TCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
