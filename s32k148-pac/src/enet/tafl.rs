#[doc = "Register `TAFL` reader"]
pub struct R(crate::R<TAFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFL` writer"]
pub struct W(crate::W<TAFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFL_SPEC>;
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
impl From<crate::W<TAFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ALMOST_FULL` reader - Value Of The Transmit FIFO Almost Full Threshold"]
pub struct TX_ALMOST_FULL_R(crate::FieldReader<u8, u8>);
impl TX_ALMOST_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_ALMOST_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ALMOST_FULL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ALMOST_FULL` writer - Value Of The Transmit FIFO Almost Full Threshold"]
pub struct TX_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALMOST_FULL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn tx_almost_full(&self) -> TX_ALMOST_FULL_R {
        TX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn tx_almost_full(&mut self) -> TX_ALMOST_FULL_W {
        TX_ALMOST_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafl](index.html) module"]
pub struct TAFL_SPEC;
impl crate::RegisterSpec for TAFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafl::R](R) reader structure"]
impl crate::Readable for TAFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafl::W](W) writer structure"]
impl crate::Writable for TAFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAFL to value 0x08"]
impl crate::Resettable for TAFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
