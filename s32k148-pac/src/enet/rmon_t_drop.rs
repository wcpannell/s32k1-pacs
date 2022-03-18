#[doc = "Register `RMON_T_DROP` reader"]
pub struct R(crate::R<RMON_T_DROP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_T_DROP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_T_DROP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_T_DROP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_drop](index.html) module"]
pub struct RMON_T_DROP_SPEC;
impl crate::RegisterSpec for RMON_T_DROP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_t_drop::R](R) reader structure"]
impl crate::Readable for RMON_T_DROP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_T_DROP to value 0"]
impl crate::Resettable for RMON_T_DROP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
