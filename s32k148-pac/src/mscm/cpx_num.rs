#[doc = "Register `CPxNUM` reader"]
pub struct R(crate::R<CPXNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPXNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPXNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPXNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPN` reader - Processor x Number"]
pub struct CPN_R(crate::FieldReader<bool, bool>);
impl CPN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Processor x Number"]
    #[inline(always)]
    pub fn cpn(&self) -> CPN_R {
        CPN_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Processor X Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpx_num](index.html) module"]
pub struct CPXNUM_SPEC;
impl crate::RegisterSpec for CPXNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpx_num::R](R) reader structure"]
impl crate::Readable for CPXNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CPxNUM to value 0"]
impl crate::Resettable for CPXNUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
