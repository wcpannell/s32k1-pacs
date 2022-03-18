#[doc = "Register `RBSR` reader"]
pub struct R(crate::R<RBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDBFL` reader - RX Buffer Fill Level"]
pub struct RDBFL_R(crate::FieldReader<u8, u8>);
impl RDBFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDBFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDBFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDCTR` reader - Read Counter"]
pub struct RDCTR_R(crate::FieldReader<u16, u16>);
impl RDCTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RDCTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDCTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:13 - RX Buffer Fill Level"]
    #[inline(always)]
    pub fn rdbfl(&self) -> RDBFL_R {
        RDBFL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Read Counter"]
    #[inline(always)]
    pub fn rdctr(&self) -> RDCTR_R {
        RDCTR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RX Buffer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbsr](index.html) module"]
pub struct RBSR_SPEC;
impl crate::RegisterSpec for RBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbsr::R](R) reader structure"]
impl crate::Readable for RBSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RBSR to value 0"]
impl crate::Resettable for RBSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
