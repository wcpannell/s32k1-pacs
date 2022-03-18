#[doc = "Register `SIER` reader"]
pub struct R(crate::R<SIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIER` writer"]
pub struct W(crate::W<SIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIER_SPEC>;
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
impl From<crate::W<SIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDIE_A {
    #[doc = "0: Disabled"]
    TDIE_0 = 0,
    #[doc = "1: Enabled"]
    TDIE_1 = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIE` reader - Transmit Data Interrupt Enable"]
pub struct TDIE_R(crate::FieldReader<bool, TDIE_A>);
impl TDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::TDIE_0,
            true => TDIE_A::TDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TDIE_0`"]
    #[inline(always)]
    pub fn is_tdie_0(&self) -> bool {
        **self == TDIE_A::TDIE_0
    }
    #[doc = "Checks if the value of the field is `TDIE_1`"]
    #[inline(always)]
    pub fn is_tdie_1(&self) -> bool {
        **self == TDIE_A::TDIE_1
    }
}
impl core::ops::Deref for TDIE_R {
    type Target = crate::FieldReader<bool, TDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDIE` writer - Transmit Data Interrupt Enable"]
pub struct TDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tdie_0(self) -> &'a mut W {
        self.variant(TDIE_A::TDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tdie_1(self) -> &'a mut W {
        self.variant(TDIE_A::TDIE_1)
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
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIE_A {
    #[doc = "0: Disabled"]
    RDIE_0 = 0,
    #[doc = "1: Enabled"]
    RDIE_1 = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDIE` reader - Receive Data Interrupt Enable"]
pub struct RDIE_R(crate::FieldReader<bool, RDIE_A>);
impl RDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::RDIE_0,
            true => RDIE_A::RDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RDIE_0`"]
    #[inline(always)]
    pub fn is_rdie_0(&self) -> bool {
        **self == RDIE_A::RDIE_0
    }
    #[doc = "Checks if the value of the field is `RDIE_1`"]
    #[inline(always)]
    pub fn is_rdie_1(&self) -> bool {
        **self == RDIE_A::RDIE_1
    }
}
impl core::ops::Deref for RDIE_R {
    type Target = crate::FieldReader<bool, RDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIE` writer - Receive Data Interrupt Enable"]
pub struct RDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn rdie_0(self) -> &'a mut W {
        self.variant(RDIE_A::RDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rdie_1(self) -> &'a mut W {
        self.variant(RDIE_A::RDIE_1)
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
#[doc = "Address Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVIE_A {
    #[doc = "0: Disabled"]
    AVIE_0 = 0,
    #[doc = "1: Enabled"]
    AVIE_1 = 1,
}
impl From<AVIE_A> for bool {
    #[inline(always)]
    fn from(variant: AVIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVIE` reader - Address Valid Interrupt Enable"]
pub struct AVIE_R(crate::FieldReader<bool, AVIE_A>);
impl AVIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AVIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVIE_A {
        match self.bits {
            false => AVIE_A::AVIE_0,
            true => AVIE_A::AVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVIE_0`"]
    #[inline(always)]
    pub fn is_avie_0(&self) -> bool {
        **self == AVIE_A::AVIE_0
    }
    #[doc = "Checks if the value of the field is `AVIE_1`"]
    #[inline(always)]
    pub fn is_avie_1(&self) -> bool {
        **self == AVIE_A::AVIE_1
    }
}
impl core::ops::Deref for AVIE_R {
    type Target = crate::FieldReader<bool, AVIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVIE` writer - Address Valid Interrupt Enable"]
pub struct AVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn avie_0(self) -> &'a mut W {
        self.variant(AVIE_A::AVIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn avie_1(self) -> &'a mut W {
        self.variant(AVIE_A::AVIE_1)
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
#[doc = "Transmit ACK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAIE_A {
    #[doc = "0: Disabled"]
    TAIE_0 = 0,
    #[doc = "1: Enabled"]
    TAIE_1 = 1,
}
impl From<TAIE_A> for bool {
    #[inline(always)]
    fn from(variant: TAIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - Transmit ACK Interrupt Enable"]
pub struct TAIE_R(crate::FieldReader<bool, TAIE_A>);
impl TAIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TAIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAIE_A {
        match self.bits {
            false => TAIE_A::TAIE_0,
            true => TAIE_A::TAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TAIE_0`"]
    #[inline(always)]
    pub fn is_taie_0(&self) -> bool {
        **self == TAIE_A::TAIE_0
    }
    #[doc = "Checks if the value of the field is `TAIE_1`"]
    #[inline(always)]
    pub fn is_taie_1(&self) -> bool {
        **self == TAIE_A::TAIE_1
    }
}
impl core::ops::Deref for TAIE_R {
    type Target = crate::FieldReader<bool, TAIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAIE` writer - Transmit ACK Interrupt Enable"]
pub struct TAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn taie_0(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn taie_1(self) -> &'a mut W {
        self.variant(TAIE_A::TAIE_1)
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
#[doc = "Repeated Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSIE_A {
    #[doc = "0: Disabled"]
    RSIE_0 = 0,
    #[doc = "1: Enabled"]
    RSIE_1 = 1,
}
impl From<RSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSIE` reader - Repeated Start Interrupt Enable"]
pub struct RSIE_R(crate::FieldReader<bool, RSIE_A>);
impl RSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSIE_A {
        match self.bits {
            false => RSIE_A::RSIE_0,
            true => RSIE_A::RSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RSIE_0`"]
    #[inline(always)]
    pub fn is_rsie_0(&self) -> bool {
        **self == RSIE_A::RSIE_0
    }
    #[doc = "Checks if the value of the field is `RSIE_1`"]
    #[inline(always)]
    pub fn is_rsie_1(&self) -> bool {
        **self == RSIE_A::RSIE_1
    }
}
impl core::ops::Deref for RSIE_R {
    type Target = crate::FieldReader<bool, RSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSIE` writer - Repeated Start Interrupt Enable"]
pub struct RSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn rsie_0(self) -> &'a mut W {
        self.variant(RSIE_A::RSIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn rsie_1(self) -> &'a mut W {
        self.variant(RSIE_A::RSIE_1)
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
#[doc = "STOP Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDIE_A {
    #[doc = "0: Disabled"]
    SDIE_0 = 0,
    #[doc = "1: Enabled"]
    SDIE_1 = 1,
}
impl From<SDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIE` reader - STOP Detect Interrupt Enable"]
pub struct SDIE_R(crate::FieldReader<bool, SDIE_A>);
impl SDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIE_A {
        match self.bits {
            false => SDIE_A::SDIE_0,
            true => SDIE_A::SDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SDIE_0`"]
    #[inline(always)]
    pub fn is_sdie_0(&self) -> bool {
        **self == SDIE_A::SDIE_0
    }
    #[doc = "Checks if the value of the field is `SDIE_1`"]
    #[inline(always)]
    pub fn is_sdie_1(&self) -> bool {
        **self == SDIE_A::SDIE_1
    }
}
impl core::ops::Deref for SDIE_R {
    type Target = crate::FieldReader<bool, SDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIE` writer - STOP Detect Interrupt Enable"]
pub struct SDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn sdie_0(self) -> &'a mut W {
        self.variant(SDIE_A::SDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn sdie_1(self) -> &'a mut W {
        self.variant(SDIE_A::SDIE_1)
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
#[doc = "Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEIE_A {
    #[doc = "0: Disabled"]
    BEIE_0 = 0,
    #[doc = "1: Enabled"]
    BEIE_1 = 1,
}
impl From<BEIE_A> for bool {
    #[inline(always)]
    fn from(variant: BEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEIE` reader - Bit Error Interrupt Enable"]
pub struct BEIE_R(crate::FieldReader<bool, BEIE_A>);
impl BEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEIE_A {
        match self.bits {
            false => BEIE_A::BEIE_0,
            true => BEIE_A::BEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BEIE_0`"]
    #[inline(always)]
    pub fn is_beie_0(&self) -> bool {
        **self == BEIE_A::BEIE_0
    }
    #[doc = "Checks if the value of the field is `BEIE_1`"]
    #[inline(always)]
    pub fn is_beie_1(&self) -> bool {
        **self == BEIE_A::BEIE_1
    }
}
impl core::ops::Deref for BEIE_R {
    type Target = crate::FieldReader<bool, BEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIE` writer - Bit Error Interrupt Enable"]
pub struct BEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn beie_0(self) -> &'a mut W {
        self.variant(BEIE_A::BEIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn beie_1(self) -> &'a mut W {
        self.variant(BEIE_A::BEIE_1)
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
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Disabled"]
    FEIE_0 = 0,
    #[doc = "1: Enabled"]
    FEIE_1 = 1,
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
            false => FEIE_A::FEIE_0,
            true => FEIE_A::FEIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FEIE_0`"]
    #[inline(always)]
    pub fn is_feie_0(&self) -> bool {
        **self == FEIE_A::FEIE_0
    }
    #[doc = "Checks if the value of the field is `FEIE_1`"]
    #[inline(always)]
    pub fn is_feie_1(&self) -> bool {
        **self == FEIE_A::FEIE_1
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn feie_1(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_1)
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
#[doc = "Address Match 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0IE_A {
    #[doc = "0: Enabled"]
    AM0IE_0 = 0,
    #[doc = "1: Disabled"]
    AM0IE_1 = 1,
}
impl From<AM0IE_A> for bool {
    #[inline(always)]
    fn from(variant: AM0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AM0IE` reader - Address Match 0 Interrupt Enable"]
pub struct AM0IE_R(crate::FieldReader<bool, AM0IE_A>);
impl AM0IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AM0IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM0IE_A {
        match self.bits {
            false => AM0IE_A::AM0IE_0,
            true => AM0IE_A::AM0IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM0IE_0`"]
    #[inline(always)]
    pub fn is_am0ie_0(&self) -> bool {
        **self == AM0IE_A::AM0IE_0
    }
    #[doc = "Checks if the value of the field is `AM0IE_1`"]
    #[inline(always)]
    pub fn is_am0ie_1(&self) -> bool {
        **self == AM0IE_A::AM0IE_1
    }
}
impl core::ops::Deref for AM0IE_R {
    type Target = crate::FieldReader<bool, AM0IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM0IE` writer - Address Match 0 Interrupt Enable"]
pub struct AM0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AM0IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM0IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn am0ie_0(self) -> &'a mut W {
        self.variant(AM0IE_A::AM0IE_0)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn am0ie_1(self) -> &'a mut W {
        self.variant(AM0IE_A::AM0IE_1)
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
#[doc = "Address Match 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1F_A {
    #[doc = "0: Disabled"]
    AM1F_0 = 0,
    #[doc = "1: Enabled"]
    AM1F_1 = 1,
}
impl From<AM1F_A> for bool {
    #[inline(always)]
    fn from(variant: AM1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AM1F` reader - Address Match 1 Interrupt Enable"]
pub struct AM1F_R(crate::FieldReader<bool, AM1F_A>);
impl AM1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AM1F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM1F_A {
        match self.bits {
            false => AM1F_A::AM1F_0,
            true => AM1F_A::AM1F_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM1F_0`"]
    #[inline(always)]
    pub fn is_am1f_0(&self) -> bool {
        **self == AM1F_A::AM1F_0
    }
    #[doc = "Checks if the value of the field is `AM1F_1`"]
    #[inline(always)]
    pub fn is_am1f_1(&self) -> bool {
        **self == AM1F_A::AM1F_1
    }
}
impl core::ops::Deref for AM1F_R {
    type Target = crate::FieldReader<bool, AM1F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM1F` writer - Address Match 1 Interrupt Enable"]
pub struct AM1F_W<'a> {
    w: &'a mut W,
}
impl<'a> AM1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AM1F_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn am1f_0(self) -> &'a mut W {
        self.variant(AM1F_A::AM1F_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn am1f_1(self) -> &'a mut W {
        self.variant(AM1F_A::AM1F_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "General Call Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCIE_A {
    #[doc = "0: Disabled"]
    GCIE_0 = 0,
    #[doc = "1: Enabled"]
    GCIE_1 = 1,
}
impl From<GCIE_A> for bool {
    #[inline(always)]
    fn from(variant: GCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCIE` reader - General Call Interrupt Enable"]
pub struct GCIE_R(crate::FieldReader<bool, GCIE_A>);
impl GCIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCIE_A {
        match self.bits {
            false => GCIE_A::GCIE_0,
            true => GCIE_A::GCIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCIE_0`"]
    #[inline(always)]
    pub fn is_gcie_0(&self) -> bool {
        **self == GCIE_A::GCIE_0
    }
    #[doc = "Checks if the value of the field is `GCIE_1`"]
    #[inline(always)]
    pub fn is_gcie_1(&self) -> bool {
        **self == GCIE_A::GCIE_1
    }
}
impl core::ops::Deref for GCIE_R {
    type Target = crate::FieldReader<bool, GCIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCIE` writer - General Call Interrupt Enable"]
pub struct GCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> GCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn gcie_0(self) -> &'a mut W {
        self.variant(GCIE_A::GCIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn gcie_1(self) -> &'a mut W {
        self.variant(GCIE_A::GCIE_1)
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
#[doc = "SMBus Alert Response Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARIE_A {
    #[doc = "0: Disabled"]
    SARIE_0 = 0,
    #[doc = "1: Enabled"]
    SARIE_1 = 1,
}
impl From<SARIE_A> for bool {
    #[inline(always)]
    fn from(variant: SARIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SARIE` reader - SMBus Alert Response Interrupt Enable"]
pub struct SARIE_R(crate::FieldReader<bool, SARIE_A>);
impl SARIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SARIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SARIE_A {
        match self.bits {
            false => SARIE_A::SARIE_0,
            true => SARIE_A::SARIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SARIE_0`"]
    #[inline(always)]
    pub fn is_sarie_0(&self) -> bool {
        **self == SARIE_A::SARIE_0
    }
    #[doc = "Checks if the value of the field is `SARIE_1`"]
    #[inline(always)]
    pub fn is_sarie_1(&self) -> bool {
        **self == SARIE_A::SARIE_1
    }
}
impl core::ops::Deref for SARIE_R {
    type Target = crate::FieldReader<bool, SARIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SARIE` writer - SMBus Alert Response Interrupt Enable"]
pub struct SARIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SARIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SARIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn sarie_0(self) -> &'a mut W {
        self.variant(SARIE_A::SARIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn sarie_1(self) -> &'a mut W {
        self.variant(SARIE_A::SARIE_1)
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
impl R {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avie(&self) -> AVIE_R {
        AVIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TAIE_R {
        TAIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsie(&self) -> RSIE_R {
        RSIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&self) -> SDIE_R {
        SDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn am0ie(&self) -> AM0IE_R {
        AM0IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn am1f(&self) -> AM1F_R {
        AM1F_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline(always)]
    pub fn gcie(&self) -> GCIE_R {
        GCIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub fn sarie(&self) -> SARIE_R {
        SARIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&mut self) -> TDIE_W {
        TDIE_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&mut self) -> RDIE_W {
        RDIE_W { w: self }
    }
    #[doc = "Bit 2 - Address Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avie(&mut self) -> AVIE_W {
        AVIE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit ACK Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&mut self) -> TAIE_W {
        TAIE_W { w: self }
    }
    #[doc = "Bit 8 - Repeated Start Interrupt Enable"]
    #[inline(always)]
    pub fn rsie(&mut self) -> RSIE_W {
        RSIE_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&mut self) -> SDIE_W {
        SDIE_W { w: self }
    }
    #[doc = "Bit 10 - Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(&mut self) -> BEIE_W {
        BEIE_W { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 12 - Address Match 0 Interrupt Enable"]
    #[inline(always)]
    pub fn am0ie(&mut self) -> AM0IE_W {
        AM0IE_W { w: self }
    }
    #[doc = "Bit 13 - Address Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn am1f(&mut self) -> AM1F_W {
        AM1F_W { w: self }
    }
    #[doc = "Bit 14 - General Call Interrupt Enable"]
    #[inline(always)]
    pub fn gcie(&mut self) -> GCIE_W {
        GCIE_W { w: self }
    }
    #[doc = "Bit 15 - SMBus Alert Response Interrupt Enable"]
    #[inline(always)]
    pub fn sarie(&mut self) -> SARIE_W {
        SARIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sier](index.html) module"]
pub struct SIER_SPEC;
impl crate::RegisterSpec for SIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sier::R](R) reader structure"]
impl crate::Readable for SIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sier::W](W) writer structure"]
impl crate::Writable for SIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIER to value 0"]
impl crate::Resettable for SIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
