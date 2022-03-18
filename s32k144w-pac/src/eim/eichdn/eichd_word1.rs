#[doc = "Register `EICHD_WORD1` reader"]
pub struct R(crate::R<EICHD_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EICHD_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EICHD_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EICHD_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EICHD_WORD1` writer"]
pub struct W(crate::W<EICHD_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EICHD_WORD1_SPEC>;
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
impl From<crate::W<EICHD_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EICHD_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0_3DATA_MASK` reader - Data Mask Bytes 0-3"]
pub struct B0_3DATA_MASK_R(crate::FieldReader<u32, u32>);
impl B0_3DATA_MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        B0_3DATA_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_3DATA_MASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_3DATA_MASK` writer - Data Mask Bytes 0-3"]
pub struct B0_3DATA_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_3DATA_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Mask Bytes 0-3"]
    #[inline(always)]
    pub fn b0_3data_mask(&self) -> B0_3DATA_MASK_R {
        B0_3DATA_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Mask Bytes 0-3"]
    #[inline(always)]
    pub fn b0_3data_mask(&mut self) -> B0_3DATA_MASK_W {
        B0_3DATA_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Injection Channel Descriptor n, Word1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eichd_word1](index.html) module"]
pub struct EICHD_WORD1_SPEC;
impl crate::RegisterSpec for EICHD_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eichd_word1::R](R) reader structure"]
impl crate::Readable for EICHD_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eichd_word1::W](W) writer structure"]
impl crate::Writable for EICHD_WORD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EICHD_WORD1 to value 0"]
impl crate::Resettable for EICHD_WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
