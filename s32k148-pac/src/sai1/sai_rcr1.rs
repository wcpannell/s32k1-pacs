#[doc = "Register `SAI_RCR1` reader"]
pub struct R(crate::R<SAI_RCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_RCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_RCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_RCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_RCR1` writer"]
pub struct W(crate::W<SAI_RCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_RCR1_SPEC>;
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
impl From<crate::W<SAI_RCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_RCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFW` reader - Receive FIFO Watermark"]
pub struct RFW_R(crate::FieldReader<u8, u8>);
impl RFW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFW` writer - Receive FIFO Watermark"]
pub struct RFW_W<'a> {
    w: &'a mut W,
}
impl<'a> RFW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rfw(&mut self) -> RFW_W {
        RFW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_rcr1](index.html) module"]
pub struct SAI_RCR1_SPEC;
impl crate::RegisterSpec for SAI_RCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_rcr1::R](R) reader structure"]
impl crate::Readable for SAI_RCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_rcr1::W](W) writer structure"]
impl crate::Writable for SAI_RCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_RCR1 to value 0"]
impl crate::Resettable for SAI_RCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
