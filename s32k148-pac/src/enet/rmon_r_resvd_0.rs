#[doc = "Register `RMON_R_RESVD_0` reader"]
pub struct R(crate::R<RMON_R_RESVD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_RESVD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_RESVD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_RESVD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_resvd_0](index.html) module"]
pub struct RMON_R_RESVD_0_SPEC;
impl crate::RegisterSpec for RMON_R_RESVD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_resvd_0::R](R) reader structure"]
impl crate::Readable for RMON_R_RESVD_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_RESVD_0 to value 0"]
impl crate::Resettable for RMON_R_RESVD_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
