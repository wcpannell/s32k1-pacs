#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Edge Filter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDFLTDIS_A {
    #[doc = "0: Edge filter is enabled"]
    ENABLE = 0,
    #[doc = "1: Edge filter is disabled"]
    DISABLE = 1,
}
impl From<EDFLTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: EDFLTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDFLTDIS` reader - Edge Filter Disable"]
pub struct EDFLTDIS_R(crate::FieldReader<bool, EDFLTDIS_A>);
impl EDFLTDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EDFLTDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDFLTDIS_A {
        match self.bits {
            false => EDFLTDIS_A::ENABLE,
            true => EDFLTDIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EDFLTDIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EDFLTDIS_A::DISABLE
    }
}
impl core::ops::Deref for EDFLTDIS_R {
    type Target = crate::FieldReader<bool, EDFLTDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDFLTDIS` writer - Edge Filter Disable"]
pub struct EDFLTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDFLTDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDFLTDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge filter is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EDFLTDIS_A::ENABLE)
    }
    #[doc = "Edge filter is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EDFLTDIS_A::DISABLE)
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
#[doc = "ISO CAN FD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOCANFDEN_A {
    #[doc = "0: FlexCAN operates using the non-ISO CAN FD protocol."]
    NON_ISO = 0,
    #[doc = "1: FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    ISO = 1,
}
impl From<ISOCANFDEN_A> for bool {
    #[inline(always)]
    fn from(variant: ISOCANFDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOCANFDEN` reader - ISO CAN FD Enable"]
pub struct ISOCANFDEN_R(crate::FieldReader<bool, ISOCANFDEN_A>);
impl ISOCANFDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOCANFDEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOCANFDEN_A {
        match self.bits {
            false => ISOCANFDEN_A::NON_ISO,
            true => ISOCANFDEN_A::ISO,
        }
    }
    #[doc = "Checks if the value of the field is `NON_ISO`"]
    #[inline(always)]
    pub fn is_non_iso(&self) -> bool {
        **self == ISOCANFDEN_A::NON_ISO
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        **self == ISOCANFDEN_A::ISO
    }
}
impl core::ops::Deref for ISOCANFDEN_R {
    type Target = crate::FieldReader<bool, ISOCANFDEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOCANFDEN` writer - ISO CAN FD Enable"]
pub struct ISOCANFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOCANFDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOCANFDEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    #[inline(always)]
    pub fn non_iso(self) -> &'a mut W {
        self.variant(ISOCANFDEN_A::NON_ISO)
    }
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(ISOCANFDEN_A::ISO)
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
#[doc = "Protocol Exception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREXCEN_A {
    #[doc = "0: Protocol exception is disabled."]
    DISABLE = 0,
    #[doc = "1: Protocol exception is enabled."]
    ENABLE = 1,
}
impl From<PREXCEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREXCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREXCEN` reader - Protocol Exception Enable"]
pub struct PREXCEN_R(crate::FieldReader<bool, PREXCEN_A>);
impl PREXCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PREXCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREXCEN_A {
        match self.bits {
            false => PREXCEN_A::DISABLE,
            true => PREXCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == PREXCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == PREXCEN_A::ENABLE
    }
}
impl core::ops::Deref for PREXCEN_R {
    type Target = crate::FieldReader<bool, PREXCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREXCEN` writer - Protocol Exception Enable"]
