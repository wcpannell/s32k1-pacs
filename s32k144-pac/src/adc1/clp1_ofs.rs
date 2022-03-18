#[doc = "Register `CLP1_OFS` reader"]
pub struct R(crate::R<CLP1_OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP1_OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP1_OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP1_OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP1_OFS` writer"]
pub struct W(crate::W<CLP1_OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP1_OFS_SPEC>;
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
impl From<crate::W<CLP1_OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP1_OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP1_OFS` reader - CLP1 Offset"]
pub struct CLP1_OFS_R(crate::FieldReader<u8, u8>);
impl CLP1_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLP1_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLP1_OFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLP1_OFS` writer - CLP1 Offset"]
pub struct CLP1_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP1_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLP1 Offset"]
    #[inline(always)]
    pub fn clp1_ofs(&self) -> CLP1_OFS_R {
        CLP1_OFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLP1 Offset"]
    #[inline(always)]
    pub fn clp1_ofs(&mut self) -> CLP1_OFS_W {
        CLP1_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1_ofs](index.html) module"]
pub struct CLP1_OFS_SPEC;
impl crate::RegisterSpec for CLP1_OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp1_ofs::R](R) reader structure"]
impl crate::Readable for CLP1_OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp1_ofs::W](W) writer structure"]
impl crate::Writable for CLP1_OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLP1_OFS to value 0"]
impl crate::Resettable for CLP1_OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
