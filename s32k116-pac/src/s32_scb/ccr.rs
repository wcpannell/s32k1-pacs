#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Always reads as one, indicates that all unaligned accesses generate a HardFault"]
pub struct UNALIGN_TRP_R(crate::FieldReader<bool, bool>);
impl UNALIGN_TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UNALIGN_TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNALIGN_TRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub struct STKALIGN_R(crate::FieldReader<bool, bool>);
impl STKALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STKALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STKALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Always reads as one, indicates that all unaligned accesses generate a HardFault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CCR to value 0x0208"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0208
    }
}
