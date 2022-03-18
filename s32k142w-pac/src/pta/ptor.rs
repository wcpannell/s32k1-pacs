#[doc = "Register `PTOR` reader"]
pub struct R(crate::R<PTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTOR` writer"]
pub struct W(crate::W<PTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTOR_SPEC>;
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
impl From<crate::W<PTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTTO` reader - Port Toggle Output"]
pub struct PTTO_R(crate::FieldReader<u32, u32>);
impl PTTO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PTTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTTO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTTO` writer - Port Toggle Output"]
pub struct PTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto(&self) -> PTTO_R {
        PTTO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto(&mut self) -> PTTO_W {
        PTTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Toggle Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptor](index.html) module"]
pub struct PTOR_SPEC;
impl crate::RegisterSpec for PTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptor::R](R) reader structure"]
impl crate::Readable for PTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptor::W](W) writer structure"]
impl crate::Writable for PTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
