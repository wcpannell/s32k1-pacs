#[doc = "Register `NVICICER6` reader"]
pub struct R(crate::R<NVICICER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICICER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICICER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICICER6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICICER6` writer"]
pub struct W(crate::W<NVICICER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICICER6_SPEC>;
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
impl From<crate::W<NVICICER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICICER6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRENA` reader - Interrupt clear-enable bits"]
pub struct CLRENA_R(crate::FieldReader<u32, u32>);
impl CLRENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLRENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRENA` writer - Interrupt clear-enable bits"]
pub struct CLRENA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits"]
    #[inline(always)]
    pub fn clrena(&self) -> CLRENA_R {
        CLRENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-enable bits"]
    #[inline(always)]
    pub fn clrena(&mut self) -> CLRENA_W {
        CLRENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Enable Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicicer6](index.html) module"]
pub struct NVICICER6_SPEC;
impl crate::RegisterSpec for NVICICER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvicicer6::R](R) reader structure"]
impl crate::Readable for NVICICER6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicicer6::W](W) writer structure"]
impl crate::Writable for NVICICER6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICICER6 to value 0"]
impl crate::Resettable for NVICICER6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
