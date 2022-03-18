#[doc = "Register `TCD2_DLASTSGA` reader"]
pub struct R(crate::R<TCD2_DLASTSGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD2_DLASTSGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD2_DLASTSGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD2_DLASTSGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD2_DLASTSGA` writer"]
pub struct W(crate::W<TCD2_DLASTSGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD2_DLASTSGA_SPEC>;
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
impl From<crate::W<TCD2_DLASTSGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD2_DLASTSGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLASTSGA` reader - DLASTSGA"]
pub struct DLASTSGA_R(crate::FieldReader<u32, u32>);
impl DLASTSGA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DLASTSGA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLASTSGA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLASTSGA` writer - DLASTSGA"]
pub struct DLASTSGA_W<'a> {
    w: &'a mut W,
}
impl<'a> DLASTSGA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DLASTSGA"]
    #[inline(always)]
    pub fn dlastsga(&self) -> DLASTSGA_R {
        DLASTSGA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DLASTSGA"]
    #[inline(always)]
    pub fn dlastsga(&mut self) -> DLASTSGA_W {
        DLASTSGA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_dlastsga](index.html) module"]
pub struct TCD2_DLASTSGA_SPEC;
impl crate::RegisterSpec for TCD2_DLASTSGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd2_dlastsga::R](R) reader structure"]
impl crate::Readable for TCD2_DLASTSGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd2_dlastsga::W](W) writer structure"]
impl crate::Writable for TCD2_DLASTSGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD2_DLASTSGA to value 0"]
impl crate::Resettable for TCD2_DLASTSGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
