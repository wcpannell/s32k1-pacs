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
#[doc = "Service Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serv](index.html) module"]
pub struct SERV_SPEC;
impl crate::RegisterSpec for SERV_SPEC {
    type Ux = u8;
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
