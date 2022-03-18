#[doc = "Register `NVICIP125` reader"]
pub struct R(crate::R<NVICIP125_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP125_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP125_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP125_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP125` writer"]
pub struct W(crate::W<NVICIP125_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP125_SPEC>;
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
impl From<crate::W<NVICIP125_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP125_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI125` reader - Priority of interrupt 125"]
pub struct PRI125_R(crate::FieldReader<u8, u8>);
impl PRI125_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI125_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI125` writer - Priority of interrupt 125"]
pub struct PRI125_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI125_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 125"]
    #[inline(always)]
    pub fn pri125(&self) -> PRI125_R {
        PRI125_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 125"]
    #[inline(always)]
    pub fn pri125(&mut self) -> PRI125_W {
        PRI125_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip125](index.html) module"]
pub struct NVICIP125_SPEC;
impl crate::RegisterSpec for NVICIP125_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip125::R](R) reader structure"]
impl crate::Readable for NVICIP125_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip125::W](W) writer structure"]
impl crate::Writable for NVICIP125_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP125 to value 0"]
impl crate::Resettable for NVICIP125_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
