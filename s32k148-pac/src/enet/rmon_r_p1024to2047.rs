#[doc = "Register `RMON_R_P1024TO2047` reader"]
pub struct R(crate::R<RMON_R_P1024TO2047_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_P1024TO2047_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_P1024TO2047_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_P1024TO2047_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of 1024- to 2047-byte recieve packets"]
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
    #[doc = "Bits 0:15 - Number of 1024- to 2047-byte recieve packets"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Rx 1024- to 2047-Byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_p1024to2047](index.html) module"]
pub struct RMON_R_P1024TO2047_SPEC;
impl crate::RegisterSpec for RMON_R_P1024TO2047_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_p1024to2047::R](R) reader structure"]
impl crate::Readable for RMON_R_P1024TO2047_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_P1024TO2047 to value 0"]
impl crate::Resettable for RMON_R_P1024TO2047_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
