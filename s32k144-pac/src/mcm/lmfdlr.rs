#[doc = "Register `LMFDLR` reader"]
pub struct R(crate::R<LMFDLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LMFDLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LMFDLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LMFDLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEFDL` reader - Parity or ECC Fault Data Low"]
pub struct PEFDL_R(crate::FieldReader<u32, u32>);
impl PEFDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PEFDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEFDL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Parity or ECC Fault Data Low"]
    #[inline(always)]
    pub fn pefdl(&self) -> PEFDL_R {
        PEFDL_R::new(self.bits)
    }
}
#[doc = "LMEM Fault Data Low Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lmfdlr](index.html) module"]
pub struct LMFDLR_SPEC;
impl crate::RegisterSpec for LMFDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lmfdlr::R](R) reader structure"]
impl crate::Readable for LMFDLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LMFDLR to value 0"]
impl crate::Resettable for LMFDLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
