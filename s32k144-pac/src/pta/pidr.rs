#[doc = "Register `PIDR` reader"]
pub struct R(crate::R<PIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR` writer"]
pub struct W(crate::W<PIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR_SPEC>;
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
impl From<crate::W<PIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - Port Input Disable"]
pub struct PID_R(crate::FieldReader<u32, u32>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - Port Input Disable"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Input Disable"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Input Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr](index.html) module"]
pub struct PIDR_SPEC;
impl crate::RegisterSpec for PIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr::R](R) reader structure"]
impl crate::Readable for PIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr::W](W) writer structure"]
impl crate::Writable for PIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIDR to value 0"]
impl crate::Resettable for PIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
