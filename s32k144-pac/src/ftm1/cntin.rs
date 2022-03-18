#[doc = "Register `CNTIN` reader"]
pub struct R(crate::R<CNTIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTIN` writer"]
pub struct W(crate::W<CNTIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTIN_SPEC>;
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
impl From<crate::W<CNTIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - INIT"]
pub struct INIT_R(crate::FieldReader<u16, u16>);
impl INIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT` writer - INIT"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter Initial Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntin](index.html) module"]
pub struct CNTIN_SPEC;
impl crate::RegisterSpec for CNTIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntin::R](R) reader structure"]
impl crate::Readable for CNTIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntin::W](W) writer structure"]
impl crate::Writable for CNTIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTIN to value 0"]
impl crate::Resettable for CNTIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
