#[doc = "Register `NVICIP143` reader"]
pub struct R(crate::R<NVICIP143_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP143_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP143_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP143_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP143` writer"]
pub struct W(crate::W<NVICIP143_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP143_SPEC>;
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
impl From<crate::W<NVICIP143_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP143_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI143` reader - Priority of interrupt 143"]
pub struct PRI143_R(crate::FieldReader<u8, u8>);
impl PRI143_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI143_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI143_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI143` writer - Priority of interrupt 143"]
pub struct PRI143_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI143_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 143"]
    #[inline(always)]
    pub fn pri143(&self) -> PRI143_R {
        PRI143_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 143"]
    #[inline(always)]
    pub fn pri143(&mut self) -> PRI143_W {
        PRI143_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip143](index.html) module"]
pub struct NVICIP143_SPEC;
impl crate::RegisterSpec for NVICIP143_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip143::R](R) reader structure"]
impl crate::Readable for NVICIP143_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip143::W](W) writer structure"]
impl crate::Writable for NVICIP143_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP143 to value 0"]
impl crate::Resettable for NVICIP143_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
