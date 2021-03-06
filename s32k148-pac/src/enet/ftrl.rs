#[doc = "Register `FTRL` reader"]
pub struct R(crate::R<FTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTRL` writer"]
pub struct W(crate::W<FTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTRL_SPEC>;
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
impl From<crate::W<FTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRUNC_FL` reader - Frame Truncation Length"]
pub struct TRUNC_FL_R(crate::FieldReader<u16, u16>);
impl TRUNC_FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TRUNC_FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRUNC_FL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRUNC_FL` writer - Frame Truncation Length"]
pub struct TRUNC_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRUNC_FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    pub fn trunc_fl(&self) -> TRUNC_FL_R {
        TRUNC_FL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    pub fn trunc_fl(&mut self) -> TRUNC_FL_W {
        TRUNC_FL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Truncation Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftrl](index.html) module"]
pub struct FTRL_SPEC;
impl crate::RegisterSpec for FTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftrl::R](R) reader structure"]
impl crate::Readable for FTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftrl::W](W) writer structure"]
impl crate::Writable for FTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTRL to value 0x07ff"]
impl crate::Resettable for FTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07ff
    }
}
