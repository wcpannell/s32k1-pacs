#[doc = "Register `NVICIP165` reader"]
pub struct R(crate::R<NVICIP165_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP165_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP165_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP165_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP165` writer"]
pub struct W(crate::W<NVICIP165_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP165_SPEC>;
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
impl From<crate::W<NVICIP165_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP165_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI165` reader - Priority of interrupt 165"]
pub struct PRI165_R(crate::FieldReader<u8, u8>);
impl PRI165_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI165_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI165_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI165` writer - Priority of interrupt 165"]
pub struct PRI165_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI165_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 165"]
    #[inline(always)]
    pub fn pri165(&self) -> PRI165_R {
        PRI165_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 165"]
    #[inline(always)]
    pub fn pri165(&mut self) -> PRI165_W {
        PRI165_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip165](index.html) module"]
pub struct NVICIP165_SPEC;
impl crate::RegisterSpec for NVICIP165_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip165::R](R) reader structure"]
impl crate::Readable for NVICIP165_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip165::W](W) writer structure"]
impl crate::Writable for NVICIP165_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP165 to value 0"]
impl crate::Resettable for NVICIP165_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
