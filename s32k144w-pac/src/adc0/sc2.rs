#[doc = "Register `SC2` reader"]
pub struct R(crate::R<SC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC2` writer"]
pub struct W(crate::W<SC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC2_SPEC>;
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
impl From<crate::W<SC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    REFSEL_0 = 0,
    #[doc = "1: Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU."]
    REFSEL_1 = 1,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub struct REFSEL_R(crate::FieldReader<u8, REFSEL_A>);
impl REFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::REFSEL_0),
            1 => Some(REFSEL_A::REFSEL_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REFSEL_0`"]
    #[inline(always)]
    pub fn is_refsel_0(&self) -> bool {
        **self == REFSEL_A::REFSEL_0
    }
    #[doc = "Checks if the value of the field is `REFSEL_1`"]
    #[inline(always)]
    pub fn is_refsel_1(&self) -> bool {
        **self == REFSEL_A::REFSEL_1
    }
}
impl core::ops::Deref for REFSEL_R {
    type Target = crate::FieldReader<u8, REFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline(always)]
    pub fn refsel_0(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_0)
    }
    #[doc = "Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU."]
    #[inline(always)]
    pub fn refsel_1(self) -> &'a mut W {
        self.variant(REFSEL_A::REFSEL_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    DMAEN_0 = 0,
    #[doc = "1: DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n\\[COCO\\]
flag is asserted."]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        **self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        **self == DMAEN_A::DMAEN_1
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n\\[COCO\\]
flag is asserted."]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
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
#[doc = "Field `ACREN` reader - Compare Function Range Enable"]
pub struct ACREN_R(crate::FieldReader<bool, bool>);
impl ACREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACREN` writer - Compare Function Range Enable"]
pub struct ACREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACREN_W<'a> {
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
#[doc = "Field `ACFGT` reader - Compare Function Greater Than Enable"]
pub struct ACFGT_R(crate::FieldReader<bool, bool>);
impl ACFGT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACFGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACFGT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACFGT` writer - Compare Function Greater Than Enable"]
pub struct ACFGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFGT_W<'a> {
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
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled."]
    ACFE_0 = 0,
    #[doc = "1: Compare function enabled."]
    ACFE_1 = 1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFE` reader - Compare Function Enable"]
pub struct ACFE_R(crate::FieldReader<bool, ACFE_A>);
impl ACFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::ACFE_0,
            true => ACFE_A::ACFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFE_0`"]
    #[inline(always)]
    pub fn is_acfe_0(&self) -> bool {
        **self == ACFE_A::ACFE_0
    }
    #[doc = "Checks if the value of the field is `ACFE_1`"]
    #[inline(always)]
    pub fn is_acfe_1(&self) -> bool {
        **self == ACFE_A::ACFE_1
    }
}
impl core::ops::Deref for ACFE_R {
    type Target = crate::FieldReader<bool, ACFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACFE` writer - Compare Function Enable"]
pub struct ACFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn acfe_0(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn acfe_1(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_1)
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
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected."]
    ADTRG_0 = 0,
    #[doc = "1: Hardware trigger selected."]
    ADTRG_1 = 1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADTRG` reader - Conversion Trigger Select"]
pub struct ADTRG_R(crate::FieldReader<bool, ADTRG_A>);
impl ADTRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADTRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::ADTRG_0,
            true => ADTRG_A::ADTRG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADTRG_0`"]
    #[inline(always)]
    pub fn is_adtrg_0(&self) -> bool {
        **self == ADTRG_A::ADTRG_0
    }
    #[doc = "Checks if the value of the field is `ADTRG_1`"]
    #[inline(always)]
    pub fn is_adtrg_1(&self) -> bool {
        **self == ADTRG_A::ADTRG_1
    }
}
impl core::ops::Deref for ADTRG_R {
    type Target = crate::FieldReader<bool, ADTRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADTRG` writer - Conversion Trigger Select"]
pub struct ADTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADTRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn adtrg_0(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn adtrg_1(self) -> &'a mut W {
        self.variant(ADTRG_A::ADTRG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    ADACT_0 = 0,
    #[doc = "1: Conversion in progress."]
    ADACT_1 = 1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACT` reader - Conversion Active"]
pub struct ADACT_R(crate::FieldReader<bool, ADACT_A>);
impl ADACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::ADACT_0,
            true => ADACT_A::ADACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACT_0`"]
    #[inline(always)]
    pub fn is_adact_0(&self) -> bool {
        **self == ADACT_A::ADACT_0
    }
    #[doc = "Checks if the value of the field is `ADACT_1`"]
    #[inline(always)]
    pub fn is_adact_1(&self) -> bool {
        **self == ADACT_A::ADACT_1
    }
}
impl core::ops::Deref for ADACT_R {
    type Target = crate::FieldReader<bool, ADACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> ACREN_R {
        ACREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&mut self) -> ACREN_W {
        ACREN_W { w: self }
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&mut self) -> ACFGT_W {
        ACFGT_W { w: self }
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&mut self) -> ACFE_W {
        ACFE_W { w: self }
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&mut self) -> ADTRG_W {
        ADTRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](index.html) module"]
pub struct SC2_SPEC;
impl crate::RegisterSpec for SC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc2::R](R) reader structure"]
impl crate::Readable for SC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc2::W](W) writer structure"]
impl crate::Writable for SC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC2 to value 0"]
impl crate::Resettable for SC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
