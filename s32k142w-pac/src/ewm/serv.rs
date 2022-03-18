#[doc = "Register `SERV` reader"]
pub struct R(crate::R<SERV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SERV` writer"]
pub struct W(crate::W<SERV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERV_SPEC>;
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
impl From<crate::W<SERV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERVICE` reader - SERVICE"]
pub struct SERVICE_R(crate::FieldReader<u8, u8>);
impl SERVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SERVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERVICE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERVICE` writer - SERVICE"]
pub struct SERVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> SERVICE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SERVICE"]
    #[inline(always)]
    pub fn service(&self) -> SERVICE_R {
        SERVICE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SERVICE"]
    #[inline(always)]
    pub fn service(&mut self) -> SERVICE_W {
        SERVICE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serv](index.html) module"]
pub struct SERV_SPEC;
impl crate::RegisterSpec for SERV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [serv::R](R) reader structure"]
impl crate::Readable for SERV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [serv::W](W) writer structure"]
impl crate::Writable for SERV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SERV to value 0"]
impl crate::Resettable for SERV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
