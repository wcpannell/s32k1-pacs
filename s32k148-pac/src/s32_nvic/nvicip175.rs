#[doc = "Register `NVICIP175` reader"]
pub struct R(crate::R<NVICIP175_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP175_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP175_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP175_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP175` writer"]
pub struct W(crate::W<NVICIP175_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP175_SPEC>;
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
impl From<crate::W<NVICIP175_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP175_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI175` reader - Priority of interrupt 175"]
pub struct PRI175_R(crate::FieldReader<u8, u8>);
impl PRI175_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI175_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI175_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI175` writer - Priority of interrupt 175"]
pub struct PRI175_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI175_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 175"]
    #[inline(always)]
    pub fn pri175(&self) -> PRI175_R {
        PRI175_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 175"]
    #[inline(always)]
    pub fn pri175(&mut self) -> PRI175_W {
        PRI175_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip175](index.html) module"]
pub struct NVICIP175_SPEC;
impl crate::RegisterSpec for NVICIP175_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip175::R](R) reader structure"]
impl crate::Readable for NVICIP175_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip175::W](W) writer structure"]
impl crate::Writable for NVICIP175_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP175 to value 0"]
impl crate::Resettable for NVICIP175_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
