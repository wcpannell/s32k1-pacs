#[doc = "Register `NVICIP54` reader"]
pub struct R(crate::R<NVICIP54_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP54_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP54_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP54_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP54` writer"]
pub struct W(crate::W<NVICIP54_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP54_SPEC>;
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
impl From<crate::W<NVICIP54_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP54_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI54` reader - Priority of interrupt 54"]
pub struct PRI54_R(crate::FieldReader<u8, u8>);
impl PRI54_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI54_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI54` writer - Priority of interrupt 54"]
pub struct PRI54_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI54_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 54"]
    #[inline(always)]
    pub fn pri54(&self) -> PRI54_R {
        PRI54_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 54"]
    #[inline(always)]
    pub fn pri54(&mut self) -> PRI54_W {
        PRI54_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip54](index.html) module"]
pub struct NVICIP54_SPEC;
impl crate::RegisterSpec for NVICIP54_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip54::R](R) reader structure"]
impl crate::Readable for NVICIP54_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip54::W](W) writer structure"]
impl crate::Writable for NVICIP54_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP54 to value 0"]
impl crate::Resettable for NVICIP54_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
