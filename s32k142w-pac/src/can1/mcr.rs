#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXMB` reader - Number Of The Last Message Buffer"]
pub struct MAXMB_R(crate::FieldReader<u8, u8>);
impl MAXMB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXMB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXMB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXMB` writer - Number Of The Last Message Buffer"]
pub struct MAXMB_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXMB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Format A: One full ID (standard and extended) per ID filter table element."]
    ONE_FULL_ID = 0,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
    TWO_FULL_ID = 1,
    #[doc = "2: Format C: Four partial 8-bit standard IDs per ID filter table element."]
    FOUR_PARTIAL_ID = 2,
    #[doc = "3: Format D: All frames rejected."]
    ALL_FRAMES_REJECTED = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDAM` reader - ID Acceptance Mode"]
pub struct IDAM_R(crate::FieldReader<u8, IDAM_A>);
impl IDAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::ONE_FULL_ID,
            1 => IDAM_A::TWO_FULL_ID,
            2 => IDAM_A::FOUR_PARTIAL_ID,
            3 => IDAM_A::ALL_FRAMES_REJECTED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE_FULL_ID`"]
    #[inline(always)]
    pub fn is_one_full_id(&self) -> bool {
        **self == IDAM_A::ONE_FULL_ID
    }
    #[doc = "Checks if the value of the field is `TWO_FULL_ID`"]
    #[inline(always)]
    pub fn is_two_full_id(&self) -> bool {
        **self == IDAM_A::TWO_FULL_ID
    }
    #[doc = "Checks if the value of the field is `FOUR_PARTIAL_ID`"]
    #[inline(always)]
    pub fn is_four_partial_id(&self) -> bool {
        **self == IDAM_A::FOUR_PARTIAL_ID
    }
    #[doc = "Checks if the value of the field is `ALL_FRAMES_REJECTED`"]
    #[inline(always)]
    pub fn is_all_frames_rejected(&self) -> bool {
        **self == IDAM_A::ALL_FRAMES_REJECTED
    }
}
impl core::ops::Deref for IDAM_R {
    type Target = crate::FieldReader<u8, IDAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDAM` writer - ID Acceptance Mode"]
pub struct IDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Format A: One full ID (standard and extended) per ID filter table element."]
    #[inline(always)]
    pub fn one_full_id(self) -> &'a mut W {
        self.variant(IDAM_A::ONE_FULL_ID)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
    #[inline(always)]
    pub fn two_full_id(self) -> &'a mut W {
        self.variant(IDAM_A::TWO_FULL_ID)
    }
    #[doc = "Format C: Four partial 8-bit standard IDs per ID filter table element."]
    #[inline(always)]
    pub fn four_partial_id(self) -> &'a mut W {
        self.variant(IDAM_A::FOUR_PARTIAL_ID)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn all_frames_rejected(self) -> &'a mut W {
        self.variant(IDAM_A::ALL_FRAMES_REJECTED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "CAN FD operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDEN_A {
    #[doc = "0: CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    CAN_FD_DISABLED = 0,
    #[doc = "1: CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    CAN_FD_ENABLED = 1,
}
impl From<FDEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDEN` reader - CAN FD operation enable"]
pub struct FDEN_R(crate::FieldReader<bool, FDEN_A>);
impl FDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDEN_A {
        match self.bits {
            false => FDEN_A::CAN_FD_DISABLED,
            true => FDEN_A::CAN_FD_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CAN_FD_DISABLED`"]
    #[inline(always)]
    pub fn is_can_fd_disabled(&self) -> bool {
        **self == FDEN_A::CAN_FD_DISABLED
    }
    #[doc = "Checks if the value of the field is `CAN_FD_ENABLED`"]
    #[inline(always)]
    pub fn is_can_fd_enabled(&self) -> bool {
        **self == FDEN_A::CAN_FD_ENABLED
    }
}
impl core::ops::Deref for FDEN_R {
    type Target = crate::FieldReader<bool, FDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDEN` writer - CAN FD operation enable"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    #[inline(always)]
    pub fn can_fd_disabled(self) -> &'a mut W {
        self.variant(FDEN_A::CAN_FD_DISABLED)
    }
    #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    #[inline(always)]
    pub fn can_fd_enabled(self) -> &'a mut W {
        self.variant(FDEN_A::CAN_FD_ENABLED)
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
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled."]
    ABORT_DISABLED = 0,
    #[doc = "1: Abort enabled."]
    ABORT_ENABLED = 1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AEN` reader - Abort Enable"]
pub struct AEN_R(crate::FieldReader<bool, AEN_A>);
impl AEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::ABORT_DISABLED,
            true => AEN_A::ABORT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ABORT_DISABLED`"]
    #[inline(always)]
    pub fn is_abort_disabled(&self) -> bool {
        **self == AEN_A::ABORT_DISABLED
    }
    #[doc = "Checks if the value of the field is `ABORT_ENABLED`"]
    #[inline(always)]
    pub fn is_abort_enabled(&self) -> bool {
        **self == AEN_A::ABORT_ENABLED
    }
}
impl core::ops::Deref for AEN_R {
    type Target = crate::FieldReader<bool, AEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AEN` writer - Abort Enable"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Abort disabled."]
    #[inline(always)]
    pub fn abort_disabled(self) -> &'a mut W {
        self.variant(AEN_A::ABORT_DISABLED)
    }
    #[doc = "Abort enabled."]
    #[inline(always)]
    pub fn abort_enabled(self) -> &'a mut W {
        self.variant(AEN_A::ABORT_ENABLED)
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
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled."]
    LOCAL_PRIORITY_DISABLED = 0,
    #[doc = "1: Local Priority enabled."]
    LOCAL_PRIORITY_ENABLED = 1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPRIOEN` reader - Local Priority Enable"]
pub struct LPRIOEN_R(crate::FieldReader<bool, LPRIOEN_A>);
impl LPRIOEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPRIOEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::LOCAL_PRIORITY_DISABLED,
            true => LPRIOEN_A::LOCAL_PRIORITY_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `LOCAL_PRIORITY_DISABLED`"]
    #[inline(always)]
    pub fn is_local_priority_disabled(&self) -> bool {
        **self == LPRIOEN_A::LOCAL_PRIORITY_DISABLED
    }
    #[doc = "Checks if the value of the field is `LOCAL_PRIORITY_ENABLED`"]
    #[inline(always)]
    pub fn is_local_priority_enabled(&self) -> bool {
        **self == LPRIOEN_A::LOCAL_PRIORITY_ENABLED
    }
}
impl core::ops::Deref for LPRIOEN_R {
    type Target = crate::FieldReader<bool, LPRIOEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRIOEN` writer - Local Priority Enable"]
