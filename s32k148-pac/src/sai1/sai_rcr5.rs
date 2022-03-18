#[doc = "Register `SAI_RCR5` reader"]
pub struct R(crate::R<SAI_RCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_RCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_RCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_RCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_RCR5` writer"]
pub struct W(crate::W<SAI_RCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_RCR5_SPEC>;
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
impl From<crate::W<SAI_RCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_RCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FBT` reader - First Bit Shifted"]
pub struct FBT_R(crate::FieldReader<u8, u8>);
impl FBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBT` writer - First Bit Shifted"]
pub struct FBT_W<'a> {
    w: &'a mut W,
}
impl<'a> FBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `W0W` reader - Word 0 Width"]
pub struct W0W_R(crate::FieldReader<u8, u8>);
impl W0W_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        W0W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for W0W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `W0W` writer - Word 0 Width"]
pub struct W0W_W<'a> {
    w: &'a mut W,
}
impl<'a> W0W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `WNW` reader - Word N Width"]
pub struct WNW_R(crate::FieldReader<u8, u8>);
impl WNW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WNW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WNW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WNW` writer - Word N Width"]
pub struct WNW_W<'a> {
    w: &'a mut W,
}
impl<'a> WNW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&self) -> FBT_R {
        FBT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&self) -> W0W_R {
        W0W_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&self) -> WNW_R {
        WNW_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&mut self) -> FBT_W {
        FBT_W { w: self }
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&mut self) -> W0W_W {
        W0W_W { w: self }
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&mut self) -> WNW_W {
        WNW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 5 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_rcr5](index.html) module"]
pub struct SAI_RCR5_SPEC;
impl crate::RegisterSpec for SAI_RCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_rcr5::R](R) reader structure"]
impl crate::Readable for SAI_RCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_rcr5::W](W) writer structure"]
impl crate::Writable for SAI_RCR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_RCR5 to value 0"]
impl crate::Resettable for SAI_RCR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
