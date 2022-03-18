#[doc = "Register `CLPX_OFS` reader"]
pub struct R(crate::R<CLPX_OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPX_OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPX_OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPX_OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPX_OFS` writer"]
pub struct W(crate::W<CLPX_OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPX_OFS_SPEC>;
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
impl From<crate::W<CLPX_OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPX_OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPX_OFS` reader - CLPX Offset"]
pub struct CLPX_OFS_R(crate::FieldReader<u16, u16>);
impl CLPX_OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CLPX_OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLPX_OFS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLPX_OFS` writer - CLPX Offset"]
pub struct CLPX_OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLPX_OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - CLPX Offset"]
    #[inline(always)]
    pub fn clpx_ofs(&self) -> CLPX_OFS_R {
        CLPX_OFS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - CLPX Offset"]
    #[inline(always)]
    pub fn clpx_ofs(&mut self) -> CLPX_OFS_W {
        CLPX_OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpx_ofs](index.html) module"]
pub struct CLPX_OFS_SPEC;
impl crate::RegisterSpec for CLPX_OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clpx_ofs::R](R) reader structure"]
impl crate::Readable for CLPX_OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clpx_ofs::W](W) writer structure"]
impl crate::Writable for CLPX_OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLPX_OFS to value 0x0440"]
impl crate::Resettable for CLPX_OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0440
    }
}
