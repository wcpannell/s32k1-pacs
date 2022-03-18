#[doc = "Register `S32_NVIC_ICPR` reader"]
pub struct R(crate::R<S32_NVIC_ICPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S32_NVIC_ICPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S32_NVIC_ICPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S32_NVIC_ICPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S32_NVIC_ICPR` writer"]
pub struct W(crate::W<S32_NVIC_ICPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S32_NVIC_ICPR_SPEC>;
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
impl From<crate::W<S32_NVIC_ICPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S32_NVIC_ICPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRPEND` reader - Interrupt clear-pending bits"]
pub struct CLRPEND_R(crate::FieldReader<u32, u32>);
impl CLRPEND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CLRPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRPEND_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRPEND` writer - Interrupt clear-pending bits"]
pub struct CLRPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRPEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits"]
    #[inline(always)]
    pub fn clrpend(&self) -> CLRPEND_R {
        CLRPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt clear-pending bits"]
    #[inline(always)]
    pub fn clrpend(&mut self) -> CLRPEND_W {
        CLRPEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s32_nvic_icpr](index.html) module"]
pub struct S32_NVIC_ICPR_SPEC;
impl crate::RegisterSpec for S32_NVIC_ICPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s32_nvic_icpr::R](R) reader structure"]
impl crate::Readable for S32_NVIC_ICPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s32_nvic_icpr::W](W) writer structure"]
impl crate::Writable for S32_NVIC_ICPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S32_NVIC_ICPR to value 0"]
impl crate::Resettable for S32_NVIC_ICPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