pub struct PREXCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREXCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREXCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Protocol exception is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PREXCEN_A::DISABLE)
    }
    #[doc = "Protocol exception is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PREXCEN_A::ENABLE)
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
#[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACEN_A {
    #[doc = "0: Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    RTR_COMPARE_NO = 0,
    #[doc = "1: Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    RTR_COMPARE_YES = 1,
}
impl From<EACEN_A> for bool {
    #[inline(always)]
    fn from(variant: EACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EACEN` reader - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub struct EACEN_R(crate::FieldReader<bool, EACEN_A>);
impl EACEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EACEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EACEN_A {
        match self.bits {
            false => EACEN_A::RTR_COMPARE_NO,
            true => EACEN_A::RTR_COMPARE_YES,
        }
    }
    #[doc = "Checks if the value of the field is `RTR_COMPARE_NO`"]
    #[inline(always)]
    pub fn is_rtr_compare_no(&self) -> bool {
        **self == EACEN_A::RTR_COMPARE_NO
    }
    #[doc = "Checks if the value of the field is `RTR_COMPARE_YES`"]
    #[inline(always)]
    pub fn is_rtr_compare_yes(&self) -> bool {
        **self == EACEN_A::RTR_COMPARE_YES
    }
}
impl core::ops::Deref for EACEN_R {
    type Target = crate::FieldReader<bool, EACEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EACEN` writer - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub struct EACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EACEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn rtr_compare_no(self) -> &'a mut W {
        self.variant(EACEN_A::RTR_COMPARE_NO)
    }
    #[doc = "Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn rtr_compare_yes(self) -> &'a mut W {
        self.variant(EACEN_A::RTR_COMPARE_YES)
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
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: Remote response frame is generated."]
    REMOTE_RESPONSE_FRAME_NOT_GENERATED = 0,
    #[doc = "1: Remote request frame is stored."]
    REMOTE_RESPONSE_FRAME_GENERATED = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - Remote Request Storing"]
pub struct RRS_R(crate::FieldReader<bool, RRS_A>);
impl RRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::REMOTE_RESPONSE_FRAME_NOT_GENERATED,
            true => RRS_A::REMOTE_RESPONSE_FRAME_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `REMOTE_RESPONSE_FRAME_NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_remote_response_frame_not_generated(&self) -> bool {
        **self == RRS_A::REMOTE_RESPONSE_FRAME_NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `REMOTE_RESPONSE_FRAME_GENERATED`"]
    #[inline(always)]
    pub fn is_remote_response_frame_generated(&self) -> bool {
        **self == RRS_A::REMOTE_RESPONSE_FRAME_GENERATED
    }
}
impl core::ops::Deref for RRS_R {
    type Target = crate::FieldReader<bool, RRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRS` writer - Remote Request Storing"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Remote response frame is generated."]
    #[inline(always)]
    pub fn remote_response_frame_not_generated(self) -> &'a mut W {
        self.variant(RRS_A::REMOTE_RESPONSE_FRAME_NOT_GENERATED)
    }
    #[doc = "Remote request frame is stored."]
    #[inline(always)]
    pub fn remote_response_frame_generated(self) -> &'a mut W {
        self.variant(RRS_A::REMOTE_RESPONSE_FRAME_GENERATED)
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
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
    ID2 = 0,
    #[doc = "1: Matching starts from mailboxes and continues on Rx FIFO."]
    ID4 = 1,
}
impl From<MRP_A> for bool {
    #[inline(always)]
    fn from(variant: MRP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRP` reader - Mailboxes Reception Priority"]
pub struct MRP_R(crate::FieldReader<bool, MRP_A>);
impl MRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRP_A {
        match self.bits {
            false => MRP_A::ID2,
            true => MRP_A::ID4,
        }
    }
    #[doc = "Checks if the value of the field is `ID2`"]
    #[inline(always)]
    pub fn is_id2(&self) -> bool {
        **self == MRP_A::ID2
    }
    #[doc = "Checks if the value of the field is `ID4`"]
    #[inline(always)]
    pub fn is_id4(&self) -> bool {
        **self == MRP_A::ID4
    }
}
impl core::ops::Deref for MRP_R {
    type Target = crate::FieldReader<bool, MRP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRP` writer - Mailboxes Reception Priority"]
pub struct MRP_W<'a> {
    w: &'a mut W,
}
impl<'a> MRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Matching starts from Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
    #[inline(always)]
    pub fn id2(self) -> &'a mut W {
        self.variant(MRP_A::ID2)
    }
    #[doc = "Matching starts from mailboxes and continues on Rx FIFO."]
    #[inline(always)]
    pub fn id4(self) -> &'a mut W {
        self.variant(MRP_A::ID4)
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
#[doc = "Field `TASD` reader - Tx Arbitration Start Delay"]
pub struct TASD_R(crate::FieldReader<u8, u8>);
impl TASD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TASD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TASD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TASD` writer - Tx Arbitration Start Delay"]
pub struct TASD_W<'a> {
    w: &'a mut W,
}
impl<'a> TASD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `RFFN` reader - Number Of Rx FIFO Filters"]
pub struct RFFN_R(crate::FieldReader<u8, u8>);
impl RFFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFFN` writer - Number Of Rx FIFO Filters"]
pub struct RFFN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Bus Off Done Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFDONEMSK_A {
    #[doc = "0: Bus off done interrupt disabled."]
    DISABLE = 0,
    #[doc = "1: Bus off done interrupt enabled."]
    ENABLE = 1,
}
impl From<BOFFDONEMSK_A> for bool {
    #[inline(always)]
    fn from(variant: BOFFDONEMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFDONEMSK` reader - Bus Off Done Interrupt Mask"]
pub struct BOFFDONEMSK_R(crate::FieldReader<bool, BOFFDONEMSK_A>);
impl BOFFDONEMSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BOFFDONEMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOFFDONEMSK_A {
        match self.bits {
            false => BOFFDONEMSK_A::DISABLE,
            true => BOFFDONEMSK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BOFFDONEMSK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BOFFDONEMSK_A::ENABLE
    }
}
impl core::ops::Deref for BOFFDONEMSK_R {
    type Target = crate::FieldReader<bool, BOFFDONEMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOFFDONEMSK` writer - Bus Off Done Interrupt Mask"]
pub struct BOFFDONEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFFDONEMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOFFDONEMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bus off done interrupt disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BOFFDONEMSK_A::DISABLE)
    }
    #[doc = "Bus off done interrupt enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BOFFDONEMSK_A::ENABLE)
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
#[doc = "Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSK_FAST_A {
    #[doc = "0: ERRINT_FAST error interrupt disabled."]
    DISABLE = 0,
    #[doc = "1: ERRINT_FAST error interrupt enabled."]
    ENABLE = 1,
}
impl From<ERRMSK_FAST_A> for bool {
    #[inline(always)]
    fn from(variant: ERRMSK_FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRMSK_FAST` reader - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
pub struct ERRMSK_FAST_R(crate::FieldReader<bool, ERRMSK_FAST_A>);
impl ERRMSK_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRMSK_FAST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRMSK_FAST_A {
        match self.bits {
            false => ERRMSK_FAST_A::DISABLE,
            true => ERRMSK_FAST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ERRMSK_FAST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ERRMSK_FAST_A::ENABLE
    }
}
impl core::ops::Deref for ERRMSK_FAST_R {
    type Target = crate::FieldReader<bool, ERRMSK_FAST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRMSK_FAST` writer - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
pub struct ERRMSK_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRMSK_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRMSK_FAST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ERRINT_FAST error interrupt disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ERRMSK_FAST_A::DISABLE)
    }
    #[doc = "ERRINT_FAST error interrupt enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ERRMSK_FAST_A::ENABLE)
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
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&self) -> EDFLTDIS_R {
        EDFLTDIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&self) -> ISOCANFDEN_R {
        ISOCANFDEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&self) -> PREXCEN_R {
        PREXCEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&self) -> BOFFDONEMSK_R {
        BOFFDONEMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&self) -> ERRMSK_FAST_R {
        ERRMSK_FAST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&mut self) -> EDFLTDIS_W {
        EDFLTDIS_W { w: self }
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&mut self) -> ISOCANFDEN_W {
        ISOCANFDEN_W { w: self }
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&mut self) -> PREXCEN_W {
        PREXCEN_W { w: self }
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&mut self) -> EACEN_W {
        EACEN_W { w: self }
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&mut self) -> MRP_W {
        MRP_W { w: self }
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&mut self) -> TASD_W {
        TASD_W { w: self }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&mut self) -> RFFN_W {
        RFFN_W { w: self }
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&mut self) -> BOFFDONEMSK_W {
        BOFFDONEMSK_W { w: self }
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&mut self) -> ERRMSK_FAST_W {
        ERRMSK_FAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0x0080_0000"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
