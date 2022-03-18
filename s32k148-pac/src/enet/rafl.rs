#[doc = "Register `RAFL` reader"]
pub struct R(crate::R<RAFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAFL` writer"]
pub struct W(crate::W<RAFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAFL_SPEC>;
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
impl From<crate::W<RAFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_ALMOST_FULL` reader - Value Of The Receive FIFO Almost Full Threshold"]
pub struct RX_ALMOST_FULL_R(crate::FieldReader<u8, u8>);
impl RX_ALMOST_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_ALMOST_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ALMOST_FULL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ALMOST_FULL` writer - Value Of The Receive FIFO Almost Full Threshold"]
pub struct RX_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ALMOST_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn rx_almost_full(&self) -> RX_ALMOST_FULL_R {
        RX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn rx_almost_full(&mut self) -> RX_ALMOST_FULL_W {
        RX_ALMOST_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rafl](index.html) module"]
pub struct RAFL_SPEC;
impl crate::RegisterSpec for RAFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rafl::R](R) reader structure"]
impl crate::Readable for RAFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rafl::W](W) writer structure"]
impl crate::Writable for RAFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAFL to value 0x04"]
impl crate::Resettable for RAFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
