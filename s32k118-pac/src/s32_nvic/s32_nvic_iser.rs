#[doc = "Register `S32_NVIC_ISER` reader"]
pub struct R(crate::R<S32_NVIC_ISER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S32_NVIC_ISER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S32_NVIC_ISER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S32_NVIC_ISER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S32_NVIC_ISER` writer"]
pub struct W(crate::W<S32_NVIC_ISER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S32_NVIC_ISER_SPEC>;
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
impl From<crate::W<S32_NVIC_ISER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S32_NVIC_ISER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENA` reader - Interrupt set enable bits"]
pub struct SETENA_R(crate::FieldReader<u32, u32>);
impl SETENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SETENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETENA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETENA` writer - Interrupt set enable bits"]
pub struct SETENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SETENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt set enable bits"]
    #[inline(always)]
    pub fn setena(&self) -> SETENA_R {
        SETENA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt set enable bits"]
    #[inline(always)]
    pub fn setena(&mut self) -> SETENA_W {
        SETENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s32_nvic_iser](index.html) module"]
pub struct S32_NVIC_ISER_SPEC;
impl crate::RegisterSpec for S32_NVIC_ISER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s32_nvic_iser::R](R) reader structure"]
impl crate::Readable for S32_NVIC_ISER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s32_nvic_iser::W](W) writer structure"]
impl crate::Writable for S32_NVIC_ISER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S32_NVIC_ISER to value 0"]
impl crate::Resettable for S32_NVIC_ISER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
