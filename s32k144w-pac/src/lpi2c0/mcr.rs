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
#[doc = "Master Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEN_A {
    #[doc = "0: Master logic is disabled"]
    MEN_0 = 0,
    #[doc = "1: Master logic is enabled"]
    MEN_1 = 1,
}
impl From<MEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEN` reader - Master Enable"]
pub struct MEN_R(crate::FieldReader<bool, MEN_A>);
impl MEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEN_A {
        match self.bits {
            false => MEN_A::MEN_0,
            true => MEN_A::MEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MEN_0`"]
    #[inline(always)]
    pub fn is_men_0(&self) -> bool {
        **self == MEN_A::MEN_0
    }
    #[doc = "Checks if the value of the field is `MEN_1`"]
    #[inline(always)]
    pub fn is_men_1(&self) -> bool {
        **self == MEN_A::MEN_1
    }
}
impl core::ops::Deref for MEN_R {
    type Target = crate::FieldReader<bool, MEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MEN` writer - Master Enable"]
pub struct MEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master logic is disabled"]
    #[inline(always)]
    pub fn men_0(self) -> &'a mut W {
        self.variant(MEN_A::MEN_0)
    }
    #[doc = "Master logic is enabled"]
    #[inline(always)]
    pub fn men_1(self) -> &'a mut W {
        self.variant(MEN_A::MEN_1)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Master logic is not reset"]
    RST_0 = 0,
    #[doc = "1: Master logic is reset"]
    RST_1 = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::RST_0,
            true => RST_A::RST_1,
        }
    }
    #[doc = "Checks if the value of the field is `RST_0`"]
    #[inline(always)]
    pub fn is_rst_0(&self) -> bool {
        **self == RST_A::RST_0
    }
    #[doc = "Checks if the value of the field is `RST_1`"]
    #[inline(always)]
    pub fn is_rst_1(&self) -> bool {
        **self == RST_A::RST_1
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master logic is not reset"]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RST_A::RST_0)
    }
    #[doc = "Master logic is reset"]
    #[inline(always)]
    pub fn rst_1(self) -> &'a mut W {
        self.variant(RST_A::RST_1)
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
#[doc = "Doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEN_A {
    #[doc = "0: Master is enabled in Doze mode"]
    DOZEN_0 = 0,
    #[doc = "1: Master is disabled in Doze mode"]
    DOZEN_1 = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze mode enable"]
pub struct DOZEN_R(crate::FieldReader<bool, DOZEN_A>);
impl DOZEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DOZEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::DOZEN_0,
            true => DOZEN_A::DOZEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DOZEN_0`"]
    #[inline(always)]
    pub fn is_dozen_0(&self) -> bool {
        **self == DOZEN_A::DOZEN_0
    }
    #[doc = "Checks if the value of the field is `DOZEN_1`"]
    #[inline(always)]
    pub fn is_dozen_1(&self) -> bool {
        **self == DOZEN_A::DOZEN_1
    }
}
impl core::ops::Deref for DOZEN_R {
    type Target = crate::FieldReader<bool, DOZEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOZEN` writer - Doze mode enable"]
pub struct DOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master is enabled in Doze mode"]
    #[inline(always)]
    pub fn dozen_0(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_0)
    }
    #[doc = "Master is disabled in Doze mode"]
    #[inline(always)]
    pub fn dozen_1(self) -> &'a mut W {
        self.variant(DOZEN_A::DOZEN_1)
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
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGEN_A {
    #[doc = "0: Master is disabled in debug mode"]
    DBGEN_0 = 0,
    #[doc = "1: Master is enabled in debug mode"]
    DBGEN_1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` reader - Debug Enable"]
pub struct DBGEN_R(crate::FieldReader<bool, DBGEN_A>);
impl DBGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DBGEN_0,
            true => DBGEN_A::DBGEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBGEN_0`"]
    #[inline(always)]
    pub fn is_dbgen_0(&self) -> bool {
        **self == DBGEN_A::DBGEN_0
    }
    #[doc = "Checks if the value of the field is `DBGEN_1`"]
    #[inline(always)]
    pub fn is_dbgen_1(&self) -> bool {
        **self == DBGEN_A::DBGEN_1
    }
}
impl core::ops::Deref for DBGEN_R {
    type Target = crate::FieldReader<bool, DBGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBGEN` writer - Debug Enable"]
pub struct DBGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master is disabled in debug mode"]
    #[inline(always)]
    pub fn dbgen_0(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_0)
    }
    #[doc = "Master is enabled in debug mode"]
    #[inline(always)]
    pub fn dbgen_1(self) -> &'a mut W {
        self.variant(DBGEN_A::DBGEN_1)
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
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTF_A {
    #[doc = "0: No effect"]
    RTF_0 = 0,
    #[doc = "1: Transmit FIFO is reset"]
    RTF_1 = 1,
}
impl From<RTF_A> for bool {
    #[inline(always)]
    fn from(variant: RTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTF` reader - Reset Transmit FIFO"]
pub struct RTF_R(crate::FieldReader<bool, RTF_A>);
impl RTF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTF_A {
        match self.bits {
            false => RTF_A::RTF_0,
            true => RTF_A::RTF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTF_0`"]
    #[inline(always)]
    pub fn is_rtf_0(&self) -> bool {
        **self == RTF_A::RTF_0
    }
    #[doc = "Checks if the value of the field is `RTF_1`"]
    #[inline(always)]
    pub fn is_rtf_1(&self) -> bool {
        **self == RTF_A::RTF_1
    }
}
impl core::ops::Deref for RTF_R {
    type Target = crate::FieldReader<bool, RTF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTF` writer - Reset Transmit FIFO"]
pub struct RTF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rtf_0(self) -> &'a mut W {
        self.variant(RTF_A::RTF_0)
    }
    #[doc = "Transmit FIFO is reset"]
    #[inline(always)]
    pub fn rtf_1(self) -> &'a mut W {
        self.variant(RTF_A::RTF_1)
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
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRF_A {
    #[doc = "0: No effect"]
    RRF_0 = 0,
    #[doc = "1: Receive FIFO is reset"]
    RRF_1 = 1,
}
impl From<RRF_A> for bool {
    #[inline(always)]
    fn from(variant: RRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRF` reader - Reset Receive FIFO"]
pub struct RRF_R(crate::FieldReader<bool, RRF_A>);
impl RRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRF_A {
        match self.bits {
            false => RRF_A::RRF_0,
            true => RRF_A::RRF_1,
        }
    }
    #[doc = "Checks if the value of the field is `RRF_0`"]
    #[inline(always)]
    pub fn is_rrf_0(&self) -> bool {
        **self == RRF_A::RRF_0
    }
    #[doc = "Checks if the value of the field is `RRF_1`"]
    #[inline(always)]
    pub fn is_rrf_1(&self) -> bool {
        **self == RRF_A::RRF_1
    }
}
impl core::ops::Deref for RRF_R {
    type Target = crate::FieldReader<bool, RRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRF` writer - Reset Receive FIFO"]
pub struct RRF_W<'a> {
    w: &'a mut W,
}
impl<'a> RRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn rrf_0(self) -> &'a mut W {
        self.variant(RRF_A::RRF_0)
    }
    #[doc = "Receive FIFO is reset"]
    #[inline(always)]
    pub fn rrf_1(self) -> &'a mut W {
        self.variant(RRF_A::RRF_1)
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
    #[doc = "Bit 0 - Master Enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&self) -> RTF_R {
        RTF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable"]
    #[inline(always)]
    pub fn men(&mut self) -> MEN_W {
        MEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&mut self) -> DOZEN_W {
        DOZEN_W { w: self }
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W {
        DBGEN_W { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&mut self) -> RTF_W {
        RTF_W { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&mut self) -> RRF_W {
        RRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
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
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
