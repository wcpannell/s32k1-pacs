#[doc = "Register `WATER` reader"]
pub struct R(crate::R<WATER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WATER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WATER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WATER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WATER` writer"]
pub struct W(crate::W<WATER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WATER_SPEC>;
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
impl From<crate::W<WATER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WATER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXWATER` reader - Transmit Watermark"]
pub struct TXWATER_R(crate::FieldReader<u8, u8>);
impl TXWATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXWATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXWATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXWATER` writer - Transmit Watermark"]
pub struct TXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TXCOUNT` reader - Transmit Counter"]
pub struct TXCOUNT_R(crate::FieldReader<u8, u8>);
impl TXCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWATER` reader - Receive Watermark"]
pub struct RXWATER_R(crate::FieldReader<u8, u8>);
impl RXWATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXWATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXWATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXWATER` writer - Receive Watermark"]
pub struct RXWATER_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RXCOUNT` reader - Receive Counter"]
pub struct RXCOUNT_R(crate::FieldReader<u8, u8>);
impl RXCOUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCOUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Transmit Counter"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - Receive Counter"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&mut self) -> TXWATER_W {
        TXWATER_W { w: self }
    }
    #[doc = "Bits 16:17 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&mut self) -> RXWATER_W {
        RXWATER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [water](index.html) module"]
pub struct WATER_SPEC;
impl crate::RegisterSpec for WATER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [water::R](R) reader structure"]
impl crate::Readable for WATER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [water::W](W) writer structure"]
impl crate::Writable for WATER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WATER to value 0"]
impl crate::Resettable for WATER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
