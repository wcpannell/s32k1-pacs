#[doc = "Register `TAEM` reader"]
pub struct R(crate::R<TAEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAEM` writer"]
pub struct W(crate::W<TAEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAEM_SPEC>;
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
impl From<crate::W<TAEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ALMOST_EMPTY` reader - Value of Transmit FIFO Almost Empty Threshold"]
pub struct TX_ALMOST_EMPTY_R(crate::FieldReader<u8, u8>);
impl TX_ALMOST_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_ALMOST_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ALMOST_EMPTY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ALMOST_EMPTY` writer - Value of Transmit FIFO Almost Empty Threshold"]
pub struct TX_ALMOST_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ALMOST_EMPTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub fn tx_almost_empty(&self) -> TX_ALMOST_EMPTY_R {
        TX_ALMOST_EMPTY_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value of Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub fn tx_almost_empty(&mut self) -> TX_ALMOST_EMPTY_W {
        TX_ALMOST_EMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Almost Empty Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taem](index.html) module"]
pub struct TAEM_SPEC;
impl crate::RegisterSpec for TAEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taem::R](R) reader structure"]
impl crate::Readable for TAEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [taem::W](W) writer structure"]
impl crate::Writable for TAEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAEM to value 0x04"]
impl crate::Resettable for TAEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
