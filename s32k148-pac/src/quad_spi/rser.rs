#[doc = "Register `RSER` reader"]
pub struct R(crate::R<RSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSER` writer"]
pub struct W(crate::W<RSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSER_SPEC>;
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
impl From<crate::W<RSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transaction Finished Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFIE_A {
    #[doc = "0: No TFF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TFF interrupt will be generated"]
    _1 = 1,
}
impl From<TFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFIE` reader - Transaction Finished Interrupt Enable"]
pub struct TFIE_R(crate::FieldReader<bool, TFIE_A>);
impl TFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFIE_A {
        match self.bits {
            false => TFIE_A::_0,
            true => TFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFIE_A::_1
    }
}
impl core::ops::Deref for TFIE_R {
    type Target = crate::FieldReader<bool, TFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFIE` writer - Transaction Finished Interrupt Enable"]
pub struct TFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No TFF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFIE_A::_0)
    }
    #[doc = "TFF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFIE_A::_1)
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
#[doc = "IP Command Trigger during IP Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPIEIE_A {
    #[doc = "0: No IPIEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IPIEF interrupt will be generated"]
    _1 = 1,
}
impl From<IPIEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IPIEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPIEIE` reader - IP Command Trigger during IP Access Error Interrupt Enable"]
pub struct IPIEIE_R(crate::FieldReader<bool, IPIEIE_A>);
impl IPIEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPIEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPIEIE_A {
        match self.bits {
            false => IPIEIE_A::_0,
            true => IPIEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IPIEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IPIEIE_A::_1
    }
}
impl core::ops::Deref for IPIEIE_R {
    type Target = crate::FieldReader<bool, IPIEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPIEIE` writer - IP Command Trigger during IP Access Error Interrupt Enable"]
pub struct IPIEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPIEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPIEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No IPIEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPIEIE_A::_0)
    }
    #[doc = "IPIEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPIEIE_A::_1)
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
#[doc = "IP Command Trigger during AHB Access Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPAEIE_A {
    #[doc = "0: No IPAEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: IPAEF interrupt will be generated"]
    _1 = 1,
}
impl From<IPAEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IPAEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IPAEIE` reader - IP Command Trigger during AHB Access Error Interrupt Enable"]
pub struct IPAEIE_R(crate::FieldReader<bool, IPAEIE_A>);
impl IPAEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IPAEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPAEIE_A {
        match self.bits {
            false => IPAEIE_A::_0,
            true => IPAEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IPAEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IPAEIE_A::_1
    }
}
impl core::ops::Deref for IPAEIE_R {
    type Target = crate::FieldReader<bool, IPAEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPAEIE` writer - IP Command Trigger during AHB Access Error Interrupt Enable"]
pub struct IPAEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IPAEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPAEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No IPAEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPAEIE_A::_0)
    }
    #[doc = "IPAEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPAEIE_A::_1)
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
#[doc = "AHB Buffer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABOIE_A {
    #[doc = "0: No ABOF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ABOF interrupt will be generated"]
    _1 = 1,
}
impl From<ABOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABOIE` reader - AHB Buffer Overflow Interrupt Enable"]
pub struct ABOIE_R(crate::FieldReader<bool, ABOIE_A>);
impl ABOIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABOIE_A {
        match self.bits {
            false => ABOIE_A::_0,
            true => ABOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABOIE_A::_1
    }
}
impl core::ops::Deref for ABOIE_R {
    type Target = crate::FieldReader<bool, ABOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABOIE` writer - AHB Buffer Overflow Interrupt Enable"]
pub struct ABOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ABOF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABOIE_A::_0)
    }
    #[doc = "ABOF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABOIE_A::_1)
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
#[doc = "AHB Illegal Burst Size Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIBSIE_A {
    #[doc = "0: No AIBSEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: AIBSEF interrupt will be generated"]
    _1 = 1,
}
impl From<AIBSIE_A> for bool {
    #[inline(always)]
    fn from(variant: AIBSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIBSIE` reader - AHB Illegal Burst Size Interrupt Enable"]
pub struct AIBSIE_R(crate::FieldReader<bool, AIBSIE_A>);
impl AIBSIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AIBSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIBSIE_A {
        match self.bits {
            false => AIBSIE_A::_0,
            true => AIBSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AIBSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AIBSIE_A::_1
    }
}
impl core::ops::Deref for AIBSIE_R {
    type Target = crate::FieldReader<bool, AIBSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIBSIE` writer - AHB Illegal Burst Size Interrupt Enable"]
pub struct AIBSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AIBSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AIBSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AIBSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIBSIE_A::_0)
    }
    #[doc = "AIBSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIBSIE_A::_1)
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
#[doc = "AHB Illegal transaction interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITIE_A {
    #[doc = "0: No AITEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: AITEF interrupt will be generated"]
    _1 = 1,
}
impl From<AITIE_A> for bool {
    #[inline(always)]
    fn from(variant: AITIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AITIE` reader - AHB Illegal transaction interrupt enable."]
pub struct AITIE_R(crate::FieldReader<bool, AITIE_A>);
impl AITIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AITIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AITIE_A {
        match self.bits {
            false => AITIE_A::_0,
            true => AITIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == AITIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == AITIE_A::_1
    }
}
impl core::ops::Deref for AITIE_R {
    type Target = crate::FieldReader<bool, AITIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AITIE` writer - AHB Illegal transaction interrupt enable."]
pub struct AITIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AITIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AITIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No AITEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AITIE_A::_0)
    }
    #[doc = "AITEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AITIE_A::_1)
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
#[doc = "AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABSEIE_A {
    #[doc = "0: No ABSEF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ABSEF interrupt will be generated"]
    _1 = 1,
}
impl From<ABSEIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABSEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABSEIE` reader - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
pub struct ABSEIE_R(crate::FieldReader<bool, ABSEIE_A>);
impl ABSEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABSEIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABSEIE_A {
        match self.bits {
            false => ABSEIE_A::_0,
            true => ABSEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ABSEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ABSEIE_A::_1
    }
}
impl core::ops::Deref for ABSEIE_R {
    type Target = crate::FieldReader<bool, ABSEIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABSEIE` writer - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
pub struct ABSEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABSEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABSEIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ABSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABSEIE_A::_0)
    }
    #[doc = "ABSEF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABSEIE_A::_1)
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
#[doc = "RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDIE_A {
    #[doc = "0: No RBDF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: RBDF Interrupt will be generated"]
    _1 = 1,
}
impl From<RBDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBDIE` reader - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
pub struct RBDIE_R(crate::FieldReader<bool, RBDIE_A>);
impl RBDIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBDIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDIE_A {
        match self.bits {
            false => RBDIE_A::_0,
            true => RBDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RBDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RBDIE_A::_1
    }
}
impl core::ops::Deref for RBDIE_R {
    type Target = crate::FieldReader<bool, RBDIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBDIE` writer - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
pub struct RBDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBDIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No RBDF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDIE_A::_0)
    }
    #[doc = "RBDF Interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDIE_A::_1)
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
#[doc = "RX Buffer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBOIE_A {
    #[doc = "0: No RBOF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: RBOF interrupt will be generated"]
    _1 = 1,
}
impl From<RBOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBOIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBOIE` reader - RX Buffer Overflow Interrupt Enable"]
pub struct RBOIE_R(crate::FieldReader<bool, RBOIE_A>);
impl RBOIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBOIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBOIE_A {
        match self.bits {
            false => RBOIE_A::_0,
            true => RBOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RBOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RBOIE_A::_1
    }
}
impl core::ops::Deref for RBOIE_R {
    type Target = crate::FieldReader<bool, RBOIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBOIE` writer - RX Buffer Overflow Interrupt Enable"]
pub struct RBOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBOIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No RBOF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBOIE_A::_0)
    }
    #[doc = "RBOF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBOIE_A::_1)
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
#[doc = "RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBDDE_A {
    #[doc = "0: No DMA request will be generated"]
    _0 = 0,
    #[doc = "1: DMA request will be generated"]
    _1 = 1,
}
impl From<RBDDE_A> for bool {
    #[inline(always)]
    fn from(variant: RBDDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBDDE` reader - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
pub struct RBDDE_R(crate::FieldReader<bool, RBDDE_A>);
impl RBDDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBDDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBDDE_A {
        match self.bits {
            false => RBDDE_A::_0,
            true => RBDDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RBDDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RBDDE_A::_1
    }
}
impl core::ops::Deref for RBDDE_R {
    type Target = crate::FieldReader<bool, RBDDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBDDE` writer - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
pub struct RBDDE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBDDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBDDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA request will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RBDDE_A::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RBDDE_A::_1)
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
#[doc = "Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILLINIE_A {
    #[doc = "0: No ILLINE interrupt will be generated"]
    _0 = 0,
    #[doc = "1: ILLINE interrupt will be generated"]
    _1 = 1,
}
impl From<ILLINIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILLINIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILLINIE` reader - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
pub struct ILLINIE_R(crate::FieldReader<bool, ILLINIE_A>);
impl ILLINIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ILLINIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILLINIE_A {
        match self.bits {
            false => ILLINIE_A::_0,
            true => ILLINIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ILLINIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ILLINIE_A::_1
    }
}
impl core::ops::Deref for ILLINIE_R {
    type Target = crate::FieldReader<bool, ILLINIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ILLINIE` writer - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
pub struct ILLINIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILLINIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILLINIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ILLINE interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILLINIE_A::_0)
    }
    #[doc = "ILLINE interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILLINIE_A::_1)
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
#[doc = "TX Buffer Fill DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFDE_A {
    #[doc = "0: No DMA request will be generated"]
    _0 = 0,
    #[doc = "1: DMA request will be generated"]
    _1 = 1,
}
impl From<TBFDE_A> for bool {
    #[inline(always)]
    fn from(variant: TBFDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBFDE` reader - TX Buffer Fill DMA Enable"]
pub struct TBFDE_R(crate::FieldReader<bool, TBFDE_A>);
impl TBFDE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBFDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFDE_A {
        match self.bits {
            false => TBFDE_A::_0,
            true => TBFDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TBFDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TBFDE_A::_1
    }
}
impl core::ops::Deref for TBFDE_R {
    type Target = crate::FieldReader<bool, TBFDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBFDE` writer - TX Buffer Fill DMA Enable"]
pub struct TBFDE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBFDE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DMA request will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFDE_A::_0)
    }
    #[doc = "DMA request will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFDE_A::_1)
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
#[doc = "TX Buffer Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBUIE_A {
    #[doc = "0: No TBUF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TBUF interrupt will be generated"]
    _1 = 1,
}
impl From<TBUIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBUIE` reader - TX Buffer Underrun Interrupt Enable"]
pub struct TBUIE_R(crate::FieldReader<bool, TBUIE_A>);
impl TBUIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBUIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBUIE_A {
        match self.bits {
            false => TBUIE_A::_0,
            true => TBUIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TBUIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TBUIE_A::_1
    }
}
impl core::ops::Deref for TBUIE_R {
    type Target = crate::FieldReader<bool, TBUIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBUIE` writer - TX Buffer Underrun Interrupt Enable"]
pub struct TBUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBUIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No TBUF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBUIE_A::_0)
    }
    #[doc = "TBUF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBUIE_A::_1)
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
#[doc = "TX Buffer Fill Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBFIE_A {
    #[doc = "0: No TBFF interrupt will be generated"]
    _0 = 0,
    #[doc = "1: TBFF interrupt will be generated"]
    _1 = 1,
}
impl From<TBFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBFIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBFIE` reader - TX Buffer Fill Interrupt Enable"]
pub struct TBFIE_R(crate::FieldReader<bool, TBFIE_A>);
impl TBFIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TBFIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBFIE_A {
        match self.bits {
            false => TBFIE_A::_0,
            true => TBFIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TBFIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TBFIE_A::_1
    }
}
impl core::ops::Deref for TBFIE_R {
    type Target = crate::FieldReader<bool, TBFIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBFIE` writer - TX Buffer Fill Interrupt Enable"]
pub struct TBFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TBFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBFIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No TBFF interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TBFIE_A::_0)
    }
    #[doc = "TBFF interrupt will be generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TBFIE_A::_1)
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
impl R {
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tfie(&self) -> TFIE_R {
        TFIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipieie(&self) -> IPIEIE_R {
        IPIEIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipaeie(&self) -> IPAEIE_R {
        IPAEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn aboie(&self) -> ABOIE_R {
        ABOIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline(always)]
    pub fn aibsie(&self) -> AIBSIE_R {
        AIBSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline(always)]
    pub fn aitie(&self) -> AITIE_R {
        AITIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline(always)]
    pub fn abseie(&self) -> ABSEIE_R {
        ABSEIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdie(&self) -> RBDIE_R {
        RBDIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rboie(&self) -> RBOIE_R {
        RBOIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdde(&self) -> RBDDE_R {
        RBDDE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline(always)]
    pub fn illinie(&self) -> ILLINIE_R {
        ILLINIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline(always)]
    pub fn tbfde(&self) -> TBFDE_R {
        TBFDE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline(always)]
    pub fn tbfie(&self) -> TBFIE_R {
        TBFIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Finished Interrupt Enable"]
    #[inline(always)]
    pub fn tfie(&mut self) -> TFIE_W {
        TFIE_W { w: self }
    }
    #[doc = "Bit 6 - IP Command Trigger during IP Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipieie(&mut self) -> IPIEIE_W {
        IPIEIE_W { w: self }
    }
    #[doc = "Bit 7 - IP Command Trigger during AHB Access Error Interrupt Enable"]
    #[inline(always)]
    pub fn ipaeie(&mut self) -> IPAEIE_W {
        IPAEIE_W { w: self }
    }
    #[doc = "Bit 12 - AHB Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn aboie(&mut self) -> ABOIE_W {
        ABOIE_W { w: self }
    }
    #[doc = "Bit 13 - AHB Illegal Burst Size Interrupt Enable"]
    #[inline(always)]
    pub fn aibsie(&mut self) -> AIBSIE_W {
        AIBSIE_W { w: self }
    }
    #[doc = "Bit 14 - AHB Illegal transaction interrupt enable."]
    #[inline(always)]
    pub fn aitie(&mut self) -> AITIE_W {
        AITIE_W { w: self }
    }
    #[doc = "Bit 15 - AHB Sequence Error Interrupt Enable: Triggered by ABSEF flags of QSPI_FR"]
    #[inline(always)]
    pub fn abseie(&mut self) -> ABSEIE_W {
        ABSEIE_W { w: self }
    }
    #[doc = "Bit 16 - RX Buffer Drain Interrupt Enable: Enables generation of IRQ requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdie(&mut self) -> RBDIE_W {
        RBDIE_W { w: self }
    }
    #[doc = "Bit 17 - RX Buffer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rboie(&mut self) -> RBOIE_W {
        RBOIE_W { w: self }
    }
    #[doc = "Bit 21 - RX Buffer Drain DMA Enable: Enables generation of DMA requests for RX Buffer Drain"]
    #[inline(always)]
    pub fn rbdde(&mut self) -> RBDDE_W {
        RBDDE_W { w: self }
    }
    #[doc = "Bit 23 - Illegal Instruction Error Interrupt Enable. Triggered by ILLINE flag in QSPI_FR"]
    #[inline(always)]
    pub fn illinie(&mut self) -> ILLINIE_W {
        ILLINIE_W { w: self }
    }
    #[doc = "Bit 25 - TX Buffer Fill DMA Enable"]
    #[inline(always)]
    pub fn tbfde(&mut self) -> TBFDE_W {
        TBFDE_W { w: self }
    }
    #[doc = "Bit 26 - TX Buffer Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W {
        TBUIE_W { w: self }
    }
    #[doc = "Bit 27 - TX Buffer Fill Interrupt Enable"]
    #[inline(always)]
    pub fn tbfie(&mut self) -> TBFIE_W {
        TBFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt and DMA Request Select and Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rser](index.html) module"]
pub struct RSER_SPEC;
impl crate::RegisterSpec for RSER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rser::R](R) reader structure"]
impl crate::Readable for RSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rser::W](W) writer structure"]
impl crate::Writable for RSER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSER to value 0"]
impl crate::Resettable for RSER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
