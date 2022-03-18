#[doc = "Register `CMPL` reader"]
pub struct R(crate::R<CMPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPL` writer"]
pub struct W(crate::W<CMPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPL_SPEC>;
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
impl From<crate::W<CMPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPAREL` reader - COMPAREL"]
pub struct COMPAREL_R(crate::FieldReader<u8, u8>);
impl COMPAREL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPAREL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPAREL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREL` writer - COMPAREL"]
pub struct COMPAREL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPAREL"]
    #[inline(always)]
    pub fn comparel(&self) -> COMPAREL_R {
        COMPAREL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPAREL"]
    #[inline(always)]
    pub fn comparel(&mut self) -> COMPAREL_W {
        COMPAREL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpl](index.html) module"]
pub struct CMPL_SPEC;
impl crate::RegisterSpec for CMPL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpl::R](R) reader structure"]
impl crate::Readable for CMPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpl::W](W) writer structure"]
impl crate::Writable for CMPL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPL to value 0"]
impl crate::Resettable for CMPL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
