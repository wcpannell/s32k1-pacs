#[doc = "Register `CMPH` reader"]
pub struct R(crate::R<CMPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPH` writer"]
pub struct W(crate::W<CMPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPH_SPEC>;
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
impl From<crate::W<CMPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPAREH` reader - COMPAREH"]
pub struct COMPAREH_R(crate::FieldReader<u8, u8>);
impl COMPAREH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COMPAREH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPAREH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPAREH` writer - COMPAREH"]
pub struct COMPAREH_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAREH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPAREH"]
    #[inline(always)]
    pub fn compareh(&self) -> COMPAREH_R {
        COMPAREH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPAREH"]
    #[inline(always)]
    pub fn compareh(&mut self) -> COMPAREH_W {
        COMPAREH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmph](index.html) module"]
pub struct CMPH_SPEC;
impl crate::RegisterSpec for CMPH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmph::R](R) reader structure"]
impl crate::Readable for CMPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmph::W](W) writer structure"]
impl crate::Writable for CMPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPH to value 0xff"]
impl crate::Resettable for CMPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
