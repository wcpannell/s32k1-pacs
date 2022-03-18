#[doc = "Register `NVICIP105` reader"]
pub struct R(crate::R<NVICIP105_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP105_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP105_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP105_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP105` writer"]
pub struct W(crate::W<NVICIP105_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP105_SPEC>;
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
impl From<crate::W<NVICIP105_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP105_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI105` reader - Priority of interrupt 105"]
pub struct PRI105_R(crate::FieldReader<u8, u8>);
impl PRI105_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI105_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI105` writer - Priority of interrupt 105"]
pub struct PRI105_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI105_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 105"]
    #[inline(always)]
    pub fn pri105(&self) -> PRI105_R {
        PRI105_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 105"]
    #[inline(always)]
    pub fn pri105(&mut self) -> PRI105_W {
        PRI105_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip105](index.html) module"]
pub struct NVICIP105_SPEC;
impl crate::RegisterSpec for NVICIP105_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip105::R](R) reader structure"]
impl crate::Readable for NVICIP105_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip105::W](W) writer structure"]
impl crate::Writable for NVICIP105_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP105 to value 0"]
impl crate::Resettable for NVICIP105_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
