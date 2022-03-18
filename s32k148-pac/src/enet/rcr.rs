#[doc = "Register `RCR` reader"]
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR` writer"]
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Loopback\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOP_A {
    #[doc = "0: Loopback disabled."]
    _0 = 0,
    #[doc = "1: Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    _1 = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOP` reader - Internal Loopback"]
pub struct LOOP_R(crate::FieldReader<bool, LOOP_A>);
impl LOOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::_0,
            true => LOOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LOOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOOP_A::_1
    }
}
impl core::ops::Deref for LOOP_R {
    type Target = crate::FieldReader<bool, LOOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOP` writer - Internal Loopback"]
pub struct LOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Loopback disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOP_A::_0)
    }
    #[doc = "Transmitted frames are looped back internal to the device and transmit MII output signals are not asserted. DRT must be cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOP_A::_1)
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
#[doc = "Disable Receive On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRT_A {
    #[doc = "0: Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    _0 = 0,
    #[doc = "1: Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    _1 = 1,
}
impl From<DRT_A> for bool {
    #[inline(always)]
    fn from(variant: DRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRT` reader - Disable Receive On Transmit"]
pub struct DRT_R(crate::FieldReader<bool, DRT_A>);
impl DRT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRT_A {
        match self.bits {
            false => DRT_A::_0,
            true => DRT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DRT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DRT_A::_1
    }
}
impl core::ops::Deref for DRT_R {
    type Target = crate::FieldReader<bool, DRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRT` writer - Disable Receive On Transmit"]
pub struct DRT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive path operates independently of transmit (i.e., full-duplex mode). Can also be used to monitor transmit activity in half-duplex mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRT_A::_0)
    }
    #[doc = "Disable reception of frames while transmitting. (Normally used for half-duplex mode.)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRT_A::_1)
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
#[doc = "Media Independent Interface Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MII_MODE_A {
    #[doc = "1: MII or RMII mode, as indicated by the RMII_MODE field."]
    _1 = 1,
}
impl From<MII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MII_MODE` reader - Media Independent Interface Mode"]
pub struct MII_MODE_R(crate::FieldReader<bool, MII_MODE_A>);
impl MII_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MII_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MII_MODE_A> {
        match self.bits {
            true => Some(MII_MODE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MII_MODE_A::_1
    }
}
impl core::ops::Deref for MII_MODE_R {
    type Target = crate::FieldReader<bool, MII_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MII_MODE` writer - Media Independent Interface Mode"]
pub struct MII_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MII_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MII_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MII or RMII mode, as indicated by the RMII_MODE field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MII_MODE_A::_1)
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
#[doc = "Promiscuous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROM_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Enabled."]
    _1 = 1,
}
impl From<PROM_A> for bool {
    #[inline(always)]
    fn from(variant: PROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROM` reader - Promiscuous Mode"]
pub struct PROM_R(crate::FieldReader<bool, PROM_A>);
impl PROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROM_A {
        match self.bits {
            false => PROM_A::_0,
            true => PROM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PROM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PROM_A::_1
    }
}
impl core::ops::Deref for PROM_R {
    type Target = crate::FieldReader<bool, PROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROM` writer - Promiscuous Mode"]
pub struct PROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROM_A::_0)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROM_A::_1)
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
#[doc = "Field `BC_REJ` reader - Broadcast Frame Reject"]
pub struct BC_REJ_R(crate::FieldReader<bool, bool>);
impl BC_REJ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BC_REJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BC_REJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BC_REJ` writer - Broadcast Frame Reject"]
pub struct BC_REJ_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_REJ_W<'a> {
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
#[doc = "Field `FCE` reader - Flow Control Enable"]
pub struct FCE_R(crate::FieldReader<bool, bool>);
impl FCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCE` writer - Flow Control Enable"]
pub struct FCE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCE_W<'a> {
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
#[doc = "RMII Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_MODE_A {
    #[doc = "0: MAC configured for MII mode."]
    _0 = 0,
    #[doc = "1: MAC configured for RMII operation."]
    _1 = 1,
}
impl From<RMII_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_MODE` reader - RMII Mode Enable"]
pub struct RMII_MODE_R(crate::FieldReader<bool, RMII_MODE_A>);
impl RMII_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_MODE_A {
        match self.bits {
            false => RMII_MODE_A::_0,
            true => RMII_MODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RMII_MODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RMII_MODE_A::_1
    }
}
impl core::ops::Deref for RMII_MODE_R {
    type Target = crate::FieldReader<bool, RMII_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_MODE` writer - RMII Mode Enable"]
pub struct RMII_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMII_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC configured for MII mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMII_MODE_A::_0)
    }
    #[doc = "MAC configured for RMII operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMII_MODE_A::_1)
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
#[doc = "Enables 10-Mbit/s mode of the RMII .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMII_10T_A {
    #[doc = "0: 100-Mbit/s operation."]
    _0 = 0,
    #[doc = "1: 10-Mbit/s operation."]
    _1 = 1,
}
impl From<RMII_10T_A> for bool {
    #[inline(always)]
    fn from(variant: RMII_10T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMII_10T` reader - Enables 10-Mbit/s mode of the RMII ."]
pub struct RMII_10T_R(crate::FieldReader<bool, RMII_10T_A>);
impl RMII_10T_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RMII_10T_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMII_10T_A {
        match self.bits {
            false => RMII_10T_A::_0,
            true => RMII_10T_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RMII_10T_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RMII_10T_A::_1
    }
}
impl core::ops::Deref for RMII_10T_R {
    type Target = crate::FieldReader<bool, RMII_10T_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_10T` writer - Enables 10-Mbit/s mode of the RMII ."]
pub struct RMII_10T_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_10T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMII_10T_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "100-Mbit/s operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMII_10T_A::_0)
    }
    #[doc = "10-Mbit/s operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMII_10T_A::_1)
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
#[doc = "Enable Frame Padding Remove On Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADEN_A {
    #[doc = "0: No padding is removed on receive by the MAC."]
    _0 = 0,
    #[doc = "1: Padding is removed from received frames."]
    _1 = 1,
}
impl From<PADEN_A> for bool {
    #[inline(always)]
    fn from(variant: PADEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PADEN` reader - Enable Frame Padding Remove On Receive"]
pub struct PADEN_R(crate::FieldReader<bool, PADEN_A>);
impl PADEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADEN_A {
        match self.bits {
            false => PADEN_A::_0,
            true => PADEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PADEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PADEN_A::_1
    }
}
impl core::ops::Deref for PADEN_R {
    type Target = crate::FieldReader<bool, PADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PADEN` writer - Enable Frame Padding Remove On Receive"]
pub struct PADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No padding is removed on receive by the MAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PADEN_A::_0)
    }
    #[doc = "Padding is removed from received frames."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PADEN_A::_1)
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
#[doc = "Terminate/Forward Pause Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAUFWD_A {
    #[doc = "0: Pause frames are terminated and discarded in the MAC."]
    _0 = 0,
    #[doc = "1: Pause frames are forwarded to the user application."]
    _1 = 1,
}
impl From<PAUFWD_A> for bool {
    #[inline(always)]
    fn from(variant: PAUFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAUFWD` reader - Terminate/Forward Pause Frames"]
pub struct PAUFWD_R(crate::FieldReader<bool, PAUFWD_A>);
impl PAUFWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAUFWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAUFWD_A {
        match self.bits {
            false => PAUFWD_A::_0,
            true => PAUFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PAUFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PAUFWD_A::_1
    }
}
impl core::ops::Deref for PAUFWD_R {
    type Target = crate::FieldReader<bool, PAUFWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAUFWD` writer - Terminate/Forward Pause Frames"]
pub struct PAUFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAUFWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pause frames are terminated and discarded in the MAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PAUFWD_A::_0)
    }
    #[doc = "Pause frames are forwarded to the user application."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PAUFWD_A::_1)
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
#[doc = "Terminate/Forward Received CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWD_A {
    #[doc = "0: The CRC field of received frames is transmitted to the user application."]
    _0 = 0,
    #[doc = "1: The CRC field is stripped from the frame."]
    _1 = 1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCFWD` reader - Terminate/Forward Received CRC"]
pub struct CRCFWD_R(crate::FieldReader<bool, CRCFWD_A>);
impl CRCFWD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCFWD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCFWD_A {
        match self.bits {
            false => CRCFWD_A::_0,
            true => CRCFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CRCFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CRCFWD_A::_1
    }
}
impl core::ops::Deref for CRCFWD_R {
    type Target = crate::FieldReader<bool, CRCFWD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCFWD` writer - Terminate/Forward Received CRC"]
pub struct CRCFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCFWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CRC field of received frames is transmitted to the user application."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWD_A::_0)
    }
    #[doc = "The CRC field is stripped from the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCFWD_A::_1)
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
#[doc = "MAC Control Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFEN_A {
    #[doc = "0: MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    _0 = 0,
    #[doc = "1: MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    _1 = 1,
}
impl From<CFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFEN` reader - MAC Control Frame Enable"]
pub struct CFEN_R(crate::FieldReader<bool, CFEN_A>);
impl CFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFEN_A {
        match self.bits {
            false => CFEN_A::_0,
            true => CFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CFEN_A::_1
    }
}
impl core::ops::Deref for CFEN_R {
    type Target = crate::FieldReader<bool, CFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEN` writer - MAC Control Frame Enable"]
pub struct CFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are accepted and forwarded to the client interface."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFEN_A::_0)
    }
    #[doc = "MAC control frames with any opcode other than 0x0001 (pause frame) are silently discarded."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFEN_A::_1)
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
#[doc = "Field `MAX_FL` reader - Maximum Frame Length"]
pub struct MAX_FL_R(crate::FieldReader<u16, u16>);
impl MAX_FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MAX_FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_FL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_FL` writer - Maximum Frame Length"]
pub struct MAX_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Payload Length Check Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NLC_A {
    #[doc = "0: The payload length check is disabled."]
    _0 = 0,
    #[doc = "1: The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    _1 = 1,
}
impl From<NLC_A> for bool {
    #[inline(always)]
    fn from(variant: NLC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NLC` reader - Payload Length Check Disable"]
pub struct NLC_R(crate::FieldReader<bool, NLC_A>);
impl NLC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NLC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NLC_A {
        match self.bits {
            false => NLC_A::_0,
            true => NLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == NLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NLC_A::_1
    }
}
impl core::ops::Deref for NLC_R {
    type Target = crate::FieldReader<bool, NLC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NLC` writer - Payload Length Check Disable"]
pub struct NLC_W<'a> {
    w: &'a mut W,
}
impl<'a> NLC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NLC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The payload length check is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NLC_A::_0)
    }
    #[doc = "The core checks the frame's payload length with the frame length/type field. Errors are indicated in the EIR\\[PLR\\]
field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NLC_A::_1)
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
#[doc = "Field `GRS` reader - Graceful Receive Stopped"]
pub struct GRS_R(crate::FieldReader<bool, bool>);
impl GRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    pub fn drt(&self) -> DRT_R {
        DRT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    pub fn mii_mode(&self) -> MII_MODE_R {
        MII_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prom(&self) -> PROM_R {
        PROM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    pub fn bc_rej(&self) -> BC_REJ_R {
        BC_REJ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    pub fn fce(&self) -> FCE_R {
        FCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    pub fn rmii_mode(&self) -> RMII_MODE_R {
        RMII_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub fn rmii_10t(&self) -> RMII_10T_R {
        RMII_10T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub fn paden(&self) -> PADEN_R {
        PADEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub fn paufwd(&self) -> PAUFWD_R {
        PAUFWD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    pub fn cfen(&self) -> CFEN_R {
        CFEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    pub fn max_fl(&self) -> MAX_FL_R {
        MAX_FL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    pub fn nlc(&self) -> NLC_R {
        NLC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Graceful Receive Stopped"]
    #[inline(always)]
    pub fn grs(&self) -> GRS_R {
        GRS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Loopback"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LOOP_W {
        LOOP_W { w: self }
    }
    #[doc = "Bit 1 - Disable Receive On Transmit"]
    #[inline(always)]
    pub fn drt(&mut self) -> DRT_W {
        DRT_W { w: self }
    }
    #[doc = "Bit 2 - Media Independent Interface Mode"]
    #[inline(always)]
    pub fn mii_mode(&mut self) -> MII_MODE_W {
        MII_MODE_W { w: self }
    }
    #[doc = "Bit 3 - Promiscuous Mode"]
    #[inline(always)]
    pub fn prom(&mut self) -> PROM_W {
        PROM_W { w: self }
    }
    #[doc = "Bit 4 - Broadcast Frame Reject"]
    #[inline(always)]
    pub fn bc_rej(&mut self) -> BC_REJ_W {
        BC_REJ_W { w: self }
    }
    #[doc = "Bit 5 - Flow Control Enable"]
    #[inline(always)]
    pub fn fce(&mut self) -> FCE_W {
        FCE_W { w: self }
    }
    #[doc = "Bit 8 - RMII Mode Enable"]
    #[inline(always)]
    pub fn rmii_mode(&mut self) -> RMII_MODE_W {
        RMII_MODE_W { w: self }
    }
    #[doc = "Bit 9 - Enables 10-Mbit/s mode of the RMII ."]
    #[inline(always)]
    pub fn rmii_10t(&mut self) -> RMII_10T_W {
        RMII_10T_W { w: self }
    }
    #[doc = "Bit 12 - Enable Frame Padding Remove On Receive"]
    #[inline(always)]
    pub fn paden(&mut self) -> PADEN_W {
        PADEN_W { w: self }
    }
    #[doc = "Bit 13 - Terminate/Forward Pause Frames"]
    #[inline(always)]
    pub fn paufwd(&mut self) -> PAUFWD_W {
        PAUFWD_W { w: self }
    }
    #[doc = "Bit 14 - Terminate/Forward Received CRC"]
    #[inline(always)]
    pub fn crcfwd(&mut self) -> CRCFWD_W {
        CRCFWD_W { w: self }
    }
    #[doc = "Bit 15 - MAC Control Frame Enable"]
    #[inline(always)]
    pub fn cfen(&mut self) -> CFEN_W {
        CFEN_W { w: self }
    }
    #[doc = "Bits 16:29 - Maximum Frame Length"]
    #[inline(always)]
    pub fn max_fl(&mut self) -> MAX_FL_W {
        MAX_FL_W { w: self }
    }
    #[doc = "Bit 30 - Payload Length Check Disable"]
    #[inline(always)]
    pub fn nlc(&mut self) -> NLC_W {
        NLC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr](index.html) module"]
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr::R](R) reader structure"]
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr::W](W) writer structure"]
impl crate::Writable for RCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCR to value 0x05ee_0001"]
impl crate::Resettable for RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05ee_0001
    }
}
