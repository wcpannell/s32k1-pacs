#[doc = "Register `PCOR` reader"]
pub struct R(crate::R<PCOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCOR` writer"]
pub struct W(crate::W<PCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCOR_SPEC>;
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
impl From<crate::W<PCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTCO` reader - Port Clear Output"]
pub struct PTCO_R(crate::FieldReader<u32, u32>);
impl PTCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PTCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTCO_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTCO` writer - Port Clear Output"]
pub struct PTCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PTCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco(&self) -> PTCO_R {
        PTCO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Clear Output"]
    #[inline(always)]
    pub fn ptco(&mut self) -> PTCO_W {
        PTCO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Clear Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcor](index.html) module"]
pub struct PCOR_SPEC;
impl crate::RegisterSpec for PCOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcor::R](R) reader structure"]
impl crate::Readable for PCOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcor::W](W) writer structure"]
impl crate::Writable for PCOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCOR to value 0"]
impl crate::Resettable for PCOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
