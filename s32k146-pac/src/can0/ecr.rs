#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERRCNT` reader - Transmit Error Counter"]
pub struct TXERRCNT_R(crate::FieldReader<u8, u8>);
impl TXERRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXERRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERRCNT` writer - Transmit Error Counter"]
pub struct TXERRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `RXERRCNT` reader - Receive Error Counter"]
pub struct RXERRCNT_R(crate::FieldReader<u8, u8>);
impl RXERRCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXERRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERRCNT` writer - Receive Error Counter"]
pub struct RXERRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TXERRCNT_FAST` reader - Transmit Error Counter for fast bits"]
pub struct TXERRCNT_FAST_R(crate::FieldReader<u8, u8>);
impl TXERRCNT_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXERRCNT_FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRCNT_FAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERRCNT_FAST` writer - Transmit Error Counter for fast bits"]
pub struct TXERRCNT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRCNT_FAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RXERRCNT_FAST` reader - Receive Error Counter for fast bits"]
pub struct RXERRCNT_FAST_R(crate::FieldReader<u8, u8>);
impl RXERRCNT_FAST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXERRCNT_FAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERRCNT_FAST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERRCNT_FAST` writer - Receive Error Counter for fast bits"]
pub struct RXERRCNT_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRCNT_FAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerrcnt(&self) -> TXERRCNT_R {
        TXERRCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RXERRCNT_R {
        RXERRCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline(always)]
    pub fn txerrcnt_fast(&self) -> TXERRCNT_FAST_R {
        TXERRCNT_FAST_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline(always)]
    pub fn rxerrcnt_fast(&self) -> RXERRCNT_FAST_R {
        RXERRCNT_FAST_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerrcnt(&mut self) -> TXERRCNT_W {
        TXERRCNT_W { w: self }
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerrcnt(&mut self) -> RXERRCNT_W {
        RXERRCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline(always)]
    pub fn txerrcnt_fast(&mut self) -> TXERRCNT_FAST_W {
        TXERRCNT_FAST_W { w: self }
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline(always)]
    pub fn rxerrcnt_fast(&mut self) -> RXERRCNT_FAST_W {
        RXERRCNT_FAST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
