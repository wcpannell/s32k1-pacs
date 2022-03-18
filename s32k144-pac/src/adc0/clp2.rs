#[doc = "Register `CLP2` reader"]
pub struct R(crate::R<CLP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP2` writer"]
pub struct W(crate::W<CLP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP2_SPEC>;
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
impl From<crate::W<CLP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP2` reader - Calibration Value"]
pub struct CLP2_R(crate::FieldReader<u16, u16>);
impl CLP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLP2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLP2` writer - Calibration Value"]
pub struct CLP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&self) -> CLP2_R {
        CLP2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&mut self) -> CLP2_W {
        CLP2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2](index.html) module"]
pub struct CLP2_SPEC;
impl crate::RegisterSpec for CLP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp2::R](R) reader structure"]
impl crate::Readable for CLP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp2::W](W) writer structure"]
impl crate::Writable for CLP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLP2 to value 0xb8"]
impl crate::Resettable for CLP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb8
    }
}
