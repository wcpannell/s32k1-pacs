#[doc = "Register `ATPER` reader"]
pub struct R(crate::R<ATPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATPER` writer"]
pub struct W(crate::W<ATPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATPER_SPEC>;
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
impl From<crate::W<ATPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Value for generating periodic events"]
pub struct PERIOD_R(crate::FieldReader<u32, u32>);
impl PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - Value for generating periodic events"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value for generating periodic events"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value for generating periodic events"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atper](index.html) module"]
pub struct ATPER_SPEC;
impl crate::RegisterSpec for ATPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atper::R](R) reader structure"]
impl crate::Readable for ATPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atper::W](W) writer structure"]
impl crate::Writable for ATPER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ATPER to value 0x3b9a_ca00"]
impl crate::Resettable for ATPER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3b9a_ca00
    }
}
