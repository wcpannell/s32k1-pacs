#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CLKPRES` reader - Clock Present"]
pub struct CLKPRES_R(crate::FieldReader<u8, u8>);
impl CLKPRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLKPRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVPRES` reader - Divider Present"]
pub struct DIVPRES_R(crate::FieldReader<u8, u8>);
impl DIVPRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVPRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVPRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Present"]
    #[inline(always)]
    pub fn clkpres(&self) -> CLKPRES_R {
        CLKPRES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 27:31 - Divider Present"]
    #[inline(always)]
    pub fn divpres(&self) -> DIVPRES_R {
        DIVPRES_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0xf800_00fe"]
impl crate::Resettable for PARAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf800_00fe
    }
}
