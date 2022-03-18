#[doc = "Register `S32_NVIC_ICER` reader"]
pub struct R(crate::R<S32_NVIC_ICER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S32_NVIC_ICER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S32_NVIC_ICER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S32_NVIC_ICER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S32_NVIC_ICER` writer"]
pub struct W(crate::W<S32_NVIC_ICER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S32_NVIC_ICER_SPEC>;
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
impl From<crate::W<S32_NVIC_ICER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S32_NVIC_ICER_SPEC>) -> Self {
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
#[doc = "Interrupt Clear Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s32_nvic_icer](index.html) module"]
pub struct S32_NVIC_ICER_SPEC;
impl crate::RegisterSpec for S32_NVIC_ICER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s32_nvic_icer::R](R) reader structure"]
impl crate::Readable for S32_NVIC_ICER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s32_nvic_icer::W](W) writer structure"]
impl crate::Writable for S32_NVIC_ICER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S32_NVIC_ICER to value 0"]
impl crate::Resettable for S32_NVIC_ICER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
