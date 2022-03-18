#[doc = "Register `RMON_R_UNDERSIZE` reader"]
pub struct R(crate::R<RMON_R_UNDERSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_UNDERSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_UNDERSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_UNDERSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Number of receive packets with less than 64 bytes and good CRC"]
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
    #[doc = "Bits 0:15 - Number of receive packets with less than 64 bytes and good CRC"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_undersize](index.html) module"]
pub struct RMON_R_UNDERSIZE_SPEC;
impl crate::RegisterSpec for RMON_R_UNDERSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_undersize::R](R) reader structure"]
impl crate::Readable for RMON_R_UNDERSIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_UNDERSIZE to value 0"]
impl crate::Resettable for RMON_R_UNDERSIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
