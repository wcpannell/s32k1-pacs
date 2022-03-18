#[doc = "Register `SCFGR1` reader"]
pub struct R(crate::R<SCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGR1` writer"]
pub struct W(crate::W<SCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGR1_SPEC>;
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
impl From<crate::W<SCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Address SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    ADRSTALL_0 = 0,
    #[doc = "1: Clock stretching is enabled"]
    ADRSTALL_1 = 1,
}
impl From<ADRSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ADRSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRSTALL` reader - Address SCL Stall"]
pub struct ADRSTALL_R(crate::FieldReader<bool, ADRSTALL_A>);
impl ADRSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADRSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRSTALL_A {
        match self.bits {
            false => ADRSTALL_A::ADRSTALL_0,
            true => ADRSTALL_A::ADRSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADRSTALL_0`"]
    #[inline(always)]
    pub fn is_adrstall_0(&self) -> bool {
        **self == ADRSTALL_A::ADRSTALL_0
    }
    #[doc = "Checks if the value of the field is `ADRSTALL_1`"]
    #[inline(always)]
    pub fn is_adrstall_1(&self) -> bool {
        **self == ADRSTALL_A::ADRSTALL_1
    }
}
impl core::ops::Deref for ADRSTALL_R {
    type Target = crate::FieldReader<bool, ADRSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRSTALL` writer - Address SCL Stall"]
pub struct ADRSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADRSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn adrstall_0(self) -> &'a mut W {
        self.variant(ADRSTALL_A::ADRSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn adrstall_1(self) -> &'a mut W {
        self.variant(ADRSTALL_A::ADRSTALL_1)
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
#[doc = "RX SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    RXSTALL_0 = 0,
    #[doc = "1: Clock stretching is enabled"]
    RXSTALL_1 = 1,
}
impl From<RXSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTALL` reader - RX SCL Stall"]
pub struct RXSTALL_R(crate::FieldReader<bool, RXSTALL_A>);
impl RXSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTALL_A {
        match self.bits {
            false => RXSTALL_A::RXSTALL_0,
            true => RXSTALL_A::RXSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXSTALL_0`"]
    #[inline(always)]
    pub fn is_rxstall_0(&self) -> bool {
        **self == RXSTALL_A::RXSTALL_0
    }
    #[doc = "Checks if the value of the field is `RXSTALL_1`"]
    #[inline(always)]
    pub fn is_rxstall_1(&self) -> bool {
        **self == RXSTALL_A::RXSTALL_1
    }
}
impl core::ops::Deref for RXSTALL_R {
    type Target = crate::FieldReader<bool, RXSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALL` writer - RX SCL Stall"]
pub struct RXSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn rxstall_0(self) -> &'a mut W {
        self.variant(RXSTALL_A::RXSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn rxstall_1(self) -> &'a mut W {
        self.variant(RXSTALL_A::RXSTALL_1)
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
#[doc = "TX Data SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    TXDSTALL_0 = 0,
    #[doc = "1: Clock stretching is enabled"]
    TXDSTALL_1 = 1,
}
impl From<TXDSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: TXDSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSTALL` reader - TX Data SCL Stall"]
pub struct TXDSTALL_R(crate::FieldReader<bool, TXDSTALL_A>);
impl TXDSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXDSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDSTALL_A {
        match self.bits {
            false => TXDSTALL_A::TXDSTALL_0,
            true => TXDSTALL_A::TXDSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXDSTALL_0`"]
    #[inline(always)]
    pub fn is_txdstall_0(&self) -> bool {
        **self == TXDSTALL_A::TXDSTALL_0
    }
    #[doc = "Checks if the value of the field is `TXDSTALL_1`"]
    #[inline(always)]
    pub fn is_txdstall_1(&self) -> bool {
        **self == TXDSTALL_A::TXDSTALL_1
    }
}
impl core::ops::Deref for TXDSTALL_R {
    type Target = crate::FieldReader<bool, TXDSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDSTALL` writer - TX Data SCL Stall"]
pub struct TXDSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn txdstall_0(self) -> &'a mut W {
        self.variant(TXDSTALL_A::TXDSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn txdstall_1(self) -> &'a mut W {
        self.variant(TXDSTALL_A::TXDSTALL_1)
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
#[doc = "ACK SCL Stall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKSTALL_A {
    #[doc = "0: Clock stretching is disabled"]
    ACKSTALL_0 = 0,
    #[doc = "1: Clock stretching is enabled"]
    ACKSTALL_1 = 1,
}
impl From<ACKSTALL_A> for bool {
    #[inline(always)]
    fn from(variant: ACKSTALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKSTALL` reader - ACK SCL Stall"]
pub struct ACKSTALL_R(crate::FieldReader<bool, ACKSTALL_A>);
impl ACKSTALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACKSTALL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKSTALL_A {
        match self.bits {
            false => ACKSTALL_A::ACKSTALL_0,
            true => ACKSTALL_A::ACKSTALL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACKSTALL_0`"]
    #[inline(always)]
    pub fn is_ackstall_0(&self) -> bool {
        **self == ACKSTALL_A::ACKSTALL_0
    }
    #[doc = "Checks if the value of the field is `ACKSTALL_1`"]
    #[inline(always)]
    pub fn is_ackstall_1(&self) -> bool {
        **self == ACKSTALL_A::ACKSTALL_1
    }
}
impl core::ops::Deref for ACKSTALL_R {
    type Target = crate::FieldReader<bool, ACKSTALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKSTALL` writer - ACK SCL Stall"]
pub struct ACKSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKSTALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACKSTALL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock stretching is disabled"]
    #[inline(always)]
    pub fn ackstall_0(self) -> &'a mut W {
        self.variant(ACKSTALL_A::ACKSTALL_0)
    }
    #[doc = "Clock stretching is enabled"]
    #[inline(always)]
    pub fn ackstall_1(self) -> &'a mut W {
        self.variant(ACKSTALL_A::ACKSTALL_1)
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
#[doc = "General Call Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCEN_A {
    #[doc = "0: General Call address is disabled"]
    GCEN_0 = 0,
    #[doc = "1: General Call address is enabled"]
    GCEN_1 = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - General Call Enable"]
pub struct GCEN_R(crate::FieldReader<bool, GCEN_A>);
impl GCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::GCEN_0,
            true => GCEN_A::GCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GCEN_0`"]
    #[inline(always)]
    pub fn is_gcen_0(&self) -> bool {
        **self == GCEN_A::GCEN_0
    }
    #[doc = "Checks if the value of the field is `GCEN_1`"]
    #[inline(always)]
    pub fn is_gcen_1(&self) -> bool {
        **self == GCEN_A::GCEN_1
    }
}
impl core::ops::Deref for GCEN_R {
    type Target = crate::FieldReader<bool, GCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCEN` writer - General Call Enable"]
pub struct GCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "General Call address is disabled"]
    #[inline(always)]
    pub fn gcen_0(self) -> &'a mut W {
        self.variant(GCEN_A::GCEN_0)
    }
    #[doc = "General Call address is enabled"]
    #[inline(always)]
    pub fn gcen_1(self) -> &'a mut W {
        self.variant(GCEN_A::GCEN_1)
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
#[doc = "SMBus Alert Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAEN_A {
    #[doc = "0: Disables match on SMBus Alert"]
    SAEN_0 = 0,
    #[doc = "1: Enables match on SMBus Alert"]
    SAEN_1 = 1,
}
impl From<SAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAEN` reader - SMBus Alert Enable"]
pub struct SAEN_R(crate::FieldReader<bool, SAEN_A>);
impl SAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAEN_A {
        match self.bits {
            false => SAEN_A::SAEN_0,
            true => SAEN_A::SAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAEN_0`"]
    #[inline(always)]
    pub fn is_saen_0(&self) -> bool {
        **self == SAEN_A::SAEN_0
    }
    #[doc = "Checks if the value of the field is `SAEN_1`"]
    #[inline(always)]
    pub fn is_saen_1(&self) -> bool {
        **self == SAEN_A::SAEN_1
    }
}
impl core::ops::Deref for SAEN_R {
    type Target = crate::FieldReader<bool, SAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAEN` writer - SMBus Alert Enable"]
pub struct SAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables match on SMBus Alert"]
    #[inline(always)]
    pub fn saen_0(self) -> &'a mut W {
        self.variant(SAEN_A::SAEN_0)
    }
    #[doc = "Enables match on SMBus Alert"]
    #[inline(always)]
    pub fn saen_1(self) -> &'a mut W {
        self.variant(SAEN_A::SAEN_1)
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
#[doc = "Transmit Flag Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCFG_A {
    #[doc = "0: Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
    TXCFG_0 = 0,
    #[doc = "1: Transmit Data Flag will assert whenever the Transmit Data register is empty"]
    TXCFG_1 = 1,
}
impl From<TXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: TXCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCFG` reader - Transmit Flag Configuration"]
pub struct TXCFG_R(crate::FieldReader<bool, TXCFG_A>);
impl TXCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXCFG_A {
        match self.bits {
            false => TXCFG_A::TXCFG_0,
            true => TXCFG_A::TXCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXCFG_0`"]
    #[inline(always)]
    pub fn is_txcfg_0(&self) -> bool {
        **self == TXCFG_A::TXCFG_0
    }
    #[doc = "Checks if the value of the field is `TXCFG_1`"]
    #[inline(always)]
    pub fn is_txcfg_1(&self) -> bool {
        **self == TXCFG_A::TXCFG_1
    }
}
impl core::ops::Deref for TXCFG_R {
    type Target = crate::FieldReader<bool, TXCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCFG` writer - Transmit Flag Configuration"]
pub struct TXCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the Transmit Data register is empty"]
    #[inline(always)]
    pub fn txcfg_0(self) -> &'a mut W {
        self.variant(TXCFG_A::TXCFG_0)
    }
    #[doc = "Transmit Data Flag will assert whenever the Transmit Data register is empty"]
    #[inline(always)]
    pub fn txcfg_1(self) -> &'a mut W {
        self.variant(TXCFG_A::TXCFG_1)
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
#[doc = "Receive Data Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCFG_A {
    #[doc = "0: Reading the Receive Data register will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
    RXCFG_0 = 0,
    #[doc = "1: Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
    RXCFG_1 = 1,
}
impl From<RXCFG_A> for bool {
    #[inline(always)]
    fn from(variant: RXCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXCFG` reader - Receive Data Configuration"]
pub struct RXCFG_R(crate::FieldReader<bool, RXCFG_A>);
impl RXCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCFG_A {
        match self.bits {
            false => RXCFG_A::RXCFG_0,
            true => RXCFG_A::RXCFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXCFG_0`"]
    #[inline(always)]
    pub fn is_rxcfg_0(&self) -> bool {
        **self == RXCFG_A::RXCFG_0
    }
    #[doc = "Checks if the value of the field is `RXCFG_1`"]
    #[inline(always)]
    pub fn is_rxcfg_1(&self) -> bool {
        **self == RXCFG_A::RXCFG_1
    }
}
impl core::ops::Deref for RXCFG_R {
    type Target = crate::FieldReader<bool, RXCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXCFG` writer - Receive Data Configuration"]
pub struct RXCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reading the Receive Data register will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
    #[inline(always)]
    pub fn rxcfg_0(self) -> &'a mut W {
        self.variant(RXCFG_A::RXCFG_0)
    }
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\])is set, will return the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, will return received data and clear the Receive Data flag (MSR\\[RDF\\])."]
    #[inline(always)]
    pub fn rxcfg_1(self) -> &'a mut W {
        self.variant(RXCFG_A::RXCFG_1)
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
#[doc = "Ignore NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACK_A {
    #[doc = "0: Slave will end transfer when NACK is detected"]
    IGNACK_0 = 0,
    #[doc = "1: Slave will not end transfer when NACK detected"]
    IGNACK_1 = 1,
}
impl From<IGNACK_A> for bool {
    #[inline(always)]
    fn from(variant: IGNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IGNACK` reader - Ignore NACK"]
pub struct IGNACK_R(crate::FieldReader<bool, IGNACK_A>);
impl IGNACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IGNACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGNACK_A {
        match self.bits {
            false => IGNACK_A::IGNACK_0,
            true => IGNACK_A::IGNACK_1,
        }
    }
    #[doc = "Checks if the value of the field is `IGNACK_0`"]
    #[inline(always)]
    pub fn is_ignack_0(&self) -> bool {
        **self == IGNACK_A::IGNACK_0
    }
    #[doc = "Checks if the value of the field is `IGNACK_1`"]
    #[inline(always)]
    pub fn is_ignack_1(&self) -> bool {
        **self == IGNACK_A::IGNACK_1
    }
}
impl core::ops::Deref for IGNACK_R {
    type Target = crate::FieldReader<bool, IGNACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNACK` writer - Ignore NACK"]
pub struct IGNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGNACK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave will end transfer when NACK is detected"]
    #[inline(always)]
    pub fn ignack_0(self) -> &'a mut W {
        self.variant(IGNACK_A::IGNACK_0)
    }
    #[doc = "Slave will not end transfer when NACK detected"]
    #[inline(always)]
    pub fn ignack_1(self) -> &'a mut W {
        self.variant(IGNACK_A::IGNACK_1)
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
#[doc = "High Speed Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMEN_A {
    #[doc = "0: Disables detection of HS-mode master code"]
    HSMEN_0 = 0,
    #[doc = "1: Enables detection of HS-mode master code"]
    HSMEN_1 = 1,
}
impl From<HSMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMEN` reader - High Speed Mode Enable"]
pub struct HSMEN_R(crate::FieldReader<bool, HSMEN_A>);
impl HSMEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSMEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMEN_A {
        match self.bits {
            false => HSMEN_A::HSMEN_0,
            true => HSMEN_A::HSMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSMEN_0`"]
    #[inline(always)]
    pub fn is_hsmen_0(&self) -> bool {
        **self == HSMEN_A::HSMEN_0
    }
    #[doc = "Checks if the value of the field is `HSMEN_1`"]
    #[inline(always)]
    pub fn is_hsmen_1(&self) -> bool {
        **self == HSMEN_A::HSMEN_1
    }
}
impl core::ops::Deref for HSMEN_R {
    type Target = crate::FieldReader<bool, HSMEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMEN` writer - High Speed Mode Enable"]
pub struct HSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSMEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables detection of HS-mode master code"]
    #[inline(always)]
    pub fn hsmen_0(self) -> &'a mut W {
        self.variant(HSMEN_A::HSMEN_0)
    }
    #[doc = "Enables detection of HS-mode master code"]
    #[inline(always)]
    pub fn hsmen_1(self) -> &'a mut W {
        self.variant(HSMEN_A::HSMEN_1)
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
#[doc = "Address Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDRCFG_A {
    #[doc = "0: Address match 0 (7-bit)"]
    ADDRCFG_0 = 0,
    #[doc = "1: Address match 0 (10-bit)"]
    ADDRCFG_1 = 1,
    #[doc = "2: Address match 0 (7-bit) or Address match 1 (7-bit)"]
    ADDRCFG_2 = 2,
    #[doc = "3: Address match 0 (10-bit) or Address match 1 (10-bit)"]
    ADDRCFG_3 = 3,
    #[doc = "4: Address match 0 (7-bit) or Address match 1 (10-bit)"]
    ADDRCFG_4 = 4,
    #[doc = "5: Address match 0 (10-bit) or Address match 1 (7-bit)"]
    ADDRCFG_5 = 5,
    #[doc = "6: From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    ADDRCFG_6 = 6,
    #[doc = "7: From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    ADDRCFG_7 = 7,
}
impl From<ADDRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDRCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDRCFG` reader - Address Configuration"]
pub struct ADDRCFG_R(crate::FieldReader<u8, ADDRCFG_A>);
impl ADDRCFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDRCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRCFG_A {
        match self.bits {
            0 => ADDRCFG_A::ADDRCFG_0,
            1 => ADDRCFG_A::ADDRCFG_1,
            2 => ADDRCFG_A::ADDRCFG_2,
            3 => ADDRCFG_A::ADDRCFG_3,
            4 => ADDRCFG_A::ADDRCFG_4,
            5 => ADDRCFG_A::ADDRCFG_5,
            6 => ADDRCFG_A::ADDRCFG_6,
            7 => ADDRCFG_A::ADDRCFG_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_0`"]
    #[inline(always)]
    pub fn is_addrcfg_0(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_0
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_1`"]
    #[inline(always)]
    pub fn is_addrcfg_1(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_1
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_2`"]
    #[inline(always)]
    pub fn is_addrcfg_2(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_2
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_3`"]
    #[inline(always)]
    pub fn is_addrcfg_3(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_3
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_4`"]
    #[inline(always)]
    pub fn is_addrcfg_4(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_4
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_5`"]
    #[inline(always)]
    pub fn is_addrcfg_5(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_5
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_6`"]
    #[inline(always)]
    pub fn is_addrcfg_6(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_6
    }
    #[doc = "Checks if the value of the field is `ADDRCFG_7`"]
    #[inline(always)]
    pub fn is_addrcfg_7(&self) -> bool {
        **self == ADDRCFG_A::ADDRCFG_7
    }
}
impl core::ops::Deref for ADDRCFG_R {
    type Target = crate::FieldReader<u8, ADDRCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRCFG` writer - Address Configuration"]
pub struct ADDRCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Address match 0 (7-bit)"]
    #[inline(always)]
    pub fn addrcfg_0(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_0)
    }
    #[doc = "Address match 0 (10-bit)"]
    #[inline(always)]
    pub fn addrcfg_1(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_1)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn addrcfg_2(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_2)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn addrcfg_3(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_3)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn addrcfg_4(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_4)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn addrcfg_5(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_5)
    }
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    #[inline(always)]
    pub fn addrcfg_6(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_6)
    }
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    #[inline(always)]
    pub fn addrcfg_7(self) -> &'a mut W {
        self.variant(ADDRCFG_A::ADDRCFG_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    pub fn adrstall(&self) -> ADRSTALL_R {
        ADRSTALL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    pub fn rxstall(&self) -> RXSTALL_R {
        RXSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    pub fn txdstall(&self) -> TXDSTALL_R {
        TXDSTALL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    pub fn ackstall(&self) -> ACKSTALL_R {
        ACKSTALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    pub fn saen(&self) -> SAEN_R {
        SAEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    pub fn txcfg(&self) -> TXCFG_R {
        TXCFG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    pub fn rxcfg(&self) -> RXCFG_R {
        RXCFG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    pub fn ignack(&self) -> IGNACK_R {
        IGNACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsmen(&self) -> HSMEN_R {
        HSMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    pub fn addrcfg(&self) -> ADDRCFG_R {
        ADDRCFG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline(always)]
    pub fn adrstall(&mut self) -> ADRSTALL_W {
        ADRSTALL_W { w: self }
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline(always)]
    pub fn rxstall(&mut self) -> RXSTALL_W {
        RXSTALL_W { w: self }
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline(always)]
    pub fn txdstall(&mut self) -> TXDSTALL_W {
        TXDSTALL_W { w: self }
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline(always)]
    pub fn ackstall(&mut self) -> ACKSTALL_W {
        ACKSTALL_W { w: self }
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W {
        GCEN_W { w: self }
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline(always)]
    pub fn saen(&mut self) -> SAEN_W {
        SAEN_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline(always)]
    pub fn txcfg(&mut self) -> TXCFG_W {
        TXCFG_W { w: self }
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline(always)]
    pub fn rxcfg(&mut self) -> RXCFG_W {
        RXCFG_W { w: self }
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline(always)]
    pub fn ignack(&mut self) -> IGNACK_W {
        IGNACK_W { w: self }
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline(always)]
    pub fn hsmen(&mut self) -> HSMEN_W {
        HSMEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline(always)]
    pub fn addrcfg(&mut self) -> ADDRCFG_W {
        ADDRCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr1](index.html) module"]
pub struct SCFGR1_SPEC;
impl crate::RegisterSpec for SCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgr1::R](R) reader structure"]
impl crate::Readable for SCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgr1::W](W) writer structure"]
impl crate::Writable for SCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCFGR1 to value 0"]
impl crate::Resettable for SCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
