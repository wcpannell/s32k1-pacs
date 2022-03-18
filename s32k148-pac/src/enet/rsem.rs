#[doc = "Register `RSEM` reader"]
pub struct R(crate::R<RSEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSEM` writer"]
pub struct W(crate::W<RSEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSEM_SPEC>;
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
impl From<crate::W<RSEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SECTION_EMPTY` reader - Value Of The Receive FIFO Section Empty Threshold"]
pub struct RX_SECTION_EMPTY_R(crate::FieldReader<u8, u8>);
impl RX_SECTION_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_SECTION_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_SECTION_EMPTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_SECTION_EMPTY` writer - Value Of The Receive FIFO Section Empty Threshold"]
pub struct RX_SECTION_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_SECTION_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `STAT_SECTION_EMPTY` reader - RX Status FIFO Section Empty Threshold"]
pub struct STAT_SECTION_EMPTY_R(crate::FieldReader<u8, u8>);
impl STAT_SECTION_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STAT_SECTION_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_SECTION_EMPTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT_SECTION_EMPTY` writer - RX Status FIFO Section Empty Threshold"]
pub struct STAT_SECTION_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_SECTION_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn rx_section_empty(&self) -> RX_SECTION_EMPTY_R {
        RX_SECTION_EMPTY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn stat_section_empty(&self) -> STAT_SECTION_EMPTY_R {
        STAT_SECTION_EMPTY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn rx_section_empty(&mut self) -> RX_SECTION_EMPTY_W {
        RX_SECTION_EMPTY_W { w: self }
    }
    #[doc = "Bits 16:20 - RX Status FIFO Section Empty Threshold"]
    #[inline(always)]
    pub fn stat_section_empty(&mut self) -> STAT_SECTION_EMPTY_W {
        STAT_SECTION_EMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Section Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsem](index.html) module"]
pub struct RSEM_SPEC;
impl crate::RegisterSpec for RSEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsem::R](R) reader structure"]
impl crate::Readable for RSEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsem::W](W) writer structure"]
impl crate::Writable for RSEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSEM to value 0"]
impl crate::Resettable for RSEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
