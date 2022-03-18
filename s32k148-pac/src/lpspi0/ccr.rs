#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKDIV` reader - SCK Divider"]
pub struct SCKDIV_R(crate::FieldReader<u8, u8>);
impl SCKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKDIV` writer - SCK Divider"]
pub struct SCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DBT` reader - Delay Between Transfers"]
pub struct DBT_R(crate::FieldReader<u8, u8>);
impl DBT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBT` writer - Delay Between Transfers"]
pub struct DBT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PCSSCK` reader - PCS to SCK Delay"]
pub struct PCSSCK_R(crate::FieldReader<u8, u8>);
impl PCSSCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PCSSCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSSCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSSCK` writer - PCS to SCK Delay"]
pub struct PCSSCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SCKPCS` reader - SCK to PCS Delay"]
pub struct SCKPCS_R(crate::FieldReader<u8, u8>);
impl SCKPCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCKPCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCKPCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCKPCS` writer - SCK to PCS Delay"]
pub struct SCKPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCKPCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    pub fn sckdiv(&self) -> SCKDIV_R {
        SCKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    pub fn dbt(&self) -> DBT_R {
        DBT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PCS to SCK Delay"]
    #[inline(always)]
    pub fn pcssck(&self) -> PCSSCK_R {
        PCSSCK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCK to PCS Delay"]
    #[inline(always)]
    pub fn sckpcs(&self) -> SCKPCS_R {
        SCKPCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCK Divider"]
    #[inline(always)]
    pub fn sckdiv(&mut self) -> SCKDIV_W {
        SCKDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Delay Between Transfers"]
    #[inline(always)]
    pub fn dbt(&mut self) -> DBT_W {
        DBT_W { w: self }
    }
    #[doc = "Bits 16:23 - PCS to SCK Delay"]
    #[inline(always)]
    pub fn pcssck(&mut self) -> PCSSCK_W {
        PCSSCK_W { w: self }
    }
    #[doc = "Bits 24:31 - SCK to PCS Delay"]
    #[inline(always)]
    pub fn sckpcs(&mut self) -> SCKPCS_W {
        SCKPCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
