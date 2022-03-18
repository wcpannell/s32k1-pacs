#[doc = "Register `MIER` reader"]
pub struct R(crate::R<MIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIER` writer"]
pub struct W(crate::W<MIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIER_SPEC>;
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
impl From<crate::W<MIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIER_SPEC>) -> Self {
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
#[doc = "End Packet Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPIE_A {
    #[doc = "0: Disabled"]
    EPIE_0 = 0,
    #[doc = "1: Enabled"]
    EPIE_1 = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIE` reader - End Packet Interrupt Enable"]
pub struct EPIE_R(crate::FieldReader<bool, EPIE_A>);
impl EPIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::EPIE_0,
            true => EPIE_A::EPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EPIE_0`"]
    #[inline(always)]
    pub fn is_epie_0(&self) -> bool {
        **self == EPIE_A::EPIE_0
    }
    #[doc = "Checks if the value of the field is `EPIE_1`"]
    #[inline(always)]
    pub fn is_epie_1(&self) -> bool {
        **self == EPIE_A::EPIE_1
    }
}
impl core::ops::Deref for EPIE_R {
    type Target = crate::FieldReader<bool, EPIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPIE` writer - End Packet Interrupt Enable"]
pub struct EPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn epie_0(self) -> &'a mut W {
        self.variant(EPIE_A::EPIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn epie_1(self) -> &'a mut W {
        self.variant(EPIE_A::EPIE_1)
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
#[doc = "NACK Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDIE_A {
    #[doc = "0: Disabled"]
    NDIE_0 = 0,
    #[doc = "1: Enabled"]
    NDIE_1 = 1,
}
impl From<NDIE_A> for bool {
    #[inline(always)]
    fn from(variant: NDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDIE` reader - NACK Detect Interrupt Enable"]
pub struct NDIE_R(crate::FieldReader<bool, NDIE_A>);
impl NDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDIE_A {
        match self.bits {
            false => NDIE_A::NDIE_0,
            true => NDIE_A::NDIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDIE_0`"]
    #[inline(always)]
    pub fn is_ndie_0(&self) -> bool {
        **self == NDIE_A::NDIE_0
    }
    #[doc = "Checks if the value of the field is `NDIE_1`"]
    #[inline(always)]
    pub fn is_ndie_1(&self) -> bool {
        **self == NDIE_A::NDIE_1
    }
}
impl core::ops::Deref for NDIE_R {
    type Target = crate::FieldReader<bool, NDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDIE` writer - NACK Detect Interrupt Enable"]
pub struct NDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ndie_0(self) -> &'a mut W {
        self.variant(NDIE_A::NDIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ndie_1(self) -> &'a mut W {
        self.variant(NDIE_A::NDIE_1)
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
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIE_A {
    #[doc = "0: Disabled"]
    ALIE_0 = 0,
    #[doc = "1: Enabled"]
    ALIE_1 = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable"]
pub struct ALIE_R(crate::FieldReader<bool, ALIE_A>);
impl ALIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::ALIE_0,
            true => ALIE_A::ALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ALIE_0`"]
    #[inline(always)]
    pub fn is_alie_0(&self) -> bool {
        **self == ALIE_A::ALIE_0
    }
    #[doc = "Checks if the value of the field is `ALIE_1`"]
    #[inline(always)]
    pub fn is_alie_1(&self) -> bool {
        **self == ALIE_A::ALIE_1
    }
}
impl core::ops::Deref for ALIE_R {
    type Target = crate::FieldReader<bool, ALIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable"]
pub struct ALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn alie_0(self) -> &'a mut W {
        self.variant(ALIE_A::ALIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn alie_1(self) -> &'a mut W {
        self.variant(ALIE_A::ALIE_1)
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
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIE_A {
    #[doc = "0: Enabled"]
    FEIE_0 = 0,
    #[doc = "1: Disabled"]
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
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn feie_0(self) -> &'a mut W {
        self.variant(FEIE_A::FEIE_0)
    }
    #[doc = "Disabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Pin Low Timeout Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTIE_A {
    #[doc = "0: Disabled"]
    PLTIE_0 = 0,
    #[doc = "1: Enabled"]
    PLTIE_1 = 1,
}
impl From<PLTIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLTIE` reader - Pin Low Timeout Interrupt Enable"]
pub struct PLTIE_R(crate::FieldReader<bool, PLTIE_A>);
impl PLTIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTIE_A {
        match self.bits {
            false => PLTIE_A::PLTIE_0,
            true => PLTIE_A::PLTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLTIE_0`"]
    #[inline(always)]
    pub fn is_pltie_0(&self) -> bool {
        **self == PLTIE_A::PLTIE_0
    }
    #[doc = "Checks if the value of the field is `PLTIE_1`"]
    #[inline(always)]
    pub fn is_pltie_1(&self) -> bool {
        **self == PLTIE_A::PLTIE_1
    }
}
impl core::ops::Deref for PLTIE_R {
    type Target = crate::FieldReader<bool, PLTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLTIE` writer - Pin Low Timeout Interrupt Enable"]
pub struct PLTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn pltie_0(self) -> &'a mut W {
        self.variant(PLTIE_A::PLTIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn pltie_1(self) -> &'a mut W {
        self.variant(PLTIE_A::PLTIE_1)
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
#[doc = "Data Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMIE_A {
    #[doc = "0: Disabled"]
    DMIE_0 = 0,
    #[doc = "1: Enabled"]
    DMIE_1 = 1,
}
impl From<DMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMIE` reader - Data Match Interrupt Enable"]
pub struct DMIE_R(crate::FieldReader<bool, DMIE_A>);
impl DMIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIE_A {
        match self.bits {
            false => DMIE_A::DMIE_0,
            true => DMIE_A::DMIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMIE_0`"]
    #[inline(always)]
    pub fn is_dmie_0(&self) -> bool {
        **self == DMIE_A::DMIE_0
    }
    #[doc = "Checks if the value of the field is `DMIE_1`"]
    #[inline(always)]
    pub fn is_dmie_1(&self) -> bool {
        **self == DMIE_A::DMIE_1
    }
}
impl core::ops::Deref for DMIE_R {
    type Target = crate::FieldReader<bool, DMIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIE` writer - Data Match Interrupt Enable"]
pub struct DMIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dmie_0(self) -> &'a mut W {
        self.variant(DMIE_A::DMIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dmie_1(self) -> &'a mut W {
        self.variant(DMIE_A::DMIE_1)
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
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&self) -> SDIE_R {
        SDIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ndie(&self) -> NDIE_R {
        NDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn pltie(&self) -> PLTIE_R {
        PLTIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&self) -> DMIE_R {
        DMIE_R::new(((self.bits >> 14) & 0x01) != 0)
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
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W {
        EPIE_W { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&mut self) -> SDIE_W {
        SDIE_W { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ndie(&mut self) -> NDIE_W {
        NDIE_W { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W {
        ALIE_W { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn pltie(&mut self) -> PLTIE_W {
        PLTIE_W { w: self }
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&mut self) -> DMIE_W {
        DMIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mier](index.html) module"]
pub struct MIER_SPEC;
impl crate::RegisterSpec for MIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mier::R](R) reader structure"]
impl crate::Readable for MIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mier::W](W) writer structure"]
impl crate::Writable for MIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIER to value 0"]
impl crate::Resettable for MIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
