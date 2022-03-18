#[doc = "Register `NVICICER0` reader"]
pub struct R(crate::R<NVICICER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICICER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICICER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICICER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICICER0` writer"]
pub struct W(crate::W<NVICICER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICICER0_SPEC>;
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
impl From<crate::W<NVICICER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICICER0_SPEC>) -> Self {
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
#[doc = "Interrupt Clear Enable Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicicer0](index.html) module"]
pub struct NVICICER0_SPEC;
impl crate::RegisterSpec for NVICICER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvicicer0::R](R) reader structure"]
impl crate::Readable for NVICICER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicicer0::W](W) writer structure"]
impl crate::Writable for NVICICER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICICER0 to value 0"]
impl crate::Resettable for NVICICER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
