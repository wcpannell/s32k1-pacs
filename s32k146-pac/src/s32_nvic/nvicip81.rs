#[doc = "Register `NVICIP81` reader"]
pub struct R(crate::R<NVICIP81_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP81_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP81_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP81_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP81` writer"]
pub struct W(crate::W<NVICIP81_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP81_SPEC>;
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
impl From<crate::W<NVICIP81_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP81_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI81` reader - Priority of interrupt 81"]
pub struct PRI81_R(crate::FieldReader<u8, u8>);
impl PRI81_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI81_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI81_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI81` writer - Priority of interrupt 81"]
pub struct PRI81_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI81_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 81"]
    #[inline(always)]
    pub fn pri81(&self) -> PRI81_R {
        PRI81_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 81"]
    #[inline(always)]
    pub fn pri81(&mut self) -> PRI81_W {
        PRI81_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip81](index.html) module"]
pub struct NVICIP81_SPEC;
impl crate::RegisterSpec for NVICIP81_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip81::R](R) reader structure"]
impl crate::Readable for NVICIP81_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip81::W](W) writer structure"]
impl crate::Writable for NVICIP81_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP81 to value 0"]
impl crate::Resettable for NVICIP81_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
