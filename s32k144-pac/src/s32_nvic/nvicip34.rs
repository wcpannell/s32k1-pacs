#[doc = "Register `NVICIP34` reader"]
pub struct R(crate::R<NVICIP34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVICIP34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVICIP34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVICIP34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVICIP34` writer"]
pub struct W(crate::W<NVICIP34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVICIP34_SPEC>;
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
impl From<crate::W<NVICIP34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVICIP34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI34` reader - Priority of interrupt 34"]
pub struct PRI34_R(crate::FieldReader<u8, u8>);
impl PRI34_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI34_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI34` writer - Priority of interrupt 34"]
pub struct PRI34_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Priority of interrupt 34"]
    #[inline(always)]
    pub fn pri34(&self) -> PRI34_R {
        PRI34_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority of interrupt 34"]
    #[inline(always)]
    pub fn pri34(&mut self) -> PRI34_W {
        PRI34_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvicip34](index.html) module"]
pub struct NVICIP34_SPEC;
impl crate::RegisterSpec for NVICIP34_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nvicip34::R](R) reader structure"]
impl crate::Readable for NVICIP34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvicip34::W](W) writer structure"]
impl crate::Writable for NVICIP34_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVICIP34 to value 0"]
impl crate::Resettable for NVICIP34_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
