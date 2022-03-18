#[doc = "Register `CPxTYPE` reader"]
pub struct R(crate::R<CPXTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPXTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPXTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPXTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RYPZ` reader - Processor x Revision"]
pub struct RYPZ_R(crate::FieldReader<u8, u8>);
impl RYPZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RYPZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RYPZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERSONALITY` reader - Processor x Personality"]
pub struct PERSONALITY_R(crate::FieldReader<u32, u32>);
impl PERSONALITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PERSONALITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERSONALITY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Processor x Revision"]
    #[inline(always)]
    pub fn rypz(&self) -> RYPZ_R {
        RYPZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Processor x Personality"]
    #[inline(always)]
    pub fn personality(&self) -> PERSONALITY_R {
        PERSONALITY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "Processor X Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_type](index.html) module"]
pub struct CPXTYPE_SPEC;
impl crate::RegisterSpec for CPXTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpx_type::R](R) reader structure"]
impl crate::Readable for CPXTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPxTYPE to value 0"]
impl crate::Resettable for CPXTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
