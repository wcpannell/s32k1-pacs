#[doc = "Register `ATCOR` reader"]
pub struct R(crate::R<ATCOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCOR` writer"]
pub struct W(crate::W<ATCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCOR_SPEC>;
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
impl From<crate::W<ATCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COR` reader - Correction Counter Wrap-Around Value"]
pub struct COR_R(crate::FieldReader<u32, u32>);
impl COR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        COR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COR` writer - Correction Counter Wrap-Around Value"]
pub struct COR_W<'a> {
    w: &'a mut W,
}
impl<'a> COR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub fn cor(&self) -> COR_R {
        COR_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub fn cor(&mut self) -> COR_W {
        COR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcor](index.html) module"]
pub struct ATCOR_SPEC;
impl crate::RegisterSpec for ATCOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcor::R](R) reader structure"]
impl crate::Readable for ATCOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcor::W](W) writer structure"]
impl crate::Writable for ATCOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATCOR to value 0"]
impl crate::Resettable for ATCOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
