#[doc = "Register `DFWR` reader"]
pub struct R(crate::R<DFWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFWR` writer"]
pub struct W(crate::W<DFWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFWR_SPEC>;
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
impl From<crate::W<DFWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT` reader - Filter Length"]
pub struct FILT_R(crate::FieldReader<u8, u8>);
impl FILT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FILT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FILT` writer - Filter Length"]
pub struct FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Filter Length"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Filter Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfwr](index.html) module"]
pub struct DFWR_SPEC;
impl crate::RegisterSpec for DFWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfwr::R](R) reader structure"]
impl crate::Readable for DFWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfwr::W](W) writer structure"]
impl crate::Writable for DFWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFWR to value 0"]
impl crate::Resettable for DFWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
