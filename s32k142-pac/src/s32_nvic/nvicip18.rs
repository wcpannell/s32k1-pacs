#[doc = "Register `NVICIP18` reader"]
pub struct R(crate::R<NVICIP18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP18` writer"]
pub struct W(crate::W<NVICIP18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP18_SPEC>;
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
impl From<crate::W<NVICIP18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP18_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI18` reader - Priority of interrupt 18"]
pub struct PRI18_R(crate::FieldReader<u8, u8>);
impl PRI18_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI18` writer - Priority of interrupt 18"]
pub struct PRI18_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn pri18(&self) -> PRI18_R {
        PRI18_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn pri18(&mut self) -> PRI18_W {
        PRI18_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip18](index.html) module"]
pub struct NVICIP18_SPEC;
impl crate::RegisterSpec for NVICIP18_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip18::R](R) reader structure"]
impl crate::Readable for NVICIP18_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip18::W](W) writer structure"]
impl crate::Writable for NVICIP18_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP18 to value 0"]
impl crate::Resettable for NVICIP18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
