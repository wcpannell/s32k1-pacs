#[doc = "Register `CLP9` reader"]
pub struct R(crate::R<CLP9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP9` writer"]
pub struct W(crate::W<CLP9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP9_SPEC>;
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
impl From<crate::W<CLP9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP9` reader - CLP9"]
pub struct CLP9_R(crate::FieldReader<u8, u8>);
impl CLP9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLP9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLP9` writer - CLP9"]
pub struct CLP9_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - CLP9"]
    #[inline(always)]
    pub fn clp9(&self) -> CLP9_R {
        CLP9_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - CLP9"]
    #[inline(always)]
    pub fn clp9(&mut self) -> CLP9_W {
        CLP9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp9](index.html) module"]
pub struct CLP9_SPEC;
impl crate::RegisterSpec for CLP9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp9::R](R) reader structure"]
impl crate::Readable for CLP9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp9::W](W) writer structure"]
impl crate::Writable for CLP9_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLP9 to value 0"]
impl crate::Resettable for CLP9_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
