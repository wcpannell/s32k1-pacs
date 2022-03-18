#[doc = "Register `OFS` reader"]
pub struct R(crate::R<OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFS` writer"]
pub struct W(crate::W<OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFS_SPEC>;
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
impl From<crate::W<OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFS` reader - Offset Error Correction Value"]
pub struct OFS_R(crate::FieldReader<u16, u16>);
impl OFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFS` writer - Offset Error Correction Value"]
pub struct OFS_W<'a> {
    w: &'a mut W,
}
impl<'a> OFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    pub fn ofs(&mut self) -> OFS_W {
        OFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](index.html) module"]
pub struct OFS_SPEC;
impl crate::RegisterSpec for OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofs::R](R) reader structure"]
impl crate::Readable for OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofs::W](W) writer structure"]
impl crate::Writable for OFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OFS to value 0"]
impl crate::Resettable for OFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
