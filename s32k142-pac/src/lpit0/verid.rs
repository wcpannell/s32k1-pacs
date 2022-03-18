#[doc = "Register `VERID` reader"]
pub struct R(crate::R<VERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEATURE` reader - Feature Number"]
pub struct FEATURE_R(crate::FieldReader<u16, u16>);
impl FEATURE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FEATURE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEATURE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINOR` reader - Minor Version Number"]
pub struct MINOR_R(crate::FieldReader<u8, u8>);
impl MINOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAJOR` reader - Major Version Number"]
pub struct MAJOR_R(crate::FieldReader<u8, u8>);
impl MAJOR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Feature Number"]
    #[inline(always)]
    pub fn feature(&self) -> FEATURE_R {
        FEATURE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verid](index.html) module"]
pub struct VERID_SPEC;
impl crate::RegisterSpec for VERID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [verid::R](R) reader structure"]
impl crate::Readable for VERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERID to value 0x0100_0000"]
impl crate::Resettable for VERID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
