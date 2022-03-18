#[doc = "Register `RBCT` reader"]
pub struct R(crate::R<RBCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBCT` writer"]
pub struct W(crate::W<RBCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBCT_SPEC>;
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
impl From<crate::W<RBCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMRK` reader - RX Buffer Watermark"]
pub struct WMRK_R(crate::FieldReader<u8, u8>);
impl WMRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WMRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMRK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMRK` writer - RX Buffer Watermark"]
pub struct WMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> WMRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRD_A {
    #[doc = "0: RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB31."]
    _0 = 0,
    #[doc = "1: RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR31."]
    _1 = 1,
}
impl From<RXBRD_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBRD` reader - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
pub struct RXBRD_R(crate::FieldReader<bool, RXBRD_A>);
impl RXBRD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXBRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBRD_A {
        match self.bits {
            false => RXBRD_A::_0,
            true => RXBRD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == RXBRD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBRD_A::_1
    }
}
impl core::ops::Deref for RXBRD_R {
    type Target = crate::FieldReader<bool, RXBRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBRD` writer - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
pub struct RXBRD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBRD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RX Buffer content is read using the AHB Bus registers QSPI_ARDB0 to QSPI_ARDB31."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXBRD_A::_0)
    }
    #[doc = "RX Buffer content is read using the IP Bus registers QSPI_RBDR0 to QSPI_RBDR31."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXBRD_A::_1)
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
impl R {
    #[doc = "Bits 0:4 - RX Buffer Watermark"]
    #[inline(always)]
    pub fn wmrk(&self) -> WMRK_R {
        WMRK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline(always)]
    pub fn rxbrd(&self) -> RXBRD_R {
        RXBRD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - RX Buffer Watermark"]
    #[inline(always)]
    pub fn wmrk(&mut self) -> WMRK_W {
        WMRK_W { w: self }
    }
    #[doc = "Bit 8 - RX Buffer Readout. This field specifies the access scheme for the RX Buffer readout."]
    #[inline(always)]
    pub fn rxbrd(&mut self) -> RXBRD_W {
        RXBRD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Buffer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbct](index.html) module"]
pub struct RBCT_SPEC;
impl crate::RegisterSpec for RBCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbct::R](R) reader structure"]
impl crate::Readable for RBCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbct::W](W) writer structure"]
impl crate::Writable for RBCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RBCT to value 0"]
impl crate::Resettable for RBCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
