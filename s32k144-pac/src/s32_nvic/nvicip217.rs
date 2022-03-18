#[doc = "Register `NVICIP217` reader"]
pub struct R(crate::R<NVICIP217_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP217_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP217_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP217_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP217` writer"]
pub struct W(crate::W<NVICIP217_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP217_SPEC>;
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
impl From<crate::W<NVICIP217_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP217_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI217` reader - Priority of interrupt 217"]
pub struct PRI217_R(crate::FieldReader<u8, u8>);
impl PRI217_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI217_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI217_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI217` writer - Priority of interrupt 217"]
pub struct PRI217_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI217_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 217"]
    #[inline(always)]
    pub fn pri217(&self) -> PRI217_R {
        PRI217_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 217"]
    #[inline(always)]
    pub fn pri217(&mut self) -> PRI217_W {
        PRI217_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip217](index.html) module"]
pub struct NVICIP217_SPEC;
impl crate::RegisterSpec for NVICIP217_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip217::R](R) reader structure"]
impl crate::Readable for NVICIP217_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip217::W](W) writer structure"]
impl crate::Writable for NVICIP217_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP217 to value 0"]
impl crate::Resettable for NVICIP217_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
