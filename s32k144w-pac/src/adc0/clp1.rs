#[doc = "Register `CLP1` reader"]
pub struct R(crate::R<CLP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP1` writer"]
pub struct W(crate::W<CLP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP1_SPEC>;
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
impl From<crate::W<CLP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP1` reader - CLP1"]
pub struct CLP1_R(crate::FieldReader<u16, u16>);
impl CLP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLP1` writer - CLP1"]
pub struct CLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - CLP1"]
    #[inline(always)]
    pub fn clp1(&self) -> CLP1_R {
        CLP1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - CLP1"]
    #[inline(always)]
    pub fn clp1(&mut self) -> CLP1_W {
        CLP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1](index.html) module"]
pub struct CLP1_SPEC;
impl crate::RegisterSpec for CLP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp1::R](R) reader structure"]
impl crate::Readable for CLP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp1::W](W) writer structure"]
impl crate::Writable for CLP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLP1 to value 0"]
impl crate::Resettable for CLP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
