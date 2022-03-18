#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEN_A {
    #[doc = "0: I2C Slave mode is disabled"]
    SEN_0 = 0,
    #[doc = "1: I2C Slave mode is enabled"]
    SEN_1 = 1,
}
impl From<SEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEN` reader - Slave Enable"]
pub struct SEN_R(crate::FieldReader<bool, SEN_A>);
impl SEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN_A {
        match self.bits {
            false => SEN_A::SEN_0,
            true => SEN_A::SEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SEN_0`"]
    #[inline(always)]
    pub fn is_sen_0(&self) -> bool {
        **self == SEN_A::SEN_0
    }
    #[doc = "Checks if the value of the field is `SEN_1`"]
    #[inline(always)]
    pub fn is_sen_1(&self) -> bool {
        **self == SEN_A::SEN_1
    }
}
impl core::ops::Deref for SEN_R {
    type Target = crate::FieldReader<bool, SEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEN` writer - Slave Enable"]
pub struct SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "I2C Slave mode is disabled"]
    #[inline(always)]
    pub fn sen_0(self) -> &'a mut W {
        self.variant(SEN_A::SEN_0)
    }
    #[doc = "I2C Slave mode is enabled"]
    #[inline(always)]
    pub fn sen_1(self) -> &'a mut W {
        self.variant(SEN_A::SEN_1)
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
    #[doc = "0: Slave mode logic is not reset"]
    RST_0 = 0,
    #[doc = "1: Slave mode logic is reset"]
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
    #[doc = "Slave mode logic is not reset"]
    #[inline(always)]
    pub fn rst_0(self) -> &'a mut W {
        self.variant(RST_A::RST_0)
    }
    #[doc = "Slave mode logic is reset"]
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
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEN_A {
    #[doc = "0: Disable digital filter and output delay counter for slave mode"]
    FILTEN_0 = 0,
    #[doc = "1: Enable digital filter and output delay counter for slave mode"]
    FILTEN_1 = 1,
}
impl From<FILTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTEN` reader - Filter Enable"]
pub struct FILTEN_R(crate::FieldReader<bool, FILTEN_A>);
impl FILTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTEN_A {
        match self.bits {
            false => FILTEN_A::FILTEN_0,
            true => FILTEN_A::FILTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTEN_0`"]
    #[inline(always)]
    pub fn is_filten_0(&self) -> bool {
        **self == FILTEN_A::FILTEN_0
    }
    #[doc = "Checks if the value of the field is `FILTEN_1`"]
    #[inline(always)]
    pub fn is_filten_1(&self) -> bool {
        **self == FILTEN_A::FILTEN_1
    }
}
impl core::ops::Deref for FILTEN_R {
    type Target = crate::FieldReader<bool, FILTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTEN` writer - Filter Enable"]
pub struct FILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn filten_0(self) -> &'a mut W {
        self.variant(FILTEN_A::FILTEN_0)
    }
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn filten_1(self) -> &'a mut W {
        self.variant(FILTEN_A::FILTEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Filter Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTDZ_A {
    #[doc = "0: Filter remains enabled in Doze mode"]
    FILTDZ_0 = 0,
    #[doc = "1: Filter is disabled in Doze mode"]
    FILTDZ_1 = 1,
}
impl From<FILTDZ_A> for bool {
    #[inline(always)]
    fn from(variant: FILTDZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTDZ` reader - Filter Doze Enable"]
pub struct FILTDZ_R(crate::FieldReader<bool, FILTDZ_A>);
impl FILTDZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTDZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTDZ_A {
        match self.bits {
            false => FILTDZ_A::FILTDZ_0,
            true => FILTDZ_A::FILTDZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FILTDZ_0`"]
    #[inline(always)]
    pub fn is_filtdz_0(&self) -> bool {
        **self == FILTDZ_A::FILTDZ_0
    }
    #[doc = "Checks if the value of the field is `FILTDZ_1`"]
    #[inline(always)]
    pub fn is_filtdz_1(&self) -> bool {
        **self == FILTDZ_A::FILTDZ_1
    }
}
impl core::ops::Deref for FILTDZ_R {
    type Target = crate::FieldReader<bool, FILTDZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILTDZ` writer - Filter Doze Enable"]
pub struct FILTDZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTDZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTDZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Filter remains enabled in Doze mode"]
    #[inline(always)]
    pub fn filtdz_0(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTDZ_0)
    }
    #[doc = "Filter is disabled in Doze mode"]
    #[inline(always)]
    pub fn filtdz_1(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTDZ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTF_A {
    #[doc = "0: No effect"]
    RTF_0 = 0,
    #[doc = "1: Transmit Data Register is now empty"]
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
    #[doc = "Transmit Data Register is now empty"]
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
    #[doc = "1: Receive Data Register is now empty"]
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
    #[doc = "Receive Data Register is now empty"]
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    pub fn filtdz(&self) -> FILTDZ_R {
        FILTDZ_R::new(((self.bits >> 5) & 0x01) != 0)
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    pub fn filten(&mut self) -> FILTEN_W {
        FILTEN_W { w: self }
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    pub fn filtdz(&mut self) -> FILTDZ_W {
        FILTDZ_W { w: self }
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
#[doc = "Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
