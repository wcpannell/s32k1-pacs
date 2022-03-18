#[doc = "Register `RCCR` reader"]
pub struct R(crate::R<RCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCCR` writer"]
pub struct W(crate::W<RCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCCR_SPEC>;
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
impl From<crate::W<RCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_CNT` reader - Reference Clock Count"]
pub struct REF_CNT_R(crate::FieldReader<u16, u16>);
impl REF_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REF_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REF_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REF_CNT` writer - Reference Clock Count"]
pub struct REF_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Reference Clock Count"]
    #[inline(always)]
    pub fn ref_cnt(&self) -> REF_CNT_R {
        REF_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reference Clock Count"]
    #[inline(always)]
    pub fn ref_cnt(&mut self) -> REF_CNT_W {
        REF_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMU Frequency Check Reference Count Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rccr](index.html) module"]
pub struct RCCR_SPEC;
impl crate::RegisterSpec for RCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rccr::R](R) reader structure"]
impl crate::Readable for RCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rccr::W](W) writer structure"]
impl crate::Writable for RCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCCR to value 0"]
impl crate::Resettable for RCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
