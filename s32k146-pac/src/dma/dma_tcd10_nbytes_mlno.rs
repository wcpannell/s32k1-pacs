#[doc = "Register `TCD10_NBYTES_MLNO` reader"]
pub struct R(crate::R<DMA_TCD10_NBYTES_MLNO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TCD10_NBYTES_MLNO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TCD10_NBYTES_MLNO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TCD10_NBYTES_MLNO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD10_NBYTES_MLNO` writer"]
pub struct W(crate::W<DMA_TCD10_NBYTES_MLNO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TCD10_NBYTES_MLNO_SPEC>;
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
impl From<crate::W<DMA_TCD10_NBYTES_MLNO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TCD10_NBYTES_MLNO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBYTES` reader - Minor Byte Transfer Count"]
pub struct NBYTES_R(crate::FieldReader<u32, u32>);
impl NBYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - Minor Byte Transfer Count"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Minor Byte Transfer Count"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tcd10_nbytes_mlno](index.html) module"]
pub struct DMA_TCD10_NBYTES_MLNO_SPEC;
impl crate::RegisterSpec for DMA_TCD10_NBYTES_MLNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_tcd10_nbytes_mlno::R](R) reader structure"]
impl crate::Readable for DMA_TCD10_NBYTES_MLNO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tcd10_nbytes_mlno::W](W) writer structure"]
impl crate::Writable for DMA_TCD10_NBYTES_MLNO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD10_NBYTES_MLNO to value 0"]
impl crate::Resettable for DMA_TCD10_NBYTES_MLNO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
