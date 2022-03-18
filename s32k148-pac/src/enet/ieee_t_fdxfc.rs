#[doc = "Register `IEEE_T_FDXFC` reader"]
pub struct R(crate::R<IEEE_T_FDXFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_T_FDXFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_T_FDXFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_T_FDXFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of flow-control pause frames transmitted"]
pub struct COUNT_R(crate::FieldReader<u16, u16>);
impl COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of flow-control pause frames transmitted"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Flow Control Pause Frames Transmitted Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_fdxfc](index.html) module"]
pub struct IEEE_T_FDXFC_SPEC;
impl crate::RegisterSpec for IEEE_T_FDXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_t_fdxfc::R](R) reader structure"]
impl crate::Readable for IEEE_T_FDXFC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IEEE_T_FDXFC to value 0"]
impl crate::Resettable for IEEE_T_FDXFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
