#[doc = "Register `TIMCMP0` reader"]
pub struct R(crate::R<TIMCMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCMP0` writer"]
pub struct W(crate::W<TIMCMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCMP0_SPEC>;
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
impl From<crate::W<TIMCMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - Timer Compare Value"]
pub struct CMP_R(crate::FieldReader<u16, u16>);
impl CMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP` writer - Timer Compare Value"]
pub struct CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Compare Value"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Compare Value"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Compare N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcmp0](index.html) module"]
pub struct TIMCMP0_SPEC;
impl crate::RegisterSpec for TIMCMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timcmp0::R](R) reader structure"]
impl crate::Readable for TIMCMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timcmp0::W](W) writer structure"]
impl crate::Writable for TIMCMP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMCMP0 to value 0"]
impl crate::Resettable for TIMCMP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
