#[doc = "Register `MRBR` reader"]
pub struct R(crate::R<MRBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRBR` writer"]
pub struct W(crate::W<MRBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRBR_SPEC>;
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
impl From<crate::W<MRBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R_BUF_SIZE` reader - Receive buffer size in bytes"]
pub struct R_BUF_SIZE_R(crate::FieldReader<u16, u16>);
impl R_BUF_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        R_BUF_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R_BUF_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R_BUF_SIZE` writer - Receive buffer size in bytes"]
pub struct R_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> R_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 4)) | ((value as u32 & 0x03ff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:13 - Receive buffer size in bytes"]
    #[inline(always)]
    pub fn r_buf_size(&self) -> R_BUF_SIZE_R {
        R_BUF_SIZE_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:13 - Receive buffer size in bytes"]
    #[inline(always)]
    pub fn r_buf_size(&mut self) -> R_BUF_SIZE_W {
        R_BUF_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum Receive Buffer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrbr](index.html) module"]
pub struct MRBR_SPEC;
impl crate::RegisterSpec for MRBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrbr::R](R) reader structure"]
impl crate::Readable for MRBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrbr::W](W) writer structure"]
impl crate::Writable for MRBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MRBR to value 0"]
impl crate::Resettable for MRBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
