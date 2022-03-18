#[doc = "Register `CPxCFG2` reader"]
pub struct R(crate::R<CPXCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPXCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPXCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPXCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TMUSZ` reader - Tightly-coupled Memory Upper Size"]
pub struct TMUSZ_R(crate::FieldReader<u8, u8>);
impl TMUSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMUSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMUSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMLSZ` reader - Tightly-coupled Memory Lower Size"]
pub struct TMLSZ_R(crate::FieldReader<u8, u8>);
impl TMLSZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMLSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMLSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:15 - Tightly-coupled Memory Upper Size"]
    #[inline(always)]
    pub fn tmusz(&self) -> TMUSZ_R {
        TMUSZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Tightly-coupled Memory Lower Size"]
    #[inline(always)]
    pub fn tmlsz(&self) -> TMLSZ_R {
        TMLSZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Processor X Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_cfg2](index.html) module"]
pub struct CPXCFG2_SPEC;
impl crate::RegisterSpec for CPXCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpx_cfg2::R](R) reader structure"]
impl crate::Readable for CPXCFG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPxCFG2 to value 0x0001_0001"]
impl crate::Resettable for CPXCFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0001
    }
}
