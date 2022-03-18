#[doc = "Register `LMFAR` reader"]
pub struct R(crate::R<LMFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFADD` reader - ECC Fault Address"]
pub struct EFADD_R(crate::FieldReader<u32, u32>);
impl EFADD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EFADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ECC Fault Address"]
    #[inline(always)]
    pub fn efadd(&self) -> EFADD_R {
        EFADD_R::new(self.bits)
    }
}
#[doc = "LMEM Fault Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfar](index.html) module"]
pub struct LMFAR_SPEC;
impl crate::RegisterSpec for LMFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmfar::R](R) reader structure"]
impl crate::Readable for LMFAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LMFAR to value 0"]
impl crate::Resettable for LMFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
