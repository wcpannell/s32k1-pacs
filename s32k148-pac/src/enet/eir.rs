#[doc = "Register `EIR` reader"]
pub struct R(crate::R<EIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIR` writer"]
pub struct W(crate::W<EIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIR_SPEC>;
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
impl From<crate::W<EIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_TIMER` reader - Timestamp Timer"]
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
#[doc = "Field `TS_TIMER` writer - Timestamp Timer"]
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
#[doc = "Field `TS_AVAIL` reader - Transmit Timestamp Available"]
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
#[doc = "Field `TS_AVAIL` writer - Transmit Timestamp Available"]
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
#[doc = "Field `WAKEUP` reader - Node Wakeup Request Indication"]
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
#[doc = "Field `WAKEUP` writer - Node Wakeup Request Indication"]
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
#[doc = "Field `PLR` reader - Payload Receive Error"]
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
#[doc = "Field `PLR` writer - Payload Receive Error"]
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
#[doc = "Field `UN` reader - Transmit FIFO Underrun"]
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
#[doc = "Field `UN` writer - Transmit FIFO Underrun"]
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
#[doc = "Field `RL` reader - Collision Retry Limit"]
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
#[doc = "Field `RL` writer - Collision Retry Limit"]
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
#[doc = "Field `LC` reader - Late Collision"]
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
#[doc = "Field `LC` writer - Late Collision"]
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
#[doc = "Field `EBERR` reader - Ethernet Bus Error"]
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
#[doc = "Field `EBERR` writer - Ethernet Bus Error"]
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
#[doc = "Field `MII` reader - MII Interrupt."]
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
#[doc = "Field `MII` writer - MII Interrupt."]
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
#[doc = "Field `RXB` reader - Receive Buffer Interrupt"]
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
#[doc = "Field `RXB` writer - Receive Buffer Interrupt"]
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
#[doc = "Field `RXF` reader - Receive Frame Interrupt"]
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
#[doc = "Field `RXF` writer - Receive Frame Interrupt"]
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
#[doc = "Field `TXB` reader - Transmit Buffer Interrupt"]
pub struct TXB_R(crate::FieldReader<bool, bool>);
impl TXB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXB` writer - Transmit Buffer Interrupt"]
pub struct TXB_W<'a> {
    w: &'a mut W,
}
impl<'a> TXB_W<'a> {
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
#[doc = "Field `TXF` reader - Transmit Frame Interrupt"]
pub struct TXF_R(crate::FieldReader<bool, bool>);
impl TXF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXF` writer - Transmit Frame Interrupt"]
pub struct TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXF_W<'a> {
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
#[doc = "Field `GRA` reader - Graceful Stop Complete"]
pub struct GRA_R(crate::FieldReader<bool, bool>);
impl GRA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GRA` writer - Graceful Stop Complete"]
pub struct GRA_W<'a> {
    w: &'a mut W,
}
impl<'a> GRA_W<'a> {
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
#[doc = "Field `BABT` reader - Babbling Transmit Error"]
pub struct BABT_R(crate::FieldReader<bool, bool>);
impl BABT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BABT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BABT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BABT` writer - Babbling Transmit Error"]
pub struct BABT_W<'a> {
    w: &'a mut W,
}
impl<'a> BABT_W<'a> {
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
#[doc = "Field `BABR` reader - Babbling Receive Error"]
pub struct BABR_R(crate::FieldReader<bool, bool>);
impl BABR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BABR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BABR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BABR` writer - Babbling Receive Error"]
pub struct BABR_W<'a> {
    w: &'a mut W,
}
impl<'a> BABR_W<'a> {
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
    #[doc = "Bit 15 - Timestamp Timer"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit Timestamp Available"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Node Wakeup Request Indication"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Payload Receive Error"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underrun"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Collision Retry Limit"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Late Collision"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Ethernet Bus Error"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt."]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive Buffer Interrupt"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive Frame Interrupt"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Transmit Buffer Interrupt"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transmit Frame Interrupt"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Graceful Stop Complete"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Babbling Transmit Error"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Babbling Receive Error"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Timestamp Timer"]
    #[inline(always)]
    pub fn ts_timer(&mut self) -> TS_TIMER_W {
        TS_TIMER_W { w: self }
    }
    #[doc = "Bit 16 - Transmit Timestamp Available"]
    #[inline(always)]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W {
        TS_AVAIL_W { w: self }
    }
    #[doc = "Bit 17 - Node Wakeup Request Indication"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
    #[doc = "Bit 18 - Payload Receive Error"]
    #[inline(always)]
    pub fn plr(&mut self) -> PLR_W {
        PLR_W { w: self }
    }
    #[doc = "Bit 19 - Transmit FIFO Underrun"]
    #[inline(always)]
    pub fn un(&mut self) -> UN_W {
        UN_W { w: self }
    }
    #[doc = "Bit 20 - Collision Retry Limit"]
    #[inline(always)]
    pub fn rl(&mut self) -> RL_W {
        RL_W { w: self }
    }
    #[doc = "Bit 21 - Late Collision"]
    #[inline(always)]
    pub fn lc(&mut self) -> LC_W {
        LC_W { w: self }
    }
    #[doc = "Bit 22 - Ethernet Bus Error"]
    #[inline(always)]
    pub fn eberr(&mut self) -> EBERR_W {
        EBERR_W { w: self }
    }
    #[doc = "Bit 23 - MII Interrupt."]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W {
        MII_W { w: self }
    }
    #[doc = "Bit 24 - Receive Buffer Interrupt"]
    #[inline(always)]
    pub fn rxb(&mut self) -> RXB_W {
        RXB_W { w: self }
    }
    #[doc = "Bit 25 - Receive Frame Interrupt"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 26 - Transmit Buffer Interrupt"]
    #[inline(always)]
    pub fn txb(&mut self) -> TXB_W {
        TXB_W { w: self }
    }
    #[doc = "Bit 27 - Transmit Frame Interrupt"]
    #[inline(always)]
    pub fn txf(&mut self) -> TXF_W {
        TXF_W { w: self }
    }
    #[doc = "Bit 28 - Graceful Stop Complete"]
    #[inline(always)]
    pub fn gra(&mut self) -> GRA_W {
        GRA_W { w: self }
    }
    #[doc = "Bit 29 - Babbling Transmit Error"]
    #[inline(always)]
    pub fn babt(&mut self) -> BABT_W {
        BABT_W { w: self }
    }
    #[doc = "Bit 30 - Babbling Receive Error"]
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
#[doc = "Interrupt Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eir](index.html) module"]
pub struct EIR_SPEC;
impl crate::RegisterSpec for EIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eir::R](R) reader structure"]
impl crate::Readable for EIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eir::W](W) writer structure"]
impl crate::Writable for EIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EIR to value 0"]
impl crate::Resettable for EIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
