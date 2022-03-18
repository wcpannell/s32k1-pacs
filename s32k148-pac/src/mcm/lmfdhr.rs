#[doc = "Register `LMFDHR` reader"]
pub struct R(crate::R<LMFDHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMFDHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMFDHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMFDHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEFDH` reader - Parity or ECC Fault Data High"]
pub struct PEFDH_R(crate::FieldReader<u32, u32>);
impl PEFDH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PEFDH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEFDH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Parity or ECC Fault Data High"]
    #[inline(always)]
    pub fn pefdh(&self) -> PEFDH_R {
        PEFDH_R::new(self.bits)
    }
}
#[doc = "LMEM Fault Data High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfdhr](index.html) module"]
pub struct LMFDHR_SPEC;
impl crate::RegisterSpec for LMFDHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmfdhr::R](R) reader structure"]
impl crate::Readable for LMFDHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LMFDHR to value 0"]
impl crate::Resettable for LMFDHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
