#[doc = "Register `SHIFTERR` reader"]
pub struct R(crate::R<SHIFTERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTERR` writer"]
pub struct W(crate::W<SHIFTERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTERR_SPEC>;
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
impl From<crate::W<SHIFTERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEF` reader - Shifter Error Flags"]
pub struct SEF_R(crate::FieldReader<u8, u8>);
impl SEF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEF` writer - Shifter Error Flags"]
pub struct SEF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&mut self) -> SEF_W {
        SEF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shifterr](index.html) module"]
pub struct SHIFTERR_SPEC;
impl crate::RegisterSpec for SHIFTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shifterr::R](R) reader structure"]
impl crate::Readable for SHIFTERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shifterr::W](W) writer structure"]
impl crate::Writable for SHIFTERR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTERR to value 0"]
impl crate::Resettable for SHIFTERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
