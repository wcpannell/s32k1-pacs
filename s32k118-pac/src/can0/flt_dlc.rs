#[doc = "Register `FLT_DLC` reader"]
pub struct R(crate::R<FLT_DLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT_DLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT_DLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT_DLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT_DLC` writer"]
pub struct W(crate::W<FLT_DLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT_DLC_SPEC>;
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
impl From<crate::W<FLT_DLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT_DLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLT_DLC_HI` reader - Upper Limit for Length of Data Bytes Filter"]
pub struct FLT_DLC_HI_R(crate::FieldReader<u8, u8>);
impl FLT_DLC_HI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLT_DLC_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_DLC_HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_DLC_HI` writer - Upper Limit for Length of Data Bytes Filter"]
pub struct FLT_DLC_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_DLC_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `FLT_DLC_LO` reader - Lower Limit for Length of Data Bytes Filter"]
pub struct FLT_DLC_LO_R(crate::FieldReader<u8, u8>);
impl FLT_DLC_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLT_DLC_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLT_DLC_LO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLT_DLC_LO` writer - Lower Limit for Length of Data Bytes Filter"]
pub struct FLT_DLC_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_DLC_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Upper Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_hi(&self) -> FLT_DLC_HI_R {
        FLT_DLC_HI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Lower Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_lo(&self) -> FLT_DLC_LO_R {
        FLT_DLC_LO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Upper Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_hi(&mut self) -> FLT_DLC_HI_W {
        FLT_DLC_HI_W { w: self }
    }
    #[doc = "Bits 16:19 - Lower Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub fn flt_dlc_lo(&mut self) -> FLT_DLC_LO_W {
        FLT_DLC_LO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pretended Networking DLC Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt_dlc](index.html) module"]
pub struct FLT_DLC_SPEC;
impl crate::RegisterSpec for FLT_DLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt_dlc::R](R) reader structure"]
impl crate::Readable for FLT_DLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt_dlc::W](W) writer structure"]
impl crate::Writable for FLT_DLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT_DLC to value 0x08"]
impl crate::Resettable for FLT_DLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
