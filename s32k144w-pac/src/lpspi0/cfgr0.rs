#[doc = "Register `CFGR0` reader"]
pub struct R(crate::R<CFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR0` writer"]
pub struct W(crate::W<CFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR0_SPEC>;
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
impl From<crate::W<CFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Host Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HREN_A {
    #[doc = "0: Host request is disabled"]
    DISABLED = 0,
    #[doc = "1: Host request is enabled"]
    ENABLED = 1,
}
impl From<HREN_A> for bool {
    #[inline(always)]
    fn from(variant: HREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HREN` reader - Host Request Enable"]
pub struct HREN_R(crate::FieldReader<bool, HREN_A>);
impl HREN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HREN_A {
        match self.bits {
            false => HREN_A::DISABLED,
            true => HREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HREN_A::ENABLED
    }
}
impl core::ops::Deref for HREN_R {
    type Target = crate::FieldReader<bool, HREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HREN` writer - Host Request Enable"]
pub struct HREN_W<'a> {
    w: &'a mut W,
}
impl<'a> HREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host request is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HREN_A::DISABLED)
    }
    #[doc = "Host request is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HREN_A::ENABLED)
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
#[doc = "Host Request Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRPOL_A {
    #[doc = "0: LPSPI_HREQ pin is active low"]
    DISABLED = 0,
    #[doc = "1: LPSPI_HREQ pin is active high"]
    ENABLED = 1,
}
impl From<HRPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HRPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRPOL` reader - Host Request Polarity"]
pub struct HRPOL_R(crate::FieldReader<bool, HRPOL_A>);
impl HRPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPOL_A {
        match self.bits {
            false => HRPOL_A::DISABLED,
            true => HRPOL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == HRPOL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == HRPOL_A::ENABLED
    }
}
impl core::ops::Deref for HRPOL_R {
    type Target = crate::FieldReader<bool, HRPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRPOL` writer - Host Request Polarity"]
pub struct HRPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPSPI_HREQ pin is active low"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HRPOL_A::DISABLED)
    }
    #[doc = "LPSPI_HREQ pin is active high"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HRPOL_A::ENABLED)
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
#[doc = "Host Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSEL_A {
    #[doc = "0: Host request input is the LPSPI_HREQ pin"]
    HREQPIN = 0,
    #[doc = "1: Host request input is the input trigger"]
    INPUT_TRIGGER = 1,
}
impl From<HRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HRSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRSEL` reader - Host Request Select"]
pub struct HRSEL_R(crate::FieldReader<bool, HRSEL_A>);
impl HRSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HRSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRSEL_A {
        match self.bits {
            false => HRSEL_A::HREQPIN,
            true => HRSEL_A::INPUT_TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `HREQPIN`"]
    #[inline(always)]
    pub fn is_hreqpin(&self) -> bool {
        **self == HRSEL_A::HREQPIN
    }
    #[doc = "Checks if the value of the field is `INPUT_TRIGGER`"]
    #[inline(always)]
    pub fn is_input_trigger(&self) -> bool {
        **self == HRSEL_A::INPUT_TRIGGER
    }
}
impl core::ops::Deref for HRSEL_R {
    type Target = crate::FieldReader<bool, HRSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRSEL` writer - Host Request Select"]
pub struct HRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host request input is the LPSPI_HREQ pin"]
    #[inline(always)]
    pub fn hreqpin(self) -> &'a mut W {
        self.variant(HRSEL_A::HREQPIN)
    }
    #[doc = "Host request input is the input trigger"]
    #[inline(always)]
    pub fn input_trigger(self) -> &'a mut W {
        self.variant(HRSEL_A::INPUT_TRIGGER)
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
#[doc = "Circular FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRFIFO_A {
    #[doc = "0: Circular FIFO is disabled"]
    DISABLED = 0,
    #[doc = "1: Circular FIFO is enabled"]
    ENABLED = 1,
}
impl From<CIRFIFO_A> for bool {
    #[inline(always)]
    fn from(variant: CIRFIFO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRFIFO` reader - Circular FIFO Enable"]
pub struct CIRFIFO_R(crate::FieldReader<bool, CIRFIFO_A>);
impl CIRFIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CIRFIFO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRFIFO_A {
        match self.bits {
            false => CIRFIFO_A::DISABLED,
            true => CIRFIFO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CIRFIFO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CIRFIFO_A::ENABLED
    }
}
impl core::ops::Deref for CIRFIFO_R {
    type Target = crate::FieldReader<bool, CIRFIFO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CIRFIFO` writer - Circular FIFO Enable"]
pub struct CIRFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CIRFIFO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIRFIFO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Circular FIFO is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRFIFO_A::DISABLED)
    }
    #[doc = "Circular FIFO is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRFIFO_A::ENABLED)
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
#[doc = "Receive Data Match Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMO_A {
    #[doc = "0: Received data is stored in the receive FIFO as in normal operations"]
    STORED = 0,
    #[doc = "1: Received data is discarded unless the Data Match Flag (DMF) is set"]
    DISCARDED = 1,
}
impl From<RDMO_A> for bool {
    #[inline(always)]
    fn from(variant: RDMO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMO` reader - Receive Data Match Only"]
pub struct RDMO_R(crate::FieldReader<bool, RDMO_A>);
impl RDMO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDMO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMO_A {
        match self.bits {
            false => RDMO_A::STORED,
            true => RDMO_A::DISCARDED,
        }
    }
    #[doc = "Checks if the value of the field is `STORED`"]
    #[inline(always)]
    pub fn is_stored(&self) -> bool {
        **self == RDMO_A::STORED
    }
    #[doc = "Checks if the value of the field is `DISCARDED`"]
    #[inline(always)]
    pub fn is_discarded(&self) -> bool {
        **self == RDMO_A::DISCARDED
    }
}
impl core::ops::Deref for RDMO_R {
    type Target = crate::FieldReader<bool, RDMO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMO` writer - Receive Data Match Only"]
pub struct RDMO_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Received data is stored in the receive FIFO as in normal operations"]
    #[inline(always)]
    pub fn stored(self) -> &'a mut W {
        self.variant(RDMO_A::STORED)
    }
    #[doc = "Received data is discarded unless the Data Match Flag (DMF) is set"]
    #[inline(always)]
    pub fn discarded(self) -> &'a mut W {
        self.variant(RDMO_A::DISCARDED)
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
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    pub fn hren(&self) -> HREN_R {
        HREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    pub fn hrpol(&self) -> HRPOL_R {
        HRPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    pub fn hrsel(&self) -> HRSEL_R {
        HRSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    pub fn cirfifo(&self) -> CIRFIFO_R {
        CIRFIFO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    pub fn rdmo(&self) -> RDMO_R {
        RDMO_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    pub fn hren(&mut self) -> HREN_W {
        HREN_W { w: self }
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    pub fn hrpol(&mut self) -> HRPOL_W {
        HRPOL_W { w: self }
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    pub fn hrsel(&mut self) -> HRSEL_W {
        HRSEL_W { w: self }
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    pub fn cirfifo(&mut self) -> CIRFIFO_W {
        CIRFIFO_W { w: self }
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    pub fn rdmo(&mut self) -> RDMO_W {
        RDMO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr0](index.html) module"]
pub struct CFGR0_SPEC;
impl crate::RegisterSpec for CFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr0::R](R) reader structure"]
impl crate::Readable for CFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr0::W](W) writer structure"]
impl crate::Writable for CFGR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR0 to value 0"]
impl crate::Resettable for CFGR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
