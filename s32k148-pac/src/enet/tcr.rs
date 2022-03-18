#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTS` reader - Graceful Transmit Stop"]
pub struct GTS_R(crate::FieldReader<bool, bool>);
impl GTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GTS` writer - Graceful Transmit Stop"]
pub struct GTS_W<'a> {
    w: &'a mut W,
}
impl<'a> GTS_W<'a> {
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
#[doc = "Field `FDEN` reader - Full-Duplex Enable"]
pub struct FDEN_R(crate::FieldReader<bool, bool>);
impl FDEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDEN` writer - Full-Duplex Enable"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
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
#[doc = "Transmit Frame Control Pause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_PAUSE_A {
    #[doc = "0: No PAUSE frame transmitted."]
    _0 = 0,
    #[doc = "1: The MAC stops transmission of data frames after the current transmission is complete."]
    _1 = 1,
}
impl From<TFC_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: TFC_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFC_PAUSE` reader - Transmit Frame Control Pause"]
pub struct TFC_PAUSE_R(crate::FieldReader<bool, TFC_PAUSE_A>);
impl TFC_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFC_PAUSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_PAUSE_A {
        match self.bits {
            false => TFC_PAUSE_A::_0,
            true => TFC_PAUSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TFC_PAUSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TFC_PAUSE_A::_1
    }
}
impl core::ops::Deref for TFC_PAUSE_R {
    type Target = crate::FieldReader<bool, TFC_PAUSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFC_PAUSE` writer - Transmit Frame Control Pause"]
pub struct TFC_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFC_PAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFC_PAUSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No PAUSE frame transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_0)
    }
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_1)
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
#[doc = "Field `RFC_PAUSE` reader - Receive Frame Control Pause"]
pub struct RFC_PAUSE_R(crate::FieldReader<bool, bool>);
impl RFC_PAUSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFC_PAUSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC_PAUSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Source MAC Address Select On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDSEL_A {
    #[doc = "0: Node MAC address programmed on PADDR1/2 registers."]
    _000 = 0,
}
impl From<ADDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADDSEL` reader - Source MAC Address Select On Transmit"]
pub struct ADDSEL_R(crate::FieldReader<u8, ADDSEL_A>);
impl ADDSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDSEL_A> {
        match self.bits {
            0 => Some(ADDSEL_A::_000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        **self == ADDSEL_A::_000
    }
}
impl core::ops::Deref for ADDSEL_R {
    type Target = crate::FieldReader<u8, ADDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDSEL` writer - Source MAC Address Select On Transmit"]
pub struct ADDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDSEL_A::_000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Set MAC Address On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDINS_A {
    #[doc = "0: The source MAC address is not modified by the MAC."]
    _0 = 0,
    #[doc = "1: The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    _1 = 1,
}
impl From<ADDINS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDINS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDINS` reader - Set MAC Address On Transmit"]
pub struct ADDINS_R(crate::FieldReader<bool, ADDINS_A>);
impl ADDINS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADDINS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDINS_A {
        match self.bits {
            false => ADDINS_A::_0,
            true => ADDINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADDINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADDINS_A::_1
    }
}
impl core::ops::Deref for ADDINS_R {
    type Target = crate::FieldReader<bool, ADDINS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDINS` writer - Set MAC Address On Transmit"]
pub struct ADDINS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDINS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The source MAC address is not modified by the MAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDINS_A::_0)
    }
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDINS_A::_1)
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
#[doc = "Forward Frame From Application With CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWD_A {
    #[doc = "0: TxBD\\[TC\\]
controls whether the frame has a CRC from the application."]
    _0 = 0,
    #[doc = "1: The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    _1 = 1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCFWD` reader - Forward Frame From Application With CRC"]
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
#[doc = "Field `CRCFWD` writer - Forward Frame From Application With CRC"]
pub struct CRCFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCFWD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TxBD\\[TC\\]
controls whether the frame has a CRC from the application."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWD_A::_0)
    }
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    pub fn gts(&self) -> GTS_R {
        GTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    pub fn tfc_pause(&self) -> TFC_PAUSE_R {
        TFC_PAUSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Frame Control Pause"]
    #[inline(always)]
    pub fn rfc_pause(&self) -> RFC_PAUSE_R {
        RFC_PAUSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub fn addsel(&self) -> ADDSEL_R {
        ADDSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    pub fn addins(&self) -> ADDINS_R {
        ADDINS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    pub fn gts(&mut self) -> GTS_W {
        GTS_W { w: self }
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    pub fn tfc_pause(&mut self) -> TFC_PAUSE_W {
        TFC_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub fn addsel(&mut self) -> ADDSEL_W {
        ADDSEL_W { w: self }
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    pub fn addins(&mut self) -> ADDINS_W {
        ADDINS_W { w: self }
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    pub fn crcfwd(&mut self) -> CRCFWD_W {
        CRCFWD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
