#[doc = "Register `ESR1` reader"]
pub struct R(crate::R<ESR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR1` writer"]
pub struct W(crate::W<ESR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR1_SPEC>;
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
impl From<crate::W<ESR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_A {
    #[doc = "0: No such occurrence."]
    DISABLE = 0,
    #[doc = "1: Indicates setting of any error bit in the Error and Status register."]
    ENABLE = 1,
}
impl From<ERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub struct ERRINT_R(crate::FieldReader<bool, ERRINT_A>);
impl ERRINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_A {
        match self.bits {
            false => ERRINT_A::DISABLE,
            true => ERRINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ERRINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ERRINT_A::ENABLE
    }
}
impl core::ops::Deref for ERRINT_R {
    type Target = crate::FieldReader<bool, ERRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub struct ERRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRINT_A::DISABLE)
    }
    #[doc = "Indicates setting of any error bit in the Error and Status register."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERRINT_A::ENABLE)
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
#[doc = "Bus Off Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFINT_A {
    #[doc = "0: No such occurrence."]
    DISABLE = 0,
    #[doc = "1: FlexCAN module entered Bus Off state."]
    ENABLE = 1,
}
impl From<BOFFINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFINT` reader - Bus Off Interrupt"]
pub struct BOFFINT_R(crate::FieldReader<bool, BOFFINT_A>);
impl BOFFINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFFINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFINT_A {
        match self.bits {
            false => BOFFINT_A::DISABLE,
            true => BOFFINT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BOFFINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BOFFINT_A::ENABLE
    }
}
impl core::ops::Deref for BOFFINT_R {
    type Target = crate::FieldReader<bool, BOFFINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFFINT` writer - Bus Off Interrupt"]
pub struct BOFFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BOFFINT_A::DISABLE)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BOFFINT_A::ENABLE)
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
#[doc = "FlexCAN In Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: FlexCAN is not receiving a message."]
    DISABLE = 0,
    #[doc = "1: FlexCAN is receiving a message."]
    ENABLE = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - FlexCAN In Reception"]
pub struct RX_R(crate::FieldReader<bool, RX_A>);
impl RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::DISABLE,
            true => RX_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_A::ENABLE
    }
}
impl core::ops::Deref for RX_R {
    type Target = crate::FieldReader<bool, RX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fault Confinement State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLTCONF_A {
    #[doc = "0: Error Active"]
    ERROR_ACTIVE = 0,
    #[doc = "1: Error Passive"]
    ERROR_PASSIVE = 1,
    #[doc = "2: Bus Off"]
    BUS_OFF = 2,
}
impl From<FLTCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: FLTCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLTCONF` reader - Fault Confinement State"]
pub struct FLTCONF_R(crate::FieldReader<u8, FLTCONF_A>);
impl FLTCONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLTCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLTCONF_A> {
        match self.bits {
            0 => Some(FLTCONF_A::ERROR_ACTIVE),
            1 => Some(FLTCONF_A::ERROR_PASSIVE),
            2 => Some(FLTCONF_A::BUS_OFF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERROR_ACTIVE`"]
    #[inline(always)]
    pub fn is_error_active(&self) -> bool {
        **self == FLTCONF_A::ERROR_ACTIVE
    }
    #[doc = "Checks if the value of the field is `ERROR_PASSIVE`"]
    #[inline(always)]
    pub fn is_error_passive(&self) -> bool {
        **self == FLTCONF_A::ERROR_PASSIVE
    }
    #[doc = "Checks if the value of the field is `BUS_OFF`"]
    #[inline(always)]
    pub fn is_bus_off(&self) -> bool {
        **self == FLTCONF_A::BUS_OFF
    }
}
impl core::ops::Deref for FLTCONF_R {
    type Target = crate::FieldReader<u8, FLTCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "FlexCAN In Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: FlexCAN is not transmitting a message."]
    TRANSMIT_MESSAGE_NO = 0,
    #[doc = "1: FlexCAN is transmitting a message."]
    TRANSMIT_MESSAGE_YES = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - FlexCAN In Transmission"]
pub struct TX_R(crate::FieldReader<bool, TX_A>);
impl TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::TRANSMIT_MESSAGE_NO,
            true => TX_A::TRANSMIT_MESSAGE_YES,
        }
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_MESSAGE_NO`"]
    #[inline(always)]
    pub fn is_transmit_message_no(&self) -> bool {
        **self == TX_A::TRANSMIT_MESSAGE_NO
    }
    #[doc = "Checks if the value of the field is `TRANSMIT_MESSAGE_YES`"]
    #[inline(always)]
    pub fn is_transmit_message_yes(&self) -> bool {
        **self == TX_A::TRANSMIT_MESSAGE_YES
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<bool, TX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: No such occurrence."]
    CAN_BUS_NOT_IDLE = 0,
    #[doc = "1: CAN bus is now IDLE."]
    CAN_BUS_IDLE = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - IDLE"]
pub struct IDLE_R(crate::FieldReader<bool, IDLE_A>);
impl IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::CAN_BUS_NOT_IDLE,
            true => IDLE_A::CAN_BUS_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `CAN_BUS_NOT_IDLE`"]
    #[inline(always)]
    pub fn is_can_bus_not_idle(&self) -> bool {
        **self == IDLE_A::CAN_BUS_NOT_IDLE
    }
    #[doc = "Checks if the value of the field is `CAN_BUS_IDLE`"]
    #[inline(always)]
    pub fn is_can_bus_idle(&self) -> bool {
        **self == IDLE_A::CAN_BUS_IDLE
    }
}
impl core::ops::Deref for IDLE_R {
    type Target = crate::FieldReader<bool, IDLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Rx Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXWRN_A {
    #[doc = "0: No such occurrence."]
    RXERRCNT_LT_96 = 0,
    #[doc = "1: RXERRCNT is greater than or equal to 96."]
    RXERRCNT_GTE_96 = 1,
}
impl From<RXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: RXWRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXWRN` reader - Rx Error Warning"]
pub struct RXWRN_R(crate::FieldReader<bool, RXWRN_A>);
impl RXWRN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXWRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXWRN_A {
        match self.bits {
            false => RXWRN_A::RXERRCNT_LT_96,
            true => RXWRN_A::RXERRCNT_GTE_96,
        }
    }
    #[doc = "Checks if the value of the field is `RXERRCNT_LT_96`"]
    #[inline(always)]
    pub fn is_rxerrcnt_lt_96(&self) -> bool {
        **self == RXWRN_A::RXERRCNT_LT_96
    }
    #[doc = "Checks if the value of the field is `RXERRCNT_GTE_96`"]
    #[inline(always)]
    pub fn is_rxerrcnt_gte_96(&self) -> bool {
        **self == RXWRN_A::RXERRCNT_GTE_96
    }
}
impl core::ops::Deref for RXWRN_R {
    type Target = crate::FieldReader<bool, RXWRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TX Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXWRN_A {
    #[doc = "0: No such occurrence."]
    TXERRCNT_LT_96 = 0,
    #[doc = "1: TXERRCNT is greater than or equal to 96."]
    TXERRCNT_GTE_96 = 1,
}
impl From<TXWRN_A> for bool {
    #[inline(always)]
    fn from(variant: TXWRN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXWRN` reader - TX Error Warning"]
pub struct TXWRN_R(crate::FieldReader<bool, TXWRN_A>);
impl TXWRN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXWRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXWRN_A {
        match self.bits {
            false => TXWRN_A::TXERRCNT_LT_96,
            true => TXWRN_A::TXERRCNT_GTE_96,
        }
    }
    #[doc = "Checks if the value of the field is `TXERRCNT_LT_96`"]
    #[inline(always)]
    pub fn is_txerrcnt_lt_96(&self) -> bool {
        **self == TXWRN_A::TXERRCNT_LT_96
    }
    #[doc = "Checks if the value of the field is `TXERRCNT_GTE_96`"]
    #[inline(always)]
    pub fn is_txerrcnt_gte_96(&self) -> bool {
        **self == TXWRN_A::TXERRCNT_GTE_96
    }
}
impl core::ops::Deref for TXWRN_R {
    type Target = crate::FieldReader<bool, TXWRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stuffing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERR_A {
    #[doc = "0: No such occurrence."]
    STUFFING_ERROR_NO = 0,
    #[doc = "1: A stuffing error occurred since last read of this register."]
    STUFFING_ERROR_YES = 1,
}
impl From<STFERR_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STFERR` reader - Stuffing Error"]
pub struct STFERR_R(crate::FieldReader<bool, STFERR_A>);
impl STFERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STFERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_A {
        match self.bits {
            false => STFERR_A::STUFFING_ERROR_NO,
            true => STFERR_A::STUFFING_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `STUFFING_ERROR_NO`"]
    #[inline(always)]
    pub fn is_stuffing_error_no(&self) -> bool {
        **self == STFERR_A::STUFFING_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `STUFFING_ERROR_YES`"]
    #[inline(always)]
    pub fn is_stuffing_error_yes(&self) -> bool {
        **self == STFERR_A::STUFFING_ERROR_YES
    }
}
impl core::ops::Deref for STFERR_R {
    type Target = crate::FieldReader<bool, STFERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_A {
    #[doc = "0: No such occurrence."]
    FORM_ERROR_NO = 0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    FORM_ERROR_YES = 1,
}
impl From<FRMERR_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - Form Error"]
pub struct FRMERR_R(crate::FieldReader<bool, FRMERR_A>);
impl FRMERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRMERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_A {
        match self.bits {
            false => FRMERR_A::FORM_ERROR_NO,
            true => FRMERR_A::FORM_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR_NO`"]
    #[inline(always)]
    pub fn is_form_error_no(&self) -> bool {
        **self == FRMERR_A::FORM_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR_YES`"]
    #[inline(always)]
    pub fn is_form_error_yes(&self) -> bool {
        **self == FRMERR_A::FORM_ERROR_YES
    }
}
impl core::ops::Deref for FRMERR_R {
    type Target = crate::FieldReader<bool, FRMERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Cyclic Redundancy Check Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_A {
    #[doc = "0: No such occurrence."]
    CRC_ERROR_NO = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CRC_ERROR_YES = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - Cyclic Redundancy Check Error"]
pub struct CRCERR_R(crate::FieldReader<bool, CRCERR_A>);
impl CRCERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::CRC_ERROR_NO,
            true => CRCERR_A::CRC_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR_NO`"]
    #[inline(always)]
    pub fn is_crc_error_no(&self) -> bool {
        **self == CRCERR_A::CRC_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR_YES`"]
    #[inline(always)]
    pub fn is_crc_error_yes(&self) -> bool {
        **self == CRCERR_A::CRC_ERROR_YES
    }
}
impl core::ops::Deref for CRCERR_R {
    type Target = crate::FieldReader<bool, CRCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKERR_A {
    #[doc = "0: No such occurrence."]
    ACK_ERROR_NO = 0,
    #[doc = "1: An ACK error occurred since last read of this register."]
    ACK_ERROR_YES = 1,
}
impl From<ACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACKERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKERR` reader - Acknowledge Error"]
pub struct ACKERR_R(crate::FieldReader<bool, ACKERR_A>);
impl ACKERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACKERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKERR_A {
        match self.bits {
            false => ACKERR_A::ACK_ERROR_NO,
            true => ACKERR_A::ACK_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `ACK_ERROR_NO`"]
    #[inline(always)]
    pub fn is_ack_error_no(&self) -> bool {
        **self == ACKERR_A::ACK_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `ACK_ERROR_YES`"]
    #[inline(always)]
    pub fn is_ack_error_yes(&self) -> bool {
        **self == ACKERR_A::ACK_ERROR_YES
    }
}
impl core::ops::Deref for ACKERR_R {
    type Target = crate::FieldReader<bool, ACKERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bit0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERR_A {
    #[doc = "0: No such occurrence."]
    BIT0_ERROR_NO = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    BIT0_ERROR_YES = 1,
}
impl From<BIT0ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT0ERR` reader - Bit0 Error"]
pub struct BIT0ERR_R(crate::FieldReader<bool, BIT0ERR_A>);
impl BIT0ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT0ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_A {
        match self.bits {
            false => BIT0ERR_A::BIT0_ERROR_NO,
            true => BIT0ERR_A::BIT0_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR_NO`"]
    #[inline(always)]
    pub fn is_bit0_error_no(&self) -> bool {
        **self == BIT0ERR_A::BIT0_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR_YES`"]
    #[inline(always)]
    pub fn is_bit0_error_yes(&self) -> bool {
        **self == BIT0ERR_A::BIT0_ERROR_YES
    }
}
impl core::ops::Deref for BIT0ERR_R {
    type Target = crate::FieldReader<bool, BIT0ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bit1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERR_A {
    #[doc = "0: No such occurrence."]
    BIT1_ERROR_NO = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    BIT1_ERROR_YES = 1,
}
impl From<BIT1ERR_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT1ERR` reader - Bit1 Error"]
pub struct BIT1ERR_R(crate::FieldReader<bool, BIT1ERR_A>);
impl BIT1ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT1ERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_A {
        match self.bits {
            false => BIT1ERR_A::BIT1_ERROR_NO,
            true => BIT1ERR_A::BIT1_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR_NO`"]
    #[inline(always)]
    pub fn is_bit1_error_no(&self) -> bool {
        **self == BIT1ERR_A::BIT1_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR_YES`"]
    #[inline(always)]
    pub fn is_bit1_error_yes(&self) -> bool {
        **self == BIT1ERR_A::BIT1_ERROR_YES
    }
}
impl core::ops::Deref for BIT1ERR_R {
    type Target = crate::FieldReader<bool, BIT1ERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Rx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNINT_A {
    #[doc = "0: No such occurrence."]
    RX_WARNING_INT_NO = 0,
    #[doc = "1: The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    RX_WARNING_INT_YES = 1,
}
impl From<RWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: RWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWRNINT` reader - Rx Warning Interrupt Flag"]
pub struct RWRNINT_R(crate::FieldReader<bool, RWRNINT_A>);
impl RWRNINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWRNINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWRNINT_A {
        match self.bits {
            false => RWRNINT_A::RX_WARNING_INT_NO,
            true => RWRNINT_A::RX_WARNING_INT_YES,
        }
    }
    #[doc = "Checks if the value of the field is `RX_WARNING_INT_NO`"]
    #[inline(always)]
    pub fn is_rx_warning_int_no(&self) -> bool {
        **self == RWRNINT_A::RX_WARNING_INT_NO
    }
    #[doc = "Checks if the value of the field is `RX_WARNING_INT_YES`"]
    #[inline(always)]
    pub fn is_rx_warning_int_yes(&self) -> bool {
        **self == RWRNINT_A::RX_WARNING_INT_YES
    }
}
impl core::ops::Deref for RWRNINT_R {
    type Target = crate::FieldReader<bool, RWRNINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWRNINT` writer - Rx Warning Interrupt Flag"]
pub struct RWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWRNINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn rx_warning_int_no(self) -> &'a mut W {
        self.variant(RWRNINT_A::RX_WARNING_INT_NO)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn rx_warning_int_yes(self) -> &'a mut W {
        self.variant(RWRNINT_A::RX_WARNING_INT_YES)
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
#[doc = "Tx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNINT_A {
    #[doc = "0: No such occurrence."]
    TX_WARNING_INT_NO = 0,
    #[doc = "1: The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    TX_WARNING_INT_YES = 1,
}
impl From<TWRNINT_A> for bool {
    #[inline(always)]
    fn from(variant: TWRNINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWRNINT` reader - Tx Warning Interrupt Flag"]
pub struct TWRNINT_R(crate::FieldReader<bool, TWRNINT_A>);
impl TWRNINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TWRNINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TWRNINT_A {
        match self.bits {
            false => TWRNINT_A::TX_WARNING_INT_NO,
            true => TWRNINT_A::TX_WARNING_INT_YES,
        }
    }
    #[doc = "Checks if the value of the field is `TX_WARNING_INT_NO`"]
    #[inline(always)]
    pub fn is_tx_warning_int_no(&self) -> bool {
        **self == TWRNINT_A::TX_WARNING_INT_NO
    }
    #[doc = "Checks if the value of the field is `TX_WARNING_INT_YES`"]
    #[inline(always)]
    pub fn is_tx_warning_int_yes(&self) -> bool {
        **self == TWRNINT_A::TX_WARNING_INT_YES
    }
}
impl core::ops::Deref for TWRNINT_R {
    type Target = crate::FieldReader<bool, TWRNINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWRNINT` writer - Tx Warning Interrupt Flag"]
pub struct TWRNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TWRNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TWRNINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn tx_warning_int_no(self) -> &'a mut W {
        self.variant(TWRNINT_A::TX_WARNING_INT_NO)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn tx_warning_int_yes(self) -> &'a mut W {
        self.variant(TWRNINT_A::TX_WARNING_INT_YES)
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
#[doc = "CAN Synchronization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCH_A {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus."]
    CAN_BUS_SYNC_NO = 0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus."]
    CAN_BUS_SYNC_YES = 1,
}
impl From<SYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCH` reader - CAN Synchronization Status"]
pub struct SYNCH_R(crate::FieldReader<bool, SYNCH_A>);
impl SYNCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCH_A {
        match self.bits {
            false => SYNCH_A::CAN_BUS_SYNC_NO,
            true => SYNCH_A::CAN_BUS_SYNC_YES,
        }
    }
    #[doc = "Checks if the value of the field is `CAN_BUS_SYNC_NO`"]
    #[inline(always)]
    pub fn is_can_bus_sync_no(&self) -> bool {
        **self == SYNCH_A::CAN_BUS_SYNC_NO
    }
    #[doc = "Checks if the value of the field is `CAN_BUS_SYNC_YES`"]
    #[inline(always)]
    pub fn is_can_bus_sync_yes(&self) -> bool {
        **self == SYNCH_A::CAN_BUS_SYNC_YES
    }
}
impl core::ops::Deref for SYNCH_R {
    type Target = crate::FieldReader<bool, SYNCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bus Off Done Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFDONEINT_A {
    #[doc = "0: No such occurrence."]
    BUS_OFF_NOT_DONE = 0,
    #[doc = "1: FlexCAN module has completed Bus Off process."]
    BUS_OFF_DONE = 1,
}
impl From<BOFFDONEINT_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFDONEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFDONEINT` reader - Bus Off Done Interrupt"]
pub struct BOFFDONEINT_R(crate::FieldReader<bool, BOFFDONEINT_A>);
impl BOFFDONEINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFFDONEINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFDONEINT_A {
        match self.bits {
            false => BOFFDONEINT_A::BUS_OFF_NOT_DONE,
            true => BOFFDONEINT_A::BUS_OFF_DONE,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_NOT_DONE`"]
    #[inline(always)]
    pub fn is_bus_off_not_done(&self) -> bool {
        **self == BOFFDONEINT_A::BUS_OFF_NOT_DONE
    }
    #[doc = "Checks if the value of the field is `BUS_OFF_DONE`"]
    #[inline(always)]
    pub fn is_bus_off_done(&self) -> bool {
        **self == BOFFDONEINT_A::BUS_OFF_DONE
    }
}
impl core::ops::Deref for BOFFDONEINT_R {
    type Target = crate::FieldReader<bool, BOFFDONEINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFFDONEINT` writer - Bus Off Done Interrupt"]
pub struct BOFFDONEINT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFDONEINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFDONEINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn bus_off_not_done(self) -> &'a mut W {
        self.variant(BOFFDONEINT_A::BUS_OFF_NOT_DONE)
    }
    #[doc = "FlexCAN module has completed Bus Off process."]
    #[inline(always)]
    pub fn bus_off_done(self) -> &'a mut W {
        self.variant(BOFFDONEINT_A::BUS_OFF_DONE)
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
#[doc = "Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRINT_FAST_A {
    #[doc = "0: No such occurrence."]
    ERRORS_DATA_PHASE_NO = 0,
    #[doc = "1: Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    ERRORS_DATA_PHASE_YES = 1,
}
impl From<ERRINT_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: ERRINT_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT_FAST` reader - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
pub struct ERRINT_FAST_R(crate::FieldReader<bool, ERRINT_FAST_A>);
impl ERRINT_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRINT_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRINT_FAST_A {
        match self.bits {
            false => ERRINT_FAST_A::ERRORS_DATA_PHASE_NO,
            true => ERRINT_FAST_A::ERRORS_DATA_PHASE_YES,
        }
    }
    #[doc = "Checks if the value of the field is `ERRORS_DATA_PHASE_NO`"]
    #[inline(always)]
    pub fn is_errors_data_phase_no(&self) -> bool {
        **self == ERRINT_FAST_A::ERRORS_DATA_PHASE_NO
    }
    #[doc = "Checks if the value of the field is `ERRORS_DATA_PHASE_YES`"]
    #[inline(always)]
    pub fn is_errors_data_phase_yes(&self) -> bool {
        **self == ERRINT_FAST_A::ERRORS_DATA_PHASE_YES
    }
}
impl core::ops::Deref for ERRINT_FAST_R {
    type Target = crate::FieldReader<bool, ERRINT_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRINT_FAST` writer - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
pub struct ERRINT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRINT_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRINT_FAST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn errors_data_phase_no(self) -> &'a mut W {
        self.variant(ERRINT_FAST_A::ERRORS_DATA_PHASE_NO)
    }
    #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    #[inline(always)]
    pub fn errors_data_phase_yes(self) -> &'a mut W {
        self.variant(ERRINT_FAST_A::ERRORS_DATA_PHASE_YES)
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
#[doc = "Error Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROVR_A {
    #[doc = "0: Overrun has not occurred."]
    OVERRUN_NOT_OCCURRED = 0,
    #[doc = "1: Overrun has occurred."]
    OVERRUN_OCCURRED = 1,
}
impl From<ERROVR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROVR` reader - Error Overrun"]
pub struct ERROVR_R(crate::FieldReader<bool, ERROVR_A>);
impl ERROVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERROVR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROVR_A {
        match self.bits {
            false => ERROVR_A::OVERRUN_NOT_OCCURRED,
            true => ERROVR_A::OVERRUN_OCCURRED,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRUN_NOT_OCCURRED`"]
    #[inline(always)]
    pub fn is_overrun_not_occurred(&self) -> bool {
        **self == ERROVR_A::OVERRUN_NOT_OCCURRED
    }
    #[doc = "Checks if the value of the field is `OVERRUN_OCCURRED`"]
    #[inline(always)]
    pub fn is_overrun_occurred(&self) -> bool {
        **self == ERROVR_A::OVERRUN_OCCURRED
    }
}
impl core::ops::Deref for ERROVR_R {
    type Target = crate::FieldReader<bool, ERROVR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROVR` writer - Error Overrun"]
pub struct ERROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROVR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Overrun has not occurred."]
    #[inline(always)]
    pub fn overrun_not_occurred(self) -> &'a mut W {
        self.variant(ERROVR_A::OVERRUN_NOT_OCCURRED)
    }
    #[doc = "Overrun has occurred."]
    #[inline(always)]
    pub fn overrun_occurred(self) -> &'a mut W {
        self.variant(ERROVR_A::OVERRUN_OCCURRED)
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
#[doc = "Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STFERR_FAST_A {
    #[doc = "0: No such occurrence."]
    STUFFING_ERROR_NO = 0,
    #[doc = "1: A stuffing error occurred since last read of this register."]
    STUFFING_ERROR_YES = 1,
}
impl From<STFERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: STFERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STFERR_FAST` reader - Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub struct STFERR_FAST_R(crate::FieldReader<bool, STFERR_FAST_A>);
impl STFERR_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STFERR_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STFERR_FAST_A {
        match self.bits {
            false => STFERR_FAST_A::STUFFING_ERROR_NO,
            true => STFERR_FAST_A::STUFFING_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `STUFFING_ERROR_NO`"]
    #[inline(always)]
    pub fn is_stuffing_error_no(&self) -> bool {
        **self == STFERR_FAST_A::STUFFING_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `STUFFING_ERROR_YES`"]
    #[inline(always)]
    pub fn is_stuffing_error_yes(&self) -> bool {
        **self == STFERR_FAST_A::STUFFING_ERROR_YES
    }
}
impl core::ops::Deref for STFERR_FAST_R {
    type Target = crate::FieldReader<bool, STFERR_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Form Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRMERR_FAST_A {
    #[doc = "0: No such occurrence."]
    FORM_ERROR_NO = 0,
    #[doc = "1: A form error occurred since last read of this register."]
    FORM_ERROR_YES = 1,
}
impl From<FRMERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FRMERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR_FAST` reader - Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub struct FRMERR_FAST_R(crate::FieldReader<bool, FRMERR_FAST_A>);
impl FRMERR_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRMERR_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRMERR_FAST_A {
        match self.bits {
            false => FRMERR_FAST_A::FORM_ERROR_NO,
            true => FRMERR_FAST_A::FORM_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR_NO`"]
    #[inline(always)]
    pub fn is_form_error_no(&self) -> bool {
        **self == FRMERR_FAST_A::FORM_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `FORM_ERROR_YES`"]
    #[inline(always)]
    pub fn is_form_error_yes(&self) -> bool {
        **self == FRMERR_FAST_A::FORM_ERROR_YES
    }
}
impl core::ops::Deref for FRMERR_FAST_R {
    type Target = crate::FieldReader<bool, FRMERR_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERR_FAST_A {
    #[doc = "0: No such occurrence."]
    CRC_ERROR_NO = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CRC_ERROR_YES = 1,
}
impl From<CRCERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR_FAST` reader - Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
pub struct CRCERR_FAST_R(crate::FieldReader<bool, CRCERR_FAST_A>);
impl CRCERR_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_FAST_A {
        match self.bits {
            false => CRCERR_FAST_A::CRC_ERROR_NO,
            true => CRCERR_FAST_A::CRC_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR_NO`"]
    #[inline(always)]
    pub fn is_crc_error_no(&self) -> bool {
        **self == CRCERR_FAST_A::CRC_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `CRC_ERROR_YES`"]
    #[inline(always)]
    pub fn is_crc_error_yes(&self) -> bool {
        **self == CRCERR_FAST_A::CRC_ERROR_YES
    }
}
impl core::ops::Deref for CRCERR_FAST_R {
    type Target = crate::FieldReader<bool, CRCERR_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT0ERR_FAST_A {
    #[doc = "0: No such occurrence."]
    BIT0_ERROR_NO = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    BIT0_ERROR_YES = 1,
}
impl From<BIT0ERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: BIT0ERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT0ERR_FAST` reader - Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub struct BIT0ERR_FAST_R(crate::FieldReader<bool, BIT0ERR_FAST_A>);
impl BIT0ERR_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT0ERR_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT0ERR_FAST_A {
        match self.bits {
            false => BIT0ERR_FAST_A::BIT0_ERROR_NO,
            true => BIT0ERR_FAST_A::BIT0_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR_NO`"]
    #[inline(always)]
    pub fn is_bit0_error_no(&self) -> bool {
        **self == BIT0ERR_FAST_A::BIT0_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `BIT0_ERROR_YES`"]
    #[inline(always)]
    pub fn is_bit0_error_yes(&self) -> bool {
        **self == BIT0ERR_FAST_A::BIT0_ERROR_YES
    }
}
impl core::ops::Deref for BIT0ERR_FAST_R {
    type Target = crate::FieldReader<bool, BIT0ERR_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT1ERR_FAST_A {
    #[doc = "0: No such occurrence."]
    BIT1_ERROR_NO = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    BIT1_ERROR_YES = 1,
}
impl From<BIT1ERR_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: BIT1ERR_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT1ERR_FAST` reader - Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub struct BIT1ERR_FAST_R(crate::FieldReader<bool, BIT1ERR_FAST_A>);
impl BIT1ERR_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT1ERR_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT1ERR_FAST_A {
        match self.bits {
            false => BIT1ERR_FAST_A::BIT1_ERROR_NO,
            true => BIT1ERR_FAST_A::BIT1_ERROR_YES,
        }
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR_NO`"]
    #[inline(always)]
    pub fn is_bit1_error_no(&self) -> bool {
        **self == BIT1ERR_FAST_A::BIT1_ERROR_NO
    }
    #[doc = "Checks if the value of the field is `BIT1_ERROR_YES`"]
    #[inline(always)]
    pub fn is_bit1_error_yes(&self) -> bool {
        **self == BIT1ERR_FAST_A::BIT1_ERROR_YES
    }
}
impl core::ops::Deref for BIT1ERR_FAST_R {
    type Target = crate::FieldReader<bool, BIT1ERR_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ERRINT_R {
        ERRINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&self) -> BOFFINT_R {
        BOFFINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline(always)]
    pub fn fltconf(&self) -> FLTCONF_R {
        FLTCONF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline(always)]
    pub fn rxwrn(&self) -> RXWRN_R {
        RXWRN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline(always)]
    pub fn txwrn(&self) -> TXWRN_R {
        TXWRN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline(always)]
    pub fn stferr(&self) -> STFERR_R {
        STFERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline(always)]
    pub fn ackerr(&self) -> ACKERR_R {
        ACKERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline(always)]
    pub fn bit0err(&self) -> BIT0ERR_R {
        BIT0ERR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline(always)]
    pub fn bit1err(&self) -> BIT1ERR_R {
        BIT1ERR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RWRNINT_R {
        RWRNINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&self) -> TWRNINT_R {
        TWRNINT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline(always)]
    pub fn synch(&self) -> SYNCH_R {
        SYNCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&self) -> BOFFDONEINT_R {
        BOFFDONEINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&self) -> ERRINT_FAST_R {
        ERRINT_FAST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Error Overrun"]
    #[inline(always)]
    pub fn errovr(&self) -> ERROVR_R {
        ERROVR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn stferr_fast(&self) -> STFERR_FAST_R {
        STFERR_FAST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn frmerr_fast(&self) -> FRMERR_FAST_R {
        FRMERR_FAST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn crcerr_fast(&self) -> CRCERR_FAST_R {
        CRCERR_FAST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit0err_fast(&self) -> BIT0ERR_FAST_R {
        BIT0ERR_FAST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit1err_fast(&self) -> BIT1ERR_FAST_R {
        BIT1ERR_FAST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ERRINT_W {
        ERRINT_W { w: self }
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&mut self) -> BOFFINT_W {
        BOFFINT_W { w: self }
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&mut self) -> RWRNINT_W {
        RWRNINT_W { w: self }
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&mut self) -> TWRNINT_W {
        TWRNINT_W { w: self }
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&mut self) -> BOFFDONEINT_W {
        BOFFDONEINT_W { w: self }
    }
    #[doc = "Bit 20 - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&mut self) -> ERRINT_FAST_W {
        ERRINT_FAST_W { w: self }
    }
    #[doc = "Bit 21 - Error Overrun"]
    #[inline(always)]
    pub fn errovr(&mut self) -> ERROVR_W {
        ERROVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error and Status 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr1](index.html) module"]
pub struct ESR1_SPEC;
impl crate::RegisterSpec for ESR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr1::R](R) reader structure"]
impl crate::Readable for ESR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr1::W](W) writer structure"]
impl crate::Writable for ESR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESR1 to value 0"]
impl crate::Resettable for ESR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
