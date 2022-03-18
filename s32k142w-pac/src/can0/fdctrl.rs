#[doc = "Register `FDCTRL` reader"]
pub struct R(crate::R<FDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCTRL` writer"]
pub struct W(crate::W<FDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCTRL_SPEC>;
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
impl From<crate::W<FDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDCVAL` reader - Transceiver Delay Compensation Value"]
pub struct TDCVAL_R(crate::FieldReader<u8, u8>);
impl TDCVAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCVAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCOFF` reader - Transceiver Delay Compensation Offset"]
pub struct TDCOFF_R(crate::FieldReader<u8, u8>);
impl TDCOFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCOFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCOFF` writer - Transceiver Delay Compensation Offset"]
pub struct TDCOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Transceiver Delay Compensation Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCFAIL_A {
    #[doc = "0: Measured loop delay is in range."]
    IN_RANGE = 0,
    #[doc = "1: Measured loop delay is out of range."]
    OUT_OF_RANGE = 1,
}
impl From<TDCFAIL_A> for bool {
    #[inline(always)]
    fn from(variant: TDCFAIL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCFAIL` reader - Transceiver Delay Compensation Fail"]
pub struct TDCFAIL_R(crate::FieldReader<bool, TDCFAIL_A>);
impl TDCFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDCFAIL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCFAIL_A {
        match self.bits {
            false => TDCFAIL_A::IN_RANGE,
            true => TDCFAIL_A::OUT_OF_RANGE,
        }
    }
    #[doc = "Checks if the value of the field is `IN_RANGE`"]
    #[inline(always)]
    pub fn is_in_range(&self) -> bool {
        **self == TDCFAIL_A::IN_RANGE
    }
    #[doc = "Checks if the value of the field is `OUT_OF_RANGE`"]
    #[inline(always)]
    pub fn is_out_of_range(&self) -> bool {
        **self == TDCFAIL_A::OUT_OF_RANGE
    }
}
impl core::ops::Deref for TDCFAIL_R {
    type Target = crate::FieldReader<bool, TDCFAIL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCFAIL` writer - Transceiver Delay Compensation Fail"]
pub struct TDCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCFAIL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCFAIL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Measured loop delay is in range."]
    #[inline(always)]
    pub fn in_range(self) -> &'a mut W {
        self.variant(TDCFAIL_A::IN_RANGE)
    }
    #[doc = "Measured loop delay is out of range."]
    #[inline(always)]
    pub fn out_of_range(self) -> &'a mut W {
        self.variant(TDCFAIL_A::OUT_OF_RANGE)
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
#[doc = "Transceiver Delay Compensation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCEN_A {
    #[doc = "0: TDC is disabled"]
    DISABLE = 0,
    #[doc = "1: TDC is enabled"]
    ENABLE = 1,
}
impl From<TDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCEN` reader - Transceiver Delay Compensation Enable"]
pub struct TDCEN_R(crate::FieldReader<bool, TDCEN_A>);
impl TDCEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDCEN_A {
        match self.bits {
            false => TDCEN_A::DISABLE,
            true => TDCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TDCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TDCEN_A::ENABLE
    }
}
impl core::ops::Deref for TDCEN_R {
    type Target = crate::FieldReader<bool, TDCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCEN` writer - Transceiver Delay Compensation Enable"]
pub struct TDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "TDC is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TDCEN_A::DISABLE)
    }
    #[doc = "TDC is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TDCEN_A::ENABLE)
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
#[doc = "Message Buffer Data Size for Region 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBDSR0_A {
    #[doc = "0: Selects 8 bytes per message buffer."]
    R0_8_BYTES = 0,
    #[doc = "1: Selects 16 bytes per message buffer."]
    R0_16_BYTES = 1,
    #[doc = "2: Selects 32 bytes per message buffer."]
    R0_32_BYTES = 2,
    #[doc = "3: Selects 64 bytes per message buffer."]
    R0_64_BYTES = 3,
}
impl From<MBDSR0_A> for u8 {
    #[inline(always)]
    fn from(variant: MBDSR0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MBDSR0` reader - Message Buffer Data Size for Region 0"]
pub struct MBDSR0_R(crate::FieldReader<u8, MBDSR0_A>);
impl MBDSR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MBDSR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBDSR0_A {
        match self.bits {
            0 => MBDSR0_A::R0_8_BYTES,
            1 => MBDSR0_A::R0_16_BYTES,
            2 => MBDSR0_A::R0_32_BYTES,
            3 => MBDSR0_A::R0_64_BYTES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R0_8_BYTES`"]
    #[inline(always)]
    pub fn is_r0_8_bytes(&self) -> bool {
        **self == MBDSR0_A::R0_8_BYTES
    }
    #[doc = "Checks if the value of the field is `R0_16_BYTES`"]
    #[inline(always)]
    pub fn is_r0_16_bytes(&self) -> bool {
        **self == MBDSR0_A::R0_16_BYTES
    }
    #[doc = "Checks if the value of the field is `R0_32_BYTES`"]
    #[inline(always)]
    pub fn is_r0_32_bytes(&self) -> bool {
        **self == MBDSR0_A::R0_32_BYTES
    }
    #[doc = "Checks if the value of the field is `R0_64_BYTES`"]
    #[inline(always)]
    pub fn is_r0_64_bytes(&self) -> bool {
        **self == MBDSR0_A::R0_64_BYTES
    }
}
impl core::ops::Deref for MBDSR0_R {
    type Target = crate::FieldReader<u8, MBDSR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBDSR0` writer - Message Buffer Data Size for Region 0"]
pub struct MBDSR0_W<'a> {
    w: &'a mut W,
}
impl<'a> MBDSR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBDSR0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Selects 8 bytes per message buffer."]
    #[inline(always)]
    pub fn r0_8_bytes(self) -> &'a mut W {
        self.variant(MBDSR0_A::R0_8_BYTES)
    }
    #[doc = "Selects 16 bytes per message buffer."]
    #[inline(always)]
    pub fn r0_16_bytes(self) -> &'a mut W {
        self.variant(MBDSR0_A::R0_16_BYTES)
    }
    #[doc = "Selects 32 bytes per message buffer."]
    #[inline(always)]
    pub fn r0_32_bytes(self) -> &'a mut W {
        self.variant(MBDSR0_A::R0_32_BYTES)
    }
    #[doc = "Selects 64 bytes per message buffer."]
    #[inline(always)]
    pub fn r0_64_bytes(self) -> &'a mut W {
        self.variant(MBDSR0_A::R0_64_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Message Buffer Data Size for Region 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MBDSR1_A {
    #[doc = "0: Selects 8 bytes per message buffer."]
    R1_8_BYTES = 0,
    #[doc = "1: Selects 16 bytes per message buffer."]
    R1_16_BYTES = 1,
    #[doc = "2: Selects 32 bytes per message buffer."]
    R1_32_BYTES = 2,
    #[doc = "3: Selects 64 bytes per message buffer."]
    R1_64_BYTES = 3,
}
impl From<MBDSR1_A> for u8 {
    #[inline(always)]
    fn from(variant: MBDSR1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MBDSR1` reader - Message Buffer Data Size for Region 1"]
pub struct MBDSR1_R(crate::FieldReader<u8, MBDSR1_A>);
impl MBDSR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MBDSR1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MBDSR1_A {
        match self.bits {
            0 => MBDSR1_A::R1_8_BYTES,
            1 => MBDSR1_A::R1_16_BYTES,
            2 => MBDSR1_A::R1_32_BYTES,
            3 => MBDSR1_A::R1_64_BYTES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R1_8_BYTES`"]
    #[inline(always)]
    pub fn is_r1_8_bytes(&self) -> bool {
        **self == MBDSR1_A::R1_8_BYTES
    }
    #[doc = "Checks if the value of the field is `R1_16_BYTES`"]
    #[inline(always)]
    pub fn is_r1_16_bytes(&self) -> bool {
        **self == MBDSR1_A::R1_16_BYTES
    }
    #[doc = "Checks if the value of the field is `R1_32_BYTES`"]
    #[inline(always)]
    pub fn is_r1_32_bytes(&self) -> bool {
        **self == MBDSR1_A::R1_32_BYTES
    }
    #[doc = "Checks if the value of the field is `R1_64_BYTES`"]
    #[inline(always)]
    pub fn is_r1_64_bytes(&self) -> bool {
        **self == MBDSR1_A::R1_64_BYTES
    }
}
impl core::ops::Deref for MBDSR1_R {
    type Target = crate::FieldReader<u8, MBDSR1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MBDSR1` writer - Message Buffer Data Size for Region 1"]
pub struct MBDSR1_W<'a> {
    w: &'a mut W,
}
impl<'a> MBDSR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBDSR1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Selects 8 bytes per message buffer."]
    #[inline(always)]
    pub fn r1_8_bytes(self) -> &'a mut W {
        self.variant(MBDSR1_A::R1_8_BYTES)
    }
    #[doc = "Selects 16 bytes per message buffer."]
    #[inline(always)]
    pub fn r1_16_bytes(self) -> &'a mut W {
        self.variant(MBDSR1_A::R1_16_BYTES)
    }
    #[doc = "Selects 32 bytes per message buffer."]
    #[inline(always)]
    pub fn r1_32_bytes(self) -> &'a mut W {
        self.variant(MBDSR1_A::R1_32_BYTES)
    }
    #[doc = "Selects 64 bytes per message buffer."]
    #[inline(always)]
    pub fn r1_64_bytes(self) -> &'a mut W {
        self.variant(MBDSR1_A::R1_64_BYTES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Bit Rate Switch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDRATE_A {
    #[doc = "0: Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    NOMINAL = 0,
    #[doc = "1: Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    BIT_RATE_SWITCHING = 1,
}
impl From<FDRATE_A> for bool {
    #[inline(always)]
    fn from(variant: FDRATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDRATE` reader - Bit Rate Switch Enable"]
pub struct FDRATE_R(crate::FieldReader<bool, FDRATE_A>);
impl FDRATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FDRATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDRATE_A {
        match self.bits {
            false => FDRATE_A::NOMINAL,
            true => FDRATE_A::BIT_RATE_SWITCHING,
        }
    }
    #[doc = "Checks if the value of the field is `NOMINAL`"]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        **self == FDRATE_A::NOMINAL
    }
    #[doc = "Checks if the value of the field is `BIT_RATE_SWITCHING`"]
    #[inline(always)]
    pub fn is_bit_rate_switching(&self) -> bool {
        **self == FDRATE_A::BIT_RATE_SWITCHING
    }
}
impl core::ops::Deref for FDRATE_R {
    type Target = crate::FieldReader<bool, FDRATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDRATE` writer - Bit Rate Switch Enable"]
pub struct FDRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDRATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDRATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    #[inline(always)]
    pub fn nominal(self) -> &'a mut W {
        self.variant(FDRATE_A::NOMINAL)
    }
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    #[inline(always)]
    pub fn bit_rate_switching(self) -> &'a mut W {
        self.variant(FDRATE_A::BIT_RATE_SWITCHING)
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
    #[doc = "Bits 0:5 - Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcval(&self) -> TDCVAL_R {
        TDCVAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&self) -> TDCOFF_R {
        TDCOFF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&self) -> TDCFAIL_R {
        TDCFAIL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&self) -> MBDSR0_R {
        MBDSR0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Message Buffer Data Size for Region 1"]
    #[inline(always)]
    pub fn mbdsr1(&self) -> MBDSR1_R {
        MBDSR1_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&self) -> FDRATE_R {
        FDRATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&mut self) -> TDCOFF_W {
        TDCOFF_W { w: self }
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&mut self) -> TDCFAIL_W {
        TDCFAIL_W { w: self }
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TDCEN_W {
        TDCEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&mut self) -> MBDSR0_W {
        MBDSR0_W { w: self }
    }
    #[doc = "Bits 19:20 - Message Buffer Data Size for Region 1"]
    #[inline(always)]
    pub fn mbdsr1(&mut self) -> MBDSR1_W {
        MBDSR1_W { w: self }
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&mut self) -> FDRATE_W {
        FDRATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN FD Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdctrl](index.html) module"]
pub struct FDCTRL_SPEC;
impl crate::RegisterSpec for FDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdctrl::R](R) reader structure"]
impl crate::Readable for FDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdctrl::W](W) writer structure"]
impl crate::Writable for FDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCTRL to value 0x8000_0100"]
impl crate::Resettable for FDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0100
    }
}