pub struct LPRIOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRIOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPRIOEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Local Priority disabled."]
    #[inline(always)]
    pub fn local_priority_disabled(self) -> &'a mut W {
        self.variant(LPRIOEN_A::LOCAL_PRIORITY_DISABLED)
    }
    #[doc = "Local Priority enabled."]
    #[inline(always)]
    pub fn local_priority_enabled(self) -> &'a mut W {
        self.variant(LPRIOEN_A::LOCAL_PRIORITY_ENABLED)
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
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: DMA feature for RX FIFO disabled."]
    ID2 = 0,
    #[doc = "1: DMA feature for RX FIFO enabled."]
    ID4 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::ID2,
            true => DMA_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == DMA_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == DMA_A::ID4
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA feature for RX FIFO disabled."]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(DMA_A::ID2)
    }
    #[doc = "DMA feature for RX FIFO enabled."]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(DMA_A::ID4)
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
#[doc = "Individual Rx Masking And Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    INDIVIDUAL_RX_MASKING_DISABLED = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    INDIVIDUAL_RX_MASKING_ENABLED = 1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRMQ` reader - Individual Rx Masking And Queue Enable"]
pub struct IRMQ_R(crate::FieldReader<bool, IRMQ_A>);
impl IRMQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRMQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::INDIVIDUAL_RX_MASKING_DISABLED,
            true => IRMQ_A::INDIVIDUAL_RX_MASKING_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_RX_MASKING_DISABLED`"]
    #[inline(always)]
    pub fn is_individual_rx_masking_disabled(&self) -> bool {
        **self == IRMQ_A::INDIVIDUAL_RX_MASKING_DISABLED
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_RX_MASKING_ENABLED`"]
    #[inline(always)]
    pub fn is_individual_rx_masking_enabled(&self) -> bool {
        **self == IRMQ_A::INDIVIDUAL_RX_MASKING_ENABLED
    }
}
impl core::ops::Deref for IRMQ_R {
    type Target = crate::FieldReader<bool, IRMQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRMQ` writer - Individual Rx Masking And Queue Enable"]
pub struct IRMQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IRMQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRMQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn individual_rx_masking_disabled(self) -> &'a mut W {
        self.variant(IRMQ_A::INDIVIDUAL_RX_MASKING_DISABLED)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn individual_rx_masking_enabled(self) -> &'a mut W {
        self.variant(IRMQ_A::INDIVIDUAL_RX_MASKING_ENABLED)
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
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRXDIS_A {
    #[doc = "0: Self-reception enabled."]
    SELF_RECEPTION_ENABLED = 0,
    #[doc = "1: Self-reception disabled."]
    SELF_RECEPTION_DISABLED = 1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRXDIS` reader - Self Reception Disable"]
pub struct SRXDIS_R(crate::FieldReader<bool, SRXDIS_A>);
impl SRXDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRXDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::SELF_RECEPTION_ENABLED,
            true => SRXDIS_A::SELF_RECEPTION_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `SELF_RECEPTION_ENABLED`"]
    #[inline(always)]
    pub fn is_self_reception_enabled(&self) -> bool {
        **self == SRXDIS_A::SELF_RECEPTION_ENABLED
    }
    #[doc = "Checks if the value of the field is `SELF_RECEPTION_DISABLED`"]
    #[inline(always)]
    pub fn is_self_reception_disabled(&self) -> bool {
        **self == SRXDIS_A::SELF_RECEPTION_DISABLED
    }
}
impl core::ops::Deref for SRXDIS_R {
    type Target = crate::FieldReader<bool, SRXDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRXDIS` writer - Self Reception Disable"]
pub struct SRXDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRXDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Self-reception enabled."]
    #[inline(always)]
    pub fn self_reception_enabled(self) -> &'a mut W {
        self.variant(SRXDIS_A::SELF_RECEPTION_ENABLED)
    }
    #[doc = "Self-reception disabled."]
    #[inline(always)]
    pub fn self_reception_disabled(self) -> &'a mut W {
        self.variant(SRXDIS_A::SELF_RECEPTION_DISABLED)
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
#[doc = "Low-Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMACK_A {
    #[doc = "0: FlexCAN is not in a low-power mode."]
    LOW_POWER_NO = 0,
    #[doc = "1: FlexCAN is in a low-power mode."]
    LOW_POWER_YES = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMACK` reader - Low-Power Mode Acknowledge"]
pub struct LPMACK_R(crate::FieldReader<bool, LPMACK_A>);
impl LPMACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPMACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::LOW_POWER_NO,
            true => LPMACK_A::LOW_POWER_YES,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_NO`"]
    #[inline(always)]
    pub fn is_low_power_no(&self) -> bool {
        **self == LPMACK_A::LOW_POWER_NO
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_YES`"]
    #[inline(always)]
    pub fn is_low_power_yes(&self) -> bool {
        **self == LPMACK_A::LOW_POWER_YES
    }
}
impl core::ops::Deref for LPMACK_R {
    type Target = crate::FieldReader<bool, LPMACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRNEN_A {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    TWRNINT_RWRNINT_INACTIVE = 0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    TWRNINT_RWRNINT_ACTIVE = 1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRNEN` reader - Warning Interrupt Enable"]
pub struct WRNEN_R(crate::FieldReader<bool, WRNEN_A>);
impl WRNEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRNEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::TWRNINT_RWRNINT_INACTIVE,
            true => WRNEN_A::TWRNINT_RWRNINT_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `TWRNINT_RWRNINT_INACTIVE`"]
    #[inline(always)]
    pub fn is_twrnint_rwrnint_inactive(&self) -> bool {
        **self == WRNEN_A::TWRNINT_RWRNINT_INACTIVE
    }
    #[doc = "Checks if the value of the field is `TWRNINT_RWRNINT_ACTIVE`"]
    #[inline(always)]
    pub fn is_twrnint_rwrnint_active(&self) -> bool {
        **self == WRNEN_A::TWRNINT_RWRNINT_ACTIVE
    }
}
impl core::ops::Deref for WRNEN_R {
    type Target = crate::FieldReader<bool, WRNEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRNEN` writer - Warning Interrupt Enable"]
pub struct WRNEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRNEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRNEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn twrnint_rwrnint_inactive(self) -> &'a mut W {
        self.variant(WRNEN_A::TWRNINT_RWRNINT_INACTIVE)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn twrnint_rwrnint_active(self) -> &'a mut W {
        self.variant(WRNEN_A::TWRNINT_RWRNINT_ACTIVE)
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
#[doc = "Supervisor Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPV_A {
    #[doc = "0: FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    ID2 = 0,
    #[doc = "1: FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    ID4 = 1,
}
impl From<SUPV_A> for bool {
    #[inline(always)]
    fn from(variant: SUPV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUPV` reader - Supervisor Mode"]
pub struct SUPV_R(crate::FieldReader<bool, SUPV_A>);
impl SUPV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUPV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPV_A {
        match self.bits {
            false => SUPV_A::ID2,
            true => SUPV_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == SUPV_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == SUPV_A::ID4
    }
}
impl core::ops::Deref for SUPV_R {
    type Target = crate::FieldReader<bool, SUPV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPV` writer - Supervisor Mode"]
pub struct SUPV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUPV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(SUPV_A::ID2)
    }
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(SUPV_A::ID4)
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
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZACK_A {
    #[doc = "0: FlexCAN not in Freeze mode, prescaler running."]
    FREEZE_MODE_NO = 0,
    #[doc = "1: FlexCAN in Freeze mode, prescaler stopped."]
    FREEZE_MODE_YES = 1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZACK` reader - Freeze Mode Acknowledge"]
pub struct FRZACK_R(crate::FieldReader<bool, FRZACK_A>);
impl FRZACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::FREEZE_MODE_NO,
            true => FRZACK_A::FREEZE_MODE_YES,
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE_MODE_NO`"]
    #[inline(always)]
    pub fn is_freeze_mode_no(&self) -> bool {
        **self == FRZACK_A::FREEZE_MODE_NO
    }
    #[doc = "Checks if the value of the field is `FREEZE_MODE_YES`"]
    #[inline(always)]
    pub fn is_freeze_mode_yes(&self) -> bool {
        **self == FRZACK_A::FREEZE_MODE_YES
    }
}
impl core::ops::Deref for FRZACK_R {
    type Target = crate::FieldReader<bool, FRZACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request."]
    SOFTRST_NO_RESET_REQUEST = 0,
    #[doc = "1: Resets the registers affected by soft reset."]
    SOFTRST_RESET_REGISTERS = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRST` reader - Soft Reset"]
pub struct SOFTRST_R(crate::FieldReader<bool, SOFTRST_A>);
impl SOFTRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFTRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::SOFTRST_NO_RESET_REQUEST,
            true => SOFTRST_A::SOFTRST_RESET_REGISTERS,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTRST_NO_RESET_REQUEST`"]
    #[inline(always)]
    pub fn is_softrst_no_reset_request(&self) -> bool {
        **self == SOFTRST_A::SOFTRST_NO_RESET_REQUEST
    }
    #[doc = "Checks if the value of the field is `SOFTRST_RESET_REGISTERS`"]
    #[inline(always)]
    pub fn is_softrst_reset_registers(&self) -> bool {
        **self == SOFTRST_A::SOFTRST_RESET_REGISTERS
    }
}
impl core::ops::Deref for SOFTRST_R {
    type Target = crate::FieldReader<bool, SOFTRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTRST` writer - Soft Reset"]
pub struct SOFTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No reset request."]
    #[inline(always)]
    pub fn softrst_no_reset_request(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_NO_RESET_REQUEST)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn softrst_reset_registers(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_RESET_REGISTERS)
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
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRDY_A {
    #[doc = "0: FlexCAN module is either in Normal mode, Listen-Only mode, or Loop-Back mode."]
    ID1 = 0,
    #[doc = "1: FlexCAN module is either in Disable mode, Stop mode, or Freeze mode."]
    ID4 = 1,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOTRDY` reader - FlexCAN Not Ready"]
pub struct NOTRDY_R(crate::FieldReader<bool, NOTRDY_A>);
impl NOTRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOTRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRDY_A {
        match self.bits {
            false => NOTRDY_A::ID1,
            true => NOTRDY_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID1`"]
    #[inline(always)]
    pub fn is_id1(&self) -> bool {
        **self == NOTRDY_A::ID1
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == NOTRDY_A::ID4
    }
}
impl core::ops::Deref for NOTRDY_R {
    type Target = crate::FieldReader<bool, NOTRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: No Freeze mode request."]
    HALT_DISABLE = 0,
    #[doc = "1: Enters Freeze mode if the FRZ bit is asserted."]
    HALT_ENABLE = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT` reader - Halt FlexCAN"]
pub struct HALT_R(crate::FieldReader<bool, HALT_A>);
impl HALT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HALT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::HALT_DISABLE,
            true => HALT_A::HALT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_DISABLE`"]
    #[inline(always)]
    pub fn is_halt_disable(&self) -> bool {
        **self == HALT_A::HALT_DISABLE
    }
    #[doc = "Checks if the value of the field is `HALT_ENABLE`"]
    #[inline(always)]
    pub fn is_halt_enable(&self) -> bool {
        **self == HALT_A::HALT_ENABLE
    }
}
impl core::ops::Deref for HALT_R {
    type Target = crate::FieldReader<bool, HALT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT` writer - Halt FlexCAN"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Freeze mode request."]
    #[inline(always)]
    pub fn halt_disable(self) -> &'a mut W {
        self.variant(HALT_A::HALT_DISABLE)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn halt_enable(self) -> &'a mut W {
        self.variant(HALT_A::HALT_ENABLE)
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
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFEN_A {
    #[doc = "0: Rx FIFO not enabled."]
    ID2 = 0,
    #[doc = "1: Rx FIFO enabled."]
    ID4 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFEN` reader - Rx FIFO Enable"]
pub struct RFEN_R(crate::FieldReader<bool, RFEN_A>);
impl RFEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::ID2,
            true => RFEN_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == RFEN_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == RFEN_A::ID4
    }
}
impl core::ops::Deref for RFEN_R {
    type Target = crate::FieldReader<bool, RFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEN` writer - Rx FIFO Enable"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rx FIFO not enabled."]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(RFEN_A::ID2)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(RFEN_A::ID4)
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
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze mode."]
    FREEZE_MODE_DISABLED = 0,
    #[doc = "1: Enabled to enter Freeze mode."]
    FREEZE_MODE_ENABLED = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze Enable"]
pub struct FRZ_R(crate::FieldReader<bool, FRZ_A>);
impl FRZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::FREEZE_MODE_DISABLED,
            true => FRZ_A::FREEZE_MODE_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FREEZE_MODE_DISABLED`"]
    #[inline(always)]
    pub fn is_freeze_mode_disabled(&self) -> bool {
        **self == FRZ_A::FREEZE_MODE_DISABLED
    }
    #[doc = "Checks if the value of the field is `FREEZE_MODE_ENABLED`"]
    #[inline(always)]
    pub fn is_freeze_mode_enabled(&self) -> bool {
        **self == FRZ_A::FREEZE_MODE_ENABLED
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, FRZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - Freeze Enable"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn freeze_mode_disabled(self) -> &'a mut W {
        self.variant(FRZ_A::FREEZE_MODE_DISABLED)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn freeze_mode_enabled(self) -> &'a mut W {
        self.variant(FRZ_A::FREEZE_MODE_ENABLED)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FlexCAN module."]
    FLEXCAN_ENABLED = 0,
    #[doc = "1: Disable the FlexCAN module."]
    FLEXCAN_DISABLED = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub struct MDIS_R(crate::FieldReader<bool, MDIS_A>);
impl MDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::FLEXCAN_ENABLED,
            true => MDIS_A::FLEXCAN_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXCAN_ENABLED`"]
    #[inline(always)]
    pub fn is_flexcan_enabled(&self) -> bool {
        **self == MDIS_A::FLEXCAN_ENABLED
    }
    #[doc = "Checks if the value of the field is `FLEXCAN_DISABLED`"]
    #[inline(always)]
    pub fn is_flexcan_disabled(&self) -> bool {
        **self == MDIS_A::FLEXCAN_DISABLED
    }
}
impl core::ops::Deref for MDIS_R {
    type Target = crate::FieldReader<bool, MDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn flexcan_enabled(self) -> &'a mut W {
        self.variant(MDIS_A::FLEXCAN_ENABLED)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn flexcan_disabled(self) -> &'a mut W {
        self.variant(MDIS_A::FLEXCAN_DISABLED)
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
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&mut self) -> MAXMB_W {
        MAXMB_W { w: self }
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&mut self) -> IDAM_W {
        IDAM_W { w: self }
    }
    #[doc = "Bit 11 - CAN FD operation enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&mut self) -> LPRIOEN_W {
        LPRIOEN_W { w: self }
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&mut self) -> IRMQ_W {
        IRMQ_W { w: self }
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&mut self) -> SRXDIS_W {
        SRXDIS_W { w: self }
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&mut self) -> WRNEN_W {
        WRNEN_W { w: self }
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&mut self) -> SUPV_W {
        SUPV_W { w: self }
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&mut self) -> SOFTRST_W {
        SOFTRST_W { w: self }
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCR to value 0xd890_000f"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xd890_000f
    }
}
