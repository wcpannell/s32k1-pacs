#[doc = "Register `CLP2_OFS` reader"]
pub struct R(crate::R<CLP2_OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP2_OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP2_OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP2_OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP2_OFS` writer"]
pub struct W(crate::W<CLP2_OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP2_OFS_SPEC>;
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
impl From<crate::W<CLP2_OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP2_OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP2_OFS` reader - CLP2 Offset"]
pub struct CLP2_OFS_R(crate::FieldReader<u8, u8>);
impl CLP2_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLP2_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLP2_OFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLP2_OFS` writer - CLP2 Offset"]
pub struct CLP2_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLP2_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLP2 Offset"]
    #[inline(always)]
    pub fn clp2_ofs(&self) -> CLP2_OFS_R {
        CLP2_OFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLP2 Offset"]
    #[inline(always)]
    pub fn clp2_ofs(&mut self) -> CLP2_OFS_W {
        CLP2_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2_ofs](index.html) module"]
pub struct CLP2_OFS_SPEC;
impl crate::RegisterSpec for CLP2_OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp2_ofs::R](R) reader structure"]
impl crate::Readable for CLP2_OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp2_ofs::W](W) writer structure"]
impl crate::Writable for CLP2_OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLP2_OFS to value 0"]
impl crate::Resettable for CLP2_OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
