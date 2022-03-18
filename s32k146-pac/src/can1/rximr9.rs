#[doc = "Register `RXIMR9` reader"]
pub struct R(crate::R<RXIMR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIMR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIMR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIMR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIMR9` writer"]
pub struct W(crate::W<RXIMR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIMR9_SPEC>;
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
impl From<crate::W<RXIMR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIMR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MI` reader - Individual Mask Bits"]
pub struct MI_R(crate::FieldReader<u32, u32>);
impl MI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MI` writer - Individual Mask Bits"]
pub struct MI_W<'a> {
    w: &'a mut W,
}
impl<'a> MI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&mut self) -> MI_W {
        MI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx Individual Mask Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rximr9](index.html) module"]
pub struct RXIMR9_SPEC;
impl crate::RegisterSpec for RXIMR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rximr9::R](R) reader structure"]
impl crate::Readable for RXIMR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rximr9::W](W) writer structure"]
impl crate::Writable for RXIMR9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXIMR9 to value 0"]
impl crate::Resettable for RXIMR9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
