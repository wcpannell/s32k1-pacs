#[doc = "Register `WMB0_ID` reader"]
pub struct R(crate::R<WMB0_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMB0_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMB0_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMB0_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Received ID under Pretended Networking mode"]
pub struct ID_R(crate::FieldReader<u32, u32>);
impl ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:28 - Received ID under Pretended Networking mode"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
#[doc = "Wake Up Message Buffer Register for ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmb0_id](index.html) module"]
pub struct WMB0_ID_SPEC;
impl crate::RegisterSpec for WMB0_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmb0_id::R](R) reader structure"]
impl crate::Readable for WMB0_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WMB0_ID to value 0"]
impl crate::Resettable for WMB0_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
